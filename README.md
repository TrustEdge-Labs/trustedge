<!--
Copyright (c) 2025 John Turner
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/johnzilla/trustedge
-->


# TrustEdge — Trustable Edge AI (Rust)

> Not another CRUD app. Learning Rust through **Trustable Edge AI** — privacy-preserving edge pipelines.

---

## Why This Project?

Most people learning Rust start with CRUD web apps. I'm taking a different route that aligns with my background in IoT product development, security/PKI and edge systems:

* **Privacy by design**: encrypt at the edge, not just TLS in transit
* **Rust at the edge**: safety + performance for streaming workloads  
* **Learning in public**: small, honest milestones → real, reviewable code

**TrustEdge** is a Rust prototype for privacy-preserving, provenance-aware edge audio.

- **Private by default:** audio chunks are encrypted with AES-256-GCM before leaving the device
- **Provenance by design:** each chunk carries a signed manifest (C2PA-inspired) whose hash is bound into AEAD AAD; tampering breaks decryption
- **Streaming-friendly:** fixed nonce discipline (prefix||counter) and per-chunk records

**Technology Stack:**
- Language: Rust (stable)
- Crypto: `aes-gcm` (AEAD), 256-bit keys, 96-bit nonces
- Key Management: Pluggable backends (keyring, TPM, HSM planned)

---

## Quick Start

### Installation

```bash
# Install Rust (if needed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and build
git clone https://github.com/yourusername/trustedge.git
cd trustedge/trustedge-audio
cargo build --release
```

### Basic Usage

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

**Start Server:**
```bash
./target/release/trustedge-server 
  --port 8080 
  --decrypt 
  --output-dir ./received 
  --backend keyring 
  --salt-hex $(openssl rand -hex 16)
```

**Send Data from Client:**
```bash
./target/release/trustedge-client 
  --server 127.0.0.1:8080 
  --input data.wav 
  --backend keyring 
  --salt-hex <same-salt-as-server>
```

---

## How It Works

TrustEdge processes files in configurable chunks (default 4KB) with the following security properties:

1. **Per-Chunk Encryption**: Each chunk encrypted with AES-256-GCM
2. **Signed Manifests**: Ed25519 signatures provide authenticity and provenance
3. **Integrity Binding**: Cryptographic binding prevents tampering and replay attacks
4. **Streaming Support**: Chunks can be processed independently for real-time workflows

**Key Features:**
- ✅ Chunked encryption for memory efficiency
- ✅ Authenticated encryption (AES-256-GCM)
- ✅ Pluggable key management backends
- ✅ Network streaming support
- ✅ Comprehensive validation and error handling
- ✅ Test vector validation for format stability

---

## Documentation

### User Guides
- **[CLI.md](./CLI.md)** — Complete command-line reference with examples
- **[EXAMPLES.md](./EXAMPLES.md)** — Real-world usage examples and workflows
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

**🔄 Phase 3: Network Operations (IN PROGRESS)**
- Basic client-server architecture ✅
- Enhanced connection management 🔄
- Server authentication and client validation 📋

**📋 Phase 4: Security Hardening (PLANNED)**
- TPM backend implementation
- Hardware security module support  
- Key rotation mechanisms

See **[DEVELOPMENT.md](./DEVELOPMENT.md)** for complete roadmap and contribution guidelines.

---

## Security

**Current Security Properties:**
- AES-256-GCM authenticated encryption
- Ed25519 digital signatures for provenance
- PBKDF2 key derivation (100,000 iterations)
- Comprehensive validation prevents tampering

**Security Limitations:**
- Demo/development keys (not production-ready)
- No key rotation or revocation yet
- Limited to software-based key storage

For detailed security analysis, see **[THREAT_MODEL.md](./THREAT_MODEL.md)**.

**Vulnerability Reporting:** See **[SECURITY.md](./SECURITY.md)** for responsible disclosure process.

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
