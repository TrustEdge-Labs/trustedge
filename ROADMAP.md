# TrustEdge Roadmap

> **North Star:** Trusted data from the edge. Capture bytes (audio/video/sensor), encrypt at the edge, attach verifiable provenance, and move them through untrusted pipes. Anyone downstream can **route** and **verify**; nobody can **forge** or **peek**.

---

## Outcomes (What we’ll ship)

1. **TrustEdge SDK (Rust crate + FFI-ready)**

   * Per-chunk AES-GCM with strict nonce discipline.
   * C2PA-inspired signed manifests bound into AEAD AAD.
   * Stable **envelope format** (`.trst`) with preamble & versioning.
   * Key management adapters (demo keyring → TPM/KMS path).
   * Verifier APIs (manifest sig + AAD + payload hash).

2. **CLI tools**

   * `trustedge-audio` — local capture → encrypt → envelope; `--decrypt` verify/restore.
   * `trustedge-client` / `trustedge-server` — reference network path with ACKs.

3. **Reference Router / Gateway (stateless)**

   * Accept `.trst` streams, enforce invariants, route to storage/queues (S3, NATS/Kafka).
   * Verify-optional; never needs decryption to route.

4. **Verification tooling**

   * `trustedge-verify` CLI: validate envelopes/streams and emit human/JSONL audit.

5. **Format spec**

   * Public, minimal, **test-vectored** spec for `StreamHeader` + `Record` + manifest schema.
   * Clear evolution story (MAGIC, VERSION, reserved fields).

---

## Architecture (snapshot)

* **Edge/Producer**
  Chunk → hash → sign manifest (Ed25519) → AAD = `{ header_hash || seq || nonce || blake3(manifest) }` → AES-GCM encrypt → emit `.trst` or network frame.

* **Transit/Router**
  Sees headers/sequences; can route/rate-limit; cannot read or forge.

* **Consumer**
  Verify signature & AAD, decrypt (if authorized), reassemble; or verify-only.

**Security goals**

* **Confidentiality:** AES-256-GCM; unique nonce per key/session.
* **Integrity & provenance:** Ed25519 over manifest + AAD binding.
* **Replay resistance:** contiguous `seq`, checked nonce prefix.
* **Privacy by default:** only minimal routable metadata is exposed.

---

## Current Status

* [x] Per-chunk encrypt/decrypt round-trip
* [x] Signed manifest bound into AAD
* [x] `.trst` envelope with preamble (`MAGIC="TRST"`, `VERSION=1`) and invariants
* [x] Reference client/server with ACKs
* [x] Keyring-derived key support
* [x] Shared types/helpers centralized in the lib crate

---

## Milestone M1 — **Format v1 Freeze (MVP hardening)**

**Goal:** Freeze v1 of the envelope & manifest and publish a minimal spec + vectors.

**Scope**

* [ ] Finalize manifest fields (add `key_id`; keep `model_ids`, `ai_used`; reserve `device_attest`).
* [ ] Confirm invariants: contiguous `seq`, fixed `nonce_prefix`, `header_hash` match, AAD layout.
* [ ] File framing: keep preamble + bincode framing; document record boundaries & EOF handling.
* [ ] Spec doc (`FORMAT.md`): structures, byte orders, invariants, failure modes.
* [ ] Test vectors: tiny `.trst` with 1–3 records + known keys; publish expected hashes/tags.

**Acceptance**

* A reader implemented from the spec (no repo code) can verify test vectors.
* Fuzz on `deserialize(Record)` doesn’t crash (see M3).

---

## Milestone M2 — **Key Management (production-ish)**

**Goal:** Introduce durable key IDs and a rotation story.

**Scope**

* [ ] Add `key_id` to `FileHeader` and surface in spec + CLIs.
* [ ] Derive per-session AEAD keys from a device root via HKDF (demo root = keyring passphrase + salt).
* [ ] Reject decrypt when `key_id` mismatches the configured/derivable key.
* [ ] CLI UX: `--use-keyring --salt-hex …` (derive); `--key-hex` (override).
* [ ] Document rotation & recovery (how to re-derive past session keys).

**Acceptance**

* Decrypt fails fast on wrong `key_id`.
* Round-trip passes with both keyring-derived and explicit `--key-hex`.

---

## Milestone M3 — **Verification Tooling & QA**

**Goal:** Ship a verifier and raise safety with tests & fuzzing.

**Scope**

* [ ] `trustedge-verify`: read `.trst`, print human-readable report (seq, signer, hashes, sizes).
* [ ] `--json` output for SIEM (JSONL per record).
* [ ] Property tests (proptest): AAD layout, seq monotonicity, header hash binding.
* [ ] Fuzzing (`cargo-fuzz`) on decoders (preamble, `StreamHeader`, `Record`).
* [ ] Known-answer tests (KATs): deterministic vectors in CI.
* [ ] Zeroization audit for secrets; non-leaky error messages.

**Acceptance**

* CI runs proptests + fuzz corpus sanity (+1min smoke).
* Verifier detects: nonce prefix mismatch, seq gap, tampered manifest/payload.

---

## Milestone M4 — **Transport & Router**

**Goal:** Better network story + reference router.

**Scope**

* [ ] QUIC or NATS transport with backpressure/ordering metrics.
* [ ] Stateless Router: verify (optional), enforce invariants, forward to sinks (S3/NATS/Kafka).
* [ ] Structured logs + Prometheus counters: chunks, drops, verify failures, latency.

**Acceptance**

* E2E demo: client → router → sink → verifier/decrypt.
* Backpressure test: router throttles without data loss or deadlocks.

---

## Milestone M5 — **Attestation Hook (future-proof)**

**Goal:** Reserve space for device/TEE attestation without blocking v1.

**Scope**

* [ ] Add optional `device_attest` field in manifest (opaque bytes, signed).
* [ ] Document how a TEE/TPM quote would be produced/verified (deferred).
* [ ] Keep verification tolerant when absent.

**Acceptance**

* Manifests with/without attestation both verify; size overhead documented.

---

## Longer-Term

* **Interoperability:** Optional CBOR/COSE envelope variant; map manifest to C2PA claims where practical.
* **Packaging:** C ABI for SDK; language bindings.
* **Edge AI:** Standardize `model_ids` (name+version+hash); record basic runtime signals.
* **Storage adapters:** S3/GCS writers & lifecycle policies for encrypted objects.

---

## Non-Goals (for v1)

* End-to-end identity binding to a PKI (beyond Ed25519 keys in manifests).
* Multi-party re-encryption (KMS/ABE/MLS) — future exploration.
* Lossless media containerization — payload is opaque bytes.

---

## Risks & Mitigations

* **Nonce misuse** → Strict prefix+counter enforcement; tests preventing reuse; fail-closed.
* **Spec drift** → Single source of truth in `trustedge_audio` crate; doc + vectors.
* **Key loss** → Documented key derivation and rotation; warn loud in CLIs.
* **DoS via malformed input** → Length checks, fuzzing, non-allocating decode paths where possible.

---

## Developer Ergonomics

* **Issue labels:** `M1-Format`, `M2-KeyMgmt`, `M3-Verify`, `M4-Transport`, `M5-Attest`, `good-first-issue`.
* **CI matrix:** linux/mac/windows stable; minimal nightly for fuzz.
* **Examples:** `examples/` with tiny WAV → `.trst` → verify → restore pipeline.

---

## Quick Commands

```bash
# Encrypt → envelope + plaintext
trustedge-audio -i input.wav -o roundtrip.wav \
  --chunk 8192 --envelope out.trst --use-keyring --salt-hex "$SALT"

# Decrypt → verify + restore from envelope
trustedge-audio --decrypt -i out.trst -o restored.wav \
  --use-keyring --salt-hex "$SALT"

# Network demo (TCP)
trustedge-server --listen 127.0.0.1:8080 --decrypt --use-keyring --salt-hex "$SALT"
trustedge-client --server 127.0.0.1:8080 --file input.wav --use-keyring --salt-hex "$SALT"
```

---

## Definition Snippets (v1)

* **AAD layout:** `header_hash (32) || seq (8, BE) || nonce (12) || blake3(manifest) (32)`
* **Preamble:** `MAGIC="TRST"`, `VERSION=1`
* **Envelope:** `StreamHeader { v, header (58), header_hash (32) }` then repeated
  `Record { seq, nonce, sm, ct }`

---

## Success Criteria

* A third-party can implement a verifier from the spec + vectors and reach the same results.
* Tamper and misuse cases are reliably detected (and documented).
* Transport/storage backends are interchangeable without changing guarantees.
* Low-friction adoption: simple SDK, clear examples, deterministic tests.
