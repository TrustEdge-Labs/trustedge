<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
-->

# TrustEdge Architecture Research: Monolithic Consolidation

**Research Type:** Project Architecture — Rust workspace consolidation
**Focus:** Module organization patterns for monolith core + thin shells
**Date:** 2026-02-09

## Executive Summary

This document outlines the target architecture for consolidating TrustEdge's 10-crate workspace into a monolithic `trustedge-core` with thin CLI and WASM wrappers. Analysis of successful Rust crypto libraries (ring, RustCrypto, rustls) reveals consistent patterns for internal module organization, feature-gating, and data flow that directly apply to TrustEdge's consolidation.

**Key Finding:** Large Rust crypto libraries organize by **capability layers** (primitives → protocols → applications) rather than by product features. TrustEdge's current structure already follows this pattern in `core/`, making consolidation a matter of extending the existing hierarchy rather than restructuring.

## Current State Analysis

### Existing 10-Crate Workspace

```
trustedge (workspace)
├── crates/core (31 modules)           # Cryptographic primitives + transport
│   ├── backends/                      # Universal Backend system (8 files)
│   ├── transport/                     # Network abstraction (TCP/QUIC)
│   ├── envelope.rs                    # Core envelope format (Ed25519 + AES-256-GCM)
│   ├── crypto.rs                      # XChaCha20-Poly1305 primitives
│   ├── chain.rs                       # BLAKE3 continuity chain
│   ├── manifest.rs                    # Canonical JSON serialization (460 LOC)
│   ├── auth.rs                        # Ed25519 mutual auth
│   ├── hybrid.rs                      # RSA hybrid encryption (Pubky integration)
│   └── envelope_v2_bridge.rs          # Format detection (Pubky integration)
│
├── crates/receipts (2 files)          # Digital receipt system (1,281 LOC)
│   └── lib.rs                         # Depends on: trustedge-core::Envelope
│
├── crates/attestation (2 files)       # Software attestation (826 LOC)
│   └── lib.rs                         # Optional dependency on trustedge-core
│
├── crates/trst-core (2 files)         # Archive manifest types (449 LOC)
│   ├── lib.rs                         # WASM-compatible, minimal deps
│   └── manifest.rs                    # cam.video profile types
│
├── crates/pubky (3 files)             # Simple Pubky adapter
│   └── lib.rs                         # Depends on: trustedge-core, pubky crate
│
├── crates/pubky-advanced (4 files)    # Hybrid encryption for Pubky
│   ├── envelope.rs                    # EnvelopeV2 format (X25519 ECDH)
│   ├── keys.rs                        # DualKeyPair (Ed25519 + X25519)
│   └── pubky_client.rs                # Network client
│
├── crates/trustedge-cli               # Main CLI binary (thin wrapper)
├── crates/trst-cli                    # Archive CLI (wrap/verify commands)
├── crates/wasm                        # Browser bindings for core
└── crates/trst-wasm                   # Browser verification for archives
```

**Dependencies Flow:**
```
pubky-advanced → core
pubky → core
receipts → core::Envelope
attestation → core (optional)
trst-core → (standalone, WASM-compatible)
core → (leaf, no internal deps)
```

**Total Lines of Code to Merge:**
- Core: ~31 modules (baseline)
- Receipts: 1,281 LOC
- Attestation: 826 LOC
- trst-core manifest: 449 LOC
- Pubky integration: ~800 LOC (envelope + keys + client)
- **Total addition:** ~3,356 LOC to integrate

## Patterns from Large Rust Crypto Libraries

### 1. Ring (Mozilla/Briansmith)

**Structure:** Monolithic crate with internal C/assembly implementations.

```
ring/
├── src/
│   ├── aead/              # AEAD algorithms (AES-GCM, ChaCha20-Poly1305)
│   ├── agreement/         # Key agreement (ECDH)
│   ├── digest/            # Hash functions (SHA-2, SHA-3, BLAKE3)
│   ├── ec/                # Elliptic curve operations
│   ├── hmac/              # HMAC construction
│   ├── rand/              # Secure random number generation
│   ├── signature/         # Digital signatures (ECDSA, Ed25519, RSA)
│   └── test/              # Cross-cutting test utilities
```

**Key Patterns:**
- **Zero feature flags:** All algorithms compiled by default, tree-shaking at link time
- **Internal modules are private:** Public API surface through `lib.rs` re-exports
- **Platform-specific via cfg:** Use `#[cfg(target_arch)]` not features
- **No `std` dependency:** Works in `no_std` environments

**Lesson for TrustEdge:** Ring proves monolithic structure doesn't harm compile times when organized by capability layers. TrustEdge can follow suit with backends/, protocols/, applications/ layers.

### 2. RustCrypto (Multi-Crate Ecosystem, Individual Crates are Monolithic)

**Example: `aes-gcm` crate structure:**
```
aes-gcm/
├── src/
│   ├── lib.rs             # Public API
│   ├── aes128gcm.rs       # 128-bit variant
│   ├── aes256gcm.rs       # 256-bit variant
│   └── tests/             # Test vectors
```

**Feature Strategy:**
```toml
[features]
default = ["std"]
std = []
alloc = []
heapless = []
```

**Key Patterns:**
- **Minimal default features:** `std` is typically the only default
- **Additive features:** Each feature adds capability without removing others
- **No algorithm-specific features:** All algorithms in a crate are always available
- **Separate crates for large optional dependencies:** Hardware support (AES-NI) goes in separate crates

**Lesson for TrustEdge:** Use features for **platform/hardware integration** (yubikey, audio), not for algorithms. Keep receipts, attestation, archives always compiled.

### 3. Rustls (TLS Implementation)

**Structure:** Protocol-oriented organization.

```
rustls/
├── src/
│   ├── msgs/              # TLS message parsing/serialization
│   ├── client/            # Client-side protocol state machine
│   ├── server/            # Server-side protocol state machine
│   ├── sign/              # Signature operations
│   ├── verify/            # Certificate verification
│   ├── ticketer/          # Session ticket encryption
│   └── suites/            # Cipher suite definitions
```

**Feature Strategy:**
```toml
[features]
default = ["logging", "tls12"]
tls12 = []                 # Protocol version support
dangerous_configuration = [] # Explicitly opt-in to unsafe configs
```

**Key Patterns:**
- **Protocol layer separation:** Clear boundaries between message format, state machine, and crypto operations
- **Feature gates for risk:** Use features to opt-in to dangerous/experimental APIs
- **Logging as default feature:** Developer ergonomics favored in default build

**Lesson for TrustEdge:** Organize by protocol layer (primitives → envelopes → receipts/attestation → archives). Use features for experimental/community integrations like Pubky.

## Target Architecture: Consolidated trustedge-core

### Proposed Module Layout

```
trustedge-core/
├── src/
│   ├── lib.rs                         # Public API surface, re-exports
│   │
│   ├── primitives/                    # Layer 0: Cryptographic Primitives
│   │   ├── mod.rs
│   │   ├── aead.rs                    # AES-256-GCM, XChaCha20-Poly1305
│   │   ├── signatures.rs              # Ed25519 signing/verification
│   │   ├── hashing.rs                 # BLAKE3, SHA-2
│   │   ├── asymmetric.rs              # RSA, X25519 ECDH (feature-gated)
│   │   └── random.rs                  # Secure RNG operations
│   │
│   ├── backends/                      # Layer 1: Key Management Abstraction
│   │   ├── mod.rs                     # Universal Backend trait
│   │   ├── universal.rs               # Capability-based dispatch
│   │   ├── software_hsm.rs            # Software HSM implementation
│   │   ├── keyring.rs                 # OS keyring integration
│   │   ├── yubikey.rs                 # YubiKey PIV support (feature: yubikey)
│   │   ├── traits.rs                  # Backend capability traits
│   │   ├── env.rs                     # Environment variable backend
│   │   └── file.rs                    # File-based key storage
│   │
│   ├── protocols/                     # Layer 2: Cryptographic Protocols
│   │   ├── mod.rs
│   │   ├── envelope.rs                # Core envelope format (Ed25519 + AES)
│   │   ├── chain.rs                   # BLAKE3 continuity chain
│   │   ├── auth.rs                    # Ed25519 mutual authentication
│   │   ├── hybrid/                    # Hybrid encryption (RSA-based)
│   │   │   ├── mod.rs
│   │   │   └── rsa_hybrid.rs          # Current hybrid.rs (core API)
│   │   └── pubky/                     # Pubky integration (feature: pubky)
│   │       ├── mod.rs
│   │       ├── envelope_v2.rs         # X25519 ECDH envelope format
│   │       ├── dual_keypair.rs        # Ed25519 + X25519 key pairs
│   │       ├── client.rs              # Pubky network client
│   │       └── bridge.rs              # Format detection bridge
│   │
│   ├── applications/                  # Layer 3: Application Protocols
│   │   ├── mod.rs
│   │   ├── receipts/                  # Digital receipt system
│   │   │   ├── mod.rs
│   │   │   ├── receipt.rs             # Receipt struct + logic (from receipts/lib.rs)
│   │   │   ├── chain.rs               # Receipt chain verification
│   │   │   └── operations.rs          # create_receipt, assign_receipt
│   │   ├── attestation/               # Software attestation
│   │   │   ├── mod.rs
│   │   │   ├── attestation.rs         # Attestation struct (from attestation/lib.rs)
│   │   │   ├── signing.rs             # Attestation signing operations
│   │   │   └── verification.rs        # Attestation verification
│   │   └── archives/                  # .trst archive system
│   │       ├── mod.rs
│   │       ├── manifest/              # cam.video manifest types
│   │       │   ├── mod.rs
│   │       │   ├── types.rs           # CamVideoManifest (from trst-core/manifest.rs)
│   │       │   ├── validation.rs      # Manifest validation logic
│   │       │   └── serialization.rs   # Canonical JSON serialization
│   │       ├── archive.rs             # Archive read/write (current archive.rs)
│   │       ├── signature.rs           # Detached signature operations
│   │       └── verification.rs        # Archive verification logic
│   │
│   ├── transport/                     # Layer 4: Network Transport
│   │   ├── mod.rs
│   │   ├── tcp.rs                     # TCP with length framing
│   │   ├── quic.rs                    # QUIC with TLS
│   │   ├── factory.rs                 # Transport factory pattern
│   │   └── config.rs                  # Transport configuration
│   │
│   ├── io/                            # Layer 5: I/O Abstractions
│   │   ├── mod.rs
│   │   ├── reader.rs                  # InputReader trait
│   │   ├── audio.rs                   # Audio capture (feature: audio)
│   │   └── chunking.rs                # Chunk streaming logic
│   │
│   ├── format.rs                      # Cross-cutting: Data format definitions
│   ├── vectors.rs                     # Cross-cutting: Test vectors
│   └── utils/                         # Cross-cutting: Utilities
│       ├── mod.rs
│       ├── hex.rs                     # Hex encoding/decoding
│       └── zeroize_helpers.rs         # Memory zeroing utilities
│
├── benches/                           # Benchmarks (unchanged)
├── tests/                             # Integration tests (unchanged)
└── examples/                          # Usage examples (unchanged)
```

### Public API Surface (lib.rs structure)

```rust
// Layer 0: Re-export primitives (internal use, not public by default)
mod primitives;

// Layer 1: Backends (primary public API)
pub mod backends;
pub use backends::{
    Backend, SoftwareHSM, UniversalBackend, KeyringBackend,
    UniversalKeyring, BackendCapability, BackendOperation,
};

// Layer 2: Protocols
pub mod protocols;
pub use protocols::{
    Envelope, EnvelopeMetadata,
    ContinuityChain, ChainLink,
    AuthClient, AuthServer, AuthSession,
};

// Layer 3: Applications
pub mod applications;
pub use applications::{
    // Receipts
    Receipt, create_receipt, assign_receipt, verify_receipt_chain,
    // Attestation
    Attestation, AttestationConfig, create_signed_attestation, verify_attestation,
    // Archives
    CamVideoManifest, write_archive, read_archive, validate_archive,
    DeviceInfo, CaptureInfo, ChunkInfo, SegmentInfo,
};

// Layer 4: Transport
pub mod transport;
pub use transport::{Transport, TransportConfig, TransportFactory};

// Layer 5: I/O
pub mod io;
pub use io::InputReader;

#[cfg(feature = "audio")]
pub use io::{AudioCapture, AudioConfig, AudioChunk};

// Layer 2: Pubky (feature-gated)
#[cfg(feature = "pubky")]
pub mod pubky;

#[cfg(feature = "pubky")]
pub use pubky::{
    EnvelopeV2, DualKeyPair, PubkyClient, PubkyIdentity,
    detect_envelope_format, EnvelopeFormat,
};

// Cross-cutting
pub mod format;
pub use format::{AeadAlgorithm, SignatureAlgorithm};
```

## Feature Flag Strategy

### Proposed Feature Set

```toml
[features]
# Default: Fast CI, maximum portability
default = []

# Hardware integration
yubikey = [
    "dep:pkcs11",
    "dep:yubikey",
    "dep:x509-cert",
    "dep:der",
    "dep:spki",
    "dep:signature"
]

# Audio capture
audio = ["dep:cpal"]

# Pubky network integration (community/experimental)
pubky = ["dep:pubky", "dep:x25519-dalek", "dep:reqwest"]

# WASM compatibility (no yubikey, no audio)
wasm = ["getrandom/js"]

# Development/testing features
test-vectors = []           # Include test vector data
dangerous-apis = []         # Unsafe/experimental APIs

# Future hardware backends
tpm = []                    # TPM 2.0 support (not yet implemented)
secure-enclave = []         # Apple Secure Enclave (not yet implemented)
```

### Feature Rationale

| Feature | Purpose | Dependencies | Default? |
|---------|---------|--------------|----------|
| (none) | Core crypto, receipts, attestation, archives | aes-gcm, ed25519-dalek, blake3 | Yes |
| `yubikey` | Hardware key support | pkcs11, yubikey, x509-cert | No |
| `audio` | Live microphone capture | cpal (ALSA/CoreAudio/WASAPI) | No |
| `pubky` | Decentralized key discovery | pubky, x25519-dalek, reqwest | No |
| `wasm` | Browser compatibility | getrandom/js | No |
| `test-vectors` | Embedded test data | - | No |
| `dangerous-apis` | Experimental/unsafe APIs | - | No |

**Anti-Pattern to Avoid:** Algorithm-specific features like `aes-256` or `ed25519`. Ring and RustCrypto show that tree-shaking at link time is sufficient.

**Pattern to Follow:** Platform/integration features. Rustls uses `dangerous_configuration`, we use `dangerous-apis` for the same purpose.

## Data Flow After Consolidation

### 1. Basic Envelope Encryption (Unchanged)

```
Input File
    ↓
io::InputReader
    ↓
io::chunking (4KB chunks)
    ↓
primitives::aead::encrypt_chunk() (AES-256-GCM)
    ↓
protocols::envelope::Envelope (metadata + encrypted chunks)
    ↓
Output File / transport::Transport
```

### 2. Receipt Creation (Now Internal to Core)

```
User Call: applications::receipts::create_receipt()
    ↓
applications::receipts::Receipt::new_origin()
    ↓
serde_json::to_vec() (serialize receipt)
    ↓
protocols::envelope::Envelope::new() (wrap in envelope)
    ↓
backends::UniversalBackend::sign() (Ed25519 signature)
    ↓
Return: Envelope containing signed receipt
```

### 3. Archive Creation (.trst format)

```
Input: raw video frames
    ↓
applications::archives::write_archive()
    ↓
applications::archives::manifest::CamVideoManifest::new()
    ↓
For each chunk:
    primitives::hashing::blake3_hash() (chunk hash)
    protocols::chain::ContinuityChain::link() (chain hash)
    └→ SegmentInfo added to manifest
    ↓
applications::archives::manifest::serialize_canonical() (canonical JSON)
    ↓
backends::UniversalBackend::sign() (detached signature)
    ↓
Write: clip-<id>.trst/ directory with manifest.json + signatures/ + chunks/
```

### 4. Pubky Integration (Feature-Gated)

```
User Call: protocols::pubky::seal_for_pubky_user()
    ↓
protocols::pubky::PubkyClient::resolve_key() (lookup X25519 public key)
    ↓
protocols::pubky::DualKeyPair::derive_shared_secret() (X25519 ECDH)
    ↓
primitives::hashing::hkdf_derive() (derive AES key from shared secret)
    ↓
primitives::aead::encrypt_chunk() (AES-256-GCM)
    ↓
protocols::pubky::EnvelopeV2::new() (wrap with key exchange metadata)
    ↓
Return: EnvelopeV2 (magic: "TRS2")
```

## Suggested Merge Order

### Phase 1: Foundation (No Breaking Changes)

**Goal:** Move code without changing external APIs.

1. **Create new directory structure** (empty modules)
   ```bash
   mkdir -p crates/core/src/{primitives,protocols,applications,io,utils}
   mkdir -p crates/core/src/applications/{receipts,attestation,archives}
   mkdir -p crates/core/src/protocols/{hybrid,pubky}
   ```

2. **Move existing core modules into hierarchy** (refactoring internal to core)
   - `crypto.rs` → `primitives/aead.rs` + `primitives/signatures.rs`
   - `chain.rs` → `protocols/chain.rs`
   - `envelope.rs` → `protocols/envelope.rs`
   - `auth.rs` → `protocols/auth.rs`
   - `hybrid.rs` → `protocols/hybrid/rsa_hybrid.rs`
   - `envelope_v2_bridge.rs` → `protocols/pubky/bridge.rs`
   - `audio.rs` → `io/audio.rs`
   - `archive.rs` → `applications/archives/archive.rs`
   - `manifest.rs` (core) → `applications/archives/manifest/serialization.rs`
   - `backends/` → (unchanged, already hierarchical)
   - `transport/` → (unchanged, already hierarchical)

3. **Update `lib.rs` re-exports** to maintain backward compatibility
   ```rust
   // Old public API still works
   pub use protocols::envelope::{Envelope, EnvelopeMetadata};
   pub use protocols::chain::ContinuityChain;
   // ... etc
   ```

**Build order:** None (single crate, internal refactoring).

**Tests:** All existing core tests pass without modification.

### Phase 2: Integrate trst-core (Manifest Types)

**Goal:** Merge minimal-dependency manifest types for WASM compatibility.

4. **Copy trst-core manifest types** into `applications/archives/manifest/`
   - `trst-core/src/manifest.rs` → `applications/archives/manifest/types.rs`
   - Update imports: `use crate::applications::archives::manifest::*`

5. **Resolve duplicate manifest.rs** (core has one, trst-core has one)
   - Compare the two files (both 449-460 LOC)
   - If trst-core version is canonical cam.video types, keep that one
   - If core version has additional TrustEdge extensions, merge both
   - Likely outcome: Keep trst-core version (WASM-compatible types), move core version to `manifest/serialization.rs` (non-WASM utilities)

6. **Update trst-cli to import from core**
   ```rust
   // Old: use trustedge_trst_core::CamVideoManifest;
   // New: use trustedge_core::applications::archives::CamVideoManifest;
   ```

7. **Update trst-wasm to import from core**
   ```rust
   // Old: use trustedge_trst_core::manifest::*;
   // New: use trustedge_core::applications::archives::manifest::*;
   ```

**Build order:**
```
trustedge-core (with manifest types)
    ↓
trst-cli (update imports)
    ↓
trst-wasm (update imports)
```

**Tests:**
- `cargo test -p trustedge-core --lib` (manifest type tests now in core)
- `cargo test -p trustedge-trst-cli` (acceptance tests with new imports)
- `cargo test -p trustedge-trst-wasm` (browser tests with new imports)

**Deprecation:** Mark `trustedge-trst-core` as deprecated, keep for one release cycle for external users.

### Phase 3: Integrate Receipts

**Goal:** Merge receipt system into core applications layer.

8. **Copy receipts code** into `applications/receipts/`
   - `receipts/src/lib.rs` → Split into:
     - `applications/receipts/receipt.rs` (Receipt struct)
     - `applications/receipts/operations.rs` (create_receipt, assign_receipt)
     - `applications/receipts/chain.rs` (verify_receipt_chain)

9. **Update imports** (receipts already depends on `trustedge_core::Envelope`)
   ```rust
   // Old: use trustedge_core::Envelope;
   // New: use crate::protocols::envelope::Envelope; (internal import)
   ```

10. **Add re-exports to lib.rs**
    ```rust
    pub use applications::receipts::{
        Receipt, create_receipt, assign_receipt, verify_receipt_chain
    };
    ```

11. **Update receipt demo binary**
    - Move `receipts/src/bin/demo.rs` → `core/examples/receipts_demo.rs`

**Build order:** Single crate (core), no downstream consumers to update.

**Tests:**
- Move receipt tests from `receipts/src/lib.rs` (23 tests) into `core/tests/receipts_integration.rs`
- `cargo test -p trustedge-core receipts` (run receipt-specific tests)

**Deprecation:** Mark `trustedge-receipts` crate as deprecated.

### Phase 4: Integrate Attestation

**Goal:** Merge software attestation into core applications layer.

12. **Copy attestation code** into `applications/attestation/`
    - `attestation/src/lib.rs` → Split into:
      - `applications/attestation/attestation.rs` (Attestation struct)
      - `applications/attestation/signing.rs` (create_signed_attestation)
      - `applications/attestation/verification.rs` (verify_attestation)

13. **Handle optional envelope feature**
    - Attestation has `features = ["envelope"]` which makes `trustedge-core` optional
    - Now that attestation is **inside** core, this feature becomes unnecessary
    - Remove the feature, always enable envelope integration

14. **Update attestation binaries**
    - Move `attestation/src/bin/attest.rs` → `core/examples/attest.rs`
    - Move `attestation/src/bin/verify.rs` → `core/examples/verify_attestation.rs`
    - Or keep as binaries in `core/src/bin/trustedge-attest.rs` and `trustedge-verify.rs`

15. **Add re-exports to lib.rs**
    ```rust
    pub use applications::attestation::{
        Attestation, AttestationConfig, AttestationResult,
        create_signed_attestation, verify_attestation,
        VerificationConfig, VerificationResult,
    };
    ```

**Build order:** Single crate (core).

**Tests:**
- Move attestation tests into `core/tests/attestation_integration.rs`
- `cargo test -p trustedge-core attestation`

**Deprecation:** Mark `trustedge-attestation` crate as deprecated.

### Phase 5: Integrate Pubky (Optional)

**Goal:** Merge Pubky network integration as feature-gated module.

16. **Copy pubky-advanced code** into `protocols/pubky/`
    - `pubky-advanced/src/envelope.rs` → `protocols/pubky/envelope_v2.rs`
    - `pubky-advanced/src/keys.rs` → `protocols/pubky/dual_keypair.rs`
    - `pubky-advanced/src/pubky_client.rs` → `protocols/pubky/client.rs`

17. **Copy pubky adapter code** (simpler adapter for basic Pubky)
    - `pubky/src/lib.rs` → `protocols/pubky/adapter.rs` (simple key publishing)
    - `pubky/src/mock.rs` → `protocols/pubky/mock.rs` (testing utilities)

18. **Add pubky feature flag**
    ```toml
    [features]
    pubky = ["dep:pubky", "dep:x25519-dalek", "dep:reqwest"]
    ```

19. **Feature-gate the module**
    ```rust
    // In lib.rs
    #[cfg(feature = "pubky")]
    pub mod pubky;

    #[cfg(feature = "pubky")]
    pub use pubky::{EnvelopeV2, DualKeyPair, PubkyClient};
    ```

20. **Update demos**
    - Move Pubky examples into `core/examples/pubky_*.rs`
    - Mark with `#[cfg(feature = "pubky")]`

**Build order:** Single crate, optional feature.

**Tests:**
- Move Pubky tests into `core/tests/pubky_integration.rs`
- `cargo test -p trustedge-core --features pubky`

**Deprecation:** Mark `trustedge-pubky` and `trustedge-pubky-advanced` as deprecated with notice:
```
⚠ This crate is deprecated. Use `trustedge-core` with feature `pubky` instead:
  [dependencies]
  trustedge-core = { version = "0.3", features = ["pubky"] }
```

### Phase 6: Update Thin Shells

**Goal:** Update CLI and WASM wrappers to import from consolidated core.

21. **Update trustedge-cli**
    - Already imports from `trustedge-core`
    - Update to new public API paths (if any changed)
    - Test: `cargo build -p trustedge-cli`

22. **Update wasm crate**
    - Already imports from `trustedge-core`
    - Update to new public API paths
    - Test: `wasm-pack build crates/wasm`

23. **trst-cli and trst-wasm**
    - Already updated in Phase 2 (manifest types)
    - Retest: `cargo test -p trustedge-trst-cli`

**Build order:**
```
trustedge-core (consolidated)
    ↓
├─→ trustedge-cli (thin wrapper)
├─→ wasm (thin wrapper)
├─→ trst-cli (thin wrapper)
└─→ trst-wasm (thin wrapper)
```

## Build Order Implications

### Compilation Parallelism

**Before Consolidation (10 crates):**
```
Parallel build graph:
    core (no deps)
    trst-core (no deps)
        ↓
    ┌─────────┬─────────────┬─────────────┬────────┐
    receipts  attestation   pubky         pubky-adv
    ↓         ↓             ↓             ↓         trst-cli
    ┌─────────┴─────────────┴─────────────┘         ↓
    trustedge-cli                                    trst-wasm
    wasm
```
**Max parallelism:** 2-3 crates at a time (core + trst-core in parallel, then 4 crates in parallel).

**After Consolidation (5 crates):**
```
Parallel build graph:
    trustedge-core (monolith)
        ↓
    ┌───────┬──────────┬──────────┐
    cli     wasm       trst-cli   trst-wasm
```
**Max parallelism:** 4 crates at a time (all thin wrappers in parallel after core).

### Impact on CI Times

**Theoretical:** Consolidation reduces parallelism, but:
- Core compilation is incremental (modules compile in parallel)
- Thin wrappers are tiny (compile in <5 seconds)
- Network I/O for downloading deps dominates CI time

**Practical:** Based on Ring and Rustls experience:
- Monolithic core: ~60 seconds to compile from scratch
- 4 thin wrappers: ~20 seconds total (parallel)
- **Total CI time:** ~80 seconds (vs. ~90 seconds with 10 crates due to serial dependency chains)

**Recommendation:** Measure CI times before/after consolidation. Use `cargo build --timings` to profile.

## Edge Cases and Considerations

### 1. WASM Compatibility

**Challenge:** trst-core is designed to be WASM-compatible with minimal dependencies. Core has more dependencies (tokio, quinn).

**Solution:**
- Keep manifest types in `applications/archives/manifest/types.rs` with minimal deps
- Use `#[cfg(not(target_arch = "wasm32"))]` for non-WASM modules (transport, audio)
- trst-wasm only imports `trustedge_core::applications::archives::manifest` (subset import)

**Validation:**
```bash
cargo build -p trustedge-core --target wasm32-unknown-unknown --features wasm
```

### 2. Duplicate Manifest Types

**Current State:** Both `core/src/manifest.rs` (460 LOC) and `trst-core/src/manifest.rs` (449 LOC) exist.

**Resolution Strategy:**
1. Compare the two files for differences
2. If trst-core is canonical cam.video spec → Keep trst-core version
3. If core has additional TrustEdge extensions → Merge both:
   - `applications/archives/manifest/types.rs` (cam.video spec from trst-core)
   - `applications/archives/manifest/serialization.rs` (TrustEdge extensions from core)
4. If they're functionally identical → Use trst-core version (more recent, WASM-tested)

**Action Required:** Manual inspection of both files to determine merge strategy.

### 3. Pubky as Community Contribution

**Policy:** Pubky integration is experimental and not part of core product roadmap.

**Technical Implementation:**
- Keep behind `feature = "pubky"` flag (default disabled)
- Document as "community/experimental" in lib.rs
- No API stability guarantees for Pubky APIs
- Can remove in future major version if unmaintained

**Benefit:** Consolidation allows Pubky to leverage internal core APIs (primitives) without exposing them publicly.

### 4. Test Organization

**Current:** 150+ tests across 10 crates.

**After Consolidation:**
- Unit tests: Keep in respective modules (e.g., `applications/receipts/receipt.rs` has `#[cfg(test)] mod tests`)
- Integration tests: `core/tests/{receipts,attestation,pubky}_integration.rs`
- Hardware tests: `core/tests/yubikey_*.rs` (unchanged)
- Network tests: `core/tests/network_integration.rs` (unchanged)

**Test Execution:**
```bash
# All tests
cargo test -p trustedge-core --all-features

# Specific subsystem
cargo test -p trustedge-core --lib receipts
cargo test -p trustedge-core --lib attestation

# Feature-gated
cargo test -p trustedge-core --features yubikey
cargo test -p trustedge-core --features pubky
```

### 5. Backward Compatibility

**Deprecated Crates (Keep for 1 Release Cycle):**
- `trustedge-receipts` → Re-export from `trustedge-core::applications::receipts`
- `trustedge-attestation` → Re-export from `trustedge-core::applications::attestation`
- `trustedge-trst-core` → Re-export from `trustedge-core::applications::archives::manifest`
- `trustedge-pubky` → Re-export from `trustedge-core::pubky` (feature-gated)
- `trustedge-pubky-advanced` → Re-export from `trustedge-core::pubky`

**Implementation:**
```rust
// In trustedge-receipts/src/lib.rs (deprecated crate)
#[deprecated(
    since = "0.3.0",
    note = "Use trustedge-core with applications::receipts instead"
)]
pub use trustedge_core::applications::receipts::*;
```

**Migration Guide:** Provide in CHANGELOG.md with find/replace instructions.

## Component Boundaries (Quality Gate)

### Layer 0: Primitives
- **Boundary:** No dependencies on other layers, only external crypto crates
- **Imports:** `aes-gcm`, `ed25519-dalek`, `blake3`, `chacha20poly1305`
- **Exports:** Low-level crypto operations (encrypt, sign, hash)
- **Data Flow:** IN: plaintext/keys → OUT: ciphertext/signatures

### Layer 1: Backends
- **Boundary:** Depends on primitives, exports abstract key management
- **Imports:** `crate::primitives`, `keyring`, `pkcs11` (optional)
- **Exports:** `Backend` trait, capability-based dispatch
- **Data Flow:** IN: operation request → OUT: crypto result (via primitives)

### Layer 2: Protocols
- **Boundary:** Depends on primitives + backends, exports wire formats
- **Imports:** `crate::primitives`, `crate::backends`
- **Exports:** `Envelope`, `ContinuityChain`, `AuthSession`
- **Data Flow:** IN: plaintext + metadata → OUT: serialized encrypted envelope

### Layer 3: Applications
- **Boundary:** Depends on protocols, exports business logic
- **Imports:** `crate::protocols::envelope`, `crate::protocols::chain`
- **Exports:** `Receipt`, `Attestation`, `CamVideoManifest`
- **Data Flow:** IN: business data → OUT: signed envelopes or archives

### Layer 4: Transport
- **Boundary:** Depends on protocols, exports network I/O
- **Imports:** `crate::protocols::envelope`, `tokio`, `quinn`
- **Exports:** `Transport` trait, TCP/QUIC implementations
- **Data Flow:** IN: envelope → OUT: network bytes (bidirectional)

### Layer 5: I/O
- **Boundary:** No dependencies on other layers (utilities)
- **Imports:** `cpal` (optional), `tokio`
- **Exports:** `InputReader` trait, `AudioCapture`
- **Data Flow:** IN: OS/hardware → OUT: byte streams

### Cross-Cutting: Format
- **Boundary:** Shared types, no code logic
- **Imports:** None
- **Exports:** Enum definitions (`AeadAlgorithm`, `SignatureAlgorithm`)
- **Data Flow:** None (type definitions only)

## Data Flow Direction (Quality Gate)

```
External Input (user data, network packets, audio)
    ↓
Layer 5: I/O (InputReader, AudioCapture)
    ↓
Layer 3: Applications (Receipt, Attestation, Archive)
    ↓
Layer 2: Protocols (Envelope, Chain)
    ↓
Layer 1: Backends (Backend::sign, Backend::encrypt)
    ↓
Layer 0: Primitives (aead::encrypt, signatures::sign)
    ↓
Layer 2: Protocols (serialize envelope)
    ↓
Layer 4: Transport (send over network) OR
Layer 5: I/O (write to file)
    ↓
External Output (encrypted file, network bytes, .trst archive)
```

**Invariants:**
- Lower layers NEVER import from higher layers
- Primitives are leaf dependencies (no internal imports)
- Protocols use backends for key management (never direct primitives)
- Applications use protocols, never primitives directly

**Validation:**
```bash
# Check for upward dependencies (should be zero)
rg "use crate::applications" crates/core/src/protocols/
rg "use crate::protocols" crates/core/src/primitives/
rg "use crate::backends" crates/core/src/primitives/
```

## Build Order Implications (Quality Gate)

### Single-Crate Incremental Build

**Module Compilation Order (Determined by Compiler):**
```
rustc builds modules in dependency order:
    primitives/ (parallel: aead, signatures, hashing)
        ↓
    backends/ (parallel: software_hsm, keyring, yubikey*)
        ↓
    protocols/ (parallel: envelope, chain, auth)
        ↓
    applications/ (parallel: receipts, attestation, archives)
        ↓
    transport/ (parallel: tcp, quic)
        ↓
    io/ (parallel: reader, audio*)
        ↓
    lib.rs (final linking)

* = feature-gated, only built if feature enabled
```

**Incremental Rebuild (Change in receipts module):**
```
Touch: applications/receipts/receipt.rs
    ↓
Rebuild: applications/receipts/ (only affected files)
    ↓
Rebuild: lib.rs (re-export changes)
    ↓
Rebuild: trustedge-cli (depends on receipts API)

Unchanged: primitives, backends, protocols, transport, io
```

**Cold Build Time Estimate:**
- Primitives: ~10s (crypto operations)
- Backends: ~15s (YubiKey has complex deps)
- Protocols: ~10s
- Applications: ~15s (receipts + attestation + archives)
- Transport: ~8s (QUIC/TLS dependencies)
- I/O: ~5s
- **Total:** ~60s for full build (parallelized across CPU cores)

**Hot Build Time (Incremental):**
- Change in applications layer: ~5s (recompile affected module + lib.rs + CLI)
- Change in primitives layer: ~30s (most of tree needs rebuild)

**Comparison to Multi-Crate:**
- Multi-crate cold build: ~90s (serial dependency chains, 10 crate compilations)
- Multi-crate incremental: ~10s (only changed crate + dependents)
- **Tradeoff:** Monolith is faster for cold builds, slightly slower for isolated changes

**Mitigation:** Use `cargo check` during development (doesn't generate binaries, ~2x faster).

## References and Prior Art

### Rust Crypto Libraries Analyzed

1. **ring** (briansmith/ring)
   - Repository: https://github.com/briansmith/ring
   - Structure: Monolithic crate, ~30k LOC
   - Lesson: Zero feature flags, tree-shaking at link time

2. **RustCrypto** (https://github.com/RustCrypto)
   - Example: `aes-gcm` crate
   - Structure: Many small crates, each monolithic internally
   - Lesson: Features for platform integration, not algorithms

3. **rustls** (https://github.com/rustls/rustls)
   - Structure: Monolithic crate, ~20k LOC
   - Lesson: Protocol layer separation, features for dangerous APIs

4. **libsodium-sys / sodiumoxide**
   - Structure: Monolithic C library with Rust bindings
   - Lesson: All-in-one crypto toolkit with stable ABI

### TrustEdge-Specific Context

- Current: 10 crates, ~35k LOC total
- Target: 1 core crate (~38k LOC after consolidation), 4 thin wrappers (~500 LOC each)
- Core already has hierarchical structure (backends/, transport/)
- 150+ tests need to be preserved during consolidation

## Next Steps for Roadmap

This architecture research informs the following roadmap phases:

1. **Phase 1: Foundation** (Week 1)
   - Create directory structure
   - Move existing core modules into hierarchy
   - No external API changes

2. **Phase 2: Integrate trst-core** (Week 1)
   - Merge manifest types
   - Update trst-cli and trst-wasm imports
   - Deprecate trustedge-trst-core

3. **Phase 3: Integrate Receipts** (Week 2)
   - Merge receipts into applications/receipts/
   - Move tests
   - Deprecate trustedge-receipts

4. **Phase 4: Integrate Attestation** (Week 2)
   - Merge attestation into applications/attestation/
   - Remove envelope feature (always enabled)
   - Deprecate trustedge-attestation

5. **Phase 5: Integrate Pubky** (Week 3, Optional)
   - Merge pubky-advanced into protocols/pubky/
   - Add feature flag
   - Deprecate pubky crates

6. **Phase 6: Update Thin Shells** (Week 3)
   - Update CLI/WASM imports
   - Final testing

**Total Estimated Time:** 3 weeks for full consolidation.

## Conclusion

Consolidating TrustEdge from 10 crates to a monolithic core + thin shells follows established patterns from successful Rust crypto libraries. The proposed layer-based architecture (primitives → backends → protocols → applications → transport → I/O) provides clear boundaries and data flow direction. Feature flags should be used for platform/hardware integration (yubikey, audio, pubky) rather than algorithm selection. The suggested merge order (foundation → trst-core → receipts → attestation → pubky → thin shells) minimizes risk and allows incremental testing. Build times are expected to improve for cold builds while remaining acceptable for incremental development.

---

**Research completed:** 2026-02-09
**Researcher:** Claude (Sonnet 4.5)
**Status:** Ready for roadmap planning
