<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->

# TrustEdge Architecture

This document covers TrustEdge's internal architecture, crate organization, core systems, and testing infrastructure. For quick start and usage, see the [root README](../README.md).

---

## Project Structure

TrustEdge is organized as a Cargo workspace with specialized crates:

```
trustedge/
├── crates/
│   ├── core/                     # Core cryptographic library (trustedge-core)
│   ├── types/                    # Shared wire types (trustedge-types)
│   ├── platform/                 # Verification and CA service (trustedge-platform)
│   ├── platform-server/          # Standalone HTTP server binary
│   ├── trustedge-cli/            # Main envelope encryption CLI (binary: trustedge)
│   ├── trst-cli/                 # .trst archive CLI tool (trustedge-trst-cli, binary: trst)
│   ├── trst-protocols/           # Canonical cam.video manifest types (WASM-compatible)
│   ├── trst-wasm/                # .trst verification WebAssembly bindings
│   ├── wasm/                     # Core WebAssembly bindings
│   └── experimental/             # Separate workspace for community/experimental crates
│       ├── pubky/                # Pubky network adapter
│       └── pubky-advanced/       # Advanced Pubky integration
├── examples/                     # Example implementations and demos
└── docs/                         # Documentation and guides
```

> **Experimental crates** (`trustedge-pubky`, `trustedge-pubky-advanced`) live in `crates/experimental/` as a separate Cargo workspace. They are not part of the root workspace build or CI pipeline.

### Crate Overview

| Crate | Purpose | Documentation |
|-------|---------|---------------|
| **trustedge-types** | Shared wire types for platform services (verification, receipts, policies) | [Types Documentation](../crates/types/) |
| **trustedge-core** | Core cryptographic library with envelope encryption | [Core Documentation](../crates/core/) |
| **trustedge-platform** | Consolidated verification and CA service (feature-gated: http, postgres, ca, yubikey) | [Platform Documentation](../crates/platform/) |
| **trustedge-platform-server** | Standalone HTTP server binary | [Server Documentation](../crates/platform-server/) |
| **trustedge-cli** | Main CLI for envelope encryption (binary: `trustedge`) | [CLI Documentation](../crates/trustedge-cli/) |
| **trustedge-trst-cli** | CLI for .trst archive creation and verification (binary: `trst`) | [Archive CLI Documentation](../crates/trst-cli/) |
| **trustedge-trst-protocols** | Canonical cam.video manifest types (WASM-compatible) | [Archive Format Documentation](../crates/trst-protocols/) |
| **trustedge-trst-wasm** | .trst archive verification in the browser | [Archive WASM Documentation](../crates/trst-wasm/) |
| **trustedge-wasm** | WebAssembly bindings for browser/Node.js integration | [WASM Documentation](../crates/wasm/) |

---

## Technology Stack

- **Language**: Rust (stable) for memory safety and performance
- **Cryptography**: AES-256-GCM, Ed25519, X25519 ECDH, HKDF-SHA256, RSA OAEP-SHA256, BLAKE3 with algorithm agility
- **Key Files**: TRUSTEDGE-KEY-V1 format — PBKDF2-HMAC-SHA256 (600k iterations) + AES-256-GCM encryption at rest
- **Audio**: Cross-platform support (Linux/ALSA, Windows/WASAPI, macOS/CoreAudio)
- **Hardware**: YubiKey PIV operations via `yubikey` crate and PCSC
- **Network**: Ed25519-based mutual authentication with X25519 ECDH session key derivation

---

## Data Flow

1. Input via `InputReader` trait (file, audio stream)
2. Chunking (default 4KB for envelope encryption, 1MB for .trst archives)
3. Per-chunk AES-256-GCM encryption
4. Envelope creation with metadata manifest
5. Transport (local file or network)

### Key Modules in `crates/core/src/`

| Module | Purpose |
|--------|---------|
| `backends/` | Universal Backend system - pluggable crypto ops (Software HSM, Keyring, YubiKey) |
| `transport/` | Network transport abstraction (TCP with framing, QUIC with TLS) |
| `envelope.rs` | **Core envelope format** - Ed25519 signed, AES-256-GCM encrypted chunks (used by receipts, attestation) |
| `crypto.rs` | XChaCha20-Poly1305 encryption, Ed25519 signing |
| `chain.rs` | BLAKE3-based continuity chain with genesis seed |
| `archive.rs` | .trst archive read/write and validation |
| `auth.rs` | Ed25519 mutual authentication with X25519 ECDH session key derivation |
| `audio.rs` | Live audio capture (feature-gated) |
| `hybrid.rs` | RSA hybrid encryption (asymmetric operations) |

---

## Core Systems

### Universal Backend System

TrustEdge features a **capability-based Universal Backend system** that provides pluggable cryptographic operations across different hardware and software backends.

**Supported Backends:**
- **Keyring Backend**: OS keyring integration for key derivation and storage
- **YubiKey Backend**: Hardware PIV operations (ECDSA P-256, RSA-2048) via `yubikey` crate
- **Software HSM**: In-memory cryptographic operations for development
- **TPM Backend**: Planned for future milestone

All crypto operations use capability-based dispatch:

```rust
// Check capability before use
if backend.supports_operation(&operation) {
    let result = backend.perform_operation(key_id, operation)?;
}
```

**For detailed backend documentation, see [docs/technical/universal-backend.md](technical/universal-backend.md).**

### Digital Receipt System

TrustEdge includes a **production-ready digital receipt system** that enables cryptographically secure ownership transfer of digital assets with comprehensive security testing.

**Key Properties:**
- Cryptographic ownership chains with hash binding
- Ed25519 signatures for authenticity and non-repudiation
- Attack resistance against tampering, replay, and forgery
- 23 comprehensive security tests covering all attack scenarios

**For complete receipt system documentation, see [crates/core/](../crates/core/).**

### Network Operations

TrustEdge supports secure client-server communication with **Mutual Authentication** using Ed25519 digital signatures and **automated X25519 ECDH key exchange** for session encryption.

**Security Features:**
- Mutual authentication between clients and servers
- Automated session encryption key derivation via X25519 ECDH (no out-of-band key sharing needed)
- Session isolation with time-limited cryptographic sessions
- Replay protection through challenge-response protocols with BLAKE3 domain-separated KDF
- Forward security with automatic session expiration

**For authentication setup and network security, see [docs/user/authentication.md](user/authentication.md).**

---

## .trst Archive Format

The P0 `.trst` specification includes:

- **Manifest Canonicalization**: Ordered JSON fields with signature exclusion
- **BLAKE3 Continuity Chain**: Genesis seed `blake3("trustedge:genesis")` with segment linking
- **XChaCha20-Poly1305 Encryption**: Per-segment encryption with unique nonces
- **Ed25519 Signatures**: Device key signing with "ed25519:BASE64" format
- **Archive Layout**: `clip-<id>.trst/` directory with manifest, signatures, and chunks

### Archive Directory Structure

```
clip-<id>.trst/
├── manifest.json           # Canonical cam.video manifest
├── signatures/
│   └── manifest.sig        # Detached Ed25519 signature
└── chunks/
    ├── 00000.bin           # Zero-padded chunk files
    └── ...
```

### Working with Archives

```bash
# Generate device keypair (passphrase prompted for encrypted key file)
cargo run -p trustedge-trst-cli -- keygen --out-key device.key --out-pub device.pub

# For CI/automation (unencrypted key — no passphrase)
cargo run -p trustedge-trst-cli -- keygen --out-key device.key --out-pub device.pub --unencrypted

# Create archive (cam.video profile)
cargo run -p trustedge-trst-cli -- wrap --profile cam.video --in sample.bin --out archive.trst --device-key device.key --device-pub device.pub

# Verify archive
cargo run -p trustedge-trst-cli -- verify archive.trst --device-pub "ed25519:..."

# Recover original data
cargo run -p trustedge-trst-cli -- unwrap archive.trst --device-key device.key --out recovered.bin
```

---

## Testing and Quality Assurance

TrustEdge includes a comprehensive test suite with **406 automated tests** across 9 workspace crates:

- **160+ Core Tests**: Envelope encryption, Universal Backend system, receipts, attestation, transport layer (includes 18 YubiKey simulation tests)
- **4+ Auth Integration Tests**: Mutual authentication, session management, ECDH session key derivation, key uniqueness
- **9 Hardware Integration Tests**: YubiKey PIV operations (require physical device, run manually)
- **7 Archive Tests**: .trst format wrap/verify, cryptographic validation, CLI integration (trustedge-trst-cli acceptance)
- **19+ Platform Tests**: Verification engine, HTTP round-trip, CORS, router parity
- **18 Type Tests**: Shared wire type validation (trustedge-types)
- **45+ Security Tests**: Timestamp validation, error handling, permissions, cryptographic correctness (v2.3–v2.4)

```bash
# Run complete test suite
./scripts/ci-check.sh

# Run tests by crate
cargo test -p trustedge-core --lib                # Core cryptography tests
cargo test -p trustedge-types                     # Type tests (18)
cargo test -p trustedge-trst-cli --test acceptance # Archive validation (7)
cargo test -p trustedge-platform --lib            # Platform unit tests
cargo test --features yubikey --test yubikey_integration  # Hardware tests (need YubiKey)
```

**For detailed testing procedures, see [docs/developer/testing.md](developer/testing.md).**

---

## Documentation Index

### User Guides
- **[CLI Reference](user/cli.md)** - Complete command-line interface documentation
- **[Examples](user/examples/README.md)** - Real-world usage examples and workflows
- **[Authentication Guide](user/authentication.md)** - Network security setup
- **[Troubleshooting](user/troubleshooting.md)** - Common issues and solutions

### Technical Reference
- **[Universal Backend](technical/universal-backend.md)** - Backend system architecture
- **[Binary Format](technical/format.md)** - File format specification
- **[Network Protocol](technical/protocol.md)** - Communication protocol details
- **[Security Model](../SECURITY.md)** - Security architecture and threat model

### Development
- **[Contributing](../CONTRIBUTING.md)** - How to contribute to TrustEdge
- **[Development Guide](developer/development.md)** - Development setup and workflows
- **[Testing Guide](developer/testing.md)** - Test procedures and validation
- **[Coding Standards](developer/coding-standards.md)** - Code style and conventions
