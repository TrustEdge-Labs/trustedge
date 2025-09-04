<!--
Copyright (c) 2025 John Turner
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->

# TrustEdge — Trustable Edge AI

---

## Why This Project?

**Trustable Edge AI** — privacy-preserving edge pipelines with **secure** and data-agnostic encryption.

TrustEdge features:

- **Data-agnostic encryption:** Works with files, live audio, sensor data, or any binary stream
- **Live audio capture:** Real-time microphone input with configurable quality and device selection
- **Provenance by design:** each chunk carries a signed manifest (C2PA-inspired) whose hash is bound into AEAD AAD; tampering breaks decryption
- **Privacy by design & default**: encrypt at the edge, not just TLS in transit, audio chunks are encrypted with AES-256-GCM before leaving the device
- **Rust at the edge**: safety + performance for streaming workloads  
- **Streaming-friendly:** fixed nonce discipline (prefix||counter) and per-chunk records
- **Universal Backend System**: Capability-based crypto operations supporting keyring, TPM, HSM, and YubiKey backends
- **Learning in public**: small, honest milestones → real, reviewable code

**Technology Stack:**
- Language: Rust (stable)
- Crypto: `aes-gcm` (AEAD), 256-bit keys, 96-bit nonces
- Audio: `cpal` library with cross-platform support (Linux/ALSA, Windows/WASAPI, macOS/CoreAudio)
- Key Management: Universal Backend system with pluggable crypto operations
- Authentication: Ed25519-based mutual authentication with session management

## Testing & Quality Assurance

TrustEdge includes a comprehensive test suite with **32 automated tests** covering all aspects of the system:

- **Unit Tests (7)**: Core functionality validation
- **Authentication Tests (3)**: Certificate generation, mutual authentication, session management
- **Roundtrip Tests (15)**: End-to-end encryption/decryption validation including comprehensive MIME type detection
- **Network Integration Tests (7)**: Distributed encryption workflows with real network communication

---

## Quick Start

### Installation

**Basic Installation (no audio):**
```bash
# Install Rust (if needed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and build (file encryption only)
git clone https://github.com/TrustEdge-Labs/trustedge.git
cd trustedge/trustedge-audio
cargo build --release --no-default-features
```

**Full Installation (with live audio capture):**
```bash
# Install audio system dependencies
# On Ubuntu/Debian:
sudo apt-get install libasound2-dev pkg-config

# On macOS (via Homebrew):
# Audio libraries included with Xcode/Command Line Tools

# On Windows:
# Audio libraries included with Windows SDK

# Build with audio features
cargo build --release --features audio
```

### Basic Usage

**Live Audio Capture (NEW!):**
```bash
# Capture 10 seconds of live audio and encrypt it
./target/release/trustedge-audio \
  --live-capture \
  --envelope voice_note.trst \
  --key-out voice_key.hex \
  --max-duration 10

# List available audio devices
./target/release/trustedge-audio --list-audio-devices

# Capture with specific device and quality
./target/release/trustedge-audio \
  --live-capture \
  --audio-device "hw:CARD=USB_AUDIO,DEV=0" \
  --sample-rate 48000 \
  --channels 2 \
  --envelope stereo_voice.trst \
  --use-keyring \
  --max-duration 30

# Decrypt captured audio (produces raw PCM audio data)
./target/release/trustedge-audio \
  --decrypt \
  --input voice_note.trst \
  --out recovered_audio.raw \
  --key-hex $(cat voice_key.hex)

# Convert raw PCM to playable WAV file (requires ffmpeg)
ffmpeg -f f32le -ar 44100 -ac 1 -i recovered_audio.raw recovered_audio.wav
```

**📋 Format-Aware Decryption:** 
- **File inputs**: Decryption preserves original file format with MIME type detection (PDF→PDF, JSON→JSON, etc.)
- **Live audio inputs**: Decryption outputs **raw PCM data** (requires conversion for playback)
- **Inspection**: Use `--inspect` to view data type and format without decryption

```bash
# Inspect encrypted data format without decrypting
./target/release/trustedge-audio --input data.trst --inspect --verbose

# Example output:
# TrustEdge Archive Information:
#   File: data.trst
#   Data Type: File
#   MIME Type: application/json
#   Output Behavior: Original file format preserved
```

**Simple File Encryption:**
```bash
# Encrypt file with random key
./target/release/trustedge-audio 
  --input document.txt 
  --envelope document.trst 
  --key-out mykey.hex

# Decrypt file with format-aware output
./target/release/trustedge-audio 
  --decrypt 
  --input document.trst 
  --out recovered.txt 
  --key-hex $(cat mykey.hex)
  --verbose

# Example verbose output:
# ● Input Type: File
#   MIME Type: text/plain
# ✔ Output: Original file format preserved
# ✔ Decrypt complete. Wrote 1337 bytes.

# Verify integrity
diff document.txt recovered.txt  # Should be identical
```

**Keyring-Based Encryption:**
```bash
# One-time setup and basic keyring usage
./target/release/trustedge-audio --set-passphrase "my secure passphrase"
./target/release/trustedge-audio --input file.txt --envelope file.trst --use-keyring --salt-hex $(openssl rand -hex 16)
```

**🔧 For complete CLI options and key management backends, see [CLI.md](CLI.md#key-management-options).**

**💡 For detailed workflows and backend configuration, see [EXAMPLES.md](EXAMPLES.md#key-management-scenarios).**

### Network Mode

TrustEdge supports secure client-server communication with mutual authentication:

```bash
# Quick start: Authenticated server
./target/release/trustedge-server --listen 127.0.0.1:8080 --require-auth --decrypt

# Quick start: Authenticated client  
./target/release/trustedge-client --server 127.0.0.1:8080 --input file.txt --require-auth
```

**📖 For complete authentication setup, security considerations, and production deployment, see [AUTHENTICATION_GUIDE.md](AUTHENTICATION_GUIDE.md).**

**� For detailed CLI options and network configuration, see [CLI.md](CLI.md#network-options).**

**💡 For comprehensive examples and use cases, see [EXAMPLES.md](EXAMPLES.md#network-mode-examples).**

---

## Universal Backend System

TrustEdge features a **capability-based Universal Backend system** that provides pluggable cryptographic operations across different hardware and software backends:

### Backend Architecture

```mermaid
graph TD
    A[TrustEdge Client] --> B[Universal Backend Registry]
    B --> C[Backend Discovery]
    C --> D{Available Backends}
    
    D --> E[Keyring Backend]
    D --> F[YubiKey Backend]
    D --> G[TPM Backend]
    D --> H[HSM Backend]
    
    E --> I[OS Keyring]
    F --> J[YubiKey PIV]
    G --> K[TPM 2.0]
    H --> L[PKCS#11]
    
    I --> M[Crypto Operations]
    J --> M
    K --> M
    L --> M
```

### Key Features

- **✅ Capability Discovery**: Automatic detection of backend capabilities
- **✅ Operation Dispatch**: Enum-based operation routing with type safety
- **✅ Preference-Based Selection**: Configurable backend preferences and fallbacks
- **✅ Runtime Backend Detection**: Dynamic discovery of available backends
- **✅ Pluggable Architecture**: Easy addition of new backends (YubiKey, TPM, HSM)

### Supported Operations

```rust
// Core cryptographic operations across all backends
pub enum CryptoOperation {
    DeriveKey { domain: String, purpose: String },
    ComputeHash { algorithm: String, data: Vec<u8> },
    GenerateNonce { size: usize },
    // Future: Sign, Verify, Encrypt, Decrypt (hardware backends)
}
```

### Backend Status

| Backend | Status | Operations | Notes |
|---------|--------|------------|-------|
| **Keyring** | ✅ Available | Key derivation, hashing, nonce generation | OS keyring integration |
| **YubiKey** | 🔄 Planned | PIV operations, hardware signing | Requires YubiKey PIV |
| **TPM** | 🔄 Planned | TPM 2.0 operations, attestation | Linux/Windows TPM support |
| **HSM** | 🔄 Planned | PKCS#11 operations | Enterprise HSM integration |

### Demo and Examples

```bash
# Run the Universal Backend demonstration
cargo run --example universal_backend_demo

# Expected output:
# ✔ Available backends: keyring
# ✔ Backend capabilities: DeriveKey, ComputeHash, GenerateNonce
# ✔ Key derivation: Successfully derived key for audio.encryption
# ✔ Hash computation: Successfully computed SHA-256 hash
# ✔ Nonce generation: Successfully generated 12-byte nonce
```

**📖 For detailed Universal Backend documentation and implementation guides, see [UNIVERSAL_BACKEND.md](UNIVERSAL_BACKEND.md).**

---

## How It Works

TrustEdge uses a **data-agnostic architecture** with **format-aware decryption** that treats all input sources uniformly while preserving format information:

### Format-Aware Processing Flow

```mermaid
graph TD
    A[Input Source] --> B{Source Type}
    B -->|File| C[File Input]
    B -->|Live Audio| D[Audio Capture]
    
    C --> E[MIME Detection]
    E --> F[DataType::File]
    D --> G[Audio Config]
    G --> H[DataType::Audio]
    
    F --> I[Manifest Creation]
    H --> I
    I --> J[Encryption + Signing]
    J --> K[.trst Archive]
    
    K --> L[Decrypt Request]
    L --> M[Read Manifest]
    M --> N{Data Type?}
    
    N -->|File| O[Format Preserved]
    N -->|Audio| P[Raw PCM Output]
    
    O --> Q[📄 Original Format]
    P --> R[🎵 PCM + Conversion Info]
    
    style A fill:#e1f5fe
    style K fill:#fff3e0
    style Q fill:#e8f5e8
    style R fill:#ffe8e8
```

### Data Sources
- **Files**: Documents, images, videos, any binary data
- **Live Audio**: Real-time microphone capture with configurable quality
- **Future**: Camera feeds, sensor data, IoT device streams

### Processing Pipeline
```
Data Source → Raw Chunks → Metadata + Encryption → .trst Format
                ↓
          (MIME type, format, device info stored in manifest)
                ↓
          Receiver → Decrypt + Format-Aware Output → Consumer Application
```

### Security Architecture

TrustEdge implements defense-in-depth with multiple security layers:

```mermaid
graph TD
    A[Client App] -->|1. Mutual Auth| B[Authentication Layer]
    B -->|2. Session ID| C[Session Management]
    C -->|3. Encrypted Data| D[Transport Layer]
    D -->|4. Chunked Transfer| E[Server]
    
    B --> F[Ed25519 Certificates]
    B --> G[Challenge-Response]
    C --> H[Cryptographic Sessions]
    C --> I[Timeout Management]
    D --> J[AES-256-GCM Encryption]
    D --> K[Signed Manifests]
    
    style A fill:#e1f5fe
    style E fill:#e8f5e8
    style F fill:#fff3e0
    style G fill:#fff3e0
    style H fill:#f3e5f5
    style I fill:#f3e5f5
    style J fill:#ffebee
    style K fill:#ffebee
```

**🔐 For complete security flow details, see [AUTHENTICATION_GUIDE.md](AUTHENTICATION_GUIDE.md#how-trustedge-secure-session-works).**

**Security Properties** (applies to all data types):
1. **Per-Chunk Encryption**: Each chunk encrypted with AES-256-GCM
2. **Signed Manifests**: Ed25519 signatures provide authenticity and provenance
3. **Mutual Authentication**: Ed25519-based client/server authentication with certificate validation
4. **Session Management**: Cryptographically secure session IDs with configurable timeouts
5. **Data Type Metadata**: Format info (audio: sample rate, channels, bit depth) travels securely
6. **Integrity Binding**: Cryptographic binding prevents tampering and replay attacks
7. **Streaming Support**: Chunks can be processed independently for real-time workflows

**Key Features:**
- ✅ Data-agnostic encryption (files, audio, sensors, etc.)
- ✅ Live audio capture with cross-platform support
- ✅ **Mutual authentication system with Ed25519 certificates**
- ✅ **Session validation and automatic session cleanup**
- ✅ **Server identity verification and client authorization**
- ✅ Metadata preservation (audio format, device info, etc.)
- ✅ Chunked encryption for memory efficiency
- ✅ Authenticated encryption (AES-256-GCM)
- ✅ Pluggable key management backends
- ✅ Network streaming support with robust connection handling
- ✅ Connection timeouts and retry logic with exponential backoff
- ✅ Graceful server shutdown with signal handling
- ✅ Comprehensive validation and error handling
- ✅ Test vector validation for format stability
- ✅ Production-ready network resilience features
- ✅ **Comprehensive test suite with 45 tests covering all workflows**
- ✅ **Format-specific validation (PDF, MP3, JSON, binary, text)**
- ✅ **End-to-end network testing with real client-server communication**

---

## Documentation

### User Guides
- **[CLI.md](./CLI.md)** — Complete command-line reference with examples
- **[EXAMPLES.md](./EXAMPLES.md)** — Real-world usage examples and workflows
- **[AUTHENTICATION_GUIDE.md](./AUTHENTICATION_GUIDE.md)** — Complete authentication setup and security guide
- **[TROUBLESHOOTING.md](./TROUBLESHOOTING.md)** — Error handling, diagnostics, and common issues
- **[TESTING.md](./TESTING.md)** — Testing procedures and validation

### Technical Documentation  
- **[PROTOCOL.md](./PROTOCOL.md)** — Network protocol and wire format specification
- **[FORMAT.md](./FORMAT.md)** — Binary format specification and validation rules
- **[DEVELOPMENT.md](./DEVELOPMENT.md)** — Development guide, roadmap, and contribution guidelines

### Security & Policy
- **[THREAT_MODEL.md](./THREAT_MODEL.md)** — Security analysis and threat mitigation
- **[SECURITY.md](./SECURITY.md)** — Security policy and vulnerability reporting

---

## Testing & Quality Assurance

TrustEdge features comprehensive testing with **45 tests** covering all workflows:

### Test Suite Overview
```bash
# Run complete test suite (45 tests)
cargo test

# Test execution summary:
✅ Unit Tests:              20/20  passed (library functionality + Universal Backend)
✅ Auth Integration:         3/3   passed (mutual authentication)  
✅ Roundtrip Integration:   15/15  passed (encryption/decryption workflows)
✅ Network Integration:      7/7   passed (client-server communication)
────────────────────────────────────────────────────────────────────────
✅ Total Tests:             45/45  passed (100% success rate)
✅ Total Execution:         ~10 seconds (efficient testing)
```

### Validation Coverage
- **📄 Format-Specific Testing**: PDF, MP3, JSON, binary, text files with byte-perfect restoration
- **🌐 Network Protocol Testing**: Real client-server communication with authentication
- **🔒 Security Testing**: Mutual authentication, session management, data integrity
- **🔧 Universal Backend Testing**: Capability discovery, operation dispatch, backend registry
- **⚡ Performance Testing**: Large file handling, chunked transfer, memory efficiency
- **🎯 Edge Case Testing**: Empty files, unknown formats, connection errors
- **🔍 CLI Testing**: Real binary execution with proper argument validation

### Quality Assurance Tools
```bash
# Pre-commit validation (prevents CI failures)
./scripts/ci-check.sh

# Individual test suites
cargo test --test network_integration      # Network testing
cargo test --test roundtrip_integration    # Local workflows  
cargo test --test auth_integration         # Authentication
```

**For detailed testing procedures, see [TESTING.md](./TESTING.md).**

---

## Project Status

## Project Status

**✅ Phase 1: Foundation (COMPLETED)**
- Core encryption/decryption with AES-256-GCM
- Binary format specification and validation
- Test vector system with golden hash verification

**✅ Phase 2: Key Management (COMPLETED)**  
- Pluggable backend architecture
- Keyring integration with PBKDF2
- **Universal Backend system with capability-based operations**
- **YubiKey-ready architecture with enum-based dispatch**
- Professional code quality standards

**✅ Phase 3: Network Operations (COMPLETED)**
- Basic client-server architecture ✅
- Connection timeouts and retry logic ✅ 
- Graceful server shutdown ✅
- Enhanced connection management ✅
- **Server authentication** (Day 10) ✅
- **Client authentication** (Day 11) ✅
- **Enhanced security features** (Day 12) ✅
- **Ed25519 mutual authentication system** ✅

**� Phase 4: Live Audio Capture (IN PROGRESS)**
- Real-time audio chunking and streaming pipeline
- Cross-platform audio capture integration  
- Live audio processing features
- Enhanced streaming protocols

**📋 Phase 5: Hardware Security & Advanced Backends (PLANNED)**
- YubiKey PIV backend implementation (Universal Backend ready)
- TPM 2.0 backend implementation (Universal Backend ready)
- HSM/PKCS#11 backend implementation (Universal Backend ready)
- Comprehensive testing, fuzzing & audit infrastructure

**📋 Phase 6: Community & Deployment (PLANNED)**
- Community engagement and beta testing program
- Example configuration files for deployment scenarios
- Documentation improvements and tutorials

See **[GitHub Issues](https://github.com/TrustEdge-Labs/trustedge/issues)** for detailed tasks and **[Issue #16](https://github.com/TrustEdge-Labs/trustedge/issues/16)** for progress tracking.

### 📊 Project Tracking
- **GitHub Project Board**: [TrustEdge Development](https://github.com/TrustEdge-Labs/projects/2)
- **Current Focus**: [Phase 4: Live Audio Capture](https://github.com/TrustEdge-Labs/trustedge/issues?q=label%3APhase-3-LiveAudio+is%3Aopen)
- **Progress Tracker**: [Issue #16](https://github.com/TrustEdge-Labs/trustedge/issues/16)
- **All Issues**: [View on GitHub](https://github.com/TrustEdge-Labs/trustedge/issues)

---

## Security

**Current Security Properties:**
- AES-256-GCM authenticated encryption
- Ed25519 digital signatures for provenance and authentication
- **Mutual authentication between clients and servers**
- **Cryptographically secure session management**
- PBKDF2 key derivation (100,000 iterations)
- Comprehensive validation prevents tampering

**Security Limitations:**
- Demo/development keys (not production-ready)
- No key rotation or revocation yet
- Limited to software-based key storage

For detailed security analysis, see **[THREAT_MODEL.md](./THREAT_MODEL.md)**.

---

## TrustEdge Ecosystem Overview

```mermaid
graph TB
    subgraph "Input Sources"
        A1[📄 Files<br/>JSON, PDF, Images]
        A2[🎵 Live Audio<br/>Microphone Capture]
        A3[🔮 Future<br/>Camera, Sensors]
    end
    
    subgraph "Format Detection"
        B1[MIME Detection<br/>30+ File Types]
        B2[Audio Metadata<br/>Sample Rate, Channels]
    end
    
    subgraph "Encryption Pipeline"
        C1[📝 Manifest Creation<br/>DataType + Metadata]
        C2[🔒 AES-256-GCM<br/>Authenticated Encryption]
        C3[✍️ Ed25519 Signatures<br/>Provenance + Auth]
    end
    
    subgraph "Storage & Transport"
        D1[💾 .trst Archives<br/>Encrypted + Metadata]
        D2[🌐 Network Transfer<br/>Mutual Authentication]
        D3[🔍 Inspection Tools<br/>Format Without Decrypt]
    end
    
    subgraph "Format-Aware Decryption"
        E1[📋 Read Manifest<br/>Detect Original Type]
        E2{Data Type?}
        E3[📄 File Output<br/>Format Preserved]
        E4[🎵 PCM Output<br/>+ Conversion Guide]
    end
    
    subgraph "Key Management"
        F1[🔐 OS Keyring<br/>PBKDF2 + Salt]
        F2[🔑 Hardware Keys<br/>TPM, HSM Future]
        F3[🎟️ Session Keys<br/>Mutual Auth]
    end
    
    A1 --> B1
    A2 --> B2
    A3 --> B1
    
    B1 --> C1
    B2 --> C1
    
    C1 --> C2
    C2 --> C3
    C3 --> D1
    
    D1 --> D2
    D1 --> D3
    
    D2 --> E1
    D3 --> E1
    E1 --> E2
    E2 -->|File| E3
    E2 -->|Audio| E4
    
    F1 --> C2
    F2 --> C2
    F3 --> D2
    
    style A1 fill:#e1f5fe
    style A2 fill:#e8f5e8
    style C2 fill:#fff3e0
    style E3 fill:#e8f5e8
    style E4 fill:#ffe8e8
    style F1 fill:#f3e5f5
```

**Key Features:**
- 🔒 **End-to-End Security**: AES-256-GCM + Ed25519 signatures
- 📋 **Format Awareness**: MIME detection with intelligent output handling
- 🎵 **Audio-First Design**: Live capture with metadata preservation
- 🔍 **Inspection Tools**: View format without decryption
- 🌐 **Network Security**: Mutual authentication and session management
- 🔐 **Flexible Key Management**: OS keyring with future hardware support

---

**Vulnerability Reporting:** See **[SECURITY.md](./SECURITY.md)** for responsible disclosure process.

---

## Contributing

We welcome contributions! Please see our comprehensive guidelines and project management resources:

### 📋 **Contribution Guidelines**
- **[CONTRIBUTING.md](./CONTRIBUTING.md)** — Complete contribution guide and standards
- **[CODING_STANDARDS.md](./CODING_STANDARDS.md)** — Detailed coding conventions and style guide
- **[DEVELOPMENT.md](./DEVELOPMENT.md)** — Development setup and technical guidelines

### 🎯 **Project Management**
- **[Project Board](https://github.com/TrustEdge-Labs/projects/2)** — Visual progress tracking and task organization
- **[GitHub Issues](https://github.com/TrustEdge-Labs/trustedge/issues)** — Bug reports, feature requests, and tasks
- **[Progress Tracker](https://github.com/TrustEdge-Labs/trustedge/issues/16)** — Current development status

**Note**: GitHub project boards require manual addition of issues. Use `./scripts/project/manage-board.sh` to add issues to the project board.

### 📝 **Issue Templates**
- 🐛 **[Bug Reports](./.github/ISSUE_TEMPLATE/bug-report.yml)** — Report issues with detailed information
- ✨ **[Feature Requests](./.github/ISSUE_TEMPLATE/feature-request.yml)** — Suggest new features and improvements
- 📚 **[Documentation Issues](./.github/ISSUE_TEMPLATE/documentation.yml)** — Help improve documentation
- 🔒 **[Security Issues](./.github/ISSUE_TEMPLATE/security.yml)** — Report security concerns and improvements

### 🚀 **Getting Started**
1. **Check existing work**: Browse [open issues](https://github.com/TrustEdge-Labs/trustedge/issues) and [project board](https://github.com/TrustEdge-Labs/projects/2)
2. **Read the guides**: Review [CONTRIBUTING.md](./CONTRIBUTING.md) and [DEVELOPMENT.md](./DEVELOPMENT.md)
3. **Pick an issue**: Start with issues labeled `good-first-issue` or current [Phase 4 tasks](https://github.com/TrustEdge-Labs/trustedge/issues?q=label%3APhase-3-LiveAudio+is%3Aopen)
4. **Follow standards**: Use our [PR template](./.github/pull_request_template.md) and code quality requirements

**Before Contributing:**
- ✅ Read the contribution guidelines
- ✅ Check for existing related issues or PRs
- ✅ Follow our code style and testing requirements
- ✅ Use the appropriate issue/PR templates

---

## License

This project is licensed under the **Mozilla Public License 2.0 (MPL-2.0)**.
See **[LICENSE](./LICENSE)** for details.

**Disclaimer:** This project is developed independently, on personal time and equipment, and is **not affiliated with or endorsed by my employer**.

---

## Legal & Attribution

**Copyright** © 2025 John Turner. All rights reserved.

**License**: This documentation is licensed under the [Mozilla Public License 2.0 (MPL-2.0)](https://mozilla.org/MPL/2.0/).

**Project**: [TrustEdge](https://github.com/TrustEdge-Labs/trustedge) — Privacy and trust at the edge.

**Third-party Dependencies**: See **[Cargo.toml](./trustedge-audio/Cargo.toml)** for complete dependency information and licenses.
