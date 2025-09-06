<!--
Copyright (c) 2025 John Turner
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/johnzilla/trustedge
-->


# TrustEdge AI Coding Agent Instructions

## 🏗️ Project Architecture

**TrustEdge** is a privacy-preserving edge data encryption platform built in Rust with a modular architecture:

- **Core Library** (`trustedge-audio/src/lib.rs`): Data-agnostic encryption with NetworkChunk abstraction
- **Universal Backend System** (`src/backends/`): Capability-based crypto operations (Software HSM, Keyring, future TPM/YubiKey)
- **Transport Layer** (`src/transport/`): Async trait abstraction over TCP/QUIC for network operations
- **Format Detection** (`src/format.rs`): MIME type detection and C2PA-inspired signed manifests
- **Authentication** (`src/auth.rs`): Ed25519-based mutual authentication with session management

### Key Architectural Patterns

1. **Universal Backend Pattern**: All crypto operations use `CryptoOperation` enum → `CryptoResult` pattern with capability discovery
2. **Transport Abstraction**: Unified async interface over TCP (length-prefixed) and QUIC (built-in framing)
3. **Provenance by Design**: Each data chunk carries signed manifest bound to AEAD AAD
4. **Streaming Architecture**: Fixed nonce discipline (prefix||counter) for real-time processing

## 🔧 Essential Development Commands

### Quality Checks (Run Before Every Commit)
```bash
# Run from trustedge-audio/ directory
./ci-check.sh                    # Prevents GitHub CI failures
```

This script runs the **exact same checks** as GitHub CI:
- `cargo fmt --check` (formatting)
- `cargo clippy --all-targets --no-default-features -- -D warnings` (strict linting)
- `cargo build --all-targets` (build validation)
- `cargo test` (all 93 tests)

### Test Categories
```bash
cargo test --lib                                    # Unit tests (53)
cargo test --test software_hsm_integration          # Software HSM integration (9)
cargo test --test roundtrip_integration             # End-to-end workflows (15)
cargo test --test auth_integration                  # Authentication (3)
cargo test --test network_integration               # Network operations (7)
cargo test --test universal_backend_integration     # Backend selection (6)
```

### Development Workflow
```bash
# Feature development pattern
cargo run --example transport_demo                  # Test transport abstraction
cargo run --bin software-hsm-demo generate ed25519 test_key  # Test crypto backends
cargo run -- --input test.txt --envelope out.trst --key-hex $(openssl rand -hex 32)  # Test encryption
```

## 🎯 Project-Specific Conventions

### Universal Backend Implementation
When implementing new backends (TPM, YubiKey, HSM, etc.):

1. **Implement trait methods** in this order:
   ```rust
   impl UniversalBackend for YourBackend {
       fn perform_operation(&self, key_id: &str, operation: CryptoOperation) -> Result<CryptoResult>
       fn supports_operation(&self, operation: &CryptoOperation) -> bool
       fn get_capabilities(&self) -> BackendCapabilities
       fn backend_info(&self) -> BackendInfo
       fn list_keys(&self) -> Result<Vec<KeyMetadata>>
   }
   ```

2. **Capability Discovery Pattern**: Use `supports_operation()` for runtime checks, `get_capabilities()` for static discovery
3. **Registry Integration**: Register with `UniversalBackendRegistry` using preference-based selection

### Specific Backend Examples

**YubiKey Backend Implementation:**
```rust
impl UniversalBackend for YubiKeyBackend {
    fn perform_operation(&self, key_id: &str, operation: CryptoOperation) -> Result<CryptoResult> {
        match operation {
            CryptoOperation::Sign { data, algorithm } => {
                let signature = self.piv_sign(key_id, &data, algorithm)?;
                Ok(CryptoResult::Signed(signature))
            }
            CryptoOperation::Attest { challenge } => {
                let proof = self.hardware_attest(&challenge)?;
                Ok(CryptoResult::AttestationProof(proof))
            }
            _ => Err(anyhow!("Operation not supported by YubiKey backend"))
        }
    }
    
    fn get_capabilities(&self) -> BackendCapabilities {
        BackendCapabilities {
            hardware_backed: true,
            supports_attestation: true,
            asymmetric_algorithms: vec![AsymmetricAlgorithm::EcdsaP256],
            signature_algorithms: vec![SignatureAlgorithm::EcdsaP256],
            max_key_size: Some(256),
            ..Default::default()
        }
    }
}
```

**TPM 2.0 Backend Pattern:**
```rust
impl UniversalBackend for TpmBackend {
    fn backend_info(&self) -> BackendInfo {
        BackendInfo {
            name: "tpm",
            description: "TPM 2.0 hardware security module",
            available: self.detect_tpm_availability(),
            config_requirements: vec!["device_path", "key_handle"],
            ..Default::default()
        }
    }
}
```

### Terminal Output Standards
**Always use professional UTF-8 symbols** (never emojis):
- Success: `✔` (U+2714)
- Error: `✖` (U+2716)
- Warning: `⚠` (U+26A0)
- Info: `●` (U+25CF)
- Audio: `♪` (U+266A)
- Video: `■` (U+25A0)

### Network Protocol Pattern
When working with transport layer:
```rust
// Always use NetworkChunk for network data
let chunk = NetworkChunk::new(sequence, encrypted_data, manifest);

// Use Transport trait for protocol-agnostic networking
let transport = TransportFactory::create_quic(config)?;
transport.connect(addr).await?;
transport.send_chunk(&chunk).await?;
```

## 🧪 Testing Patterns

### Integration Test Structure
Follow the pattern in `tests/software_hsm_integration.rs`:
1. **Setup Phase**: Create test directories, configurations
2. **Backend Creation**: Use `with_config()` for custom setups
3. **Registry Integration**: Test capability-based selection
4. **Cleanup**: Use `TempDir` for automatic cleanup

### Roundtrip Testing
Critical pattern for end-to-end validation:
```rust
// Encrypt → Inspect → Decrypt → Verify integrity
let original_data = fs::read(&input_path)?;
// ... encryption logic ...
let decrypted_data = fs::read(&output_path)?;
assert_eq!(original_data, decrypted_data, "Roundtrip integrity failed");
```

### Cross-Session Persistence Testing
```rust
// Session 1: Create and use
let signature = {
    let mut backend = Backend::with_config(config.clone())?;
    backend.generate_key_pair("persistent_key", algorithm, None)?;
    backend.sign_data("persistent_key", test_data, algorithm)?
};

// Session 2: Load and verify  
{
    let backend = Backend::with_config(config)?;
    let keys = backend.list_keys()?;
    assert_eq!(keys.len(), 1);
    assert!(backend.verify_signature("persistent_key", test_data, &signature, algorithm)?);
}
```

## 🔍 Key Files for Understanding

- `UNIVERSAL_BACKEND.md`: Comprehensive backend system documentation
- `src/backends/software_hsm.rs`: Reference backend implementation (350+ lines)
- `src/transport/quic.rs`: Modern async networking with Quinn 0.11
- `tests/roundtrip_integration.rs`: End-to-end testing patterns
- `examples/transport_demo.rs`: Transport abstraction usage
- `TESTING_PATTERNS.md`: Comprehensive testing guide (see below)

## 📚 Essential Documentation

**Core Architecture:**
- `README.md`: Project overview and getting started
- `UNIVERSAL_BACKEND.md`: Backend system architecture
- `PROTOCOL.md`: Network protocol specification
- `AUTHENTICATION_GUIDE.md`: Authentication implementation

**Development:**
- `DEVELOPMENT.md`: Development setup and workflows
- `CODING_STANDARDS.md`: Code quality requirements
- `CONTRIBUTING.md`: Contribution guidelines
- `TESTING.md`: Testing strategy and execution

**Security & Operations:**
- `SECURITY.md`: Security policy and features
- `THREAT_MODEL.md`: Security threat analysis
- `ROADMAP.md`: Future development plans
- `TROUBLESHOOTING.md`: Common issues and solutions

## ⚠️ Common Pitfalls

1. **CI Failures**: Always run `./ci-check.sh` before commits
2. **Backend Mutability**: Some operations need `&mut self` - clone backend for demos
3. **Transport Framing**: TCP needs length-prefixed messages, QUIC has built-in framing
4. **Capability Checks**: Always verify `supports_operation()` before calling `perform_operation()`
5. **Professional Output**: Use UTF-8 symbols, not emojis in terminal output
6. **Backend Config**: Each backend type has specific `config_requirements` - check `BackendInfo`

Focus on the Universal Backend system for extensibility - it's the foundation for all future crypto integrations (TPM, HSM, YubiKey, post-quantum).
