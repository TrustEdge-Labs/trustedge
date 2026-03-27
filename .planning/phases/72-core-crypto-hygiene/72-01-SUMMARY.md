---
phase: 72-core-crypto-hygiene
plan: 01
subsystem: crypto
tags: [blake3, bincode, serde_json, envelope, receipts, error-handling]

# Dependency graph
requires: []
provides:
  - "generate_aad() documents infallible intent with .expect() instead of bare .unwrap()"
  - "Envelope::hash() returns Result<[u8; 32]> eliminating silent empty-input hash on serialization error"
  - "receipts/mod.rs production caller propagates hash error with ?"
affects: [any phase using Envelope::hash()]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Infallible serialization documented with .expect() rather than bare .unwrap()"
    - "Fallible crypto ops return Result; production callers propagate with ?; test callers use .unwrap()"

key-files:
  created: []
  modified:
    - crates/core/src/crypto.rs
    - crates/core/src/envelope.rs
    - crates/core/src/applications/receipts/mod.rs

key-decisions:
  - "Use anyhow::anyhow! for envelope hash error to match established pattern in this file"
  - "Test callers use .unwrap() per project conventions — tests should panic on unexpected errors"

patterns-established:
  - "Production crypto paths never silently discard errors — either propagate with ? or document infallibility with .expect()"

requirements-completed: [CORE-01, CORE-02]

# Metrics
duration: ~12min
completed: 2026-03-27
---

# Phase 72 Plan 01: Core Crypto Hygiene Summary

**Eliminate silent error-swallowing in two production crypto paths: generate_aad() .expect() documents infallible AAD serialization, Envelope::hash() returns Result to prevent silent empty-input BLAKE3 hashes**

## Performance

- **Duration:** ~12 min
- **Started:** 2026-03-27T14:00:00Z
- **Completed:** 2026-03-27T14:12:00Z
- **Tasks:** 2
- **Files modified:** 3

## Accomplishments

- `generate_aad()` in crypto.rs now uses `.expect("AAD serialization is infallible")` instead of bare `.unwrap()`, documenting intent explicitly
- `Envelope::hash()` changed from `[u8; 32]` to `Result<[u8; 32]>`, eliminating the `unwrap_or_default()` path that would silently hash empty bytes on serialization error
- Production caller in `receipts/mod.rs assign_ownership` propagates hash error with `?`
- All 15 test call sites updated with `.unwrap()` per project conventions
- All 184 trustedge-core tests pass; clippy and fmt clean

## Task Commits

1. **Task 1: Replace generate_aad() unwrap with expect (CORE-01)** - `e85ea25` (fix)
2. **Task 2: Change Envelope::hash() to return Result and update callers (CORE-02)** - `370d5e3` (fix)

## Files Created/Modified

- `crates/core/src/crypto.rs` - `.unwrap()` -> `.expect("AAD serialization is infallible")` in generate_aad
- `crates/core/src/envelope.rs` - hash() return type `[u8; 32]` -> `Result<[u8; 32]>`, test callers updated
- `crates/core/src/applications/receipts/mod.rs` - production caller uses `?`, test callers use `.unwrap()`

## Decisions Made

- Used `anyhow::anyhow!` for the envelope hash error (consistent with existing patterns in envelope.rs)
- Test callers use `.unwrap()` per project conventions: tests should panic on unexpected errors

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- CORE-01 and CORE-02 requirements satisfied
- Phase 73 (deployment hygiene) can proceed independently; these changes have no cross-phase dependencies
- All workspace tests pass, no known regressions

---
*Phase: 72-core-crypto-hygiene*
*Completed: 2026-03-27*
