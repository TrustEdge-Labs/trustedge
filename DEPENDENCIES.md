# Dependency Audit — Core Crates

*Last audited: 2026-02-12*
*Milestone: v1.2 (DEPS-01)*

This document provides a comprehensive audit of all dependencies in the 5 stable tier crates of the TrustEdge workspace.

## trustedge-core

Core cryptographic library with network transport, backends, and protocol implementations.

| Dependency | Version | Justification | Status |
|------------|---------|---------------|--------|
| aead | 0.5 | AEAD trait for encryption backends (generic cipher interface) | Used |
| aes-gcm | 0.10.3 | AES-256-GCM envelope encryption (core crypto primitive) | Used |
| anyhow | 1.0 | Error handling with context for binaries and complex operations | Used |
| async-trait | 0.1 | Async trait definitions for backends (not directly used, transitive) | Transitive |
| bincode | 1.3 | Binary serialization for vector storage and inspection tools | Used |
| blake3 | 1.5 | Cryptographic hashing for continuity chains and manifests | Used |
| chacha20poly1305 | 0.10 | XChaCha20-Poly1305 encryption for crypto.rs module | Used |
| chrono | 0.4 | Timestamp formatting for binaries (server, demos) | Used |
| clap | 4.5 | CLI argument parsing for server, client, and demo binaries | Used |
| cpal | 0.15 | Live audio capture (feature-gated: audio) | Used (optional) |
| ed25519-dalek | 2 | Ed25519 signing and verification for auth and envelopes | Used |
| git2 | 0.18 | Git integration for attestation module | Used |
| hex | 0.4 | Hex encoding/decoding for key material in tests | Used |
| keyring | 2.0 | OS keyring integration for keyring backend | Used |
| num-traits | 0.2 | Numeric trait conversions for audio sample processing | Used |
| p256 | 0.13 | NIST P-256 ECDH for Software HSM backend | Used |
| pbkdf2 | 0.12 | Key derivation for keyring backends | Used |
| pkcs11 | 0.5 | PKCS#11 interface (feature-gated: yubikey) | Used (optional) |
| yubikey | 0.7 | YubiKey hardware backend (feature-gated: yubikey) | Used (optional) |
| rand | 0.8 | Random number generation (primarily for testing) | Used |
| rand_core | 0.6 | RNG traits and OsRng for key generation | Used |
| rsa | 0.9 | RSA asymmetric encryption for hybrid crypto | Used |
| serde | 1.0 | Serialization framework for protocols and auth | Used |
| serde_bytes | 0.11 | Efficient byte array serialization (attribute usage) | Used |
| serde_json | 1.0 | JSON serialization for metadata and protocols | Used |
| trustedge-trst-protocols | path | Archive manifest types (CamVideoManifest) | Used |
| sha2 | 0.10 | SHA-256 hashing for keyring key derivation | Used |
| tokio | 1.0 | Async runtime for network operations, I/O, and binaries | Used |
| tokio-util | 0.7 | Length-delimited codec for TCP framing | Used |
| futures-util | 0.3 | SinkExt/StreamExt for async I/O in transport layer | Used |
| zeroize | 1.7 | Secure memory zeroing for key material | Used |
| thiserror | 1.0 | Structured error types for library code | Used |
| quinn | 0.11 | QUIC transport implementation | Used |
| rustls | 0.23 | TLS for QUIC transport | Used |
| x509-cert | 0.2 | X.509 certificate generation (feature-gated: yubikey) | Used (optional) |
| signature | 2.2 | Signature traits (feature-gated: yubikey) | Used (optional) |
| der | 0.7 | DER encoding for YubiKey operations (feature-gated: yubikey) | Used (optional) |
| spki | 0.7 | SubjectPublicKeyInfo handling (feature-gated: yubikey) | Used (optional) |
| rcgen | 0.13 | Certificate generation for YubiKey (feature-gated: yubikey) | Used (optional) |

**Tokio features used:**
- `io-util` - AsyncReadExt, AsyncWriteExt
- `net` - TcpListener, TcpStream
- `fs` - File operations
- `sync` - broadcast channels
- `time` - timeout, sleep
- `rt-multi-thread` - Multi-threaded runtime
- `macros` - #[tokio::main], #[tokio::test]
- `signal` - Signal handling

**Current config:** `features = ["full"]` — OPTIMIZATION OPPORTUNITY: trim to minimal feature set

## trustedge-cli

Main CLI binary for envelope encryption operations.

| Dependency | Version | Justification | Status |
|------------|---------|---------------|--------|
| trustedge-core | path | Core library with all cryptographic operations | Used |
| aead | 0.5 | AEAD trait for direct cipher instantiation in CLI | Used |
| aes-gcm | 0.10.3 | Direct Aes256Gcm cipher usage in encrypt/decrypt commands | Used |
| blake3 | 1.5 | Direct hashing for header and manifest verification | Used |
| ed25519-dalek | 2 | Direct SigningKey/VerifyingKey usage for signing operations | Used |
| rand_core | 0.6 | OsRng for key generation in CLI commands | Used |
| anyhow | 1.0 | Error handling with context for CLI | Used |
| bincode | 1.3 | Binary serialization for envelope headers | Used |
| clap | 4.5 | CLI argument parsing | Used |
| hex | 0.4 | Hex encoding/decoding for key input/output | Used |
| zeroize | 1.7 | Secure memory handling for keys | Used |

**Note:** The crypto dependencies (aead, aes-gcm, blake3, ed25519-dalek, rand_core) are INTENTIONALLY duplicated from trustedge-core. The CLI directly instantiates ciphers and signing keys for its encrypt/decrypt/sign commands rather than going through core's abstractions. This is legitimate use, not redundancy.

## trustedge-trst-protocols

Protocol and format definitions for .trst archives (WASM-compatible, minimal dependencies).

| Dependency | Version | Justification | Status |
|------------|---------|---------------|--------|
| serde | 1.0 | Serialization traits for manifest types | Used |
| serde_json | 1.0 | JSON serialization for cam.video manifests | Used |
| thiserror | 1.0 | Error types for format validation | Used |

**Design goal:** Minimal dependency footprint for WASM compatibility. All dependencies are essential.

## trustedge-trst-cli

CLI tool for .trst archive wrap/verify operations.

| Dependency | Version | Justification | Status |
|------------|---------|---------------|--------|
| trustedge-core | path | Core library for crypto operations | Used |
| anyhow | 1.0 | Error handling with context | Used |
| base64 | 0.22 | Base64 encoding for signatures | Used |
| chrono | 0.4 | Timestamp formatting for manifests | Used |
| clap | 4.5 | CLI argument parsing | Used |
| ed25519-dalek | 2 | Signature verification for archives | Used |
| rand_core | 0.6 | RNG for key generation | Used |
| serde | 1.0 | Serialization for manifests | Used |
| serde_json | 1.0 | JSON manifest parsing | Used |
| hex | 0.4 | Hex encoding for keys | Used |
| chacha20poly1305 | 0.10 | Encryption for archive chunks | Used |
| rand | 0.8 | Random number generation | Used |
| rand_chacha | 0.3 | ChaCha RNG for deterministic randomness | Used |
| reqwest | 0.11 | HTTP client for --post option (POSTs verify requests) | Used |
| tokio | 1.0 | Async runtime for reqwest | Used |
| blake3 | 1.5 | Hashing for chunk verification | Used |

**Tokio features used:**
- `macros` - #[tokio::main]
- `rt-multi-thread` - Runtime for async reqwest calls

**Current config:** `features = ["full"]` — OPTIMIZATION OPPORTUNITY: trim to ["macros", "rt-multi-thread"]

**reqwest justification:** The `trst sign` command has a `--post` option that POSTs the generated verify request to a remote URL. This is a legitimate feature for integration with verification services. The dependency SHOULD be kept but could be made optional with a feature flag in the future.

## trustedge-trst-wasm

WASM bindings for browser-based archive verification.

| Dependency | Version | Justification | Status |
|------------|---------|---------------|--------|
| trustedge-trst-protocols | path | Manifest type definitions | Used |
| serde | 1.0 | Serialization for WASM bindings | Used |
| serde_json | 1.0 | JSON parsing in WASM | Used |
| serde-wasm-bindgen | 0.6 | Serde integration for wasm-bindgen | Used |
| wasm-bindgen | 0.2 | Rust/JS interop | Used |
| wasm-bindgen-futures | 0.4 | Async support for WASM | Used |
| js-sys | 0.3 | JavaScript standard library bindings | Used |
| console_error_panic_hook | 0.1 | Better panic messages in browser console | Used |
| ed25519-dalek | 2 | Signature verification in browser | Used |
| getrandom | 0.2 | WASM RNG support (js feature activation) | Used |
| base64 | 0.22 | Base64 decoding for signatures | Used |
| web-sys | 0.3 | Browser APIs (File, FileReader, FileSystem) | Used |

**Note on getrandom:** cargo-machete flags this as unused, but it MUST be present to activate the "js" feature for wasm32-unknown-unknown target. This is a known false positive documented in project memory. DO NOT REMOVE.

---

## Findings

### Redundant Dependencies

**NONE.** Initial analysis suggested trustedge-cli's crypto deps (aead, aes-gcm, blake3, ed25519-dalek, rand_core) might be redundant since trustedge-core exports them. However, code inspection reveals the CLI directly instantiates ciphers and signing keys for its commands rather than using core's abstractions. This is intentional and appropriate for a CLI tool.

### Unused Dependencies

**NONE.** All dependencies in the 5 core crates have verified usage in source code.

**False positives from cargo-machete:**
- `pkcs11` in trustedge-core — Only used when yubikey feature is enabled
- `getrandom` in trustedge-trst-wasm — Required for WASM feature activation (no direct imports)

### Optimization Opportunities

#### 1. Tokio feature flag trimming (DEPS-04)

**trustedge-core:**
- Current: `features = ["full"]` (includes 30+ features)
- Needed: `["io-util", "net", "fs", "sync", "time", "rt-multi-thread", "macros", "signal"]`
- Estimated savings: Faster compilation, smaller binary

**trustedge-trst-cli:**
- Current: `features = ["full"]`
- Needed: `["macros", "rt-multi-thread"]` (only for async runtime)
- Estimated savings: Significant compilation time reduction

#### 2. Future feature flags

**reqwest in trst-cli** could be made optional with a `network` or `remote-verify` feature flag. The --post option is useful but not core functionality. However, this would be a breaking change for users who rely on it.

### Additional Notes

1. **async-trait in trustedge-core:** Marked as "Transitive" because it's not directly used with `#[async_trait]` macros in the current codebase, but may be needed for trait object support. Consider auditing if this is actually required.

2. **cargo-machete configuration:** trustedge-core already has `[package.metadata.cargo-machete]` ignoring serde_bytes. Should also add getrandom to ignored list in trustedge-trst-wasm.

3. **Workspace dependency management:** All core crates properly use workspace dependencies where available, minimizing version skew and maintenance burden.

4. **YubiKey feature dependencies:** All YubiKey-related deps (pkcs11, yubikey, x509-cert, signature, der, spki, rcgen) are properly feature-gated and have verified usage in backends/yubikey.rs.

---

## Recommendations

1. **Immediate (this phase):**
   - Trim tokio features in trustedge-core from "full" to minimal set
   - Trim tokio features in trustedge-trst-cli from "full" to ["macros", "rt-multi-thread"]
   - Add getrandom to cargo-machete ignored list in trustedge-trst-wasm

2. **Future (post-v1.2):**
   - Consider making reqwest in trst-cli optional with a feature flag
   - Audit whether async-trait is actually required in trustedge-core
   - Consider consolidating chacha20poly1305 dependency between core and trst-cli

3. **Documentation:**
   - Add inline comments in Cargo.toml files explaining why CLI duplicates crypto deps
   - Document the WASM getrandom requirement in trst-wasm's Cargo.toml
