---
phase: 76-cli-platform-endpoint
plan: "02"
subsystem: platform-http
tags: [attestation, http, endpoint, integration-tests]
one_liner: "POST /v1/verify-attestation endpoint with Ed25519 signature verification and JWS receipt"
dependency_graph:
  requires:
    - "75-01: PointAttestation struct with verify_signature and canonical_bytes"
    - "crates/platform/src/http/handlers.rs: existing verify_handler pattern"
    - "crates/platform/src/verify/signing.rs: sign_receipt_jws"
  provides:
    - "POST /v1/verify-attestation HTTP endpoint"
    - "VerifyAttestationResponse type"
    - "5 integration tests for attestation endpoint"
  affects:
    - "crates/platform/src/http/router.rs: route registration"
    - "crates/platform/tests/verify_integration.rs: 5 new tests"
tech_stack:
  added: [tempfile (dev-dependency for integration tests)]
  patterns:
    - "Axum String extractor for raw body (no JSON Content-Type constraint)"
    - "Dual feature-gate import pattern (sign_receipt_jws imported for both postgres and non-postgres)"
    - "ReceiptClaims populated minimally for point attestations (no segment/chain fields)"
key_files:
  created: []
  modified:
    - "crates/platform/src/http/handlers.rs"
    - "crates/platform/src/http/router.rs"
    - "crates/platform/tests/verify_integration.rs"
    - "crates/platform/Cargo.toml"
decisions:
  - "Use String extractor (not Json<>) for request body — attestation JSON has no Content-Type enforcement requirement"
  - "sign_receipt_jws imported with cfg(not(postgres)) to supplement existing cfg(postgres) import"
  - "Chain tip set to 'none' for point attestations (not applicable)"
  - "No feature gate on verify_attestation_handler — stateless, works with or without postgres"
metrics:
  duration_minutes: 20
  completed_date: "2026-04-02"
  tasks_completed: 2
  files_modified: 4
  tests_added: 5
---

# Phase 76 Plan 02: Platform verify-attestation Endpoint Summary

POST /v1/verify-attestation endpoint with Ed25519 signature verification and JWS receipt, plus 5 integration tests covering success and all error paths.

## What Was Built

### Task 1: verify_attestation_handler and Route Registration

Added `VerifyAttestationResponse` type and `verify_attestation_handler` function to `crates/platform/src/http/handlers.rs`:

- Accepts raw attestation JSON as request body (String extractor)
- Validates format discriminant (`te-point-attestation-v1`)
- Checks signature field is present
- Verifies Ed25519 signature via `PointAttestation::verify_signature()` using the embedded public key
- On success: builds `ReceiptClaims`, calls `sign_receipt_jws`, returns 200 with JWS receipt
- On failure: returns 200 with `status: "failed"` (no internal details leaked)
- On malformed/invalid input: returns 400 with error codes

Registered `/v1/verify-attestation` route in `router.rs` in the rate-limited `verify_router` alongside `/v1/verify` — shares the same governor rate limit.

### Task 2: Integration Tests

Added 5 integration tests to `crates/platform/tests/verify_integration.rs` in the existing `http_tests` module:

| Test | Scenario | Expected |
|------|----------|----------|
| `test_verify_attestation_valid` | Valid signed attestation | 200, status=verified, receipt=JWS |
| `test_verify_attestation_bad_signature` | Tampered nonce field | 200, status=failed |
| `test_verify_attestation_malformed_json` | "not json at all" | 400, error=invalid_attestation |
| `test_verify_attestation_wrong_format` | format=wrong-format-v99 | 400, error=invalid_format |
| `test_verify_attestation_missing_signature` | signature=null | 400, error=missing_signature |

## Verification Results

```
cargo build -p trustedge-platform --features http  → success
cargo test ...verify_integration --features http -- verify_attestation → 5/5 passed
cargo test ...verify_integration --features http  → 32/32 passed (all tests)
cargo clippy -p trustedge-platform --features http -- -D warnings → clean
cargo fmt --check → clean
```

## Deviations from Plan

**1. [Rule 2 - Missing functionality] Import sign_receipt_jws for non-postgres builds**

- **Found during:** Task 1
- **Issue:** The plan specified the handler works identically regardless of postgres feature, but `sign_receipt_jws` was only imported under `#[cfg(feature = "postgres")]`. The handler needed the function in both build configurations.
- **Fix:** Added a second `#[cfg(not(feature = "postgres"))]` import for `sign_receipt_jws`.
- **Files modified:** `crates/platform/src/http/handlers.rs`

**2. [Rule 1 - Implementation adjustment] Used trustedge_core::chain::segment_hash instead of blake3::hash**

- **Found during:** Task 1
- **Issue:** `blake3` crate was not a direct dependency of `trustedge-platform`, but it is available via `trustedge-core`. The handler needed BLAKE3 hashing for the manifest digest.
- **Fix:** Used `trustedge_core::chain::segment_hash()` + `BASE64.encode()` — same pattern as `compute_manifest_digest_blake3` already in handlers.rs.
- **Files modified:** `crates/platform/src/http/handlers.rs`

## Self-Check

- [x] Task 1 committed: 9c16dc5
- [x] Task 2 committed: acafa8d
- [x] `crates/platform/src/http/handlers.rs` contains `verify_attestation_handler`
- [x] `crates/platform/src/http/router.rs` contains `/v1/verify-attestation` route
- [x] `crates/platform/tests/verify_integration.rs` contains 5 `test_verify_attestation_*` tests
- [x] All 32 integration tests pass

## Self-Check: PASSED
