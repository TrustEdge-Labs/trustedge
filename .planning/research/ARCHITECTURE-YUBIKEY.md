# Architecture Research: YubiKey Backend Rewrite

**Domain:** Hardware security module integration (YubiKey PIV backend)
**Researched:** 2026-02-11
**Confidence:** HIGH

## Executive Summary

The YubiKey backend rewrite integrates into the existing Universal Backend architecture in `crates/core/src/backends/`. The architecture composes three distinct layers:

1. **Hardware Operations Layer** (`yubikey` crate) — Direct PIV applet communication
2. **Certificate Generation Layer** (`rcgen` crate) — X.509 certificate creation
3. **Universal Backend Integration Layer** — Trait implementation and error handling

This is NOT a monolithic backend like the current 3,263-line implementation. It's a clean separation of hardware ops, cert generation, and backend trait glue.

## Current Architecture Context

### Existing Backend System

```
crates/core/src/backends/
├── mod.rs                   # Exports, BackendRegistry
├── traits.rs                # KeyBackend trait (legacy)
├── universal.rs             # UniversalBackend trait (current)
├── software_hsm.rs          # 1,419 lines - reference implementation
├── keyring.rs               # 185 lines - minimal backend
└── yubikey.rs               # 3,263 lines - BEING DELETED
```

### Universal Backend Trait (The Integration Contract)

```rust
pub trait UniversalBackend: Send + Sync {
    fn perform_operation(&self, key_id: &str, operation: CryptoOperation)
        -> Result<CryptoResult, BackendError>;

    fn supports_operation(&self, operation: &CryptoOperation) -> bool;

    fn get_capabilities(&self) -> BackendCapabilities;

    fn backend_info(&self) -> BackendInfo;

    fn list_keys(&self) -> Result<Vec<KeyMetadata>, BackendError> {
        Ok(vec![]) // Optional
    }
}
```

**Key Operations for YubiKey:**
- `CryptoOperation::Sign { data, algorithm }` → Hardware signing via PIV
- `CryptoOperation::GetPublicKey` → Extract from PIV slot
- `CryptoOperation::GenerateKeyPair { algorithm }` → PIV key generation
- `CryptoOperation::Attest { challenge }` → YubiKey attestation

### Error Hierarchy (crates/core/src/error.rs)

```rust
pub enum TrustEdgeError {
    Backend(BackendError),   // Top-level variant
    // ... other variants
}

pub enum BackendError {
    UnsupportedOperation(String),
    KeyNotFound(String),
    InitializationFailed(String),
    HardwareError(String),      // YubiKey-specific errors go here
    OperationFailed(String),
}
```

**Fail-Closed Pattern:**
- Hardware errors → `BackendError::HardwareError`
- Missing keys → `BackendError::KeyNotFound`
- Unsupported ops → `BackendError::UnsupportedOperation`
- ALL errors bubble up through `perform_operation() -> Result<_, BackendError>`

## Recommended New Architecture

### Module Structure

```
crates/core/src/backends/yubikey/
├── mod.rs                   # Public API, YubiKeyBackend struct
├── hardware.rs              # PIV operations (yubikey crate wrapper)
├── certificates.rs          # rcgen integration for X.509 generation
├── operations.rs            # UniversalBackend implementation
└── error.rs                 # YubiKey-specific error types (internal)

OR (simpler, recommended for rewrite):

crates/core/src/backends/yubikey.rs  # Single file (~500-800 lines)
```

**Recommendation:** Start with single-file approach. The current `software_hsm.rs` is 1,419 lines and works well. YubiKey backend should be similar size (500-800 lines):
- ~150 lines: Initialization, config, struct definition
- ~200 lines: PIV hardware operations
- ~150 lines: Certificate generation with rcgen
- ~150 lines: UniversalBackend trait implementation
- ~100 lines: Helper functions and tests

### Component Boundaries

| Component | Responsibility | Communicates With |
|-----------|----------------|-------------------|
| `YubiKeyBackend` struct | Configuration, lifecycle, state management | All layers |
| `hardware.rs` (or inline fns) | Direct PIV ops: sign, get pubkey, attest | `yubikey` crate |
| `certificates.rs` (or inline fns) | X.509 cert generation using rcgen | `rcgen` crate |
| `operations.rs` (or trait impl) | Maps `CryptoOperation` to hardware/cert ops | `UniversalBackend` trait |
| Error conversion | Converts yubikey errors → `BackendError` | `error.rs` |

### YubiKeyBackend Struct

```rust
#[cfg(feature = "yubikey")]
pub struct YubiKeyBackend {
    config: YubiKeyConfig,

    // Hardware connection (yubikey crate)
    yubikey: Option<YubiKey>,  // Initialized lazily or on creation

    // Key cache (optional optimization)
    key_cache: HashMap<String, CachedKeyInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YubiKeyConfig {
    /// PIN for PIV operations (required for signing)
    pub pin: Option<String>,

    /// Default PIV slot (9a, 9c, 9d, 9e)
    pub default_slot: String,

    /// Enable verbose logging
    pub verbose: bool,
}

struct CachedKeyInfo {
    slot: PivSlot,
    algorithm: AsymmetricAlgorithm,
    public_key_der: Vec<u8>,  // Cached to avoid re-extraction
}
```

**Key Design Decision:** No PKCS#11 dependency. Use `yubikey` crate's native PIV interface.

**Rationale:**
1. Current implementation has 3,263 lines largely due to PKCS#11 complexity
2. `yubikey` crate provides direct PIV access (cleaner, more maintainable)
3. PKCS#11 adds external dependency (OpenSC library) and platform variance
4. PIV is YubiKey's native protocol — better error messages, simpler debugging

## Integration Patterns

### Pattern 1: Hardware Operations (yubikey crate wrapper)

**What:** Thin wrapper around `yubikey` crate for PIV operations

**Implementation:**
```rust
impl YubiKeyBackend {
    fn piv_sign(&self, slot: PivSlot, data: &[u8]) -> Result<Vec<u8>, BackendError> {
        let yubikey = self.yubikey.as_ref()
            .ok_or_else(|| BackendError::HardwareError("YubiKey not connected".into()))?;

        // Authenticate with PIN if needed
        if let Some(pin) = &self.config.pin {
            yubikey.verify_pin(pin.as_bytes())
                .map_err(|e| BackendError::HardwareError(format!("PIN verification failed: {}", e)))?;
        }

        // Perform signature
        yubikey.sign_data(
            slot,
            data,
            SignatureAlgorithm::EcdsaP256  // Example
        )
        .map_err(|e| BackendError::HardwareError(format!("Signing failed: {}", e)))
    }

    fn piv_get_public_key(&self, slot: PivSlot) -> Result<Vec<u8>, BackendError> {
        let yubikey = self.yubikey.as_ref()
            .ok_or_else(|| BackendError::HardwareError("YubiKey not connected".into()))?;

        let public_key = yubikey.fetch_pubkey(slot)
            .map_err(|e| BackendError::HardwareError(format!("Failed to fetch public key: {}", e)))?;

        // Convert to DER format (SubjectPublicKeyInfo)
        self.encode_public_key_der(&public_key)
    }
}
```

**Trade-offs:**
- ✓ Direct hardware access, no PKCS#11 middleware
- ✓ Better error messages (native YubiKey errors)
- ✗ YubiKey-specific (not generic PKCS#11 device support)

**When to use:** Always for YubiKey backend rewrite. Generic PKCS#11 support can be separate backend later.

### Pattern 2: Certificate Generation (rcgen integration)

**What:** Use `rcgen` crate to generate X.509 certificates signed by YubiKey hardware

**Implementation:**
```rust
use rcgen::{Certificate, CertificateParams, DistinguishedName, KeyPair};

impl YubiKeyBackend {
    fn generate_certificate(&self, slot: PivSlot, subject: &str) -> Result<Vec<u8>, BackendError> {
        // 1. Extract public key from YubiKey
        let public_key_der = self.piv_get_public_key(slot)?;

        // 2. Create rcgen certificate parameters
        let mut params = CertificateParams::new(vec![subject.to_string()]);
        params.distinguished_name = DistinguishedName::new();
        params.distinguished_name.push(
            rcgen::DnType::CommonName,
            subject
        );

        // 3. Create custom key pair wrapper (rcgen requires this)
        let custom_key = CustomKeyPair {
            backend: self,
            slot,
            public_key_der: public_key_der.clone(),
        };

        params.key_pair = Some(KeyPair::from_custom(custom_key)?);

        // 4. Generate certificate (rcgen will call our signing function)
        let cert = Certificate::from_params(params)
            .map_err(|e| BackendError::OperationFailed(format!("Certificate generation failed: {}", e)))?;

        // 5. Serialize to DER
        cert.serialize_der()
            .map_err(|e| BackendError::OperationFailed(format!("Certificate serialization failed: {}", e)))
    }
}

// Custom key pair that delegates signing to YubiKey
struct CustomKeyPair<'a> {
    backend: &'a YubiKeyBackend,
    slot: PivSlot,
    public_key_der: Vec<u8>,
}

impl<'a> rcgen::CustomKeyPair for CustomKeyPair<'a> {
    fn sign(&self, msg: &[u8]) -> Result<Vec<u8>, rcgen::Error> {
        self.backend.piv_sign(self.slot, msg)
            .map_err(|e| rcgen::Error::from_str(&format!("Hardware signing failed: {}", e)))
    }

    fn algorithm(&self) -> &'static rcgen::SignatureAlgorithm {
        &rcgen::PKCS_ECDSA_P256_SHA256
    }
}
```

**Trade-offs:**
- ✓ Standards-compliant X.509 certificates (rcgen is battle-tested)
- ✓ Hardware-backed signing (private key never leaves YubiKey)
- ✓ Simple API (rcgen handles all ASN.1/DER complexity)
- ✗ Requires implementing `CustomKeyPair` trait (10-20 lines of boilerplate)

**When to use:** Always for certificate generation. Avoids manual ASN.1/DER encoding.

### Pattern 3: UniversalBackend Integration

**What:** Map `CryptoOperation` enum to hardware operations with fail-closed error handling

**Implementation:**
```rust
impl UniversalBackend for YubiKeyBackend {
    fn perform_operation(
        &self,
        key_id: &str,
        operation: CryptoOperation,
    ) -> Result<CryptoResult, BackendError> {
        // Parse key_id as PIV slot (e.g., "9c" → PivSlot::Signature)
        let slot = self.parse_slot(key_id)?;

        match operation {
            CryptoOperation::Sign { data, algorithm } => {
                // Validate algorithm is supported
                if !self.supports_algorithm(&algorithm) {
                    return Err(BackendError::UnsupportedOperation(
                        format!("Algorithm {:?} not supported", algorithm)
                    ));
                }

                // Hash data if needed (YubiKey signs hashes, not raw data)
                let digest = self.hash_for_signing(&data, &algorithm)?;

                // Hardware sign
                let signature = self.piv_sign(slot, &digest)?;

                Ok(CryptoResult::Signed(signature))
            }

            CryptoOperation::GetPublicKey => {
                let pubkey_der = self.piv_get_public_key(slot)?;
                Ok(CryptoResult::PublicKey(pubkey_der))
            }

            CryptoOperation::GenerateKeyPair { algorithm } => {
                // Generate key in hardware
                self.piv_generate_key(slot, algorithm)?;

                // Fetch the generated public key
                let pubkey_der = self.piv_get_public_key(slot)?;

                Ok(CryptoResult::KeyPair {
                    public_key: pubkey_der,
                    private_key_id: key_id.to_string(),  // Slot identifier
                })
            }

            CryptoOperation::Attest { challenge } => {
                let attestation = self.piv_attest(slot, &challenge)?;
                Ok(CryptoResult::AttestationProof(attestation))
            }

            // Unsupported operations
            CryptoOperation::Verify { .. } => {
                Err(BackendError::UnsupportedOperation(
                    "YubiKey backend does not support verification (use public key directly)".into()
                ))
            }

            CryptoOperation::Encrypt { .. } | CryptoOperation::Decrypt { .. } => {
                Err(BackendError::UnsupportedOperation(
                    "YubiKey PIV does not support symmetric encryption".into()
                ))
            }

            _ => Err(BackendError::UnsupportedOperation(
                format!("Operation not implemented for YubiKey backend")
            ))
        }
    }

    fn supports_operation(&self, operation: &CryptoOperation) -> bool {
        matches!(operation,
            CryptoOperation::Sign { algorithm, .. }
                if matches!(algorithm, SignatureAlgorithm::EcdsaP256 | SignatureAlgorithm::RsaPkcs1v15)
            | CryptoOperation::GetPublicKey
            | CryptoOperation::GenerateKeyPair { algorithm }
                if matches!(algorithm, AsymmetricAlgorithm::EcdsaP256 | AsymmetricAlgorithm::Rsa2048)
            | CryptoOperation::Attest { .. }
        )
    }

    fn get_capabilities(&self) -> BackendCapabilities {
        BackendCapabilities {
            symmetric_algorithms: vec![],  // PIV doesn't do symmetric
            asymmetric_algorithms: vec![
                AsymmetricAlgorithm::EcdsaP256,
                AsymmetricAlgorithm::Rsa2048,
                AsymmetricAlgorithm::Rsa4096,
            ],
            signature_algorithms: vec![
                SignatureAlgorithm::EcdsaP256,
                SignatureAlgorithm::RsaPkcs1v15,
            ],
            hash_algorithms: vec![
                HashAlgorithm::Sha256,
                HashAlgorithm::Sha384,
            ],
            hardware_backed: true,
            supports_key_derivation: false,
            supports_key_generation: true,
            supports_attestation: true,
            max_key_size: Some(4096),
        }
    }

    fn backend_info(&self) -> BackendInfo {
        BackendInfo {
            name: "yubikey",
            description: "YubiKey PIV hardware security module",
            version: "2.0.0",
            available: self.yubikey.is_some(),
            config_requirements: vec!["pin", "slot"],
        }
    }
}
```

**Trade-offs:**
- ✓ Fail-closed: All errors return `BackendError` variants
- ✓ Clear capability advertisement via `supports_operation()`
- ✓ Explicit unsupported operation handling
- ✗ Requires careful slot parsing and validation

**When to use:** This is the mandatory integration pattern. All backends implement this trait.

## Data Flow

### Signing Operation Flow

```
User calls perform_operation(key_id="9c", Sign { data, algorithm })
    ↓
1. Parse slot: "9c" → PivSlot::Signature
    ↓
2. Validate algorithm: EcdsaP256 supported? ✓
    ↓
3. Hash data: SHA256(data) → digest
    ↓
4. Authenticate: yubikey.verify_pin(pin)
    ↓
5. Sign: yubikey.sign_data(slot, digest, algorithm) → signature
    ↓
6. Return: Ok(CryptoResult::Signed(signature))

Error at any step → Err(BackendError::HardwareError(...))
```

### Certificate Generation Flow

```
User calls generate_certificate(slot, subject)
    ↓
1. Extract public key: yubikey.fetch_pubkey(slot) → pubkey
    ↓
2. Encode as DER: encode_public_key_der(pubkey) → pubkey_der
    ↓
3. Create rcgen params: CertificateParams { subject, ... }
    ↓
4. Wrap as CustomKeyPair: delegates signing to YubiKey
    ↓
5. rcgen generates TBS cert → calls CustomKeyPair::sign(tbs_data)
    ↓
6. YubiKey signs: yubikey.sign_data(slot, tbs_data) → signature
    ↓
7. rcgen assembles: TBS + signature → X.509 certificate DER
    ↓
8. Return: Ok(certificate_der)

Error at any step → Err(BackendError::OperationFailed(...))
```

### Key Generation Flow

```
User calls perform_operation(key_id="9c", GenerateKeyPair { EcdsaP256 })
    ↓
1. Parse slot: "9c" → PivSlot::Signature
    ↓
2. Authenticate: yubikey.verify_pin(pin)
    ↓
3. Generate: yubikey.generate(slot, AlgorithmId::EccP256) → generates on-chip
    ↓
4. Fetch pubkey: yubikey.fetch_pubkey(slot) → new public key
    ↓
5. Encode: encode_public_key_der(pubkey) → pubkey_der
    ↓
6. Return: Ok(CryptoResult::KeyPair {
       public_key: pubkey_der,
       private_key_id: "9c"
   })

Private key NEVER leaves hardware
Error at any step → Err(BackendError::HardwareError(...))
```

## Error Handling Architecture

### Error Conversion Strategy

```rust
// Internal error types (not exposed)
enum YubiKeyInternalError {
    PivError(yubikey::Error),
    RcgenError(rcgen::Error),
    EncodingError(String),
}

impl From<YubiKeyInternalError> for BackendError {
    fn from(err: YubiKeyInternalError) -> Self {
        match err {
            YubiKeyInternalError::PivError(e) => {
                // Classify yubikey errors
                if e.to_string().contains("not found") {
                    BackendError::KeyNotFound(e.to_string())
                } else if e.to_string().contains("PIN") {
                    BackendError::HardwareError(format!("Authentication failed: {}", e))
                } else {
                    BackendError::HardwareError(e.to_string())
                }
            }

            YubiKeyInternalError::RcgenError(e) => {
                BackendError::OperationFailed(format!("Certificate generation failed: {}", e))
            }

            YubiKeyInternalError::EncodingError(msg) => {
                BackendError::OperationFailed(format!("Encoding error: {}", msg))
            }
        }
    }
}
```

### Fail-Closed Guarantees

1. **Hardware not connected:** `BackendError::InitializationFailed` during `new()`
2. **PIN verification fails:** `BackendError::HardwareError("Authentication failed")`
3. **Key not in slot:** `BackendError::KeyNotFound(slot)`
4. **Unsupported algorithm:** `BackendError::UnsupportedOperation(algorithm)`
5. **Signature fails:** `BackendError::HardwareError("Signing failed")`

**All errors bubble through `Result<CryptoResult, BackendError>`** — no panics, no silent failures.

## Test Organization

### Test File Structure

```
crates/core/tests/
├── yubikey_backend_integration.rs    # Integration tests (requires hardware)
└── yubikey_backend_unit.rs           # Unit tests (always-run, mock hardware)
```

### Integration Tests (Hardware-Required)

```rust
// tests/yubikey_backend_integration.rs
#![cfg(all(test, feature = "yubikey"))]

use trustedge_core::backends::{UniversalBackend, YubiKeyBackend};

#[test]
#[ignore]  // Run with: cargo test --features yubikey -- --ignored
fn test_real_hardware_signing() -> anyhow::Result<()> {
    // Requires YubiKey connected with key in slot 9c
    let config = YubiKeyConfig {
        pin: Some("123456".into()),
        default_slot: "9c".into(),
        verbose: true,
    };

    let backend = YubiKeyBackend::new(config)?;

    let operation = CryptoOperation::Sign {
        data: b"test data".to_vec(),
        algorithm: SignatureAlgorithm::EcdsaP256,
    };

    let result = backend.perform_operation("9c", operation)?;

    match result {
        CryptoResult::Signed(signature) => {
            assert!(!signature.is_empty());
            println!("✓ Signature: {} bytes", signature.len());
            Ok(())
        }
        _ => panic!("Expected Signed result"),
    }
}

#[test]
#[ignore]
fn test_certificate_generation() -> anyhow::Result<()> {
    let backend = YubiKeyBackend::new(YubiKeyConfig::default())?;

    let cert_der = backend.generate_certificate(
        PivSlot::Signature,
        "CN=Test Device"
    )?;

    // Validate certificate structure
    let cert = x509_parser::parse_x509_certificate(&cert_der)
        .expect("Valid X.509 certificate");

    assert_eq!(cert.1.subject().to_string(), "CN=Test Device");
    Ok(())
}
```

### Unit Tests (Always-Run, No Hardware)

```rust
// tests/yubikey_backend_unit.rs OR inline in yubikey.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slot_parsing() {
        // Test slot string → PivSlot conversion
        assert_eq!(parse_slot("9c"), Ok(PivSlot::Signature));
        assert_eq!(parse_slot("9a"), Ok(PivSlot::Authentication));
        assert!(parse_slot("invalid").is_err());
    }

    #[test]
    fn test_supports_operation() {
        let backend = create_mock_backend();

        // Should support signing
        let sign_op = CryptoOperation::Sign {
            data: vec![],
            algorithm: SignatureAlgorithm::EcdsaP256,
        };
        assert!(backend.supports_operation(&sign_op));

        // Should NOT support encryption
        let encrypt_op = CryptoOperation::Encrypt {
            plaintext: vec![],
            algorithm: SymmetricAlgorithm::Aes256Gcm,
        };
        assert!(!backend.supports_operation(&encrypt_op));
    }

    #[test]
    fn test_error_conversion() {
        // Test yubikey::Error → BackendError conversion
        let yubikey_err = yubikey::Error::KeyNotFound;
        let backend_err: BackendError = yubikey_err.into();

        assert!(matches!(backend_err, BackendError::KeyNotFound(_)));
    }

    // Mock backend for testing without hardware
    fn create_mock_backend() -> YubiKeyBackend {
        YubiKeyBackend {
            config: YubiKeyConfig::default(),
            yubikey: None,  // No hardware
            key_cache: HashMap::new(),
        }
    }
}
```

### Test Gating Strategy

**Run always (no hardware required):**
```bash
cargo test -p trustedge-core --lib yubikey_backend_unit
```

**Run with hardware:**
```bash
cargo test -p trustedge-core --features yubikey --test yubikey_backend_integration -- --ignored
```

**CI configuration:**
- Default CI: Run unit tests only (no hardware, no `--ignored`)
- Manual/nightly CI: Run integration tests with hardware connected

## Build Order Considerations

### Dependency Graph

```
trustedge-core (yubikey.rs)
    ↓ depends on
yubikey = { version = "0.7", optional = true }
rcgen = { version = "0.13", optional = true }
    ↓ (both gated by feature flag)
[feature = "yubikey"]
```

### Feature Flag Configuration (Cargo.toml)

```toml
[dependencies]
yubikey = { version = "0.7", optional = true, features = ["untested"] }
rcgen = { version = "0.13", optional = true }

[features]
default = []
yubikey = ["dep:yubikey", "dep:rcgen"]
```

**Build commands:**
```bash
# Without YubiKey support (default)
cargo build

# With YubiKey support
cargo build --features yubikey

# Run YubiKey tests (unit only, no hardware)
cargo test --features yubikey yubikey_backend_unit

# Run YubiKey integration tests (requires hardware)
cargo test --features yubikey --test yubikey_backend_integration -- --ignored
```

### Conditional Compilation

```rust
#[cfg(feature = "yubikey")]
pub mod yubikey;

#[cfg(feature = "yubikey")]
pub use yubikey::{YubiKeyBackend, YubiKeyConfig};

// Stub implementation when feature disabled
#[cfg(not(feature = "yubikey"))]
pub struct YubiKeyBackend;

#[cfg(not(feature = "yubikey"))]
impl YubiKeyBackend {
    pub fn new(_config: ()) -> Result<Self, BackendError> {
        Err(BackendError::UnsupportedOperation(
            "YubiKey support not compiled (enable 'yubikey' feature)".into()
        ))
    }
}
```

## Integration with Existing Architecture

### Modified Components

| File | Change Type | Description |
|------|-------------|-------------|
| `backends/yubikey.rs` | **DELETE + REWRITE** | Replace 3,263 lines with 500-800 line rewrite |
| `backends/mod.rs` | **MODIFY** | Update exports (same public API) |
| `error.rs` | **NO CHANGE** | `BackendError` already sufficient |
| `Cargo.toml` | **ADD DEP** | Add `rcgen = { version = "0.13", optional = true }` |

### New Components

| Component | Location | Purpose |
|-----------|----------|---------|
| `YubiKeyBackend::piv_*` methods | `backends/yubikey.rs` | Hardware operation wrappers |
| `generate_certificate()` | `backends/yubikey.rs` | rcgen-based cert generation |
| `CustomKeyPair` impl | `backends/yubikey.rs` | rcgen signing delegation |
| Unit tests | `backends/yubikey.rs` (inline) | No-hardware tests |
| Integration tests | `tests/yubikey_backend_integration.rs` | Hardware tests |

### Unchanged Components

- `UniversalBackend` trait definition (no changes needed)
- `BackendError` hierarchy (sufficient for all error cases)
- `SoftwareHsmBackend` (reference implementation)
- `KeyringBackend` (minimal backend example)
- `BackendRegistry` (auto-discovers via `mod.rs` export)

## Anti-Patterns to Avoid

### Anti-Pattern 1: Monolithic Implementation (Current Problem)

**What people do:** Put all logic in one 3,263-line file with PKCS#11, certificate generation, error handling, and testing mixed together.

**Why it's wrong:**
- Impossible to test without hardware
- PKCS#11 dependency adds platform variance
- Hard to understand data flow
- Error handling scattered throughout

**Do this instead:**
- Separate concerns: hardware ops, cert generation, backend trait
- Use native `yubikey` crate (not PKCS#11)
- Clear error conversion at boundaries
- Unit tests don't require hardware

### Anti-Pattern 2: Silent Fallbacks

**What people do:** When hardware operation fails, silently return placeholder data or "success" result.

**Why it's wrong:**
- Security critical: signature validation fails if placeholder used
- Debugging nightmare: no indication of failure
- Violates fail-closed principle

**Do this instead:**
```rust
// BAD
fn piv_sign(&self, slot: PivSlot, data: &[u8]) -> Result<Vec<u8>, BackendError> {
    match self.yubikey.sign(slot, data) {
        Ok(sig) => Ok(sig),
        Err(_) => Ok(vec![0; 64]),  // ❌ SILENT FAILURE
    }
}

// GOOD
fn piv_sign(&self, slot: PivSlot, data: &[u8]) -> Result<Vec<u8>, BackendError> {
    let yubikey = self.yubikey.as_ref()
        .ok_or_else(|| BackendError::HardwareError("YubiKey not connected".into()))?;

    yubikey.sign(slot, data)
        .map_err(|e| BackendError::HardwareError(format!("Signing failed: {}", e)))
    // ✓ FAIL CLOSED
}
```

### Anti-Pattern 3: Manual ASN.1/DER Encoding for Certificates

**What people do:** Manually construct X.509 certificates with raw DER encoding.

**Why it's wrong:**
- Error-prone (easy to get wrong)
- Hard to maintain (ASN.1 is complex)
- No validation (manually constructed certs may be invalid)
- Reinventing the wheel (rcgen exists)

**Do this instead:**
```rust
// BAD: Manual DER encoding (150+ lines of fragile code)
fn build_x509_certificate(&self, pubkey: &[u8]) -> Result<Vec<u8>, BackendError> {
    let mut cert_der = Vec::new();
    cert_der.push(0x30); // SEQUENCE
    cert_der.push(0x82); // Length > 255
    // ... 100+ more lines of DER assembly
}

// GOOD: Use rcgen (10-20 lines)
fn generate_certificate(&self, slot: PivSlot, subject: &str) -> Result<Vec<u8>, BackendError> {
    let pubkey_der = self.piv_get_public_key(slot)?;

    let mut params = CertificateParams::new(vec![subject.to_string()]);
    params.key_pair = Some(KeyPair::from_custom(CustomKeyPair { ... })?);

    Certificate::from_params(params)?.serialize_der()
        .map_err(|e| BackendError::OperationFailed(format!("Cert generation: {}", e)))
}
```

### Anti-Pattern 4: Ignoring PIN Verification Failures

**What people do:** Continue operations after PIN verification fails.

**Why it's wrong:**
- Subsequent operations will fail anyway (or use wrong key)
- Security risk: may allow operations with default/test PINs
- Confusing error messages later

**Do this instead:**
```rust
// BAD
fn initialize(&mut self) -> Result<(), BackendError> {
    let yubikey = YubiKey::open()?;

    if let Some(pin) = &self.config.pin {
        let _ = yubikey.verify_pin(pin.as_bytes());  // ❌ IGNORING RESULT
    }

    self.yubikey = Some(yubikey);
    Ok(())
}

// GOOD
fn initialize(&mut self) -> Result<(), BackendError> {
    let yubikey = YubiKey::open()
        .map_err(|e| BackendError::InitializationFailed(format!("YubiKey open: {}", e)))?;

    if let Some(pin) = &self.config.pin {
        yubikey.verify_pin(pin.as_bytes())
            .map_err(|e| BackendError::HardwareError(format!("PIN verification failed: {}", e)))?;
        // ✓ FAIL IF PIN WRONG
    }

    self.yubikey = Some(yubikey);
    Ok(())
}
```

## Scaling Considerations

**Current scale:** Single YubiKey per backend instance (typical use case)

| Scale | Approach |
|-------|----------|
| 1 YubiKey | Single `YubiKeyBackend` instance, direct connection |
| Multiple YubiKeys | Multiple backend instances, one per device |
| High-frequency signing | Implement key caching (public key extraction) |
| Concurrent operations | YubiKey PIV is single-threaded, use mutex or operation queue |

### Performance Optimizations (If Needed)

```rust
// Optimization 1: Cache public keys (avoid re-extraction)
struct CachedKeyInfo {
    public_key_der: Vec<u8>,
    cached_at: SystemTime,
}

impl YubiKeyBackend {
    fn get_cached_public_key(&mut self, slot: PivSlot) -> Result<Vec<u8>, BackendError> {
        if let Some(cached) = self.key_cache.get(&slot) {
            if cached.cached_at.elapsed()? < Duration::from_secs(300) {  // 5 min TTL
                return Ok(cached.public_key_der.clone());
            }
        }

        // Cache miss or expired
        let pubkey = self.piv_get_public_key(slot)?;
        self.key_cache.insert(slot, CachedKeyInfo {
            public_key_der: pubkey.clone(),
            cached_at: SystemTime::now(),
        });

        Ok(pubkey)
    }
}

// Optimization 2: Operation queue for concurrency
use tokio::sync::Mutex;

struct ConcurrentYubiKeyBackend {
    inner: Arc<Mutex<YubiKeyBackend>>,
}

impl ConcurrentYubiKeyBackend {
    async fn perform_operation(&self, key_id: &str, op: CryptoOperation)
        -> Result<CryptoResult, BackendError>
    {
        let backend = self.inner.lock().await;
        backend.perform_operation(key_id, op)
    }
}
```

**When to optimize:**
- Cache public keys: If extracting keys 100+ times/second
- Concurrent queue: If multiple threads need YubiKey access
- Default implementation: Direct, synchronous operations (sufficient for most use cases)

## Summary: New vs Modified Components

### NEW Components (Rewrite)

| Component | Lines (est) | Purpose |
|-----------|-------------|---------|
| `YubiKeyBackend` struct | ~50 | Config, state, lifecycle |
| PIV operations wrapper | ~200 | `yubikey` crate integration |
| Certificate generation | ~150 | `rcgen` integration |
| `UniversalBackend` impl | ~150 | Trait implementation |
| Error conversion | ~50 | `yubikey::Error` → `BackendError` |
| Unit tests | ~100 | No-hardware tests |
| **Total** | **~700 lines** | (vs 3,263 current) |

### MODIFIED Components

| File | Change | Reason |
|------|--------|--------|
| `Cargo.toml` | Add `rcgen` dependency | Certificate generation |
| `backends/mod.rs` | Update exports (same API) | Re-export new implementation |

### UNCHANGED Components

| Component | Why No Change |
|-----------|---------------|
| `UniversalBackend` trait | Already perfect for YubiKey integration |
| `BackendError` enum | Sufficient for all YubiKey error cases |
| `error.rs` hierarchy | Well-designed fail-closed architecture |
| Other backends | Independent, unaffected by YubiKey rewrite |

## Sources

- **YubiKey PIV Architecture:** https://developers.yubico.com/PIV/Introduction/
- **yubikey crate documentation:** https://docs.rs/yubikey/latest/yubikey/
- **rcgen certificate generation:** https://docs.rs/rcgen/latest/rcgen/
- **Existing Universal Backend pattern:** `crates/core/src/backends/software_hsm.rs` (1,419 lines, reference implementation)
- **TrustEdge error hierarchy:** `crates/core/src/error.rs` (BackendError, TrustEdgeError)

---
*Architecture research for: YubiKey backend rewrite integration*
*Researched: 2026-02-11*
*Confidence: HIGH (based on existing codebase analysis and YubiKey PIV documentation)*
