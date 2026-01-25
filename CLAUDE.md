# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Test Commands

```bash
# Full CI validation (run before committing)
./scripts/ci-check.sh

# Build workspace
cargo build --workspace --release

# Test entire workspace (150+ tests)
cargo test --workspace

# Test specific crates
cargo test -p trustedge-core --lib                # Core cryptography (101 tests)
cargo test -p trustedge-receipts                  # Digital receipts (23 tests)
cargo test -p trustedge-trst-cli --test acceptance # Archive validation (7 tests)

# Run a single test
cargo test -p trustedge-core test_name -- --nocapture

# Build/test with optional features
cargo build -p trustedge-cli --features audio     # Live audio capture CLI
cargo build -p trustedge-core --features yubikey  # YubiKey hardware support
cargo test --features yubikey --test yubikey_integration
```

## Architecture Overview

TrustEdge is a Cargo workspace with 10 crates under `crates/`:

**Core Platform:**
- `trustedge-core` - Core cryptographic library: envelope encryption (AES-256-GCM), Universal Backend system, network client/server, auth
- `trustedge-cli` - Main CLI for envelope encryption (binary: `trustedge`)
- `trustedge-receipts` - Digital receipt system with cryptographic ownership chains
- `trustedge-attestation` - Software attestation and provenance tracking
- `trustedge-wasm` - WebAssembly bindings for browser integration

**Pubky Network Integration:**
- `trustedge-pubky` - Simple adapter for key publishing/resolution using existing Ed25519 keys
- `trustedge-pubky-advanced` - Hybrid encryption with X25519 ECDH, forward secrecy, large file handling

**Archive System (.trst format):**
- `trustedge-trst-core` - Archive format primitives
- `trustedge-trst-cli` - CLI tool (binary: `trst`) for wrap/verify operations
- `trustedge-trst-wasm` - Browser verification

### Key Modules in `crates/core/src/`

| Module | Purpose |
|--------|---------|
| `backends/` | Universal Backend system - pluggable crypto ops (Software HSM, Keyring, YubiKey) |
| `transport/` | Network transport abstraction (TCP with framing, QUIC with TLS) |
| `envelope.rs` | Cryptographic envelope format with Ed25519 signatures |
| `crypto.rs` | XChaCha20-Poly1305 encryption, Ed25519 signing |
| `chain.rs` | BLAKE3-based continuity chain with genesis seed |
| `manifest.rs` | Canonical JSON serialization for cam.video profile |
| `auth.rs` | Ed25519 mutual authentication with sessions |
| `audio.rs` | Live audio capture (feature-gated) |

### Data Flow

1. Input → `InputReader` trait (file, audio stream)
2. Chunking (default 4KB)
3. Per-chunk AES-256-GCM encryption
4. Envelope creation with metadata manifest
5. Transport (local file or network)

## Code Standards

- **Formatting**: `cargo fmt` and `cargo clippy -- -D warnings` must pass
- **No emoji in code**: Use UTF-8 symbols for terminal output: ✔ ✖ ⚠ ● ♪ ■
- **Error handling**: `anyhow` for CLIs, `thiserror` for libraries; no `unwrap()` in production
- **Security**: Use `zeroize` for key material, constant-time comparisons for sensitive data
- **Copyright headers**: All `.rs` files need the MPL-2.0 header (run `./scripts/fix-copyright.sh`)

### Rust Copyright Header

```rust
//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//
```

## Key Patterns

### Universal Backend Pattern

All crypto operations use capability-based dispatch:

```rust
// Check capability before use
if backend.supports_operation(&operation) {
    let result = backend.perform_operation(key_id, operation)?;
}
```

### .trst Archive Structure

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
# Create archive
cargo run -p trustedge-trst-cli -- wrap --profile cam.video --in sample.bin --out archive.trst

# Verify archive
cargo run -p trustedge-trst-cli -- verify archive.trst --device-pub "ed25519:..."
```

## Feature Flags

| Feature | Purpose | Dependencies |
|---------|---------|--------------|
| `audio` | Live microphone capture | cpal (ALSA/CoreAudio/WASAPI) |
| `yubikey` | Hardware security keys | pkcs11, yubikey, x509-cert |

Default build has no features enabled for fast CI and maximum portability.

## CLI Binaries

| Binary | Source | Purpose |
|--------|--------|---------|
| `trustedge` | `crates/trustedge-cli/src/main.rs` | Main envelope encryption CLI |
| `trustedge-server` | `crates/core/src/bin/trustedge-server.rs` | Network server |
| `trustedge-client` | `crates/core/src/bin/trustedge-client.rs` | Network client |
| `trst` | `crates/trst-cli/src/main.rs` | Archive wrap/verify CLI |

## Common Tasks

### Adding Crypto Operations

1. Add to Universal Backend trait in `crates/core/src/backends/universal.rs`
2. Implement in relevant backends (`software_hsm.rs`, `yubikey.rs`)
3. Add tests including security scenarios
4. Update capability discovery

### Network Testing

```bash
# Terminal 1: Server
./target/release/trustedge-server --listen 127.0.0.1:8080 --decrypt --key-hex $(openssl rand -hex 32)

# Terminal 2: Client
./target/release/trustedge-client --server 127.0.0.1:8080 --input test.txt --key-hex $(cat shared.key)
```

### Debugging

```bash
RUST_LOG=debug cargo test failing_test -- --nocapture
```
