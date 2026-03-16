# Project Research Summary

**Project:** TrustEdge v2.1 — Data Lifecycle & Hardware Integration
**Domain:** Rust crypto workspace — archive decryption CLI, YubiKey CLI signing, named archive profiles
**Researched:** 2026-03-15
**Confidence:** HIGH

## Executive Summary

TrustEdge v2.1 completes the data lifecycle for the `.trst` archive format by adding three tightly-scoped features: `trst unwrap` (archive decryption and reassembly), YubiKey hardware signing in `trst wrap`/`trst verify`, and three named archive profiles (sensor, audio, log). All three features are implementable entirely within the existing dependency set — the workspace already contains XChaCha20Poly1305, Ed25519, BLAKE3, HKDF, the YubiKey PIV backend, and the `ProfileMetadata` enum extensibility point. The only candidate new dependency is `rpassword` (interactive PIN prompt for YubiKey), and only if ENV-variable PIN supply is insufficient. No breaking changes to the manifest format are required; the signature format extension (`"ecdsa-p256:<base64>"`) is additive alongside the existing `"ed25519:..."` path.

The recommended approach is to resolve key management first, then build profiles, then wire YubiKey. The hardcoded demo encryption key in `trst wrap` (`0123456789abcdef...`) is the single most dangerous open issue: `trst unwrap` cannot ship until wrap is fixed to use a real key. The recommended fix is HKDF-SHA256 derivation of the XChaCha20Poly1305 chunk key from the Ed25519 signing key (domain tag `"TRUSTEDGE_TRST_CHUNK_KEY"`), allowing a single `--device-key` file to serve both wrap and unwrap with no additional key management burden. This directly mirrors the v1.8 envelope KDF design and eliminates dual key management entirely. The YubiKey feature brings a distinct constraint: PIV hardware only supports ECDSA P-256, not Ed25519, so hardware-signed archives need a separate signature format alongside the existing Ed25519 path.

The primary risks are not cryptographic — the primitives are proven and already in the codebase. The risks are architectural: silently keeping the demo key, adding profile names without updating `validate()`, adding YubiKey backend depth before the CLI wire-up exists, and confusing the Envelope system's deterministic HKDF nonces with the `.trst` system's random per-chunk nonces (which are stored with the ciphertext). All six critical pitfalls are preventable with specific ordering of design decisions before code is written.

## Key Findings

### Recommended Stack

All three features are implementable with no new crate dependencies beyond a possible `rpassword` addition for interactive YubiKey PIN prompts. The stack is stable and does not require any version changes. The only `Cargo.toml` change needed is adding a `yubikey` feature flag to `trustedge-trst-cli` that gates on `trustedge-core/yubikey`, following the identical pattern used by `trustedge-platform`.

**Core technologies (unchanged):**
- `trustedge-core` — All crypto: XChaCha20Poly1305, Ed25519, BLAKE3, HKDF, YubiKey backend; everything needed is already here
- `trustedge-trst-protocols` — Archive format types, canonical serialization, profile metadata; extensibility point for new profiles
- `chacha20poly1305 = "0.10"` — Already in workspace; used for `.trst` chunk encryption (not AES-GCM — that is the Envelope system)
- `yubikey = "0.7"` — Already in core (feature-gated, no `untested` flag); ECDSA P-256 / RSA-2048 PIV signing fully implemented

**Possible addition (Phase 3 only):**
- `rpassword = "7.3"` — Interactive terminal PIN prompt without echo; single-purpose, minimal transitive deps; only needed if interactive PIN is in scope for YubiKey CLI

**Feature flag addition required:**
- `yubikey` feature in `trustedge-trst-cli/Cargo.toml` gating on `trustedge-core/yubikey` — follows the identical pattern already used by `trustedge-platform`

### Expected Features

All six v2.1 features are P1 priority. There are no ambiguous prioritization decisions.

**Must have (table stakes):**
- `trst unwrap` — decrypt and reassemble archive to output file; if wrap exists, unwrap must exist; asymmetric tools feel broken to operators
- `trst unwrap` verify-before-decrypt — validate signature and continuity chain before handing back any plaintext; never decrypt unauthenticated ciphertext
- `trst verify` accepts ECDSA P-256 public keys — required for YubiKey-signed archives to be usable at all
- `trst wrap --backend yubikey` — hardware-backed signing is the core value of YubiKey integration
- Named profile: `sensor` — typed `SensorMetadata` with sample_rate_hz, unit, sensor_type, labels; distinct from generic
- Named profile: `audio` — typed `AudioMetadata` with sample_rate_hz, bit_depth, channels, codec; aligned with existing `audio.rs` capture module
- Named profile: `log` — typed `LogMetadata` with log_level, source, format; lowest complexity of the three

**Should have (v2.1 if capacity):**
- HKDF key derived from signing key — eliminates dual key management; `trst wrap --device-key` and `trst unwrap --device-key` use the same file
- Distinct exit codes for unwrap failure modes (10 = wrong key/signature, 11 = corrupt chunk, 12 = invalid structure)
- `--yubikey-slot` flag — thin wrapper over existing `YubiKeyConfig.default_slot`; operators with multiple slots need this

**Defer (v2.x+):**
- `trst unwrap --key-hex` escape hatch — add after HKDF derivation proves sufficient
- Profile-specific post-decrypt validation — add when profiles are stable and consumers report gaps
- WASM unwrap — requires key delivery mechanism not yet designed
- Streaming unwrap — current archive sizes make full-load acceptable

**Anti-features (do not build):**
- Decrypt without verification — defeats the value proposition entirely
- Ed25519 on YubiKey — PIV hardware does not support it; the crate's `untested` feature flag is explicitly off-limits
- YubiKey as encryption backend — PIV does not expose symmetric crypto operations
- Separate per-chunk encryption keys — massive storage overhead, no practical security benefit given BLAKE3 continuity chain

### Architecture Approach

The codebase follows a monolith-core / thin-shell architecture: all crypto and archive logic lives in `trustedge-core`, and `trustedge-trst-cli` is pure argument parsing, file I/O, and output formatting. New code must respect this boundary. `decrypt_archive()` belongs in `trustedge-core::archive`, not in `handle_unwrap()`. `sign_manifest_ecdsa()` belongs in `trustedge-core::crypto`. The CLI handlers call core functions and format output. The YubiKey backend (`YubiKeyBackend` in `crates/core/src/backends/yubikey.rs`) is already fully implemented and tested — no changes needed there. The workspace build order flows from lowest-level types to CLI:

```
trustedge-trst-protocols (types) -> trustedge-core (crypto/archive) -> trustedge-trst-cli (CLI)
```

**Major components and v2.1 changes:**
1. `trustedge-trst-protocols` — Add `SensorMetadata`, `AudioMetadata`, `LogMetadata` structs; add three variants to `ProfileMetadata` enum; update `validate()` and `serialize_canonical()`
2. `trustedge-core::archive` — Add `decrypt_archive()` that calls `decrypt_segment` per chunk; keeps decryption logic in core, reusable by future WASM consumers
3. `trustedge-core::crypto` — Add `sign_manifest_ecdsa()` and `verify_manifest_ecdsa()` for ECDSA P-256 path alongside existing Ed25519 functions
4. `trustedge-trst-cli` — Add `Unwrap` subcommand; add `--backend [software|yubikey]` flag to `WrapCmd`/`VerifyCmd`; add profile-specific metadata flags; add `yubikey` feature gate

**Components unchanged:** `YubiKeyBackend`, `read_archive()`, `write_archive()`, `validate_archive()`, `TrstManifest` struct, `trustedge-platform` verify handler (operates on `serde_json::Value`, profile-agnostic), `trustedge-trst-wasm` (may need rebuild but no logic change)

### Critical Pitfalls

1. **Hardcoded demo key vs. real unwrap** — `trst wrap` uses `b"0123456789abcdef0123456789abcdef"` (line 287 of trst-cli/main.rs). Fix wrap's key derivation and implement unwrap in the same atomic phase. Resolve key management strategy as the first design decision before writing a single line of unwrap logic.

2. **Profile validation hard-rejects new profile names** — `TrstManifest::validate()` (line 367 of manifest.rs) hard-errors on anything other than `"generic"` or `"cam.video"`. Update `validate()` as the first code change in the named profiles phase. Run `cargo test -p trustedge-trst-protocols` before any CLI work.

3. **Canonical serialization breaks silently on new enum variants** — `serialize_canonical()` manually controls JSON field ordering. A new `ProfileMetadata` variant with wrong field order compiles and signs but fails verification. Write a fixture test pinning canonical JSON before adding any new enum variant.

4. **ECDSA P-256 vs Ed25519 signature format incompatibility** — YubiKey returns DER-encoded ECDSA bytes; `trst verify` accepts only `"ed25519:<base64>"` prefixed strings. Decide the manifest signature format (`"ecdsa-p256:<base64>"`) before writing any YubiKey CLI code. The acceptance criterion: `trst verify` exits 0 on a YubiKey-signed archive.

5. **Backend depth before CLI breadth** — Project history (v1.1 scorched-earth rewrite) shows a repeating pattern of building backend depth before any CLI exposes it to users. For YubiKey CLI, start from the CLI contract and work backward; do not add new `UniversalBackend` methods or `CryptoOperation` variants that are not called by the CLI in the same phase.

6. **YubiKey PIN not prompted** — `YubiKeyBackend::verify_pin()` requires PIN in config; if absent it returns a library-internal error. Implement PIN acquisition (stdin prompt via `rpassword` or `TRUSTEDGE_YUBIKEY_PIN` env variable) as part of the initial YubiKey CLI implementation, not as a follow-up.

## Implications for Roadmap

Based on combined research, three phases are recommended. They follow the codebase build order (protocols -> core -> CLI), address design decisions before code in each phase, and map directly to the pitfall checklist in PITFALLS.md.

### Phase 1: Named Archive Profiles

**Rationale:** `trustedge-trst-protocols` has no upstream workspace dependencies, making it the natural starting point. Profile types are pure data definitions — no runtime risk, no hardware dependency, no key management complexity. Completing this phase unblocks the profile-specific CLI argument work that overlaps with unwrap and YubiKey. Most critically, updating `validate()` here eliminates Pitfall 2 before any other work can accidentally create archives that hard-fail on verification.

**Delivers:** Three new typed `ProfileMetadata` variants (`Sensor`, `Audio`, `Log`) with validated structs, extended `validate()`, updated `serialize_canonical()`, canonical JSON fixture tests (pinned output), and CLI flags for profile-specific metadata fields in `trst wrap`.

**Addresses:** Named profile features (sensor, audio, log) from FEATURES.md

**Avoids:** Pitfall 2 (profile name rejection) and Pitfall 3 (canonical serializer divergence) — both resolved before CLI or cryptographic work begins

**Key design decision to lock first:** Whether to use typed enum variants (recommended for schema enforcement) or labeled strings in `GenericMetadata` (lower risk if `#[serde(untagged)]` disambiguation becomes complex). Each new variant must have at least one required field not present in `Generic` for unambiguous serde deserialization.

**Research flag:** Standard patterns — struct additions to an existing enum, serde serialization, well-documented. Does not need `/gsd:research-phase`. The `CamVideoMetadata` struct is a working template for the three new structs.

---

### Phase 2: Archive Decryption (`trst unwrap`)

**Rationale:** `trst unwrap` is self-contained (no hardware dependency, no manifest format change) and delivers the highest immediate user value — completing the data lifecycle symmetry. It depends on Phase 1 (so profile validation does not reject archives) but has no dependency on YubiKey. This ordering allows hardware-independent testing and a clean decryption implementation before the more complex format-change decisions required by YubiKey signing.

**Delivers:** Fixed `trst wrap` key derivation (HKDF-SHA256 from device signing key, domain tag `"TRUSTEDGE_TRST_CHUNK_KEY"`); `decrypt_archive()` in `trustedge-core::archive`; `trst unwrap` subcommand with verify-before-decrypt, plaintext output file, BLAKE3 hash re-verification after decryption, overwrite guard (`--force`), and distinct exit codes. Both wrap and unwrap updated atomically in the same phase.

**Addresses:** `trst unwrap` table stakes from FEATURES.md; key derivation design

**Avoids:** Pitfall 1 (hardcoded demo key) — must be resolved atomically; wrap and unwrap updated together

**Key design decision to lock first:** HKDF-derived key vs. explicit `--key` file. HKDF is recommended (matches Envelope v1.8 design, eliminates dual key management, uses domain separation). Note: XChaCha20Poly1305 nonces are random per-chunk at wrap time and stored alongside the ciphertext — they are not reconstructible and do NOT use the deterministic counter-nonce approach of the Envelope v2 system.

**Research flag:** Does not need `/gsd:research-phase`. HKDF derivation pattern is modeled on `envelope.rs` v1.8, already in the workspace. The key decision (HKDF vs. explicit file) should be confirmed in requirements since it affects backward compatibility with any v2.0 demo archives.

---

### Phase 3: YubiKey CLI Integration

**Rationale:** YubiKey integration is the most complex feature due to the Ed25519/ECDSA P-256 algorithm mismatch, signature format design decision, and hardware-dependent testing. Placed last because: (1) the YubiKey backend is already complete — only the CLI wire-up is missing; (2) the signature format extension to `trst verify` should not be in-flight during unwrap development; (3) hardware-dependent acceptance tests require a physical YubiKey device. Phases 1 and 2 can be fully validated before hardware testing begins.

**Delivers:** `--backend [software|yubikey]` flag on `trst wrap` and `trst verify`; `sign_manifest_ecdsa()` and `verify_manifest_ecdsa()` in `trustedge-core::crypto`; `"ecdsa-p256:<base64>"` signature format (additive alongside Ed25519); `yubikey` feature flag in `trustedge-trst-cli`; PIN acquisition via stdin prompt or `TRUSTEDGE_YUBIKEY_PIN`; fail-closed hardware-absent error (preserving existing `ensure_connected()` message); optional `--yubikey-slot` flag.

**Addresses:** YubiKey CLI signing and verify features from FEATURES.md

**Avoids:** Pitfall 4 (ECDSA/Ed25519 incompatibility — format decided before code), Pitfall 5 (PIN not prompted), Pitfall 6 (backend depth before CLI breadth)

**Key design decision to lock first:** Precise manifest signature format for ECDSA P-256. Recommendation: store as `"ecdsa-p256:<base64_der>"` in the existing `signature` field alongside a `signature_algorithm` discriminant, OR as a separate `ecdsa_signature` manifest field. Either approach must be locked in the plan before any implementation begins.

**Research flag:** Needs attention during planning. Confirm whether the `p256` crate (RustCrypto ecosystem) is needed for the verification-side ECDSA P-256 path, and check for version conflicts with existing workspace deps (`elliptic-curve`, `k256`). Low risk — well-maintained crate family — but verify before coding.

---

### Phase Ordering Rationale

- Protocols-first follows the workspace build order dependency graph (`trst-protocols` has no workspace deps; `trustedge-core` depends on it; `trst-cli` depends on core)
- Unwrap before YubiKey keeps hardware-independent features deployable early and avoids touching `trst verify`'s signature dispatch code twice (once for profiles, once for ECDSA)
- Each phase locks its critical design decision as the first task: profile variant strategy, key management strategy, signature format strategy — preventing the "looks done but isn't" failure modes in PITFALLS.md
- The ordering maps cleanly to the pitfall-to-phase table in PITFALLS.md: Phase 1 resolves pitfalls 2-3, Phase 2 resolves pitfall 1, Phase 3 resolves pitfalls 4-6

### Research Flags

Phases needing attention during planning (design decisions before code):
- **Phase 2 (unwrap):** Key management decision (HKDF vs. explicit file) must be confirmed in requirements; backward compatibility note for v2.0 demo archives that used the hardcoded key
- **Phase 3 (YubiKey):** Signature format for ECDSA P-256 in manifest must be specified precisely before implementation; confirm `p256` crate availability and workspace version compatibility

Phases with standard patterns (research-phase not needed):
- **Phase 1 (named profiles):** Struct additions and serde enum extension are well-documented Rust patterns; `CamVideoMetadata` is a working template; no external research needed

## Confidence Assessment

| Area | Confidence | Notes |
|------|------------|-------|
| Stack | HIGH | All conclusions from direct codebase inspection; zero new dependencies for two of three features; no external library version research needed |
| Features | HIGH | Feature list derived from codebase state and PROJECT.md requirements; anti-features clearly documented from v1.1 YubiKey constraints already established |
| Architecture | HIGH | Build order and component boundaries confirmed by reading actual source files; no inference required; existing YubiKey backend eliminates hardware uncertainty |
| Pitfalls | HIGH | All six critical pitfalls traceable to specific line numbers in the live codebase (line 287 demo key, line 367 profile allowlist, lines 227-291 canonical serializer) |

**Overall confidence:** HIGH

### Gaps to Address

- **`p256` crate for ECDSA verification:** The YubiKey backend signs with ECDSA P-256 but the workspace may not have a crate for verifying ECDSA P-256 signatures on the non-hardware side. During Phase 3 planning, confirm whether `p256` (RustCrypto ecosystem) is needed and check for version conflicts with existing `k256`/`elliptic-curve` workspace deps. Low risk, well-maintained crate family.

- **`rpassword` version validation:** Research notes v7.3 with medium confidence (training data). Validate with `cargo add rpassword --dry-run` during Phase 3 plan to confirm current version before committing it to `Cargo.toml`.

- **Backward compatibility of wrap key change:** When Phase 2 replaces the hardcoded demo key with HKDF derivation, any v2.0 demo archives become undecryptable with the new unwrap. Research treats this as acceptable given their demo status, but the requirements and CHANGELOG should explicitly document this breakage.

- **`#[serde(untagged)]` disambiguation:** If new profile variants have optional fields that overlap with `GenericMetadata`'s optional fields, serde will silently deserialize them as `Generic`. Phase 1 planning must lock the required field set for each new variant to ensure unambiguous disambiguation before any serialization code is written.

## Sources

### Primary (HIGH confidence)
- `crates/trst-cli/src/main.rs` — current command structure; hardcoded demo key at line 287; existing wrap/verify flow
- `crates/trst-protocols/src/archive/manifest.rs` — `ProfileMetadata` enum, `TrstManifest`, profile allowlist at line 367, canonical serializer dispatch at lines 227-291
- `crates/core/src/crypto.rs` — `decrypt_segment()`, `generate_aad()`, `DeviceKeypair`, `sign_manifest()`
- `crates/core/src/archive.rs` — `read_archive()`, `write_archive()`, `validate_archive()` signatures confirmed
- `crates/core/src/backends/yubikey.rs` — ECDSA P-256 confirmed, Ed25519 rejection documented at lines 291-298
- `crates/core/src/envelope.rs` — HKDF-SHA256 KDF pattern (v1.8) as model for chunk key derivation
- `crates/core/src/backends/universal.rs` — `SignatureAlgorithm` variants, `UniversalBackend` trait
- `.planning/PROJECT.md` — v2.1 goals, active requirements, out-of-scope constraints

### Secondary (MEDIUM confidence)
- `rpassword = "7.3"` — version from training data; confirm with `cargo add --dry-run` before use
- XChaCha20 nonce-prepend pattern — standard practice per libsodium/NaCl documentation; well-established

---
*Research completed: 2026-03-15*
*Ready for roadmap: yes*
