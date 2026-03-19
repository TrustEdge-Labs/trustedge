---
phase: 46-envelope-hardening
plan: "02"
subsystem: crypto
tags: [pbkdf2, kdf, security, keyring, backends]

requires: []
provides:
  - "PBKDF2_MIN_ITERATIONS = 300_000 constant in universal.rs"
  - "Builder-level minimum enforcement in KeyDerivationContext::with_iterations"
  - "Builder-level minimum enforcement in KeyContext::with_iterations"
  - "Backend-level minimum enforcement in universal_keyring derive_key_internal"
  - "Backend-level minimum enforcement in keyring.rs derive_key"
  - "Rejection tests at both builder and backend level"
affects: [47-key-encryption]

tech-stack:
  added: []
  patterns:
    - "Belt-and-suspenders: assert! at builder level, error return at backend execution level"
    - "PBKDF2_MIN_ITERATIONS constant as single source of truth for KDF floor"

key-files:
  created: []
  modified:
    - crates/core/src/backends/universal.rs
    - crates/core/src/backends/traits.rs
    - crates/core/src/backends/universal_keyring.rs
    - crates/core/src/backends/keyring.rs
    - crates/core/examples/universal_backend_demo.rs

key-decisions:
  - "Use assert! (panic) at builder level — callers with bad iteration counts are programming errors, not runtime errors"
  - "Use error return at backend level — defense-in-depth if context constructed without using builder"
  - "Use BackendError::OperationFailed (not InvalidParameter which does not exist) in keyring.rs"

patterns-established:
  - "PBKDF2_MIN_ITERATIONS: single constant in universal.rs, referenced via crate::backends::universal:: path in other modules"

requirements-completed: [KDF-01]

duration: 40min
completed: 2026-03-19
---

# Phase 46 Plan 02: PBKDF2 Minimum Iterations Enforcement Summary

**Belt-and-suspenders PBKDF2 minimum of 300,000 iterations enforced at both builder level (assert!) and backend execution level (error return) across all four keyring/universal backend files**

## Performance

- **Duration:** 40 min
- **Started:** 2026-03-19T00:50:41Z
- **Completed:** 2026-03-19T01:30:00Z
- **Tasks:** 1
- **Files modified:** 5

## Accomplishments

- Added `PBKDF2_MIN_ITERATIONS = 300_000` public constant to `universal.rs`
- `KeyDerivationContext::with_iterations()` now panics for iterations below 300,000
- `KeyContext::with_iterations()` in `traits.rs` now panics for iterations below 300,000
- `derive_key_internal()` in `universal_keyring.rs` returns an error if iterations below minimum
- `derive_key()` in `keyring.rs` returns an error if iterations below minimum
- Two new rejection tests added (one per builder type); existing tests updated to use 600,000

## Task Commits

1. **Task 1: Enforce PBKDF2 minimum in builders and backends** - `2c40152` (feat)

## Files Created/Modified

- `crates/core/src/backends/universal.rs` - Added PBKDF2_MIN_ITERATIONS constant, validation in with_iterations, updated test to use 600k, added rejection test
- `crates/core/src/backends/traits.rs` - Added validation in KeyContext::with_iterations
- `crates/core/src/backends/universal_keyring.rs` - Added backend-level guard before pbkdf2_hmac call, updated test iteration count, added should_panic test
- `crates/core/src/backends/keyring.rs` - Added backend-level guard before pbkdf2_hmac call
- `crates/core/examples/universal_backend_demo.rs` - Updated 10_000 demo iterations to 600_000

## Decisions Made

- Used `assert!` (panic) at builder level because callers passing sub-minimum iteration counts are programming errors, not runtime conditions. This fails fast at call site.
- Used `BackendError::OperationFailed` in `keyring.rs` (not `InvalidParameter` which doesn't exist in the `BackendError` enum).
- Backend-level guard uses `unwrap_or(600_000)` before checking — default already exceeds minimum, so default callers are unaffected.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] BackendError::InvalidParameter does not exist**
- **Found during:** Task 1 (backend-level validation in keyring.rs)
- **Issue:** Plan specified `BackendError::InvalidParameter` but the enum only has `UnsupportedOperation`, `KeyNotFound`, `InitializationFailed`, `HardwareError`, `OperationFailed`
- **Fix:** Used `BackendError::OperationFailed` instead
- **Files modified:** crates/core/src/backends/keyring.rs
- **Verification:** Clippy passes with no warnings; semantics are equivalent for this validation case
- **Committed in:** 2c40152 (Task 1 commit)

---

**Total deviations:** 1 auto-fixed (Rule 1 - bug in plan specification)
**Impact on plan:** Minor fix, no scope change. Error variant semantics unchanged from caller perspective.

## Issues Encountered

- The `universal_keyring` module is behind the `keyring` feature flag — its tests only run with `--features keyring`. Confirmed all 6 tests pass including `test_pbkdf2_minimum_iterations_rejected`.
- `test_many_keys` in software_hsm is a pre-existing slow test that exceeds 60-second timeout; not related to these changes.

## Next Phase Readiness

- KDF-01 satisfied: any PBKDF2 call below 300,000 iterations fails at two independent enforcement points
- Default 600,000 iterations unchanged; all existing code continues to work
- Phase 47 (key encryption) can rely on these guards being in place

## Self-Check: PASSED

- `crates/core/src/backends/universal.rs` — FOUND, contains PBKDF2_MIN_ITERATIONS (3x) and 300_000 (1x)
- `crates/core/src/backends/traits.rs` — FOUND, contains PBKDF2_MIN_ITERATIONS (2x)
- `crates/core/src/backends/universal_keyring.rs` — FOUND, contains PBKDF2_MIN_ITERATIONS (2x), rejection test
- `crates/core/src/backends/keyring.rs` — FOUND, contains PBKDF2_MIN_ITERATIONS (2x)
- `crates/core/examples/universal_backend_demo.rs` — FOUND, no 10_000 iterations
- Commit `2c40152` — FOUND in git log

---
*Phase: 46-envelope-hardening*
*Completed: 2026-03-19*
