<!--
Copyright (c) 2025 John Turner
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedg**Acceptance**

* ✅ A reader implemented from the spec (no repo code) can verify test vectors.
* ✅ Deterministic test vectors with golden hash: `8ecc3b2fcb0887dfd6ff3513c0caa3febb2150a920213fa5b622243ad530f34c`
* ✅ Network stack: complete client/server with chunk validation and ACK protocol
* ✅ Comprehensive validation: all security invariants enforced during processing
* [ ] Fuzz on `deserialize(Record)` doesn't crash (see M3).Privacy and trust at the edge.
-->

# TrustEdge Roadmap

> **North Star:** Trusted data from the edge. Capture bytes (audio/video/sensor), encrypt at the edge, attach verifiable provenance, and move them through untrusted pipes. Anyone downstream can **route** and **verify**; nobody can **forge** or **peek**.

> **Security**: For security policies and vulnerability reporting, see [`SECURITY.md`](./SECURITY.md)

---

## Outcomes (To Be Shipped)

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
* [x] Reference client/server with ACKs and network streaming
* [x] Keyring-derived key support with PBKDF2
* [x] Shared types/helpers centralized in the lib crate
* [x] Format types consolidated in `src/format.rs`
* [x] Key ID field added to manifest for rotation support
* [x] Comprehensive validation: header consistency, sequence integrity, nonce verification
* [x] Production-ready network stack with TCP client/server
* [x] Deterministic test vectors with golden hash verification
* [x] Integration testing with CLI and network protocol validation

---

## Milestone M1 — **Format v1 Freeze (MVP hardening)**

**Goal:** Freeze v1 of the envelope & manifest and publish a minimal spec + vectors.

**Scope**

* [x] Finalize manifest fields (add `key_id`; keep `model_ids`, `ai_used`; reserve `device_attest`).
* [x] Consolidate format types in centralized `format.rs` module.
* [x] Spec doc (`FORMAT.md`): structures, byte orders, invariants, failure modes.
* [x] Confirm invariants: contiguous `seq`, fixed `nonce_prefix`, `header_hash` match, `key_id` match, AAD layout.
* [x] Test vectors: deterministic `.trst` with known keys and golden hash verification.
* [x] File framing: keep preamble + bincode framing; document record boundaries & EOF handling.
* [x] Enhanced validation: header consistency, key ID verification, strict sequencing
* [x] Network protocol: complete client/server implementation with validation
* [x] Comprehensive testing: unit tests, integration tests, CLI testing, network validation

**Acceptance**

* A reader implemented from the spec (no repo code) can verify test vectors.
* Fuzz on `deserialize(Record)` doesn’t crash (see M3).

---

## Milestone M2 — **Key Management & Enhanced Security**

**Goal:** Production-ready key management and advanced security features.

**Scope**

* [x] Add `key_id` to `FileHeader` and surface in spec + CLIs.
* [x] Derive per-session AEAD keys from a device root via PBKDF2 (keyring passphrase + salt).
* [x] Reject decrypt when `key_id` mismatches the configured/derivable key.
* [x] CLI UX: `--use-keyring --salt-hex …` (derive); `--key-hex` (override).
* [x] Document rotation & recovery (how to re-derive past session keys).
* [ ] Add key versioning and migration tools
* [ ] Implement secure key storage with proper zeroization
* [ ] Add HSM/TPM integration points for production deployments

**Acceptance**

* ✅ Decrypt fails fast on wrong `key_id`.
* ✅ Round-trip passes with both keyring-derived and explicit `--key-hex`.
* [ ] Key rotation scenarios fully tested and documented
* [ ] Production key management best practices documented

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

## Milestone M6 — **Security Review & Hardening**

**Goal:** Comprehensive security audit and hardening before v1.0 release.

**Scope**

* [ ] External cryptographic audit of implementation
* [ ] Side-channel attack analysis and mitigations
* [ ] Fuzzing campaign for all deserializers and network code
* [ ] Security testing of key management and nonce handling
* [ ] Review dependency supply chain and implement security monitoring
* [ ] Documentation review for security implications
* [ ] Penetration testing of network protocol components
* [ ] Performance impact analysis of security features

**Acceptance**

* Clean external security audit report
* Fuzzing runs 24h+ without crashes on all parsers
* Security test suite covers all critical paths
* Security documentation complete and reviewed

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

> **See Also**: [`THREAT_MODEL.md`](./THREAT_MODEL.md) for comprehensive threat analysis and [`SECURITY.md`](./SECURITY.md) for security policies.

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

# Network demo (TCP) - start server
trustedge-server --listen 127.0.0.1:8080 --decrypt \
  --use-keyring --salt-hex "$SALT" --verbose

# Network demo (TCP) - send file
trustedge-client --server 127.0.0.1:8080 --file input.wav \
  --use-keyring --salt-hex "$SALT"

# Test network with synthetic chunks
trustedge-client --server 127.0.0.1:8080 --test-chunks 100 \
  --key-hex <64-char-hex-key>
```

---

## Definition Snippets (v1)

* **AAD layout:** `header_hash (32) || seq (8, BE) || nonce (12) || blake3(manifest) (32)`
* **Preamble:** `MAGIC="TRST"`, `VERSION=1`
* **Envelope:** `StreamHeader { v, header (58), header_hash (32) }` then repeated
  `Record { seq, nonce, sm, ct }`
* **Manifest:** Includes `key_id` field for key rotation support
* **FileHeader:** 58 bytes with `key_id`, `device_id_hash`, `nonce_prefix`, and other metadata

---

## Success Criteria

* A third-party can implement a verifier from the spec + vectors and reach the same results.
* Tamper and misuse cases are reliably detected (and documented).
* Transport/storage backends are interchangeable without changing guarantees.
* Low-friction adoption: simple SDK, clear examples, deterministic tests.

---

## Legal & Attribution

**Copyright** © 2025 John Turner. All rights reserved.

**License**: This roadmap is licensed under the [Mozilla Public License 2.0 (MPL-2.0)](https://mozilla.org/MPL/2.0/).

**Project**: [TrustEdge](https://github.com/johnzilla/trustedge) — Privacy and trust at the edge.

**Contributing**: See project repository for contribution guidelines and milestone tracking.
