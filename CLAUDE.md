<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/johnzilla/trustedge
-->


# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Quick Start Commands

### Essential Development Commands
```bash
# Complete CI checks (run before committing)
./scripts/ci-check.sh

# Build and test workflow
cargo build --workspace --release
cargo test --workspace
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --check --all

# Run tests for specific components
cargo test -p trustedge-core --lib        # Core cryptography tests (86)
cargo test -p trustedge-receipts          # Receipt system tests (23)
cargo test --features yubikey             # Hardware integration tests
```

### Build Variations
```bash
# Build with audio support
cargo build --release --features audio

# Build with YubiKey support
cargo build --release --features yubikey

# Build all features
cargo build --release --features audio,yubikey

# Build workspace
cargo build --workspace --release
```

### Testing Commands
```bash
# Full test suite (109 tests total)
cargo test --workspace

# Run specific test categories
cargo test --test roundtrip_integration    # End-to-end workflow tests
cargo test --test network_integration      # Client-server tests
cargo test --test yubikey_integration     # Hardware tests
cargo test software_hsm                   # HSM backend tests

# Manual smoke test
echo "test" > test.txt
./target/release/trustedge-core --input test.txt --envelope test.trst --key-out test.key
./target/release/trustedge-core --decrypt --input test.trst --out recovered.txt --key-hex $(cat test.key)
diff test.txt recovered.txt  # Should be identical
```

## Architecture Overview

### Workspace Structure
TrustEdge is a Cargo workspace with specialized crates organized under `crates/`:

**Core TrustEdge Platform:**
- **crates/core** (`trustedge-core`): Core cryptographic library and CLI tools
  - Envelope encryption with AES-256-GCM
  - Universal Backend system for key management
  - Network client/server binaries
  - Live audio capture (with `--features audio`)
  - YubiKey hardware integration (with `--features yubikey`)

- **crates/receipts** (`trustedge-receipts`): Digital receipt system with cryptographic ownership transfer
- **crates/attestation** (`trustedge-attestation`): Software attestation and verification system
- **crates/wasm** (`trustedge-wasm`): WebAssembly bindings for browser integration
- **crates/pubky** (`trustedge-pubky`): Pubky network adapter for decentralized keys
- **crates/pubky-advanced** (`trustedge-pubky-advanced`): Advanced Pubky integration

**Archive System (.trst format):**
- **crates/trst-core** (`trustedge-trst-core`): Archive format primitives for .trst files
- **crates/trst-cli** (`trustedge-trst-cli`): CLI tool for creating/verifying .trst archives (binary: `trst`)
- **crates/trst-wasm** (`trustedge-trst-wasm`): Browser verification of .trst archives

### Key Systems

**Universal Backend System**: Pluggable cryptographic operations
- Software HSM backend (in-memory operations)
- Keyring backend (OS keyring integration)
- YubiKey backend (PKCS#11 hardware operations)

**Data-Agnostic Encryption**: Handles any data type with metadata preservation
- File encryption with MIME type detection
- Live audio capture with format metadata
- Chunked streaming for large files

**Network Operations**: Secure client-server communication
- TCP transport with length framing
- QUIC transport with TLS (planned)
- Mutual authentication with Ed25519 signatures

### Core Data Flow
1. Input → InputReader trait (file, audio, etc.)
2. Data chunking (default 4KB chunks)
3. Per-chunk AES-256-GCM encryption
4. Envelope creation with metadata manifest
5. Transport (local file or network)

## Development Guidelines

### Code Standards
- Follow Rust conventions: `cargo fmt` and `cargo clippy -- -D warnings`
- NO emoji or special Unicode characters in code, comments, or strings
- Use professional UTF-8 symbols in terminal output: ✔ ✖ ⚠ ● ♪ ■
- Explicitly specify UTF-8 in all file operations and configurations
- Test character encoding scenarios before claiming implementation is complete
- No `unwrap()` or `panic!()` in production code
- All public APIs must have rustdoc documentation
- Use `anyhow` for CLI errors, `thiserror` for library errors
- do not add your name to any copyrights or git commit messages

### API Integration Requirements  
- ALWAYS read API documentation completely before implementing external service calls
- Validate all assumptions by checking official documentation first  
- Never assume API behavior - verify with documentation
- Include comprehensive error handling for all API calls

### Rust-Specific Standards
- Follow Rust best practices and idioms for this privacy platform
- Use proper error handling with Result types
- Include comprehensive documentation for public interfaces
- Write descriptive variable and function names
- Add inline documentation for complex TPM/hardware security logic

### Security Requirements
- Use established crypto libraries (aes-gcm, ed25519-dalek)
- Implement proper key zeroization with `zeroize` crate
- Validate all external inputs
- Use constant-time operations for sensitive comparisons

### Testing Approach
- Perform all tests
- Never use fake data, placeholders or automatic assertions unless clearly allowed and explained
- Unit tests co-located with code (`#[cfg(test)]`)
- Integration tests in `tests/` directory
- Property-based testing for crypto functions
- Test vectors for format validation

### Feature Flags
- `audio`: Enables live audio capture (requires system audio libraries)
- `yubikey`: Enables YubiKey hardware backend (requires PKCS#11)

## Common Tasks

### Implementation Process
1. Read relevant API/library documentation thoroughly
2. Identify all requirements and constraints  
3. Plan implementation with explicit UTF-8 considerations
4. Validate compliance throughout codebase
5. Ask clarifying questions about unclear requirements

### Adding New Cryptographic Operations
1. Add to Universal Backend trait in `trustedge-core/src/backends/universal.rs`
2. Implement in relevant backends (software_hsm.rs, yubikey.rs)
3. Add comprehensive tests including security scenarios
4. Update capability discovery system

### Working with Audio Features
```bash
# Check audio device availability
./target/release/trustedge-core --list-audio-devices

# Test audio capture
./target/release/trustedge-core --live-capture --max-duration 5 --envelope test.trst --key-out test.key
```

### Network Testing
```bash
# Start server
./target/release/trustedge-server --listen 127.0.0.1:8080 --decrypt --key-hex $(openssl rand -hex 32)

# Test client
./target/release/trustedge-client --server 127.0.0.1:8080 --input test.txt --key-hex $(cat shared.key)
```

### Working with YubiKey
Requires YubiKey with PIV applet and PKCS#11 module installed:
```bash
# Generate test keys
ykman piv keys generate 9a /tmp/pubkey.pem

# Run YubiKey tests
cargo test --features yubikey --test yubikey_integration
```

## Important Files

### CLI Binaries
- `trustedge-core/src/main.rs`: Main CLI application
- `trustedge-core/src/bin/trustedge-server.rs`: Network server
- `trustedge-core/src/bin/trustedge-client.rs`: Network client
- `trustedge-core/src/bin/inspect-trst.rs`: Metadata inspection utility

### Key Modules
- `trustedge-core/src/backends/`: Universal Backend system
- `trustedge-core/src/transport/`: Network transport abstraction
- `trustedge-core/src/format.rs`: TrustEdge envelope format
- `trustedge-core/src/audio.rs`: Live audio capture (feature-gated)
- `trustedge-core/src/auth.rs`: Ed25519 mutual authentication

### Test Suites
- `trustedge-core/tests/roundtrip_integration.rs`: End-to-end workflows
- `trustedge-core/tests/network_integration.rs`: Client-server communication
- `trustedge-core/tests/yubikey_integration.rs`: Hardware integration
- `trustedge-receipts/tests/`: Digital receipt security tests

## Error Handling Patterns

### Common Error Types
- `EncryptionError`: Cryptographic operation failures
- `FormatError`: Envelope format issues
- `NetworkError`: Connection and transport failures
- `AuthError`: Authentication and certificate issues
- `AudioError`: Audio capture and device issues

### Debugging Tips
```bash
# Run with debug output
RUST_LOG=debug cargo test failing_test -- --nocapture

# Check format compliance
cargo test vectors::tests::golden_trst_digest_is_stable

# Verify YubiKey connectivity
ykman piv info
```

## Performance Considerations

- Streaming processing maintains <50MB RAM usage regardless of file size
- Target >10MB/s encryption throughput
- Default 4KB chunk size balances memory usage and performance
- Network operations designed for 100+ concurrent connections

This codebase prioritizes security, performance, and maintainability. When in doubt, favor explicit error handling, comprehensive testing, and clear documentation.