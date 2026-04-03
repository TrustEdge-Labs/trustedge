<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


---
phase: 58-platform-fixes
plan: 01
subsystem: api
tags: [axum, cors, postgres, platform, http]

# Dependency graph
requires:
  - phase: 57-core-crypto-hardening
    provides: Zeroize/Drop on key-holding structs and PBKDF2 minimum iteration enforcement
provides:
  - Optional OrgContext extraction in postgres verify_handler (no 500 for unauthenticated callers)
  - CORS_ORIGINS env var support for production deployments
affects: [59-platform-fixes, 60-platform-fixes, platform server deployment]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Option<Extension<T>> for Axum extractors on public routes that are also used in authenticated contexts"
    - "AllowOrigin::list() for dynamic CORS origin configuration from environment"

key-files:
  created: []
  modified:
    - crates/platform/src/http/handlers.rs
    - crates/platform/src/http/router.rs

key-decisions:
  - "Option<Extension<OrgContext>> for verify_handler: public /v1/verify route must accept unauthenticated requests; uuid::Uuid::nil() as sentinel org_id for tenant-agnostic DB operations"
  - "CORS_ORIGINS env var: comma-separated, falls back to localhost:3000,localhost:8080; invalid entries skipped with warn; AllowOrigin::list() for Vec<HeaderValue> compatibility"

patterns-established:
  - "Public routes that optionally participate in auth context use Option<Extension<T>> — never panic on missing extension"
  - "Env-var CORS: read at router construction, log active origins at info level, skip invalid with warn"

requirements-completed: [PLAT-01, PLAT-02]

# Metrics
duration: 15min
completed: 2026-03-24
---

# Phase 58 Plan 01: Platform Fixes Summary

**Optional OrgContext in postgres verify_handler via Option<Extension<OrgContext>>, and env-driven CORS origins via CORS_ORIGINS fallback to localhost pair**

## Performance

- **Duration:** ~15 min
- **Started:** 2026-03-24T12:45:00Z
- **Completed:** 2026-03-24T13:00:00Z
- **Tasks:** 2
- **Files modified:** 2

## Accomplishments

- Fixed PLAT-01: postgres `verify_handler` now accepts requests without OrgContext; public callers no longer get 500 from missing Axum extension
- Fixed PLAT-02: CORS allowed origins driven by `CORS_ORIGINS` env var with localhost fallback; enables production deployments to configure CORS without recompiling
- All 22 HTTP verify integration tests pass; full workspace test suite (406+ tests) passes

## Task Commits

Each task was committed atomically:

1. **Task 1: Make OrgContext optional in postgres verify_handler (PLAT-01)** - `6e273dc` (fix)
2. **Task 2: Read CORS origins from CORS_ORIGINS env var (PLAT-02)** - `fe8f796` (fix)

**Plan metadata:** (docs commit — see below)

## Files Created/Modified

- `crates/platform/src/http/handlers.rs` - Changed `Extension<OrgContext>` to `Option<Extension<OrgContext>>`; device lookup skipped when absent; `uuid::Uuid::nil()` used as sentinel org_id for create_verification; debug log added for tenant-agnostic mode
- `crates/platform/src/http/router.rs` - Replaced hardcoded localhost CORS origins with `std::env::var("CORS_ORIGINS")`; fallback to `"http://localhost:3000,http://localhost:8080"`; uses `AllowOrigin::list()` for Vec compatibility

## Decisions Made

- `uuid::Uuid::nil()` (all-zeros UUID) chosen as sentinel org_id for tenant-agnostic DB writes — no new column or nullable change needed, nil UUID is a clear convention
- `AllowOrigin::list()` used instead of direct `Vec<HeaderValue>` because tower-http's `Into<AllowOrigin>` for Vec is not a blanket impl in all versions; explicit call is safer
- Debug log (not warn/info) for absent OrgContext — expected on public routes, not an error condition

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None.

## User Setup Required

**Production deployments** can now set `CORS_ORIGINS=https://app.example.com,https://api.example.com` in environment to configure allowed CORS origins. Without this variable, the service falls back to localhost:3000 and localhost:8080 (suitable for development only).

## Next Phase Readiness

- Platform postgres mode fully operational for unauthenticated /v1/verify callers
- CORS configurable for production without code changes
- Remaining v2.6 phases (59, 60) can proceed

---
*Phase: 58-platform-fixes*
*Completed: 2026-03-24*
