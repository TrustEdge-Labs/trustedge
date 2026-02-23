<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Test Commands

```bash
# Full CI validation (run before committing)
./scripts/ci-check.sh

# Build workspace
cargo build --workspace --release

# Test entire workspace (265+ tests)
cargo test --workspace

# Test specific crates
cargo test -p trustedge-types                     # Shared wire types (18 tests)
cargo test -p trustedge-core --lib                # Core cryptography (160 tests)
cargo test -p trustedge-trst-cli --test acceptance # Archive validation (7 tests)
cargo test -p trustedge-platform --lib            # Platform unit tests (12 tests)
cargo test -p trustedge-platform --test verify_integration           # Verify integration (5 tests)
cargo test -p trustedge-platform --test verify_integration --features http  # All verify integration (7 tests)

# Run a single test
cargo test -p trustedge-core test_name -- --nocapture

# Build/test with optional features
cargo build -p trustedge-cli --features audio                        # Live audio capture CLI
cargo build -p trustedge-core --features yubikey                     # YubiKey hardware support
cargo build -p trustedge-platform --features "http,postgres,ca"      # Full platform service
cargo test --features yubikey --test yubikey_integration
```

### Dashboard (web/dashboard/)

```bash
# Install dependencies
cd web/dashboard && npm install

# Development server
cd web/dashboard && npm run dev

# Production build
cd web/dashboard && npm run build

# Type checking
cd web/dashboard && npm run check
```

The dashboard is a SvelteKit app that provides a web UI for managing devices and viewing verification receipts. It connects to the platform server API at the URL configured in `.env.local` (defaults to `http://localhost:3001`).

## Architecture Overview

TrustEdge is a Cargo workspace with 9 crates under `crates/` (plus `examples/cam.video`):

**Core Platform:**
- `trustedge-types` - Shared wire types for platform services (verification, receipts, policies); re-exported from trustedge-core
- `trustedge-core` - Core cryptographic library: envelope encryption (AES-256-GCM), Universal Backend system, network client/server, auth, receipts, attestation; re-exports trustedge-types
- `trustedge-platform` - Consolidated verification and CA service: BLAKE3+Ed25519 verify engine, JWKS key manager, Axum HTTP layer, PostgreSQL multi-tenant backend; feature flags: `http`, `postgres`, `ca`, `yubikey`, `openapi`
- `trustedge-platform-server` - Standalone HTTP server binary (Axum + clap CLI)
- `trustedge-cli` - Main CLI for envelope encryption (binary: `trustedge`)
- `trustedge-wasm` - WebAssembly bindings for browser integration

**Archive System (.trst format):**
- `trustedge-trst-protocols` - Canonical cam.video manifest types (WASM-compatible, minimal dependencies)
- `trustedge-trst-cli` - CLI tool (binary: `trst`) for wrap/verify operations
- `trustedge-trst-wasm` - Browser verification (imports manifest types from trst-protocols)

**Experimental Crates:**
- Experimental community crates (`trustedge-pubky`, `trustedge-pubky-advanced`) live in `crates/experimental/` as a separate standalone workspace. They are not part of the root workspace or CI pipeline.

**Web Dashboard:**
- `web/dashboard/` - SvelteKit admin dashboard for device management and receipt viewing

### Key Modules in `crates/core/src/`

| Module | Purpose |
|--------|---------|
| `backends/` | Universal Backend system - pluggable crypto ops (Software HSM, Keyring, YubiKey) |
| `transport/` | Network transport abstraction (TCP with framing, QUIC with TLS) |
| `envelope.rs` | **Core envelope format** - Ed25519 signed, AES-256-GCM encrypted chunks (used by receipts, attestation) |
| `crypto.rs` | XChaCha20-Poly1305 encryption, Ed25519 signing |
| `chain.rs` | BLAKE3-based continuity chain with genesis seed |
| `manifest.rs` | Canonical JSON serialization for cam.video profile |
| `auth.rs` | Ed25519 mutual authentication with X25519 ECDH session key derivation |
| `audio.rs` | Live audio capture (feature-gated) |
| `hybrid.rs` | RSA hybrid encryption (asymmetric operations) |

### Data Flow

1. Input → `InputReader` trait (file, audio stream)
2. Chunking (default 4KB)
3. Per-chunk AES-256-GCM encryption
4. Envelope creation with metadata manifest
5. Transport (local file or network)

### GitHub Organization

The TrustEdge-Labs GitHub org contains 3 repos:
- **trustedge** — This monorepo: Rust workspace + SvelteKit dashboard
- **trustedgelabs-website** — Product website
- **shipsecure** — Separate product

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

### trustedge-core

| Feature | Purpose | Dependencies |
|---------|---------|--------------|
| `audio` | Live microphone capture | cpal (ALSA/CoreAudio/WASAPI) |
| `yubikey` | Hardware security keys | yubikey, x509-cert, rcgen, der, spki, signature |

### trustedge-platform

| Feature | Purpose | Dependencies |
|---------|---------|--------------|
| `http` | Axum HTTP layer (verify, jwks, health endpoints) | axum, tower, tower-http, tokio |
| `postgres` | PostgreSQL multi-tenant backend (devices, receipts, orgs) | sqlx, bcrypt |
| `ca` | Certificate Authority service via UniversalBackend | trustedge-core, x509-parser |
| `openapi` | OpenAPI schema generation | utoipa |
| `yubikey` | YubiKey-backed CA operations | trustedge-core/yubikey |
| `test-utils` | Exports `create_test_app` for integration tests | (no new deps) |

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
# With authentication (ECDH-derived session key - no --key-hex needed):
# Terminal 1: Server
./target/release/trustedge-server --listen 127.0.0.1:8080 --require-auth --decrypt

# Terminal 2: Client
./target/release/trustedge-client --server 127.0.0.1:8080 --input test.txt --enable-auth --server-cert server.cert

# Without authentication (manual key sharing):
# Terminal 1: Server
./target/release/trustedge-server --listen 127.0.0.1:8080 --decrypt --key-hex $(openssl rand -hex 32)

# Terminal 2: Client
./target/release/trustedge-client --server 127.0.0.1:8080 --input test.txt --key-hex $(cat shared.key)
```

### Debugging

```bash
RUST_LOG=debug cargo test failing_test -- --nocapture
```
