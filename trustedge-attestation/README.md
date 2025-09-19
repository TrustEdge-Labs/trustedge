<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->

# TrustEdge Attestation - Software Birth Certificates

**Hardware-backed software attestation and provenance tracking for TrustEdge applications.**

[![License: MPL 2.0](https://img.shields.io/badge/License-MPL_2.0-brightgreen.svg)](https://opensource.org/licenses/MPL-2.0)
[![Hardware](https://img.shields.io/badge/Hardware-YubiKey%20Supported-green.svg)](https://www.yubico.com/)

---

## Overview

TrustEdge Attestation provides cryptographic "birth certificates" for software, enabling verifiable proof of:

- **Source Code Integrity**: Git commit hashes and repository state
- **Build Environment**: Compiler versions, dependencies, and build flags  
- **Development Provenance**: Who built what, when, and where
- **Hardware Attestation**: Cryptographic signatures from YubiKey or HSM
- **Supply Chain Security**: Complete audit trail from source to binary

## Key Features

- **🔐 Hardware-Backed Signing**: Uses YubiKey PIV or HSM for cryptographic operations
- **📋 Git Integration**: Captures commit info, branch state, and working directory status
- **🏗️ Build Environment**: Records Rust/Cargo versions, target arch, and dependencies
- **🔗 Provenance Chains**: Links attestations to form verifiable build history
- **🛡️ Tamper Evidence**: Cryptographic proof of software integrity
- **⚡ Easy Integration**: Simple builder pattern API

## Architecture

The attestation system follows TrustEdge's separation of concerns:

```
┌─────────────────────────────────────────────────────────────┐
│                    Software Attestation                     │
├─────────────────────────────────────────────────────────────┤
│  Business Logic (trustedge-attestation)                    │
│  • GitInfo capture                                         │
│  • BuildInfo collection                                    │  
│  • Metadata management                                     │
│  • Attestation builder                                     │
├─────────────────────────────────────────────────────────────┤
│  Security Layer (trustedge-core)                           │
│  • Hardware-backed signing                                 │
│  • Envelope encryption                                     │
│  • Cryptographic operations                                │
│  • YubiKey/HSM integration                                 │
└─────────────────────────────────────────────────────────────┘
```

## Quick Start

### Basic Attestation

```rust
use trustedge_attestation::{AttestationBuilder, GitInfo, BuildInfo};
use trustedge_core::backends::UniversalBackendRegistry;

// Create hardware-backed attestation
let registry = UniversalBackendRegistry::with_defaults()?;
let backend = registry.get_backend("yubikey").unwrap();

let attestation = AttestationBuilder::new()
    .with_git_info(GitInfo::from_current_repo()?)
    .with_build_info()?
    .with_metadata("purpose".to_string(), "release-v1.0.0".to_string())
    .sign_with_backend(backend, "attestation_key")?;

println!("Software birth certificate created and hardware-signed");
```

### Git Information Capture

```rust
let git_info = GitInfo::from_current_repo()?;

println!("Commit: {}", git_info.commit_hash);
println!("Branch: {}", git_info.branch);
println!("Clean: {}", git_info.is_clean);

if !git_info.is_clean {
    println!("Modified files: {:?}", git_info.modified_files);
}
```

### Build Environment Capture

```rust
let build_info = BuildInfo::capture_current()?;

println!("Rust version: {}", build_info.rustc_version);
println!("Target: {}", build_info.target_arch);
println!("Profile: {}", build_info.profile);
println!("Dependencies: {:#?}", build_info.dependencies);
```

## Use Cases

### Release Signing

Create verifiable attestations for software releases:

```bash
# Build and attest a release
cargo build --release
cargo run --bin create-attestation -- --profile release --purpose "v1.0.0-release"
```

### CI/CD Integration

Integrate into CI pipelines for automated attestation:

```rust
// In CI environment
let attestation = AttestationBuilder::new()
    .with_git_info(GitInfo::from_current_repo()?)
    .with_build_info()?
    .with_metadata("ci_job".to_string(), env::var("CI_JOB_ID")?)
    .with_metadata("ci_runner".to_string(), env::var("CI_RUNNER_ID")?)
    .sign_with_backend(backend, "ci_attestation_key")?;
```

### Supply Chain Verification

Verify software provenance throughout the supply chain:

```rust
// Verify attestation integrity
let is_valid = attestation.verify()?;
println!("Attestation valid: {}", is_valid);

// Check git state was clean
let git_info = &attestation.git_info;
if !git_info.is_clean {
    eprintln!("Warning: Software built from dirty working directory");
}
```

## Data Structures

### GitInfo

Captures complete Git repository state:

```rust
pub struct GitInfo {
    pub commit_hash: String,        // Full SHA-1 commit hash
    pub branch: String,             // Current branch name  
    pub remote_url: Option<String>, // Repository remote URL
    pub is_clean: bool,             // No uncommitted changes
    pub modified_files: Vec<String>, // List of modified files
    pub commit_timestamp: u64,      // Commit time (Unix epoch)
    pub author: String,             // Commit author
    pub message: String,            // Commit message
}
```

### BuildInfo

Records complete build environment:

```rust
pub struct BuildInfo {
    pub rustc_version: String,                    // Rust compiler version
    pub cargo_version: String,                    // Cargo version
    pub target_arch: String,                      // Target architecture
    pub profile: String,                          // debug/release
    pub build_timestamp: u64,                     // Build time
    pub build_env: HashMap<String, String>,       // Environment variables
    pub dependencies: HashMap<String, String>,    // Cargo.lock versions
}
```

### SoftwareAttestation

Complete attestation with cryptographic proof:

```rust
pub struct SoftwareAttestation {
    pub git_info: GitInfo,                        // Repository state
    pub build_info: BuildInfo,                    // Build environment
    pub metadata: HashMap<String, String>,        // Custom metadata
    pub created_at: u64,                          // Attestation timestamp
    pub fingerprint: String,                      // Crypto fingerprint
}
```

## Security Model

### Hardware Attestation

- **YubiKey PIV**: Uses hardware-protected private keys for signing
- **HSM Support**: Integrates with enterprise hardware security modules
- **Key Isolation**: Signing keys never leave hardware security boundary

### Cryptographic Verification

- **Ed25519 Signatures**: Fast, secure digital signatures
- **SHA-256 Hashing**: Cryptographic fingerprints of attestation data
- **Envelope Encryption**: Attestations sealed using TrustEdge envelope system

### Threat Mitigation

- **Supply Chain Attacks**: Verifiable build provenance 
- **Binary Tampering**: Cryptographic integrity validation
- **Insider Threats**: Hardware-backed non-repudiation
- **Build Environment Compromise**: Complete environment fingerprinting

## Integration Examples

### Cargo Build Hook

```rust
// In build.rs
use trustedge_attestation::AttestationBuilder;

fn main() {
    if env::var("TRUSTEDGE_ATTEST").is_ok() {
        let attestation = AttestationBuilder::new()
            .with_git_info(GitInfo::from_current_repo().unwrap())
            .with_build_info().unwrap()
            .sign_with_backend(get_backend(), "build_key").unwrap();
            
        // Save attestation alongside binary
        fs::write("target/attestation.json", serde_json::to_string(&attestation)?)?;
    }
}
```

### Runtime Verification

```rust
// Verify software on startup
fn verify_self() -> Result<()> {
    let attestation_data = fs::read_to_string("attestation.json")?;
    let attestation: SoftwareAttestation = serde_json::from_str(&attestation_data)?;
    
    if !attestation.verify()? {
        return Err(anyhow!("Software attestation verification failed"));
    }
    
    println!("✔ Software attestation verified");
    Ok(())
}
```

## Future Roadmap

- **Reproducible Builds**: Deterministic build verification
- **SLSA Framework**: Compliance with SLSA provenance standards  
- **Container Attestation**: Docker/OCI image attestation
- **SBOM Integration**: Software Bill of Materials support
- **Post-Quantum**: Algorithm agility for future crypto standards

---

## License

Licensed under the Mozilla Public License 2.0 (MPL-2.0).
See [LICENSE](../LICENSE) for details.
