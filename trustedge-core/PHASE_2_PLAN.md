<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->

# Phase 2 Plan: Real X.509 Certificate Generation with x509-cert Crate

## Overview

Building on the successful Phase 1 implementation, Phase 2 focuses on implementing **proper X.509 certificate generation** using the `x509-cert` crate and enhancing the YubiKey public key extraction to use real hardware PKCS#11 calls.

## Phase 2 Goals

### 🎯 Primary Objectives

1. **Real PKCS#11 Public Key Extraction**
   - Replace placeholder implementation with actual PKCS#11 C_GetAttributeValue calls
   - Extract real EC parameters and points from YubiKey PIV slots
   - Add RSA modulus/exponent extraction support
   - Support multiple key types (ECDSA P-256, P-384, RSA 2048/4096)

2. **Proper X.509 Certificate Generation**
   - Integrate `x509-cert` crate for standards-compliant certificate creation
   - Generate self-signed certificates for QUIC/TLS authentication
   - Support certificate extensions (Subject Alternative Name, Key Usage, etc.)
   - Implement proper certificate validity periods and serial numbers

3. **Enhanced Hardware Integration**
   - Certificate signing using YubiKey private keys
   - Hardware attestation integration for certificate trustworthiness
   - Support for certificate chains and CA functionality

## Technical Implementation Plan

### 2.1 Real PKCS#11 Public Key Extraction

**Target**: Replace `build_placeholder_ecdsa_p256_spki()` with real hardware extraction

```rust
// Enhanced extract_public_key implementation
impl YubiKeyBackend {
    pub fn extract_public_key(&self, key_id: &str) -> Result<Vec<u8>> {
        let key_handle = self.find_public_key_by_id(key_id)?;
        let key_type = self.get_key_type(key_handle)?;
        
        match key_type {
            KeyType::EcdsaP256 => self.extract_ecdsa_public_key(key_handle),
            KeyType::EcdsaP384 => self.extract_ecdsa_public_key(key_handle),
            KeyType::Rsa2048 | KeyType::Rsa4096 => self.extract_rsa_public_key(key_handle),
        }
    }
}
```

**Implementation Steps:**
1. Implement `find_public_key_by_id()` with real PKCS#11 object search
2. Add `get_key_type()` using CKA_KEY_TYPE attribute extraction
3. Create `extract_ecdsa_public_key()` with CKA_EC_PARAMS and CKA_EC_POINT
4. Implement `extract_rsa_public_key()` with CKA_MODULUS and CKA_PUBLIC_EXPONENT
5. Build proper SubjectPublicKeyInfo using extracted parameters

### 2.2 X.509 Certificate Generation with x509-cert

**Target**: Generate proper X.509 certificates using extracted public keys

```rust
use x509_cert::{
    builder::{CertificateBuilder, Profile},
    name::Name,
    serial_number::SerialNumber,
    time::Validity,
};

impl YubiKeyBackend {
    pub fn generate_x509_certificate(
        &self,
        key_id: &str,
        params: CertificateParams,
    ) -> Result<HardwareCertificate> {
        // Extract real public key from hardware
        let public_key_der = self.extract_public_key(key_id)?;
        let subject_public_key_info = SubjectPublicKeyInfo::from_der(&public_key_der)?;
        
        // Create X.509 certificate
        let cert = CertificateBuilder::new(
            Profile::Leaf,
            SerialNumber::from(self.generate_serial_number()?),
            Validity::from_now(Duration::from_secs(params.validity_days as u64 * 24 * 3600))?,
            Name::from_str(&params.subject)?,
            subject_public_key_info,
            &signing_key, // Hardware signing key
        )?.build()?;
        
        Ok(HardwareCertificate {
            certificate_der: cert.to_der()?,
            attestation_proof: self.generate_attestation_proof(key_id)?,
            key_id: key_id.to_string(),
            subject: params.subject,
        })
    }
}
```

### 2.3 Enhanced Certificate Features

**Advanced X.509 Features:**
1. **Subject Alternative Names (SAN)**
   - DNS names for QUIC server certificates
   - IP addresses for local network authentication
   - Custom extensions for hardware attestation

2. **Key Usage Extensions**
   - Digital Signature for authentication
   - Key Encipherment for TLS
   - Certificate Sign for CA functionality

3. **Certificate Chains**
   - Root CA certificate generation
   - Intermediate CA support
   - End-entity certificate signing

## Implementation Phases

### Phase 2A: Real PKCS#11 Integration (Week 1)

**Deliverables:**
- [ ] Real `find_public_key_by_id()` implementation
- [ ] Hardware key type detection (`get_key_type()`)
- [ ] ECDSA public key extraction with real EC parameters
- [ ] RSA public key extraction with modulus/exponent
- [ ] Enhanced error handling and hardware detection

**Acceptance Criteria:**
- Extract real public keys from actual YubiKey hardware
- Support multiple key types (ECDSA P-256, P-384, RSA 2048/4096)
- Maintain backward compatibility with Phase 1 API
- Comprehensive testing with real hardware

### Phase 2B: X.509 Certificate Generation (Week 2)

**Deliverables:**
- [ ] `x509-cert` crate integration
- [ ] Proper certificate generation with extracted public keys
- [ ] Certificate signing using YubiKey private keys
- [ ] Subject Alternative Name (SAN) support
- [ ] Certificate validation and verification

**Acceptance Criteria:**
- Generate standards-compliant X.509 certificates
- Self-signed certificates for QUIC/TLS authentication
- Hardware-backed certificate signing
- Certificate chain validation

### Phase 2C: Advanced Features (Week 3)

**Deliverables:**
- [ ] Certificate Authority (CA) functionality
- [ ] Certificate chain generation
- [ ] Enhanced hardware attestation
- [ ] QUIC/TLS integration testing
- [ ] Performance optimization

**Acceptance Criteria:**
- End-to-end QUIC authentication with hardware certificates
- CA certificate generation and intermediate signing
- Hardware attestation proof integration
- Production-ready performance

## Technical Architecture Updates

### Enhanced Backend Structure

```
YubiKeyBackend (Phase 2)
├── Hardware Key Extraction
│   ├── find_public_key_by_id() - Real PKCS#11 object search
│   ├── get_key_type() - CKA_KEY_TYPE attribute extraction
│   ├── extract_ecdsa_public_key() - EC parameters + point
│   └── extract_rsa_public_key() - Modulus + exponent
├── X.509 Certificate Generation
│   ├── generate_x509_certificate() - x509-cert integration
│   ├── sign_certificate() - Hardware private key signing
│   ├── generate_ca_certificate() - CA functionality
│   └── build_certificate_chain() - Multi-level certificates
└── Enhanced Features
    ├── subject_alternative_names() - SAN extension support
    ├── certificate_validation() - Chain verification
    └── hardware_attestation() - Trust anchoring
```

### QUIC Integration Enhancement

```
QUIC Transport (Phase 2)
├── HardwareBackedVerifier
│   ├── Real X.509 certificate validation
│   ├── Hardware attestation verification
│   └── Certificate chain validation
├── HardwareCertificateProvider
│   ├── On-demand certificate generation
│   ├── Certificate caching and renewal
│   └── Multiple certificate support
└── Enhanced Security
    ├── Hardware-backed TLS handshake
    ├── Certificate transparency logging
    └── Revocation checking (OCSP)
```

## Dependencies and Requirements

### New Crate Dependencies

```toml
# Enhanced Cargo.toml additions
[dependencies]
x509-cert = { version = "0.2", features = ["builder", "pem"] }
der = { version = "0.7", features = ["alloc", "pem"] }
spki = { version = "0.7", features = ["alloc", "pem"] }
signature = { version = "2.2", features = ["alloc"] }
const-oid = "0.9"  # For additional OID support
```

### System Requirements

- **PKCS#11 Module**: OpenSC or YubiKey PKCS#11 driver
- **Hardware**: YubiKey 4/5 with PIV support
- **OS Support**: Linux, macOS, Windows (PKCS#11 compatible)

## Testing Strategy

### Phase 2A Testing: Real Hardware Integration

```rust
#[cfg(test)]
mod hardware_tests {
    #[test]
    fn test_real_ecdsa_extraction() {
        let backend = YubiKeyBackend::new()?;
        let public_key = backend.extract_public_key("9a")?; // PIV SIGN slot
        assert!(public_key.len() > 90); // Real key should be proper size
        validate_der_structure(&public_key)?;
    }

    #[test]
    fn test_multiple_key_types() {
        // Test ECDSA P-256, P-384, RSA 2048, RSA 4096
    }
}
```

### Phase 2B Testing: X.509 Certificates

```rust
#[cfg(test)]
mod certificate_tests {
    #[test]
    fn test_x509_generation() {
        let cert = backend.generate_x509_certificate("9a", params)?;
        let parsed = Certificate::from_der(&cert.certificate_der)?;
        assert_eq!(parsed.tbs_certificate.subject, expected_subject);
    }

    #[test]
    fn test_certificate_signing() {
        // Test hardware-backed certificate signing
    }
}
```

### Phase 2C Testing: End-to-End Integration

```rust
#[cfg(test)]
mod integration_tests {
    #[test]
    async fn test_quic_hardware_auth() {
        // End-to-end QUIC connection with hardware certificates
    }

    #[test]
    fn test_certificate_chain_validation() {
        // Multi-level certificate chain verification
    }
}
```

## Risk Assessment and Mitigation

| Risk | Impact | Probability | Mitigation |
|------|--------|-------------|------------|
| PKCS#11 API variations | Medium | Low | Extensive testing with OpenSC |
| x509-cert crate compatibility | High | Low | Version pinning and testing |
| Hardware availability | Low | Medium | Graceful fallback to Phase 1 |
| Performance overhead | Medium | Medium | Caching and optimization |

## Success Metrics

### Phase 2A Metrics
- [ ] Extract real public keys from YubiKey hardware
- [ ] Support 4+ key types (ECDSA P-256/384, RSA 2048/4096)
- [ ] <100ms public key extraction time
- [ ] 100% compatibility with OpenSC PKCS#11

### Phase 2B Metrics
- [ ] Generate valid X.509 certificates per RFC 5280
- [ ] Hardware-signed certificates with YubiKey private keys
- [ ] Certificate validation passes with standard tools
- [ ] <500ms certificate generation time

### Phase 2C Metrics
- [ ] End-to-end QUIC authentication working
- [ ] Certificate chain validation functional
- [ ] Hardware attestation integration complete
- [ ] Production-ready performance (<1s full flow)

## Deliverables Timeline

**Week 1 (Phase 2A)**: Real PKCS#11 Integration
- Monday-Tuesday: PKCS#11 object search and key type detection
- Wednesday-Thursday: ECDSA and RSA public key extraction
- Friday: Testing and validation with real hardware

**Week 2 (Phase 2B)**: X.509 Certificate Generation
- Monday-Tuesday: x509-cert crate integration
- Wednesday-Thursday: Certificate generation and signing
- Friday: Certificate validation and testing

**Week 3 (Phase 2C)**: Advanced Features and Integration
- Monday-Tuesday: CA functionality and certificate chains
- Wednesday-Thursday: QUIC integration and attestation
- Friday: Performance optimization and documentation

## Next Steps

1. **Review and Approval**: Review Phase 2 plan and approve scope
2. **Environment Setup**: Ensure YubiKey hardware available for testing
3. **Phase 2A Start**: Begin real PKCS#11 integration implementation
4. **Checkpoint Reviews**: Weekly progress reviews and plan adjustments

---
*Ready to proceed with Phase 2A: Real PKCS#11 Integration*
