<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->

# TrustEdge — Trustable Edge AI

[![CI](https://github.com/TrustEdge-Labs/trustedge/workflows/CI/badge.svg)](https://github.com/TrustEdge-Labs/trustedge/actions)
[![License: MPL 2.0](https://img.shields.io/badge/License-MPL_2.0-brightgreen.svg)](https://opensource.org/licenses/MPL-2.0)
[![Commercial License](https://img.shields.io/badge/Commercial-License%20Available-blue.svg)](mailto:enterprise@trustedgelabs.com)
[![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)](https://www.rust-lang.org)
[![Version](https://img.shields.io/badge/version-0.2.0-blue.svg)](https://github.com/TrustEdge-Labs/trustedge/releases/tag/v0.2.0)
[![YubiKey](https://img.shields.io/badge/YubiKey-Hardware%20Supported-green.svg)](https://www.yubico.com/)

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

## What's New in 0.2.0

- 🔐 **Production-Ready Cryptography** - Real AES-256-GCM encryption with PBKDF2 key derivation
- 🧾 **Digital Receipt System** - Cryptographically secure transferable receipts
- 🔑 **YubiKey Hardware Integration** - Real PKCS#11 support with hardware signing
- 🏗️ **Universal Backend Architecture** - Pluggable crypto backends
- 🌐 **Production Transport Layer** - Real TCP operations with concurrent connections
- 🧪 **Comprehensive Test Suite** - 109 tests including security attack scenarios

---

## Project Architecture

TrustEdge is organized as a Cargo workspace with specialized crates:

```
trustedge/
├── trustedge-core/               # Core cryptographic library and CLI tools
├── trustedge-attestation/        # Software attestation and provenance system
├── trustedge-receipts/           # Digital receipt system with ownership chains
├── trustedge-wasm/               # WebAssembly bindings for browser integration
├── trustedge-pubky/              # Pubky network adapter for decentralized keys
├── trustedge-pubky-advanced/     # Advanced Pubky integration with hybrid encryption
└── docs/                         # Documentation and guides
```

### Crate Overview

| Crate | Purpose | Documentation |
|-------|---------|---------------|
| **trustedge-core** | Core cryptographic library with envelope encryption | [Core Documentation](trustedge-core/) |
| **trustedge-attestation** | Software attestation and provenance tracking with cryptographic "birth certificates" | [Attestation Documentation](trustedge-attestation/) |
| **trustedge-receipts** | Digital receipt system with cryptographic ownership transfer | [Receipt Documentation](trustedge-receipts/) |
| **trustedge-wasm** | WebAssembly bindings for browser/Node.js integration | [WASM Documentation](trustedge-wasm/) |
| **trustedge-pubky** | Clean Pubky network adapter for decentralized key discovery | [Pubky Documentation](trustedge-pubky/) |
| **trustedge-pubky-advanced** | Advanced Pubky integration with hybrid encryption | [Advanced Pubky Documentation](trustedge-pubky-advanced/) |

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

**📖 For detailed installation instructions including system dependencies, see [EXAMPLES.md](EXAMPLES.md#installation-guide).**

### Basic Usage

```bash
# Encrypt a file
./target/release/trustedge-core --input document.txt --envelope document.trst --key-out mykey.hex

# Decrypt a file
./target/release/trustedge-core --decrypt --input document.trst --out recovered.txt --key-hex $(cat mykey.hex)

# Network mode (server)
./target/release/trustedge-server --listen 127.0.0.1:8080 --require-auth

# Network mode (client)
./target/release/trustedge-client --server 127.0.0.1:8080 --input file.txt --require-auth
```

### Next Steps

- **📖 [CLI Reference](CLI.md)** - Complete command-line options and usage
- **💡 [Examples](EXAMPLES.md)** - Real-world workflows and use cases  
- **🔐 [Authentication Guide](AUTHENTICATION_GUIDE.md)** - Secure network setup
- **🏗️ [Architecture Guide](UNIVERSAL_BACKEND.md)** - System design and backends

---

## Core Systems

### Universal Backend System

TrustEdge features a **capability-based Universal Backend system** that provides pluggable cryptographic operations across different hardware and software backends.

**Supported Backends:**
- **Keyring Backend**: OS keyring integration for key derivation and storage
- **YubiKey Backend**: Hardware PIV operations with PKCS#11 support
- **Software HSM**: In-memory cryptographic operations for development
- **TPM Backend**: TPM 2.0 operations and attestation (planned)

**📖 For detailed backend documentation, see [UNIVERSAL_BACKEND.md](UNIVERSAL_BACKEND.md).**

### Digital Receipt System

TrustEdge includes a **production-ready digital receipt system** that enables cryptographically secure ownership transfer of digital assets with comprehensive security testing.

**Key Properties:**
- Cryptographic ownership chains with hash binding
- Ed25519 signatures for authenticity and non-repudiation
- Attack resistance against tampering, replay, and forgery
- 23 comprehensive security tests covering all attack scenarios

**📖 For complete receipt system documentation, see [trustedge-receipts/](trustedge-receipts/).**

### Network Operations

TrustEdge supports secure client-server communication with **mutual authentication** using Ed25519 digital signatures and cryptographically secure session management.

**Security Features:**
- Mutual authentication between clients and servers
- Session isolation with time-limited cryptographic sessions
- Replay protection through challenge-response protocols
- Forward security with automatic session expiration

**📖 For authentication setup and network security, see [AUTHENTICATION_GUIDE.md](AUTHENTICATION_GUIDE.md).**

---

## Testing & Quality Assurance

TrustEdge includes a comprehensive test suite with **109 automated tests** covering all aspects of the system:

- **86 Core Tests**: Envelope encryption, Universal Backend system, transport layer
- **23 Receipt Tests**: Digital receipt security, attack resistance, chain validation
- **Security Tests**: Cryptographic isolation, tampering resistance, replay protection
- **Hardware Tests**: YubiKey integration, PKCS#11 operations, certificate workflows

```bash
# Run complete test suite
./ci-check.sh

# Run tests by category
cargo test -p trustedge-core --lib        # Core cryptography tests
cargo test -p trustedge-receipts          # Receipt system tests
cargo test --features yubikey             # Hardware integration tests
```

**📖 For detailed testing procedures, see [TESTING.md](TESTING.md).**

---

## Documentation

### User Guides
- **[CLI Reference](CLI.md)** - Complete command-line interface documentation
- **[Examples](EXAMPLES.md)** - Real-world usage examples and workflows
- **[Authentication Guide](AUTHENTICATION_GUIDE.md)** - Network security setup
- **[Troubleshooting](TROUBLESHOOTING.md)** - Common issues and solutions

### Technical Reference
- **[Universal Backend](UNIVERSAL_BACKEND.md)** - Backend system architecture
- **[Binary Format](FORMAT.md)** - File format specification
- **[Network Protocol](PROTOCOL.md)** - Communication protocol details
- **[Security Model](SECURITY.md)** - Security architecture and threat model

### Development
- **[Contributing](CONTRIBUTING.md)** - How to contribute to TrustEdge
- **[Development Guide](DEVELOPMENT.md)** - Development setup and workflows
- **[Testing Guide](TESTING.md)** - Test procedures and validation
- **[Coding Standards](CODING_STANDARDS.md)** - Code style and conventions

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

**📖 For detailed security analysis, see [SECURITY.md](SECURITY.md) and [THREAT_MODEL.md](THREAT_MODEL.md).**

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
./ci-check.sh

# Run specific component tests
cargo test -p trustedge-core
cargo test -p trustedge-receipts
```

---

## License

This project is licensed under the Mozilla Public License 2.0 (MPL-2.0). See [LICENSE](LICENSE) for details.

### Legal & Attribution

- **Copyright**: © 2025 TrustEdge Labs LLC
- **License**: Mozilla Public License 2.0
- **Commercial Licensing**: Available for enterprise use
- **Contributor Agreement**: [Developer Certificate of Origin](DCO.md)

---

*TrustEdge — Privacy and trust at the edge.*