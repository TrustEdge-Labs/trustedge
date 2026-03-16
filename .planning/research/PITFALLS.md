# Pitfalls Research

**Domain:** Adding archive decryption (trst unwrap), YubiKey CLI integration, and named profiles to an existing Rust crypto workspace
**Researched:** 2026-03-15
**Confidence:** HIGH — all pitfalls drawn from direct code inspection of the live v2.1 codebase

---

## Critical Pitfalls

### Pitfall 1: The Hardcoded Demo Key Is the Decryption Target

**What goes wrong:**
`trst wrap` encrypts every archive using a hardcoded 32-byte key: `b"0123456789abcdef0123456789abcdef"` (line 287 of `crates/trst-cli/src/main.rs`). A naive `trst unwrap` implementation that attempts to use the device signing key, an HKDF-derived key, or any user-supplied key will fail cryptographically on every real archive produced by the demo. The comment in the code reads "32 bytes for demo" — but these are the only archives that exist today.

**Why it happens:**
`trst wrap` was written during v2.0 with a deferred key management strategy. The demo needed working encryption for the wrap-verify flow; decryption (unwrap) was not in scope. The placeholder was rational at the time. Now that unwrap is the goal, the key management must be resolved first or all progress on unwrap will be built on top of a compatibility break.

**How to avoid:**
Before writing any unwrap logic, define and implement the real key management strategy for wrap. The two realistic options are: (a) derive the symmetric key from the device signing key via HKDF, binding it to the device identity; (b) accept a user-supplied key file via `--key` flag with `trst wrap` and require the same key for `trst unwrap`. Whichever is chosen, `wrap` and `unwrap` must use the same derivation in a single atomic commit — they must never diverge. Note: the encryption algorithm in use is **XChaCha20Poly1305** (not AES-256-GCM; that is the `Envelope` system in `trustedge-core`). Nonces are random per-chunk and stored in the archive alongside the ciphertext — they cannot be reconstructed deterministically.

**Warning signs:**
- Any `trst unwrap` test that passes only against archives created in the same test run (not against existing demo archives from v2.0)
- An unwrap implementation merged without a simultaneous update to wrap's key derivation

**Phase to address:**
Archive decryption phase — must be the first decision made, before writing a single line of unwrap logic.

---

### Pitfall 2: Profile Validation Hard-Rejects "sensor", "audio", "log"

**What goes wrong:**
`TrstManifest::validate()` at line 367 of `crates/trst-protocols/src/archive/manifest.rs` explicitly rejects any profile that is not `"generic"` or `"cam.video"`:

```
if self.profile != "generic" && self.profile != "cam.video" {
    return Err(ManifestFormatError::InvalidField(format!(
        "profile must be 'generic' or 'cam.video', got '{}'",
        self.profile
    )));
}
```

Adding `--profile sensor` to `trst wrap` will produce archives that fail `validate()` everywhere it is called — local `trst verify`, the `validate_archive()` function in core, and the platform HTTP verify endpoint.

**Why it happens:**
The allowlist was correct when only two profiles existed. The named profile feature adds new profile names that must be explicitly admitted, or validation must be restructured to allow any non-empty profile string.

**How to avoid:**
Decide the naming strategy before writing profile code. Two paths: (1) named profiles ("sensor", "audio", "log") as distinct `ProfileMetadata` enum variants with their own metadata structs — requires updating the enum, `serialize_canonical()`, `validate()`, and `#[serde(untagged)]` deserialization; (2) named profiles as string values of the `profile` field that reuse the `Generic` metadata schema — requires only updating `validate()` to admit additional names. Path (2) has dramatically lower risk. Either way, update validation as the first step and run `cargo test -p trustedge-trst-protocols` before touching the CLI.

**Warning signs:**
- `trst verify` returning exit code 12 ("Validation error") on sensor/audio/log archives
- Tests that create archives with new profile names but don't call `validate()`

**Phase to address:**
Named profiles phase — validation must be updated before any CLI flag or archive creation code.

---

### Pitfall 3: Adding a ProfileMetadata Enum Variant Silently Breaks Canonical Serialization

**What goes wrong:**
`TrstManifest::serialize_canonical()` manually dispatches on `ProfileMetadata` variants using a match expression with hard-coded field order. The match at line 227 of `manifest.rs` handles `CamVideo` and `Generic`. If a new variant (e.g., `ProfileMetadata::Sensor`) is added, the compiler enforces exhaustiveness — but getting the field order wrong in the new arm causes canonical serialization to diverge from serde's output. Signatures on archives created with the new profile will fail verification because `to_canonical_bytes()` and serde produce different JSON.

**Why it happens:**
The canonical serializer exists to produce deterministic JSON with controlled field ordering (serde's output order is not guaranteed across library versions). It duplicates schema knowledge from the struct definitions. Every new metadata variant requires a manually-correct match arm in two places: the struct and the serializer. The failure mode is silent: serialization succeeds, signing succeeds, but verification fails.

**How to avoid:**
When adding a new `ProfileMetadata` variant, write a fixture test before implementing the match arm: serialize a hand-crafted manifest with the new profile using both `to_canonical_bytes()` and `serde_json::to_string()`, and assert the fields appear in the expected order. A simpler approach: name the profile via the string field without adding a new enum variant (reuse `Generic` metadata), which eliminates this risk entirely.

**Warning signs:**
- `trst verify` returning "Signature verification failed" on archives you just created in the same binary build
- Tests that create and immediately verify an archive pass, but verification of archives from a previous build fails

**Phase to address:**
Named profiles phase — write the fixture test before adding any new enum variant.

---

### Pitfall 4: YubiKey Signs ECDSA P-256 (DER Bytes); trst Verifies Ed25519 (Base64 String)

**What goes wrong:**
The YubiKey backend in `crates/core/src/backends/yubikey.rs` produces ECDSA P-256 signatures as raw `Vec<u8>` (DER-encoded). The `trst` archive system uses Ed25519 signing via `DeviceKeypair`, where signatures are stored as `"ed25519:<base64>"` formatted strings and verified with `verify_manifest()`. These two systems are incompatible at three levels: algorithm (ECDSA vs Ed25519), wire format (DER bytes vs prefixed base64 string), and verification code path. Adding `--backend yubikey` to `trst wrap` without bridging this gap will produce archives whose signatures cannot be verified by any existing `trst verify` invocation.

**Why it happens:**
The YubiKey backend was built for the Universal Backend system, which is algorithm-agnostic and returns raw bytes. The `trst` archive system was built with a hardcoded Ed25519 assumption. The incompatibility was explicitly documented in the YubiKey backend since v1.1: "Ed25519 is NOT supported by YubiKey PIV hardware." The v2.1 milestone is the first time these two systems need to interoperate.

**How to avoid:**
Before writing any YubiKey CLI integration code, decide on the manifest signature format for hardware-signed archives. The options are: (a) add an `ecdsa_signature` field alongside the existing `signature` field and extend `trst verify` to check whichever is present; (b) add a `signature_algorithm` field and dispatch on it in the verify path; (c) convert the ECDSA signature to a deterministic format that fits in the existing `signature` field with a new prefix (e.g., `"ecdsa256:<base64>"`). Any of these requires coordinated changes to `trst wrap`, `TrstManifest`, and `trst verify`. The decision must precede implementation.

**Warning signs:**
- Any YubiKey wrap test that "passes" without calling `trst verify` on the resulting archive
- `trst verify` returning "pass" for a YubiKey-signed archive when the device public key was actually an Ed25519 key

**Phase to address:**
YubiKey CLI integration phase — signature format decision is the first design step.

---

### Pitfall 5: YubiKey PIN Not Prompted in CLI Context

**What goes wrong:**
`YubiKeyBackend::verify_pin()` requires the PIN to be set in `YubiKeyConfig` before any signing operation. If the PIN is absent, it returns `BackendError::HardwareError("PIN not configured. Set YubiKeyConfig.pin before operations.")`. A CLI that passes this error through unmodified gives the user an internal library error message. If instead the CLI accepts a `--yubikey-pin` flag, the PIN appears in `ps` output, shell history, and log files.

**Why it happens:**
The backend was designed as a library component. PIN acquisition is the caller's responsibility. The CLI layer does not yet exist and has not been designed.

**How to avoid:**
Implement PIN acquisition through a stdin prompt (using `rpassword` or an equivalent) when `--backend yubikey` is specified and no pin flag is given. The `--yubikey-pin` flag should exist for scripted/CI use but must be documented as unsafe in interactive sessions. Never accept the PIN as a positional argument.

**Warning signs:**
- Demo scripts or CI jobs that pass the PIN as a shell argument
- Users reporting "PIN not configured" with no explanation of what to do

**Phase to address:**
YubiKey CLI integration phase — PIN acquisition must be part of the initial implementation, not a follow-up.

---

### Pitfall 6: Building Backend Depth Before CLI Breadth (Repeat of v1.1 Pattern)

**What goes wrong:**
The project history shows a clear pattern: the YubiKey backend was built, then entirely rewritten (v1.1 scorched-earth), all before any CLI exposed it to users. The result was 400+ LOC of backend code that zero users could actually invoke. The `--backend yubikey` CLI flag has never existed. For v2.1, the same risk applies: spending a phase deepening the backend (new operation types, new configuration options, new error variants) before the CLI wire-up exists.

**Why it happens:**
Backend-first development feels safe because backends are unit-testable in isolation. The CLI wire-up feels like "glue code" but is where the real complexity lives: argument parsing, error presentation, output formatting, PIN handling, and signature format decisions.

**How to avoid:**
Start from the CLI contract: write `trst wrap --backend yubikey --slot 9c sample.bin archive.trst` and make it produce a valid, verifiable archive. Work backward from that contract to whatever backend changes are needed. Do not add new `UniversalBackend` methods, new `CryptoOperation` variants, or new configuration fields that are not called by the CLI within the same phase.

**Warning signs:**
- More than 2 days of backend work before the CLI subcommand argument exists
- New backend methods at phase end that have no CLI entry point

**Phase to address:**
YubiKey CLI integration phase — wire CLI first, fill in backend as required.

---

## Technical Debt Patterns

| Shortcut | Immediate Benefit | Long-term Cost | When Acceptable |
|----------|-------------------|----------------|-----------------|
| Keep hardcoded demo key during unwrap implementation | Fast initial test of unwrap logic | v2.0 demo archives cannot be decrypted; wrap and unwrap permanently diverge | Never — must be resolved atomically with unwrap |
| Named profiles as string values of `profile` field (not new enum variants) | Avoids canonical serializer changes, lowest risk | Profile-specific validation must live in application layer | Acceptable for v2.1 if GenericMetadata fields are sufficient |
| Accept ECDSA signature alongside Ed25519 in manifest | Allows YubiKey signing without breaking existing format | Dual-format complexity in verify path, manifest schema grows | Acceptable if both paths have acceptance test coverage |
| Stdin PIN prompt rather than OS secret store integration | Simple implementation, no new dependencies | User must re-enter PIN each invocation; not composable in pipelines | Acceptable for v2.1 |
| Skip BLAKE3 hash re-verification during unwrap | Faster implementation | Corrupt archives accepted as valid; silent data corruption | Never |

---

## Integration Gotchas

| Integration | Common Mistake | Correct Approach |
|-------------|----------------|------------------|
| XChaCha20Poly1305 nonces in .trst chunks | Trying to reconstruct nonces deterministically (confusing with Envelope v2) | Nonces are random-per-chunk at wrap time, stored in the archive — read from the chunk file on disk |
| YubiKey `piv_sign` return value | Treating the `Vec<u8>` DER signature as a base64 string or prepending `ed25519:` | The return is a DER-encoded ECDSA signature; store in a manifest field separate from `signature` |
| `ProfileMetadata` with `#[serde(untagged)]` | Adding a new variant whose optional fields match Generic's optional fields | Untagged deserialization is order-dependent; CamVideo succeeds first because it has required fields (timezone, fps, resolution, codec) that Generic lacks — a new variant with only optional fields deserializes as Generic |
| `generate_aad()` binds to profile string | Normalizing profile names after archive creation (e.g., `"cam.video"` -> `"camvideo"`) | AAD is bound to the exact profile string at wrap time; any post-creation normalization breaks the auth tag |
| YubiKey `ensure_connected()` | Checking hardware presence at construction time and skipping during operations | `ensure_connected()` must be called before every signing operation; hardware state can change between construction and use |

---

## Performance Traps

| Trap | Symptoms | Prevention | When It Breaks |
|------|----------|------------|----------------|
| Holding `Mutex<YubiKey>` lock during slow PIN prompt | CLI hangs indefinitely waiting for user input while holding the hardware lock | Acquire PIN string before locking the Mutex; pass PIN bytes to `verify_pin` within the locked section | Immediately on first interactive use |
| Reading all chunks into memory before decrypting during unwrap | OOM on archives with 100+ 1MB chunks (sensor data, long audio) | Stream chunk-by-chunk: read, decrypt, write output, drop; never accumulate decrypted data | Archives larger than half available RAM |
| Re-deriving HKDF key material per-chunk if key derivation is added to unwrap | Noticeable latency on large archives | Derive key material once per archive, pass to per-chunk decryption | Not a concern at current archive sizes but establishes a bad pattern |

---

## Security Mistakes

| Mistake | Risk | Prevention |
|---------|------|------------|
| Accepting `--yubikey-pin` as a positional argument | PIN in shell history, `ps` output, CI logs | Require `--yubikey-pin` to be an option flag, document it as scripted-use-only, use stdin prompt by default |
| Writing decrypted output to the archive directory without `--out` flag | Accidental plaintext next to ciphertext | Require explicit `--out` path for `trst unwrap`; refuse to write to the source directory |
| Not validating BLAKE3 hashes of decrypted chunks | Silent corruption accepted; attacker can swap chunk content if they can edit the archive and control plaintext | After decrypting each chunk, recompute BLAKE3 over the plaintext and compare to `segment.blake3_hash` in the manifest |
| Signature format fallback that silently accepts unsigned archives | An archive with no signature field passes verification | Fail closed: if `--backend yubikey` was used at wrap time and the manifest has no ECDSA signature, verification must fail |
| Using `Default::default()` for `YubiKeyConfig` in CLI without PIN | No PIN = `verify_pin` errors on every operation | Require PIN acquisition before constructing the backend when `--backend yubikey` is specified |

---

## UX Pitfalls

| Pitfall | User Impact | Better Approach |
|---------|-------------|-----------------|
| `trst unwrap` exits with generic "decryption failed" | User cannot distinguish wrong key from corrupt archive from wrong algorithm | Use distinct exit codes matching `trst verify` conventions: 10 = wrong key/signature, 11 = corrupt chunk (hash mismatch), 12 = invalid archive structure |
| `trst wrap --backend yubikey` silently falls back to software key when YubiKey absent | User believes archive is hardware-signed when it is not | Fail immediately with nonzero exit and clear message: "YubiKey not connected. Insert device and retry." |
| `trst unwrap` overwrites an existing output file without warning | Data loss on re-runs | Refuse to overwrite unless `--force` is passed; mirror the behavior of `trst keygen` |
| Named profile `--profile sensor` accepted but sensor-specific fields not surfaced | User wraps non-sensor data or omits useful metadata with no feedback | Emit a warning (not an error) if the expected profile-specific fields (e.g., `--sample-rate` for audio, `--sensor-type` for sensor) are absent |

---

## "Looks Done But Isn't" Checklist

- [ ] **trst unwrap:** Often missing BLAKE3 hash re-verification after decryption — verify `segment.blake3_hash` in the manifest matches BLAKE3 of the decrypted plaintext, not the ciphertext
- [ ] **trst unwrap:** Often missing output file overwrite guard — verify CLI refuses to clobber an existing `--out` path
- [ ] **trst unwrap:** Often tested only against archives created in the same test run — verify it can decrypt a v2.0 demo archive (requires resolving the hardcoded key first)
- [ ] **YubiKey wrap:** Often missing signature format decision — verify `trst verify` exits 0 on an archive produced by `trst wrap --backend yubikey` using the documented public key flag
- [ ] **YubiKey wrap:** Often missing hardware-absent error test — verify the CLI exits nonzero with a human-readable message when YubiKey is not plugged in
- [ ] **Named profiles:** Often missing validation update — verify `trst verify` does not return exit 12 on a `--profile sensor` archive
- [ ] **Named profiles:** Often missing canonical serialization fixture — verify a pinned-output test exists and fails if canonical JSON changes
- [ ] **All three features:** Often missing end-to-end acceptance test — verify `cargo test -p trustedge-trst-cli --test acceptance` covers at least one roundtrip (wrap + verify + unwrap) for each new feature

---

## Recovery Strategies

| Pitfall | Recovery Cost | Recovery Steps |
|---------|---------------|----------------|
| Hardcoded key divergence between wrap and unwrap | HIGH — breaks all existing archives | Define key management strategy, update wrap and unwrap atomically, note in CHANGELOG that archives from before this version require re-wrapping |
| Profile validation rejecting new profile names | LOW — one-line fix | Add profile names to the allowlist in `validate()`, re-run `cargo test -p trustedge-trst-protocols` |
| Canonical serializer mismatch for new profile variant | MEDIUM — signature failure on newly-created archives | Write fixture test pinning canonical JSON, fix field order in the serializer match arm, re-create test archives |
| YubiKey signature format incompatibility discovered post-implementation | HIGH — requires manifest format change | Add `signature_algorithm` field to `TrstManifest`, extend verify to dispatch on it, update acceptance tests, semver consideration |

---

## Pitfall-to-Phase Mapping

| Pitfall | Prevention Phase | Verification |
|---------|------------------|--------------|
| Hardcoded demo key vs. unwrap decryption | Archive decryption phase (first decision) | `trst unwrap` succeeds on an archive created by `trst wrap` in a separate invocation |
| Profile validation rejects new names | Named profiles phase (first code change) | `cargo test -p trustedge-trst-protocols` passes with `profile: "sensor"` manifest |
| Canonical serializer breaks on new variant | Named profiles phase | Fixture test pins canonical JSON for each new profile; two runs produce identical output |
| ECDSA/Ed25519 signature format incompatibility | YubiKey CLI integration phase (design step before code) | `trst verify` exits 0 on a YubiKey-signed archive using the hardware public key |
| PIN not prompted in CLI context | YubiKey CLI integration phase | Interactive test: `trst wrap --backend yubikey` without `--yubikey-pin` prompts before attempting hardware operation |
| Backend depth before CLI breadth | YubiKey CLI integration phase (guard) | Every new backend method added in this phase is called by a CLI path in the same phase |

---

## Sources

- Direct code inspection: `crates/trst-cli/src/main.rs` line 287 — hardcoded key `b"0123456789abcdef0123456789abcdef"`
- Direct code inspection: `crates/trst-protocols/src/archive/manifest.rs` lines 367-373 — profile name allowlist; lines 227-291 — manual canonical serializer dispatch
- Direct code inspection: `crates/core/src/backends/yubikey.rs` lines 291-298 — Ed25519 rejection; lines 300-305 — ECDSA P-256 DER signing output
- Direct code inspection: `crates/core/src/envelope.rs` — v2 HKDF+AES-256-GCM system (distinct from .trst XChaCha20Poly1305 system)
- Direct code inspection: `crates/core/src/crypto.rs` lines 148-190 — `encrypt_segment`/`decrypt_segment` use XChaCha20Poly1305 with 24-byte nonces
- Project history: `.planning/PROJECT.md` Key Decisions table — "ECDSA P-256 only for certs — Simplicity for initial release"; "Key generation and attestation deferred to future"
- Project history: MEMORY.md — "Key generation and attestation deferred to future (yubikey crate API limitations)"; v1.1 scorched-earth rewrite pattern

---
*Pitfalls research for: TrustEdge v2.1 — archive decryption, YubiKey CLI integration, named profiles*
*Researched: 2026-03-15*
