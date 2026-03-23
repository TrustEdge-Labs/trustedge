---
phase: 55-platform-http-hardening
plan: 02
subsystem: platform
tags: [jwks, key-management, security, permissions, env-var]

requires:
  - phase: 55-platform-http-hardening
    provides: "tower-http limit feature added by plan 55-01"

provides:
  - "KeyManager reads JWKS_KEY_PATH env var, defaults to system temp dir (not target/dev/)"
  - "new_with_path() public constructor for deterministic test paths"
  - "Signing key file gets 0600 Unix permissions on write"
  - "jwks.json co-located with signing key (same directory)"
  - "4 integration tests proving path config, permissions, co-location, and non-target/dev default"

affects: [platform-http-hardening, platform-server, verify-integration-tests]

tech-stack:
  added: []
  patterns:
    - "Env-var-driven file path with temp dir fallback: std::env::var().unwrap_or_else(|_| temp_dir()...)"
    - "0600 permissions on sensitive key files via #[cfg(unix)] PermissionsExt"
    - "Co-located companion file (jwks.json) derived from parent dir of primary file"

key-files:
  created: []
  modified:
    - crates/platform/src/verify/jwks.rs
    - crates/platform-server/src/main.rs
    - crates/platform/tests/verify_integration.rs
    - Cargo.toml

key-decisions:
  - "KeyManager stores key_path as struct field so save_to_file/write_jwks_file/rotate_key all use it without passing args"
  - "jwks_path() derives from signing key's parent directory — always co-located, no separate config needed"
  - "new_with_path() made public for test isolation — allows custom temp dirs without touching env vars"
  - "Fixed tower-http missing limit feature (Rule 3) — was blocking build from plan 55-01"

patterns-established:
  - "Env-var key path pattern: std::env::var(KEY).unwrap_or_else(|_| temp_dir().join(filename).to_string_lossy().into_owned())"
  - "Unix permissions on sensitive files: #[cfg(unix)] block with PermissionsExt::from_mode(0o600)"

requirements-completed: [HTTP-03, HTTP-04]

duration: 33min
completed: 2026-03-23
---

# Phase 55 Plan 02: JWKS Key Path Hardening Summary

**KeyManager signing key moved from hardcoded target/dev/ to JWKS_KEY_PATH env var (temp dir default), with 0600 Unix permissions and 4 tests proving path configuration, permissions, and co-location.**

## Performance

- **Duration:** 33 min
- **Started:** 2026-03-23T15:45:00Z
- **Completed:** 2026-03-23T16:18:29Z
- **Tasks:** 2
- **Files modified:** 4

## Accomplishments

- KeyManager now reads `JWKS_KEY_PATH` env var; defaults to system temp directory (not `target/dev/`)
- All 4+ hardcoded `"target/dev/"` string references removed from `jwks.rs`
- Signing key file gets 0600 Unix permissions immediately after write
- `new_with_path()` public constructor enables test isolation with deterministic paths
- `jwks.json` always co-located with signing key via derived parent-dir path
- Platform server logs JWKS key path at startup
- 4 new integration tests: custom path, default not target/dev, permissions, co-location

## Task Commits

1. **Task 1: Refactor KeyManager with env-var key path and 0600 permissions** - `ce807ef` (feat)
2. **Task 2: Add JWKS key path configuration and permissions tests** - included in `2f720d5` (test, plan 55-01)

## Files Created/Modified

- `crates/platform/src/verify/jwks.rs` - Added `key_path` field, `new_with_path()`, env-var reading, 0600 permissions, derived jwks path
- `crates/platform-server/src/main.rs` - Added JWKS key path startup log line
- `crates/platform/tests/verify_integration.rs` - Added 4 JWKS key path tests
- `Cargo.toml` - Added `limit` feature to `tower-http` workspace dep (deviation fix)

## Decisions Made

- Stored `key_path` as a struct field on `KeyManager` so all methods (`save_to_file`, `write_jwks_file`, `rotate_key`) use it without passing path arguments — clean ownership model
- `jwks_path()` private method derives `jwks.json` path from signing key's parent directory — eliminates second config variable
- Made `new_with_path()` public for test isolation rather than manipulating `JWKS_KEY_PATH` env var in tests
- Added anyhow `with_context()` to all file I/O errors for actionable error messages

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Added `limit` feature to `tower-http` workspace dependency**
- **Found during:** Task 1 build verification
- **Issue:** `tower_http::limit::RequestBodyLimitLayer` (added by plan 55-01) requires `limit` feature in `tower-http`. Workspace `Cargo.toml` only declared `cors` and `trace` features. Build failed with `could not find 'limit' in 'tower_http'`
- **Fix:** Added `"limit"` to `tower-http` features in workspace `Cargo.toml`
- **Files modified:** `Cargo.toml`
- **Verification:** `cargo build -p trustedge-platform --features http` succeeded
- **Committed in:** `ce807ef` (Task 1 commit)

---

**Total deviations:** 1 auto-fixed (1 blocking — missing feature flag from parallel plan)
**Impact on plan:** The fix was essential to unblock the build. Plan 55-01 added `RequestBodyLimitLayer` but the workspace feature was not updated; fixing it here enables both plans to build together.

## Issues Encountered

- Parallel plan 55-01 ran on top of this plan's `ce807ef` commit and picked up the `verify_integration.rs` test file changes I had written to disk. The tests were included in plan 55-01's test commit (`2f720d5`) rather than as a separate Task 2 commit. Both task changes are committed and all tests pass.

## Next Phase Readiness

- JWKS key path fully configurable via `JWKS_KEY_PATH` — production deployments can set this to a secure path
- No files written under `target/dev/` during server startup — build artifact directory is clean
- All existing 18+ integration tests continue to pass
- Requirements HTTP-03 and HTTP-04 marked complete

---
*Phase: 55-platform-http-hardening*
*Completed: 2026-03-23*
