<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->

# TrustEdge AI Coding Agent Instructions

## 🏗️ Project Architecture

**TrustEdge** is a privacy-preserving edge data encryption platform built as a **Cargo workspace** with specialized crates:

### Workspace Structure (9 Crates)
```
trustedge/
├── crates/core/              # trustedge-core: Core crypto library + CLI binaries
├── crates/trst-cli/          # trustedge-trst-cli: .trst archive CLI (binary: trst)
├── crates/trst-core/         # trustedge-trst-core: Archive format primitives
├── crates/attestation/       # trustedge-attestation: Software attestation system
├── crates/receipts/          # trustedge-receipts: Digital receipt ownership chains
├── crates/wasm/              # trustedge-wasm: Core WebAssembly bindings
├── crates/trst-wasm/         # trustedge-trst-wasm: Archive verification WASM
├── crates/pubky/             # trustedge-pubky: Decentralized key discovery
└── crates/pubky-advanced/    # trustedge-pubky-advanced: Hybrid encryption
```

### Core Architectural Components (`crates/core/src/`)

- **Universal Backend System** (`backends/`): Capability-based crypto operations (Software HSM, Keyring, YubiKey)
- **Transport Layer** (`transport/`): Async trait abstraction over TCP/QUIC
- **Archive System** (`archive.rs`): .trst directory format with manifest + signatures
- **Continuity Chain** (`chain.rs`): BLAKE3-based segment linking with genesis seed
- **Crypto Operations** (`crypto.rs`): XChaCha20-Poly1305 encryption, Ed25519 signing
- **Manifest** (`manifest.rs`): Canonical JSON serialization for cam.video profile
- **Authentication** (`auth.rs`): Ed25519-based mutual authentication with sessions

### Key Architectural Patterns

1. **Universal Backend Pattern**: All crypto operations use `CryptoOperation` enum → `CryptoResult` with capability discovery
2. **Workspace Organization**: Each crate has specific responsibility - use correct package with `cargo run -p <package-name>`
3. **P0 Golden Profile**: `cam.video` profile is locked specification - cannot change after P0 completion
4. **Continuity Chain**: BLAKE3 genesis seed `blake3("trustedge:genesis")` links segments with hash chains
5. **Canonical Manifests**: Ordered JSON keys with signature field excluded from canonicalization

## 🔧 Essential Development Commands

### Quality Checks (Run Before Every Commit)
```bash
# From workspace root - checks ALL crates
./scripts/ci-check.sh         # Auto-formats, clippy, build, test entire workspace

# From crates/core/ - checks only core crate
./ci-check.sh                 # Auto-formats, clippy with features, build, test
```

**Critical CI alignment**: These scripts run the **exact same checks** as GitHub CI to prevent failures:
- `cargo fmt --check --all` (workspace) or `cargo fmt` (core - auto-fixes)
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo build --workspace --all-targets --all-features`
- `cargo test --workspace --all-features`

### Workspace Commands (from project root)
```bash
# Build specific crate
cargo build -p trustedge-trst-cli
cargo build -p trustedge-core --features audio,yubikey

# Run specific binary
cargo run -p trustedge-trst-cli -- wrap --profile cam.video --in input.bin --out archive.trst
cargo run -p trustedge-core --bin software-hsm-demo generate ed25519 test_key

# Test specific crate
cargo test -p trustedge-core
cargo test -p trustedge-receipts
cargo test -p trustedge-trst-cli --test acceptance  # Acceptance tests only

# Run all workspace tests
cargo test --workspace --all-features

# Makefile shortcuts
make build              # Build entire workspace
make test               # Test entire workspace
make demo               # P0 golden path demo (wrap + verify)
make ci-check           # Full CI validation
```

### P0 Golden Path (cam.video)
```bash
# Complete P0 workflow in 4 commands
head -c 32M </dev/urandom > sample.bin
cargo run -p trustedge-trst-cli -- wrap --profile cam.video --in sample.bin --out clip.trst
cargo run -p trustedge-trst-cli -- verify clip.trst --device-pub "$(cat device.pub)" --json
cargo test -p trustedge-trst-cli --test acceptance  # A1-A6 test suite

# Deterministic testing (CI)
cargo run -p trustedge-trst-cli -- wrap --profile cam.video --in sample.bin --out clip.trst --seed 42
```

## 🎯 Project-Specific Conventions

### Copyright Headers (Required)
**Every source file MUST have the correct copyright header**:

**Rust files (.rs):**
```rust
//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//
```

**Markdown files (.md):**
```markdown
<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->
```

**Run `./scripts/fix-copyright.sh` to auto-add headers to missing files.**

### Terminal Output Standards
**Always use professional UTF-8 symbols** (never emojis):
- Success: `✔` (U+2714)
- Error: `✖` (U+2716)
- Warning: `⚠` (U+26A0)
- Info: `●` (U+25CF)
- Audio: `♪` (U+266A)
- Video: `■` (U+25A0)

### Crate Naming Convention
- **Package name**: `trustedge-<name>` (with hyphens, lowercase)
- **Binary name**: Short form without prefix (e.g., `trst`, `trustedge-attest`)
- **Library name**: `trustedge_<name>` (with underscores for Rust imports)

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

### Archive (.trst) Format Patterns
```rust
// Archive directory structure:
// clip-<id>.trst/
//   ├── manifest.json          # Canonical cam.video manifest
//   ├── signatures/
//   │   └── manifest.sig        # Detached Ed25519 signature
//   └── chunks/
//       ├── 00000.bin           # Zero-padded 5-digit chunk filenames
//       ├── 00001.bin
//       └── ...

// Key functions in crates/core/src/archive.rs:
write_archive(dir, manifest, chunks)  // Create .trst archive
read_archive(dir)                      // Load archive for verification
validate_archive(dir)                  // Full integrity check

// Key functions in crates/trst-core/src/:
wrap::wrap_archive()                   // CLI wrap implementation
verify::verify_archive()               // CLI verify implementation
```

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

### Acceptance Test Pattern (P0)
See `crates/trst-cli/tests/acceptance.rs` for the A1-A6 test suite:
- **A1**: Basic wrap and verify workflow
- **A2**: Signature validation failure detection
- **A3**: Continuity chain gap detection
- **A4**: Out-of-order chunk detection
- **A5**: Truncated archive detection
- **A6**: Comprehensive JSON verification output

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
- `FEATURES.md`: **Comprehensive feature flag reference** - All cargo features, dependencies, and usage
- `WASM.md`: **Complete WASM build and deployment guide** - Browser/Node.js integration

**Security & Operations:**
- `SECURITY.md`: Security policy and features
- `THREAT_MODEL.md`: Security threat analysis
- `ROADMAP.md`: Future development plans
- `TROUBLESHOOTING.md`: Common issues and solutions

**Future Enhancements:**
- `RFC_K256_SUPPORT.md`: **secp256k1 (Bitcoin/Ethereum) integration plan** - K1 curve support alongside P-256

## ⚠️ Common Pitfalls

1. **CI Failures**: Always run `./scripts/ci-check.sh` (workspace) or `./ci-check.sh` (core crate) before commits
2. **Feature Flags**: Check `FEATURES.md` for complete feature documentation - audio/yubikey must be explicitly enabled
3. **WASM Builds**: See `WASM.md` for complete build/test/deploy guide - requires `wasm-pack` installation
4. **Backend Mutability**: Some operations need `&mut self` - clone backend for demos
5. **Transport Framing**: TCP needs length-prefixed messages, QUIC has built-in framing
6. **Capability Checks**: Always verify `supports_operation()` before calling `perform_operation()`
7. **Professional Output**: Use UTF-8 symbols, not emojis in terminal output
8. **Backend Config**: Each backend type has specific `config_requirements` - check `BackendInfo`
9. **Curve Selection**: P-256 (R1) for YubiKey/TPM hardware, K1 for Bitcoin/Ethereum (see `RFC_K256_SUPPORT.md`)

Focus on the Universal Backend system for extensibility - it's the foundation for all future crypto integrations (TPM, HSM, YubiKey, post-quantum, K1 curves).
