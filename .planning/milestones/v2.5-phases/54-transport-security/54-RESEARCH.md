# Phase 54: Transport Security - Research

**Researched:** 2026-03-22
**Domain:** rustls 0.23 TLS signature verification in Quinn QUIC transport
**Confidence:** HIGH

---

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

- **D-01:** Delegate TLS handshake signature verification to the rustls default provider (aws-lc-rs). The code already looks up the matching `SignatureScheme` — the fix is to call `verify()` on the actual message, cert, and signature instead of discarding the verifier with `_verifier`.
- **D-02:** Both `verify_tls12_signature` and `verify_tls13_signature` must use the same approach — call the provider's verify method with the real `message`, `cert`, and `dss` parameters.
- **D-03:** Gate `accept_any_hardware()` behind the existing `insecure-tls` compile-time feature flag. This is consistent with how `SkipServerVerification` is already gated. Dev mode remains available for development but cannot ship in release builds.
- **D-04:** When `insecure-tls` is not enabled, `accept_any_hardware()` must not be callable — remove it from the public API surface in default builds.
- **D-05:** Unit tests: construct invalid `DigitallySignedStruct` payloads and verify they are rejected by `verify_tls12_signature` and `verify_tls13_signature`. Also test that valid signatures pass.
- **D-06:** Integration tests: attempt a QUIC connection with a wrong/forged certificate and verify the handshake fails. Verify legitimate connections still succeed.

### Claude's Discretion

- Exact test certificate generation approach (self-signed vs rcgen)
- Whether to refactor common verification logic into a shared helper
- Error message text for verification failures

### Deferred Ideas (OUT OF SCOPE)

None — discussion stayed within phase scope.
</user_constraints>

---

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| TSEC-01 | QUIC `HardwareBackedVerifier` performs actual TLS signature verification instead of returning unconditional `HandshakeSignatureValid::assertion()` | D-01/D-02: Call `rustls::crypto::verify_tls12_signature` / `verify_tls13_signature` with provider's `signature_verification_algorithms` |
| TSEC-02 | MITM attack against QUIC TLS handshake is rejected (test proves verification catches bad signatures) | D-05/D-06: Unit tests with forged `DigitallySignedStruct` via `Codec::read`, integration test with mismatched certificate |
</phase_requirements>

---

## Summary

The bug is surgically narrow: `verify_tls12_signature` and `verify_tls13_signature` in `HardwareBackedVerifier` (lines 409 and 431 of `crates/core/src/transport/quic.rs`) look up the correct `SignatureScheme` from the aws-lc-rs provider but bind the result to `_verifier` (discarding it), then unconditionally return `HandshakeSignatureValid::assertion()`. This means no MITM protection exists on the hardware-verified QUIC path — any certificate is accepted regardless of signature validity.

The fix is a one-liner per method: delegate to `rustls::crypto::verify_tls12_signature` / `rustls::crypto::verify_tls13_signature`, passing the real `message`, `cert`, `dss`, and `provider.signature_verification_algorithms`. These free functions are the canonical delegation pattern used by `WebPkiServerVerifier` itself (the rustls built-in verifier). Separately, `accept_any_hardware()` must be gated behind `#[cfg(feature = "insecure-tls")]` following the exact pattern of `SkipServerVerification`.

For unit tests (D-05): `DigitallySignedStruct::new` is `pub(crate)` in rustls — not directly constructible from test code. However, `rustls::internal::msgs::codec::{Codec, Reader}` are public (`rustls-0.23.36/src/lib.rs` line 476), and `DigitallySignedStruct` implements `Codec`. A forged `DigitallySignedStruct` can be constructed by crafting wire-format bytes (2-byte scheme + 2-byte length prefix + payload bytes) and calling `Codec::read`. For the integration test (D-06): `rcgen::generate_simple_self_signed` + `quinn::ServerConfig::with_single_cert` is the standard scaffold (used by quinn's own test suite).

**Primary recommendation:** Use `rustls::crypto::verify_tls12_signature(message, cert, dss, &provider.signature_verification_algorithms)` and `rustls::crypto::verify_tls13_signature(message, cert, dss, &provider.signature_verification_algorithms)`. Gate `accept_any_hardware()` behind `#[cfg(feature = "insecure-tls")]`. Construct forged `DigitallySignedStruct` via `Codec::read` from crafted bytes in unit tests.

---

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| rustls | 0.23.36 (resolved) | TLS implementation | Already in tree; `verify_tls12/13_signature` free functions are the canonical delegation path |
| aws-lc-rs | 1.16.2 (resolved) | Crypto provider | Already the default provider; `default_provider()` already called in broken code |
| quinn | 0.11.9 (resolved) | QUIC transport | Already in tree; `ServerConfig::with_single_cert` used for test server setup |
| rcgen | 0.13.2 (optional dep, resolved via `yubikey`) | Test certificate generation | Already in dependency graph; add to `[dev-dependencies]` without feature gate |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| rustls::internal::msgs::codec | (re-exported from rustls 0.23 `internal` module) | `Codec`, `Reader` — construct `DigitallySignedStruct` from wire bytes in unit tests | Unit tests that need forged DSS without quinn handshake |

**Installation (dev-dependency addition):**
```toml
# crates/core/Cargo.toml [dev-dependencies]
rcgen = "0.13"
```

This makes `rcgen` unconditionally available in tests without requiring `--features yubikey`.

---

## Architecture Patterns

### The Fix — Canonical Delegation Pattern

`rustls` exposes two free functions in `rustls::crypto` (re-exported from `rustls::webpki::verify`) that are the canonical correct implementation:

```rust
// Source: rustls-0.23.36/src/crypto/mod.rs lines 13-15 (confirmed)
// pub use crate::webpki::{
//     WebPkiSupportedAlgorithms, verify_tls12_signature, verify_tls13_signature,
//     verify_tls13_signature_with_raw_key,
// };
```

`WebPkiServerVerifier` (the rustls built-in) implements `verify_tls12_signature` and `verify_tls13_signature` exactly as:

```rust
// Source: rustls-0.23.36/src/webpki/server_verifier.rs lines 280-298 (confirmed)
fn verify_tls12_signature(
    &self,
    message: &[u8],
    cert: &CertificateDer<'_>,
    dss: &DigitallySignedStruct,
) -> Result<HandshakeSignatureValid, Error> {
    verify_tls12_signature(message, cert, dss, &self.supported)
}

fn verify_tls13_signature(
    &self,
    message: &[u8],
    cert: &CertificateDer<'_>,
    dss: &DigitallySignedStruct,
) -> Result<HandshakeSignatureValid, Error> {
    verify_tls13_signature(message, cert, dss, &self.supported)
}
```

where `self.supported` is a `WebPkiSupportedAlgorithms`. In `HardwareBackedVerifier`, `provider.signature_verification_algorithms` IS the `WebPkiSupportedAlgorithms`.

### Fixed `verify_tls12_signature`

```rust
// Before (broken — no-op):
fn verify_tls12_signature(
    &self,
    _message: &[u8],
    _cert: &CertificateDer<'_>,
    dss: &rustls::DigitallySignedStruct,
) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
    let provider = rustls::crypto::aws_lc_rs::default_provider();
    let _verifier = provider
        .signature_verification_algorithms
        .supported_schemes()
        .iter()
        .find(|scheme| **scheme == dss.scheme)
        .ok_or(rustls::Error::UnsupportedNameType)?;
    Ok(rustls::client::danger::HandshakeSignatureValid::assertion())
}

// After (correct — actual verification):
fn verify_tls12_signature(
    &self,
    message: &[u8],
    cert: &CertificateDer<'_>,
    dss: &rustls::DigitallySignedStruct,
) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
    let provider = rustls::crypto::aws_lc_rs::default_provider();
    rustls::crypto::verify_tls12_signature(
        message,
        cert,
        dss,
        &provider.signature_verification_algorithms,
    )
}
```

### Fixed `verify_tls13_signature`

```rust
// After (correct — actual verification):
fn verify_tls13_signature(
    &self,
    message: &[u8],
    cert: &CertificateDer<'_>,
    dss: &rustls::DigitallySignedStruct,
) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
    let provider = rustls::crypto::aws_lc_rs::default_provider();
    rustls::crypto::verify_tls13_signature(
        message,
        cert,
        dss,
        &provider.signature_verification_algorithms,
    )
}
```

### `accept_any_hardware()` Feature Gate

The pattern to follow is identical to `SkipServerVerification` (already in quic.rs lines 355-363 for `accept_any_hardware()`, gated struct at lines 471-522):

```rust
// Gate the method itself inside impl HardwareBackedVerifier
#[cfg(feature = "insecure-tls")]
pub fn accept_any_hardware() -> Self {
    Self {
        trusted_certificates: Vec::new(),
        validate_attestation: false,
    }
}
```

The call site in `create_hardware_verified_endpoint()` (line 117-123) also needs gating:

```rust
pub fn create_hardware_verified_endpoint(
    trusted_certificates: Vec<Vec<u8>>,
) -> Result<Endpoint> {
    #[cfg(feature = "insecure-tls")]
    let verifier = if trusted_certificates.is_empty() {
        Arc::new(HardwareBackedVerifier::accept_any_hardware())
    } else {
        Arc::new(HardwareBackedVerifier::new(trusted_certificates))
    };

    #[cfg(not(feature = "insecure-tls"))]
    let verifier = {
        anyhow::ensure!(
            !trusted_certificates.is_empty(),
            "trusted_certificates must not be empty in secure builds"
        );
        Arc::new(HardwareBackedVerifier::new(trusted_certificates))
    };
    // ...
}
```

### Integration Test Setup Pattern (from quinn's own test suite)

```rust
// Source: quinn-0.11.9/tests/many_connections.rs lines 144-162 (confirmed)
// Add `rcgen = "0.13"` to [dev-dependencies] in crates/core/Cargo.toml

fn gen_test_cert() -> (CertificateDer<'static>, rustls::pki_types::PrivatePkcs8KeyDer<'static>) {
    let cert = rcgen::generate_simple_self_signed(vec!["localhost".to_string()]).unwrap();
    (
        cert.cert.der().clone(),
        rustls::pki_types::PrivatePkcs8KeyDer::from(cert.key_pair.serialize_der()),
    )
}

fn make_server_endpoint(
    cert_der: CertificateDer<'static>,
    key_der: rustls::pki_types::PrivatePkcs8KeyDer<'static>,
) -> quinn::Endpoint {
    let server_config = quinn::ServerConfig::with_single_cert(
        vec![cert_der],
        key_der.into(),
    ).unwrap();
    quinn::Endpoint::server(server_config, "127.0.0.1:0".parse().unwrap()).unwrap()
}
```

### Unit Test — Forged `DigitallySignedStruct` via `Codec::read`

`DigitallySignedStruct::new` is `pub(crate)` (confirmed: `rustls-0.23.36/src/verify.rs` line 336). The `sig` field is also private (`PayloadU16`). Direct construction is impossible from outside rustls. However, `rustls::internal::msgs::codec::{Codec, Reader}` are public (confirmed: `rustls-0.23.36/src/lib.rs` line 476). `DigitallySignedStruct` implements `Codec::read`. Wire format: 2-byte `SignatureScheme` value + 2-byte big-endian length + signature bytes.

```rust
// In #[cfg(test)] mod tests inside quic.rs:
use rustls::internal::msgs::codec::{Codec, Reader};

fn make_digitally_signed_struct(
    scheme: rustls::SignatureScheme,
    sig_bytes: &[u8],
) -> rustls::DigitallySignedStruct {
    // Wire format: u16 scheme || u16 length || bytes
    let mut wire = Vec::new();
    let scheme_u16 = u16::from(scheme);
    wire.extend_from_slice(&scheme_u16.to_be_bytes());
    let len = sig_bytes.len() as u16;
    wire.extend_from_slice(&len.to_be_bytes());
    wire.extend_from_slice(sig_bytes);
    let mut reader = Reader::init(&wire);
    rustls::DigitallySignedStruct::read(&mut reader).expect("valid wire format")
}
```

This helper is used in unit tests to construct both valid (real signature) and forged (garbage bytes) `DigitallySignedStruct` values.

Note: The `SignatureScheme` wire encoding should be verified against `rustls::SignatureScheme` to confirm `u16::from(scheme)` is the correct encoding. Inspect `SignatureScheme::encode` in rustls source if needed.

### Anti-Patterns to Avoid

- **Removing `_` prefix from parameter names without actually using them:** The existing code has `_message` and `_cert` — the fix must remove the underscore prefix AND pass them to the real verifier function.
- **Calling `.verify()` on the `SignatureScheme` iterator result:** `SignatureScheme` is an enum — it has no `verify()` method. The correct path is `rustls::crypto::verify_tls12_signature(...)`.
- **Adding `rcgen` to production dependencies:** It belongs in `[dev-dependencies]` only.
- **Calling `rustls::DigitallySignedStruct::new(...)` from test code:** It is `pub(crate)` — use `Codec::read` from wire bytes instead.

---

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| TLS 1.2 handshake signature verification | Custom webpki cert/signature parsing | `rustls::crypto::verify_tls12_signature` | Handles multi-algorithm fallback, webpki error mapping, all edge cases |
| TLS 1.3 handshake signature verification | Custom signature check | `rustls::crypto::verify_tls13_signature` | Enforces `supported_in_tls13()` constraint, canonical implementation |
| Test certificate generation | Manual DER encoding | `rcgen::generate_simple_self_signed` | Used by quinn's own test suite; one-liner for valid self-signed cert + key |
| Quinn test server | Custom TLS server | `quinn::ServerConfig::with_single_cert` + `quinn::Endpoint::server` | Standard pattern from quinn docs and tests |
| Constructing `DigitallySignedStruct` in tests | Direct struct construction | `Codec::read` from wire bytes | `new()` is `pub(crate)`, `sig` field is private — only `Codec::read` is publicly accessible |

**Key insight:** rustls already ships the correct verification logic as public free functions. The bug is purely that `HardwareBackedVerifier` never calls them.

---

## Common Pitfalls

### Pitfall 1: Incorrect Import Path for `verify_tls12_signature`
**What goes wrong:** Trying to call `rustls::webpki::verify_tls12_signature` — this path is not public. The function is re-exported from `rustls::crypto`.
**Why it happens:** The function lives in `rustls::webpki::verify` internally, but the public API is in `rustls::crypto`.
**How to avoid:** Use `rustls::crypto::verify_tls12_signature` and `rustls::crypto::verify_tls13_signature`. Confirmed: `rustls-0.23.36/src/crypto/mod.rs` line 14 re-exports both.
**Warning signs:** Compile error "module `webpki` is private".

### Pitfall 2: `rustls::Error::UnsupportedNameType` is Wrong Error for Unsupported Scheme
**What goes wrong:** The existing code returns `Err(rustls::Error::UnsupportedNameType)` for an unsupported `SignatureScheme`. This is semantically incorrect.
**Why it happens:** Code was written quickly as a placeholder.
**How to avoid:** `rustls::crypto::verify_tls12_signature` handles unsupported-scheme errors correctly internally via `convert_scheme()`. Remove the manual scheme lookup; let the delegation handle it.
**Warning signs:** Clippy may not flag this, but it is wrong error semantics.

### Pitfall 3: CryptoProvider Not Installed in Tests
**What goes wrong:** Calling `default_provider()` works but `quinn::crypto::rustls::QuicClientConfig::try_from(crypto)?` or other quinn paths may need the provider installed.
**Why it happens:** rustls 0.23 requires exactly one provider to be installed globally in some paths.
**How to avoid:** In integration tests, call `let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();` at the start. This is already done in existing test `test_default_build_uses_secure_tls`.
**Warning signs:** `CryptoProviderNotInstalled` or `NoDefaultCryptoProvider` error at runtime.

### Pitfall 4: rcgen 0.13 API Differences from 0.12
**What goes wrong:** Using the old `cert.serialize_der().unwrap()` pattern from rcgen 0.12, or accessing `cert.signing_key` (renamed to `cert.key_pair`).
**Why it happens:** Many examples online use rcgen 0.12 API.
**How to avoid:** In rcgen 0.13, `CertifiedKey { cert, key_pair }` — use `cert.cert.der().clone()` for `CertificateDer<'static>`. Use `cert.key_pair.serialize_der()`. Confirmed from rcgen-0.13.2 source.
**Warning signs:** Compile error "no field `signing_key` on type `CertifiedKey`".

### Pitfall 5: `accept_any_hardware()` Call Site Not Gated
**What goes wrong:** Gating the method but not the call site in `create_hardware_verified_endpoint()` — compile error in default builds.
**Why it happens:** The call to `accept_any_hardware()` on line 119 still exists after the method is gated.
**How to avoid:** Both the method definition AND the call site must use `#[cfg(feature = "insecure-tls")]` / `#[cfg(not(feature = "insecure-tls"))]` branches.
**Warning signs:** Compile error "no method named `accept_any_hardware` found for struct `HardwareBackedVerifier`" in default build.

### Pitfall 6: `DigitallySignedStruct` Wire Format Encoding
**What goes wrong:** Encoding the `SignatureScheme` value incorrectly when constructing wire bytes for the test helper.
**Why it happens:** `SignatureScheme` is a TLS enum — its wire encoding may not be a simple `u16` cast.
**How to avoid:** Check `rustls::SignatureScheme::encode` in `rustls-0.23.36/src/enums.rs` or use `Codec::encode` on the scheme to get the correct bytes, then prepend the length-prefixed sig. Alternatively, use the integration test path (QUIC handshake) which constructs DSS internally — no wire encoding needed.
**Warning signs:** `Reader::init` parses successfully but the scheme value is wrong, causing `UnsupportedNameType` error instead of signature failure.

---

## Code Examples

### Verified Fix Pattern (from rustls source)

```rust
// Source: rustls-0.23.36/src/webpki/server_verifier.rs lines 280-298
// This is exactly how WebPkiServerVerifier (rustls built-in) delegates verification.
// HardwareBackedVerifier should mirror this pattern.

fn verify_tls12_signature(
    &self,
    message: &[u8],
    cert: &CertificateDer<'_>,
    dss: &rustls::DigitallySignedStruct,
) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
    let provider = rustls::crypto::aws_lc_rs::default_provider();
    rustls::crypto::verify_tls12_signature(
        message,
        cert,
        dss,
        &provider.signature_verification_algorithms,
    )
}

fn verify_tls13_signature(
    &self,
    message: &[u8],
    cert: &CertificateDer<'_>,
    dss: &rustls::DigitallySignedStruct,
) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
    let provider = rustls::crypto::aws_lc_rs::default_provider();
    rustls::crypto::verify_tls13_signature(
        message,
        cert,
        dss,
        &provider.signature_verification_algorithms,
    )
}
```

### Test Certificate Generation (from quinn test suite)

```rust
// Source: quinn-0.11.9/tests/many_connections.rs lines 155-163 (confirmed)
// Requires: rcgen = "0.13" in [dev-dependencies]

fn gen_test_cert() -> (CertificateDer<'static>, rustls::pki_types::PrivatePkcs8KeyDer<'static>) {
    let cert = rcgen::generate_simple_self_signed(vec!["localhost".to_string()]).unwrap();
    (
        cert.cert.der().clone(),
        rustls::pki_types::PrivatePkcs8KeyDer::from(cert.key_pair.serialize_der()),
    )
}
```

### Unit Test — Forged Signature via Codec::read

```rust
// Source: rustls-0.23.36/src/lib.rs line 476 confirms:
//   pub mod internal::msgs::codec { pub use Codec, Reader; }
// Source: rustls-0.23.36/src/verify.rs line 350 confirms DigitallySignedStruct implements Codec

#[cfg(test)]
mod tests {
    use super::*;
    use rustls::internal::msgs::codec::{Codec, Reader};

    fn make_dss_with_sig(scheme: rustls::SignatureScheme, sig_bytes: &[u8]) -> rustls::DigitallySignedStruct {
        let mut wire = Vec::new();
        // Encode scheme using rustls Codec (ensures correct TLS wire format)
        scheme.encode(&mut wire);
        // Encode signature as PayloadU16: 2-byte big-endian length + bytes
        let len = sig_bytes.len() as u16;
        wire.extend_from_slice(&len.to_be_bytes());
        wire.extend_from_slice(sig_bytes);
        let mut reader = Reader::init(&wire);
        rustls::DigitallySignedStruct::read(&mut reader).expect("valid DSS wire bytes")
    }

    #[tokio::test]
    async fn test_hardware_verifier_rejects_bad_tls12_signature() {
        let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();

        let cert = rcgen::generate_simple_self_signed(vec!["localhost".to_string()]).unwrap();
        let cert_der = cert.cert.der().clone();

        let verifier = HardwareBackedVerifier::new(vec![cert_der.to_vec()]);

        let bad_dss = make_dss_with_sig(
            rustls::SignatureScheme::ECDSA_NISTP256_SHA256,
            &[0xDE, 0xAD, 0xBE, 0xEF], // garbage signature
        );

        let result = verifier.verify_tls12_signature(b"test message", &cert_der, &bad_dss);
        assert!(result.is_err(), "Forged signature must be rejected");
    }
}
```

Note: `SignatureScheme::encode` must be imported — check if `rustls::internal::msgs::codec::Codec` is needed for `encode`, or if `SignatureScheme` implements `Codec` directly. Inspect the import in test code.

---

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Manual signature scheme lookup with `_verifier` discard | Delegate to `rustls::crypto::verify_tls12/13_signature` | This phase (fix) | Closes MITM vulnerability on `HardwareBackedVerifier` path |
| `accept_any_hardware()` always public | `accept_any_hardware()` behind `insecure-tls` feature | This phase | Dev-mode bypass cannot ship in production binaries |

---

## Open Questions

1. **`SignatureScheme::encode` import in tests**
   - What we know: `rustls::internal::msgs::codec::Codec` is public. `SignatureScheme` implements `Codec` (it has `encode` and `read` methods). Using `scheme.encode(&mut wire)` requires the `Codec` trait to be in scope.
   - What's unclear: Whether `use rustls::internal::msgs::codec::Codec;` is sufficient or if `SignatureScheme::encode` needs a different import path.
   - Recommendation: Try `use rustls::internal::msgs::codec::Codec; scheme.encode(&mut wire);` first. If that fails, use raw bytes by hardcoding the known 2-byte TLS scheme value (e.g., ECDSA_NISTP256_SHA256 = `0x0403`) directly. This is a minor test implementation detail.

2. **Shared helper vs. inline delegation**
   - What we know: Both `verify_tls12_signature` and `verify_tls13_signature` are one-liners after the fix.
   - Recommendation: No shared helper needed — Claude's discretion per CONTEXT.md.

---

## Environment Availability

Step 2.6: SKIPPED (no external dependencies identified). The fix touches only `crates/core/src/transport/quic.rs` and `crates/core/Cargo.toml`. All libraries (rustls, quinn, aws-lc-rs) are already resolved in the workspace. `rcgen` will be added as a dev-dependency — it is already in the dependency graph under the `yubikey` feature at version 0.13.2.

---

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Rust built-in test + tokio (async) |
| Config file | none (cargo test discovers tests automatically) |
| Quick run command | `cargo test -p trustedge-core --lib` |
| Full suite command | `cargo test -p trustedge-core` |

### Phase Requirements → Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| TSEC-01 | `verify_tls12_signature` rejects forged signature | unit | `cargo test -p trustedge-core test_hardware_verifier_rejects_bad_tls12_signature -- --nocapture` | ❌ Wave 0 |
| TSEC-01 | `verify_tls13_signature` rejects forged signature | unit | `cargo test -p trustedge-core test_hardware_verifier_rejects_bad_tls13_signature -- --nocapture` | ❌ Wave 0 |
| TSEC-01 | Valid signature accepted by both methods | unit | `cargo test -p trustedge-core test_hardware_verifier_accepts_valid_signature -- --nocapture` | ❌ Wave 0 |
| TSEC-02 | QUIC connection with forged cert fails handshake | integration | `cargo test -p trustedge-core test_quic_mitm_certificate_rejected -- --nocapture` | ❌ Wave 0 |
| TSEC-02 | Legitimate QUIC connection with valid cert succeeds | integration | `cargo test -p trustedge-core test_quic_legitimate_connection_succeeds -- --nocapture` | ❌ Wave 0 |

### Sampling Rate
- **Per task commit:** `cargo test -p trustedge-core --lib`
- **Per wave merge:** `cargo test -p trustedge-core`
- **Phase gate:** `./scripts/ci-check.sh` green before `/gsd:verify-work`

### Wave 0 Gaps
- [ ] Unit tests for `verify_tls12/13_signature` rejection and valid acceptance — added to `crates/core/src/transport/quic.rs` test module
- [ ] Integration test for MITM rejection and legitimate connection — added to `crates/core/tests/transport_integration.rs` or a new `crates/core/tests/quic_security_integration.rs`
- [ ] `rcgen = "0.13"` added to `[dev-dependencies]` in `crates/core/Cargo.toml`

*(No framework install needed — Rust built-in test runner, all deps already available)*

---

## Sources

### Primary (HIGH confidence)
- `rustls-0.23.36/src/crypto/mod.rs` lines 13-15 — Public re-export of `verify_tls12_signature`, `verify_tls13_signature`, `WebPkiSupportedAlgorithms`
- `rustls-0.23.36/src/webpki/server_verifier.rs` lines 280-298 — Canonical `WebPkiServerVerifier` delegation pattern (exact code the fix should mirror)
- `rustls-0.23.36/src/webpki/verify.rs` lines 155-202 — Implementation of `verify_tls12_signature` and `verify_tls13_signature` free functions with full webpki machinery
- `rustls-0.23.36/src/verify.rs` lines 329-360 — `DigitallySignedStruct` struct definition: `scheme` field public, `sig` field private (`PayloadU16`), `new()` is `pub(crate)`, implements `Codec`
- `rustls-0.23.36/src/lib.rs` line 476 — `rustls::internal::msgs::codec::{Codec, Reader}` confirmed public
- `rustls-0.23.36/src/crypto/mod.rs` line 210 — `CryptoProvider.signature_verification_algorithms: WebPkiSupportedAlgorithms` field confirmed
- `quinn-0.11.9/tests/many_connections.rs` lines 144-162 — Canonical integration test setup with `rcgen::generate_simple_self_signed` + `quinn::ServerConfig::with_single_cert`
- `rcgen-0.13.2/src/certificate.rs` lines 28-57 — `Certificate::der()` returns `&CertificateDer<'static>`; `CertifiedKey.key_pair.serialize_der()` for key bytes
- `crates/core/src/transport/quic.rs` lines 409-450 — The broken no-op code (direct inspection)
- `crates/core/Cargo.toml` — Confirms rustls 0.23, quinn 0.11, rcgen 0.13 (optional/yubikey) already in tree

### Secondary (MEDIUM confidence)
- `quinn-proto-0.11.13/src/config/mod.rs` line 381 — `ServerConfig::with_single_cert` confirmed in quinn

### Tertiary (LOW confidence)
- None

---

## Project Constraints (from CLAUDE.md)

| Directive | Applies to This Phase |
|-----------|----------------------|
| `cargo fmt` and `cargo clippy -- -D warnings` must pass | Yes — run after every code change |
| No `unwrap()` in production code | Yes — use `?` in `create_hardware_verified_endpoint`; `unwrap()` acceptable in `#[cfg(test)]` helpers |
| Copyright block comment header on all `.rs` files | Yes — modified file already has it; any new test file needs the same header |
| No emoji in code — use `✔ ✖ ⚠ ● ♪ ■` | Yes — applies to any new test `println!` output |
| Run `./scripts/ci-check.sh` before committing | Yes — phase gate criterion |
| `insecure-tls` feature already has CI steps in `.github/workflows/ci.yml` and `scripts/ci-check.sh` | Existing CI covers both default and insecure-tls builds — no new CI steps needed for this phase |

---

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH — all library versions confirmed from `cargo metadata`; source files read directly from registry
- Architecture: HIGH — exact fix pattern confirmed from rustls 0.23 source; Codec::read path confirmed from rustls internal module structure
- Pitfalls: HIGH — all identified from direct source code inspection, not training data

**Research date:** 2026-03-22
**Valid until:** 2026-06-22 (rustls 0.23 is stable; quinn 0.11 is current stable)
