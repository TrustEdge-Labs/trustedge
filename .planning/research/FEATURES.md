<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->

# FEATURES.md — Crypto Library Organization Patterns

**Research Type**: Project Research — Features dimension for Rust workspace consolidation
**Milestone**: Subsequent — existing 10-crate workspace being consolidated
**Date**: 2026-02-09

## Executive Summary

Analysis of well-structured Rust crypto libraries (RustCrypto, ring, rustls, sodiumoxide, orion) reveals three tiers of organizational features for a consolidated crypto library:

1. **Table Stakes** (must-have): Unified error types, feature flags for optional dependencies, re-export facade, no_std support consideration, clear module hierarchy
2. **Differentiators** (nice-to-have): Backend plugin system, algorithm agility, compile-time feature selection, trait-based API surface
3. **Anti-Features** (deliberately avoid): Over-abstraction, kitchen sink approach, breaking stable APIs frequently

Current TrustEdge state: 10 crates with duplication in error handling (10+ distinct error types), inconsistent feature flag patterns, and unclear re-export boundaries between core/receipts/attestation/trst-*.

**Key Finding**: The Universal Backend system already implements the most complex differentiator. Consolidation should focus on table stakes (unified errors, consistent features) rather than adding new abstractions.

---

## Table Stakes Features

These are **mandatory** for a well-organized consolidated crypto library. Missing any of these will make the library harder to use and maintain.

### 1. Unified Error Type Hierarchy

**Complexity**: Medium
**Dependencies**: None (pure refactoring)
**Current State**: 10+ distinct error enums across crates

```
Current fragmentation:
- core/src/crypto.rs:       CryptoError
- core/src/chain.rs:        ChainError
- core/src/archive.rs:      ArchiveError
- core/src/asymmetric.rs:   AsymmetricError
- core/src/manifest.rs:     ManifestError
- core/src/hybrid.rs:       TrustEdgeError
- trst-core/src/manifest.rs: ManifestError (duplicate!)
- pubky/src/lib.rs:         PubkyAdapterError
- pubky-advanced/src:       PubkyError
```

**Best Practice** (from RustCrypto/ring):
- Single library-level error type with context variants
- Use `thiserror` for library code (already in core/Cargo.toml:75)
- `anyhow` only for CLI binaries
- Error variants grouped by subsystem (crypto, backend, transport, format)

**Recommended Structure**:
```rust
#[derive(Debug, thiserror::Error)]
pub enum TrustEdgeError {
    #[error("cryptographic operation failed: {0}")]
    Crypto(#[from] CryptoError),

    #[error("backend operation failed: {0}")]
    Backend(#[from] BackendError),

    #[error("transport error: {0}")]
    Transport(#[from] TransportError),

    #[error("archive format error: {0}")]
    Archive(#[from] ArchiveError),

    #[error("manifest validation failed: {0}")]
    Manifest(#[from] ManifestError),
}

// Subsystem errors with detailed context
#[derive(Debug, thiserror::Error)]
pub enum CryptoError {
    #[error("encryption failed: {reason}")]
    EncryptionFailed { reason: String },

    #[error("signature verification failed")]
    InvalidSignature,

    #[error("key derivation failed: {0}")]
    KeyDerivation(String),
}
```

**Migration Path**:
1. Create `errors.rs` module at root level
2. Define unified hierarchy with subsystem variants
3. Add `From<SubsystemError>` impls for backward compatibility
4. Deprecate old error types with compiler warnings
5. Update all `Result<T, OldError>` to `Result<T, TrustEdgeError>`

### 2. Consistent Feature Flag Architecture

**Complexity**: Low
**Dependencies**: None
**Current State**: Inconsistent patterns across crates

```
Current patterns:
- core/Cargo.toml:
  default = []
  audio = ["cpal"]
  yubikey = ["pkcs11", "dep:yubikey", "x509-cert", ...]

- attestation/Cargo.toml:
  envelope = ["trustedge-core"]  # Feature name doesn't match dependency

- wasm/Cargo.toml:
  default = ["console_error_panic_hook"]  # Has defaults (inconsistent)
```

**Best Practice** (from rustls/ring):
- Additive features only (never subtractive)
- Default feature = stable production config
- Hardware features opt-in (yubikey, tpm, secure-enclave)
- Platform features use cfg/target detection not features
- Feature names match domain concepts not dependency names

**Recommended Structure**:
```toml
[features]
# Default: Core crypto ops, no hardware, no network
default = ["std"]

# Foundation
std = []                # Standard library (vs no_std)
alloc = []              # Heap allocation without full std

# Hardware backends
yubikey = ["pkcs11", "dep:yubikey", "x509-cert", "der", "spki"]
tpm = ["tss-esapi"]     # Future: TPM 2.0 support
secure-enclave = []     # Future: macOS/iOS Secure Enclave

# I/O and capture
audio = ["cpal", "std"]
network = ["tokio", "std"]
quic = ["network", "quinn", "rustls"]

# Format support
archive = []            # .trst archive format
receipts = []           # Digital receipt chains
attestation = []        # Software attestation

# Platform integration
keyring = ["dep:keyring", "std"]
pubky = []              # Community: Pubky network adapter

# Development
test-vectors = []       # Include NIST/IETF test vectors in binary
```

**Additive Principle**: `cargo build` always works, features only add capabilities.

### 3. Re-Export Facade Pattern

**Complexity**: Low
**Dependencies**: None
**Current State**: lib.rs has 130+ pub use statements (good start!)

**Best Practice** (from tokio/async-std):
- Shallow re-exports at crate root for common types
- Preserve module structure for advanced users
- Document "prelude" pattern for glob imports
- Avoid re-exporting implementation details

**Current TrustEdge** (core/src/lib.rs:80-130):
```rust
// Good: Facade already exists
pub use archive::{archive_dir_name, read_archive, validate_archive, write_archive, ArchiveError};
pub use crypto::{decrypt, encrypt, generate_keypair, sign, verify};
pub use envelope::{Envelope, EnvelopeMetadata};

// Concern: Too many backend internals exposed
pub use backends::{
    BackendInfo, BackendOperation, BackendPreferences, BackendRegistry, /* ... 14 more items */
};
```

**Recommended Refinement**:
```rust
// Core types always re-exported
pub use crypto::{PublicKey, SecretKey, Signature};
pub use envelope::{Envelope, EnvelopeMetadata};
pub use errors::{TrustEdgeError, Result}; // Unified!

// Subsystem facades
pub mod archive {
    pub use crate::archive_impl::{read, write, validate, ArchiveError};
}

pub mod backends {
    // Simple API re-exported
    pub use crate::backends_impl::{Backend, BackendType};

    // Advanced API requires module path
    pub use crate::backends_impl::registry; // registry::register_backend()
}

// Prelude for common imports
pub mod prelude {
    pub use crate::{TrustEdgeError, Result};
    pub use crate::crypto::{PublicKey, SecretKey};
    pub use crate::envelope::Envelope;
}
```

**Usage**:
```rust
use trustedge::prelude::*;                  // Common case
use trustedge::backends::registry::*;        // Advanced case
```

### 4. Clear Module Hierarchy

**Complexity**: Medium
**Dependencies**: None (pure reorganization)
**Current State**: Flat module structure in core/src/

```
Current structure (18 top-level modules):
core/src/
├── archive.rs          # Archive I/O
├── asymmetric.rs       # RSA operations
├── audio.rs            # Audio capture
├── auth.rs             # Network auth
├── backends/           # Backend system (7 files)
├── chain.rs            # BLAKE3 chains
├── crypto.rs           # Core primitives
├── envelope.rs         # Envelope format
├── envelope_v2_bridge.rs  # Format compat
├── format.rs           # Format detection
├── hybrid.rs           # RSA hybrid (Pubky only)
├── manifest.rs         # Manifest types
├── transport/          # Network (tcp.rs, quic.rs)
└── vectors.rs          # Test vectors
```

**Best Practice** (from RustCrypto):
- Group by concern not algorithm
- Separate pure crypto from I/O/network
- Clear boundaries for optional features

**Recommended Hierarchy**:
```
trustedge-core/src/
├── lib.rs                      # Re-export facade
├── error.rs                    # Unified error types
│
├── crypto/                     # Pure cryptography (no I/O)
│   ├── mod.rs
│   ├── symmetric.rs            # AES-GCM, XChaCha20-Poly1305
│   ├── asymmetric.rs           # Ed25519, RSA (Pubky)
│   ├── hash.rs                 # BLAKE3, SHA-256
│   ├── kdf.rs                  # PBKDF2, HKDF
│   └── primitives.rs           # Low-level wrappers
│
├── formats/                    # Data formats
│   ├── mod.rs
│   ├── envelope.rs             # Core envelope v1
│   ├── envelope_v2.rs          # Pubky envelope v2
│   ├── manifest.rs             # Manifest types
│   ├── archive.rs              # .trst archive I/O
│   └── chain.rs                # BLAKE3 continuity chains
│
├── backends/                   # Backend system (unchanged structure)
│   ├── mod.rs
│   ├── traits.rs
│   ├── software_hsm.rs
│   ├── keyring.rs
│   ├── yubikey.rs              # feature = "yubikey"
│   ├── universal.rs
│   └── registry.rs
│
├── io/                         # I/O operations
│   ├── mod.rs
│   ├── audio.rs                # feature = "audio"
│   └── readers.rs              # InputReader trait
│
├── network/                    # Network operations (feature = "network")
│   ├── mod.rs
│   ├── auth.rs                 # Mutual auth protocol
│   ├── transport.rs            # Transport abstraction
│   ├── tcp.rs                  # TCP with framing
│   └── quic.rs                 # QUIC with TLS (feature = "quic")
│
├── integrations/               # External platform integrations
│   ├── mod.rs
│   ├── pubky.rs                # feature = "pubky"
│   └── receipts.rs             # Digital receipt chains
│
└── test_support/               # feature = "test-vectors"
    ├── mod.rs
    └── vectors.rs
```

**Migration Benefits**:
- Clear feature boundaries (crypto/ never needs features, io/ and network/ do)
- Easy to find related functionality
- Supports future no_std work (crypto/ and formats/ are no_std candidates)

### 5. Documentation Standards

**Complexity**: Low
**Dependencies**: None
**Current State**: Unknown (not examined in research)

**Best Practice** (from ring/rustls):
- Module-level docs explain purpose and security considerations
- Every public type has example usage
- Crate-level docs have "getting started" section
- Security-critical types have "Security Considerations" section

**Required Documentation**:
```rust
//! # trustedge-core
//!
//! Privacy-preserving cryptographic operations at the edge.
//!
//! ## Core Concepts
//!
//! - **Envelopes**: Ed25519-signed, AES-256-GCM encrypted chunks
//! - **Backends**: Pluggable key storage (software, keyring, YubiKey)
//! - **Chains**: BLAKE3-based continuity chains
//!
//! ## Quick Start
//!
//! ```rust
//! use trustedge::prelude::*;
//!
//! // Create envelope with default backend
//! let backend = SoftwareHsmBackend::new()?;
//! let envelope = Envelope::seal(data, &backend)?;
//! ```
//!
//! ## Security Considerations
//!
//! This library uses constant-time comparisons for all authentication tags
//! and signatures. Key material is zeroized on drop.

/// Envelope encryption with Ed25519 signatures.
///
/// # Security Considerations
///
/// - Uses AES-256-GCM with 96-bit random nonces (birthday bound: 2^32 messages)
/// - Ed25519 signatures provide ~128-bit security
/// - Chunks are independently encrypted (parallel decryption)
///
/// # Example
///
/// ```rust
/// # use trustedge::envelope::Envelope;
/// # use trustedge::backends::SoftwareHsmBackend;
/// let backend = SoftwareHsmBackend::new()?;
/// let envelope = Envelope::seal(b"secret data", &backend)?;
/// let plaintext = envelope.open(&backend)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub struct Envelope { /* ... */ }
```

---

## Differentiating Features

These are **nice-to-have** organizational features that elevate a library from "well-organized" to "excellent." They require more design work but provide significant DX benefits.

### 6. Backend Plugin System (Already Exists!)

**Complexity**: High (already implemented)
**Dependencies**: Trait system design
**Current State**: Universal Backend system in core/src/backends/

**Analysis**: TrustEdge already has the most sophisticated differentiator:

```rust
// core/src/backends/traits.rs
pub trait KeyBackend: Send + Sync {
    fn generate(&mut self, ctx: KeyContext) -> Result<KeyMetadata>;
    fn sign(&self, key_id: &str, data: &[u8]) -> Result<Vec<u8>>;
    fn verify(&self, key_id: &str, data: &[u8], sig: &[u8]) -> Result<bool>;
    // ... encryption, key management
}

// core/src/backends/universal.rs
pub trait UniversalBackend: Send + Sync {
    fn supports_operation(&self, op: &BackendOperation) -> bool;
    fn perform_operation(&self, key_id: &str, op: BackendOperation) -> Result<Vec<u8>>;
}
```

**Implementations**:
- `SoftwareHsmBackend` - Pure software, no dependencies
- `KeyringBackend` - OS keychain (macOS/Windows/Linux)
- `YubiKeyBackend` - PKCS#11 hardware (feature = "yubikey")
- `UniversalKeyringBackend` - Capability-based dispatch

**Best Practice Alignment**: This matches ring's "provider" pattern and rustls's "crypto provider" system.

**Consolidation Impact**: PRESERVE THIS. It's the most valuable organizational pattern. Don't simplify or merge backends during consolidation.

### 7. Algorithm Agility (Partial)

**Complexity**: High
**Dependencies**: Backend system
**Current State**: Hard-coded algorithms with backend abstraction

**Current Algorithms**:
- Symmetric: AES-256-GCM (primary), XChaCha20-Poly1305 (hybrid.rs)
- Signing: Ed25519 (core), RSA (Pubky integration only)
- Hashing: BLAKE3 (chains), SHA-256 (limited use)
- KDF: PBKDF2 (workspace dependency)

**Algorithm Agility Spectrum**:
```
None                Partial              Full
|-------------------|-------------------|
ring                TrustEdge           RustCrypto/AWS-LC
(Ed25519 only)      (Ed25519 + backend  (every NIST algorithm)
                     can add RSA)
```

**Best Practice** (from rustls CryptoProvider):
- Define algorithm identifier enums
- Backend advertises supported algorithms
- Client selects from intersection
- Format stores algorithm identifier

**Recommended Implementation**:
```rust
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[non_exhaustive]
pub enum SignatureAlgorithm {
    Ed25519,
    #[cfg(feature = "rsa")]
    RsaPss2048,
    #[cfg(feature = "rsa")]
    RsaPss4096,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[non_exhaustive]
pub enum EncryptionAlgorithm {
    Aes256Gcm,
    XChaCha20Poly1305,
}

pub trait AlgorithmProvider {
    fn supported_signature_algorithms(&self) -> &[SignatureAlgorithm];
    fn supported_encryption_algorithms(&self) -> &[EncryptionAlgorithm];
}

// Envelope metadata stores actual algorithm used
#[derive(Serialize, Deserialize)]
pub struct EnvelopeMetadata {
    pub version: u32,
    pub signature_algorithm: SignatureAlgorithm,
    pub encryption_algorithm: EncryptionAlgorithm,
    // ...
}
```

**Consolidation Priority**: LOW. Current hard-coded algorithms work fine. Add this only if:
- New backend needs different algorithms (e.g., TPM only supports RSA)
- Format versioning requires algorithm migration
- Regulatory requirements mandate algorithm choice

### 8. Compile-Time Feature Selection

**Complexity**: Medium
**Dependencies**: Feature flags (table stakes #2)
**Current State**: Runtime checks with #[cfg(feature)]

**Current Pattern**:
```rust
// core/src/lib.rs:85
#[cfg(feature = "audio")]
pub use audio::AudioCapture;

// core/src/backends/mod.rs:37
pub use yubikey::{YubiKeyBackend, YubiKeyConfig};
// This is always compiled, fails at runtime if feature disabled
```

**Best Practice** (from tokio):
- Feature-gated code doesn't compile when feature disabled
- Reduces binary size and attack surface
- Clear compile errors vs runtime errors

**Recommended Pattern**:
```rust
// Correct: Type doesn't exist without feature
#[cfg(feature = "yubikey")]
pub use yubikey::{YubiKeyBackend, YubiKeyConfig};

// Correct: Stub implementation for disabled features
#[cfg(not(feature = "network"))]
pub mod network {
    pub struct Transport;

    impl Transport {
        pub fn connect() -> Result<Self, &'static str> {
            Err("network feature not enabled")
        }
    }
}
```

**Consolidation Task**: Audit all `pub use` statements and backend registrations. Ensure:
- `#[cfg(feature = "X")]` on type definitions, not just modules
- Backend registry only includes enabled backends
- Examples and tests use proper feature gates

### 9. Trait-Based API Surface

**Complexity**: Medium
**Dependencies**: None
**Current State**: Mixed (backends use traits, formats use structs)

**Current API**:
```rust
// Good: Trait-based
pub trait KeyBackend: Send + Sync { /* ... */ }

// Concrete: Struct-based
pub struct Envelope { /* ... */ }
impl Envelope {
    pub fn seal(data: &[u8], backend: &dyn KeyBackend) -> Result<Self>;
    pub fn open(&self, backend: &dyn KeyBackend) -> Result<Vec<u8>>;
}
```

**Best Practice** (from async-trait ecosystem):
- Traits for extensibility (backends, transports)
- Structs for concrete types (envelopes, manifests)
- Builder pattern for complex configuration

**TrustEdge Status**: ALREADY CORRECT. No changes needed.

**Anti-Pattern to Avoid**:
```rust
// Bad: Over-abstraction
pub trait Encryptor {
    fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>>;
}

pub trait Signer {
    fn sign(&self, data: &[u8]) -> Result<Vec<u8>>;
}

pub trait Envelope: Encryptor + Signer { /* ... */ }
```

Why bad: Users just want `Envelope::seal()`, not to implement crypto protocols.

### 10. Prelude Module Pattern

**Complexity**: Low
**Dependencies**: Re-export facade (table stakes #3)
**Current State**: Not present

**Best Practice** (from std::prelude, tokio::prelude):
- Single glob import for 90% use cases
- Only imports types, not functions
- Stable across minor versions

**Recommended Addition**:
```rust
// core/src/prelude.rs
pub use crate::error::{Result, TrustEdgeError};
pub use crate::crypto::{PublicKey, SecretKey, Signature};
pub use crate::formats::{Envelope, EnvelopeMetadata};
pub use crate::backends::{Backend, BackendType};

// core/src/lib.rs
pub mod prelude;

// User code
use trustedge::prelude::*;
```

**Consolidation Task**: LOW priority, add after error unification complete.

---

## Anti-Features

These are organizational patterns to **deliberately avoid** during consolidation. They appear in poorly-maintained or over-engineered crypto libraries.

### 11. Over-Abstraction (Avoid)

**Example** (anti-pattern from generic crypto libraries):
```rust
// Bad: Unnecessary abstraction layers
pub trait CipherMode { /* ... */ }
pub trait BlockCipher: CipherMode { /* ... */ }
pub trait AeadCipher: BlockCipher { /* ... */ }
pub struct Aes256Gcm<M: CipherMode> { /* ... */ }

// Good: Direct implementation
pub struct Aes256Gcm { /* ... */ }
impl Aes256Gcm {
    pub fn encrypt(&self, nonce: &[u8], plaintext: &[u8]) -> Result<Vec<u8>>;
}
```

**Why Avoid**:
- Crypto libraries need performance and audit-ability, not extensibility
- Generic code harder to review for security
- Users don't need to "bring your own block cipher"

**TrustEdge Status**: Current crypto.rs is correctly concrete. Don't generify during consolidation.

### 12. Kitchen Sink Approach (Avoid)

**Example** (anti-pattern):
```rust
// Bad: Supporting every possible configuration
pub enum HashAlgorithm {
    Sha1, Sha256, Sha384, Sha512, Sha3_256, Sha3_512,
    Blake2b, Blake2s, Blake3, Md5, Ripemd160, /* ... */
}

// Good: Support what you actually use
pub enum HashAlgorithm {
    Blake3,    // Primary: chains, general hashing
    Sha256,    // Secondary: compatibility (X.509, etc.)
}
```

**Why Avoid**:
- Each algorithm increases audit surface
- Maintenance burden for unused code
- Users expect "supported" = "recommended"

**TrustEdge Consolidation Rule**: Only consolidate code that's actually used. If pubky-advanced is the only user of RSA hybrid crypto, consider leaving it separate or marking experimental.

### 13. Frequent Breaking Changes (Avoid)

**Example** (anti-pattern from immature libraries):
```
v0.1.0: Envelope v1 format
v0.2.0: BREAKING: Envelope v2 format (incompatible)
v0.3.0: BREAKING: Renamed Envelope to Container
v0.4.0: BREAKING: Different signature algorithm
```

**Why Avoid**:
- Crypto libraries need stability (stored data must decrypt)
- Breaking changes force users to maintain multiple versions
- Format versioning should be internal, not tied to crate version

**TrustEdge Strategy**:
```rust
// Good: Format versioning is internal
pub struct Envelope {
    version: u32,  // Stored in envelope, not crate version
    // ...
}

impl Envelope {
    pub fn open(&self, backend: &dyn KeyBackend) -> Result<Vec<u8>> {
        match self.version {
            1 => self.open_v1(backend),
            2 => self.open_v2(backend),
            _ => Err(TrustEdgeError::UnsupportedVersion(self.version)),
        }
    }
}
```

**Consolidation Rule**: Preserve all existing format versions. Users with existing .trst archives must be able to decrypt after consolidation.

### 14. Platform-Specific Code Without Clear Boundaries (Avoid)

**Example** (anti-pattern):
```rust
// Bad: Platform checks scattered throughout
pub fn get_key() -> Result<Vec<u8>> {
    #[cfg(target_os = "macos")]
    return macos_keychain::get_key();

    #[cfg(target_os = "windows")]
    return windows_dpapi::get_key();

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    return Err("unsupported platform");
}
```

**Why Avoid**:
- Hard to test all platforms
- Unclear what "supported" means
- Platform code pollutes high-level modules

**Good Pattern** (already in TrustEdge):
```rust
// Good: Platform-specific backends behind trait
pub trait KeyBackend { /* ... */ }

#[cfg(target_os = "macos")]
pub struct KeychainBackend { /* macOS-specific impl */ }

#[cfg(target_os = "windows")]
pub struct DpapiBackend { /* Windows-specific impl */ }

pub struct SoftwareHsmBackend { /* works everywhere */ }
```

**TrustEdge Status**: CORRECT. Backend system already isolates platform code.

### 15. No_std Half-Measures (Avoid for Now)

**Example** (anti-pattern from premature no_std):
```rust
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

#[cfg(feature = "std")]
use std::vec::Vec;

// Then half the crate doesn't compile without std anyway
```

**Why Avoid**:
- no_std requires significant design (no heap, no I/O, no system calls)
- Half-working no_std worse than honest "requires std"
- YubiKey, audio, network all require std

**TrustEdge Current Dependencies**:
```
std-requiring:
- audio (cpal)
- network (tokio)
- yubikey (pkcs11)
- keyring (OS keychains)
- archive I/O (std::fs)

no_std-capable:
- crypto primitives (aes-gcm, ed25519-dalek, blake3)
- manifest types (serde with alloc)
```

**Consolidation Strategy**:
- Phase 1: Assume `std` everywhere (current state)
- Phase 2 (future): Extract `trustedge-crypto-core` (no_std, crypto/ module only)
- Phase 3 (future): `trustedge-core` re-exports crypto-core with std features

**Current Rule**: Don't add `#[cfg(not(feature = "std"))]` during consolidation. Address in separate milestone after consolidation complete.

---

## Feature Dependencies

Visual map of how organizational features depend on each other:

```
Foundation Layer:
├── [2] Feature Flags ────────────┐
├── [1] Unified Errors            │
└── [4] Module Hierarchy          │
                                  │
Table Stakes Layer:              │
├── [3] Re-Export Facade ─────────┤
│   depends: [1], [4]             │
└── [5] Documentation ────────────┤
    depends: [1], [3], [4]        │
                                  │
Differentiators Layer:           │
├── [6] Backend Plugins ──────────┘ (already exists)
│   depends: [2]
├── [7] Algorithm Agility
│   depends: [6]
├── [8] Compile-Time Selection
│   depends: [2]
├── [9] Trait-Based API
│   depends: [6]
└── [10] Prelude Module
    depends: [3]
```

**Critical Path for Consolidation**:
1. Unified errors [1] - Blocks everything
2. Module hierarchy [4] - Reorganize before adding features
3. Feature flags [2] - Required for optional dependencies
4. Re-export facade [3] - Last step before 1.0

**Low Priority**:
- Algorithm agility [7] - Not needed unless new backend requires it
- Prelude [10] - Nice-to-have after stabilization

---

## Complexity Assessment

| Feature | Complexity | Effort (days) | Risk |
|---------|------------|---------------|------|
| **Table Stakes** |
| [1] Unified errors | Medium | 3-5 | Low (pure refactoring) |
| [2] Feature flags | Low | 1-2 | Low (already mostly correct) |
| [3] Re-export facade | Low | 1-2 | Low (already 80% done) |
| [4] Module hierarchy | Medium | 3-5 | Medium (file moves, import updates) |
| [5] Documentation | Low | 2-3 | Low (gradual) |
| **Differentiators** |
| [6] Backend plugins | High | 0 | None (already exists) |
| [7] Algorithm agility | High | 5-8 | Medium (format changes) |
| [8] Compile-time selection | Medium | 2-3 | Low (build system) |
| [9] Trait-based API | Medium | 0 | None (already correct) |
| [10] Prelude module | Low | 0.5 | Low (pure addition) |

**Total Table Stakes**: 10-17 days
**Optional Differentiators**: 7-11 days

---

## Recommendations for Consolidation

### Phase 1: Foundation (Week 1-2)

**Must Do**:
1. Create unified error hierarchy (errors.rs)
2. Reorganize into crypto/, formats/, backends/, io/, network/ structure
3. Audit feature flags for consistency
4. Update all Result<T, E> to use unified error

**Success Criteria**:
- Single `use trustedge::TrustEdgeError` imports work
- Clear module boundaries (crypto/ has no I/O)
- All tests pass with identical behavior

### Phase 2: API Surface (Week 3)

**Must Do**:
1. Refine re-export facade in lib.rs
2. Add module-level documentation
3. Audit public API surface (what should be pub?)

**Success Criteria**:
- `use trustedge::prelude::*` covers 90% of use cases
- Every public type has docs + example
- Internal types not exposed (backends::universal_registry details hidden)

### Phase 3: Optional (Week 4+)

**Consider**:
1. Compile-time feature selection audit
2. Prelude module addition
3. Algorithm agility (only if needed)

**Success Criteria**:
- cargo build --no-default-features works for core crypto
- Feature flags are additive (no subtractive features)

### What NOT to Do

**Avoid During Consolidation**:
- Don't add no_std support (separate milestone)
- Don't implement algorithm agility unless blocking
- Don't over-abstract crypto primitives
- Don't break existing .trst archive compatibility
- Don't change envelope format versions

---

## Upstream Research Sources

This analysis synthesized patterns from:

**Exemplary Rust Crypto Libraries**:
- **RustCrypto** (github.com/RustCrypto): Workspace of 50+ crates, consistent feature flags
- **ring** (github.com/briansmith/ring): Minimal API surface, hard-coded algorithms
- **rustls** (github.com/rustls/rustls): CryptoProvider trait, clear feature boundaries
- **sodiumoxide** (github.com/sodiumoxide/sodiumoxide): Simple facade over libsodium
- **orion** (github.com/orion-rs/orion): Pure Rust, clear module hierarchy

**Organizational Patterns**:
- **tokio** (tokio.rs): Feature flag mastery, prelude pattern
- **serde** (serde.rs): Re-export facade, minimal core
- **clap** (github.com/clap-rs/clap): Builder pattern, feature-gated backends

**Anti-Patterns Observed**:
- Generic crypto libraries (too abstract, unmaintained)
- Immature libraries with frequent breaking changes
- Platform-specific code without trait boundaries

---

## Conclusion

**Key Findings**:

1. **TrustEdge is 70% there**: Universal Backend system is the hardest differentiator, already implemented correctly
2. **Biggest gaps**: Error types (10+ separate enums), module organization (18 flat modules), documentation
3. **Quick wins**: Unified errors (3-5 days), module reorganization (3-5 days), feature flag audit (1-2 days)
4. **Don't over-engineer**: Avoid algorithm agility, no_std, and over-abstraction during consolidation

**Consolidation Priority**:
```
Critical:   [1] Unified errors, [4] Module hierarchy
High:       [2] Feature flags, [3] Re-export facade
Medium:     [5] Documentation, [8] Compile-time selection
Low:        [10] Prelude module
Defer:      [7] Algorithm agility, no_std support
```

**Success Metric**: After consolidation, new users should be able to:
- Start with `use trustedge::prelude::*`
- Understand all errors from `TrustEdgeError` docs
- Find functionality by module name (crypto/envelope/backends)
- Build without features for fast CI (no audio/yubikey)

This document feeds into requirements definition phase.
