<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->

# TrustEdge Core

> **STABLE** -- This crate is Tier 1 (Stable). Production-committed, tested in CI, and actively maintained.

**Core cryptographic library and CLI tools for privacy-preserving edge computing.**

[![Crates.io](https://img.shields.io/crates/v/trustedge-core.svg)](https://crates.io/crates/trustedge-core)
[![Documentation](https://docs.rs/trustedge-core/badge.svg)](https://docs.rs/trustedge-core)
[![License: MPL 2.0](https://img.shields.io/badge/License-MPL_2.0-brightgreen.svg)](https://opensource.org/licenses/MPL-2.0)

---

## Overview

TrustEdge Core is the **foundational crate** of the TrustEdge ecosystem, providing production-ready cryptographic primitives, CLI applications, and system architecture for privacy-preserving edge computing. It implements data-agnostic encryption, universal backend systems, and secure network operations.

### Key Features

- **🔐 Production Cryptography**: AES-256-GCM encryption with PBKDF2 key derivation (100k iterations)
- **🏗️ Universal Backend System**: Pluggable crypto operations (Software HSM, Keyring, YubiKey)
- **🎵 Live Audio Capture**: Real-time microphone input with configurable quality and device selection
- **🌐 Network Operations**: Secure client-server communication with mutual authentication
- **🔑 Hardware Integration**: Full YubiKey PKCS#11 support with real hardware signing
- **⚡ Algorithm Agility**: Configurable cryptographic algorithms with forward compatibility
- **📋 Format-Aware Processing**: MIME type detection and format-preserving encryption/decryption
- **🛡️ Memory Safety**: Proper key material cleanup with zeroization

[↑ Back to top](#trustedge-core)

---

## Architecture

TrustEdge Core provides both a **library** and **CLI applications**:

```
trustedge-core/
├── src/lib.rs                   # Core library exports
├── src/bin/                     # CLI tools
│   ├── trustedge-server.rs      # Network server
│   ├── trustedge-client.rs      # Network client
│   ├── software-hsm-demo.rs     # Software HSM demonstration
│   ├── inspect-trst.rs          # .trst archive inspector
│   └── yubikey-demo.rs          # YubiKey hardware operations
├── src/backends/                # Universal Backend system
├── src/transport/               # Network transport layer
├── examples/                    # Comprehensive examples
└── tests/                       # Test suite (160 tests)
```

### Core Modules

| Module | Purpose | Key Types |
|--------|---------|-----------|
| **envelope** | Cryptographic envelope format | `Envelope`, `EnvelopeMetadata` |
| **backends** | Universal Backend system | `UniversalBackend`, `KeyringBackend` |
| **audio** | Live audio capture | `AudioCapture`, `AudioConfig` |
| **auth** | Network authentication | `SessionManager`, `AuthChallenge` |
| **transport** | Network operations | `Transport`, `TransportConfig` |
| **asymmetric** | Public key cryptography | `KeyPair`, `PrivateKey`, `PublicKey` |
| **format** | Data format handling | `DataType`, `NetworkChunk` |

[↑ Back to top](#trustedge-core)

---

## Quick Start

### Library Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
trustedge-core = "0.2.0"
```

**Basic encryption/decryption:**

```rust
use trustedge_core::Envelope;
use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;

// Generate key pairs
let sender_key = SigningKey::generate(&mut OsRng);
let recipient_key = SigningKey::generate(&mut OsRng);

// Encrypt data
let data = b"Secret message";
let envelope = Envelope::seal(data, &sender_key, &recipient_key.verifying_key())?;

// Decrypt data
let decrypted = envelope.unseal(&recipient_key)?;
assert_eq!(decrypted, data);
```

**Universal Backend usage:**

```rust
use trustedge_core::backends::{UniversalBackend, CryptoOperation};

// Create backend
let backend = UniversalBackend::keyring()?;

// Perform operations
let operation = CryptoOperation::DeriveKey {
    domain: "trustedge".to_string(),
    purpose: "encryption".to_string(),
};
let result = backend.perform_operation("my_key", operation)?;
```

### CLI Applications

**Main CLI (`trustedge-core`):**
```bash
# Encrypt a file
./target/release/trustedge-core --input document.txt --envelope document.trst --key-out key.hex

# Decrypt a file
./target/release/trustedge-core --decrypt --input document.trst --out recovered.txt --key-hex $(cat key.hex)

# Live audio capture
./target/release/trustedge-core --live-capture --envelope voice.trst --key-out voice.key --max-duration 10
```

**Network Server:**
```bash
# Start authenticated server
./target/release/trustedge-server --listen 127.0.0.1:8080 --require-auth --decrypt
```

**Network Client:**
```bash
# Connect with authentication
./target/release/trustedge-client --server 127.0.0.1:8080 --input file.txt --require-auth
```

[↑ Back to top](#trustedge-core)

---

## Core Systems

### Universal Backend System

The Universal Backend provides **pluggable cryptographic operations** across different backends:

```rust
use trustedge_core::backends::{UniversalBackend, BackendCapabilities};

// Discover available backends
let registry = UniversalBackend::registry();
let backends = registry.discover_backends()?;

// Use specific backend
let yubikey_backend = UniversalBackend::yubikey()?;
if yubikey_backend.supports_operation(&operation) {
    let result = yubikey_backend.perform_operation("key_id", operation)?;
}
```

**Supported Backends:**
- **Keyring Backend**: OS keyring integration for key derivation
- **YubiKey Backend**: Hardware PIV operations with PKCS#11
- **Software HSM**: In-memory cryptographic operations
- **TPM Backend**: TPM 2.0 operations (planned)

### Envelope System

TrustEdge uses a **secure envelope format** for data protection:

```rust
use trustedge_core::Envelope;
use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;

// Generate keys
let sender_key = SigningKey::generate(&mut OsRng);
let recipient_key = SigningKey::generate(&mut OsRng);
let data = b"example data";

// Create envelope
let envelope = Envelope::seal(
    data,
    &sender_key,
    &recipient_key.verifying_key(),
)?;

// Inspect without decrypting
let info = envelope.inspect()?;
println!("Envelope hash: {:?}", envelope.hash());
println!("Beneficiary: {:?}", envelope.beneficiary());
```

### Audio Capture System

Real-time audio capture with **format-aware processing**:

```rust
use trustedge_core::{AudioCapture, AudioConfig};

// Configure audio capture
let config = AudioConfig {
    sample_rate: 44100,
    channels: 1,
    device_name: None, // Use default device
};

// Capture audio
let mut capture = AudioCapture::new(config)?;
let audio_data = capture.record_for_duration(std::time::Duration::from_secs(10))?;

// Encrypt captured audio
let envelope = Envelope::seal(&audio_data, &sender_private, &recipient_public)?;
```

### Network Authentication

**Mutual authentication** with Ed25519 signatures:

```rust
use trustedge_core::auth::{SessionManager, ServerCertificate};

// Server setup
let server_keys = KeyPair::generate(AsymmetricAlgorithm::Ed25519)?;
let mut session_manager = SessionManager::new();

// Client authentication
let client_keys = KeyPair::generate(AsymmetricAlgorithm::Ed25519)?;
let auth_result = client_authenticate(&client_keys, &server_cert)?;
```

[↑ Back to top](#trustedge-core)

---

## CLI Applications

### Main CLI (`trustedge-core`)

The primary command-line interface for TrustEdge operations:

**File Operations:**
```bash
# Basic encryption
trustedge-core --input file.txt --envelope file.trst --key-out key.hex

# Keyring-based encryption
trustedge-core --input file.txt --envelope file.trst --use-keyring --salt-hex $(openssl rand -hex 16)

# Format inspection
trustedge-core --input file.trst --inspect --verbose
```

**Audio Operations:**
```bash
# List audio devices
trustedge-core --list-audio-devices

# Capture with specific device
trustedge-core --live-capture --audio-device "hw:CARD=USB,DEV=0" --envelope audio.trst --key-out audio.key
```

### Network Applications

**Server (`trustedge-server`):**
```bash
# Basic server
trustedge-server --listen 0.0.0.0:8080

# Authenticated server with decryption
trustedge-server --listen 0.0.0.0:8080 --require-auth --decrypt --verbose
```

**Client (`trustedge-client`):**
```bash
# Send file to server
trustedge-client --server 192.168.1.100:8080 --input document.txt

# Authenticated transfer
trustedge-client --server 192.168.1.100:8080 --input document.txt --require-auth
```

### Hardware Demonstrations

**Software HSM Demo:**
```bash
# Generate key
software-hsm-demo generate-key my_key ed25519

# Sign data
software-hsm-demo sign my_key "Hello TrustEdge!"

# List keys
software-hsm-demo list-keys
```

**YubiKey Demo (requires `--features yubikey`):**
```bash
# YubiKey capabilities
yubikey-demo -p /usr/lib/x86_64-linux-gnu/opensc-pkcs11.so -v capabilities

# Generate certificate
yubikey-demo -p /usr/lib/x86_64-linux-gnu/opensc-pkcs11.so generate-cert
```

[↑ Back to top](#trustedge-core)

---

## Examples

### Example 1: Basic Library Usage

```rust
use trustedge_core::Envelope;
use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Generate keys
    let alice_key = SigningKey::generate(&mut OsRng);
    let bob_key = SigningKey::generate(&mut OsRng);
    
    // Alice encrypts for Bob
    let message = b"Hello Bob from Alice!";
    let envelope = Envelope::seal(message, &alice_key, &bob_key.verifying_key())?;
    
    // Bob decrypts
    let decrypted = envelope.unseal(&bob_key)?;
    assert_eq!(decrypted, message);
    
    println!("✅ Encryption/decryption successful");
    Ok(())
}
```

### Example 2: Universal Backend

```rust
use trustedge_core::backends::{UniversalBackend, CryptoOperation};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create keyring backend
    let backend = UniversalBackend::keyring()?;
    
    // Derive key
    let operation = CryptoOperation::DeriveKey {
        domain: "trustedge".to_string(),
        purpose: "file_encryption".to_string(),
    };
    
    let result = backend.perform_operation("user_key", operation)?;
    println!("✅ Key derivation successful");
    Ok(())
}
```

### Example 3: Audio Capture

```rust
#[cfg(feature = "audio")]
use trustedge_core::{AudioCapture, AudioConfig, Envelope};
use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;

#[cfg(feature = "audio")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup audio capture
    let config = AudioConfig {
        sample_rate: 44100,
        channels: 1,
        device_name: None,
    };
    
    let mut capture = AudioCapture::new(config)?;
    
    // Record 5 seconds
    let audio_data = capture.record_for_duration(std::time::Duration::from_secs(5))?;
    println!("Captured {} bytes of audio", audio_data.len());
    
    // Encrypt audio
    let key = SigningKey::generate(&mut OsRng);
    let envelope = Envelope::seal(&audio_data, &key, &key.verifying_key())?;
    
    println!("✅ Audio capture and encryption successful");
    Ok(())
}

#[cfg(not(feature = "audio"))]
fn main() {
    println!("Audio features not enabled. Build with --features audio");
}
```

### Example 4: Network Operations

```rust
use trustedge_core::auth::{SessionManager, ServerCertificate};
use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Server setup
    let server_key = SigningKey::generate(&mut OsRng);
    let server_cert = ServerCertificate::new(&server_key)?;
    let mut session_manager = SessionManager::new();
    
    // Client setup
    let client_key = SigningKey::generate(&mut OsRng);
    
    println!("✅ Network authentication setup complete");
    Ok(())
}
```

[↑ Back to top](#trustedge-core)

---

## Features

### Cargo Features

| Feature | Description | Default |
|---------|-------------|---------|
| `audio` | Enable live audio capture functionality | No |
| `yubikey` | Enable YubiKey hardware backend | No |
| `network` | Enable network transport features | Yes |
| `keyring` | Enable OS keyring integration | Yes |

**Build with features:**
```bash
# Audio support
cargo build --features audio

# YubiKey support  
cargo build --features yubikey

# All features
cargo build --features audio,yubikey
```

### System Dependencies

**Audio Features:**
```bash
# Ubuntu/Debian
sudo apt-get install libasound2-dev pkg-config

# macOS (included with Xcode)
# No additional packages needed

# Windows (included with Windows SDK)
# No additional packages needed
```

**YubiKey Features:**
```bash
# Ubuntu/Debian
sudo apt-get install opensc-pkcs11

# macOS
brew install opensc

# Windows
# Download OpenSC from https://github.com/OpenSC/OpenSC/releases
```

[↑ Back to top](#trustedge-core)

---

## Testing

TrustEdge Core includes **160 comprehensive tests** covering all functionality:

```bash
# Run all tests
cargo test

# Run with features
cargo test --features audio,yubikey

# Run specific test categories
cargo test envelope
cargo test backends
cargo test audio
cargo test auth

# Run benchmarks
cargo bench
```

**Test Categories:**
- **Envelope Tests**: Encryption/decryption, format handling
- **Backend Tests**: Universal Backend system, keyring integration
- **Audio Tests**: Live capture, format detection
- **Authentication Tests**: Mutual auth, session management
- **Transport Tests**: Network operations, error handling
- **Hardware Tests**: YubiKey integration (requires hardware)

### Performance Testing

```bash
# Quick benchmarks
./fast-bench.sh

# Full benchmark suite
cargo bench

# Performance analysis
cargo run --example transport_demo --release
```

[↑ Back to top](#trustedge-core)

---

## API Reference

### Core Types

#### `Envelope`
Secure cryptographic envelope for data protection:

```rust
impl Envelope {
    pub fn seal(data: &[u8], sender_private: &PrivateKey, recipient_public: &PublicKey) -> Result<Self>;
    pub fn unseal(&self, recipient_private: &PrivateKey) -> Result<Vec<u8>>;
    pub fn inspect(&self) -> Result<EnvelopeInfo>;
    pub fn verify(&self) -> bool;
}
```

#### `UniversalBackend`
Pluggable cryptographic backend system:

```rust
impl UniversalBackend {
    pub fn keyring() -> Result<Self>;
    pub fn yubikey() -> Result<Self>;
    pub fn software_hsm() -> Result<Self>;
    pub fn perform_operation(&self, key_id: &str, operation: CryptoOperation) -> Result<CryptoResult>;
    pub fn supports_operation(&self, operation: &CryptoOperation) -> bool;
}
```

#### `AudioCapture`
Live audio capture functionality:

```rust
impl AudioCapture {
    pub fn new(config: AudioConfig) -> Result<Self>;
    pub fn record_for_duration(&mut self, duration: Duration) -> Result<Vec<u8>>;
    pub fn list_devices() -> Result<Vec<String>>;
}
```

### Error Handling

TrustEdge Core provides comprehensive error types:

```rust
use trustedge_core::TrustEdgeError;

match operation_result {
    Ok(data) => println!("Success: {} bytes", data.len()),
    Err(TrustEdgeError::CryptographicError(e)) => eprintln!("Crypto error: {}", e),
    Err(TrustEdgeError::NetworkError(e)) => eprintln!("Network error: {}", e),
    Err(TrustEdgeError::AudioError(e)) => eprintln!("Audio error: {}", e),
    Err(e) => eprintln!("Other error: {}", e),
}
```

[↑ Back to top](#trustedge-core)

---

## Performance

### Benchmarks

TrustEdge Core is optimized for performance:

| Operation | Throughput | Latency |
|-----------|------------|---------|
| **File Encryption** | ~500 MB/s | ~2ms |
| **File Decryption** | ~600 MB/s | ~1.5ms |
| **Key Generation** | ~1000 keys/s | ~1ms |
| **Audio Capture** | Real-time | <10ms latency |
| **Network Auth** | ~500 auths/s | ~2ms |

### Memory Usage

- **Base Library**: ~2MB memory footprint
- **Audio Capture**: +~5MB for buffers
- **YubiKey Backend**: +~1MB for PKCS#11
- **Network Operations**: +~3MB for connections

### Optimization Tips

1. **Reuse Keys**: Generate key pairs once and reuse
2. **Batch Operations**: Process multiple files together
3. **Streaming**: Use chunked processing for large files
4. **Backend Selection**: Choose appropriate backend for use case

```rust
use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;

// Efficient batch processing
let key = SigningKey::generate(&mut OsRng);
let files = vec!["file1.txt", "file2.txt", "file3.txt"];

for file in files {
    let data = std::fs::read(file)?;
    let envelope = Envelope::seal(&data, &key, &key.verifying_key())?;
    // Process envelope...
}
```

---

## Integration

### With Other TrustEdge Crates

```rust
// Receipts (now consolidated in core)
use trustedge_core::create_receipt;
use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;

let key = SigningKey::generate(&mut OsRng);
let receipt = create_receipt(&key, &key.verifying_key(), 1000, None)?;

// Attestation (now consolidated in core)
use trustedge_core::{create_signed_attestation, AttestationConfig};

// With trustedge-wasm
use trustedge_core::Envelope;
// Export envelope functionality to WebAssembly

// With trustedge-pubky (community/experimental)
use trustedge_core::backends::UniversalBackend;
// Use core backends with Pubky network integration
```

### External Integration

```rust
// With tokio for async operations
use tokio::fs;
use trustedge_core::Envelope;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = fs::read("file.txt").await?;
    let envelope = Envelope::seal(&data, &sender_private, &recipient_public)?;
    fs::write("file.trst", envelope.serialize()?).await?;
    Ok(())
}
```

[↑ Back to top](#trustedge-core)

---

## Contributing

We welcome contributions to TrustEdge Core:

1. **Core Cryptography**: Improve encryption/decryption performance
2. **Backend Development**: Add new Universal Backend implementations
3. **Audio Processing**: Enhance audio capture capabilities
4. **Network Features**: Improve transport layer functionality
5. **Hardware Integration**: Expand hardware security module support

See [CONTRIBUTING.md](../CONTRIBUTING.md) for detailed guidelines.

### Development Setup

```bash
# Clone repository
git clone https://github.com/TrustEdge-Labs/trustedge.git
cd trustedge/trustedge-core

# Run tests
cargo test

# Run with all features
cargo test --features audio,yubikey

# Run examples
cargo run --example universal_backend_demo
cargo run --example transport_demo

# Check formatting
cargo fmt --check
```

[↑ Back to top](#trustedge-core)

---

## Documentation

### Crate-Specific Documentation
- **[AUTHENTICATION.md](AUTHENTICATION.md)** - Network authentication details
- **[BENCHMARKS.md](BENCHMARKS.md)** - Performance benchmarks and analysis
- **[PERFORMANCE.md](PERFORMANCE.md)** - Performance optimization guide
- **[SOFTWARE_HSM_TEST_REPORT.md](SOFTWARE_HSM_TEST_REPORT.md)** - Software HSM testing results

### Project Documentation
- **[Main README](../../README.md)** - Project overview and quick start
- **[CLI Reference](../../CLI.md)** - Complete command-line documentation
- **[Examples](../../EXAMPLES.md)** - Real-world usage examples
- **[Universal Backend Guide](../../UNIVERSAL_BACKEND.md)** - Backend system architecture

[↑ Back to top](#trustedge-core)

---

## License

This project is licensed under the Mozilla Public License 2.0 (MPL-2.0).

**Commercial Licensing**: Enterprise licenses available for commercial use without source disclosure requirements. Contact [enterprise@trustedgelabs.com](mailto:enterprise@trustedgelabs.com).

[↑ Back to top](#trustedge-core)

---

## Security

For security issues, please follow our [responsible disclosure policy](../SECURITY.md).

**Security Contact**: [security@trustedgelabs.com](mailto:security@trustedgelabs.com)

[↑ Back to top](#trustedge-core)

---

*TrustEdge Core - The foundation of privacy-preserving edge computing.*