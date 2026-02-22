# TrustEdge Dependency Audit

**Last audited:** 2026-02-22
**Milestone:** v1.5 (Platform Consolidation)
**Scope:** All 12 workspace crates (6 stable tier, 5 experimental tier, 1 pending classification)

This document provides comprehensive documentation of all dependencies across the TrustEdge workspace, with per-dependency justifications and security rationale for critical dependencies.

## Table of Contents

**Stable Tier (Production-Committed):**
- [trustedge-platform](#trustedge-platform) - Consolidated verification and CA service (added v1.5)
- [trustedge-core](#trustedge-core) - Core cryptographic library
- [trustedge-cli](#trustedge-cli) - Main CLI for envelope encryption
- [trustedge-trst-protocols](#trustedge-trst-protocols) - Archive format definitions
- [trustedge-trst-cli](#trustedge-trst-cli) - Archive CLI tool
- [trustedge-trst-wasm](#trustedge-trst-wasm) - Browser archive verification

**Experimental Tier (Community/Experimental):**
- [trustedge-wasm](#trustedge-wasm) - General WASM bindings
- [trustedge-pubky](#trustedge-pubky) - Pubky network adapter
- [trustedge-pubky-advanced](#trustedge-pubky-advanced) - Pubky hybrid encryption
- [trustedge-receipts](#trustedge-receipts) - Re-export facade (deprecated)
- [trustedge-attestation](#trustedge-attestation) - Re-export facade (deprecated)

**Additional Sections:**
- [Security-Critical Dependency Rationale](#security-critical-dependency-rationale)
- [Workspace Dependency Summary](#workspace-dependency-summary)

---

## trustedge-platform

Consolidated verification and CA service. Merges trustedge-verify-core and trustedge-platform-api into a single crate with feature-gated modules.

**Default (always-on — verification core):**

| Dependency | Version | Justification | Status |
|------------|---------|---------------|--------|
| trustedge-types | path | Shared wire types (VerifyRequest, SegmentRef, etc.) | Used |
| anyhow | 1.0 | Error propagation in verification functions | Used |
| thiserror | 1.0 | Structured error types for CAError | Used |
| serde | 1.0 | Serialization for all API types | Used |
| serde_json | 1.0 | JSON manifest parsing and JWKS construction | Used |
| blake3 | 1.5 | BLAKE3 continuity chain and manifest digest | Used |
| ed25519-dalek | 2 | Ed25519 manifest signature verification | Used |
| base64 | 0.22 | Base64 encode/decode for keys and signatures | Used |
| chrono | 0.4 | Timestamp generation for receipts and JWKS | Used |
| uuid | 1 | Verification and receipt IDs | Used |
| rand | 0.8 | Key generation for KeyManager | Used |
| tracing | 0.1 | Structured logging for handlers | Used |
| jsonwebtoken | 9.2 | JWS receipt signing (EdDSA algorithm) | Used |
| regex | 1.0 | Segment hash format validation (^b3:[0-9a-f]{64}$) | Used |

**Feature `http` (Axum HTTP layer):**

| Dependency | Version | Justification | Status |
|------------|---------|---------------|--------|
| axum | 0.7 | HTTP framework for REST API handlers | Used |
| tower | 0.4 | Middleware composition | Used |
| tower-http | 0.5 | CORS and TraceLayer middleware | Used |
| tokio | 1.0 | Async runtime for HTTP service | Used |
| sha2 | 0.10 | SHA-256 for Bearer token hashing in auth middleware | Used |
| dotenvy | 0.15 | `.env` file loading for Config::from_env() | Used |
| utoipa-swagger-ui | 6.0 | OpenAPI UI (also gated on `openapi` feature) | Used (optional) |

**Feature `postgres` (multi-tenant backend):**

| Dependency | Version | Justification | Status |
|------------|---------|---------------|--------|
| sqlx | 0.7 | PostgreSQL CRUD (organizations, devices, verifications, receipts) | Used |
| bcrypt | 0.15 | Password hashing (reserved for future user auth) | Used (optional) |
| sha2 | 0.10 | SHA-256 for token hashing (shared with http feature) | Used |
| dotenvy | 0.15 | Env loading for database URL (shared with http feature) | Used |

**Feature `ca` (Certificate Authority):**

| Dependency | Version | Justification | Status |
|------------|---------|---------------|--------|
| trustedge-core | path | UniversalBackend for CA signing operations | Used |
| x509-parser | 0.16 | X.509 certificate parsing and validation | Used |
| der | 0.7 | DER encoding for certificate operations | Used |
| spki | 0.7 | SubjectPublicKeyInfo for CA certificates | Used |
| pkcs8 | 0.10 | PKCS#8 key format for CA | Used |
| x509-cert | 0.2 | X.509 cert construction | Used |
| const-oid | 0.9 | OID constants for X.509 extensions | Used |
| hex | 0.4 | Hex encoding for certificate fingerprints | Used |

**Feature `openapi` (API documentation):**

| Dependency | Version | Justification | Status |
|------------|---------|---------------|--------|
| utoipa | 4.0 | OpenAPI schema generation from handler types | Used |

**Consolidation note:** `reqwest` is NOT used in trustedge-platform. The key architectural change in v1.5 is that `verify_handler` calls `verify_to_report()` directly instead of forwarding to a separate verify-core HTTP service. This eliminates the HTTP forwarding round-trip and the reqwest dependency.

---

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
| git2 | 0.18 | Git integration for attestation module (feature-gated: git-attestation) | Used (optional) |
| hex | 0.4 | Hex encoding/decoding for key material in tests | Used |
| keyring | 2.0 | OS keyring integration for keyring backend (feature-gated: keyring) | Used (optional) |
| num-traits | 0.2 | Numeric trait conversions for audio sample processing | Used |
| p256 | 0.13 | NIST P-256 ECDH for Software HSM backend | Used |
| pbkdf2 | 0.12 | Key derivation for keyring backends | Used |
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

**Configuration:** Trimmed from `["full"]` to minimal feature set: `["io-util", "net", "fs", "sync", "time", "rt-multi-thread", "macros", "signal"]`

---

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

---

## trustedge-trst-protocols

Protocol and format definitions for .trst archives (WASM-compatible, minimal dependencies).

| Dependency | Version | Justification | Status |
|------------|---------|---------------|--------|
| serde | 1.0 | Serialization traits for manifest types | Used |
| serde_json | 1.0 | JSON serialization for cam.video manifests | Used |
| thiserror | 1.0 | Error types for format validation | Used |

**Design goal:** Minimal dependency footprint for WASM compatibility. All dependencies are essential.

---

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

**Configuration:** Trimmed from `["full"]` to minimal feature set: `["macros", "rt-multi-thread"]`

**reqwest justification:** The `trst sign` command has a `--post` option that POSTs the generated verify request to a remote URL. This is a legitimate feature for integration with verification services. The dependency is kept and justified.

---

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

**Note on getrandom:** cargo-machete flags this as unused (false positive), but it MUST be present to activate the "js" feature for wasm32-unknown-unknown target. Added to `[package.metadata.cargo-machete]` ignored list to suppress false positive warnings.

---

## trustedge-wasm

General WebAssembly bindings for TrustEdge cryptographic operations.

| Dependency | Version | Justification | Status |
|------------|---------|---------------|--------|
| wasm-bindgen | 0.2 | Rust/JS interop for WASM bindings | Used |
| js-sys | 0.3 | JavaScript standard library bindings | Used |
| serde | 1.0 | JSON serialization for WASM API | Used |
| serde_json | 1.0 | JSON serialization for WASM API | Used |
| aes-gcm | 0.10.3 | AES-256-GCM encryption in browser | Used |
| rand | 0.8 | Random number generation for crypto ops | Used |
| getrandom | 0.2 | WASM RNG support (js feature activation) | Used |
| base64 | 0.22 | Base64 encoding/decoding for key material | Used |
| console_error_panic_hook | 0.1 | Better panic messages in browser console | Used |

**Note on getrandom:** Required for WASM target. Activates "js" feature for wasm32-unknown-unknown. Added to cargo-machete ignored list.

---

## trustedge-pubky

Simple adapter for Pubky network key publishing/resolution.

| Dependency | Version | Justification | Status |
|------------|---------|---------------|--------|
| trustedge-core | path | Core library dependency | Used |
| pubky | 0.5.4 | Pubky network client for key publishing/resolution | Used |
| anyhow | 1.0 | Error handling for CLI binary | Used |
| tokio | 1.0 | Async runtime for Pubky network operations | Used |
| serde | 1.0 | JSON serialization for Pubky messages | Used |
| serde_json | 1.0 | JSON serialization for Pubky messages | Used |
| hex | 0.4 | Hex encoding for key display | Used |
| thiserror | 1.0 | Structured error types | Used |
| rand | 0.8 | Random number generation | Used |
| clap | 4.5 | CLI argument parsing for trustedge-pubky binary | Used |

---

## trustedge-pubky-advanced

Hybrid encryption with X25519 ECDH for Pubky ecosystem integration.

| Dependency | Version | Justification | Status |
|------------|---------|---------------|--------|
| trustedge-core | path | Core library dependency | Used |
| pubky | 0.5.4 | Pubky network client | Used |
| ed25519-dalek | 2 | Ed25519 signing for hybrid encryption | Used |
| x25519-dalek | 2.0 | X25519 ECDH key agreement | Used |
| aes-gcm | 0.10.3 | AES-256-GCM for hybrid encryption | Used |
| hkdf | 0.12 | HKDF key derivation for hybrid encryption | Used |
| blake3 | 1.5 | Hashing for key derivation | Used |
| sha2 | 0.10 | SHA-256 for HKDF (used directly, not workspace) | Used |
| serde | 1.0 | Serialization for hybrid messages | Used |
| serde_json | 1.0 | JSON serialization for hybrid messages | Used |
| bincode | 1.3 | Binary serialization for hybrid messages | Used |
| anyhow | 1.0 | Error handling | Used |
| hex | 0.4 | Hex encoding for keys | Used |
| zeroize | 1.7 | Secure memory zeroing for key material | Used |
| rand | 0.8 | Random number generation | Used |
| thiserror | 1.0 | Structured error types | Used |
| tokio | 1.0 | Async runtime | Used |
| reqwest | 0.11 | HTTP client for Pubky network operations | Used |

---

## trustedge-receipts

Re-export facade for trustedge-core receipts module (deprecated, removal planned August 2026).

| Dependency | Version | Justification | Status |
|------------|---------|---------------|--------|
| trustedge-core | path | Re-export facade (deprecated, removal planned August 2026) | Used (facade) |

**Note:** Users should migrate to using `trustedge-core` directly. This crate provides no additional functionality.

---

## trustedge-attestation

Re-export facade for trustedge-core attestation module (deprecated, removal planned August 2026).

| Dependency | Version | Justification | Status |
|------------|---------|---------------|--------|
| trustedge-core | path | Re-export facade (deprecated, removal planned August 2026) | Used (facade) |

**Note:** Users should migrate to using `trustedge-core` directly. This crate provides no additional functionality.

---

## Security-Critical Dependency Rationale

This section provides detailed justification for dependencies that handle cryptographic operations, TLS/transport security, or key storage. Each entry explains what the dependency does, why it was chosen over alternatives, how TrustEdge uses it, and any known security considerations.

### Cryptographic Primitives

**1. aes-gcm** (AES-256-GCM): Core encryption primitive for envelope encryption. Used in trustedge-core for per-chunk encryption of input data, in trustedge-cli for direct cipher operations, and in WASM crates for browser-side encryption. Chosen as the industry-standard authenticated encryption algorithm. Part of the RustCrypto ecosystem which undergoes regular security review.

**2. chacha20poly1305** (XChaCha20-Poly1305): Alternative AEAD cipher used in crypto.rs module. Provides stream cipher encryption as complement to AES-GCM. Used in trst-cli for archive chunk encryption. Chosen for its constant-time implementation and resistance to timing attacks on platforms without AES-NI hardware acceleration.

**3. ed25519-dalek** (Ed25519): Core signing primitive. Used for envelope signing, mutual authentication, archive signature verification, and hybrid encryption in Pubky. Chosen for its small key size (32 bytes), fast verification, and resistance to many classes of implementation attacks. The dalek library is the most widely audited Rust Ed25519 implementation.

**4. blake3**: Cryptographic hash function. Used for continuity chain hashing, manifest verification, and key derivation. Chosen over SHA-256 for performance (parallelizable, SIMD-optimized) while maintaining 256-bit security. Used across core, cli, trst-cli, and pubky-advanced.

**5. rsa**: RSA asymmetric encryption. Used ONLY in Pubky hybrid encryption (experimental) and YubiKey PIV operations (feature-gated). NOT used in core production encryption path (which is Ed25519 + AES-256-GCM). Known advisory RUSTSEC-2023-0071 (Marvin Attack) accepted with risk documentation in .cargo/audit.toml since TrustEdge does not use RSA for decryption timing-sensitive operations.

**6. p256** (NIST P-256 ECDH): Used in Software HSM backend for ECDH key agreement. Provides P-256 elliptic curve operations for backends that require NIST-approved algorithms.

**7. x25519-dalek** (X25519): Used in trustedge-pubky-advanced for Diffie-Hellman key agreement in hybrid encryption. Provides modern elliptic curve key exchange complementing Ed25519 signing.

**8. hkdf**: HMAC-based Key Derivation Function. Used in pubky-advanced for deriving symmetric keys from ECDH shared secrets. Industry-standard KDF specified in RFC 5869.

**9. pbkdf2**: Password-based key derivation. Used in keyring backend for deriving encryption keys from stored secrets. Provides intentionally slow key derivation to resist brute-force attacks.

### TLS and Transport Security

**10. rustls**: TLS implementation for QUIC transport. Pure-Rust TLS 1.3 library, chosen over OpenSSL bindings for portability and memory safety. Used by quinn for QUIC connection encryption.

**11. quinn**: QUIC transport protocol implementation. Provides encrypted, multiplexed connections for TrustEdge's network transport layer. Built on rustls for TLS 1.3 support.

### Key Storage and Hardware Security

**12. keyring** (feature-gated): OS keyring integration for secure key storage. Provides cross-platform access to macOS Keychain, Windows Credential Manager, and Linux Secret Service. Feature-gated behind `keyring` flag to avoid platform-specific dependencies in default builds.

**13. yubikey** (feature-gated): YubiKey hardware security module interface. Provides PIV (Personal Identity Verification) operations for hardware-backed key storage and signing. Feature-gated behind `yubikey` flag. Depends on system PCSC daemon for hardware communication.

**14. zeroize**: Secure memory zeroing for cryptographic key material. Ensures keys are wiped from memory after use, preventing cold-boot and memory-dump attacks. Used across core, cli, and pubky-advanced for all key types.

**15. rcgen** (feature-gated): X.509 certificate generation for YubiKey operations. Generates self-signed certificates for PIV slot operations. Only compiled with yubikey feature.

---

## Workspace Dependency Summary

The workspace defines shared dependencies in `[workspace.dependencies]` to minimize version skew and maintenance burden. The following dependencies are available workspace-wide:

**Cryptography:**
- aead 0.5
- aes-gcm 0.10.3
- blake3 1.5
- ed25519-dalek 2 (with rand_core feature)
- p256 0.13 (with ecdsa, pem, ecdh features)
- pbkdf2 0.12
- rand 0.8
- rand_core 0.6
- rsa 0.9.10 (with pem feature)
- x25519-dalek 2.0 (with static_secrets feature)
- hkdf 0.12

**Pubky Integration:**
- pubky 0.5.4

**Serialization:**
- bincode 1.3
- serde 1.0 (with derive feature)
- serde_bytes 0.11
- serde_json 1.0

**Git Operations:**
- git2 0.18

**Async and Utilities:**
- anyhow 1.0
- async-trait 0.1
- chrono 0.4 (with serde feature)
- hex 0.4
- num-traits 0.2
- thiserror 1.0
- zeroize 1.7

**CLI and System:**
- clap 4.5 (with derive feature)
- keyring 2.0

**WASM-Specific:**
- wasm-bindgen 0.2
- js-sys 0.3
- serde-wasm-bindgen 0.6
- getrandom 0.2 (with js feature)

**Platform Service (trustedge-platform, feature-gated):**
- axum 0.7 (HTTP framework)
- tower 0.4 (middleware)
- tower-http 0.5 (CORS, trace)
- sqlx 0.7 (PostgreSQL, features: runtime-tokio-rustls, postgres, chrono, uuid, migrate)
- hyper 1.0 (transitive from axum)

**Development:**
- tokio 1.0 (with full feature set for development only)

**Note on Crate Classification:**

The TrustEdge workspace uses a 2-tier classification system:

- **Stable tier (6 crates):** Production-committed, tested in CI, actively maintained. These crates have `tier = "stable"` and `maintained = true` in their `[package.metadata.trustedge]` section. Includes: `trustedge-platform` (added v1.5), `trustedge-types`, `trustedge-core`, `trustedge-cli`, `trustedge-trst-protocols`, `trustedge-trst-cli`, `trustedge-trst-wasm`.

- **Experimental tier (5 crates):** Community/experimental, build-only CI validation, no maintenance commitment. These crates have `tier = "experimental"` and `maintained = false` in their `[package.metadata.trustedge]` section.

CI is tiered: stable crate test failures block builds, experimental crate failures are informational only.

---

**End of dependency audit.**
