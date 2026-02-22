---
phase: 33-platform-quality
plan: 02
subsystem: api
tags: [cors, security, tower-http, axum, ca, pki]

# Dependency graph
requires: []
provides:
  - Restrictive CORS for verify-only builds (CorsLayer::new() blocks all cross-origin)
  - Header-restricted CORS for postgres builds (Content-Type, Authorization, Accept only)
  - CA api.rs with plain service functions and no Axum coupling
  - Status doc comments on all CA sub-modules
affects:
  - ca
  - http
  - security

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "CORS hardening: CorsLayer::new() (deny-all) for verify-only, explicit header list for postgres"
    - "Library-only CA module: plain async service functions instead of Axum handlers"

key-files:
  created: []
  modified:
    - crates/platform/src/http/router.rs
    - crates/platform/src/ca/mod.rs
    - crates/platform/src/ca/api.rs
    - crates/platform/src/ca/auth.rs
    - crates/platform/src/ca/database.rs
    - crates/platform/src/ca/service.rs
    - crates/platform/src/ca/models.rs
    - crates/platform/src/ca/error.rs
    - crates/platform/src/lib.rs

key-decisions:
  - "CorsLayer::new() used for verify-only build — denies all cross-origin by tower-http default, no explicit deny needed"
  - "CA api.rs validate functions return CAError instead of String — cleaner for library-only callers"
  - "Removed #[cfg(feature = 'http')] gate from pub mod api — api.rs has no HTTP deps so it should compile with just ca feature"

patterns-established:
  - "Library-only CA pattern: service functions accept plain typed args, return typed results, no framework coupling"
  - "Status doc comment convention: each CA sub-module opens with Status: Library-only and a one-line description"

requirements-completed: [PLT-02, PLT-03]

# Metrics
duration: 4min
completed: 2026-02-22
---

# Phase 33 Plan 02: CORS Hardening and CA Module Decoupling Summary

**Restrictive CORS for both platform build variants (deny-all for verify-only, header-explicit for postgres) and CA api.rs refactored to plain service functions with zero Axum coupling**

## Performance

- **Duration:** ~4 min
- **Started:** 2026-02-22T19:32:00Z
- **Completed:** 2026-02-22T19:36:00Z
- **Tasks:** 2
- **Files modified:** 9

## Accomplishments
- Replaced `CorsLayer::permissive()` with `CorsLayer::new()` in verify-only build — browser blocks all cross-origin requests
- Replaced `tower_http::cors::Any` header allowlist with explicit `[CONTENT_TYPE, AUTHORIZATION, ACCEPT]` in postgres build
- Removed all Axum imports from `ca/api.rs` — converted handlers to plain async service functions
- Removed `pub fn create_router()` and `pub type AppState` from `ca/api.rs`
- Removed `#[cfg(feature = "http")]` gate on `pub mod api` — CA api now compiles with just the `ca` feature
- Updated `validate_certificate_request` to return `CAError` instead of `String`
- Added library-only status doc comments to all five CA sub-modules
- Updated `ca/mod.rs` module doc to clearly state library-only status with future HTTP exposure note

## Task Commits

Each task was committed atomically:

1. **Task 1: Harden CORS policy for both build variants** - `3e2d94f` (fix)
2. **Task 2: Refactor CA module as library-only and annotate sub-modules** - `fab5351` (refactor)

**Plan metadata:** `75bf1a4` (docs: complete plan)

## Files Created/Modified
- `crates/platform/src/http/router.rs` - CORS hardening: deny-all for verify-only, explicit headers for postgres
- `crates/platform/src/ca/mod.rs` - Library-only doc comment, removed http feature gate on api
- `crates/platform/src/ca/api.rs` - Removed all Axum coupling, converted handlers to plain service functions
- `crates/platform/src/ca/auth.rs` - Added library-only status doc comment
- `crates/platform/src/ca/database.rs` - Added library-only status doc comment
- `crates/platform/src/ca/service.rs` - Added library-only status doc comment
- `crates/platform/src/ca/models.rs` - Added stable types status doc comment
- `crates/platform/src/ca/error.rs` - Added stable error enum status doc comment
- `crates/platform/src/lib.rs` - Added comment noting CA module is library-only

## Decisions Made
- `CorsLayer::new()` used for verify-only build — per tower-http docs, default denies all cross-origin (no explicit deny config needed)
- `validate_certificate_request` and `validate_revocation_request` now return `CAError` instead of `String` — cleaner for library callers and consistent with the module's error type
- `#[cfg(feature = "http")]` gate removed from `pub mod api` — since api.rs no longer imports axum, it compiles fine with just the `ca` feature

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
None.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- CORS security hardening complete for both platform build variants
- CA module is cleanly decoupled from Axum — ready for future HTTP wiring via thin handler shims when needed
- All 38 unit tests and 7 integration tests pass across all feature combinations

---
*Phase: 33-platform-quality*
*Completed: 2026-02-22*
