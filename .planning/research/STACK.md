# Technology Stack: YubiKey Backend Rewrite

**Project:** TrustEdge YubiKey backend rewrite
**Researched:** 2026-02-11
**Overall Confidence:** MEDIUM (training data + codebase analysis, external docs verification restricted)

## Executive Summary

The YubiKey backend rewrite requires specific stable dependencies for hardware security module integration. Current implementation uses `yubikey` 0.7 with the `untested` feature flag, which must be eliminated. The rewrite will use stable PIV operations, rcgen for X.509 certificate generation, and maintain PKCS#11 integration for hardware operations.

**Key constraint:** NO manual cryptographic operations. Only battle-tested libraries following NIST standards.

---

## Recommended Stack Changes

### Dependencies to REPLACE

| Current | Version | Issue | Replacement Strategy |
|---------|---------|-------|---------------------|
| `yubikey = { version = "0.7", features = ["untested"] }` | 0.7 | Using unstable API surface | Use stable API only (remove `untested` feature) |
| Manual DER encoding (lines 95-150 in yubikey.rs) | N/A | Handwritten ASN.1 encoder | Replace with `rcgen` for X.509 generation |
| `x509-cert = "0.2"` (with manual builder) | 0.2 | Low-level certificate construction | Migrate to `rcgen` high-level API |

### Dependencies to ADD

| Library | Version | Purpose | Rationale |
|---------|---------|---------|-----------|
| `rcgen` | ~0.13 | X.509 certificate generation | HIGH: Industry-standard cert builder, eliminates manual DER encoding, NIST-compliant |

**Confidence:** MEDIUM - rcgen version based on training data (January 2025). Actual latest version should be verified with external sources.

### Dependencies to KEEP (Version-Locked)

| Library | Current Version | Purpose | Why Keep | Lock Strategy |
|---------|----------------|---------|----------|---------------|
| `pkcs11` | 0.5 | PKCS#11 interface for HSM | Core hardware integration | Exact pin `=0.5.0` |
| `yubikey` | 0.7 | YubiKey PIV operations | Stable API surface (without `untested`) | Exact pin `=0.7.0` |
| `der` | 0.7 | DER encoding utilities | Required for PKCS#11 key material | Compatible range `0.7` |
| `spki` | 0.7 | Subject Public Key Info | Public key serialization | Compatible range `0.7` |
| `signature` | 2.2 | Signature trait abstractions | Standard crypto traits | Compatible range `2` |

**Rationale for Version Locking:** Hardware integration is fragile. Pitfall #11 from PITFALLS.md warns that dependency version conflicts break YubiKey backends. Using exact versions for critical hardware deps prevents transitive dependency conflicts.

---

## YubiKey Crate: Stable API Surface

### What's Available WITHOUT `untested` Feature

**Confidence:** LOW - Based on training data and codebase patterns. Official docs verification blocked.

The `yubikey` crate stable API (0.7) provides:

| Operation | Stable API | Notes |
|-----------|-----------|-------|
| **YubiKey Detection** | `YubiKey::open()` | Enumerate and open connected devices |
| **PIV Operations** | `piv` module | Card authentication, slot management |
| **Certificate Access** | `Certificate::read()` | Read existing certificates from slots |
| **Attestation** | Attestation APIs | Hardware attestation certificates |

### What Requires `untested` Feature (AVOID)

Based on codebase analysis (current implementation uses `untested`), the following are likely gated:

- Low-level PIV command construction
- Direct APDU commands to card
- Experimental key generation algorithms
- Internal PIV state manipulation

**Rewrite Strategy:** Use ONLY stable PIV operations. If functionality requires `untested`, re-evaluate if it's actually necessary for the Universal Backend contract.

### Integration with Universal Backend Trait

The Universal Backend defines these operations (from `universal.rs` lines 88-135):

```rust
pub enum CryptoOperation {
    Sign { data: Vec<u8>, algorithm: SignatureAlgorithm },
    Verify { data: Vec<u8>, signature: Vec<u8>, algorithm: SignatureAlgorithm },
    GenerateKeyPair { algorithm: AsymmetricAlgorithm },
    GetPublicKey,
    Attest { challenge: Vec<u8> },
    // ... (symmetric ops delegated to software)
}
```

**YubiKey Stable API Mapping:**

| Backend Operation | YubiKey Stable Implementation |
|------------------|-------------------------------|
| `Sign` | PIV slot signing with `piv::sign_data()` |
| `GenerateKeyPair` | PIV key generation in slot 9a (Authentication) |
| `GetPublicKey` | Certificate read + public key extraction |
| `Attest` | Read attestation certificate chain |
| `Verify` | Software verification (not hardware) |

**Confidence:** MEDIUM - Based on PIV standard patterns and existing codebase structure.

---

## rcgen: X.509 Certificate Generation

### Why rcgen Over Manual DER Encoding

**Current Problem (lines 95-200 in yubikey.rs):**
- 150+ lines of hand-written ASN.1 INTEGER encoding
- Manual SEQUENCE construction for ECDSA signatures
- Risk of encoding errors breaking certificate validation

**rcgen Solution:**
- High-level API: `Certificate::from_params()`
- Automatic DER encoding following X.509 standards
- NIST-compliant certificate generation
- Used by rustls, trust-dns, other production systems

### rcgen API Surface

**Confidence:** MEDIUM - Based on training data (rcgen 0.10-0.13 range).

```rust
use rcgen::{Certificate, CertificateParams, KeyPair};

// 1. External signing (for hardware keys)
let params = CertificateParams::new(vec!["device.example.com".to_string()]);
let cert = Certificate::from_params(params)?;

// 2. External key pair (YubiKey holds private key)
// Use SignatureAlgorithm trait to integrate hardware signing

// 3. Self-signed certificates
let cert = Certificate::from_params(params)?;
let pem = cert.serialize_pem()?;
```

### Integration with YubiKey

**Pattern for Hardware-Backed Certificates:**

```rust
// 1. Generate key pair IN YubiKey (PIV slot 9a)
let public_key = yubikey_backend.generate_key_pair(AsymmetricAlgorithm::EcdsaP256)?;

// 2. Build certificate params
let mut params = CertificateParams::new(vec!["yubikey-device".to_string()]);
params.key_pair = Some(/* Convert YubiKey public key to rcgen KeyPair */);

// 3. Sign certificate using YubiKey
// rcgen supports external signers via custom KeyPair implementations
let cert = Certificate::from_params(params)?;
```

**Open Question:** Exact rcgen API for external hardware signing. May require implementing `rcgen::KeyPair` trait or using `serialize_der_with_signer()` if available.

**Confidence:** LOW - API details need verification with current rcgen documentation.

---

## PKCS#11 Integration

### Current State

- `pkcs11 = "0.5"` - PKCS#11 bindings for Rust
- Used for YubiKey PIV operations via smart card interface
- Requires PCSC daemon (pcscd) on Linux/macOS

### Keep vs Replace

**Decision: KEEP `pkcs11` 0.5**

**Rationale:**
1. Stable API for YubiKey PIV access
2. Industry-standard smart card interface
3. No breaking changes anticipated
4. Hardware integration already working (90+ simulation tests passing)

### PKCS#11 Usage Patterns

**From YUBIKEY-MANUAL-PROTOCOL.md (lines 15-22):**

```
YubiKey Integration Layers:
1. PCSC daemon (pcscd) - Smart card interface
2. PKCS#11 library - Cryptographic token interface
3. YubiKey PIV - Personal Identity Verification applet
4. Universal Backend trait - TrustEdge abstraction
```

### Critical PKCS#11 Initialization Sequence

**From PITFALLS.md (#11, lines 786-795):**

```rust
/// CRITICAL: Must be called before any PKCS#11 operations.
///
/// 1. Load PKCS#11 library (OpenSC or YubiKey Manager)
/// 2. Initialize PKCS#11 context
/// 3. Open session to slot 0
/// 4. Login with PIN
```

**Rewrite Must Preserve:**
- Initialization order (documented in code comments)
- Error handling for hardware-not-present
- PIN retry limit handling (3 attempts before block)
- Session management (open/close patterns)

---

## Additional Dependencies Analysis

### Keep Current Versions

| Dependency | Version | Purpose | Notes |
|------------|---------|---------|-------|
| `der` | 0.7 | DER encoding/decoding | Low-level, stable API |
| `spki` | 0.7 | SubjectPublicKeyInfo structures | PKCS#8 public key format |
| `signature` | 2.2 | Signature trait | RustCrypto ecosystem standard |

**Rationale:** These are low-level primitives with stable APIs. No changes needed unless rcgen brings in conflicting versions.

### Dependencies to REMOVE

| Dependency | Why Remove |
|------------|------------|
| Manual ASN.1 encoding functions (lines 95-150) | Replaced by rcgen |
| `x509-cert` builder usage | Migrating to rcgen high-level API |

**Exception:** May keep `x509-cert` if rcgen requires it as transitive dependency, but stop using it directly.

---

## Installation

### Workspace Cargo.toml Changes

**NO CHANGES** - Workspace already has shared dependencies defined.

### Core Crate Cargo.toml Changes

```toml
[dependencies]
# ... existing deps ...

# YubiKey hardware backend (feature-gated)
pkcs11 = { version = "=0.5.0", optional = true }  # Exact version lock
yubikey = { version = "=0.7.0", optional = true }  # NO untested feature
rcgen = { version = "0.13", optional = true }     # NEW: X.509 generation

# Certificate/key utilities (keep current versions)
der = { version = "0.7", optional = true }
spki = { version = "0.7", optional = true }
signature = { version = "2.2", optional = true }

# REMOVE x509-cert direct usage (if rcgen doesn't need it)
# x509-cert = { version = "0.2", features = ["builder"], optional = true }

[features]
yubikey = [
    "pkcs11",
    "dep:yubikey",
    "rcgen",      # NEW: Replace x509-cert with rcgen
    "der",
    "spki",
    "signature",
]
```

**Confidence:** HIGH - Based on codebase analysis and standard Cargo patterns.

---

## Alternatives Considered

### Alternative 1: Keep `untested` Feature

| Pro | Con |
|-----|-----|
| No API changes needed | Violates "stable API only" requirement |
| Already working | Unstable features may break in future |

**Decision: REJECT** - Milestone constraint explicitly forbids `untested` flag.

### Alternative 2: Use `x509-cert` Instead of rcgen

| Pro | Con |
|-----|-----|
| Lower-level control | Requires manual DER encoding (more error-prone) |
| Already in dependencies | Verbose API, higher complexity |

**Decision: REJECT** - rcgen eliminates manual crypto (DER encoding), follows milestone constraint.

### Alternative 3: yubico-piv-tool Bindings

| Pro | Con |
|-----|-----|
| Official Yubico library | C FFI complexity |
| Comprehensive PIV support | Less Rust-idiomatic |

**Decision: REJECT** - `yubikey` crate already provides Rust-native PIV operations.

---

## Build Requirements

### Platform Dependencies (No Changes)

**Linux:**
```bash
sudo apt install libpcsclite-dev pkgconf  # Debian/Ubuntu
sudo dnf install pcsc-lite-devel          # Fedora/RHEL
```

**macOS:**
- PCSC framework (built-in, no installation needed)

**Windows:**
- Smart card service (built-in)

### Runtime Requirements (No Changes)

- PCSC daemon (`pcscd`) must be running before YubiKey operations
- YubiKey 5 series with PIV applet enabled
- Default PIN: 123456 (production deployments should change)

---

## Version Compatibility Matrix

### Rust Ecosystem Compatibility

| Crate | Version | RustCrypto Ecosystem | NIST Standards |
|-------|---------|---------------------|----------------|
| `pkcs11` | 0.5 | Compatible | PKCS#11 v2.40 |
| `yubikey` | 0.7 (stable) | Uses `signature` 2.x | PIV SP 800-73-4 |
| `rcgen` | ~0.13 | Uses `signature`, `der`, `spki` | X.509 RFC 5280 |
| `der` | 0.7 | RustCrypto | ITU-T X.690 |
| `spki` | 0.7 | RustCrypto | RFC 5280 |
| `signature` | 2.2 | RustCrypto | Multiple (trait) |

**Confidence:** MEDIUM - Versions based on current codebase + training data. Compatibility should be verified during implementation.

### Expected Transitive Dependencies

rcgen likely brings in:
- `yasna` or `pem` for encoding
- `time` for certificate validity periods
- `ring` or `*-dalek` for signature verification (during cert generation)

**Action Item:** Run `cargo tree -p trustedge-core --features yubikey` after adding rcgen to check for conflicts.

---

## Risk Assessment

### High Risk

1. **rcgen version mismatch** - Training data may be outdated, actual latest version unknown
   - **Mitigation:** Verify rcgen version immediately during implementation

2. **YubiKey stable API insufficient** - Operations needed may require `untested`
   - **Mitigation:** Audit current `yubikey.rs` usage, verify all ops available in stable API

### Medium Risk

1. **rcgen external signer API** - May not support hardware-backed keys directly
   - **Mitigation:** Check rcgen docs for `KeyPair` trait implementation or `serialize_der_with_signer()`

2. **PKCS#11 version incompatibility** - Locking to 0.5.0 may conflict with other deps
   - **Mitigation:** Test with `cargo tree`, resolve conflicts before committing

### Low Risk

1. **Build time increase** - rcgen adds dependency tree
   - **Mitigation:** Acceptable tradeoff for eliminating manual crypto

---

## Integration with Existing Universal Backend

### No Changes Required to Trait

The `UniversalBackend` trait (from `universal.rs`) is stable and doesn't need modification.

### YubiKey Backend Implementation Changes

| Component | Current | After Rewrite |
|-----------|---------|---------------|
| Key generation | PIV + manual cert building | PIV + rcgen cert generation |
| Public key export | Manual DER encoding | rcgen DER serialization |
| Signing | PKCS#11 CKM_ECDSA | SAME (no change) |
| Attestation | X.509 parsing | rcgen parsing (if needed) |

---

## Sources

### Codebase Analysis (HIGH Confidence)

- `/home/john/vault/projects/github.com/trustedge/crates/core/Cargo.toml` - Current dependencies
- `/home/john/vault/projects/github.com/trustedge/crates/core/src/backends/yubikey.rs` - Implementation patterns
- `/home/john/vault/projects/github.com/trustedge/.planning/phases/08-validation/YUBIKEY-MANUAL-PROTOCOL.md` - Hardware integration requirements
- `/home/john/vault/projects/github.com/trustedge/.planning/research/PITFALLS.md` - Dependency locking rationale

### Training Data (MEDIUM-LOW Confidence)

- rcgen version (~0.13) - Training data from January 2025, should be verified
- YubiKey crate stable API surface - Based on typical Rust crate patterns
- PKCS#11 integration patterns - Industry-standard practices

### Standards Referenced (HIGH Confidence)

- NIST SP 800-73-4 (PIV Card Application)
- PKCS#11 v2.40 Specification
- RFC 5280 (X.509 Certificate Profile)
- ITU-T X.690 (DER Encoding)

---

## Open Questions for Implementation Phase

1. **rcgen exact version:** What is the actual latest stable version as of 2026-02-11?
2. **rcgen external signer API:** How to integrate hardware-backed key signing with rcgen?
3. **YubiKey stable API coverage:** Does stable API provide ALL operations needed for Universal Backend?
4. **Certificate validation:** Should we add `webpki` or similar for certificate chain validation?

**Recommendation:** Dedicate first implementation task to dependency verification and API compatibility testing.

---

## Success Criteria

Stack is validated when:

- [ ] rcgen version confirmed current and compatible
- [ ] YubiKey backend compiles WITHOUT `untested` feature
- [ ] All manual DER encoding removed
- [ ] X.509 certificate generation works with rcgen
- [ ] `cargo tree` shows no dependency conflicts
- [ ] 90+ simulation tests still pass
- [ ] Hardware signing operations work (manual protocol validation)
- [ ] Build time increase < 20% (acceptable for safety improvement)

**Final Confidence Assessment:**

| Area | Confidence | Reason |
|------|------------|--------|
| PKCS#11/YubiKey deps | HIGH | Existing codebase, stable usage |
| rcgen version | LOW | Training data may be outdated |
| rcgen API | MEDIUM | Standard patterns, but external signer needs verification |
| Integration approach | HIGH | Universal Backend trait is stable |
| Overall Stack | MEDIUM | Core deps solid, rcgen details need verification |

---

*Research completed: 2026-02-11*
*Researcher: GSD Project Research Agent*
*Next step: Implementation phase should verify rcgen version and API compatibility FIRST*
