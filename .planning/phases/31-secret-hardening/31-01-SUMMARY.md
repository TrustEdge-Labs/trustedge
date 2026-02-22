---
phase: 31-secret-hardening
plan: 01
subsystem: security
tags: [zeroize, secret-wrapper, memory-safety, redacted-debug]

# Dependency graph
requires: []
provides:
  - "Secret<T> wrapper type at trustedge_core::Secret"
  - "Zeroize+ZeroizeOnDrop on drop for any T: Zeroize"
  - "Redacted Debug output always showing [REDACTED]"
  - "expose_secret() as the only access path — no Deref/Display/Serialize"
affects: [32-audit-hardening, 33-ci-hardening, 34-docs-hardening]

# Tech tracking
tech-stack:
  added: [zeroize derive macros (feature = "derive")]
  patterns: [Secret<T> newtype for sensitive values, expose_secret() explicit access pattern]

key-files:
  created:
    - crates/core/src/secret.rs
  modified:
    - Cargo.toml
    - crates/core/Cargo.toml
    - crates/core/src/lib.rs

key-decisions:
  - "Implemented Secret<T> in-house rather than depending on the secrecy crate — zeroize is already a workspace dep and the API surface needed is small"
  - "Placed pub use secret::Secret after crypto re-exports in lib.rs (rustfmt ordering requirement)"
  - "Used derive(Zeroize, ZeroizeOnDrop) rather than manual impl — cleaner and less error-prone"

patterns-established:
  - "Secret<T>: wrap any sensitive value with Secret::new(), access only via expose_secret()"
  - "Debug for Secret<T>: always writes Secret([REDACTED]) — format!({:?}) is safe to log"
  - "No Display/Deref/Serialize — compile error if accidentally leaked via format!({}) or serde"

requirements-completed: [SEC-01, SEC-02]

# Metrics
duration: 2min
completed: 2026-02-22
---

# Phase 31 Plan 01: Secret<T> Wrapper Type Summary

**In-house Secret<T> newtype deriving Zeroize+ZeroizeOnDrop with redacted Debug and expose_secret() access, re-exported from trustedge_core::Secret**

## Performance

- **Duration:** 2 min
- **Started:** 2026-02-22T17:31:39Z
- **Completed:** 2026-02-22T17:34:04Z
- **Tasks:** 1
- **Files modified:** 4

## Accomplishments
- Created `Secret<T>` in `crates/core/src/secret.rs` with full zeroize-on-drop and redacted Debug
- Enabled zeroize derive macros via `features = ["derive"]` in workspace and core Cargo.toml
- 5 unit tests pass: debug_redacted, expose_secret, clone, debug_in_struct, partial_eq
- Re-exported as `trustedge_core::Secret` — all downstream crates can use immediately
- Full workspace builds and 265+ tests pass with zero regressions

## Task Commits

Each task was committed atomically:

1. **Task 1: Create Secret<T> wrapper type with zeroize derive support** - `9daf8f6` (feat)

**Plan metadata:** (committed with docs below)

## Files Created/Modified
- `crates/core/src/secret.rs` - Secret<T> newtype with Zeroize, ZeroizeOnDrop, redacted Debug, expose_secret(), Clone, PartialEq, and 5 unit tests
- `crates/core/src/lib.rs` - Added `pub mod secret` and `pub use secret::Secret`
- `Cargo.toml` - Changed `zeroize = "1.7"` to `zeroize = { version = "1.7", features = ["derive"] }`
- `crates/core/Cargo.toml` - Same zeroize derive feature update

## Decisions Made
- Implemented Secret<T> in-house rather than adding the `secrecy` crate as a dependency — zeroize is already a workspace dep, and our API surface (new, expose_secret, Debug, Clone, PartialEq) is small enough to own directly.
- Used `#[derive(Zeroize, ZeroizeOnDrop)]` rather than manual `impl Drop` — derive macros handle field-by-field zeroization correctly and are less error-prone.
- No `Display`, `Deref`, `Serialize`, or `Deserialize` impls — by design. Using `{}` or `serde` on a Secret is a compile error, forcing explicit exposure.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] cargo fmt ordering: moved pub use secret::Secret placement**
- **Found during:** Task 1 commit (pre-commit hook)
- **Issue:** Initial placement of `pub use secret::Secret` directly after `pub use archive::...` violated rustfmt's import ordering expectations; pre-commit hook blocked commit
- **Fix:** Ran `cargo fmt --all` which moved the line to the correct position after `pub use crypto::...`
- **Files modified:** crates/core/src/lib.rs
- **Verification:** `cargo fmt --all` reported clean; commit succeeded
- **Committed in:** 9daf8f6 (Task 1 commit)

---

**Total deviations:** 1 auto-fixed (1 blocking — rustfmt ordering)
**Impact on plan:** Minor formatting adjustment only. No scope change, no behavior change.

## Issues Encountered
None — the rustfmt ordering issue was caught by the pre-commit hook and resolved by running `cargo fmt --all`.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- `trustedge_core::Secret` is available for all downstream crates immediately
- Foundation ready for plan 31-02: applying Secret<T> to key material in trustedge-core
- No blockers

---
*Phase: 31-secret-hardening*
*Completed: 2026-02-22*
