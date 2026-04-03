<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


---
phase: 63-error-response-sanitization
plan: 01
subsystem: api
tags: [security, error-handling, http, tracing]

# Dependency graph
requires: []
provides:
  - Sanitized HTTP error responses for crypto verification and receipt signing failures
  - Generic "Cryptographic verification failed" and "Receipt generation failed" messages at all error sites
  - Server-side warn!() logging of full error details preserved
  - Integration tests proving error response sanitization (no library detail leakage)
affects: [platform, http, security-review]

# Tech tracking
tech-stack:
  added: []
  patterns: [Static generic error messages for security-sensitive error paths; full detail logged server-side via warn!()]

key-files:
  created: []
  modified:
    - crates/platform/src/http/handlers.rs
    - crates/platform/src/verify/validation.rs
    - crates/platform/tests/verify_integration.rs

key-decisions:
  - "Replace format!() error strings with static generic messages to prevent crypto library internals from reaching API clients"
  - "Add warn!(Full error detail) before Err return in validation.rs build_receipt_if_requested — closes D-05"
  - "Test uses manifest-missing-signature trigger (not malformed base64) because invalid signatures return HTTP 200 with passed=false, not HTTP 400"

patterns-established:
  - "Security-sensitive error paths: static generic message to client + warn!(full detail) server-side"

requirements-completed: [ERRH-01]

# Metrics
duration: 3min
completed: 2026-03-25
---

# Phase 63 Plan 01: Error Response Sanitization Summary

**Closed security Finding 6: four format!() calls leaking raw crypto error strings replaced with static generic messages; integration test proves no library detail reaches HTTP clients**

## Performance

- **Duration:** 3 min
- **Started:** 2026-03-25T13:02:30Z
- **Completed:** 2026-03-25T13:05:34Z
- **Tasks:** 2
- **Files modified:** 3

## Accomplishments

- Replaced all four `format!("... {}", e)` calls in client-visible error fields with static strings: `"Cryptographic verification failed"` (2 sites in handlers.rs) and `"Receipt generation failed"` (1 site in handlers.rs, 1 site in validation.rs)
- Added `warn!("Failed to sign receipt: {}", e)` before the Err return in `build_receipt_if_requested` (validation.rs had no warn! — added per D-05) plus `use tracing::warn;` import
- All existing `warn!()` calls logging full error detail in handlers.rs preserved unchanged
- Added two integration tests: `test_verify_error_does_not_leak_library_detail` (asserts detail == exact generic string and response body contains none of: "SignatureError", "InvalidSignature", "base64", "decode error", "verification equation", "Missing signature", "anyhow") and `test_verify_success_unaffected_by_sanitization` (regression: success path still returns 200 with passed=true)
- Total integration tests: 24 (was 22); all pass

## Task Commits

Each task was committed atomically:

1. **Task 1: Sanitize error messages in handlers.rs and validation.rs** - `7268eca` (fix)
2. **Task 2: Add integration tests proving error response sanitization** - `e16423d` (test)

## Files Created/Modified

- `crates/platform/src/http/handlers.rs` - Replaced 3 format!() error detail calls with static strings; warn!() calls unchanged
- `crates/platform/src/verify/validation.rs` - Added tracing::warn import; replaced format!() in receipt error; added warn! before Err return
- `crates/platform/tests/verify_integration.rs` - Added tests 14 and 15 in http_tests module

## Decisions Made

- Test for error sanitization uses a manifest with no `signature` field (triggers `Err` from verify_signature via `?`) rather than a malformed base64 signature; the latter returns HTTP 200 with `passed=false` because the engine treats decode errors as non-fatal verification results, not exceptional errors.
- Static message for receipt error changed from "Failed to sign receipt" to "Receipt generation failed" per plan decision D-02 (more generic, does not hint at JWS signing internals).

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - Missing Critical] Added warn!() call and tracing import to validation.rs**
- **Found during:** Task 1 (validation.rs modification)
- **Issue:** Plan D-05 required checking whether a warn!() existed before the receipt error return in build_receipt_if_requested. No warn! existed and no tracing import was present.
- **Fix:** Added `use tracing::warn;` import and `warn!("Failed to sign receipt: {}", e);` before the Err return.
- **Files modified:** crates/platform/src/verify/validation.rs
- **Verification:** cargo test --lib passes; full error detail now logged server-side from both error paths
- **Committed in:** 7268eca (Task 1 commit)

---

**Total deviations:** 1 auto-fixed (missing critical — server-side logging)
**Impact on plan:** Required for correctness per plan spec. No scope creep.

## Issues Encountered

Initial test attempt used a malformed base64 signature to trigger HTTP 400. The test returned 200 instead because the verify engine handles decode errors as `VerificationResult { passed: false }` inside `verify_signature`, not as `Err`. Fixed by using a manifest without a `signature` field, which triggers the `?`-propagated `Err("Missing signature in manifest")` path that causes the handler to return 400.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Finding 6 from security review is closed
- Phase 63 is the only plan in this phase; milestone v2.7 CI & Config Security is complete

---
*Phase: 63-error-response-sanitization*
*Completed: 2026-03-25*

## Self-Check: PASSED

- FOUND: crates/platform/src/http/handlers.rs
- FOUND: crates/platform/src/verify/validation.rs
- FOUND: crates/platform/tests/verify_integration.rs
- FOUND commit: 7268eca (fix: sanitize crypto error responses)
- FOUND commit: e16423d (test: add integration tests proving error response sanitization)
