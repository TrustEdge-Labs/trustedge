<!--
Copyright (c) 2025 John Turner
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/johnzilla/trustedge
-->


# TrustEdge — Trustable Edge AI (Rust)

> Not another CRUD app. Learning Rust through **Trustable Edge AI** — privacy-preserving edge pipelines with **live audio capture** and data-agnostic encryption.

---

## Why This Project?

Most people learning Rust start with CRUD web apps. This project stems from a question: "If I wanted to speak to a LLM, how could I make sure it was private?"

That random thought and an urge to do something out of my comfort zone led to this project, TrustEdge. 

TrustEdge is a learning journey in Rust that aligns with my background in IoT product development, security/PKI and edge systems. TrustEdge features:

- **Data-agnostic encryption:** Works with files, live audio, sensor data, or any binary stream
- **Live audio capture:** Real-time microphone input with configurable quality and device selection
- **Provenance by design:** each chunk carries a signed manifest (C2PA-inspired) whose hash is bound into AEAD AAD; tampering breaks decryption
* **Privacy by design & default**: encrypt at the edge, not just TLS in transit, audio chunks are encrypted with AES-256-GCM before leaving the device
* **Rust at the edge**: safety + performance for streaming workloads  
- **Streaming-friendly:** fixed nonce discipline (prefix||counter) and per-chunk records
* **Learning in public**: small, honest milestones → real, reviewable code

**Technology Stack:**
- Language: Rust (stable)
- Crypto: `aes-gcm` (AEAD), 256-bit keys, 96-bit nonces
- Audio: `cpal` library with cross-platform support (Linux/ALSA, Windows/WASAPI, macOS/CoreAudio)
- Key Management: Pluggable backends (keyring, TPM, HSM planned)

---

## Quick Start

### Installation

**Basic Installation (no audio):**
```bash
# Install Rust (if needed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and build (file encryption only)
git clone https://github.com/johnzilla/trustedge.git
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

# Decrypt captured audio (produces raw audio data)
./target/release/trustedge-audio \
  --decrypt \
  --input voice_note.trst \
  --out recovered_audio.raw \
  --key-hex $(cat voice_key.hex)
```

**Simple File Encryption:**
```bash
# Encrypt file with random key
./target/release/trustedge-audio 
  --input document.txt 
  --envelope document.trst 
  --key-out mykey.hex

# Decrypt file
./target/release/trustedge-audio 
  --decrypt 
  --input document.trst 
  --out recovered.txt 
  --key-hex $(cat mykey.hex)

# Verify integrity
diff document.txt recovered.txt  # Should be identical
```

**Keyring-Based Encryption:**
```bash
# One-time setup: store passphrase
./target/release/trustedge-audio --set-passphrase "my secure passphrase"

# Encrypt with keyring
./target/release/trustedge-audio 
  --input audio.wav 
  --envelope audio.trst 
  --backend keyring 
  --salt-hex $(openssl rand -hex 16)

# Decrypt with keyring
./target/release/trustedge-audio 
  --decrypt 
  --input audio.trst 
  --out recovered.wav 
  --backend keyring 
  --salt-hex <same-salt-as-encryption>
```

### Network Mode

**Secure Server (with Authentication):**
```bash
# Start authenticated server - generates certificates automatically
./target/release/trustedge-server \
  --listen 127.0.0.1:8080 \
  --require-auth \
  --server-identity "Production TrustEdge Server" \
  --decrypt \
  --use-keyring \
  --salt-hex $(openssl rand -hex 16)
```

**Authenticated Client:**
```bash
# Send data with mutual authentication
./target/release/trustedge-client \
  --server 127.0.0.1:8080 \
  --input voice_recording.wav \
  --require-auth \
  --client-identity "Mobile App v1.2.3" \
  --use-keyring \
  --salt-hex <same-salt-as-server>
```

**📖 For complete authentication setup, security considerations, and production deployment, see [AUTHENTICATION_GUIDE.md](AUTHENTICATION_GUIDE.md).**

**Legacy Server (no authentication):**
```bash
# Basic server without authentication
./target/release/trustedge-server \
  --listen 127.0.0.1:8080 \
  --decrypt \
  --output-dir ./received \
  --use-keyring \
  --salt-hex $(openssl rand -hex 16)
```

---

## How It Works

TrustEdge uses a **data-agnostic architecture** that treats all input sources uniformly:

### Data Sources
- **Files**: Documents, images, videos, any binary data
- **Live Audio**: Real-time microphone capture with configurable quality
- **Future**: Camera feeds, sensor data, IoT device streams

### Processing Pipeline
```
Data Source → Raw Chunks → Metadata + Encryption → .trst Format
                ↓
          (Sample rate, format, device info stored in manifest)
                ↓
          Receiver → Decrypt + Metadata → Consumer Application
```

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

---

## Documentation

### User Guides
- **[CLI.md](./CLI.md)** — Complete command-line reference with examples
- **[EXAMPLES.md](./EXAMPLES.md)** — Real-world usage examples and workflows
- **[AUTHENTICATION_GUIDE.md](./AUTHENTICATION_GUIDE.md)** — Complete authentication setup and security guide
- **[TESTING.md](./TESTING.md)** — Testing procedures and validation

### Technical Documentation  
- **[PROTOCOL.md](./PROTOCOL.md)** — Network protocol and wire format specification
- **[FORMAT.md](./FORMAT.md)** — Binary format specification and validation rules
- **[DEVELOPMENT.md](./DEVELOPMENT.md)** — Development guide, roadmap, and contribution guidelines

### Security & Policy
- **[THREAT_MODEL.md](./THREAT_MODEL.md)** — Security analysis and threat mitigation
- **[SECURITY.md](./SECURITY.md)** — Security policy and vulnerability reporting

---

## Project Status

**✅ Phase 1: Foundation (COMPLETED)**
- Core encryption/decryption with AES-256-GCM
- Binary format specification and validation
- Test vector system with golden hash verification

**✅ Phase 2: Key Management (COMPLETED)**  
- Pluggable backend architecture
- Keyring integration with PBKDF2
- Professional code quality standards

**✅ Phase 3: Data Sources (COMPLETED)**
- Live audio capture with cross-platform support ✅
- Data-agnostic architecture with metadata preservation ✅
- Configurable audio quality (sample rate, channels, devices) ✅
- Feature-gated compilation for CI/CD compatibility ✅

**✅ Phase 4: Network Operations (COMPLETED)**
- Basic client-server architecture ✅
- Connection timeouts and retry logic ✅
- Graceful server shutdown ✅
- Enhanced connection management ✅
- **Mutual authentication with Ed25519 certificates** ✅
- **Session management and validation** ✅
- **Production-ready security features** ✅

**📋 Phase 5: Security Hardening (PLANNED)**
- TPM backend implementation
- Hardware security module support  
- Key rotation mechanisms

See **[DEVELOPMENT.md](./DEVELOPMENT.md)** for complete roadmap and **[PHASE3_PROGRESS.md](./PHASE3_PROGRESS.md)** for current development status.

### 📊 Project Tracking
- **GitHub Project Board**: [TrustEdge Development](https://github.com/users/johnzilla/projects/2)
- **Current Milestone**: [Day 10: Server Authentication](https://github.com/johnzilla/trustedge/milestone/2)
- **Progress Tracker**: [Issue #16](https://github.com/johnzilla/trustedge/issues/16)
- **All Milestones**: [View on GitHub](https://github.com/johnzilla/trustedge/milestones)

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

**Vulnerability Reporting:** See **[SECURITY.md](./SECURITY.md)** for responsible disclosure process.

---

## Contributing

We welcome contributions! Please see our comprehensive guidelines and project management resources:

### 📋 **Contribution Guidelines**
- **[CONTRIBUTING.md](./CONTRIBUTING.md)** — Complete contribution guide and standards
- **[DEVELOPMENT.md](./DEVELOPMENT.md)** — Development setup and technical guidelines
- **[PHASE3_PROGRESS.md](./PHASE3_PROGRESS.md)** — Current development status and roadmap

### 🎯 **Project Management**
- **[Project Board](https://github.com/users/johnzilla/projects/2)** — Visual progress tracking and task organization
- **[GitHub Issues](https://github.com/johnzilla/trustedge/issues)** — Bug reports, feature requests, and tasks
- **[Milestones](https://github.com/johnzilla/trustedge/milestones)** — Development phases and deadlines

**Note**: GitHub project boards require manual addition of issues. Use `./scripts/project/manage-board.sh` to add issues to the project board.

### 📝 **Issue Templates**
- 🐛 **[Bug Reports](./.github/ISSUE_TEMPLATE/bug-report.yml)** — Report issues with detailed information
- ✨ **[Feature Requests](./.github/ISSUE_TEMPLATE/feature-request.yml)** — Suggest new features and improvements
- 📚 **[Documentation Issues](./.github/ISSUE_TEMPLATE/documentation.yml)** — Help improve documentation
- 🔒 **[Security Issues](./.github/ISSUE_TEMPLATE/security.yml)** — Report security concerns and improvements

### 🚀 **Getting Started**
1. **Check existing work**: Browse [open issues](https://github.com/johnzilla/trustedge/issues) and [project board](https://github.com/users/johnzilla/projects/2)
2. **Read the guides**: Review [CONTRIBUTING.md](./CONTRIBUTING.md) and [DEVELOPMENT.md](./DEVELOPMENT.md)
3. **Pick an issue**: Start with issues labeled `good-first-issue` or current [Phase 3 tasks](https://github.com/johnzilla/trustedge/milestone/1)
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

**Project**: [TrustEdge](https://github.com/johnzilla/trustedge) — Privacy and trust at the edge.

**Third-party Dependencies**: See **[Cargo.toml](./trustedge-audio/Cargo.toml)** for complete dependency information and licenses.
