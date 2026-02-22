---
phase: 33-platform-quality
plan: 01
subsystem: trustedge-platform
tags: [refactor, deduplication, validation, handlers]
dependency_graph:
  requires: []
  provides: [validate_verify_request_full, build_receipt_if_requested]
  affects: [crates/platform/src/verify/validation.rs, crates/platform/src/http/handlers.rs]
tech_stack:
  added: []
  patterns: [shared-validation-function, feature-gated-imports, manifest-digest-fn-parameter]
key_files:
  created: []
  modified:
    - crates/platform/src/verify/validation.rs
    - crates/platform/src/http/handlers.rs
decisions:
  - "validate_verify_request_full: 4 checks in order (segments, device_pub, manifest, hash format), first-error-wins"
  - "build_receipt_if_requested: manifest_digest_fn parameter avoids feature-flag coupling in validation.rs"
  - "postgres handler: receipt construction remains inline due to DB storage interleaving (by design)"
  - "Feature-gated imports in handlers.rs to prevent unused-import warnings under each feature combination"
metrics:
  duration: 4 minutes
  completed: 2026-02-22
  tasks_completed: 2
  files_modified: 2
---

# Phase 33 Plan 01: Verify Handler Deduplication Summary

**One-liner:** Extracted shared `validate_verify_request_full` and `build_receipt_if_requested` functions from duplicated inline handler code, eliminating ~50 lines of copy-pasted validation logic.

## What Was Built

### `validate_verify_request_full` (validation.rs)

A new public function covering all four ordered validation checks:
1. Empty segments (`invalid_segments`)
2. Empty `device_pub` (`invalid_device_pub`)
3. Null/empty manifest (`invalid_manifest`)
4. Hash format via `validate_segment_hashes` (`invalid_segments`)

First-error-wins semantics preserved from the original inline code.

### `build_receipt_if_requested` (validation.rs)

An async public function that encapsulates receipt construction for the non-postgres handler:
- Returns `Ok(None)` when receipt conditions are not met
- Returns `Ok(Some(jws))` when receipt is built and signed
- Returns `Err(ValidationError)` on signing failure
- Takes a `manifest_digest_fn: impl Fn(&serde_json::Value) -> String` parameter, allowing callers to supply the appropriate digest function without `validation.rs` needing feature-flag awareness

### Refactored `handlers.rs`

Both `verify_handler` variants now open with a single line:
```rust
validate_verify_request_full(&request).map_err(|e| (StatusCode::BAD_REQUEST, Json(e)))?;
```

The non-postgres handler also uses `build_receipt_if_requested`. The postgres handler keeps receipt construction inline (due to DB storage interleaving) but benefits from the shared validation.

## Tests Added

Five new unit tests in `validation.rs`:
- `test_full_validate_empty_segments_returns_invalid_segments`
- `test_full_validate_empty_device_pub_returns_invalid_device_pub`
- `test_full_validate_null_manifest_returns_invalid_manifest`
- `test_full_validate_empty_object_manifest_returns_invalid_manifest`
- `test_full_validate_valid_request_passes`
- `test_full_validate_first_check_wins_segments_before_device_pub`

## Verification Results

| Check | Result |
|-------|--------|
| `cargo test --lib` (18 tests) | PASS |
| `cargo test --test verify_integration --features http` (7 tests) | PASS |
| `cargo build --features http` | PASS |
| `cargo build --features "http,postgres"` | PASS |
| `cargo clippy --features "http,postgres" -- -D warnings` | PASS (0 warnings) |
| `cargo clippy --features http -- -D warnings` | PASS (0 warnings) |
| Grep: validation strings absent from handlers.rs | PASS |

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Clippy explicit-auto-deref warning**
- **Found during:** Task 2 verification
- **Issue:** `&*keys` triggered `clippy::explicit-auto-deref` under `--features http`
- **Fix:** Changed to `&keys` (auto-deref handles the RwLockReadGuard -> KeyManager coercion)
- **Files modified:** `crates/platform/src/http/handlers.rs`
- **Commit:** 1cf8e34

**2. [Rule 2 - Missing] Feature-gated imports for unused-import prevention**
- **Found during:** Task 2 verification
- **Issue:** `receipt_from_report`/`sign_receipt_jws` unused when `postgres` is not enabled; `build_receipt_if_requested` unused when `postgres` is enabled
- **Fix:** Split imports behind `#[cfg(not(feature = "postgres"))]` and `#[cfg(feature = "postgres")]` respectively
- **Files modified:** `crates/platform/src/http/handlers.rs`
- **Commit:** 1cf8e34

**3. [Rule 1 - Formatting] cargo fmt applied**
- **Found during:** Task 2 commit (pre-commit hook)
- **Issue:** `build_receipt_if_requested` call formatted differently from rustfmt's preferred style
- **Fix:** `cargo fmt --all` (also touched `ca/api.rs` with unrelated pre-existing fmt differences)
- **Commit:** 1cf8e34

## Commits

| Task | Commit | Description |
|------|--------|-------------|
| Task 1 | 68dda47 | feat(33-01): add validate_verify_request_full and build_receipt_if_requested |
| Task 2 | 1cf8e34 | refactor(33-01): deduplicate verify handler validation using shared functions |
