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

# Test entire workspace (406+ tests)
cargo test --workspace

# Test specific crates
cargo test -p trustedge-types                     # Shared wire types (12 tests)
cargo test -p trustedge-core --lib                # Core cryptography (184 tests)
cargo test -p trustedge-trst-cli --test acceptance # Archive validation (28 tests)
cargo test -p trustedge-platform --lib            # Platform unit tests (18 tests)
cargo test -p trustedge-platform --test verify_integration           # Verify integration (9 tests)
cargo test -p trustedge-platform --test verify_integration --features http  # All verify integration (27 tests)

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
| `archive.rs` | .trst archive read/write and validation |
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

### Encrypted Key Files (TRUSTEDGE-KEY-V1)

Device private keys are encrypted at rest using PBKDF2-HMAC-SHA256 (600k iterations) + AES-256-GCM.
The format header is `TRUSTEDGE-KEY-V1`. A passphrase is prompted at runtime via `rpassword`.

For CI/automation where interactive prompts are not possible, use `--unencrypted`:
- `trst keygen --unencrypted` — generates plaintext key file
- `trst wrap --unencrypted` — reads key without passphrase prompt
- `trst unwrap --unencrypted` — reads key without passphrase prompt

Production devices should always use encrypted key files. The `--unencrypted` flag is an
explicit escape hatch and is never the default.

### Working with Archives

```bash
# Generate device keypair (encrypted at rest — passphrase prompted)
cargo run -p trustedge-trst-cli -- keygen --out-key device.key --out-pub device.pub

# For CI/automation (unencrypted key file — no passphrase)
cargo run -p trustedge-trst-cli -- keygen --out-key device.key --out-pub device.pub --unencrypted

# Create archive (generic profile, default; passphrase prompted if key is encrypted)
cargo run -p trustedge-trst-cli -- wrap --in sample.bin --out archive.trst --device-key device.key --device-pub device.pub

# Create archive (cam.video profile)
cargo run -p trustedge-trst-cli -- wrap --profile cam.video --in sample.bin --out archive.trst --device-key device.key --device-pub device.pub

# Create archive (sensor profile with geo)
cargo run -p trustedge-trst-cli -- wrap --profile sensor --in data.csv --out archive.trst --sample-rate 100 --unit celsius --sensor-model DHT22 --latitude 40.7 --longitude=-74.0 --device-key device.key --device-pub device.pub

# Verify archive locally
cargo run -p trustedge-trst-cli -- verify archive.trst --device-pub "ed25519:..."

# Decrypt and recover original data
cargo run -p trustedge-trst-cli -- unwrap archive.trst --device-key device.key --out recovered.bin

# Sign with YubiKey hardware (requires yubikey feature)
cargo run -p trustedge-trst-cli --features yubikey -- wrap --backend yubikey --in data.bin --out archive.trst --device-key device.key

# Submit to platform server for verification
cargo run -p trustedge-trst-cli -- emit-request --archive archive.trst --device-pub device.pub --out request.json --post http://localhost:3001/v1/verify
```

### Running the Demo

```bash
# Full demo (requires docker-compose stack running)
./scripts/demo.sh

# Local-only demo (no docker needed)
./scripts/demo.sh --local
```

## Feature Flags

### trustedge-core

| Feature | Purpose | Dependencies |
|---------|---------|--------------|
| `audio` | Live microphone capture | cpal (ALSA/CoreAudio/WASAPI) |
| `yubikey` | Hardware security keys | yubikey, x509-cert, rcgen, der, spki, signature |
| `git-attestation` | Git repository state attestation | git2 |
| `keyring` | OS keyring integration for key storage | keyring |
| `insecure-tls` | Skip TLS certificate verification (development only) | (no new deps) |

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

## Platform Environment Variables

The `trustedge-platform-server` binary reads configuration from environment variables (or a `.env` file via `dotenvy`):

| Variable | Default | Description |
|----------|---------|-------------|
| `PORT` | `3001` | HTTP server port. Must be a valid port number (0–65535); fails fast with error if invalid. |
| `RECEIPT_TTL_SECS` | `3600` | Verification receipt TTL in seconds (1 hour). Must be a valid integer. |
| `JWT_AUDIENCE` | `trustedge-platform` | Expected JWT audience claim for verification tokens. |
| `DATABASE_URL` | (required in release) | PostgreSQL connection URL. Required in release builds; defaults to localhost in debug. Requires `postgres` feature. |

See `deploy/.env.example` for the full template with all variables documented.

## CLI Binaries

| Binary | Source | Purpose |
|--------|--------|---------|
| `trustedge` | `crates/trustedge-cli/src/main.rs` | Main envelope encryption CLI |
| `trustedge-server` | `crates/core/src/bin/trustedge-server.rs` | Network server (TCP/QUIC transport) |
| `trustedge-client` | `crates/core/src/bin/trustedge-client.rs` | Network client |
| `trustedge-platform-server` | `crates/platform-server/src/main.rs` | Platform HTTP server (verify, JWKS, health endpoints) |
| `trst` | `crates/trst-cli/src/main.rs` | Archive keygen/wrap/verify/unwrap/emit-request CLI |

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
