---
phase: 54-transport-security
plan: 01
subsystem: transport/quic
tags: [security, tls, quic, mitm, signature-verification]
dependency_graph:
  requires: []
  provides: [TSEC-01, TSEC-02]
  affects: [crates/core/src/transport/quic.rs]
tech_stack:
  added: [rcgen (dev-dependency)]
  patterns: [rustls canonical delegation, cfg feature gating]
key_files:
  modified:
    - crates/core/src/transport/quic.rs
    - crates/core/Cargo.toml
key_decisions:
  - "Delegate verify_tls12/13_signature to rustls::crypto free functions (canonical WebPkiServerVerifier pattern)"
  - "Gate accept_any_hardware() and its call site behind #[cfg(feature = \"insecure-tls\")]"
  - "Test acceptance of valid signatures via in-memory rustls loopback handshake (rcgen sign() is pub(crate))"
  - "Add rcgen to [dev-dependencies] unconditionally (no feature gate needed for test deps)"
metrics:
  duration: "42m"
  completed_date: "2026-03-23"
  tasks_completed: 2
  tasks_total: 2
  files_modified: 2
  tests_added: 6
---

# Phase 54 Plan 01: QUIC TLS Signature Verification Fix Summary

**One-liner:** Fixed QUIC MITM vulnerability by delegating TLS handshake signature verification to `rustls::crypto::verify_tls12/13_signature` and gating the dev bypass behind `insecure-tls` feature flag.

## What Was Built

### Task 1: Fix signature verification and gate accept_any_hardware

**Root cause closed:** `HardwareBackedVerifier::verify_tls12_signature` and `verify_tls13_signature` in `crates/core/src/transport/quic.rs` (lines 409-450) each looked up a `SignatureScheme` from the aws-lc-rs provider but discarded the result into `_verifier`, then unconditionally returned `HandshakeSignatureValid::assertion()`. This made every QUIC TLS handshake succeed regardless of certificate validity — a complete MITM bypass.

**Fix applied (two methods):**
```rust
// verify_tls12_signature
let provider = rustls::crypto::aws_lc_rs::default_provider();
rustls::crypto::verify_tls12_signature(message, cert, dss, &provider.signature_verification_algorithms)

// verify_tls13_signature
let provider = rustls::crypto::aws_lc_rs::default_provider();
rustls::crypto::verify_tls13_signature(message, cert, dss, &provider.signature_verification_algorithms)
```

**Feature gating:**
- `accept_any_hardware()` gated with `#[cfg(feature = "insecure-tls")]`
- `create_hardware_verified_endpoint()` has two branches:
  - `#[cfg(feature = "insecure-tls")]`: empty certs allowed (calls `accept_any_hardware()`)
  - `#[cfg(not(feature = "insecure-tls"))]`: `anyhow::ensure!(!trusted_certificates.is_empty(), "trusted_certificates must not be empty in secure builds")`

**Unit tests added (4):**
- `test_hardware_verifier_rejects_bad_tls12_signature` — garbage sig bytes rejected by TLS 1.2 verify
- `test_hardware_verifier_rejects_bad_tls13_signature` — garbage sig bytes rejected by TLS 1.3 verify
- `test_hardware_verifier_accepts_valid_tls12_signature` — in-memory rustls loopback handshake with matching cert/key proves acceptance of valid signatures
- `test_hardware_verifier_empty_certs_rejected_in_secure_build` — `create_hardware_verified_endpoint(vec![])` returns `Err` in default builds

**Dev dependency:** `rcgen = "0.13"` added to `[dev-dependencies]` (not feature-gated) in `crates/core/Cargo.toml`.

### Task 2: Integration tests for QUIC MITM rejection and legitimate connection

**Integration tests added (2):**
- `test_quic_mitm_certificate_rejected` — spawns a quinn server endpoint with cert A, builds a client trusting cert B (attacker cert), verifies the handshake returns `Err` within 5s
- `test_quic_legitimate_connection_succeeds` — spawns a quinn server endpoint with cert A, builds a client trusting cert A (same cert), verifies the handshake completes and the `Connection` object is valid

Both tests use `rcgen::generate_simple_self_signed(vec!["localhost".to_string()])` for cert generation and bind to `127.0.0.1:0` for loopback testing.

## Commits

| Hash | Task | Description |
|------|------|-------------|
| c600fd6 | Task 1 | feat: fix QUIC TLS signature verification no-op |
| 2715888 | Task 2 | test: add QUIC MITM rejection and legitimate connection tests |

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] rcgen `key_pair.sign()` is not a public method**
- **Found during:** Task 1 — `test_hardware_verifier_accepts_valid_tls12_signature`
- **Issue:** The plan specified using `cert.key_pair.sign(b"test message")` to construct a valid signature for the unit test. In rcgen 0.13.2, the `sign()` method on `KeyPair` is `pub(crate)` and not externally accessible.
- **Fix:** Replaced the approach with an in-memory rustls loopback TLS handshake using `rustls::ClientConnection` + `rustls::ServerConnection`. This proves acceptance of valid signatures by completing an actual TLS handshake with `HardwareBackedVerifier` as the client verifier — a stronger test than a unit-level DSS construction.
- **Files modified:** crates/core/src/transport/quic.rs (test module)
- **Commit:** c600fd6

## Test Results

```
running 6 tests (new security tests)
test transport::quic::tests::test_hardware_verifier_empty_certs_rejected_in_secure_build ... ok
test transport::quic::tests::test_hardware_verifier_rejects_bad_tls12_signature ... ok
test transport::quic::tests::test_hardware_verifier_rejects_bad_tls13_signature ... ok
test transport::quic::tests::test_hardware_verifier_accepts_valid_tls12_signature ... ok
test transport::quic::tests::test_quic_mitm_certificate_rejected ... ok
test transport::quic::tests::test_quic_legitimate_connection_succeeds ... ok

All 182 tests pass (excluding pre-existing slow test_many_keys).
```

## Known Stubs

None — all new code is fully wired with real cryptographic operations.

## Self-Check: PASSED
