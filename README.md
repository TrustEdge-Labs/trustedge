<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->

[![CI Status](https://github.com/TrustEdge-Labs/trustedge/workflows/CI/badge.svg)](https://github.com/TrustEdge-Labs/trustedge/actions)
[![License: MPL 2.0](https://img.shields.io/badge/License-MPL_2.0-brightgreen.svg)](https://opensource.org/licenses/MPL-2.0)
[![Commercial License](https://img.shields.io/badge/Commercial-License%20Available-blue.svg)](mailto:enterprise@trustedgelabs.com)
[![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)](https://www.rust-lang.org)
[![Version](https://img.shields.io/badge/version-1.0-blue.svg)](https://github.com/TrustEdge-Labs/trustedge/releases/tag/v1.0)
[![YubiKey](https://img.shields.io/badge/YubiKey-Hardware%20Supported-green.svg)](https://www.yubico.com/)

# TrustEdge: Hardware-Backed Security for IoT Devices

Open-source cryptographic engine with YubiKey/TPM integration for edge devices and IoT.

**[▶️ Watch YubiKey Hardware Signing Demo (2 min)](https://asciinema.org/a/aMaUEmOfw42TNYdXwAgtefcsy)**

## Why TrustEdge?

Most IoT devices use software-only encryption with keys in memory. TrustEdge provides hardware-backed security via industry-standard PKCS#11.

✅ **Hardware-backed signing** (YubiKey, TPM, HSM)  
✅ **PKCS#11 standard** interface  
✅ **X.509 certificates** with hardware signing  
✅ **Cross-platform** (Linux, macOS, Windows, ESP32, WASM)  
✅ **Open source** and fully auditable  

## Golden Path: YubiKey Hardware Signing Demo

TrustEdge's flagship demo: real cryptographic operations backed by YubiKey hardware.
This showcases hardware-backed signing, key extraction from PIV slots, X.509 certificate
generation, and certificate validation — all using your physical security key.

**Prerequisites:** YubiKey with PIV applet, [OpenSC](https://github.com/OpenSC/OpenSC) PKCS#11 module, [ykman](https://developers.yubico.com/yubikey-manager/) CLI

### Step 1: Generate a key on YubiKey

```bash
ykman piv keys generate 9a /tmp/pubkey.pem --algorithm ECCP256
ykman piv certificates generate 9a /tmp/pubkey.pem --subject "CN=Test"
```

### Step 2: Run the hardware signing demo

```bash
git clone https://github.com/trustedge-labs/trustedge.git
cd trustedge
cargo run --example yubikey_demo --features yubikey -- 123456
```

**What happens:** TrustEdge connects to your YubiKey via PKCS#11, extracts the public key
from PIV slot 9a, performs a hardware-backed ECDSA-P256 signature, generates a complete
X.509 certificate signed by the YubiKey, and exports it in standard DER format.

**No YubiKey?** See the [Software-Only Archive Demo](#software-only-archive-demo) below.


## Commercial Support

📧 **Pilot program:** pilot@trustedgelabs.com

Building IoT devices that need hardware-backed security? We offer:
- Commercial SDK with priority support
- Custom hardware integration (TPM, HSM, secure elements)
- Fleet management and key rotation
- Compliance consulting

## License

Open Core Model:
- Core engine: Mozilla Public License 2.0
- Commercial features: Proprietary license available

🌐 [trustedgelabs.com](https://trustedgelabs.com)

---

## Overview

**TrustEdge** is a privacy-preserving edge computing platform that provides **trustable edge AI** with secure, data-agnostic encryption. Built in Rust for safety and performance, TrustEdge enables secure processing of sensitive data at the edge while maintaining cryptographic guarantees.

### Key Features

- **🔐 Data-Agnostic Encryption**: Works with files, live audio, sensor data, or any binary stream
- **🧾 Digital Receipt System**: Cryptographically secure transferable receipts with ownership chains
- **🏗️ Universal Backend System**: Pluggable crypto operations (Software HSM, Keyring, YubiKey)
- **🎵 Live Audio Capture**: Real-time microphone input with configurable quality
- **🌐 Network Operations**: Secure client-server communication with mutual authentication
- **🔑 Hardware Integration**: Full YubiKey PKCS#11 support with real hardware signing
- **⚡ Algorithm Agility**: Configurable cryptographic algorithms with forward compatibility
- **🛡️ Memory Safety**: Proper key material cleanup with zeroization

### Technology Stack

- **Language**: Rust (stable) for memory safety and performance
- **Cryptography**: AES-256-GCM, Ed25519, PBKDF2, BLAKE3 with algorithm agility
- **Audio**: Cross-platform support (Linux/ALSA, Windows/WASAPI, macOS/CoreAudio)
- **Hardware**: YubiKey PIV operations, TPM support (planned)
- **Network**: Ed25519-based mutual authentication with session management

---

## What's New in v1.0

**v1.0 Consolidation Milestone** — workspace-wide quality and architecture improvements:

- 🏗️ **Crate Consolidation** - Receipts and attestation functionality merged into `trustedge-core`; standalone crates now deprecated facades with 6-month migration window
- 📦 **Dependency Cleanup** - 21 unused dependencies removed, ~2,500 LOC duplication eliminated
- 🔐 **Security Fix** - Removed unmaintained `wee_alloc` dependency
- 🧪 **Test Suite Growth** - 340+ tests across 10 crates (up from 150+), including 160 core tests
- ⚡ **Build Performance** - 45s clean release build with optimized dependency graph
- 🌐 **WASM Compatibility** - Verified for `trustedge-trst-protocols` and browser verification crates
- 📋 **Zero API Breaking Changes** - 196 semver checks per crate, all passing

### Previous Releases

**v0.3.1:** CLI extraction, manifest consolidation, YubiKey improvements

**v0.3.0 (P0 Release):** .trst archive system with cam.video profile, Ed25519 signatures, BLAKE3 chains, browser verification

**v0.2.0:** YubiKey PKCS#11 integration, Universal Backend architecture, production transport layer

---

## Project Architecture

TrustEdge is organized as a Cargo workspace with specialized crates:

```
trustedge/
├── crates/
│   ├── core/                     # Core cryptographic library (trustedge-core)
│   ├── trustedge-cli/            # Main envelope encryption CLI (binary: trustedge)
│   ├── trst-cli/                 # .trst archive CLI tool (trustedge-trst-cli, binary: trst)
│   ├── trst-protocols/            # Canonical cam.video manifest types (WASM-compatible)
│   ├── trst-wasm/                # .trst verification WebAssembly bindings
│   ├── attestation/              # Software attestation and provenance system
│   ├── receipts/                 # Digital receipt system with ownership chains
│   ├── wasm/                     # Core WebAssembly bindings
│   ├── pubky/                    # Pubky network adapter (community/experimental)
│   └── pubky-advanced/           # Advanced Pubky integration (community/experimental)
├── examples/                     # Example implementations and demos
└── docs/                         # Documentation and guides
```

### Crate Overview

| Crate | Purpose | Documentation |
|-------|---------|---------------|
| **trustedge-core** | Core cryptographic library with envelope encryption | [Core Documentation](crates/core/) |
| **trustedge-cli** | Main CLI for envelope encryption (binary: `trustedge`) | [CLI Documentation](crates/trustedge-cli/) |
| **trustedge-trst-cli** | CLI for .trst archive creation and verification (binary: `trst`) | [Archive CLI Documentation](crates/trst-cli/) |
| **trustedge-trst-protocols** | Canonical cam.video manifest types (WASM-compatible) | [Archive Format Documentation](crates/trst-protocols/) |
| **trustedge-attestation** | Software attestation and provenance tracking | [Attestation Documentation](crates/attestation/) |
| **trustedge-receipts** | Digital receipt system with cryptographic ownership transfer | [Receipt Documentation](crates/receipts/) |
| **trustedge-wasm** | WebAssembly bindings for browser/Node.js integration | [WASM Documentation](crates/wasm/) |
| **trustedge-trst-wasm** | .trst archive verification in the browser | [Archive WASM Documentation](crates/trst-wasm/) |
| **trustedge-pubky** | Pubky network adapter (community/experimental) | [Pubky Documentation](crates/pubky/) |
| **trustedge-pubky-advanced** | Hybrid encryption for Pubky (community/experimental) | [Advanced Pubky Documentation](crates/pubky-advanced/) |

---

## Quick Start

### Installation

```bash
# Install Rust (if needed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and build
git clone https://github.com/TrustEdge-Labs/trustedge.git
cd trustedge
cargo build --workspace --release
```

**Optional Features:**
- **Audio Support**: Add `--features audio` for live audio capture
- **YubiKey Support**: Add `--features yubikey` for hardware security keys
- **All Features**: Use `--features audio,yubikey` for complete functionality

**📖 Documentation:**
- **[FEATURES.md](FEATURES.md)** - Complete feature flag reference with dependencies and examples
- **[WASM.md](WASM.md)** - WebAssembly build, test, and deployment guide
- **[docs/user/examples/installation.md](docs/user/examples/installation.md)** - Detailed installation with system dependencies

### Basic Usage

**Core Envelope Encryption:**
```bash
# Encrypt a file
./target/release/trustedge --input document.txt --envelope document.trst --key-out mykey.hex

# Decrypt a file
./target/release/trustedge --decrypt --input document.trst --out recovered.txt --key-hex $(cat mykey.hex)

# Network mode (server)
./target/release/trustedge-server --listen 127.0.0.1:8080 --require-auth

# Network mode (client)
./target/release/trustedge-client --server 127.0.0.1:8080 --input file.txt --require-auth
```

**Archive Creation (.trst format):**
```bash
# Create a .trst archive
./target/release/trst wrap --profile cam.video --in sample.bin --out archive.trst

# Verify a .trst archive
./target/release/trst verify archive.trst --device-pub "ed25519:GAUpGXoor5gP..."
```

### Next Steps

- **📖 [CLI Reference](docs/user/cli.md)** - Complete command-line options and usage
- **💡 [Examples](docs/user/examples/README.md)** - Real-world workflows and use cases
- **🔐 [Authentication Guide](docs/user/authentication.md)** - Secure network setup
- **🏗️ [Architecture Guide](docs/technical/universal-backend.md)** - System design and backends

---

## Software-Only Archive Demo

No hardware required. This demo walks through the `.trst` archive format using the `cam.video` profile with software-generated keys:

### Step 1: Wrap - Create a .trst Archive
```bash
# Navigate to cam.video examples
cd examples/cam.video

# Generate sample data and create archive
cargo run --bin record_and_wrap

# Output: clip.trst archive with encrypted segments and signed manifest
```

### Step 2: Verify - Validate Archive Integrity
```bash
# Verify the created archive
cargo run --bin verify_cli clip.trst device.pub

# ✔ Signature verification
# ✔ Continuity chain validation
# ● Archive summary with segment count and duration
```

### Step 3: Acceptance Tests - Full CLI Integration
```bash
# Run comprehensive A1-A6 acceptance test suite
cargo test --test integration_tests

# Tests cover:
# A1: Basic wrap and verify workflow
# A2: Chain continuity validation
# A3: Signature verification
# A4: Malformed archive rejection
# A5: Crypto validation (end-to-end encryption/decryption)
# A6: Cross-platform compatibility
```

### Step 4: WASM Demo - Browser Verification
```bash
# Build and serve the WASM demo
./scripts/build-wasm-demo.sh

# Open http://localhost:8000 in your browser
# Upload clip.trst directory for in-browser verification
```

**📖 For detailed walkthrough and expected outputs, see [examples/cam.video/README.md](examples/cam.video/README.md).**

### P0 Implementation Details

The P0 `.trst` specification includes:

- **Manifest Canonicalization**: Ordered JSON fields with signature exclusion
- **BLAKE3 Continuity Chain**: Genesis seed `blake3("trustedge:genesis")` with segment linking
- **XChaCha20-Poly1305 Encryption**: Per-segment encryption with unique nonces
- **Ed25519 Signatures**: Device key signing with "ed25519:BASE64" format
- **Archive Layout**: `clip-<id>.trst/` directory with manifest, signatures, and chunks

For hardware-backed signing, see the [Golden Path: YubiKey Hardware Signing Demo](#golden-path-yubikey-hardware-signing-demo) above.

---

## Core Systems

### Universal Backend System

TrustEdge features a **capability-based Universal Backend system** that provides pluggable cryptographic operations across different hardware and software backends.

**Supported Backends:**
- **Keyring Backend**: OS keyring integration for key derivation and storage
- **YubiKey Backend**: Hardware PIV operations with PKCS#11 support
- **Software HSM**: In-memory cryptographic operations for development
- **TPM Backend**: TPM 2.0 operations and attestation (planned)

**📖 For detailed backend documentation, see [docs/technical/universal-backend.md](docs/technical/universal-backend.md).**

### Digital Receipt System

TrustEdge includes a **production-ready digital receipt system** that enables cryptographically secure ownership transfer of digital assets with comprehensive security testing.

**Key Properties:**
- Cryptographic ownership chains with hash binding
- Ed25519 signatures for authenticity and non-repudiation
- Attack resistance against tampering, replay, and forgery
- 23 comprehensive security tests covering all attack scenarios

**📖 For complete receipt system documentation, see [crates/receipts/](crates/receipts/).**

### Network Operations

TrustEdge supports secure client-server communication with **mutual authentication** using Ed25519 digital signatures and cryptographically secure session management.

**Security Features:**
- Mutual authentication between clients and servers
- Session isolation with time-limited cryptographic sessions
- Replay protection through challenge-response protocols
- Forward security with automatic session expiration

**📖 For authentication setup and network security, see [docs/user/authentication.md](docs/user/authentication.md).**

---

## Testing & Quality Assurance

TrustEdge includes a comprehensive test suite with **340+ automated tests** covering all aspects of the system:

- **160 Core Tests**: Envelope encryption, Universal Backend system, receipts, attestation, transport layer
- **7 Archive Tests**: .trst format verification, cryptographic validation, attack resistance
- **Security Tests**: Cryptographic isolation, tampering resistance, replay protection
- **Hardware Tests**: YubiKey integration, PKCS#11 operations, certificate workflows

```bash
# Run complete test suite
./scripts/ci-check.sh

# Run tests by category
cargo test -p trustedge-core --lib                # Core cryptography tests (160)
cargo test -p trustedge-trst-cli --test acceptance # Archive validation tests (7)
cargo test --features yubikey                     # Hardware integration tests
```

**📖 For detailed testing procedures, see [docs/developer/testing.md](docs/developer/testing.md).**

---

## Documentation

### User Guides
- **[CLI Reference](docs/user/cli.md)** - Complete command-line interface documentation
- **[Examples](docs/user/examples/README.md)** - Real-world usage examples and workflows
- **[Authentication Guide](docs/user/authentication.md)** - Network security setup
- **[Troubleshooting](docs/user/troubleshooting.md)** - Common issues and solutions

### Technical Reference
- **[Universal Backend](docs/technical/universal-backend.md)** - Backend system architecture
- **[Binary Format](docs/technical/format.md)** - File format specification
- **[Network Protocol](docs/technical/protocol.md)** - Communication protocol details
- **[Security Model](SECURITY.md)** - Security architecture and threat model

### Development
- **[Contributing](CONTRIBUTING.md)** - How to contribute to TrustEdge
- **[Development Guide](docs/developer/development.md)** - Development setup and workflows
- **[Testing Guide](docs/developer/testing.md)** - Test procedures and validation
- **[Coding Standards](docs/developer/coding-standards.md)** - Code style and conventions

**📖 For complete documentation index, see [docs/README.md](docs/README.md).**

---

## Commercial Licensing

TrustEdge is available under MPL-2.0 for open source use. Commercial licenses are available for enterprise customers requiring:

- **Proprietary modifications** without source disclosure requirements
- **Enterprise support** with SLAs and priority response
- **Custom integrations** and professional services
- **Legal indemnification** and warranty protection
- **Advanced enterprise features** (TPM integration, enhanced monitoring, compliance reporting)

**Contact:** [enterprise@trustedgelabs.com](mailto:enterprise@trustedgelabs.com) for commercial licensing inquiries.

---

## Security

For security issues, please follow our [responsible disclosure policy](SECURITY.md).

**Security Contact:** [security@trustedgelabs.com](mailto:security@trustedgelabs.com)

### Security Properties

- **Cryptographic Isolation**: Only intended recipients can decrypt data
- **Forward Secrecy**: Past communications remain secure even if keys are compromised
- **Replay Protection**: Unique cryptographic fingerprints prevent message reuse
- **Memory Safety**: Secure key material handling with automatic cleanup
- **Hardware Security**: Optional YubiKey integration for hardware-backed operations

**📖 For detailed security analysis, see [SECURITY.md](SECURITY.md) and [docs/technical/threat-model.md](docs/technical/threat-model.md).**

---

## Contributing

We welcome contributions to TrustEdge! Please see our [Contributing Guide](CONTRIBUTING.md) for details on:

- Code of conduct and community guidelines
- Development setup and workflow
- Testing requirements and procedures
- Documentation standards
- Security review process

### Quick Start for Contributors

```bash
# Clone and setup development environment
git clone https://github.com/TrustEdge-Labs/trustedge.git
cd trustedge

# Run full test suite
./scripts/ci-check.sh

# Run specific component tests
cargo test -p trustedge-core                      # Core cryptography (160 tests)
cargo test -p trustedge-trst-cli --test acceptance # Archive validation (7 tests)
```

---

## License

This project is licensed under the Mozilla Public License 2.0 (MPL-2.0). See [LICENSE](LICENSE) for details.

### Legal & Attribution

- **Copyright**: © 2025 TrustEdge Labs LLC
- **License**: Mozilla Public License 2.0
- **Commercial Licensing**: Available for enterprise use
- **Contributor Agreement**: [Developer Certificate of Origin](docs/legal/dco.md)

---

*TrustEdge — Privacy and trust at the edge.*
