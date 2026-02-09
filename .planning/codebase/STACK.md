# Technology Stack

**Analysis Date:** 2026-02-09

## Languages

**Primary:**
- Rust 2021 edition (stable toolchain) - All 10 crates in `crates/` directory use Rust as primary language

**Secondary:**
- WebAssembly (WASM) - Browser integration via `crates/wasm/` and `crates/trst-wasm/`
- JavaScript/TypeScript - WASM interop only, no native JS code in repository

## Runtime

**Environment:**
- Rust stable (dtolnay/rust-toolchain@stable via CI)
- Target platforms: Linux, macOS, Windows (via GitHub Actions)
- WASM targets: wasm32-unknown-unknown (for browser deployment)

**Package Manager:**
- Cargo (Rust standard)
- Lockfile: `Cargo.lock` present and committed (version control enabled with `--locked` flag in CI)

## Frameworks

**Core Cryptography:**
- `aes-gcm` 0.10.3 - AES-256-GCM symmetric encryption (AEAD)
- `ed25519-dalek` 2.2.0 - Ed25519 signing and verification
- `blake3` 1.5 - BLAKE3 hashing for continuity chains
- `chacha20poly1305` 0.10 - ChaCha20-Poly1305 AEAD cipher
- `p256` 0.13 - NIST P-256 elliptic curves (ECDSA, ECDH)
- `x25519-dalek` 2.0 - X25519 key exchange (hybrid encryption)
- `rsa` 0.9 - RSA hybrid encryption (Pubky integration only)
- `pbkdf2` 0.12 - Key derivation
- `hkdf` 0.12 - HKDF key expansion
- `rand`/`rand_core` 0.8/0.6 - Cryptographically secure random number generation

**Network Transport:**
- `quinn` 0.11.9 - QUIC protocol implementation (optional TLS transport)
- `rustls` 0.23 - TLS 1.3 for secure connections
- `tokio` 1.0 - Async runtime (full features enabled for networking)
- `tokio-util` 0.7 - Codec utilities for framed message transport
- `futures-util` 0.3 - Futures utilities for async operations

**Serialization:**
- `serde` 1.0 - Serialization framework
- `serde_json` 1.0 - JSON serialization (manifest format)
- `serde_bytes` 0.11 - Efficient byte serialization
- `bincode` 1.3 - Binary encoding (compact envelopes)

**CLI Framework:**
- `clap` 4.5 - Command-line argument parsing (derive macros)

**Ecosystem Integration (Optional):**
- `pubky` 0.5.4 - Decentralized key management network (experimental feature)
- `reqwest` 0.11.27 - HTTP client (used in `trustedge-pubky-advanced` and `trustedge-trst-cli`)

**Hardware Support (Feature-Gated):**
- `yubikey` 0.7 - YubiKey hardware security module support (feature: `yubikey`)
- `pkcs11` 0.5 - PKCS#11 interface for HSM operations (feature: `yubikey`)
- `x509-cert` 0.2 - X.509 certificate generation (feature: `yubikey`)
- `der`/`spki`/`signature` - Certificate encoding (feature: `yubikey`)

**Audio Processing (Feature-Gated):**
- `cpal` 0.15 - Cross-platform audio capture (feature: `audio`)

**Utilities:**
- `anyhow` 1.0 - Error handling (CLIs and examples)
- `thiserror` 1.0 - Error handling (libraries)
- `zeroize` 1.7 - Secure memory clearing for key material
- `hex` 0.4 - Hexadecimal encoding/decoding
- `chrono` 0.4 - Date/time handling with serde support
- `git2` 0.18 - Git operations (attestation provenance)
- `sha2` 0.10 - SHA-256 hashing
- `num-traits` 0.2 - Generic numeric operations

**WASM Bindings:**
- `wasm-bindgen` 0.2 - JavaScript/Rust interop
- `js-sys` 0.3 - JavaScript types and functions
- `serde-wasm-bindgen` 0.6 - Efficient WASM serialization
- `getrandom` 0.2 (with `js` feature) - Secure random in WASM
- `wasm-bindgen-futures` 0.4 - Promise integration
- `web-sys` 0.3 - Web API bindings (File, FileReader, Blob)
- `console_error_panic_hook` 0.1 - Better panic messages in browser

**Development/Testing:**
- `criterion` 0.5 - Benchmarking framework (benchmarks in `crates/core/benches/`)
- `assert_cmd` 2.0 - CLI testing
- `assert_fs` 1.0 - Filesystem assertions
- `tempfile` 3.8 - Temporary file handling in tests
- `predicates` 3.0 - Predicate assertions
- `tokio-test` 0.4 - Testing async code
- `wasm-bindgen-test` 0.3 - WASM test runner

## Configuration

**Environment:**
- Runtime configuration via command-line arguments (`clap` derive macros)
- Optional environment variables: `TRUSTEDGE_DEVICE_ID`, `TRUSTEDGE_SALT`, `BENCH_FAST`, `RUST_LOG`
- No `.env` file in repository (configuration-as-code approach)
- CI detection: checks for `CI`, `GITHUB_ACTIONS`, `GITLAB_CI`, `TRAVIS`, `CIRCLECI` env vars

**Build:**
- Workspace-level `Cargo.toml` at `/home/john/projects/github.com/trustedge/Cargo.toml`
- Profile optimization: `opt-level = "s"` (size-optimized) with LTO enabled
- Feature flags at crate level: `audio`, `yubikey`, `envelope` (for attestation)

**Workspace Structure:**
- 10 crates organized in `crates/` directory:
  - `core` - Core cryptographic library with transport layer
  - `trustedge-cli` - Main CLI binary
  - `receipts` - Digital receipt system
  - `attestation` - Software attestation and provenance
  - `wasm` - WebAssembly browser bindings
  - `pubky` - Pubky network adapter
  - `pubky-advanced` - Advanced Pubky hybrid encryption
  - `trst-core` - Canonical manifest types (WASM-compatible)
  - `trst-cli` - Archive wrap/verify CLI
  - `trst-wasm` - Browser archive verification

## Platform Requirements

**Development:**
- Rust stable toolchain with `clippy` and `rustfmt` components
- For audio feature: `libasound2-dev` (Linux ALSA), `pkg-config`
- For YubiKey feature: `libpcsclite-dev` (Linux PC/SC daemon), `pkgconf`
- Optional: `wasm-pack` for WASM builds

**Production:**
- **Native:** Linux, macOS, Windows (binaries produced by `cargo build --release`)
- **Browser:** WASM deployment requires `wasm-pack build` output (cdylib + rlib crate types)
- **Deployment targets:** GitHub Releases (via Actions), npm packages (WASM), Docker images (TBD)

**CI/CD:**
- GitHub Actions (`.github/workflows/ci.yml`, `wasm-tests.yml`, `copyright-check.yml`)
- Runs on `ubuntu-latest` with caching via `Swatinem/rust-cache@v2`
- Tests with `--locked` flag to ensure reproducible builds

---

*Stack analysis: 2026-02-09*
