---
phase: 31-secret-hardening
plan: 03
subsystem: ca
tags: [secret, zeroize, security, login, jwt, ca-config, auth-service, ci]

# Dependency graph
requires:
  - phase: 31-01
    provides: "Secret<T> wrapper type with zeroize, redacted Debug, expose_secret()"
  - phase: 31-02
    provides: "YubiKeyConfig and SoftwareHsmConfig hardened with Secret<String>, builder pattern, [REDACTED] Debug"
provides:
  - "LoginRequest.password is Secret<String> — cannot be printed or serialized by accident"
  - "LoginRequest has no Serialize — compile error prevents accidental serialization"
  - "Custom Deserialize for LoginRequest wraps password in Secret at deserialization boundary"
  - "Manual Debug for LoginRequest redacts password as [REDACTED]"
  - "CAConfig.jwt_secret is Secret<String> with manual Debug, Clone, builder pattern"
  - "AuthService.jwt_secret is Secret<String> wrapped at construction"
  - "CI Step 23 catches forbidden Serialize derives and missing [REDACTED] on all 4 secret structs"
affects: [platform-ca, trust-boundary, ci-pipeline]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "HTTP request body model with Secret<String>: no Serialize, custom Deserialize, manual Debug with [REDACTED]"
    - "Service config with Secret<String>: builder-only construction, private jwt_secret field, manual Debug/Clone"
    - "CI grep regression guard: grep -B2 pub struct | grep Serialize catches re-introduction of forbidden derives"

key-files:
  created: []
  modified:
    - crates/platform/src/ca/models.rs
    - crates/platform/src/ca/mod.rs
    - crates/platform/src/ca/auth.rs
    - scripts/ci-check.sh

key-decisions:
  - "LoginRequest uses custom Deserialize (via private LoginRequestRaw helper) — password wrapped in Secret at the JSON parsing boundary, never exists as bare String after deserialization"
  - "CAConfig builder added alongside Default impl — public API guides callers to use builder, preventing direct struct literal construction that could bypass Secret wrapping"
  - "CI Step 23 uses grep -B2 on struct declarations — catches both derive and cfg_attr-style derives placed before the struct keyword"

patterns-established:
  - "HTTP credential model pattern: no Serialize, custom Deserialize with private raw helper, Secret<String> field, manual Debug"
  - "CI secret regression pattern: grep -B2 pub struct $STRUCT | grep Serialize + grep -q REDACTED in source file"

requirements-completed: [SEC-01, SEC-02, SEC-04]

# Metrics
duration: 5min
completed: 2026-02-22
---

# Phase 31 Plan 03: Platform Secret Hardening Summary

**LoginRequest, CAConfig, and AuthService hardened with Secret<String> — password and JWT secret cannot be printed or serialized by accident; CI Step 23 enforces no-Serialize and [REDACTED] Debug on all 4 secret-holding structs**

## Performance

- **Duration:** 5 min
- **Started:** 2026-02-22T17:47:03Z
- **Completed:** 2026-02-22T17:51:13Z
- **Tasks:** 2
- **Files modified:** 4

## Accomplishments
- `LoginRequest.password` is now `Secret<String>` (private field) — no Serialize derive, compile-time protection
- Custom `Deserialize` impl for `LoginRequest` wraps password in `Secret` at the JSON parsing boundary
- Manual `Debug` for `LoginRequest` prints `[REDACTED]` for password, shows email for diagnostics
- `LoginRequest::new()` and `.password()` getter provide controlled construction and access
- `CAConfig.jwt_secret` is now `Secret<String>` (private field) with `.jwt_secret()` getter
- `CAConfig` gets builder pattern, manual `Debug` (redacts `jwt_secret`), manual `Clone`
- `AuthService.jwt_secret` is `Secret<String>`, wrapped at construction in `AuthService::new()`
- 8 new security tests: LoginRequest debug/deserialize/getter, CAConfig debug/builder/defaults/clone
- CI Step 23 added to `scripts/ci-check.sh`: catches re-introduction of `Serialize` derives and missing `[REDACTED]` impls on all 4 known secret-holding structs

## Task Commits

Each task was committed atomically:

1. **Task 1: Harden LoginRequest, CAConfig, AuthService with Secret<String>** - `402aa77` (feat)
2. **Task 2: Add CI grep check for forbidden derive patterns** - `ddb9efc` (feat)

## Files Created/Modified
- `crates/platform/src/ca/models.rs` - LoginRequest hardened: Secret<String> password, custom Deserialize, no Serialize, manual Debug, 4 new tests
- `crates/platform/src/ca/mod.rs` - CAConfig hardened: Secret<String> jwt_secret, manual Debug/Clone, builder pattern, 4 new tests
- `crates/platform/src/ca/auth.rs` - AuthService hardened: Secret<String> jwt_secret wrapped at construction
- `scripts/ci-check.sh` - Step 23 added: grep checks for Serialize derive and [REDACTED] on YubiKeyConfig, SoftwareHsmConfig, LoginRequest, CAConfig

## Decisions Made
- Custom Deserialize uses a private `LoginRequestRaw` struct — password is a bare `String` only inside the helper, immediately wrapped in `Secret::new()` before the public `LoginRequest` is constructed. This ensures the raw password never escapes the deserialization boundary.
- `CAConfig` builder added alongside `Default` impl — callers see the builder-first API in docs, preventing direct struct literal construction.
- CI Step 23 uses `grep -B2 "pub struct $STRUCT"` — catches both standalone `#[derive(...)]` and inline variants placed directly before the struct keyword.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
None — all tasks completed cleanly.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- All SEC-* requirements (SEC-01, SEC-02, SEC-03, SEC-04) are now complete
- Phase 31 (secret hardening) is fully complete
- CI enforcement added to prevent future regressions on all 4 secret-holding structs
- Ready for Phase 32 (next phase in v1.7 roadmap)

## Self-Check: PASSED

- `crates/platform/src/ca/models.rs` — FOUND
- `crates/platform/src/ca/mod.rs` — FOUND
- `crates/platform/src/ca/auth.rs` — FOUND
- `scripts/ci-check.sh` — FOUND
- `.planning/phases/31-secret-hardening/31-03-SUMMARY.md` — FOUND
- Commit `402aa77` — FOUND
- Commit `ddb9efc` — FOUND

---
*Phase: 31-secret-hardening*
*Completed: 2026-02-22*
