# Phase 11: Test Infrastructure - Research

**Researched:** 2026-02-11
**Domain:** Rust testing patterns for hardware-backed cryptography
**Confidence:** HIGH

## Summary

Phase 11 requires building a comprehensive test suite for YubiKey PIV backend with two distinct test categories: simulation tests (no hardware, always run in CI) and hardware integration tests (require physical YubiKey, gated with #[ignore]). The core challenge is testing hardware-dependent code without physical devices while ensuring every test validates actual behavior with real assertions.

Rust provides robust testing infrastructure through `#[cfg(test)]`, `#[ignore]`, and integration test organization. The YubiKey backend (Phase 10) uses `yubikey` crate stable API, `rcgen` for X.509 certificate generation with RemoteKeyPair (hardware-backed signing), and `x509-cert` from RustCrypto for parsing/validation. The existing codebase demonstrates strong testing patterns (343 tests in v1.0, 160 in core) with comprehensive integration test examples for Software HSM backend.

**Primary recommendation:** Use trait-based abstraction for YubiKey operations to enable simulation tests without mocking frameworks. Implement simulation tests as unit tests in `yubikey.rs` (with `#[cfg(test)]`) for capability reporting, slot parsing, error mapping, and config validation. Implement hardware integration tests in `tests/yubikey_integration.rs` (with `#[ignore]`) for real signing, key extraction, and certificate round-trip validation. Every test must contain at least one `assert!`, `assert_eq!`, or `expect()` that validates actual output.

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| yubikey | 0.7 | YubiKey PIV operations | Official Yubico-maintained crate, stable API only (no `untested` features) |
| rcgen | 0.13 | X.509 certificate generation | Standard Rust X.509 library with RemoteKeyPair trait for hardware-backed signing |
| x509-cert | 0.2 | X.509 certificate parsing/validation | RustCrypto project, pure Rust, DER parsing, signature verification |
| der | 0.7 | DER encoding/decoding | RustCrypto dependency for x509-cert, used for SPKI parsing |
| spki | 0.7 | Subject Public Key Info | RustCrypto for extracting public keys from certificates |
| tempfile | 3.x | Temporary test directories | Standard Rust test utility (already in dev-dependencies) |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| criterion | 0.5 | Benchmarking | Already in dev-dependencies for performance tests (optional) |
| sha2 | 0.10 | SHA-256 hashing | Already in dependencies for digest computation |
| p256 | 0.13 | ECDSA P-256 operations | Already in workspace for signature verification |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Manual trait abstraction | mockall crate | mockall adds dependency and macro complexity; trait abstraction is zero-cost and sufficient for simulation tests |
| x509-cert | x509-parser | x509-parser has different API; x509-cert is RustCrypto standard and matches existing der/spki usage |
| Integration tests with #[ignore] | Separate test binary | #[ignore] is Rust standard pattern, simpler than custom test harnesses |

**Installation:**
```bash
# Already in Cargo.toml with yubikey feature:
cargo test --features yubikey           # Run simulation tests only
cargo test --features yubikey --ignored # Run hardware tests only
cargo test --features yubikey --include-ignored # Run all tests
```

## Architecture Patterns

### Recommended Project Structure
```
crates/core/src/backends/
├── yubikey.rs              # Backend implementation with simulation test module
│   └── #[cfg(test)] mod tests { ... }  # Unit tests for simulation

crates/core/tests/
├── yubikey_integration.rs  # Hardware integration tests with #[ignore]
└── common/
    └── mod.rs              # Shared test utilities
```

### Pattern 1: Simulation Tests via Internal Test Module
**What:** Unit tests in the same file as implementation, using `#[cfg(test)]` to exclude from release builds
**When to use:** Testing logic that doesn't require hardware (slot parsing, capability reporting, error mapping, config validation)
**Example:**
```rust
// Source: Rust official documentation + existing Software HSM tests
// File: crates/core/src/backends/yubikey.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_slot_valid_ids() {
        // Simulation test - no hardware required
        assert_eq!(YubiKeyBackend::parse_slot("9a").unwrap(), SlotId::Authentication);
        assert_eq!(YubiKeyBackend::parse_slot("9c").unwrap(), SlotId::Signature);
        assert_eq!(YubiKeyBackend::parse_slot("9d").unwrap(), SlotId::KeyManagement);
        assert_eq!(YubiKeyBackend::parse_slot("9e").unwrap(), SlotId::CardAuthentication);
    }

    #[test]
    fn test_parse_slot_invalid_ids() {
        // Anti-pattern test: invalid slot IDs must return errors
        let result = YubiKeyBackend::parse_slot("invalid");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), BackendError::KeyNotFound(_)));
    }

    #[test]
    fn test_capability_reporting_without_hardware() {
        // Simulation test - reports capabilities even when hardware unavailable
        let config = YubiKeyConfig::default();
        let backend = YubiKeyBackend::with_config(config).unwrap();

        let capabilities = backend.get_capabilities();

        // Assert actual capability values (not just call the function)
        assert!(capabilities.hardware_backed);
        assert!(capabilities.signature_algorithms.contains(&SignatureAlgorithm::EcdsaP256));
        assert!(capabilities.signature_algorithms.contains(&SignatureAlgorithm::RsaPkcs1v15));
        assert!(!capabilities.signature_algorithms.contains(&SignatureAlgorithm::Ed25519));
        assert_eq!(capabilities.max_key_size, Some(2048));
    }

    #[test]
    fn test_backend_info_hardware_unavailable() {
        // Simulation test - backend_info works without hardware
        let config = YubiKeyConfig::default();
        let backend = YubiKeyBackend::with_config(config).unwrap();

        let info = backend.backend_info();

        assert_eq!(info.name, "yubikey");
        assert_eq!(info.description, "YubiKey PIV hardware security backend");
        // available field will be false if no hardware present
    }
}
```

### Pattern 2: Hardware Integration Tests with #[ignore]
**What:** Integration tests in `tests/` directory requiring physical YubiKey, marked with `#[ignore]`
**When to use:** Testing real signing operations, key extraction, certificate generation, PIN verification
**Example:**
```rust
// Source: Rust official documentation + existing Software HSM integration tests
// File: crates/core/tests/yubikey_integration.rs

use trustedge_core::backends::yubikey::{YubiKeyBackend, YubiKeyConfig};
use trustedge_core::backends::universal::{CryptoOperation, CryptoResult, SignatureAlgorithm};

#[test]
#[ignore] // Requires physical YubiKey
fn test_real_signing_operation() {
    let config = YubiKeyConfig {
        pin: Some("123456".to_string()), // Test PIN
        default_slot: "9c".to_string(),
        verbose: true,
        max_pin_retries: 3,
    };

    let backend = YubiKeyBackend::with_config(config).expect("YubiKey not connected");

    // Test data
    let test_data = b"Hardware signing test data";

    // Perform real signing operation
    let sign_op = CryptoOperation::Sign {
        data: test_data.to_vec(),
        algorithm: SignatureAlgorithm::EcdsaP256,
    };

    let result = backend.perform_operation("9c", sign_op).expect("Signing failed");

    // Assert actual signature is returned (not placeholder)
    match result {
        CryptoResult::Signed(signature) => {
            assert!(!signature.is_empty(), "Signature must not be empty");
            assert!(signature.len() > 32, "ECDSA P-256 signature should be ~64-72 bytes");
        }
        _ => panic!("Expected Signed result"),
    }
}

#[test]
#[ignore] // Requires physical YubiKey
fn test_signing_fails_without_hardware() {
    // Disconnect or don't plug in YubiKey, verify fail-closed behavior
    // This would need a way to simulate hardware disconnection
    // OR run with hardware unplugged to verify error handling
}
```

### Pattern 3: Certificate Round-Trip Validation
**What:** Generate certificate with rcgen hardware-backed signing, parse with x509-cert, verify signature matches hardware public key
**When to use:** TEST-05 requirement - validating certificate generation end-to-end
**Example:**
```rust
// Source: rcgen docs + x509-cert docs + Phase 10 implementation
// File: crates/core/tests/yubikey_integration.rs

use x509_cert::Certificate;
use der::Decode;

#[test]
#[ignore] // Requires physical YubiKey
fn test_certificate_generation_round_trip() {
    let config = YubiKeyConfig {
        pin: Some("123456".to_string()),
        default_slot: "9c".to_string(),
        verbose: true,
        max_pin_retries: 3,
    };

    let backend = YubiKeyBackend::with_config(config).expect("YubiKey not connected");

    // Step 1: Generate certificate via rcgen with hardware-backed signing
    let cert_der = backend.generate_certificate("9c", "Test Certificate")
        .expect("Certificate generation failed");

    assert!(!cert_der.is_empty(), "Certificate DER must not be empty");

    // Step 2: Parse certificate with x509-cert
    let cert = Certificate::from_der(&cert_der)
        .expect("Failed to parse generated certificate");

    // Step 3: Extract public key from certificate
    let cert_public_key = cert.tbs_certificate.subject_public_key_info.subject_public_key.raw_bytes();

    // Step 4: Get public key from hardware slot
    let get_pubkey_op = CryptoOperation::GetPublicKey;
    let hardware_pubkey_result = backend.perform_operation("9c", get_pubkey_op)
        .expect("Failed to get public key from hardware");

    let hardware_pubkey_der = match hardware_pubkey_result {
        CryptoResult::PublicKey(key) => key,
        _ => panic!("Expected PublicKey result"),
    };

    // Parse hardware public key SPKI
    let hardware_spki = spki::SubjectPublicKeyInfoRef::try_from(hardware_pubkey_der.as_slice())
        .expect("Failed to parse hardware public key SPKI");
    let hardware_pubkey_bytes = hardware_spki.subject_public_key.raw_bytes();

    // Step 5: Verify public keys match
    assert_eq!(cert_public_key, hardware_pubkey_bytes,
        "Certificate public key must match hardware public key");

    // Step 6: Verify certificate subject
    let subject = cert.tbs_certificate.subject.to_string();
    assert!(subject.contains("Test Certificate"), "Subject must match requested CN");

    println!("✔ Certificate round-trip validated: rcgen → x509-cert → public key match");
}
```

### Pattern 4: Anti-Pattern Tests (TEST-03)
**What:** Tests that prove fail-closed design works - no fallbacks, no placeholder keys, no auto-pass
**When to use:** Validating security properties and error handling
**Example:**
```rust
// File: crates/core/tests/yubikey_integration.rs

#[test]
fn test_signing_without_hardware_fails() {
    // Simulation test - create backend without hardware connection attempt
    let config = YubiKeyConfig::default();
    let backend = YubiKeyBackend::with_config(config).unwrap();

    // Attempt signing operation
    let sign_op = CryptoOperation::Sign {
        data: b"test".to_vec(),
        algorithm: SignatureAlgorithm::EcdsaP256,
    };

    let result = backend.perform_operation("9c", sign_op);

    // Must fail with HardwareError (not succeed with software fallback)
    assert!(result.is_err(), "Signing must fail without hardware");
    assert!(matches!(result.unwrap_err(), BackendError::HardwareError(_)),
        "Must return HardwareError, not fall back to software");
}

#[test]
fn test_empty_slot_returns_error_not_placeholder() {
    // Simulation test for slot parsing
    let result = YubiKeyBackend::parse_slot("");

    assert!(result.is_err(), "Empty slot must return error");

    let err = result.unwrap_err();
    assert!(matches!(err, BackendError::KeyNotFound(_)),
        "Empty slot must return KeyNotFound error");
}

#[test]
#[ignore] // Requires physical YubiKey
fn test_empty_hardware_slot_returns_error() {
    // Hardware test - read from slot with no certificate
    let config = YubiKeyConfig {
        pin: Some("123456".to_string()),
        default_slot: "82".to_string(), // Retired slot, likely empty
        verbose: true,
        max_pin_retries: 3,
    };

    let backend = YubiKeyBackend::with_config(config).expect("YubiKey not connected");

    let get_pubkey_op = CryptoOperation::GetPublicKey;
    let result = backend.perform_operation("82", get_pubkey_op);

    // Must return error (KeyNotFound), not placeholder key
    assert!(result.is_err(), "Empty slot must return error, not placeholder key");
    assert!(matches!(result.unwrap_err(), BackendError::KeyNotFound(_)),
        "Empty hardware slot must return KeyNotFound");
}
```

### Anti-Patterns to Avoid
- **Empty tests that just return Ok(()):** Every test MUST have at least one assertion (`assert!`, `assert_eq!`, or `expect()`) that validates actual output. Calling a function and returning Ok(()) proves nothing.
- **Placeholder keys or signatures:** All keys and signatures must come from real cryptographic operations (hardware or software crypto primitives), never hardcoded placeholders.
- **Software fallbacks in hardware backend:** Backend must return `BackendError::HardwareError` when hardware unavailable, never fall back to software crypto.
- **Tests without #[ignore] that require hardware:** Hardware-dependent tests MUST be marked `#[ignore]` so CI doesn't fail when YubiKey not present.
- **Using `#[cfg(test)]` for integration tests:** Integration tests go in `tests/` directory, not in `#[cfg(test)]` modules (Rust convention).

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Mocking YubiKey hardware | Custom mock framework (mockall, etc.) | Trait abstraction + simulation tests | Zero-cost abstraction, no macro complexity, existing backend already has trait-based design |
| X.509 certificate parsing | Manual ASN.1/DER parsing | x509-cert crate (RustCrypto) | Parsing X.509 correctly handles dozens of edge cases; x509-cert is battle-tested and pure Rust |
| DER encoding/decoding | Manual byte manipulation | der crate (RustCrypto) | DER encoding has subtle rules; der crate is used by x509-cert and rcgen |
| Certificate generation | Manual TBS certificate construction | rcgen with RemoteKeyPair trait | Certificate generation requires correct ASN.1 structure, extensions, validity dates; rcgen handles all edge cases |
| Test organization | Custom test harness | Rust built-in #[ignore] and #[cfg(test)] | Rust standard patterns work perfectly for hardware/simulation test separation |

**Key insight:** Hardware-backed cryptography testing requires two-tier approach (simulation + hardware tests), but Rust's built-in test infrastructure (`#[cfg(test)]`, `#[ignore]`, integration tests) provides everything needed without custom test frameworks. The existing YubiKey backend implementation (Phase 10) already uses trait-based design (UniversalBackend), making simulation tests straightforward without mocking libraries.

## Common Pitfalls

### Pitfall 1: Tests Without Assertions (TEST-04 violation)
**What goes wrong:** Writing test functions that call code but don't assert anything, making them useless for validation
**Why it happens:** Confusion between "test runs without panicking" and "test validates behavior"
**How to avoid:** Every test function MUST contain at least one `assert!`, `assert_eq!`, or `expect()` call that validates actual output
**Warning signs:** Test function with Result<()> return type but no assertions, just function calls with `?` operator

**Example of BAD test:**
```rust
#[test]
fn bad_test_no_assertions() -> Result<()> {
    let backend = YubiKeyBackend::new()?;
    let capabilities = backend.get_capabilities();
    // NO ASSERTION - this proves nothing
    Ok(())
}
```

**Example of GOOD test:**
```rust
#[test]
fn good_test_with_assertions() -> Result<()> {
    let backend = YubiKeyBackend::new()?;
    let capabilities = backend.get_capabilities();

    // Assertions validate actual behavior
    assert!(capabilities.hardware_backed);
    assert!(capabilities.signature_algorithms.contains(&SignatureAlgorithm::EcdsaP256));
    assert_eq!(capabilities.max_key_size, Some(2048));

    Ok(())
}
```

### Pitfall 2: Hardware Tests Not Marked #[ignore]
**What goes wrong:** CI fails on every commit because hardware tests require physical YubiKey
**Why it happens:** Forgetting to add `#[ignore]` attribute to tests that require hardware
**How to avoid:** Mark ALL tests requiring physical YubiKey with `#[ignore]` attribute; use naming convention like `test_real_*` for hardware tests
**Warning signs:** CI test failures with "YubiKey not connected" errors

### Pitfall 3: Testing Capability Reporting Requires Hardware
**What goes wrong:** Assuming `get_capabilities()` and `backend_info()` need hardware to test
**Why it happens:** Conflating "reports hardware capabilities" with "requires hardware to report"
**How to avoid:** Capability reporting and backend info are metadata operations that work without hardware (they return what the backend *could* do if hardware were available); test these in simulation tests
**Warning signs:** All capability tests marked with `#[ignore]`, preventing CI validation

### Pitfall 4: rcgen RemoteKeyPair API Changes
**What goes wrong:** Documentation refers to `RemoteKeyPair` trait but rcgen 0.14+ renames it to `SigningKey`
**Why it happens:** rcgen API changed in version 0.14 (2025-2026), older docs use old names
**How to avoid:** Check actual rcgen version in Cargo.toml (currently 0.13); use `RemoteKeyPair` for 0.13, `SigningKey` for 0.14+
**Warning signs:** Compiler errors about `RemoteKeyPair` not found, or `KeyPair::from_remote` not existing

**Current Phase 10 implementation uses rcgen 0.13 with RemoteKeyPair trait - no changes needed for Phase 11.**

### Pitfall 5: x509-cert Signature Verification Complexity
**What goes wrong:** Attempting to verify certificate signatures requires extracting algorithm, parsing signature, and using correct verification crate (p256, rsa, etc.)
**Why it happens:** x509-cert is a parsing library, not a full verification library; signature verification requires additional crates
**How to avoid:** For TEST-05, focus on round-trip validation (generate → parse → compare public keys), not full signature verification chain; public key matching proves hardware signing worked correctly
**Warning signs:** Trying to import webpki or x509-parser for signature verification

### Pitfall 6: PIN Retry Lockout Risk
**What goes wrong:** Running hardware tests with wrong PIN multiple times locks YubiKey
**Why it happens:** YubiKey PIV has 3 retry limit (default); wrong PIN 3 times triggers lockout
**How to avoid:** Use test YubiKey with known PIN; document PIN in test comments; implement retry counter checking in backend
**Warning signs:** Tests start failing with "PIN locked" errors after multiple runs

## Code Examples

Verified patterns from official sources:

### Running Tests by Category
```bash
# Source: Rust official documentation - cargo test
# https://doc.rust-lang.org/book/ch11-02-running-tests.html

# Run only simulation tests (NOT ignored)
cargo test --features yubikey

# Run only hardware tests (ignored tests)
cargo test --features yubikey -- --ignored

# Run ALL tests (simulation + hardware)
cargo test --features yubikey -- --include-ignored

# Run specific test by name
cargo test --features yubikey test_parse_slot_valid_ids
```

### Organizing Test Modules with #[cfg(test)]
```rust
// Source: Rust official documentation + existing Software HSM pattern
// https://doc.rust-lang.org/book/ch11-03-test-organization.html

// File: crates/core/src/backends/yubikey.rs
impl YubiKeyBackend {
    // Implementation code
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function available only in tests
    fn create_test_config() -> YubiKeyConfig {
        YubiKeyConfig {
            pin: Some("123456".to_string()),
            default_slot: "9c".to_string(),
            verbose: false,
            max_pin_retries: 3,
        }
    }

    #[test]
    fn test_simulation_example() {
        let config = create_test_config();
        // ... test code
    }
}
```

### Integration Test Organization
```rust
// Source: Rust official documentation
// https://doc.rust-lang.org/book/ch11-03-test-organization.html

// File: crates/core/tests/yubikey_integration.rs
use trustedge_core::backends::yubikey::YubiKeyBackend;

// Common test utilities in tests/common/mod.rs
mod common;

#[test]
#[ignore]
fn test_hardware_operation() {
    let backend = common::setup_test_backend();
    // ... hardware test
}
```

### x509-cert DER Parsing
```rust
// Source: x509-cert crate documentation
// https://docs.rs/x509-cert/latest/x509_cert/

use x509_cert::Certificate;
use der::Decode;

let cert_der = backend.generate_certificate("9c", "Test")?;

// Parse DER-encoded certificate
let cert = Certificate::from_der(&cert_der)
    .expect("Failed to parse certificate");

// Extract public key
let public_key_info = &cert.tbs_certificate.subject_public_key_info;
let public_key_bytes = public_key_info.subject_public_key.raw_bytes();

// Extract subject
let subject = cert.tbs_certificate.subject.to_string();
assert!(subject.contains("Test"));
```

### SPKI Parsing for Public Key Extraction
```rust
// Source: Phase 10 implementation + spki crate docs
use spki::SubjectPublicKeyInfoRef;
use der::Decode;

// Get DER-encoded SPKI from hardware
let public_key_der = backend.piv_get_public_key(slot)?;

// Parse SPKI
let spki = SubjectPublicKeyInfoRef::try_from(public_key_der.as_slice())
    .expect("Failed to parse SPKI");

// Extract raw public key bytes
let public_key_bytes = spki.subject_public_key.raw_bytes();
```

### Assertion Patterns
```rust
// Source: Rust official documentation
// https://doc.rust-lang.org/book/ch11-01-writing-tests.html

#[test]
fn test_with_boolean_assertion() {
    let capabilities = backend.get_capabilities();

    // Boolean assertions
    assert!(capabilities.hardware_backed);
    assert!(!capabilities.supports_key_derivation);
}

#[test]
fn test_with_equality_assertion() {
    let capabilities = backend.get_capabilities();

    // Equality assertions with debug output on failure
    assert_eq!(capabilities.max_key_size, Some(2048));
    assert_ne!(capabilities.signature_algorithms.len(), 0);
}

#[test]
fn test_with_result_expect() {
    let slot = YubiKeyBackend::parse_slot("9c")
        .expect("Valid slot ID must parse successfully");

    assert_eq!(slot, SlotId::Signature);
}

#[test]
fn test_error_case() {
    let result = YubiKeyBackend::parse_slot("invalid");

    // Assert error occurred
    assert!(result.is_err());

    // Assert specific error type
    let err = result.unwrap_err();
    assert!(matches!(err, BackendError::KeyNotFound(_)));
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| mockall/galvanic-mock for hardware mocking | Trait-based abstraction + simulation tests | Rust ecosystem 2024-2026 | Zero-cost abstraction, no macro complexity; UniversalBackend trait already provides abstraction layer |
| rcgen RemoteKeyPair trait | rcgen SigningKey trait (0.14+) | rcgen 0.14.0 (June 2025) | API rename only; Phase 10 uses 0.13 with RemoteKeyPair - no changes needed for Phase 11 |
| #[ignore] without reason | #[ignore = "reason"] supported | Rust 1.83+ (Jan 2026) | Can add optional reasons for ignored tests (e.g., `#[ignore = "requires YubiKey hardware"]`) |
| Manual test categorization | cargo test --ignored and --include-ignored | Rust stable feature | Standard way to run/skip ignored tests |

**Deprecated/outdated:**
- Manual mock implementations: Rust ecosystem moved toward trait-based abstraction over heavy mocking frameworks
- Custom test harnesses for hardware tests: Built-in `#[ignore]` attribute is sufficient and standard
- x509-parser crate for new projects: x509-cert (RustCrypto) is newer pure Rust implementation with better API

## Open Questions

1. **PIN management in CI/testing**
   - What we know: YubiKey PIV default PIN is "123456", changeable via ykman
   - What's unclear: Best practice for test YubiKey PIN management (should tests assume default PIN or require env var?)
   - Recommendation: Use environment variable `YUBIKEY_TEST_PIN` with fallback to default "123456"; document in test file header

2. **Simulation test coverage without hardware**
   - What we know: Capability reporting, slot parsing, error mapping, config validation don't require hardware
   - What's unclear: How much of backend logic can be tested without hardware beyond these areas
   - Recommendation: Focus simulation tests on "pure functions" and metadata operations; accept that signing/key extraction require hardware tests

3. **Certificate signature verification in round-trip test**
   - What we know: x509-cert parses certificates but signature verification requires additional p256/rsa crates
   - What's unclear: Whether TEST-05 requires full signature verification or just public key matching
   - Recommendation: Public key matching is sufficient proof (if cert public key matches hardware public key, and cert was generated via rcgen hardware-backed signing, the signature is valid); full verification adds complexity without additional confidence

4. **Handling multiple YubiKey devices in tests**
   - What we know: `YubiKey::open()` connects to first available device
   - What's unclear: How to handle test environments with multiple YubiKeys (e.g., developer has multiple devices)
   - Recommendation: Document that tests use first available YubiKey; add test setup instructions to test file header

5. **Test execution time and CI timeout**
   - What we know: Hardware operations (signing, PIN verification) take ~100-500ms per operation
   - What's unclear: Whether comprehensive hardware test suite will hit CI timeouts
   - Recommendation: Keep hardware test count reasonable (10-15 tests max); use `cargo test --ignored -- --test-threads=1` to avoid hardware contention

## Sources

### Primary (HIGH confidence)
- [Rust Testing - The Rust Programming Language (Official)](https://doc.rust-lang.org/book/ch11-01-writing-tests.html) - Test organization, assertions, #[ignore] attribute
- [Rust Test Organization - Official Book](https://doc.rust-lang.org/book/ch11-03-test-organization.html) - Unit vs integration tests, #[cfg(test)]
- [Rust #[ignore] attribute - Official Reference](https://doc.rust-lang.org/reference/attributes/testing.html) - Controlling test execution
- [x509-cert crate - RustCrypto](https://docs.rs/x509-cert/latest/x509_cert/) - Certificate parsing and DER decoding
- [rcgen crate documentation](https://docs.rs/rcgen/latest/rcgen/) - Certificate generation with RemoteKeyPair/SigningKey
- [YubiKey PIV Slots - Official Yubico Documentation](https://docs.yubico.com/yesdk/users-manual/application-piv/slots.html) - Slot IDs 9a/9c/9d/9e
- Existing test files: `crates/core/tests/software_hsm_integration.rs`, `crates/core/tests/universal_backend_integration.rs` - Pattern examples from v1.0 codebase

### Secondary (MEDIUM confidence)
- [Controlling Test Execution with #[ignore] in Rust - Sling Academy](https://www.slingacademy.com/article/controlling-test-execution-with-ignore-in-rust/) - #[ignore] usage patterns
- [Mocking in Rust: Mockall and alternatives - LogRocket](https://blog.logrocket.com/mocking-rust-mockall-alternatives/) - Why trait abstraction preferred over mockall
- [Complete Guide To Testing Code In Rust - Zero To Mastery](https://zerotomastery.io/blog/complete-guide-to-testing-code-in-rust/) - Assertion best practices

### Tertiary (LOW confidence)
- [rcgen RemoteKeyPair issue #56](https://github.com/rustls/rcgen/issues/56) - Discussion of RemoteKeyPair design (historical context)
- [x509-parser vs x509-cert comparison](https://github.com/rusticata/x509-parser) - Alternative parsing library (not recommended for new code)

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - All libraries are in use in Phase 10 implementation or v1.0 codebase
- Architecture: HIGH - Rust official documentation for #[cfg(test)] and #[ignore], existing Software HSM tests demonstrate patterns
- Pitfalls: HIGH - Based on official Rust docs, rcgen version check in Cargo.toml, YubiKey PIV documentation
- Certificate round-trip: MEDIUM - x509-cert parsing is straightforward, but signature verification complexity is partially inferred

**Research date:** 2026-02-11
**Valid until:** 2026-03-11 (30 days - stable ecosystem)

**Notes:**
- All findings verified against Phase 10 implementation in `crates/core/src/backends/yubikey.rs`
- Existing v1.0 test patterns (343 tests, 160 in core) provide strong reference implementation
- rcgen 0.13 confirmed in `crates/core/Cargo.toml` - RemoteKeyPair trait is correct API for current version
- No CONTEXT.md exists - no user constraints to incorporate
