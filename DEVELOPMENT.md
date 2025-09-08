<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->
# TrustEdge Development Guide

Development information, roadmap, and contribution guidelines for TrustEdge.

## Table of Contents
- [Project Management](#project-management)
- [Project Architecture](#project-architecture)
- [Development Roadmap](#development-roadmap)
- [Development Setup](#development-setup)
- [Contribution Guidelines](#contribution-guidelines)
- [Code Quality Standards](#code-quality-standards)
- [Security Considerations](#security-considerations)

---

## Project Management

### 📊 GitHub Project Organization

**Project Board**: [TrustEdge Development](https://github.com/TrustEdge-Labs/projects/2)
- Visual task tracking and progress monitoring
- Kanban-style organization (Todo, In Progress, Done)
- Integrated with GitHub issues and milestones
- **Note**: Issues must be manually added to project boards

**Issue Tracking**: [GitHub Issues](https://github.com/TrustEdge-Labs/trustedge/issues)
- Structured issue templates for bugs, features, docs, and security
- Comprehensive labeling system for organization
- Milestone-based development tracking
- All repository issues are listed here (whether on project board or not)

**Development Phases**: [Milestones](https://github.com/TrustEdge-Labs/trustedge/milestones)
- Phase 3: Network Operations (Current)
- Individual day-based milestones (Day 10-14)
- Clear deliverables and acceptance criteria

### 🛠️ Project Management Tools

**Status Checking**:
```bash
# Quick project status overview
./scripts/project/check-status.sh

# View specific milestone
gh issue list --milestone "Day 10: Server Authentication"

# View all Phase 3 issues
gh issue list --label "phase-3"
```

**Issue Management**:
```bash
# Create new issue with template
gh issue create --template bug-report

# Assign issue to yourself
gh issue edit <issue-number> --add-assignee @me

# Update issue status
gh issue edit <issue-number> --add-label "in-progress"

# Add issue to project board
./scripts/project/manage-board.sh
```

### 📋 Current Development Status

See **[GitHub Issues](https://github.com/TrustEdge-Labs/trustedge/issues)** for detailed tasks and **[Issue #16](https://github.com/TrustEdge-Labs/trustedge/issues/16)** for progress tracking.

---

## Project Architecture

### Core Components

```
trustedge-core/
├── src/
│   ├── lib.rs           # Main library interface
│   ├── main.rs          # CLI application entry point
│   ├── format.rs        # TrustEdge format with data type metadata
│   ├── audio.rs         # Live audio capture implementation (feature-gated)
│   ├── auth.rs          # Ed25519-based mutual authentication
│   ├── vectors.rs       # Test vectors and validation
│   ├── backends/        # Universal Backend System
│   │   ├── mod.rs       # Backend module interface
│   │   ├── universal.rs # Universal backend trait and registry
│   │   ├── software_hsm.rs # Software HSM implementation (33 tests)
│   │   ├── keyring.rs   # OS keyring integration
│   │   └── yubikey.rs   # YubiKey hardware backend (PKCS#11)
│   ├── transport/       # Network transport abstraction
│   │   ├── mod.rs       # Transport trait and configuration
│   │   ├── tcp.rs       # TCP transport with length framing (8 tests)
│   │   └── quic.rs      # QUIC transport with TLS (8 tests)
│   └── bin/
│       ├── trustedge-client.rs  # Network client
│       ├── trustedge-server.rs  # Network server
│       ├── inspect-trst.rs      # Metadata inspection utility
│       └── software-hsm-demo.rs # Backend demonstration
├── tests/               # Integration test suite (65 tests)
│   ├── yubikey_integration.rs      # YubiKey hardware tests
│   ├── transport_integration.rs   # Transport layer tests (10)
│   ├── software_hsm_integration.rs # HSM integration tests (9)
│   ├── auth_integration.rs         # Authentication tests (3)
│   ├── network_integration.rs      # Network tests (7)
│   ├── roundtrip_integration.rs    # End-to-end tests (15)
│   ├── universal_backend_integration.rs # Backend tests (6)
│   └── domain_separation_test.rs   # Security tests (7)
└── examples/            # Comprehensive demonstration examples
    ├── yubikey_quic_demo.rs       # Phase 3 QUIC integration demo
    ├── yubikey_certificate_demo.rs # Certificate generation demo
    ├── transport_demo.rs          # Transport abstraction demo
    └── universal_backend_demo.rs  # Backend selection demo
```

### Testing Architecture

**144 Total Tests** covering all system components:

**Unit Tests (79):**
- Core library functionality
- Transport layer (QUIC/TCP) validation
- Universal Backend system testing
- Software HSM comprehensive coverage

**Integration Tests (65):**
- YubiKey hardware integration (PKCS#11)
- Transport layer end-to-end validation
- Authentication and session management
- Network communication workflows
- Security and domain separation

**Quality Assurance:**
```bash
# Complete test suite (all 79 tests)
./ci-check.sh                    # CI pipeline validation

# Test categories
cargo test --lib                 # Unit tests (79)
cargo test --test yubikey_integration     # YubiKey tests (8)
cargo test --test transport_integration   # Transport tests (10)

# Hardware feature testing
cargo test --features yubikey    # Include YubiKey hardware tests
```

### Data-Agnostic Architecture

TrustEdge operates on a data-agnostic model that can encrypt any type of data while preserving relevant metadata:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataType {
    File,
    Audio { format: AudioFormat, sample_rate: u32, channels: u16 },
    Video { format: String, width: u32, height: u32 },
    Sensor { sensor_type: String, units: String },
}
```

#### Live Audio Capture
- **Cross-platform support**: Uses `cpal` library for ALSA/WASAPI/CoreAudio backends
- **Feature-gated compilation**: Optional audio dependencies via `--features audio`
- **Configurable quality**: Sample rate, channel count, duration controls
- **Metadata preservation**: Audio format details stored in manifest

#### Input Abstraction
TrustEdge uses a unified `InputReader` trait for processing different data sources:

```rust
pub trait InputReader {
    fn read_chunk(&mut self, buf: &mut [u8]) -> io::Result<usize>;
    fn size_hint(&self) -> Option<u64>;
}
```

Implementations:
- `FileInputReader`: Traditional file processing
- `AudioInputReader`: Live audio capture (when audio feature enabled)

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

### Phase 3: Network Operations ✅ (COMPLETED)
- [x] Basic client-server architecture
- [x] TCP connection handling
- [x] Chunk streaming over network
- [x] **Connection management improvements**
- [x] **Connection timeouts and retry logic**
- [x] **Graceful server shutdown handling**
- [x] **Live audio capture integration**
- [x] **Data-agnostic architecture with metadata**
- [x] **Feature-gated compilation for audio dependencies**
- [x] **Cross-platform audio support (Linux/Windows/macOS)**
- [ ] **Server authentication**  
- [ ] **Client certificate validation**
- [ ] **Concurrent client handling**
- [ ] **Network error recovery**

### Phase 4: Security Hardening 🔄 (NEXT)
- [ ] **Server authentication**  
- [ ] **Client certificate validation**
- [ ] **Concurrent client handling**
- [ ] **Network error recovery**
- [ ] **TPM backend implementation**
- [ ] **Hardware security module support**
- [ ] **Key rotation mechanisms**
- [ ] **Secure key derivation audit**
- [ ] **Side-channel attack mitigation**
- [ ] **Memory protection improvements**

### Phase 5: Production Features 📋 (PLANNED)
- [ ] **Performance optimizations**
- [ ] **Compression integration**
- [ ] **Advanced metadata preservation**
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
cd trustedge/trustedge-core

# Build project
# Build with audio support
cargo build --release --features audio

# Build with YubiKey hardware support (requires PKCS#11)
cargo build --release --features yubikey

# Build with all features enabled
cargo build --release --features audio,yubikey

# Build without optional features (CI-compatible)
cargo build --release

# Run tests
cargo test

# Test audio features specifically
cargo test --features audio

# Test YubiKey backend (stub implementation)
cargo test --features yubikey

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
cargo test --features audio  # Test audio-specific functionality
cargo clippy -- -D warnings
cargo fmt

# 4. Test end-to-end scenarios
# Basic file encryption
cargo run --release -- --input test.txt --envelope test.trst --key-hex $(openssl rand -hex 32)
cargo run --release -- --decrypt --input test.trst --out roundtrip.txt

# Live audio capture (if audio features enabled)
cargo run --release --features audio -- --audio-capture --duration 5 --envelope voice.trst --key-out key.hex
cargo run --release --features audio -- --decrypt --input voice.trst --out restored.wav --key-hex $(cat key.hex)

# Network mode testing
cargo run --release --bin trustedge-server -- --port 8080 --decrypt &
SERVER_PID=$!
cargo run --release --bin trustedge-client -- --server 127.0.0.1:8080 --input test.txt
kill $SERVER_PID

# 5. Commit and push
git add .
git commit -m "feat: implement TPM backend support"
git push origin feature/tpm-backend
```

---

## Contribution Guidelines

### 🚀 Getting Started

1. **Check Project Status**
   - Visit the [Project Board](https://github.com/TrustEdge-Labs/projects/2) for current priorities
   - Review [GitHub Issues](https://github.com/TrustEdge-Labs/trustedge/issues) for development status
   - Check [open issues](https://github.com/TrustEdge-Labs/trustedge/issues) for available tasks

2. **Choose an Issue**
   - Look for issues labeled `good-first-issue` for newcomers
   - Check current [Phase 3 milestone](https://github.com/TrustEdge-Labs/trustedge/milestone/1) for priority work
   - Assign yourself to issues you want to work on

3. **Use Templates**
   - Follow [PR template](./.github/pull_request_template.md) for submissions
   - Use [issue templates](./.github/ISSUE_TEMPLATE/) for bug reports and features
   - See [CONTRIBUTING.md](./CONTRIBUTING.md) for complete guidelines

### 🔧 Development Workflow

1. **Branch Management**
   ```bash
   # Create feature branch
   git checkout -b feature/day-10-server-auth
   
   # Make changes and commit
   git commit -m "feat(server): add certificate validation"
   
   # Push and create PR
   git push origin feature/day-10-server-auth
   ```

2. **Quality Checks**
   ```bash
   # Run all quality checks (prevents CI failures)
   ./scripts/ci-check.sh
   
   # Or run individual checks:
   cargo fmt --check
   cargo clippy --all-targets --no-default-features -- -D warnings
   cargo test
   ./scripts/project/check-status.sh  # Check issue status
   ```

3. **Issue Updates**
   ```bash
   # Link commits to issues
   git commit -m "feat(auth): implement server cert validation
   
   Implements certificate loading and validation for Day 10.
   
   Closes #11"
   ```

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
   valgrind --tool=massif ./target/release/trustedge-core
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
