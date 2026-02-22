---
phase: 28-platform-server-binary
plan: 01
subsystem: infra
tags: [axum, clap, tokio, tracing, postgres, binary, server]

# Dependency graph
requires:
  - phase: 27-platform-service
    provides: trustedge-platform crate with Config::from_env, create_router, AppState, database module
provides:
  - trustedge-platform-server binary crate at crates/platform-server/
  - serve subcommand: Axum HTTP server on configured PORT with graceful shutdown
  - migrate subcommand: runs embedded sqlx migrations via trustedge_platform::database::run_migrations
  - Startup banner: name, version, port, mode (verify-only vs full/postgres), active routes
affects: [29-dashboard-move, 30-dashboard-delete]

# Tech tracking
tech-stack:
  added:
    - tracing-subscriber 0.3 (env-filter feature) for structured logging
    - axum direct dependency for axum::serve() call
  patterns:
    - Thin binary pattern: main.rs is pure wiring — zero routing logic, all routes in trustedge-platform
    - Feature gate forwarding: postgres feature in platform-server re-exports trustedge-platform/postgres
    - Runtime mode detection: verify-only vs full determined by cfg(feature="postgres"), not DATABASE_URL

key-files:
  created:
    - crates/platform-server/Cargo.toml
    - crates/platform-server/src/main.rs
  modified:
    - Cargo.toml (workspace members)
    - crates/platform/tests/verify_integration.rs (http_tests cfg guard)
    - crates/platform/tests/platform_integration.rs (test-utils cfg guard)

key-decisions:
  - "postgres is always compiled into the binary (default feature); verify-only is not a compile-time decision"
  - "axum added as direct dependency to platform-server to call axum::serve() directly"
  - "verify_integration http_tests guarded with not(feature=postgres) since AppState requires db_pool when postgres is active"

patterns-established:
  - "Binary crate pattern: clap + tokio::main + Config::from_env + AppState + create_router + with_graceful_shutdown"
  - "Shutdown signal: tokio::select! over ctrl_c and SIGTERM (unix) / pending (non-unix)"

requirements-completed: [PLAT-01, PLAT-02, PLAT-03, PLAT-04]

# Metrics
duration: 4min
completed: 2026-02-22
---

# Phase 28 Plan 01: Platform Server Binary Summary

**Standalone `trustedge-platform-server` binary with clap CLI (serve/migrate), tracing banner, AppState wiring, and SIGTERM+Ctrl+C graceful shutdown via tokio::signal**

## Performance

- **Duration:** 4 min
- **Started:** 2026-02-22T04:15:26Z
- **Completed:** 2026-02-22T04:19:28Z
- **Tasks:** 2
- **Files modified:** 5

## Accomplishments

- New `crates/platform-server` binary crate registered in workspace — `cargo build -p trustedge-platform-server` succeeds
- `trustedge-platform-server serve` starts Axum HTTP on configured PORT, prints startup banner (name, version, port, mode, routes), handles SIGTERM and Ctrl+C with graceful drain
- `trustedge-platform-server migrate` runs embedded sqlx migrations via platform's `run_migrations()`
- Zero routing logic in main.rs — all routes delegated to `trustedge_platform::http::create_router()`
- Fixed two pre-latent test compilation issues in trustedge-platform exposed by the new crate enabling `postgres` feature workspace-wide

## Task Commits

Each task was committed atomically:

1. **Task 1: Create crates/platform-server crate with Cargo.toml and register in workspace** - `33e6af1` (chore)
2. **Task 2: Implement main.rs — clap CLI, startup banner, AppState wiring, graceful shutdown** - `54138d9` (feat)

## Files Created/Modified

- `crates/platform-server/Cargo.toml` - Binary crate manifest; postgres (default), ca (optional) features; axum + tracing-subscriber deps
- `crates/platform-server/src/main.rs` - Thin entry point: clap derive, tracing init, Config::from_env, AppState wiring, axum::serve with graceful shutdown
- `Cargo.toml` - Workspace member registration for crates/platform-server
- `crates/platform/tests/verify_integration.rs` - http_tests module guarded with `#[cfg(all(feature = "http", not(feature = "postgres")))]`
- `crates/platform/tests/platform_integration.rs` - cfg guard updated to require `feature = "test-utils"` for create_test_app availability

## Decisions Made

- `postgres` is always compiled into the binary (default feature); the plan context says verify-only is a runtime decision, but the binary always includes postgres so mode is compile-time. Simplifies AppState construction.
- `axum` added as direct dependency to `platform-server` rather than routing through `trustedge-platform`'s re-exports — `axum::serve()` is called directly in main.rs.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Fixed verify_integration http_tests: AppState missing db_pool when postgres enabled**
- **Found during:** Task 2 verification (cargo test --workspace)
- **Issue:** verify_integration.rs `http_tests` module constructed `AppState { keys: ... }` without `db_pool`. Previously harmless because no workspace member enabled postgres. Adding platform-server (which enables postgres by default) caused compilation failure.
- **Fix:** Changed cfg guard from `#[cfg(feature = "http")]` to `#[cfg(all(feature = "http", not(feature = "postgres")))]`
- **Files modified:** `crates/platform/tests/verify_integration.rs`
- **Verification:** `cargo test --workspace` passes all 265+ tests
- **Committed in:** `54138d9` (Task 2 commit)

**2. [Rule 1 - Bug] Fixed platform_integration: create_test_app unavailable without test-utils feature**
- **Found during:** Task 2 verification (cargo test --workspace)
- **Issue:** platform_integration.rs imports `handlers::create_test_app` which is gated by `feature = "test-utils"`. The file was previously compiled only when http+postgres were both active, but `test-utils` was not in the cfg guard. Adding platform-server exposed this.
- **Fix:** Added `feature = "test-utils"` to the `#![cfg(...)]` inner attribute at top of file
- **Files modified:** `crates/platform/tests/platform_integration.rs`
- **Verification:** `cargo test --workspace` passes; platform_integration shows 0 tests (correctly excluded without test-utils)
- **Committed in:** `54138d9` (Task 2 commit)

---

**Total deviations:** 2 auto-fixed (both Rule 1 - pre-latent bugs exposed by new workspace member)
**Impact on plan:** Both fixes were necessary for test suite integrity. No scope creep. The fixes make the test guards correct by construction.

## Issues Encountered

- Initial `platform-server/Cargo.toml` didn't declare `postgres` as an own feature, causing `#[cfg(feature = "postgres")]` to produce "unexpected cfg" warnings and `AppState` construction errors. Fixed by adding `postgres = ["trustedge-platform/postgres"]` as a proper feature with `default = ["postgres"]`.
- `axum` was not initially in platform-server's direct dependencies, causing `axum::serve()` to be unresolved. Fixed by adding `axum = { workspace = true }` to Cargo.toml.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- `trustedge-platform-server` binary is deployable: `cargo build -p trustedge-platform-server --release`
- Phase 29 (dashboard move) is independent and can proceed
- Phase 30 (dashboard delete) can proceed after Phase 29

---
*Phase: 28-platform-server-binary*
*Completed: 2026-02-22*
