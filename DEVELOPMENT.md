<!--
Copyright (c) 2025 John Turner
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/johnzilla/trustedge
-->
# TrustEdge Development Guide

Development information, roadmap, and contribution guidelines for TrustEdge.

## Table of Contents
- [Project Architecture](#project-architecture)
- [Development Roadmap](#development-roadmap)
- [Development Setup](#development-setup)
- [Contribution Guidelines](#contribution-guidelines)
- [Code Quality Standards](#code-quality-standards)
- [Security Considerations](#security-considerations)

---

## Project Architecture

### Core Components

```
trustedge-audio/
├── src/
│   ├── lib.rs           # Main library interface
│   ├── main.rs          # CLI application entry point
│   ├── format.rs        # TrustEdge format implementation
│   ├── vectors.rs       # Test vectors and validation
│   └── bin/
│       ├── trustedge-client.rs  # Network client
│       └── trustedge-server.rs  # Network server
└── tests/
    ├── cli_roundtrip.rs # End-to-end CLI tests
    └── vectors.rs       # Test vector validation
```

### Backend Architecture

TrustEdge uses a modular backend system for key management:

```rust
pub trait KeyBackend {
    fn get_key(&self, params: &KeyParams) -> Result<SecretKey, KeyError>;
    fn backend_type(&self) -> &'static str;
}
```

#### Current Backends
- **Keyring Backend**: PBKDF2 with OS keyring integration
- **Hex Backend**: Direct hexadecimal key specification

#### Planned Backends
- **TPM Backend**: Hardware security module integration
- **HSM Backend**: Hardware security module support
- **Cloud Backend**: Cloud key management service integration

### Chunked Encryption System

TrustEdge implements chunked encryption for performance and streaming:

1. **Input Processing**: Files are split into configurable chunks (default 4096 bytes)
2. **Per-Chunk Encryption**: Each chunk encrypted with AES-256-GCM
3. **Metadata Protection**: Chunk headers contain encrypted length and integrity data
4. **Stream Reconstruction**: Chunks can be processed independently and recombined

---

## Development Roadmap

### Phase 1: Foundation ✅ (COMPLETED)
- [x] Core TrustEdge format implementation
- [x] Chunked encryption with AES-256-GCM
- [x] Basic CLI interface
- [x] File-based round-trip encryption/decryption
- [x] Test vector validation system
- [x] Initial documentation

### Phase 2: Key Management ✅ (COMPLETED)  
- [x] Pluggable backend architecture
- [x] Keyring backend with PBKDF2
- [x] Passphrase management integration
- [x] Backend selection CLI options
- [x] Enhanced error handling
- [x] Professional code quality (clippy clean)

### Phase 3: Network Operations 🔄 (IN PROGRESS)
- [x] Basic client-server architecture
- [x] TCP connection handling
- [x] Chunk streaming over network
- [ ] **Connection management improvements**
- [ ] **Server authentication**  
- [ ] **Client certificate validation**
- [ ] **Concurrent client handling**
- [ ] **Network error recovery**

### Phase 4: Security Hardening 🔄 (PLANNED)
- [ ] **TPM backend implementation**
- [ ] **Hardware security module support**
- [ ] **Key rotation mechanisms**
- [ ] **Secure key derivation audit**
- [ ] **Side-channel attack mitigation**
- [ ] **Memory protection improvements**

### Phase 5: Production Features 📋 (PLANNED)
- [ ] **Performance optimizations**
- [ ] **Compression integration**
- [ ] **Metadata preservation**
- [ ] **Batch processing modes**
- [ ] **API library interface**
- [ ] **Language bindings (Python, C)**

### Phase 6: Enterprise Features 📋 (FUTURE)
- [ ] **Multi-user access control**
- [ ] **Audit logging system**
- [ ] **Policy-based encryption**
- [ ] **Cloud storage integration**
- [ ] **Compliance reporting**
- [ ] **Enterprise key management**

---

## Development Setup

### Prerequisites

```bash
# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Install additional tools
rustup component add clippy rustfmt
cargo install cargo-audit cargo-outdated
```

### Project Setup

```bash
# Clone repository
git clone https://github.com/yourusername/trustedge.git
cd trustedge/trustedge-audio

# Build project
cargo build --release

# Run tests
cargo test

# Check code quality
cargo clippy -- -D warnings
cargo fmt --check
cargo audit
```

### Development Workflow

```bash
# 1. Create feature branch
git checkout -b feature/tpm-backend

# 2. Implement changes
# ... edit code ...

# 3. Validate changes
cargo test
cargo clippy -- -D warnings
cargo fmt

# 4. Test end-to-end
cargo run --release -- --input test.txt --envelope test.trst --key-hex $(openssl rand -hex 32)
cargo run --release -- --decrypt --input test.trst --out roundtrip.txt

# 5. Commit and push
git add .
git commit -m "feat: implement TPM backend support"
git push origin feature/tpm-backend
```

---

## Contribution Guidelines

### Code Standards

1. **Rust Conventions**
   - Follow official Rust style guidelines
   - Use `cargo fmt` for consistent formatting
   - All code must pass `cargo clippy -- -D warnings`
   - Document public APIs with rustdoc comments

2. **Error Handling**
   - Use `Result<T, Error>` for all fallible operations
   - Provide descriptive error messages
   - Implement proper error chaining
   - Never use `unwrap()` in production code

3. **Testing Requirements**
   - Unit tests for all new functionality
   - Integration tests for CLI operations
   - Property-based tests for cryptographic functions
   - Test vector validation for format changes

4. **Documentation**
   - Update relevant markdown files
   - Include inline code documentation
   - Provide usage examples
   - Update CLI help text

### Security Guidelines

1. **Cryptographic Standards**
   - Use only well-established algorithms (AES-256-GCM, PBKDF2, etc.)
   - Implement proper key derivation
   - Ensure secure random number generation
   - Follow timing-attack mitigation practices

2. **Memory Management**
   - Clear sensitive data from memory when possible
   - Use `zeroize` crate for key material
   - Avoid exposing keys in debug output
   - Minimize key lifetime in memory

3. **Input Validation**
   - Validate all user inputs
   - Sanitize file paths
   - Check buffer bounds
   - Implement proper length checks

### Pull Request Process

1. **Before Submitting**
   - Run complete test suite
   - Verify clippy and format compliance
   - Update documentation
   - Test CLI functionality manually

2. **PR Description Should Include**
   - Clear description of changes
   - Motivation for the change
   - Testing performed
   - Breaking changes (if any)
   - Related issues

3. **Review Process**
   - Code review by project maintainers
   - Security review for cryptographic changes
   - Performance impact assessment
   - Documentation review

---

## Code Quality Standards

### Automated Checks

```bash
# Complete quality check script
#!/bin/bash
set -e

echo "Running code quality checks..."

# Format check
echo "Checking formatting..."
cargo fmt --check

# Clippy linting
echo "Running clippy..."
cargo clippy -- -D warnings

# Security audit
echo "Running security audit..."
cargo audit

# Test execution
echo "Running tests..."
cargo test

# Build verification
echo "Verifying release build..."
cargo build --release

echo "All checks passed!"
```

### Performance Standards

- **Memory Usage**: Streaming processing should use <50MB RAM regardless of file size
- **Throughput**: Target >10MB/s encryption throughput on modern hardware
- **Latency**: Network operations should have <100ms overhead per chunk
- **Scalability**: Server should handle 100+ concurrent connections

### Security Standards

- **Zero Warnings**: No clippy warnings or compiler warnings allowed
- **No Deprecated APIs**: Remove deprecated code immediately
- **Secure Defaults**: All defaults should be secure configurations
- **Regular Updates**: Dependencies updated monthly for security patches

---

## Security Considerations

### Threat Model

See [THREAT_MODEL.md](./THREAT_MODEL.md) for complete threat analysis.

**Key Security Properties:**
- Confidentiality: AES-256-GCM encryption
- Integrity: Authenticated encryption prevents tampering
- Authenticity: Key derivation ties encryption to authorized users
- Forward Secrecy: Independent chunk encryption limits exposure

### Cryptographic Choices

| Component | Algorithm | Justification |
|-----------|-----------|---------------|
| Encryption | AES-256-GCM | NIST approved, authenticated encryption |
| Key Derivation | PBKDF2-SHA256 | Industry standard, configurable iterations |
| Random Generation | OS crypto API | Hardware-backed randomness |
| Memory Protection | zeroize | Secure memory clearing |

### Security Review Process

1. **Code Review Checklist**
   - [ ] No hardcoded keys or secrets
   - [ ] Proper input validation
   - [ ] Secure error handling (no information leakage)
   - [ ] Memory clearing for sensitive data
   - [ ] Timing attack mitigation

2. **Cryptographic Review**
   - [ ] Algorithm selection justified
   - [ ] Key derivation parameters appropriate
   - [ ] Random number generation secure
   - [ ] Side-channel considerations addressed

3. **Network Security**
   - [ ] TLS encryption for network transport
   - [ ] Server authentication implemented
   - [ ] Input sanitization on network boundaries
   - [ ] DoS protection mechanisms

---

## Testing Strategy

### Test Categories

1. **Unit Tests**
   ```bash
   # Run specific test modules
   cargo test vectors::test_
   cargo test format::test_
   cargo test backends::test_
   ```

2. **Integration Tests**
   ```bash
   # CLI round-trip testing
   cargo test --test cli_roundtrip
   
   # Network functionality
   cargo test --test network_integration
   ```

3. **Security Tests**
   ```bash
   # Test vector validation
   cargo test --test vectors
   
   # Fuzzing (when implemented)
   cargo fuzz run format_parser
   ```

4. **Performance Tests**
   ```bash
   # Benchmark critical paths
   cargo bench
   
   # Memory usage profiling
   valgrind --tool=massif ./target/release/trustedge-audio
   ```

### Continuous Integration

GitHub Actions workflow ensures:
- All tests pass on multiple platforms
- Code quality standards maintained
- Security audits pass
- Documentation builds successfully
- Example code validates correctly

---

## Future Considerations

### Planned Enhancements

1. **WebAssembly Support**
   - Browser-based encryption
   - Client-side privacy protection
   - Progressive web app integration

2. **Mobile Platform Support**
   - iOS/Android bindings
   - Mobile-optimized performance
   - Platform-specific security features

3. **Cloud Integration**
   - AWS KMS backend
   - Azure Key Vault support
   - Google Cloud KMS integration
   - Multi-cloud key replication

4. **Advanced Features**
   - Compression before encryption
   - Deduplication support
   - Incremental backup systems
   - Real-time streaming optimization

### Research Areas

- **Post-Quantum Cryptography**: Preparing for quantum-resistant algorithms
- **Homomorphic Encryption**: Computing on encrypted data
- **Zero-Knowledge Proofs**: Verification without data exposure
- **Secure Multi-Party Computation**: Collaborative processing

---

For testing procedures, see [TESTING.md](./TESTING.md).

For usage examples, see [EXAMPLES.md](./EXAMPLES.md).

For CLI reference, see [CLI.md](./CLI.md).
