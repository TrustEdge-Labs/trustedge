---
phase: 34-platform-testing
plan: "02"
subsystem: testing
tags: [axum, integration-tests, cors, jws, ed25519, http, router, jwks]

# Dependency graph
requires:
  - phase: 33-platform-quality
    provides: CORS hardening, build_base_router precondition for refactor
provides:
  - build_base_router shared router function (single source of truth for base routes)
  - create_test_app delegates to create_router (middleware fidelity)
  - 4 new HTTP integration tests: CORS parity, verify round-trip, JWKS receipt verification, wrong-key negative
affects: [34-03, future platform testing phases]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - build_base_router pattern: extract base routes before feature-gated additions
    - Shared AppState clone for multi-router tests (AppState is Clone)
    - JWS round-trip verification: POST /v1/verify → GET /.well-known/jwks.json → verify Ed25519 sig

key-files:
  created: []
  modified:
    - crates/platform/src/http/router.rs
    - crates/platform/src/http/mod.rs
    - crates/platform/src/http/handlers.rs
    - crates/platform/tests/verify_integration.rs

key-decisions:
  - "build_base_router returns Router<AppState> (unfinalized) so create_router can add postgres routes before with_state"
  - "create_test_app delegates entirely to create_router — no duplicated route definitions, middleware stack identical"
  - "CORS parity test uses two independent router instances from cloned AppState, not test/prod comparison"
  - "JWS receipt JWKS 'x' field uses standard base64 (not url-safe) — matches jwks.rs BASE64.encode convention"
  - "test_verify_wrong_key: expects HTTP 200 with passed=false, not an error status code"

patterns-established:
  - "Multi-router test: clone AppState, construct two routers, compare headers — proves build_base_router parity"
  - "Round-trip test pattern: build_signed_manifest helper + build_verify_body helper reduces test boilerplate"

requirements-completed: [TST-02, TST-03]

# Metrics
duration: 6min
completed: 2026-02-22
---

# Phase 34 Plan 02: Router Parity Refactor and Verify Round-Trip Tests Summary

**Extracted build_base_router for middleware parity and added 4 HTTP integration tests covering CORS consistency, full sign-then-verify over HTTP, JWS receipt Ed25519 verification against JWKS, and wrong-key failure path**

## Performance

- **Duration:** 6 min
- **Started:** 2026-02-22T23:57:48Z
- **Completed:** 2026-02-22T23:03:59Z
- **Tasks:** 2
- **Files modified:** 4

## Accomplishments

- Extracted `build_base_router()` in router.rs as the single source of truth for base routes (verify, jwks, health); `create_router` calls it instead of inlining
- `create_test_app` (postgres variant) now delegates to `create_router` — no more duplicated route list, CORS and TraceLayer now applied in test mode (TST-02 fidelity gap closed)
- 4 new integration tests in verify_integration.rs bringing total to 11 (was 7): CORS parity, round-trip, receipt JWKS verification, wrong-key failure

## Task Commits

1. **Task 1: Extract shared router builder and refactor create_test_app** - `42f349f` (refactor)
2. **Task 2: Add CORS parity test and full verify round-trip tests** - `fa01b51` (feat, included in 34-01 commit due to concurrent execution)

## Files Created/Modified

- `crates/platform/src/http/router.rs` - Added `build_base_router()` function; `create_router` now calls it
- `crates/platform/src/http/mod.rs` - Re-export `build_base_router` alongside `create_router`
- `crates/platform/src/http/handlers.rs` - `create_test_app` now delegates to `crate::http::create_router(state)` instead of duplicating routes
- `crates/platform/tests/verify_integration.rs` - Added `make_state()`, `build_signed_manifest()`, `build_verify_body()` helpers; 4 new tests: `test_cors_preflight_parity`, `test_verify_round_trip`, `test_verify_receipt_matches_jwks`, `test_verify_wrong_key_returns_failed_signature`

## Decisions Made

- `build_base_router()` returns `Router<AppState>` (not finalized with state) so `create_router` can chain postgres-gated routes before calling `.with_state()` — this is Axum's type-state constraint
- `create_test_app` calls `create_router(state)` directly — simplest approach that guarantees middleware parity with zero duplication
- CORS parity test compares two independently constructed router instances (both from `create_router`) rather than comparing test vs production path — proves build_base_router determinism
- The JWKS `x` field is standard base64 (RFC 4648 with padding), not URL-safe — verified against jwks.rs source and used `BASE64.decode()` accordingly
- `test_verify_wrong_key_returns_failed_signature` asserts HTTP 200 (server completed verification) not an error code, and `receipt=null` for the failure case

## Deviations from Plan

None — plan executed exactly as written. The Task 2 tests ended up committed in the 34-01 commit (`fa01b51`) due to concurrent plan execution in the same git session, but all code changes are correctly implemented and passing.

## Issues Encountered

None significant. CI copyright header and security audit failures are pre-existing (node_modules, experimental/target files, sqlx 0.7.4 advisory) — confirmed by checking baseline before my changes.

## Next Phase Readiness

- Router refactor complete: any future test utilities that construct the app should use `create_router(state)` for guaranteed middleware parity
- All 11 integration tests passing under both `--features http` and default (no features) variants
- `build_base_router` exported from `http` mod for use in external test utilities if needed
- Ready for phase 34-03 (if any additional platform testing plans)

## Self-Check: PASSED

- `crates/platform/src/http/router.rs` — FOUND (contains `fn build_base_router`)
- `crates/platform/src/http/mod.rs` — FOUND (exports `build_base_router, create_router`)
- `crates/platform/src/http/handlers.rs` — FOUND (create_test_app delegates to `crate::http::create_router`)
- `crates/platform/tests/verify_integration.rs` — FOUND (644 lines, 11 tests passing)
- `.planning/phases/34-platform-testing/34-02-SUMMARY.md` — FOUND (this file)
- Commit `42f349f` — FOUND (Task 1: router refactor)
- Commit `fa01b51` — FOUND (Task 2: new integration tests)

---
*Phase: 34-platform-testing*
*Completed: 2026-02-22*
