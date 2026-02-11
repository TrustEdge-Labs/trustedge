# Phase 10: Backend Rewrite - Research

**Researched:** 2026-02-11
**Domain:** YubiKey PIV backend implementation using Universal Backend trait
**Confidence:** MEDIUM-HIGH

## Summary

Phase 10 implements a production-quality YubiKey backend using the Universal Backend architecture established in v1.0. This is a clean-slate rewrite after Phase 9 deleted the broken 8,117-line implementation. The new backend uses stable APIs only (yubikey crate without `untested` feature), delegates X.509 certificate generation to rcgen, and follows a fail-closed design that returns errors when hardware is unavailable rather than silently falling back to software.

The implementation integrates three distinct layers: hardware PIV operations (yubikey crate), certificate generation (rcgen), and Universal Backend trait implementation. Target size is 500-800 lines based on successful software_hsm.rs reference implementation (1,419 lines with comprehensive features).

**Primary recommendation:** Implement as single-file backend (backends/yubikey.rs) following software_hsm.rs pattern. Use yubikey crate stable API for PIV operations, rcgen's RemoteKeyPair/SigningKey pattern for hardware-backed certificate generation, and maintain strict fail-closed error handling with no software fallbacks.

## Standard Stack

### Core Dependencies

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| `yubikey` | 0.7 | YubiKey PIV operations | Official pure-Rust YubiKey driver, PC/SC interface, stable PIV API |
| `rcgen` | 0.13+ | X.509 certificate generation | Industry standard cert builder used by rustls/trust-dns, eliminates manual DER encoding |
| `der` | 0.7 | DER encoding/decoding utilities | RustCrypto standard, required for key material serialization |
| `spki` | 0.7 | SubjectPublicKeyInfo structures | RustCrypto standard for public key encoding (RFC 5280) |
| `signature` | 2.2 | Signature trait abstractions | RustCrypto ecosystem standard for algorithm-agnostic signing |

### Supporting Dependencies (Already in Workspace)

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `pkcs11` | 0.5 | PKCS#11 interface (optional) | Only if PKCS#11 path chosen over native PIV API |
| `x509-cert` | 0.2 | X.509 certificate parsing | Certificate validation and reading existing certs from slots |
| `sha2` | 0.10 | Hash functions | Pre-hashing data for signatures (YubiKey signs digests) |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| yubikey crate native API | PKCS#11 via pkcs11 crate | PKCS#11: Generic device support, external library dependency. Native: YubiKey-specific, simpler, better errors |
| rcgen | Manual x509-cert builder | Manual: Full control. rcgen: 1,000+ lines eliminated, battle-tested, NIST-compliant |
| Stable yubikey API | yubikey with `untested` feature | Untested: More APIs. Stable: Production-ready, documented, security-auditable |

**Decision: Use yubikey native API + rcgen**

Rationale from Phase 9 cleanup research: Current 3,263-line backend complexity largely from PKCS#11 abstraction layer. YubiKey native PIV API is simpler, more maintainable, provides better error messages. Generic PKCS#11 support can be separate backend later if needed.

**Installation:**
```bash
# Already in Cargo.toml workspace, just enable feature
cargo build --features yubikey
```

## Architecture Patterns

### Recommended Project Structure

**Single-file approach (RECOMMENDED):**
```
crates/core/src/backends/
├── yubikey.rs               # 500-800 lines total
```

**Multi-file approach (if exceeds 1,000 lines):**
```
crates/core/src/backends/yubikey/
├── mod.rs                   # Public API, YubiKeyBackend struct
├── hardware.rs              # PIV operations wrapper
├── certificates.rs          # rcgen integration
└── operations.rs            # UniversalBackend implementation
```

Start with single-file. Software HSM reference implementation is 1,419 lines and works well as single file.

### Pattern 1: YubiKey Backend Structure

**What:** Configuration-driven backend with lazy hardware connection

**When to use:** All hardware backends requiring optional availability

**Example:**
```rust
#[cfg(feature = "yubikey")]
pub struct YubiKeyBackend {
    config: YubiKeyConfig,
    // Lazy-initialized hardware connection
    yubikey: Option<yubikey::YubiKey>,
    // Optional: cache public keys to avoid re-extraction
    key_cache: HashMap<String, PublicKeyInfo>,
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

impl YubiKeyBackend {
    pub fn new() -> Result<Self, BackendError> {
        Self::with_config(YubiKeyConfig::default())
    }

    pub fn with_config(config: YubiKeyConfig) -> Result<Self, BackendError> {
        let mut backend = Self {
            config,
            yubikey: None,
            key_cache: HashMap::new(),
        };

        // Attempt connection, but allow lazy init
        backend.connect_if_available()?;
        Ok(backend)
    }

    fn connect_if_available(&mut self) -> Result<(), BackendError> {
        match yubikey::YubiKey::open() {
            Ok(yk) => {
                self.yubikey = Some(yk);
                Ok(())
            }
            Err(e) => {
                // Log warning but don't fail initialization
                // backend_info() will report available: false
                eprintln!("YubiKey not detected: {}", e);
                Ok(())
            }
        }
    }

    fn ensure_connected(&self) -> Result<&yubikey::YubiKey, BackendError> {
        self.yubikey.as_ref()
            .ok_or_else(|| BackendError::HardwareError(
                "YubiKey not connected. Insert device and retry.".into()
            ))
    }
}
```

### Pattern 2: PIV Slot Mapping

**What:** Map string key_id to PIV slot identifiers

**When to use:** UniversalBackend perform_operation() implementation

**Example:**
```rust
use yubikey::piv::{SlotId, AlgorithmId};

impl YubiKeyBackend {
    fn parse_slot(&self, key_id: &str) -> Result<SlotId, BackendError> {
        match key_id {
            "9a" => Ok(SlotId::Authentication),
            "9c" => Ok(SlotId::Signature),
            "9d" => Ok(SlotId::KeyManagement),
            "9e" => Ok(SlotId::CardAuthentication),
            _ => Err(BackendError::KeyNotFound(
                format!("Unknown PIV slot: {}", key_id)
            ))
        }
    }

    fn algorithm_to_piv(algorithm: &SignatureAlgorithm) -> Result<AlgorithmId, BackendError> {
        match algorithm {
            SignatureAlgorithm::EcdsaP256 => Ok(AlgorithmId::EccP256),
            SignatureAlgorithm::RsaPkcs1v15 | SignatureAlgorithm::RsaPss => Ok(AlgorithmId::Rsa2048),
            SignatureAlgorithm::Ed25519 => Err(BackendError::UnsupportedOperation(
                "Ed25519 not supported by YubiKey PIV (use ECDSA P-256)".into()
            )),
        }
    }
}
```

**PIV Slot Usage:**
- **9a (Authentication)**: Device/user authentication, OS logins, SSH, WiFi
- **9c (Signature)**: Digital signatures, document signing (PIN always required)
- **9d (Key Management)**: Encryption/decryption for confidentiality
- **9e (Card Authentication)**: Physical access, no PIN required

Source: [Yubico PIV Certificate Slots](https://developers.yubico.com/PIV/Introduction/Certificate_slots.html)

### Pattern 3: Hardware Signing with Pre-Hashing

**What:** YubiKey PIV signs digests (hashed data), not raw data

**When to use:** CryptoOperation::Sign implementation

**Example:**
```rust
use sha2::{Sha256, Digest};

impl YubiKeyBackend {
    fn piv_sign(&self, slot: SlotId, data: &[u8], algorithm: SignatureAlgorithm)
        -> Result<Vec<u8>, BackendError>
    {
        let yubikey = self.ensure_connected()?;

        // Verify PIN if configured
        if let Some(pin) = &self.config.pin {
            yubikey.verify_pin(pin.as_bytes())
                .map_err(|e| BackendError::HardwareError(
                    format!("PIN verification failed: {}", e)
                ))?;
        }

        // Pre-hash data (YubiKey signs digests)
        let digest = match algorithm {
            SignatureAlgorithm::EcdsaP256 => {
                let mut hasher = Sha256::new();
                hasher.update(data);
                hasher.finalize().to_vec()
            }
            SignatureAlgorithm::RsaPkcs1v15 => {
                // RSA with SHA-256
                let mut hasher = Sha256::new();
                hasher.update(data);
                hasher.finalize().to_vec()
            }
            _ => return Err(BackendError::UnsupportedOperation(
                format!("Algorithm {:?} not supported", algorithm)
            ))
        };

        // Sign digest with hardware
        yubikey.sign_data(slot, &digest)
            .map_err(|e| BackendError::HardwareError(
                format!("Hardware signing failed: {}", e)
            ))
    }
}
```

### Pattern 4: Public Key Extraction with DER Encoding

**What:** Extract public key from PIV slot and encode as SubjectPublicKeyInfo

**When to use:** CryptoOperation::GetPublicKey implementation

**Example:**
```rust
use spki::SubjectPublicKeyInfo;
use der::Encode;

impl YubiKeyBackend {
    fn piv_get_public_key(&self, slot: SlotId) -> Result<Vec<u8>, BackendError> {
        let yubikey = self.ensure_connected()?;

        // Fetch certificate from slot (contains public key)
        let cert = yubikey.fetch_certificate(slot)
            .map_err(|e| BackendError::KeyNotFound(
                format!("No certificate in slot {:?}: {}", slot, e)
            ))?;

        // Extract public key from certificate
        let public_key = cert.subject_public_key_info()
            .map_err(|e| BackendError::OperationFailed(
                format!("Failed to extract public key: {}", e)
            ))?;

        // Encode as DER (SubjectPublicKeyInfo format)
        public_key.to_der()
            .map_err(|e| BackendError::OperationFailed(
                format!("DER encoding failed: {}", e)
            ))
    }
}
```

Source: [spki crate documentation](https://docs.rs/spki/)

### Pattern 5: Certificate Generation with rcgen (Hardware-Backed Signing)

**What:** Generate X.509 certificate signed by YubiKey using rcgen's RemoteKeyPair/SigningKey

**When to use:** X.509 certificate generation operations

**Example:**
```rust
use rcgen::{CertificateParams, DistinguishedName, RemoteKeyPair, SignatureAlgorithm as RcgenAlgorithm};

impl YubiKeyBackend {
    fn generate_certificate(&self, slot: SlotId, subject: &str)
        -> Result<Vec<u8>, BackendError>
    {
        // 1. Get public key from hardware
        let public_key_der = self.piv_get_public_key(slot)?;

        // 2. Create certificate parameters
        let mut params = CertificateParams::new(vec![subject.to_string()]);
        let mut dn = DistinguishedName::new();
        dn.push(rcgen::DnType::CommonName, subject);
        params.distinguished_name = dn;

        // 3. Create remote key pair (delegates signing to hardware)
        let key_pair = RemoteKeyPair::new(
            // Public key
            public_key_der,
            // Signature algorithm
            &rcgen::PKCS_ECDSA_P256_SHA256,
            // Signing callback (calls YubiKey)
            Box::new(move |tbs_cert| {
                // tbs_cert = ToBeSigned certificate bytes
                self.piv_sign(slot, tbs_cert, SignatureAlgorithm::EcdsaP256)
                    .map_err(|e| rcgen::Error::from(format!("{}", e)))
            })
        )?;

        params.key_pair = Some(rcgen::KeyPair::from_remote(key_pair)?);

        // 4. Generate and serialize certificate
        let cert = rcgen::Certificate::from_params(params)
            .map_err(|e| BackendError::OperationFailed(
                format!("Certificate generation failed: {}", e)
            ))?;

        cert.serialize_der()
            .map_err(|e| BackendError::OperationFailed(
                format!("Certificate serialization failed: {}", e)
            ))
    }
}
```

**Note:** rcgen API changed in v0.14 - RemoteKeyPair trait renamed to SigningKey. Verify current API during implementation.

Source: [rcgen releases - v0.14.0 planning](https://github.com/rustls/rcgen/issues/357)

### Pattern 6: UniversalBackend Implementation

**What:** Map CryptoOperation enum to hardware operations with capability checks

**When to use:** Required trait implementation for all backends

**Example:**
```rust
impl UniversalBackend for YubiKeyBackend {
    fn perform_operation(
        &self,
        key_id: &str,
        operation: CryptoOperation,
    ) -> Result<CryptoResult, BackendError> {
        let slot = self.parse_slot(key_id)?;

        match operation {
            CryptoOperation::Sign { data, algorithm } => {
                let signature = self.piv_sign(slot, &data, algorithm)?;
                Ok(CryptoResult::Signed(signature))
            }

            CryptoOperation::GetPublicKey => {
                let public_key = self.piv_get_public_key(slot)?;
                Ok(CryptoResult::PublicKey(public_key))
            }

            CryptoOperation::GenerateKeyPair { algorithm } => {
                let piv_alg = Self::algorithm_to_piv(&algorithm)?;
                let yubikey = self.ensure_connected()?;

                // Generate key pair in hardware
                yubikey.generate(slot, piv_alg)
                    .map_err(|e| BackendError::HardwareError(
                        format!("Key generation failed: {}", e)
                    ))?;

                // Return public key
                let public_key = self.piv_get_public_key(slot)?;
                Ok(CryptoResult::KeyPair {
                    public_key,
                    private_key_id: key_id.to_string(),
                })
            }

            CryptoOperation::Attest { challenge } => {
                let yubikey = self.ensure_connected()?;
                let attestation = yubikey.attest(slot, &challenge)
                    .map_err(|e| BackendError::HardwareError(
                        format!("Attestation failed: {}", e)
                    ))?;
                Ok(CryptoResult::AttestationProof(attestation))
            }

            _ => Err(BackendError::UnsupportedOperation(
                format!("Operation {:?} not supported by YubiKey backend", operation)
            ))
        }
    }

    fn supports_operation(&self, operation: &CryptoOperation) -> bool {
        matches!(operation,
            CryptoOperation::Sign { algorithm: SignatureAlgorithm::EcdsaP256 | SignatureAlgorithm::RsaPkcs1v15, .. } |
            CryptoOperation::GetPublicKey |
            CryptoOperation::GenerateKeyPair { algorithm: AsymmetricAlgorithm::EcdsaP256 | AsymmetricAlgorithm::Rsa2048, .. } |
            CryptoOperation::Attest { .. }
        )
    }

    fn get_capabilities(&self) -> BackendCapabilities {
        BackendCapabilities {
            symmetric_algorithms: vec![],  // YubiKey doesn't do symmetric crypto
            asymmetric_algorithms: vec![
                AsymmetricAlgorithm::EcdsaP256,
                AsymmetricAlgorithm::Rsa2048,
            ],
            signature_algorithms: vec![
                SignatureAlgorithm::EcdsaP256,
                SignatureAlgorithm::RsaPkcs1v15,
            ],
            hash_algorithms: vec![HashAlgorithm::Sha256],
            hardware_backed: true,
            supports_key_derivation: false,
            supports_key_generation: true,
            supports_attestation: true,
            max_key_size: Some(2048),  // RSA-2048 max
        }
    }

    fn backend_info(&self) -> BackendInfo {
        BackendInfo {
            name: "yubikey",
            description: "YubiKey PIV hardware security backend",
            version: "1.0.0",
            available: self.yubikey.is_some(),  // Real hardware presence
            config_requirements: vec!["pin (optional)", "default_slot"],
        }
    }

    fn list_keys(&self) -> Result<Vec<KeyMetadata>, BackendError> {
        let yubikey = self.ensure_connected()?;
        let mut keys = Vec::new();

        // Enumerate standard PIV slots
        for slot in &[SlotId::Authentication, SlotId::Signature, SlotId::KeyManagement, SlotId::CardAuthentication] {
            if let Ok(_cert) = yubikey.fetch_certificate(*slot) {
                keys.push(KeyMetadata {
                    key_id: format!("{:?}", slot).into(),
                    description: format!("PIV slot {:?}", slot),
                    created_at: 0,  // YubiKey doesn't track creation time
                    last_used: None,
                    backend_data: vec![],
                });
            }
        }

        Ok(keys)
    }
}
```

### Anti-Patterns to Avoid

- **Silent fallbacks**: Never return software-generated keys when hardware fails. Return `BackendError::HardwareError` instead.
- **Placeholder keys**: Zero tolerance for hardcoded test vectors in production code paths.
- **Manual DER encoding**: Use `der`, `spki`, `rcgen` crates exclusively. No manual ASN.1 tag manipulation.
- **PKCS#11 + native API mixing**: Choose one interface (native recommended), never mix.
- **Uncached session state**: Open/verify/operate/close in tight scope. Don't cache sessions across operations.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| X.509 certificate generation | Manual ASN.1/DER encoding | `rcgen::CertificateParams` + `RemoteKeyPair` | 1,000+ lines eliminated, battle-tested, NIST-compliant |
| Public key serialization | Manual SPKI construction | `spki::SubjectPublicKeyInfo::to_der()` | RFC 5280 compliance, RustCrypto standard |
| Signature format encoding | Raw byte concatenation | `signature` crate traits | Algorithm abstraction, standard interfaces |
| PIN retry logic | Loop with counter | yubikey crate built-in retry limits | Prevents YubiKey lockout (3 retries then block) |
| Slot enumeration | PKCS#11 C_GetSlotList | yubikey crate PIV slot iteration | Native YubiKey API, better error messages |

**Key insight:** Current 3,263-line backend has ~1,000 lines of manual cryptographic encoding. rcgen + der/spki crates eliminate this entirely. Remaining complexity is business logic (slot mapping, error handling, capability checking).

## Common Pitfalls

### Pitfall 1: Silent Software Fallback (CRITICAL)

**What goes wrong:** Backend returns software-generated keys when YubiKey unavailable instead of returning error.

**Why it happens:** Convenience during development, desire for tests to pass without hardware.

**How to avoid:**
- `backend_info()` MUST return `available: false` when `self.yubikey.is_none()`
- `perform_operation()` MUST call `self.ensure_connected()` which returns `BackendError::HardwareError`
- Zero placeholder implementations - no fallback code paths
- Separate test fixtures from production backend (different types, not runtime flags)

**Warning signs:**
- Code contains `if hardware { real } else { fake }` patterns
- Methods return `Ok(...)` when hardware unavailable
- Backend reports `hardware_backed: true` but has no active session

**Verification:**
```rust
#[test]
fn test_no_silent_fallback() {
    let backend = YubiKeyBackend::new().unwrap();

    // If YubiKey not present, backend_info should reflect this
    if backend.backend_info().available == false {
        // Operations must fail, not fallback
        let result = backend.perform_operation("9a", CryptoOperation::GetPublicKey);
        assert!(matches!(result, Err(BackendError::HardwareError(_))));
    }
}
```

### Pitfall 2: Manual ASN.1/DER Encoding

**What goes wrong:** Implementing custom DER encoding instead of using battle-tested libraries. Results in encoding bugs, incompatibility, security vulnerabilities.

**Why it happens:** "It's just a simple format", library doesn't do exactly what you need.

**How to avoid:**
- NEVER implement ASN.1/DER/X.509 encoding manually
- Use `rcgen::CertificateParams` for certificate generation
- Use `spki::SubjectPublicKeyInfo::to_der()` for public key encoding
- Use `der::Encode` trait for any DER serialization
- Code review rule: any `push(0x30)` or manual tag construction is FORBIDDEN

**Warning signs:**
- Functions named `encode_asn1_*`, `build_*_der`, `create_*_certificate`
- Hardcoded hex constants: `0x02`, `0x30`, `0x03` (ASN.1 tags)
- Manual length calculations: `output.push(length as u8)`
- More than 50 lines in certificate generation function

**Verification:**
```bash
# Search for manual DER encoding patterns
grep -r "0x30\|0x02\|push.*tag" crates/core/src/backends/yubikey.rs
# Should return zero matches
```

### Pitfall 3: YubiKey PIV Signing Raw Data Instead of Digests

**What goes wrong:** Passing raw data to `sign_data()` instead of pre-hashed digest. YubiKey PIV expects digests (hashed data).

**Why it happens:** Software signing libraries accept raw data and hash internally. YubiKey PIV doesn't.

**How to avoid:**
- Always pre-hash data before signing: `Sha256::digest(data)`
- Match hash algorithm to signature algorithm (ECDSA P-256 → SHA-256)
- Document this in code comments
- Test with real hardware (simulation won't catch this)

**Warning signs:**
- `sign_data()` called with original message bytes
- Signature verification fails with "invalid signature"
- Different behavior between software tests and hardware tests

**Verification:**
```rust
#[test]
#[ignore] // Requires real YubiKey
fn test_signing_uses_digest() {
    let backend = YubiKeyBackend::new().unwrap();
    let message = b"test message";

    // Should hash internally before calling YubiKey
    let result = backend.perform_operation(
        "9c",
        CryptoOperation::Sign {
            data: message.to_vec(),
            algorithm: SignatureAlgorithm::EcdsaP256,
        }
    );

    // Verify signature with public key
    assert!(result.is_ok());
}
```

### Pitfall 4: rcgen API Misuse (RemoteKeyPair Pattern)

**What goes wrong:** Using rcgen's `generate()` or `generate_simple_self_signed()` which creates software keypair instead of using hardware.

**Why it happens:** rcgen examples show self-signed software certs, hardware signing is advanced use case.

**How to avoid:**
- Never use `rcgen::generate()` or `generate_simple_self_signed()`
- Always use `CertificateParams` + `RemoteKeyPair`/`SigningKey` pattern
- Public key MUST come from hardware extraction (`piv_get_public_key()`)
- Test: parse generated certificate and verify signature with hardware public key

**Warning signs:**
- `rcgen::generate()` in backend code
- Certificate signed with `cert.serialize_der()` instead of custom signer
- No round-trip test (generate cert → parse → verify signature)

**Verification:**
```rust
#[test]
#[ignore] // Requires real YubiKey
fn test_certificate_hardware_backed() {
    let backend = YubiKeyBackend::new().unwrap();
    let cert_der = backend.generate_certificate(SlotId::Signature, "CN=Test").unwrap();

    // Parse certificate
    let cert = x509_cert::Certificate::from_der(&cert_der).unwrap();

    // Extract public key from certificate
    let cert_pubkey = cert.tbs_certificate.subject_public_key_info.to_der().unwrap();

    // Get public key from hardware
    let hw_pubkey = backend.piv_get_public_key(SlotId::Signature).unwrap();

    // Must match (proves certificate uses hardware key)
    assert_eq!(cert_pubkey, hw_pubkey);
}
```

### Pitfall 5: PIN Retry Exhaustion

**What goes wrong:** Unlimited PIN retry attempts lock YubiKey (3 retries then blocked, requires PUK to reset).

**Why it happens:** Development convenience, automatic retry loops, no retry limit checking.

**How to avoid:**
- Implement strict retry limit: 3 attempts maximum
- Track retry count in backend state
- Return `BackendError::HardwareError` after 3 failures with clear message
- Document PIN reset procedure in error message

**Warning signs:**
- PIN verification in loop without counter
- No retry limit checking
- Tests that keep retrying until success

**Verification:**
```rust
impl YubiKeyBackend {
    fn verify_pin_with_retry(&self, max_retries: u8) -> Result<(), BackendError> {
        let yubikey = self.ensure_connected()?;
        let pin = self.config.pin.as_ref()
            .ok_or_else(|| BackendError::InitializationFailed("PIN not configured".into()))?;

        for attempt in 1..=max_retries {
            match yubikey.verify_pin(pin.as_bytes()) {
                Ok(()) => return Ok(()),
                Err(e) if attempt < max_retries => {
                    eprintln!("PIN verification failed (attempt {}/{})", attempt, max_retries);
                    continue;
                }
                Err(e) => {
                    return Err(BackendError::HardwareError(
                        format!("PIN verification failed after {} attempts. YubiKey may be locked. Reset with PUK or contact administrator.", max_retries)
                    ));
                }
            }
        }
        unreachable!()
    }
}
```

### Pitfall 6: Slot ID String Parsing Without Validation

**What goes wrong:** Accepting any string as slot ID without validation. Causes cryptic hardware errors.

**Why it happens:** Simple string-based key_id API, no type safety at interface boundary.

**How to avoid:**
- Validate slot ID strings against known PIV slots (9a, 9c, 9d, 9e)
- Return `BackendError::KeyNotFound` with helpful message listing valid slots
- Consider using enum internally: `pub enum PivSlotId { Auth, Sig, Mgmt, Card }`

**Warning signs:**
- Slot parsing that accepts any string
- Errors like "CKR_OBJECT_HANDLE_INVALID" without user-friendly message
- No documentation of valid slot values

**Verification:**
```rust
#[test]
fn test_slot_validation() {
    let backend = YubiKeyBackend::new().unwrap();

    // Valid slots should parse
    assert!(backend.parse_slot("9a").is_ok());
    assert!(backend.parse_slot("9c").is_ok());

    // Invalid slots should return clear error
    let err = backend.parse_slot("invalid").unwrap_err();
    assert!(matches!(err, BackendError::KeyNotFound(_)));
    assert!(format!("{}", err).contains("9a") || format!("{}", err).contains("valid slots"));
}
```

### Pitfall 7: PKCS#11 Error Code Mapping Gaps

**What goes wrong:** Generic "operation failed" errors instead of user-actionable messages.

**Why it happens:** PKCS#11 returns numeric error codes (CKR_*) that need human-readable mapping.

**How to avoid:**
- Map common PKCS#11 errors to BackendError variants with helpful messages
- Include troubleshooting hints in error messages
- Document error code meanings in code comments

**Common error codes:**
- `CKR_PIN_INCORRECT` → "Incorrect PIN. X retries remaining."
- `CKR_DEVICE_REMOVED` → "YubiKey removed. Insert device and retry."
- `CKR_FUNCTION_NOT_SUPPORTED` → "Operation not supported by YubiKey firmware version."
- `CKR_SLOT_ID_INVALID` → "Invalid PIV slot. Use 9a, 9c, 9d, or 9e."

Source: PKCS#11 specification, [OpenSC issues](https://github.com/OpenSC/OpenSC/issues)

**Warning signs:**
- All errors map to generic `BackendError::OperationFailed`
- Error messages contain numeric codes without explanation
- No troubleshooting guidance in error messages

## Code Examples

### Complete Backend Skeleton

```rust
//! YubiKey PIV backend for Universal Backend system

#[cfg(feature = "yubikey")]
use crate::backends::traits::{BackendInfo, KeyMetadata};
use crate::backends::universal::{
    AsymmetricAlgorithm, BackendCapabilities, CryptoOperation, CryptoResult,
    SignatureAlgorithm, UniversalBackend,
};
use crate::error::BackendError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[cfg(feature = "yubikey")]
use yubikey::{YubiKey, piv::{SlotId, AlgorithmId}};

/// YubiKey backend configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YubiKeyConfig {
    pub pin: Option<String>,
    pub default_slot: String,
    pub verbose: bool,
}

impl Default for YubiKeyConfig {
    fn default() -> Self {
        Self {
            pin: None,
            default_slot: "9c".to_string(),  // Signature slot
            verbose: false,
        }
    }
}

/// YubiKey PIV backend
#[cfg(feature = "yubikey")]
pub struct YubiKeyBackend {
    config: YubiKeyConfig,
    yubikey: Option<YubiKey>,
    key_cache: HashMap<String, Vec<u8>>,  // Cached public keys
}

#[cfg(feature = "yubikey")]
impl YubiKeyBackend {
    pub fn new() -> Result<Self, BackendError> {
        Self::with_config(YubiKeyConfig::default())
    }

    pub fn with_config(config: YubiKeyConfig) -> Result<Self, BackendError> {
        let mut backend = Self {
            config,
            yubikey: None,
            key_cache: HashMap::new(),
        };
        backend.connect_if_available()?;
        Ok(backend)
    }

    fn connect_if_available(&mut self) -> Result<(), BackendError> {
        match YubiKey::open() {
            Ok(yk) => {
                if self.config.verbose {
                    println!("✔ YubiKey detected");
                }
                self.yubikey = Some(yk);
                Ok(())
            }
            Err(e) => {
                if self.config.verbose {
                    eprintln!("⚠ YubiKey not detected: {}", e);
                }
                Ok(())  // Not fatal - backend_info() will report unavailable
            }
        }
    }

    fn ensure_connected(&self) -> Result<&YubiKey, BackendError> {
        self.yubikey.as_ref().ok_or_else(|| {
            BackendError::HardwareError(
                "YubiKey not connected. Insert device and retry.".into()
            )
        })
    }

    fn parse_slot(&self, key_id: &str) -> Result<SlotId, BackendError> {
        // Implementation as shown in Pattern 2
        todo!()
    }

    fn piv_sign(&self, slot: SlotId, data: &[u8], algorithm: SignatureAlgorithm)
        -> Result<Vec<u8>, BackendError>
    {
        // Implementation as shown in Pattern 3
        todo!()
    }

    fn piv_get_public_key(&self, slot: SlotId) -> Result<Vec<u8>, BackendError> {
        // Implementation as shown in Pattern 4
        todo!()
    }

    fn generate_certificate(&self, slot: SlotId, subject: &str)
        -> Result<Vec<u8>, BackendError>
    {
        // Implementation as shown in Pattern 5
        todo!()
    }
}

#[cfg(feature = "yubikey")]
impl UniversalBackend for YubiKeyBackend {
    // Implementation as shown in Pattern 6
    fn perform_operation(&self, key_id: &str, operation: CryptoOperation)
        -> Result<CryptoResult, BackendError>
    {
        todo!()
    }

    fn supports_operation(&self, operation: &CryptoOperation) -> bool {
        todo!()
    }

    fn get_capabilities(&self) -> BackendCapabilities {
        todo!()
    }

    fn backend_info(&self) -> BackendInfo {
        todo!()
    }

    fn list_keys(&self) -> Result<Vec<KeyMetadata>, BackendError> {
        todo!()
    }
}
```

### Error Conversion Helper

```rust
/// Convert yubikey crate errors to BackendError
fn yubikey_error_to_backend(e: yubikey::Error) -> BackendError {
    match e {
        yubikey::Error::NotFound => BackendError::HardwareError(
            "YubiKey not found. Insert device and retry.".into()
        ),
        yubikey::Error::AuthenticationError => BackendError::HardwareError(
            "PIN verification failed. Check PIN and retry.".into()
        ),
        yubikey::Error::GenericError(msg) => BackendError::HardwareError(msg),
        _ => BackendError::OperationFailed(format!("YubiKey operation failed: {}", e)),
    }
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Manual DER encoding (1,000+ lines) | rcgen library | v1.1 rewrite | Eliminates 15+ error-prone ASN.1 functions |
| PKCS#11 via pkcs11 crate | yubikey crate native PIV API | v1.1 rewrite | Simpler implementation, better errors, YubiKey-specific |
| `untested` feature flag | Stable API only | v1.1 rewrite | Production-ready, security-auditable code |
| Silent software fallbacks | Fail-closed design | v1.1 rewrite | Honest security guarantees, no false claims |
| Placeholder certificates | Real hardware operations | v1.1 rewrite | Every key/signature from actual crypto |
| 3,263-line monolith | 500-800 line focused backend | v1.1 rewrite | Maintainable, auditable, follows SRP |

**Deprecated/outdated:**
- Manual ASN.1/DER encoding: Replaced by rcgen + der/spki crates
- `yubikey` crate with `untested` feature: Using stable API only
- PKCS#11 as primary interface: Using native yubikey crate PIV API
- Silent fallback patterns: Removed in favor of fail-closed errors

## Open Questions

### 1. rcgen RemoteKeyPair vs SigningKey API

**What we know:** rcgen v0.14 renamed RemoteKeyPair to SigningKey

**What's unclear:** Exact API signature for signing callback in current version

**Recommendation:** Verify during implementation. Check rcgen docs for latest callback pattern. May be `Box<dyn Fn(&[u8]) -> Result<Vec<u8>>>` or trait implementation.

**Confidence:** MEDIUM - API exists but exact signature needs verification

### 2. YubiKey PIV vs PKCS#11 API Trade-offs

**What we know:** Current backend uses PKCS#11. Research recommends native PIV API.

**What's unclear:** Any operations that require PKCS#11 that PIV API doesn't support?

**Recommendation:** Start with native PIV API. Add PKCS#11 fallback only if specific operation requires it. Document in code why PKCS#11 needed for that operation.

**Confidence:** HIGH - PIV API covers all requirements (sign, get public key, generate, attest)

### 3. Ed25519 Support via PIV

**What we know:** Requirements mention Ed25519 signing (BACK-04). YubiKey PIV supports ECDSA P-256 and RSA.

**What's unclear:** Does YubiKey PIV support Ed25519? Or is this software-only?

**Recommendation:** Investigate during implementation. If PIV doesn't support Ed25519, document as limitation and suggest ECDSA P-256 alternative. Ed25519 can be software-backed via software_hsm.rs.

**Confidence:** LOW - Training data suggests PIV doesn't support Ed25519, but needs verification

Source: [YubiKey PIV Introduction](https://developers.yubico.com/PIV/Introduction/Certificate_slots.html) lists ECDSA P-256/P-384 and RSA, no Ed25519 mentioned.

### 4. Attestation Certificate Chain Format

**What we know:** BACK-08 requires attestation. yubikey crate has `attest()` API.

**What's unclear:** What format does attestation return? Full chain or single certificate?

**Recommendation:** Test with real hardware during implementation. Document attestation response format in code comments.

**Confidence:** MEDIUM - API exists, format needs verification

## Sources

### Primary (HIGH confidence)

- **Local codebase:**
  - `/home/john/vault/projects/github.com/trustedge/crates/core/src/backends/universal.rs` - UniversalBackend trait definition
  - `/home/john/vault/projects/github.com/trustedge/crates/core/src/error.rs` - BackendError hierarchy
  - `/home/john/vault/projects/github.com/trustedge/crates/core/src/backends/software_hsm.rs` - Reference implementation (1,419 lines)
  - `/home/john/vault/projects/github.com/trustedge/.planning/phases/09-cleanup/09-RESEARCH.md` - Phase 9 deletion scope
  - `/home/john/vault/projects/github.com/trustedge/.planning/research/ARCHITECTURE-YUBIKEY.md` - YubiKey architecture research
  - `/home/john/vault/projects/github.com/trustedge/.planning/research/PITFALLS.md` - Common pitfalls analysis
  - `/home/john/vault/projects/github.com/trustedge/.planning/research/STACK.md` - Technology stack decisions

### Secondary (MEDIUM confidence)

- [yubikey crate documentation](https://docs.rs/yubikey/) - Rust YubiKey driver API
- [rcgen GitHub releases](https://github.com/rustls/rcgen/releases) - Certificate generation library updates
- [spki crate documentation](https://docs.rs/spki/) - SubjectPublicKeyInfo encoding (RFC 5280)
- [Yubico PIV Certificate Slots](https://developers.yubico.com/PIV/Introduction/Certificate_slots.html) - Official PIV slot documentation
- [YubiKey PIV Introduction](https://developers.yubico.com/yubico-piv-tool/YubiKey_PIV_introduction.html) - PIV applet overview
- [iqlusioninc/yubikey.rs GitHub](https://github.com/iqlusioninc/yubikey.rs) - Rust YubiKey driver source

### Tertiary (LOW confidence)

- Web searches for "yubikey rust crate PIV sign_data" - Found general information, no specific code examples
- Web searches for "rcgen RemoteKeyPair SigningKey" - Found v0.14 API change, need verification of current API
- Web searches for "PKCS11 error codes" - Found common error codes but no comprehensive Rust mapping

## Metadata

**Confidence breakdown:**
- Standard stack: MEDIUM-HIGH - yubikey/rcgen versions from training data, need verification
- Architecture patterns: HIGH - Based on existing codebase analysis and PIV standard
- Pitfalls: HIGH - Directly from v1.0 codebase bugs and established patterns
- Code examples: MEDIUM - Patterns verified in software_hsm.rs, YubiKey specifics need testing
- API details: MEDIUM - Training data + official docs, exact APIs need verification during implementation

**Research date:** 2026-02-11
**Valid until:** 30 days (stable domain - YubiKey PIV standard and Rust crypto ecosystem)

## Planning Guidance

**For the planner:**

This phase implements a production-quality backend following the established Universal Backend architecture. The implementation is straightforward given the reference implementation (software_hsm.rs) and clear requirements.

Break into focused tasks:

1. **Backend structure** - YubiKeyBackend struct, config, initialization, connection management
2. **PIV operations** - Signing, public key extraction, key generation using yubikey crate
3. **Certificate generation** - rcgen integration with RemoteKeyPair/SigningKey pattern
4. **UniversalBackend implementation** - Trait methods, capability reporting, error handling
5. **Error handling** - PKCS#11 error mapping, fail-closed patterns, user-friendly messages
6. **Testing** - Hardware tests (ignored by default), simulation tests, round-trip validation

**Success criteria:**
- BACK-01 to BACK-14 requirements met
- Zero manual cryptographic operations (all via libraries)
- Fail-closed design (errors when hardware unavailable, no fallbacks)
- 500-800 lines total (comparable to software_hsm.rs)
- Compiles with `--features yubikey`
- All tests pass (simulation tests in CI, hardware tests manual)

**Risk mitigation:**
- Reference software_hsm.rs for patterns and structure
- Test incrementally (each operation independently)
- Use existing error types (BackendError variants already defined)
- Follow existing code style and documentation standards
- Hardware tests marked `#[ignore]` for CI compatibility
