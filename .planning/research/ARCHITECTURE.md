# Architecture Research

**Domain:** Rust cryptographic archive CLI — data lifecycle completion and hardware integration
**Researched:** 2026-03-15
**Confidence:** HIGH (primary sources: actual codebase files read directly)

---

## System Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                  CLI Layer (thin shells)                         │
│  ┌─────────────────────────────────────────────────────┐        │
│  │  trustedge-trst-cli  (trst binary)                  │        │
│  │  Commands: wrap | verify | keygen | emit-request    │        │
│  │  ADD:      unwrap | (--backend flag on wrap/verify) │        │
│  └───────────────┬──────────────────────────────────────┘       │
└──────────────────┼──────────────────────────────────────────────┘
                   │ calls
┌──────────────────▼──────────────────────────────────────────────┐
│                  trustedge-core  (all crypto logic lives here)   │
│                                                                  │
│  crates/core/src/                                                │
│  ├── archive.rs         read/write_archive, validate_archive     │
│  │   ADD: decrypt_archive() -- new public fn                     │
│  ├── crypto.rs          DeviceKeypair, sign_manifest,            │
│  │                      encrypt_segment / decrypt_segment        │
│  │   ADD: sign_manifest_ecdsa(), verify_manifest_ecdsa()         │
│  ├── chain.rs           genesis, chain_next, segment_hash        │
│  ├── envelope.rs        Envelope::seal / unseal (unchanged)      │
│  └── backends/                                                   │
│      ├── yubikey.rs     YubiKeyBackend (ECDSA P-256, RSA-2048)   │
│      ├── software_hsm.rs Ed25519 software backend                │
│      └── universal.rs  UniversalBackend trait                    │
└──────────────────┬──────────────────────────────────────────────┘
                   │ re-exports manifest types from
┌──────────────────▼──────────────────────────────────────────────┐
│         trustedge-trst-protocols  (WASM-compatible types)        │
│  crates/trst-protocols/src/archive/manifest.rs                   │
│  TrstManifest, ProfileMetadata, GenericMetadata, CamVideoMetadata│
│  ADD: SensorMetadata, AudioMetadata, LogMetadata variants        │
└─────────────────────────────────────────────────────────────────┘
```

---

## Component Responsibilities

| Component | Responsibility | v2.1 Change |
|-----------|---------------|-------------|
| `trustedge-trst-cli` | CLI arg parsing, file I/O, output formatting | Add `Unwrap` subcommand; add `--backend` flag to `WrapCmd`/`VerifyCmd` |
| `trustedge-core::archive` | `.trst` directory read/write, validation, chain check | Add `decrypt_archive()` that calls `decrypt_segment` per chunk |
| `trustedge-core::crypto` | `DeviceKeypair`, `sign_manifest`, `verify_manifest`, `encrypt_segment`/`decrypt_segment` | Add `sign_manifest_ecdsa()` and `verify_manifest_ecdsa()` for ECDSA P-256 path |
| `trustedge-core::backends::yubikey` | ECDSA P-256 / RSA-2048 hardware signing via PIV | No changes needed — already implemented and tested |
| `trustedge-trst-protocols::manifest` | `TrstManifest`, `ProfileMetadata` enum, canonical serialization | Add `Sensor`, `Audio`, `Log` variants to `ProfileMetadata`; update `validate()` and `serialize_canonical()` |

---

## Integration Points for Each Feature

### Feature 1: `trst unwrap` -- Archive Decryption

**What must happen:**
1. CLI parses `UnwrapCmd { archive: PathBuf, device_key: PathBuf, out: PathBuf }`.
2. CLI calls `read_archive(&archive)` -- already in `trustedge-core::archive` -- to get `(TrstManifest, Vec<(usize, Vec<u8>)>)`.
3. CLI (or a new `decrypt_archive` in `trustedge-core::archive`) calls `decrypt_segment` on each ciphertext chunk.
4. CLI reassembles decrypted chunks in sequence order and writes output file.

**Existing infrastructure to reuse:**

| Existing function | Location | Role in unwrap |
|-------------------|----------|----------------|
| `read_archive()` | `trustedge-core::archive` | Reads manifest + raw ciphertext bytes |
| `decrypt_segment()` | `trustedge-core::crypto` | XChaCha20-Poly1305 per-chunk decrypt |
| `generate_aad()` | `trustedge-core::crypto` | Reconstructs AAD for auth tag check |
| `DeviceKeypair::import_secret()` | `trustedge-core::crypto` | Load key from file |

**New code required:**

- `crates/core/src/archive.rs`: `pub fn decrypt_archive(base_dir, key, aad_params) -> Result<Vec<u8>>` -- calls `read_archive`, iterates chunks, calls `decrypt_segment`, concatenates plaintext. This keeps decryption logic in `trustedge-core`, not in the CLI.
- `crates/trst-cli/src/main.rs`: `UnwrapCmd` struct + `Commands::Unwrap` variant + `handle_unwrap()`.

**Critical design decision required before implementation:** The symmetric key used in `wrap` is currently a hardcoded demo constant (`"0123456789abcdef0123456789abcdef"`). For `unwrap` to work securely, the same key must be derivable or supplied. Two options:

- Option A (recommended): Derive the symmetric key from the Ed25519 signing key via HKDF. This mirrors the existing `Envelope::seal/unseal` ECDH pattern -- no separate key management needed.
- Option B: Accept an explicit `--key` file parameter. Simpler but requires users to manage a separate symmetric key file.

Option A is consistent with the existing architecture. The HKDF derivation path in `envelope.rs` is the template.

---

### Feature 2: YubiKey Hardware Signing in `trst wrap` / `trst verify`

**The central constraint established in v1.1:** YubiKey PIV does NOT support Ed25519. `YubiKeyBackend::piv_sign` explicitly returns `UnsupportedOperation` for `SignatureAlgorithm::Ed25519`. The `.trst` manifest uses Ed25519 signatures (`sign_manifest` in `crypto.rs`). These two facts are in tension.

**Resolution:** Add a `--backend [software|yubikey]` flag. When `yubikey` is selected:
- Signing uses ECDSA P-256 via `YubiKeyBackend::piv_sign`
- Signature stored as `"ecdsa-p256:<base64>"` to distinguish from `"ed25519:..."`
- Verification detects prefix and dispatches accordingly

This approach preserves all existing Ed25519 archives without schema changes and adds hardware signing as an opt-in path.

**Existing infrastructure to reuse:**

| Existing | Location | Role |
|----------|----------|------|
| `YubiKeyBackend::piv_sign()` | `crates/core/src/backends/yubikey.rs` | Does the actual ECDSA P-256 signing |
| `YubiKeyBackend::with_config()` | same | Connects to hardware, fail-closed |
| `YubiKeyConfig::builder()` | same | Constructs config with PIN via Secret<T> |
| `ensure_connected()` | same | Fail-closed gate -- returns HardwareError if no device |

**New code required:**

- `crates/core/src/crypto.rs`: `sign_manifest_ecdsa(backend: &YubiKeyBackend, key_id: &str, canonical_bytes: &[u8]) -> Result<String>` -- calls `backend.piv_sign(key_id, canonical_bytes, SignatureAlgorithm::EcdsaP256)`, returns `"ecdsa-p256:<base64>"`.
- `crates/core/src/crypto.rs`: `verify_manifest_ecdsa(cert_or_pubkey_der: &[u8], canonical_bytes: &[u8], sig: &str) -> Result<bool>` -- ECDSA P-256 verification using the public key extracted from the YubiKey certificate.
- `crates/trst-cli/src/main.rs`: `--backend [software|yubikey]` arg on `WrapCmd` and `VerifyCmd`. When `yubikey`, instantiate `YubiKeyBackend`, call `sign_manifest_ecdsa`.
- `crates/trst-cli/Cargo.toml`: `yubikey` feature flag propagating `trustedge-core/yubikey`.

**Feature flag pattern:** `trustedge-trst-cli` must grow a `yubikey` feature (matching the pattern in `trustedge-platform`). Without the feature compiled in, `--backend yubikey` returns a clear compile-time exclusion or runtime error: "YubiKey support not compiled in. Rebuild with `--features yubikey`."

---

### Feature 3: Named Archive Profiles (sensor, audio, log)

**Where profiles live:** `ProfileMetadata` enum in `crates/trst-protocols/src/archive/manifest.rs`. Currently two variants: `CamVideo(CamVideoMetadata)` and `Generic(GenericMetadata)`.

**What changes in `trustedge-trst-protocols`:**

```rust
// ADD three new variants to ProfileMetadata enum:
pub enum ProfileMetadata {
    CamVideo(CamVideoMetadata),   // existing
    Generic(GenericMetadata),     // existing
    Sensor(SensorMetadata),       // NEW
    Audio(AudioMetadata),         // NEW
    Log(LogMetadata),             // NEW
}
```

**Proposed new metadata structs:**

```rust
pub struct SensorMetadata {
    pub started_at: String,
    pub ended_at: String,
    pub sensor_type: String,         // required: "temperature", "accelerometer", "gps"
    pub unit: Option<String>,        // "celsius", "m/s^2"
    pub sample_rate_hz: Option<f64>,
    pub labels: BTreeMap<String, String>,
}

pub struct AudioMetadata {
    pub started_at: String,
    pub ended_at: String,
    pub sample_rate_hz: u32,         // required: 44100, 48000
    pub channels: u8,                // required
    pub bit_depth: u8,               // required: 16, 24
    pub codec: String,               // required: "pcm", "flac", "opus"
    pub labels: BTreeMap<String, String>,
}

pub struct LogMetadata {
    pub started_at: String,
    pub ended_at: String,
    pub log_level: Option<String>,   // "info", "debug", "error"
    pub source: Option<String>,
    pub format: Option<String>,      // "json", "syslog", "plaintext"
    pub labels: BTreeMap<String, String>,
}
```

**Cascading changes required:**

| File | Change |
|------|--------|
| `crates/trst-protocols/src/archive/manifest.rs` | Add 3 structs + 3 enum variants + update `validate()` + update `serialize_canonical()` for each variant |
| `crates/core/src/lib.rs` | Re-export new types alongside existing ones |
| `crates/trst-cli/src/main.rs` | `--profile sensor|audio|log` accepted; `WrapCmd` grows profile-specific args; `handle_wrap` match arm for each new profile |
| `crates/core/src/archive.rs` | No change -- already profile-agnostic |
| `trustedge-platform` verify handler | No change -- works off `manifest: serde_json::Value` |

**serde constraint:** `ProfileMetadata` uses `#[serde(untagged)]`. The untagged enum tries variants in declaration order; the first that deserializes without error wins. New variants must have at least one required field (not `Option`) that does not exist in `GenericMetadata` to ensure unambiguous disambiguation. `sensor_type` (required in `SensorMetadata`), `sample_rate_hz` as `u32` (required in `AudioMetadata`), and `format` scoped with a unique field name in `LogMetadata` can serve this role -- but this is a schema design decision that must be locked before coding begins.

Alternative: switch `ProfileMetadata` to `#[serde(tag = "kind")]` using the `TrstManifest.profile` field as the discriminant. This is unambiguous but changes the on-disk JSON format for existing generic and cam.video archives -- a breaking schema change. Given that v2.0 shipped generic profiles, this should be avoided unless a migration path is planned.

---

## Recommended Build Order

Dependencies flow from lowest-level to highest:

```
1. trustedge-trst-protocols  (manifest types -- no deps on other workspace crates)
        |
        v
2. trustedge-core             (archive, crypto, backends -- depends on trst-protocols)
        |
        v
3. trustedge-trst-cli         (CLI -- depends on core)
        |
        v
4. trustedge-trst-wasm        (browser verify -- depends on trst-protocols; needs rebuild
                               if manifest schema changes)
```

**Recommended phase order for v2.1:**

| Phase | Feature | Rationale |
|-------|---------|-----------|
| 1 | Named profiles (`trst-protocols`) | No runtime risk; pure data types; unblocks CLI metadata arg work; must come before CLI changes |
| 2 | `trst unwrap` decryption (core + CLI) | Self-contained; reuses existing `decrypt_segment`; delivers immediate value; no hardware dependency |
| 3 | YubiKey CLI integration (core + CLI) | Most complex due to PIV/Ed25519 tension; isolated feature flag; hardware-dependent testing |

---

## Data Flow Changes

### Current wrap flow (unchanged):
```
Input file
    -> chunk (args.chunk_size bytes each)
    -> encrypt_segment (XChaCha20-Poly1305 + AAD)
    -> segment_hash (BLAKE3 over ciphertext)
    -> chain_next (BLAKE3 continuity chain)
    -> build TrstManifest + ProfileMetadata
    -> sign_manifest (Ed25519)
    -> write_archive (manifest.json + chunks/*.bin + signatures/manifest.sig)
```

### New unwrap flow:
```
.trst archive path
    -> read_archive -> (TrstManifest, Vec<(index, ciphertext)>)
    -> verify_manifest (Ed25519 signature check)
    -> validate_archive (chain integrity)
    -> for each chunk: decrypt_segment (XChaCha20-Poly1305, reconstruct AAD)
    -> sort by index, concatenate plaintext
    -> write output file
```

### New YubiKey wrap flow (--backend yubikey):
```
Input file
    -> [same encrypt/chain steps as current wrap]
    -> build TrstManifest
    -> sign_manifest_ecdsa (ECDSA P-256 via YubiKeyBackend::piv_sign)
       stores signature as "ecdsa-p256:<base64>" to distinguish from "ed25519:..."
    -> write_archive
```

### New named profile flow (--profile sensor|audio|log):
```
Input file + profile-specific CLI args (--sensor-type, --sample-rate, etc.)
    -> build SensorMetadata / AudioMetadata / LogMetadata
    -> wrap in ProfileMetadata::Sensor / Audio / Log variant
    -> [same encrypt/chain/sign steps as current wrap]
    -> write_archive
```

---

## New vs Modified Components

### NEW (net-new code):
- `decrypt_archive()` in `trustedge-core::archive` -- decrypts all chunks, returns plaintext
- `sign_manifest_ecdsa()` in `trustedge-core::crypto` -- ECDSA P-256 wrapper over `YubiKeyBackend::piv_sign`
- `verify_manifest_ecdsa()` in `trustedge-core::crypto` -- ECDSA P-256 verification
- `Commands::Unwrap` + `UnwrapCmd` + `handle_unwrap()` in `trustedge-trst-cli`
- `SensorMetadata`, `AudioMetadata`, `LogMetadata` structs in `trustedge-trst-protocols`
- `yubikey` feature flag in `trustedge-trst-cli/Cargo.toml`

### MODIFIED (changes to existing code):
- `ProfileMetadata` enum: add 3 variants; update `serialize_canonical()` and `validate()`
- `WrapCmd`: add `--backend` flag, profile-specific metadata flags for sensor/audio/log
- `Commands` enum: add `Unwrap` variant
- `handle_wrap()`: dispatch on new profile variants; dispatch on backend flag
- `handle_verify()`: accept `ecdsa-p256:...` signature prefix alongside `ed25519:...`
- `trustedge-core::lib.rs`: re-export new types

### UNCHANGED:
- `read_archive()`, `write_archive()`, `validate_archive()` -- no behavior change
- `TrstManifest` struct itself -- only `ProfileMetadata` enum grows
- `YubiKeyBackend` -- already fully implemented; no changes needed
- `trustedge-platform` verify handler -- operates on `serde_json::Value`; profile-agnostic
- `trustedge-trst-wasm` -- may need rebuild if manifest type changes but no logic change
- All existing tests

---

## Architectural Patterns to Follow

### Pattern 1: Monolith Core, Thin CLI Shell

All crypto and archive logic lives in `trustedge-core`. The CLI is pure argument parsing + output formatting + file I/O via core function calls.

**Apply to v2.1:** `decrypt_archive()` belongs in `trustedge-core::archive`, not in `handle_unwrap()`. `sign_manifest_ecdsa()` belongs in `trustedge-core::crypto`. The CLI handler calls core functions and formats output. This keeps the CLI testable at unit level and the logic reusable.

### Pattern 2: Fail-Closed Hardware

YubiKey operations call `ensure_connected()` before any hardware op. If hardware is absent, return `BackendError::HardwareError`, never a software fallback.

**Apply to v2.1:** `handle_wrap --backend yubikey` must error cleanly if no YubiKey is present. Do not silently fall back to software Ed25519 signing. The error message from `ensure_connected()` already reads "YubiKey not connected. Insert device and retry." -- preserve this.

### Pattern 3: Feature-Gated Hardware Dependencies

The `yubikey` feature in `trustedge-core` gates the YubiKey crate and all PCSC dependencies. Default build compiles without PCSC.

**Apply to v2.1:** `trustedge-trst-cli` needs its own `yubikey` feature that propagates `trustedge-core/yubikey`. Without it, the `--backend yubikey` code path must be `#[cfg(feature = "yubikey")]` gated and return a clear error if attempted at runtime.

### Pattern 4: `#[serde(untagged)]` Disambiguation

`ProfileMetadata` uses untagged enum deserialization. Serde tries variants in declaration order; the first that deserializes without error wins. `CamVideo` is listed first because its required fields (`timezone`, `fps`, `resolution`, `codec`) uniquely distinguish it.

**Apply to v2.1:** New profile variants must have at least one required field not present in `Generic`. If disambiguation is ambiguous, the safer option is switching to `#[serde(tag = "profile")]` but this is a breaking schema change requiring migration.

---

## Anti-Patterns to Avoid

### Anti-Pattern 1: Decryption Logic in the CLI Handler

**What people do:** Put the `decrypt_segment` loop inside `handle_unwrap()` directly.
**Why it's wrong:** Breaks the monolith-core / thin-shell architecture. Cannot be tested without a full CLI invocation. Cannot be reused by WASM or other consumers.
**Do this instead:** Implement `decrypt_archive()` in `trustedge-core::archive`, call it from `handle_unwrap()`.

### Anti-Pattern 2: Silent Software Fallback for YubiKey

**What people do:** When YubiKey is unavailable with `--backend yubikey`, fall back to software Ed25519 signing.
**Why it's wrong:** Defeats the purpose of hardware signing. Data appears to be hardware-signed when it is not. Violates the v1.1 fail-closed design principle established at great cost.
**Do this instead:** Return `BackendError::HardwareError` matching the existing `ensure_connected()` message pattern.

### Anti-Pattern 3: Keeping the Hardcoded Demo Key for Real Unwrap

**What people do:** Keep the hardcoded `"0123456789abcdef..."` symmetric key from `handle_wrap()` and accept the same key in `handle_unwrap()`.
**Why it's wrong:** Data "encrypted" with a published key is not encrypted. The comment in `wrap` already says "simplified for P0."
**Do this instead:** Derive the symmetric key from the Ed25519 signing key via HKDF, or require an explicit `--key` file parameter. The HKDF approach is preferred (matches `Envelope::seal` design).

### Anti-Pattern 4: Adding Profile Logic to `archive.rs`

**What people do:** Add profile-specific behavior to `read_archive()` or `write_archive()` when new profiles are added.
**Why it's wrong:** `archive.rs` is profile-agnostic -- it treats all chunks as opaque bytes. Profile metadata only affects the manifest. Adding profile logic here would break the clean separation.
**Do this instead:** Keep all profile-specific logic in `trustedge-trst-protocols` (metadata structs + validation) and `trustedge-trst-cli` (metadata argument parsing). `archive.rs` stays unchanged.

### Anti-Pattern 5: Adding New Subcommand to Commands Enum Without Feature Gate

**What people do:** Add `Commands::Unwrap` and `Commands::YubikeySign` as unconditional enum variants.
**Why it's wrong:** The YubiKey path requires hardware that is not always available. The unwrap path requires the symmetric key design decision to be made first.
**Do this instead:** `Commands::Unwrap` is unconditional (decryption is always possible). `--backend yubikey` inside `WrapCmd`/`VerifyCmd` is `#[cfg(feature = "yubikey")]` gated.

---

## Integration Boundary Summary

| Boundary | Communication | Notes |
|----------|---------------|-------|
| `trst-cli` <-> `trustedge-core::archive` | Direct Rust function calls | `read_archive`, `write_archive`, `decrypt_archive` |
| `trst-cli` <-> `trustedge-core::crypto` | Direct Rust function calls | `sign_manifest`, `sign_manifest_ecdsa`, `verify_manifest_ecdsa` |
| `trustedge-core` <-> `trustedge-trst-protocols` | Direct Rust re-exports | Manifest types consumed via `use trustedge_trst_protocols::archive::manifest::*` |
| `trustedge-core` <-> `YubiKeyBackend` | Feature-gated Rust struct | `#[cfg(feature = "yubikey")]` |
| `trst-cli` <-> `trustedge-platform` | HTTP POST (emit-request command) | JSON `VerifyRequest` from `trustedge-types` |

---

## Sources

- `crates/core/src/archive.rs` -- direct read, confirmed `read_archive`, `write_archive`, `validate_archive` signatures
- `crates/core/src/crypto.rs` -- direct read, confirmed `DeviceKeypair`, `sign_manifest`, `decrypt_segment`
- `crates/core/src/envelope.rs` -- direct read, confirmed HKDF-based seal/unseal design
- `crates/core/src/backends/yubikey.rs` -- direct read, confirmed ECDSA P-256 support and Ed25519 rejection
- `crates/core/src/backends/universal.rs` -- direct read, confirmed `SignatureAlgorithm` variants
- `crates/trst-protocols/src/archive/manifest.rs` -- direct read, confirmed `ProfileMetadata` enum structure and `#[serde(untagged)]`
- `crates/trst-cli/src/main.rs` -- direct read, confirmed current command structure and wrap/verify implementation
- `crates/trst-cli/Cargo.toml` -- direct read, confirmed dependencies and feature structure
- `crates/core/src/lib.rs` -- direct read, confirmed public API surface and re-exports
- `.planning/PROJECT.md` -- confirmed v2.1 goals and constraints

---

*Architecture research for: TrustEdge v2.1 Data Lifecycle and Hardware Integration*
*Researched: 2026-03-15*
