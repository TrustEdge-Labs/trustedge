<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


# Codebase Structure

**Analysis Date:** 2026-02-09

## Directory Layout

```
trustedge/
├── crates/                          # Cargo workspace members
│   ├── core/                        # Core cryptographic library (primary)
│   │   ├── src/
│   │   │   ├── lib.rs              # Module declarations and re-exports
│   │   │   ├── envelope.rs         # High-level envelope API
│   │   │   ├── crypto.rs           # XChaCha20-Poly1305, Ed25519 operations
│   │   │   ├── auth.rs             # Ed25519 mutual auth, sessions
│   │   │   ├── manifest.rs         # Canonical cam.video manifest
│   │   │   ├── chain.rs            # BLAKE3 continuity chain
│   │   │   ├── format.rs           # Algorithm enums and header structures
│   │   │   ├── archive.rs          # Archive I/O (.trst format)
│   │   │   ├── asymmetric.rs       # RSA/ECDSA key exchange
│   │   │   ├── audio.rs            # Live audio capture (feature-gated)
│   │   │   ├── backends/           # Pluggable crypto backends
│   │   │   │   ├── mod.rs          # Backend registry
│   │   │   │   ├── universal.rs    # Capability-based dispatch
│   │   │   │   ├── universal_registry.rs # Backend discovery
│   │   │   │   ├── universal_keyring.rs  # Keyring implementation
│   │   │   │   ├── software_hsm.rs # File-based key storage
│   │   │   │   ├── keyring.rs      # OS keyring integration
│   │   │   │   ├── yubikey.rs      # YubiKey PKCS#11 (1392 lines)
│   │   │   │   └── traits.rs       # Backend trait definitions
│   │   │   ├── transport/          # Network protocol abstraction
│   │   │   │   ├── mod.rs          # Transport trait, config, factory
│   │   │   │   ├── tcp.rs          # TCP with length-delimited codec
│   │   │   │   └── quic.rs         # QUIC with TLS 1.3
│   │   │   ├── bin/                # Binary entry points
│   │   │   │   ├── trustedge-server.rs        # Network server (1005 lines)
│   │   │   │   ├── trustedge-client.rs        # Network client (921 lines)
│   │   │   │   ├── software-hsm-demo.rs       # HSM demonstration
│   │   │   │   ├── yubikey-demo.rs            # YubiKey demonstration
│   │   │   │   └── inspect-trst.rs            # Archive inspector
│   │   │   ├── envelope_v2_bridge.rs # Format detection/migration
│   │   │   ├── vectors.rs          # Test vectors
│   │   │   ├── hybrid.rs           # Pubky integration (X25519 ECDH)
│   │   │   └── format.rs           # Algorithm enums (not data format!)
│   │   ├── tests/                  # Integration tests
│   │   ├── benches/                # Criterion benchmarks
│   │   ├── examples/               # Example programs
│   │   ├── demo_keys/              # Pre-generated keys for demos
│   │   ├── software_hsm_keys/      # HSM storage for testing
│   │   └── Cargo.toml              # Feature flags: audio, yubikey
│   │
│   ├── trustedge-cli/              # Main CLI application
│   │   ├── src/
│   │   │   └── main.rs             # CLI entry point (parses args, calls core)
│   │   └── Cargo.toml
│   │
│   ├── receipts/                   # Transferable ownership claims
│   │   ├── src/
│   │   │   ├── lib.rs              # Receipt type and chain logic
│   │   │   └── bin/                # Receipt demo/utility binaries
│   │   └── Cargo.toml
│   │
│   ├── attestation/                # Software attestation ("birth certificates")
│   │   ├── src/
│   │   │   ├── lib.rs              # Attestation type and creation logic
│   │   │   └── bin/                # Attestation utility binaries
│   │   └── Cargo.toml
│   │
│   ├── wasm/                       # WebAssembly bindings (general)
│   │   ├── src/
│   │   │   └── lib.rs              # wasm-bindgen exports
│   │   ├── js/                     # TypeScript/JS helpers
│   │   ├── examples/               # WASM usage examples
│   │   └── Cargo.toml              # Target: wasm32-unknown-unknown
│   │
│   ├── trst-core/                  # Archive manifest types (WASM-compatible)
│   │   ├── src/
│   │   │   ├── lib.rs              # Module re-exports
│   │   │   └── manifest.rs         # CamVideoManifest, DeviceInfo, etc.
│   │   └── Cargo.toml
│   │
│   ├── trst-cli/                   # Archive wrap/verify CLI
│   │   ├── src/
│   │   │   └── main.rs             # trst binary entry point
│   │   ├── tests/                  # Acceptance tests
│   │   └── Cargo.toml
│   │
│   ├── trst-wasm/                  # Archive verification in browser
│   │   ├── src/                    # wasm-bindgen for archive ops
│   │   ├── js/                     # JS interop
│   │   └── Cargo.toml
│   │
│   ├── pubky/                      # Pubky network integration (community)
│   │   ├── src/
│   │   │   └── lib.rs
│   │   ├── bin/                    # trustedge-pubky CLI binary
│   │   ├── examples/
│   │   ├── tests/
│   │   └── Cargo.toml
│   │
│   └── pubky-advanced/             # Hybrid encryption (community)
│       ├── src/
│       │   └── lib.rs
│       ├── examples/
│       └── Cargo.toml
│
├── examples/                        # End-to-end examples
│   └── cam.video/                  # cam.video profile example
│
├── docs/                            # Documentation
│   ├── developer/                  # Development guides
│   ├── technical/                  # Architecture/technical specs
│   ├── hardware/                   # Hardware integration guides
│   ├── user/                       # User documentation
│   └── legal/
│
├── scripts/                         # Build and utility scripts
│   ├── ci-check.sh                 # Full CI validation
│   └── fix-copyright.sh             # Add/update copyright headers
│
├── .github/                         # GitHub workflows and templates
│   ├── workflows/
│   └── ISSUE_TEMPLATE/
│
├── Cargo.toml                       # Workspace root (members + shared deps)
├── Cargo.lock                       # Lock file for reproducibility
├── CLAUDE.md                        # Claude Code instructions (this repo!)
├── CONTRIBUTING.md                  # Contribution guidelines
├── README.md                        # Main project documentation
├── FEATURES.md                      # Feature descriptions
├── SECURITY.md                      # Security policy
├── CHANGELOG.md                     # Release history
└── LICENSE                          # MPL-2.0
```

## Directory Purposes

**`crates/core/`:**
- Purpose: Core cryptographic library with all primitives and backends
- Contains: Envelope, crypto operations, backends, transport, manifests
- Key files: `src/lib.rs` (exports all public API), `src/envelope.rs` (main user-facing type)
- Exports to: All other crates via `use trustedge_core::{...};`

**`crates/core/src/backends/`:**
- Purpose: Pluggable key management abstraction
- Contains: Universal trait, Software HSM, Keyring, YubiKey, registry
- Key types: `UniversalBackend`, `CryptoOperation`, `BackendCapabilities`
- Pattern: Backends implement capability-based dispatch, not monolithic trait

**`crates/core/src/transport/`:**
- Purpose: Protocol abstraction for network operations
- Contains: TCP transport (length-delimited frames), QUIC transport (TLS 1.3)
- Abstraction: `Transport` trait with async operations
- Used by: Server/client binaries, optional in applications

**`crates/core/src/bin/`:**
- Purpose: Standalone network binaries
- `trustedge-server.rs`: Listen for encrypted chunks, validate, optionally decrypt
- `trustedge-client.rs`: Connect to server, send encrypted chunks
- Demos: Software HSM usage, YubiKey usage, archive inspection

**`crates/trustedge-cli/src/`:**
- Purpose: Single-file CLI for envelope encryption
- Contains: main.rs with ~800 lines of argument parsing + encryption logic
- Pattern: Simple dispatcher to core::Envelope API
- Features: File input, live audio input (optional), multiple backends

**`crates/receipts/src/`:**
- Purpose: Business logic for transferable claims
- Contains: Receipt type with issuer, beneficiary, amount, chain link
- Pattern: Serialized as JSON, packaged inside core::Envelope
- No cryptography; core handles signing/encryption

**`crates/attestation/src/`:**
- Purpose: Software "birth certificates" with provenance
- Contains: Attestation type with artifact hash, builder ID, timestamp
- Pattern: JSON-only or sealed-envelope output
- Depends on: core::Envelope for sealed format

**`crates/trst-core/src/`:**
- Purpose: Canonical .trst archive manifest types
- Contains: CamVideoManifest, DeviceInfo, CaptureInfo, ChunkInfo, SegmentInfo
- Dependency: Minimal (only serde, ed25519-dalek, serde_json)
- Reason: Shared by browser WASM verifier and Rust CLI

**`crates/trst-cli/src/`:**
- Purpose: Archive wrap/verify CLI operations
- Binary: `trst` command
- Responsibilities: Create .trst archives, verify archive signatures

**`crates/wasm/` and `crates/trst-wasm/`:**
- Purpose: WebAssembly browser integration
- Contains: wasm-bindgen exports, TypeScript helpers
- Targets: `wasm32-unknown-unknown`
- Used by: Browser-based verification and encryption

**`crates/pubky/` and `crates/pubky-advanced/`:**
- Purpose: Community integration with Pubky network
- Status: Not part of core product roadmap
- Location: Separate from core due to optional dependency (pubky crate)
- Separation: Core remains lightweight, Pubky is opt-in

## Key File Locations

**Entry Points:**
- `crates/trustedge-cli/src/main.rs`: CLI encryption/decryption tool
- `crates/core/src/bin/trustedge-server.rs`: Network server
- `crates/core/src/bin/trustedge-client.rs`: Network client
- `crates/trst-cli/src/main.rs`: Archive operations

**Configuration:**
- `Cargo.toml`: Workspace root with shared dependencies
- `crates/core/Cargo.toml`: Feature flags (audio, yubikey)
- `CLAUDE.md`: Build and architecture instructions
- `.github/workflows/`: CI/CD pipelines

**Core Logic:**
- `crates/core/src/lib.rs`: Main library exports and NetworkChunk definition
- `crates/core/src/envelope.rs`: High-level envelope abstraction
- `crates/core/src/crypto.rs`: Encryption/decryption with ChaCha20-Poly1305
- `crates/core/src/auth.rs`: Ed25519 mutual authentication
- `crates/core/src/backends/universal.rs`: Capability-based crypto dispatch
- `crates/core/src/transport/mod.rs`: Protocol abstraction

**Testing:**
- `crates/core/tests/`: Integration tests
- `crates/core/benches/`: Criterion benchmarks
- Test files follow `*.rs` naming in test directory

## Naming Conventions

**Files:**
- Rust source: `snake_case.rs` (e.g., `software_hsm.rs`, `universal_keyring.rs`)
- Binaries: `kebab-case` (e.g., `trustedge-server`, `trustedge-client`)
- Test files: `test_name.rs` in `tests/` directory
- Example files: Example programs in `examples/` directory

**Directories:**
- Crate names: `kebab-case` (e.g., `trustedge-cli`, `pubky-advanced`)
- Nested modules: `snake_case` (e.g., `backends/`, `transport/`)
- Feature directories: `kebab-case` (e.g., `demo_keys/`, `software_hsm_keys/`)

**Types:**
- Struct/enum names: `PascalCase` (e.g., `Envelope`, `NetworkChunk`, `UniversalBackend`)
- Trait names: `PascalCase` (e.g., `Transport`, `KeyBackend`, `InputReader`)
- Constants: `SCREAMING_SNAKE_CASE` (e.g., `NONCE_LEN`, `SESSION_TIMEOUT`)
- Enum variants: `PascalCase` (e.g., `AeadAlgorithm::Aes256Gcm`)

**Functions:**
- Public API: `snake_case` (e.g., `seal()`, `unseal()`, `encrypt_segment()`)
- Helper functions: `snake_case` (e.g., `derive_shared_encryption_key()`)

## Where to Add New Code

**New Cryptographic Primitive:**
- Primary location: `crates/core/src/crypto.rs` or new module in `crates/core/src/`
- Register in: `crates/core/src/lib.rs` (module declaration + re-export)
- Tests: `crates/core/tests/` (integration) or embedded `#[cfg(test)]` blocks
- Example: To add new signature algorithm, extend `format::SignatureAlgorithm` enum

**New Backend (Keyring, TPM, etc.):**
- Implementation: `crates/core/src/backends/{name}.rs`
- Integration: Update `backends/mod.rs` (pub mod + pub use)
- Trait: Implement `UniversalBackend` or `KeyBackend` as appropriate
- Registry: Update `backends/universal_registry.rs` discovery logic
- Example: YubiKey backend at `crates/core/src/backends/yubikey.rs` (3236 lines)

**New Network Protocol:**
- Implementation: `crates/core/src/transport/{protocol}.rs`
- Trait: Implement `Transport` async trait
- Factory: Add `TransportFactory::create_{protocol}()` method
- Config: Extend `TransportConfig` if needed
- Example: QUIC at `crates/core/src/transport/quic.rs` (965 lines)

**New Business Logic (Receipts-like):**
- Create new crate: `crates/{feature_name}/`
- Core dependency: `use trustedge_core::Envelope;`
- Serialization: Use serde + bincode for binary, serde_json for text
- Pattern: Define payload type, serialize, package in Envelope
- Example: Receipts at `crates/receipts/src/lib.rs`

**New CLI Tool:**
- Option 1 - Simple wrapper: Add subcommand to `crates/trustedge-cli/src/main.rs`
- Option 2 - Standalone binary: Create in `crates/core/src/bin/{tool}.rs`
- Option 3 - New crate: Create `crates/{tool}-cli/` for larger tools
- Pattern: Use clap for arguments, call core APIs, handle errors with anyhow

**Utilities/Helpers:**
- Shared crypto: `crates/core/src/` (will be used widely)
- Backend-specific: `crates/core/src/backends/` (e.g., `env.rs` for env var parsing)
- Serialization: `crates/core/src/format.rs` (algorithm enums, headers)
- CLI utilities: `crates/trustedge-cli/src/main.rs` or extract to module

**Tests:**
- Integration tests: `crates/{name}/tests/*.rs` (one file per test suite)
- Unit tests: Inline in source files with `#[cfg(test)]` modules
- Fixtures: Create subdirectory like `crates/core/tests/fixtures/`
- Test data: Reference pre-computed test vectors in `crates/core/src/vectors.rs`

## Special Directories

**`crates/core/demo_keys/`:**
- Purpose: Pre-generated Ed25519 key pairs for demonstrations
- Generated: No (committed to repo)
- Committed: Yes (for reproducible demos)
- Usage: Demo binaries and examples
- Security: Demo-only, never use in production

**`crates/core/software_hsm_keys/`:**
- Purpose: File storage for software HSM testing
- Generated: Yes (created during test runs)
- Committed: No (in .gitignore)
- Usage: Software HSM backend tests
- Contents: JSON files with encrypted keys

**`examples/cam.video/`:**
- Purpose: End-to-end example using cam.video profile
- Type: Workspace member showing integration
- Usage: Reference implementation for cam.video manifest

**`docs/`:**
- Purpose: User, developer, and technical documentation
- Structure: Organized by audience (user/, developer/, technical/, hardware/, legal/)
- Usage: Source for project documentation

**`scripts/`:**
- Purpose: Build automation and maintenance
- `ci-check.sh`: Full CI validation pipeline
- `fix-copyright.sh`: Adds/updates MPL-2.0 headers to .rs files

**`.planning/codebase/`:**
- Purpose: Generated codebase analysis documents (you are here!)
- Documents: ARCHITECTURE.md, STRUCTURE.md, CONVENTIONS.md, TESTING.md, STACK.md, INTEGRATIONS.md, CONCERNS.md
- Generated: By gsd:map-codebase command
- Consumed by: gsd:plan-phase and gsd:execute-phase orchestrators

---

*Structure analysis: 2026-02-09*
