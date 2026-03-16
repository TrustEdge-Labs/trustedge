# Stack Research

**Domain:** Rust crypto workspace — archive decryption CLI, YubiKey CLI signing, named archive profiles
**Researched:** 2026-03-15
**Confidence:** HIGH (codebase analysis is primary source; all three features work entirely within existing dependency set)

---

## Scope

This document covers stack additions and changes needed for v2.1 only:

1. `trst unwrap` — decrypt and reassemble original data from .trst archives
2. YubiKey hardware signing in `trst wrap` and `trst verify` CLI (`--backend yubikey`)
3. Named archive profiles: `sensor`, `audio`, `log` beyond generic and cam.video

Existing validated capabilities (AES-256-GCM, Ed25519, BLAKE3, HKDF, Universal Backend, XChaCha20Poly1305) are not re-researched.

---

## Key Finding: No New Dependencies Required

All three features are implementable with dependencies already in the workspace. The only question is which existing capabilities to wire up and what format changes are needed.

---

## Feature 1: Archive Decryption (`trst unwrap`)

### Critical Design Gap to Resolve First

The current `trst wrap` implementation generates a random XChaCha20Poly1305 nonce per chunk and a hardcoded demo encryption key. Neither the per-chunk nonce nor the key is stored in the archive. This means:

- **Nonces are lost at wrap time** — the archive cannot be decrypted without them
- **The key is a hardcoded demo constant** (`0123456789abcdef...`) — not a real key management scheme

Before `trst unwrap` can work, `trst wrap` must be fixed to store recoverable decryption material. Two design options:

**Option A — Nonce prepended to chunk file (recommended)**
- Prepend the 24-byte XChaCha20 nonce to each chunk `.bin` file
- No manifest format change needed
- `unwrap` reads nonce from first 24 bytes of each chunk, decrypts remaining bytes
- Backward-incompatible with existing archives (but current archives use demo key so they are not production data)

**Option B — Nonces in manifest**
- Add `nonce: String` field to `SegmentInfo` in `trst-protocols`
- Requires serialized nonce (base64 or hex) per segment
- Changes canonical JSON for signing — existing archives would fail verification
- More explicit but heavier manifest

**Recommendation: Option A.** Prepending to the chunk file is the standard pattern for XChaCha20 streams (the nonce is self-describing with the ciphertext). No manifest schema change, no canonical signing impact.

### Key Management for `trst unwrap`

The demo key (`0123456789abcdef...`) must be replaced with a real approach. The workspace already has `hkdf`, `aes-gcm`, `chacha20poly1305`, and `DeviceKeypair` with Ed25519 signing. The simplest correct approach for a symmetric archive key:

- Generate a random 256-bit archive encryption key at wrap time
- Write it to a sidecar file (e.g., `archive.key`) or accept it via `--key` CLI flag
- `trst unwrap --key <path>` reads the key file, decrypts chunk by chunk, concatenates plaintext to output file

This is explicit and auditable. The key file approach matches how `device.key` is currently handled.

### What Already Exists in Core

| Capability | Location | Status |
|-----------|----------|--------|
| `decrypt_segment()` (XChaCha20Poly1305) | `crates/core/src/crypto.rs` | Ready to call |
| `generate_aad()` | `crates/core/src/crypto.rs` | Ready to call |
| `read_archive()` returns `(manifest, Vec<(index, ciphertext)>)` | `crates/core/src/archive.rs` | Ready to call |
| `verify_manifest()` | `crates/core/src/crypto.rs` | Should run before decrypting |

### Stack Changes for `trst unwrap`

**No new dependencies.** Changes are:

1. `trst-cli/src/main.rs` — add `Unwrap(UnwrapCmd)` subcommand, implement `handle_unwrap()`
2. `crates/core/src/archive.rs` — add `decrypt_archive()` or inline in CLI (thin-shell pattern means CLI can call `read_archive` + `decrypt_segment` directly)
3. `trst-cli/Cargo.toml` — no changes (all needed crypto is via `trustedge-core`)

**Wrap fix required**: Modify `handle_wrap()` to generate a real random key (`OsRng.fill_bytes`), write it to `<output>.key`, and prepend nonce to each chunk file.

---

## Feature 2: YubiKey Hardware Signing in CLI

### The Ed25519 / ECDSA P-256 Constraint

This is the most significant architectural constraint for this feature.

The YubiKey PIV hardware **does not support Ed25519**. The `yubikey.rs` doc comment says explicitly: "Ed25519 is NOT supported by YubiKey PIV hardware. Use ECDSA P-256 instead."

The current `trst wrap` and `trst verify` use Ed25519 manifest signatures exclusively. The manifest format stores public keys as `"ed25519:BASE64"` strings, and `verify_manifest()` parses this prefix.

**Two options:**

**Option A — ECDSA P-256 manifest signatures when `--backend yubikey`**
- The manifest `device.public_key` field stores `"p256:BASE64"` for YubiKey-signed archives
- `sign_manifest()` and `verify_manifest()` need to support both algorithms
- `trst verify` must detect which algorithm to use from the public key prefix
- Requires changes to `crypto.rs` and the manifest signing path

**Option B — Ed25519 key in software for manifest, YubiKey for a separate attestation**
- Software Ed25519 key signs the manifest (unchanged)
- YubiKey ECDSA P-256 key signs a separate attestation blob that binds the YubiKey to the archive
- Manifest format unchanged; attestation is an additional file in the archive (`signatures/yubikey.att`)
- Simpler, doesn't require algorithm agility in manifest signing

**Recommendation: Option B for v2.1.** Option A requires algorithm agility across the manifest format, changing `trst-protocols` types, and updating `validate()`. Option B delivers the hardware integration value (YubiKey-backed proof of device identity) without breaking the existing manifest signature scheme. The YubiKey attestation becomes an additional trust anchor, not a replacement.

### Existing YubiKey Backend Capabilities

The `YubiKeyBackend` (in `crates/core/src/backends/yubikey.rs`) already implements:

| Operation | Method | Status |
|-----------|--------|--------|
| ECDSA P-256 signing | `sign_data()` via PIV slot | Implemented, tested |
| Public key extraction | `get_public_key()` | Implemented |
| Hardware attestation | `attest()` | Implemented |
| PIV slot management | Multiple slots (9a, 9c, 9d, 9e) | Implemented |
| Fail-closed hardware access | `ensure_connected()` | Implemented |

The `UniversalBackend` trait and `BackendRegistry` are in core. The CLI needs to instantiate a `YubiKeyBackend` and call it.

### Stack Changes for YubiKey CLI Integration

**No new dependencies.** The `yubikey` feature already exists in `trustedge-core`. Changes are:

1. `trst-cli/Cargo.toml` — add `features = ["yubikey"]` to `trustedge-core` dependency (feature-gated, not default)
2. `trst-cli/Cargo.toml` — add a `[features]` section: `yubikey = ["trustedge-core/yubikey"]`
3. `trst-cli/src/main.rs` — add `--backend` flag to `WrapCmd` (`software` default, `yubikey` option)
4. `trst-cli/src/main.rs` — when `--backend yubikey`, instantiate `YubiKeyBackend`, produce a YubiKey attestation signature, write to `signatures/yubikey.att`
5. `trst-cli/src/main.rs` — add `--verify-yubikey` flag to `VerifyCmd` to check the `yubikey.att` file

**Feature gate pattern** (consistent with existing codebase):

```toml
# trst-cli/Cargo.toml
[features]
default = []
yubikey = ["trustedge-core/yubikey"]
```

```rust
// trst-cli/src/main.rs
#[cfg(feature = "yubikey")]
fn handle_wrap_yubikey(...) -> Result<()> { ... }
```

### PIN/Config for CLI

The `YubiKeyConfig` uses `Secret<String>` for the PIN. In CLI context, PIN should be read from environment variable `TRUSTEDGE_YUBIKEY_PIN` or prompted interactively via `rpassword`. The `rpassword` crate is the standard for this in Rust CLIs.

**One new dependency for interactive PIN prompt:**

| Library | Version | Purpose | Why |
|---------|---------|---------|-----|
| `rpassword` | `7.3` | Read PIN from terminal without echo | Standard Rust crate for secret CLI input; no transitive dependencies beyond `rustix`/`windows-sys` |

This is only needed if interactive PIN prompt is in scope. If ENV variable only, no new dep is needed.

---

## Feature 3: Named Archive Profiles (sensor, audio, log)

### What Exists

`ProfileMetadata` in `crates/trst-protocols/src/archive/manifest.rs` is an `#[serde(untagged)]` enum:

```rust
pub enum ProfileMetadata {
    CamVideo(CamVideoMetadata),  // cam.video profile
    Generic(GenericMetadata),    // generic profile (default)
}
```

The `TrstManifest.validate()` explicitly rejects profiles other than `"generic"` and `"cam.video"`.

### Design Decision: Typed Variants vs. Labels

There are two approaches:

**Option A — New enum variants per profile (SensorMetadata, AudioMetadata, LogMetadata)**
- Each profile gets a dedicated struct with typed fields
- Type-safe at compile time
- Changes to `ProfileMetadata` enum + `serialize_canonical()` + `validate()` + manifest tests
- WASM compatibility: `trst-protocols` is WASM-compatible (minimal deps) — new structs using only serde types are safe

**Option B — Generic metadata with `data_type` labels**
- `sensor`, `audio`, `log` are just `data_type` strings in `GenericMetadata`
- No code changes — already works today with `--data-type sensor`
- No type-safe schema enforcement

**Recommendation: Option A for v2.1** — the milestone explicitly says "tailored metadata schemas." Typed variants deliver the schema enforcement and make the profiles first-class citizens. The implementation cost is low (struct definitions + pattern match arms).

### Proposed New Metadata Structs

```rust
// sensor profile
pub struct SensorMetadata {
    pub started_at: String,
    pub ended_at: String,
    pub sensor_type: String,       // "temperature", "pressure", "imu", etc.
    pub unit: Option<String>,      // "celsius", "hpa", "m/s2"
    pub sample_rate_hz: Option<f64>,
    pub device_model: Option<String>,
    pub labels: BTreeMap<String, String>,
}

// audio profile
pub struct AudioMetadata {
    pub started_at: String,
    pub ended_at: String,
    pub sample_rate_hz: u32,       // 44100, 48000, etc.
    pub channels: u8,              // 1 = mono, 2 = stereo
    pub bit_depth: u8,             // 16, 24, 32
    pub codec: String,             // "pcm", "opus", "aac"
    pub labels: BTreeMap<String, String>,
}

// log profile
pub struct LogMetadata {
    pub started_at: String,
    pub ended_at: String,
    pub log_level: Option<String>, // "debug", "info", "warn", "error"
    pub source: Option<String>,    // service or process name
    pub format: Option<String>,    // "json", "syslog", "plaintext"
    pub labels: BTreeMap<String, String>,
}
```

### Stack Changes for Named Profiles

**No new dependencies.** Changes are:

1. `crates/trst-protocols/src/archive/manifest.rs` — add `SensorMetadata`, `AudioMetadata`, `LogMetadata` structs; add variants to `ProfileMetadata` enum; extend `serialize_canonical()` with new match arms; update `validate()` to accept new profile strings
2. `crates/core/src/archive.rs` — no changes (uses `TrstManifest` which automatically picks up new variants)
3. `trst-cli/src/main.rs` — add CLI args for each profile's typed fields; extend `match args.profile.as_str()` to build the new metadata variants

**WASM compatibility**: All new structs use `serde`, `BTreeMap`, `String`, `Option<f64>`, `u32`, `u8` — all WASM-safe. No changes to `trst-wasm` required unless it has profile-specific code.

---

## Recommended Stack (Complete)

### Core Technologies (Unchanged)

| Technology | Version | Purpose |
|------------|---------|---------|
| Rust | 1.88 (Dockerfile) | Language |
| `trustedge-core` | workspace | All crypto: XChaCha20, Ed25519, BLAKE3, YubiKey backend |
| `trustedge-trst-protocols` | workspace | Archive format types |
| `chacha20poly1305` | `0.10` | Already in workspace — XChaCha20Poly1305 |
| `aes-gcm` | `0.10.3` | Already in workspace |
| `yubikey` | `0.7` | Already in core (feature-gated) — no version change |

### Supporting Libraries (trst-cli additions)

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `rpassword` | `7.3` | Interactive PIN prompt without echo | Only when `--backend yubikey` and PIN not in env |

### Feature Flags

| Flag | Crate | Controls |
|------|-------|---------|
| `yubikey` | `trustedge-trst-cli` | Enables YubiKey backend in CLI; gates on `trustedge-core/yubikey` |

---

## Cargo.toml Changes

### `crates/trst-cli/Cargo.toml`

```toml
[features]
default = []
yubikey = ["trustedge-core/yubikey"]

[dependencies]
# existing deps unchanged
trustedge-core = { path = "../core" }  # unchanged

# Add only if interactive PIN prompt required:
rpassword = { version = "7.3", optional = true }
```

### `crates/trst-protocols/src/archive/manifest.rs`

No `Cargo.toml` changes — pure Rust struct additions. The `trst-protocols` crate has no new dependencies.

---

## Alternatives Considered

| Area | Recommended | Alternative | Why Not |
|------|-------------|-------------|---------|
| Nonce storage | Prepend 24 bytes to chunk file | Add `nonce` field to `SegmentInfo` in manifest | Manifest field changes canonical JSON, breaks existing archive signatures |
| YubiKey signature scheme | YubiKey produces separate attestation file | YubiKey signs manifest directly with ECDSA P-256 | Requires algorithm agility in manifest format + `verify_manifest()` refactor; larger scope |
| PIN prompt | `rpassword` crate | `dialoguer` crate | `rpassword` is single-purpose, minimal deps; `dialoguer` is heavier |
| Profile metadata | Typed enum variants per profile | `data_type` label in GenericMetadata | Labels already exist; typed variants deliver the schema enforcement the milestone requires |
| Key management for unwrap | Explicit `--key <path>` file | Derive from device key + HKDF | Explicit file is simpler, auditable, matches how device keys are handled today |

## What NOT to Use

| Avoid | Why | Use Instead |
|-------|-----|-------------|
| `age` encryption crate | Would add 10+ new deps; XChaCha20 already in workspace | `chacha20poly1305 = "0.10"` already in trst-cli |
| OpenPGP / Sequoia | Heavyweight, different threat model | Existing XChaCha20 + nonce-prepend pattern |
| Post-quantum algorithms | Out of scope per PROJECT.md | Ed25519/XChaCha20 |
| Algorithm agility in manifest | Scope explosion; breaks stable archive format | Fixed Ed25519 + separate YubiKey attestation |
| `keyring` crate for key storage | Overkill for CLI unwrap key; platform-specific | Explicit key file (`--key archive.key`) |

---

## Version Compatibility

| Package | Compatible With | Notes |
|---------|-----------------|-------|
| `chacha20poly1305 = "0.10"` | `aead = "0.5"` | Already pinned in workspace and trst-cli |
| `yubikey = "0.7"` | `rcgen = "0.13"`, `der = "0.7"`, `spki = "0.7"` | Already in core feature; no version change |
| `rpassword = "7.3"` | No workspace conflicts | No transitive crypto deps; OS-level terminal I/O only |

---

## Sources

- Codebase analysis (HIGH confidence): `crates/trst-cli/src/main.rs`, `crates/core/src/crypto.rs`, `crates/core/src/archive.rs`, `crates/trst-protocols/src/archive/manifest.rs`, `crates/core/src/backends/yubikey.rs`, `Cargo.toml` (workspace)
- `.planning/PROJECT.md` — confirmed Active requirements and Out of Scope constraints (HIGH confidence)
- XChaCha20 nonce-prepend pattern: standard practice per libsodium and NaCl documentation (HIGH confidence, well-established)
- `rpassword` v7 — current version confirmed by crates.io search (MEDIUM confidence — training data; validate with `cargo add rpassword` to confirm latest)

---

*Stack research for: TrustEdge v2.1 Data Lifecycle & Hardware Integration*
*Researched: 2026-03-15*
