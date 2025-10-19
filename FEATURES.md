<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->

# TrustEdge Feature Flags Reference

This document provides a comprehensive guide to all feature flags in the TrustEdge workspace and how they work together.

## 📋 Overview

TrustEdge uses Cargo feature flags to make optional dependencies and functionality opt-in. This keeps the default build fast and CI-friendly while allowing users to enable advanced features as needed.

**Default Philosophy**: The default build has **no features enabled** to ensure:
- Fast CI builds (no audio/hardware dependencies)
- Minimal binary size
- Maximum portability across platforms

## 🎯 Core Crate Features (`trustedge-core`)

### Default Configuration
```toml
[features]
default = []  # Empty by default - explicitly opt-in to features
```

### Available Features

#### 1. `audio` - Live Audio Capture
**Purpose**: Enables real-time microphone input with cross-platform support.

**Dependencies Added**:
- `cpal` (0.15) - Cross-platform audio I/O library

**Platforms Supported**:
- **Linux**: ALSA backend
- **macOS**: CoreAudio backend  
- **Windows**: WASAPI backend (untested - not in scope)

**Build Command**:
```bash
cargo build -p trustedge-core --features audio
```

**Test Command**:
```bash
cargo test -p trustedge-core --features audio
```

**What It Enables**:
- `AudioCapture` API in `src/audio.rs`
- Live audio streaming with configurable quality
- Audio chunk processing for edge AI applications

**Example Usage**:
```rust
use trustedge_core::AudioCapture;

let capture = AudioCapture::new()?;
let chunks = capture.start_recording()?;
for chunk in chunks {
    // Process audio in real-time
}
```

**CI Note**: This feature is **NOT** enabled in CI by default to avoid audio hardware dependencies.

---

#### 2. `yubikey` - Hardware Security Key Support
**Purpose**: Enables YubiKey PIV operations with real hardware signing and attestation.

**Dependencies Added**:
- `pkcs11` (0.5) - PKCS#11 interface for hardware tokens
- `yubikey` (0.7) - YubiKey-specific operations
- `x509-cert` (0.2) - X.509 certificate generation/validation
- `der` (0.7) - DER encoding/decoding
- `spki` (0.7) - Subject Public Key Info handling
- `signature` (2.2) - Signature trait implementations

**Build Command**:
```bash
cargo build -p trustedge-core --features yubikey
```

**Test Command**:
```bash
# Hardware detection tests (safe without YubiKey)
cargo test -p trustedge-core --features yubikey

# Real hardware tests (requires YubiKey inserted)
cargo test -p trustedge-core --features yubikey yubikey_hardware_tests
```

**What It Enables**:
- `YubiKeyBackend` implementation in `src/backends/yubikey.rs`
- Hardware-backed cryptographic operations
- PIV slot operations (signing, key generation, attestation)
- X.509 certificate generation on hardware
- Universal Backend integration with hardware capabilities

**Example Usage**:
```rust
use trustedge_core::backends::{YubiKeyBackend, YubiKeyConfig, UniversalBackend};

let config = YubiKeyConfig {
    default_slot: "9a",
    verbose: true,
    ..Default::default()
};

let backend = YubiKeyBackend::with_config(config)?;
let result = backend.perform_operation("9a", CryptoOperation::Sign { 
    data: b"test".to_vec(), 
    algorithm: SignatureAlgorithm::EcdsaP256 
})?;
```

**Binary Requirement**: The `yubikey-demo` binary is gated behind this feature:
```bash
cargo run -p trustedge-core --features yubikey --bin yubikey-demo
```

**CI Note**: This feature is tested in CI but skips actual hardware operations.

---

### Feature Combinations

#### Build Everything (Full Features)
```bash
cargo build -p trustedge-core --all-features
# Equivalent to:
cargo build -p trustedge-core --features audio,yubikey
```

#### Test Everything
```bash
cargo test -p trustedge-core --all-features
```

#### CI Check Script
The `crates/core/ci-check.sh` script tests all feature combinations:
```bash
cd crates/core && ./ci-check.sh
```

This runs:
1. `cargo clippy --all-targets --no-default-features` (baseline)
2. `cargo clippy --all-targets --features audio` (if available)
3. `cargo clippy --all-targets --features yubikey` (if available)
4. `cargo build --all-targets` (baseline)
5. `cargo test` (baseline)

---

## 🌐 WASM Crate Features

### `trustedge-wasm` (Core WASM Bindings)

```toml
[features]
default = ["wee_alloc"]  # Memory allocator optimization
```

**Purpose**: WebAssembly bindings for browser/Node.js cryptographic operations.

**Key Points**:
- `wee_alloc`: Tiny allocator for WASM (reduces binary size ~50KB)
- No optional features - always builds the same
- Target: `wasm32-unknown-unknown`

**Build Command**:
```bash
wasm-pack build crates/wasm --target web
```

**Test Command**:
```bash
wasm-pack test crates/wasm --chrome --headless
```

---

### `trustedge-trst-wasm` (Archive Verification WASM)

```toml
[features]
# No features defined - always builds the same
```

**Purpose**: WebAssembly bindings for `.trst` archive verification in browsers.

**Build Command**:
```bash
wasm-pack build crates/trst-wasm --target web --out-dir ../../web/demo/pkg
```

**What It Provides**:
- `verify_archive()` - Browser-side .trst verification
- Ed25519 signature validation
- BLAKE3 continuity chain checking
- Chunk hash verification

**Demo Usage**:
```bash
./scripts/build-wasm-demo.sh  # Builds and prepares demo
cd web/demo && npx serve .     # Serve at http://localhost:3000
```

---

## 📦 Other Crate Features

### `trustedge-attestation`

```toml
[features]
symbols = []  # Include debug symbols in attestation
```

**Purpose**: Enables debug symbol extraction for software attestation.

**Build Command**:
```bash
cargo build -p trustedge-attestation --features symbols
```

**What It Does**:
- Extracts function symbols from compiled binaries
- Includes symbol table in attestation artifacts
- Useful for debugging and verification

---

## 🔍 Feature Detection at Runtime

### Checking Available Features

The `BackendInfo` struct provides runtime feature detection:

```rust
use trustedge_core::backends::{UniversalBackend, BackendInfo};

let backend = /* ... */;
let info = backend.backend_info();

println!("Backend: {}", info.name);
println!("Hardware-backed: {}", info.hardware_backed);
println!("Attestation: {}", info.supports_attestation);
```

### Capability-Based Operations

```rust
use trustedge_core::backends::{CryptoOperation, SignatureAlgorithm};

let operation = CryptoOperation::Sign {
    data: b"test".to_vec(),
    algorithm: SignatureAlgorithm::EcdsaP256,
};

// Check before calling
if backend.supports_operation(&operation) {
    let result = backend.perform_operation(key_id, operation)?;
} else {
    println!("Backend does not support this operation");
}
```

---

## 🚀 Recommended Build Configurations

### Local Development (Full Features)
```bash
cargo build --workspace --all-features
cargo test --workspace --all-features
```

### CI/CD (Baseline Only)
```bash
cargo build --workspace  # No features
cargo test --workspace   # No features
```

### Production Deployment (Audio-Enabled Edge Device)
```bash
cargo build --release -p trustedge-core --features audio
```

### Production Deployment (Hardware-Backed Security)
```bash
cargo build --release -p trustedge-core --features yubikey
```

### Browser/WASM Deployment
```bash
wasm-pack build crates/trst-wasm --target web --release
```

---

## ⚠️ Common Issues

### 1. Audio Feature Not Working
**Symptom**: `AudioCapture` type not found
**Solution**: Add `--features audio` to your build command

### 2. YubiKey Binary Not Available
**Symptom**: `no bin target named yubikey-demo`
**Solution**: Add `--features yubikey` to enable the gated binary

### 3. WASM Build Failures
**Symptom**: `wasm-pack not found`
**Solution**: Install with `cargo install wasm-pack`

### 4. CI Failures with Audio
**Symptom**: CI fails with audio device errors
**Solution**: Audio feature should NOT be enabled in CI - use baseline tests

---

## 📚 Related Documentation

- **Universal Backend System**: See `docs/technical/universal-backend.md`
- **WASM Testing**: See `docs/developer/wasm-testing.md`
- **YubiKey Integration**: See examples in `crates/core/examples/yubikey_demo.rs`
- **Audio Capture**: See examples in `crates/core/src/audio.rs` tests

---

## 🔮 Future Features (Planned)

### `tpm` - TPM 2.0 Support
**Status**: Planned, not yet implemented
**Dependencies**: `tss-esapi` (TPM 2.0 Software Stack)
**Purpose**: Hardware attestation and sealed storage

### `secp256k1` - Bitcoin/Ethereum Curve Support
**Status**: Planned, not yet implemented  
**Dependencies**: `k256` crate
**Purpose**: Compatibility with Bitcoin/Ethereum ecosystems

### `post-quantum` - Post-Quantum Algorithms
**Status**: Research phase
**Dependencies**: `pqcrypto` or similar
**Purpose**: Future-proof cryptographic operations

---

## 💡 Best Practices

1. **Default to Minimal**: Start with no features, add as needed
2. **Test Feature Combinations**: Use `ci-check.sh` scripts to test all combinations
3. **Document Feature Requirements**: Note in README which features are needed for specific functionality
4. **CI Alignment**: Always run `./scripts/ci-check.sh` before pushing to match CI behavior
5. **Platform-Specific**: Only enable audio on platforms you can test
6. **Hardware-Specific**: Only enable yubikey if hardware is available for testing

---

For more information, see the main [README.md](README.md) and [CONTRIBUTING.md](CONTRIBUTING.md).
