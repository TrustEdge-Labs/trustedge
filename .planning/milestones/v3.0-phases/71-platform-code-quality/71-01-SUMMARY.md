<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


---
phase: 71-platform-code-quality
plan: "01"
subsystem: platform
tags: [security, configuration, http]
dependency_graph:
  requires: []
  provides: [receipt-ttl-configurable, healthz-version-removed, port-fail-fast]
  affects: [trustedge-platform, trustedge-platform-server]
tech_stack:
  added: []
  patterns: [env-var-config, fail-fast-parsing]
key_files:
  created: []
  modified:
    - crates/platform/src/http/config.rs
    - crates/platform/src/http/state.rs
    - crates/platform/src/verify/signing.rs
    - crates/platform/src/verify/validation.rs
    - crates/platform/src/http/handlers.rs
    - crates/platform-server/src/main.rs
    - crates/platform/tests/verify_integration.rs
decisions:
  - "receipt_ttl_secs defaults to 3600 and is threaded through Config -> AppState -> sign_receipt_jws"
  - "PORT parsing now fails fast with clear error message when set to invalid value"
  - "healthz returns only status and timestamp; no version field to prevent fingerprinting"
metrics:
  duration_minutes: 12
  completed_date: "2026-03-26"
  tasks_completed: 2
  files_modified: 7
---

# Phase 71 Plan 01: Platform Code Quality Summary

**One-liner:** Configurable JWS receipt TTL via RECEIPT_TTL_SECS, version-free /healthz, and fail-fast PORT parsing to address three P2 security findings.

## Tasks Completed

### Task 1: Add configurable receipt TTL (PLAT-01)
- Added `receipt_ttl_secs: u64` to `Config` struct, parsed from `RECEIPT_TTL_SECS` env var (default 3600)
- Added `receipt_ttl_secs: u64` to `AppState` struct
- Updated `sign_receipt_jws` to accept `ttl_secs: u64` parameter replacing hardcoded `3600`
- Threaded `receipt_ttl_secs` through `build_receipt_if_requested` in `validation.rs`
- Updated both verify_handler call sites in `handlers.rs` to pass `state.receipt_ttl_secs`
- Wired `receipt_ttl_secs` from config into AppState in `platform-server/main.rs`
- Updated `create_test_app` (postgres test utility) with `receipt_ttl_secs: 3600`
- Updated integration test's `make_state()` with `receipt_ttl_secs: 3600`
- Added `test_receipt_ttl_default` and `test_port_error_message_exists` unit tests

**Commit:** fab739e

### Task 2: Remove healthz version field and fail on invalid PORT (PLAT-02, PLAT-03)
- Removed `version: String` field from `HealthResponse` struct in `types.rs`
- Removed `env!("CARGO_PKG_VERSION")` from `health_handler()` in `handlers.rs`
- Updated `verify_integration.rs` test to assert `body_json.get("version").is_none()`
- PORT parsing in `config.rs` now returns clear error `"PORT env var '...' is not a valid port number"` instead of silently defaulting to 3001

**Commit:** 0674c1b

## Verification

All verification suite commands pass:
- `cargo test -p trustedge-platform --lib` — 18 tests passed
- `cargo test -p trustedge-platform --test verify_integration` — 9 tests passed
- `cargo clippy -p trustedge-platform -- -D warnings` — zero warnings
- `cargo clippy -p trustedge-platform-server -- -D warnings` — zero warnings

## Deviations from Plan

None — plan executed exactly as written. PORT parsing (PLAT-03) was implemented in Task 1 as part of the config.rs changes, alongside receipt_ttl_secs, since both touch the same `from_env()` function. This is a natural grouping, not a deviation.

## Known Stubs

None.

## Self-Check: PASSED

Files verified:
- crates/platform/src/http/config.rs — contains `receipt_ttl_secs`, `RECEIPT_TTL_SECS`, `is not a valid port number`
- crates/platform/src/http/state.rs — contains `receipt_ttl_secs`
- crates/platform/src/verify/signing.rs — contains `ttl_secs: u64`, `now + ttl_secs as i64`
- crates/platform/src/verify/validation.rs — contains `receipt_ttl_secs: u64` parameter
- crates/platform/src/verify/types.rs — HealthResponse has only `status` and `timestamp`
- crates/platform/src/http/handlers.rs — health_handler has no `version:`, no `CARGO_PKG_VERSION`
- crates/platform/tests/verify_integration.rs — asserts `body_json.get("version").is_none()`

Commits verified:
- fab739e — feat(71-01): add configurable receipt TTL via RECEIPT_TTL_SECS env var
- 0674c1b — feat(71-01): remove version from healthz and fail clearly on invalid PORT
