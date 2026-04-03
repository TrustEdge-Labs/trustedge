<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


# Feature Research: Data Lifecycle & Hardware Integration

**Domain:** Edge data archive — decryption/unwrap, YubiKey CLI signing, named archive profiles
**Researched:** 2026-03-15
**Confidence:** HIGH (derived from direct codebase analysis)

## Context: Existing Baseline

Already shipped (do NOT rebuild):
- `trst keygen` — Ed25519 keypair generation
- `trst wrap` — encrypt + sign + archive to .trst directory
- `trst verify` — signature + continuity chain verification
- `trst emit-request` — build and POST VerifyRequest to platform
- ProfileMetadata enum: `Generic` and `CamVideo` variants
- YubiKey PIV backend in trustedge-core (`--features yubikey`): ECDSA P-256, RSA-2048 signing

Critical implementation detail: `trst wrap` currently uses a **hardcoded 32-byte demo encryption key**
(`0123456789abcdef0123456789abcdef`). The key is NOT stored in the archive. This is the central
design constraint for `trst unwrap`.

## Feature Landscape

### Table Stakes (Users Expect These)

Features users assume exist. Missing these = product feels incomplete.

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| **`trst unwrap` — decrypt and reassemble** | If wrap exists, unwrap must exist. Asymmetric tools (only wrap, no unwrap) feel broken to any operator needing to recover data | HIGH | Core complexity is key management: where does the decryption key come from? Archive stores no key. XChaCha20Poly1305 decrypt_segment() already exists in core. |
| **`trst unwrap` — output file recovery** | Archive-wrapped data must be recoverable as original bytes | MEDIUM | Read manifest, decrypt each chunk in segment order, concatenate plaintext, write to output file |
| **`trst unwrap` — verify before decrypt** | Users expect unwrap to confirm data hasn't been tampered with before handing back plaintext | MEDIUM | Must call validate_archive() + verify_manifest() before any decryption. Never decrypt unauthenticated ciphertext. |
| **Consistent key argument across commands** | wrap uses `--device-key`, unwrap should accept matching argument so the same key file works for both operations | LOW | Naming and UX consistency. Key file produced by `trst keygen` must work with both wrap and unwrap |
| **YubiKey as signing backend in `trst wrap`** | Users with hardware keys expect to sign archives with their YubiKey, not just a software file key | HIGH | YubiKey PIV does NOT support Ed25519 — signatures will be ECDSA P-256. Manifest format stores the public key as a string; the verify path must accept P-256 as well as Ed25519 |
| **`--backend yubikey` flag on wrap/verify** | Hardware signing must be opt-in via explicit flag, not automatic or implicit | LOW | Guarded by `#[cfg(feature = "yubikey")]` compile-time and `--backend` runtime flag |
| **Named profile: `sensor`** | Sensor data (temperature, pressure, GPS, accelerometer) has structured metadata distinct from generic: sample rate, unit, sensor model | MEDIUM | New `SensorMetadata` struct + new `ProfileMetadata::Sensor` enum variant. Requires extending `serialize_canonical()` and `validate()` in trst-protocols |
| **Named profile: `audio`** | Audio capture has specific metadata: sample rate (Hz), bit depth, channel count, codec — all expected by audio tooling consumers | MEDIUM | New `AudioMetadata` struct. Note: trustedge-core already has `audio.rs` for live capture, so this profile should align with that module's capabilities |
| **Named profile: `log`** | Log files have distinct metadata: log level, application name, host — different enough from generic to justify its own profile | LOW | New `LogMetadata` struct. Lower complexity than sensor/audio because no hardware-specific fields |

### Differentiators (Competitive Advantage)

Features that set the product apart. Not required, but valuable.

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| **Decrypt-then-verify exit codes** | Operator tooling needs machine-readable outcomes. Exit code 0 = decrypted OK, exit code 10 = signature fail (data not decrypted), exit code 11 = continuity fail | LOW | Mirrors the exit code pattern already in `trst verify`. Consistent, scriptable |
| **Symmetric key derived from signing key** | If decryption key is derived deterministically from the Ed25519 signing key (e.g., HKDF over the raw secret key bytes), the same key file used to wrap can also unwrap — zero extra key management burden on the user | MEDIUM | Elegant: `trst unwrap --device-key device.key` just works. Requires documenting the derivation so future implementations can reproduce it. Risk: exposing the signing key to derive the encryption key — acceptable if derivation uses domain-separated HKDF |
| **Profile-specific validation in unwrap** | After decryption, validate profile-specific metadata fields (e.g., sensor sample rate > 0, audio bit depth in {8,16,24,32}). Catch corrupt archives early. | LOW | Small validator per profile variant, run after unwrap succeeds |
| **YubiKey slot selection flag** | `--yubikey-slot 9c` (default) / `9a` / `9d` — operators with multiple slots can choose which key to use for signing | LOW | Thin CLI wrapper over the existing YubiKeyConfig.default_slot field |

### Anti-Features (Commonly Requested, Often Problematic)

Features that seem good but create problems.

| Feature | Why Requested | Why Problematic | Alternative |
|---------|---------------|-----------------|-------------|
| **Decrypt without verification** | "I just need the bytes back, skip the signature check" | Decrypts potentially tampered data. Attacker can replace chunks and retrieve them. Defeats the entire value proposition. | Always verify before decrypt. If the signature key is lost, that's an operational problem, not a reason to skip verification |
| **Separate encryption key file** | "Store the encryption key separately from the signing key" | Doubles key management burden. Two files to track, two to protect, two to lose. Most operators lose one | Derive encryption key from signing key via HKDF. One key file, two purposes, zero extra burden |
| **Per-chunk encryption keys** | "Each chunk gets its own key for maximum isolation" | Massive key storage overhead in the manifest. Breaks the ability to derive from a single source key. No practical security benefit since chunks are already individually authenticated via BLAKE3 + continuity chain | Use a single derived key with deterministic counter nonces — already the pattern used by the envelope KDF (v1.8) |
| **YubiKey as encryption backend** | "Use YubiKey to encrypt data" | YubiKey PIV does not expose symmetric crypto. Attempting this would require awkward workarounds. The docs explicitly state: YubiKey for signing/attestation only | Software XChaCha20Poly1305 for encryption, YubiKey for signing only |
| **Ed25519 on YubiKey** | "Use the same Ed25519 key format for YubiKey signing" | YubiKey PIV hardware does NOT support Ed25519. The yubikey crate (stable API) supports ECDSA P-256 and RSA-2048 only. Attempting Ed25519 on YubiKey silently fails or requires the `untested` feature flag (which we explicitly avoid) | Use ECDSA P-256 for YubiKey-backed signing. Store public key in manifest as "p256:BASE64" format |
| **Generic profile as named profile catch-all** | "If we make generic flexible enough, we don't need sensor/audio/log" | Generic works for unknown types. But sensor/audio/log have well-defined schemas — using generic forces callers to pass freeform strings and loses validation. Schema-defined profiles enable downstream tooling to parse confidently | Generic remains the fallback for truly arbitrary data. Named profiles for well-understood data types |

## Feature Dependencies

```
[trst unwrap]
    └──requires──> [Key available for decryption]
                       └──option A──> [Derive from signing key via HKDF] (recommended)
                       └──option B──> [User supplies --key-hex] (escape hatch)
    └──requires──> [validate_archive() passes]
    └──requires──> [verify_manifest() passes]
    └──requires──> [decrypt_segment() — already exists in trustedge-core::crypto]

[YubiKey CLI signing in trst wrap]
    └──requires──> [YubiKeyBackend in trustedge-core] (ALREADY BUILT, v1.1)
    └──requires──> [ECDSA P-256 public key format in DeviceInfo.public_key] (format change)
    └──requires──> [verify_manifest() accepts P-256 signatures] (currently Ed25519-only)
    └──optional──> [--yubikey-slot flag] (enhances usability)

[Named profiles: sensor, audio, log]
    └──requires──> [New metadata structs in trst-protocols] (SensorMetadata, AudioMetadata, LogMetadata)
    └──requires──> [New ProfileMetadata enum variants] (Sensor, Audio, Log)
    └──requires──> [Extended serialize_canonical() in trst-protocols] (one branch per new variant)
    └──requires──> [Extended validate() to accept new profile strings]
    └──requires──> [New CLI flags in trst wrap for profile-specific fields]
    └──enhances──> [trst unwrap profile validation] (post-decrypt schema check)

[Sensor profile] ──independent──> [Audio profile] ──independent──> [Log profile]
    (all three can be built in parallel, no interdependency)

[ECDSA P-256 public key format] ──conflicts──> [Ed25519-only verify path]
    (must extend verify_manifest to dispatch on key type prefix)
```

### Dependency Notes

- **trst unwrap requires key**: The hardcoded demo key in wrap (`0123456789abcdef...`) is plainly not suitable for production use. The unwrap feature cannot ship without resolving key management. Recommended: HKDF over the Ed25519 secret key bytes with domain tag "TRUSTEDGE_TRST_CHUNK_KEY". This allows `--device-key device.key` to serve as both signing and decryption input.
- **YubiKey signing requires signature format extension**: `verify_manifest()` currently accepts only "ed25519:" prefixed public keys. P-256 signatures from YubiKey would be in ECDSA format. The manifest's `device.public_key` field stores the key as a string — a "p256:" prefix convention would allow dispatch without schema changes to TrstManifest.
- **Named profiles require trst-protocols changes**: The WASM-compatible trst-protocols crate is the single source of truth for manifest types. Any new profile adds a new struct and enum variant there first, then the CLI and canonical serializer follow.

## MVP Definition

### Launch With (v2.1)

Minimum viable product — what's needed to complete the data lifecycle milestone.

- [ ] **`trst unwrap --device-key` ** — Decrypt and reassemble archive to output file using signing key as decryption key source (HKDF-derived). Exit codes matching verify pattern.
- [ ] **`trst wrap --backend yubikey --yubikey-slot 9c`** — Sign archive with YubiKey ECDSA P-256. Verify path must accept P-256 public keys.
- [ ] **`trst verify` accepts P-256 public keys** — Extends verify_manifest() to dispatch on key type prefix. Prerequisite for YubiKey wrap to be useful.
- [ ] **Named profile: `sensor`** — SensorMetadata with sample_rate_hz, unit, sensor_model, labels. New variant in ProfileMetadata.
- [ ] **Named profile: `audio`** — AudioMetadata with sample_rate_hz, bit_depth, channels, codec. Aligns with existing audio.rs capture module.
- [ ] **Named profile: `log`** — LogMetadata with log_level, application, host. Simplest of the three named profiles.

### Add After Validation (v2.x)

Features to add once core is working.

- [ ] **`trst unwrap --key-hex`** — Escape hatch for users who need to supply a raw hex key directly (e.g., key was stored separately). Add after HKDF derivation approach proves sufficient for most users.
- [ ] **Profile-specific validation in unwrap** — Post-decrypt semantic checks (sample rate > 0 etc.). Add when profiles are stable and consumers report validation gaps.

### Future Consideration (v3+)

Features to defer until product-market fit is established.

- [ ] **WASM unwrap** — Browser-side decryption via trst-wasm. Non-trivial because the WASM environment cannot access device key files. Requires a key delivery mechanism.
- [ ] **Streaming unwrap** — For very large archives (>1GB), chunk-by-chunk decryption without loading all chunks into memory. Current target archives are small enough that full-load is fine.

## Feature Prioritization Matrix

| Feature | User Value | Implementation Cost | Priority |
|---------|------------|---------------------|----------|
| `trst unwrap` (decrypt + reassemble) | HIGH | HIGH | P1 |
| Key derivation design (HKDF from signing key) | HIGH | MEDIUM | P1 |
| Verify before decrypt (always) | HIGH | LOW | P1 |
| `trst verify` accepts P-256 public keys | HIGH | MEDIUM | P1 |
| `trst wrap --backend yubikey` | HIGH | MEDIUM | P1 |
| Named profile: sensor | MEDIUM | MEDIUM | P1 |
| Named profile: audio | MEDIUM | MEDIUM | P1 |
| Named profile: log | MEDIUM | LOW | P1 |
| Exit codes for unwrap | MEDIUM | LOW | P1 |
| --yubikey-slot flag | LOW | LOW | P2 |
| Profile-specific post-decrypt validation | LOW | LOW | P2 |
| `trst unwrap --key-hex` escape hatch | LOW | LOW | P2 |

**Priority key:**
- P1: Must have for v2.1 launch
- P2: Should have, add when possible
- P3: Nice to have, future consideration

## Implementation Notes by Feature

### trst unwrap — Key Management Decision

The hardcoded demo key is the central problem to solve before any other unwrap work can proceed.
**Recommended approach:** Derive the XChaCha20Poly1305 chunk encryption key from the Ed25519 signing key
using HKDF-SHA256 (already in workspace via hkdf crate, used in envelope.rs since v1.8).

```
input_key_material = Ed25519 secret key bytes (32 bytes)
salt = archive_id or empty (no salt acceptable for HKDF with high-entropy IKM)
info = b"TRUSTEDGE_TRST_CHUNK_KEY"
output = 32-byte XChaCha20Poly1305 key
```

This makes `trst wrap --device-key device.key` and `trst unwrap --device-key device.key` use
the same file with no additional storage or communication needed. The approach follows the same
domain-separation pattern established in v1.8's envelope key derivation.

The `generate_aad()` function in crypto.rs takes (version, profile, device_id, started_at) — unwrap
must reconstruct the same AAD from the manifest fields for each chunk to authenticate decryption.

### YubiKey CLI — Signature Format

The existing DeviceKeypair struct is software-only (Ed25519). A YubiKey-backed signing path needs:
1. A new abstraction that either: (a) returns the ECDSA P-256 signature bytes from YubiKeyBackend.perform_operation(Sign), or (b) introduces a `TrstSigner` trait dispatching on --backend
2. The public key stored in manifest.device.public_key needs a "p256:BASE64" prefix when YubiKey is used
3. `verify_manifest()` currently uses `DeviceKeypair::from_public()` which only handles "ed25519:" — must add ECDSA P-256 dispatch

**Important:** YubiKey PIV slots sign pre-hashed digests. The wrap command must hash the canonical manifest bytes with SHA-256 before passing to YubiKeyBackend. The existing YubiKey backend already handles this (uses Sha256 internally before calling PIV sign).

### Named Profiles — Where to Add Code

All profile types live in `crates/trst-protocols/src/archive/manifest.rs`. The changes are:
1. New metadata structs (SensorMetadata, AudioMetadata, LogMetadata) — same pattern as CamVideoMetadata
2. New variants in ProfileMetadata enum
3. New match arms in `serialize_canonical()` — must maintain deterministic field ordering
4. Extended `validate()` to accept the new profile strings (currently hard-errors on anything other than "generic" or "cam.video")
5. New CLI flags in trst-cli main.rs and new match arms in the profile dispatch block in handle_wrap()

The WASM crate (trst-wasm) imports from trst-protocols — new profile types will be available there automatically since they're just struct/enum additions.

## Sources

- `/home/john/vault/projects/github.com/trustedge/crates/trst-cli/src/main.rs` — Full CLI source, hardcoded encryption key found at line 287, existing command structure (HIGH confidence)
- `/home/john/vault/projects/github.com/trustedge/crates/trst-protocols/src/archive/manifest.rs` — ProfileMetadata enum, TrstManifest, validate() hard-profile-check at line 367 (HIGH confidence)
- `/home/john/vault/projects/github.com/trustedge/crates/core/src/crypto.rs` — decrypt_segment() and generate_aad() already exist and work (HIGH confidence)
- `/home/john/vault/projects/github.com/trustedge/crates/core/src/archive.rs` — read_archive(), validate_archive() provide the verification primitives needed before decryption (HIGH confidence)
- `/home/john/vault/projects/github.com/trustedge/crates/core/src/backends/yubikey.rs` — YubiKey backend confirmed: ECDSA P-256 only, no Ed25519, uses SHA-256 pre-hash internally (HIGH confidence)
- `/home/john/vault/projects/github.com/trustedge/crates/core/src/envelope.rs` — HKDF-SHA256 KDF pattern (v1.8) provides the model for chunk key derivation (HIGH confidence)
- `/home/john/vault/projects/github.com/trustedge/.planning/PROJECT.md` — Milestone requirements and constraints (HIGH confidence)

---
*Feature research for: TrustEdge v2.1 Data Lifecycle & Hardware Integration*
*Researched: 2026-03-15*
