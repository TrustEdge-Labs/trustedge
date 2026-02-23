---
phase: 34-platform-testing
plan: 01
subsystem: testing
tags: [axum, tower, integration-tests, platform-server, env-config]

# Dependency graph
requires:
  - phase: 33-platform-quality
    provides: trustedge-platform-server binary crate with Config, AppState, create_router
provides:
  - Platform-server wiring integration tests (5 tests) exercising Config env loading, AppState construction, and router responses
affects: [ci, platform-server]

# Tech tracking
tech-stack:
  added: [tower (util feature, dev-dep), serde_json (dev-dep) in platform-server]
  patterns: [per-process Mutex for env-var isolation in integration tests, tower::ServiceExt::oneshot for HTTP integration testing]

key-files:
  created:
    - crates/platform-server/tests/wiring.rs
  modified:
    - crates/platform-server/Cargo.toml
    - crates/platform/tests/verify_integration.rs (cargo fmt reformatting only)

key-decisions:
  - "Axum returns 422 Unprocessable Entity (not 400) for JSON extraction failures — test asserts UNPROCESSABLE_ENTITY"
  - "Per-process Mutex (env_lock) serializes env-var tests to prevent parallel-thread races on PORT variable"
  - "serde_json added as dev-dep — not transitively available in integration test binary without explicit declaration"
  - "Tests run with --no-default-features to target verify-only mode (drops postgres, retains http + openapi)"

patterns-established:
  - "OnceLock<Mutex<()>> pattern for env-var test serialization in Rust integration tests"
  - "tower::ServiceExt::oneshot pattern for Axum router testing without a running server"

requirements-completed: [TST-01]

# Metrics
duration: 3min
completed: 2026-02-23
---

# Phase 34 Plan 01: Platform-Server Wiring Integration Tests Summary

**5-test wiring suite for trustedge-platform-server validating Config env loading, AppState construction, and Axum router health/verify responses in verify-only mode**

## Performance

- **Duration:** 3 min
- **Started:** 2026-02-22T23:57:46Z
- **Completed:** 2026-02-23T00:00:17Z
- **Tasks:** 1
- **Files modified:** 3 (created 1, modified 2)

## Accomplishments

- Created `crates/platform-server/tests/wiring.rs` with 5 integration tests covering all startup wiring paths
- Added dev-dependencies (tower, axum, tokio, serde_json) to platform-server Cargo.toml
- All 5 tests pass under `--no-default-features` (verify-only mode): Config defaults, custom PORT, invalid PORT fallback, AppState+router health, verify endpoint rejection

## Task Commits

1. **Task 1: Add dev-dependencies and create wiring integration tests** - `fa01b51` (feat)

**Plan metadata:** (included in task commit)

## Files Created/Modified

- `crates/platform-server/tests/wiring.rs` - 5 wiring integration tests (Config env loading, AppState construction, router health, verify rejection)
- `crates/platform-server/Cargo.toml` - Added [dev-dependencies]: tower (util), axum, tokio, serde_json
- `crates/platform/tests/verify_integration.rs` - cargo fmt reformatting only (pre-existing style drift)

## Decisions Made

- **Axum 422 vs 400:** Axum's JSON extractor returns 422 Unprocessable Entity (not 400 Bad Request) for deserialization failures. The plan specified 400, but the actual behavior is 422 — test was updated to assert `UNPROCESSABLE_ENTITY`.
- **Env-var serialization:** Tests that set/clear PORT run in parallel threads sharing one process. Added `OnceLock<Mutex<()>>` guard to serialize env-var tests and eliminate flaky test races.
- **serde_json dev-dep:** Integration test binaries don't inherit transitive deps from the main crate. Added `serde_json` as an explicit dev-dependency.
- **`--no-default-features` target:** Platform-server defaults to `postgres` feature. Tests run without default features to exercise verify-only code paths (Config without database_url, AppState without db_pool).

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Added serde_json as explicit dev-dependency**
- **Found during:** Task 1 (compilation of wiring.rs)
- **Issue:** `serde_json` used in tests but not listed as dev-dependency; integration test binaries do not inherit transitive deps
- **Fix:** Added `serde_json = { workspace = true }` to `[dev-dependencies]` in Cargo.toml
- **Files modified:** `crates/platform-server/Cargo.toml`
- **Verification:** Compilation succeeded after addition
- **Committed in:** fa01b51 (Task 1 commit)

**2. [Rule 1 - Bug] Fixed test assertion: 422 not 400 for JSON rejection**
- **Found during:** Task 1 (test run)
- **Issue:** Plan specified `HTTP 400` for empty-body verify request, but Axum JSON extractor returns `422 Unprocessable Entity`
- **Fix:** Changed assert to `axum::http::StatusCode::UNPROCESSABLE_ENTITY` and updated doc comment
- **Files modified:** `crates/platform-server/tests/wiring.rs`
- **Verification:** Test passes with correct status code
- **Committed in:** fa01b51 (Task 1 commit)

**3. [Rule 1 - Bug] Fixed env-var test races with per-process Mutex**
- **Found during:** Task 1 (test run — `test_config_from_env_custom_port` failed intermittently)
- **Issue:** Parallel test threads racing on `PORT` env var — `set_var("PORT", "9999")` in one thread overwritten by `remove_var("PORT")` in another
- **Fix:** Added `OnceLock<Mutex<()>>` guard (`env_lock()`) acquired at the start of each env-var test
- **Files modified:** `crates/platform-server/tests/wiring.rs`
- **Verification:** All 5 tests pass consistently
- **Committed in:** fa01b51 (Task 1 commit)

---

**Total deviations:** 3 auto-fixed (1 blocking dep, 2 bug — wrong status code + race condition)
**Impact on plan:** All auto-fixes necessary for correctness. No scope creep.

## Issues Encountered

None beyond the auto-fixed deviations above.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Platform-server wiring tests complete: `cargo test -p trustedge-platform-server --no-default-features --test wiring` passes 5/5
- Ready to proceed with remaining 34-platform-testing plans

---
*Phase: 34-platform-testing*
*Completed: 2026-02-23*

## Self-Check: PASSED

- FOUND: crates/platform-server/tests/wiring.rs (191 lines, min 75 required)
- FOUND: crates/platform-server/Cargo.toml (contains [dev-dependencies])
- FOUND: .planning/phases/34-platform-testing/34-01-SUMMARY.md
- FOUND: commit fa01b51
