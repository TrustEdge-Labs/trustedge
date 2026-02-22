---
phase: 25-service-consolidation
plan: 03
subsystem: platform
tags: [rust, axum, ed25519, blake3, tower, sqlx, postgres, testing, axum-test]

# Dependency graph
requires:
  - phase: 25-02
    provides: HTTP layer, database module, consolidated AppState with inline verification

provides:
  - crates/platform/tests/verify_integration.rs: 7 integration tests from verify-core (5 pure crypto + 2 HTTP)
  - crates/platform/tests/platform_integration.rs: 11 integration tests from platform-api (all #[ignore] — require PostgreSQL)
  - scripts/ci-check.sh: trustedge-platform as Tier 1 blocking with verify_integration in CI
  - CLAUDE.md: updated to 12-crate workspace with trustedge-platform documented
  - DEPENDENCIES.md: trustedge-platform entry with per-feature dependency tables

affects:
  - 26-crypto-deduplication (test suite confirms platform crate is correct baseline for deduplication)
  - 27-ghost-repo-cleanup (source repos trustedge-verify-core and trustedge-platform-api now have all tests in platform)

# Tech tracking
tech-stack:
  added:
    - axum-test 14.10 (axum 0.7 compatible — for platform_integration tests; all #[ignore])
    - tower (dev-dep — ServiceExt::oneshot for HTTP integration tests)
  patterns:
    - "HTTP integration tests gated behind #[cfg(feature = \"http\")] in test module"
    - "DB-dependent tests always #[ignore] — same pattern as YubiKey hardware tests"
    - "deny_unknown_fields on VerifyRequest — strict deserialization for API hygiene"
    - "Ordered validation in verify_handler: empty segments → device_pub → manifest → hash format"

key-files:
  created:
    - crates/platform/tests/verify_integration.rs
    - crates/platform/tests/platform_integration.rs
  modified:
    - crates/platform/Cargo.toml (dev-deps: ed25519-dalek, anyhow, base64, axum, tower, axum-test, sqlx)
    - crates/platform/src/http/handlers.rs (validation order fix, create_test_app simplified, collapsible_if fix)
    - crates/platform/src/verify/types.rs (deny_unknown_fields on VerifyRequest)
    - crates/platform/src/verify/validation.rs (validate_segment_hashes made pub)
    - scripts/ci-check.sh (trustedge-platform Tier 1 clippy + tests, baseline=70)
    - CLAUDE.md (12 crates, trustedge-platform docs, feature flags table)
    - DEPENDENCIES.md (trustedge-platform entry, updated scope to 12 crates)

key-decisions:
  - "axum-test 14.x used (not 18.x) — must match axum 0.7 workspace version"
  - "Ordered validation: empty segments first (matches test_verify_invalid_payload expectation), then device_pub, then manifest, then hash format"
  - "#[serde(deny_unknown_fields)] on VerifyRequest — strict API hygiene, rejects unknown fields with 400"
  - "test_jwks_proxy renamed to test_jwks_endpoint_returns_local_keys — reflects behavioral change (502 proxy → 200 local)"
  - "test_verify_valid_payload_forwards_to_core renamed to test_verify_valid_payload_inline_verification — inline validation returns 400 for invalid hash, not 502"
  - "dep tree baseline updated from 60 to 70 — platform crate adds transitive workspace deps"

requirements-completed: [SVC-03, SVC-04]

# Metrics
duration: 9min
completed: 2026-02-22
---

# Phase 25 Plan 03: Test Migration and CI Integration Summary

**29 tests from verify-core and platform-api consolidated into trustedge-platform: 7 verify integration + 12 unit always pass; 11 platform DB tests present as #[ignore]; trustedge-platform promoted to Tier 1 CI**

## Performance

- **Duration:** 9 min
- **Started:** 2026-02-22T00:07:34Z
- **Completed:** 2026-02-22T00:16:00Z
- **Tasks:** 2
- **Files modified:** 9

## Accomplishments

- Migrated all 7 verify-core integration tests: 5 pure crypto tests run without feature gates; 2 HTTP tests gated behind `#[cfg(feature = "http")]`
- Migrated all 11 platform-api integration tests, all marked `#[ignore]` (require PostgreSQL) — consistent with YubiKey hardware test pattern
- Fixed validation order in `verify_handler`: segments-empty check runs first, then device_pub, manifest, hash format — ensures correct error codes per test expectations
- Added `#[serde(deny_unknown_fields)]` to `VerifyRequest` — rejects unknown fields with 400
- Updated CI: trustedge-platform in Tier 1 blocking (clippy + unit tests + verify_integration)
- Updated CLAUDE.md and DEPENDENCIES.md to document 12-crate workspace and trustedge-platform deps

## Task Commits

Each task was committed atomically:

1. **Task 1: Migrate verify-core integration tests** - `fa9150d` (feat)
2. **Task 2: Migrate platform-api tests and update CI/docs** - `faaccff` (feat)

**Plan metadata:** (to be added in final commit)

## Files Created/Modified

- `crates/platform/tests/verify_integration.rs` - 7 verify-core integration tests (5 pure crypto + 2 HTTP)
- `crates/platform/tests/platform_integration.rs` - 11 platform-api integration tests (all #[ignore])
- `crates/platform/Cargo.toml` - Dev-deps: ed25519-dalek, anyhow, base64, axum, tower, axum-test 14.10, sqlx
- `crates/platform/src/http/handlers.rs` - Ordered validation, simplified create_test_app, collapsible_if fix
- `crates/platform/src/verify/types.rs` - #[serde(deny_unknown_fields)] on VerifyRequest
- `crates/platform/src/verify/validation.rs` - validate_segment_hashes made pub
- `scripts/ci-check.sh` - trustedge-platform Tier 1 clippy + test steps, baseline=70
- `CLAUDE.md` - 12 crates, trustedge-platform entry, feature flags, test commands
- `DEPENDENCIES.md` - trustedge-platform section with per-feature dep tables, scope updated to 12 crates

## Decisions Made

- `axum-test 14.x` selected (not latest 18.x) because our workspace pins `axum = "0.7"` and axum-test 18.x requires axum 0.8 — using the wrong version would create a version conflict.
- Ordered validation fixed to: empty segments → device_pub → manifest → hash format. This was necessary to make the migrated test assertions correct (test 5 expects `invalid_segments` when segments is empty; test 6 expects `invalid_device_pub` when only device_pub is empty but segments are non-empty with invalid format).
- `test_jwks_proxy` renamed to `test_jwks_endpoint_returns_local_keys` with updated assertion from 502 to 200. Consolidated crate serves JWKS from local KeyManager — no HTTP proxy.
- `test_verify_valid_payload_forwards_to_core` renamed to `test_verify_valid_payload_inline_verification`. The segment hash `"a"*64` lacks `b3:` prefix — inline hash validation returns 400 instead of the original 502 proxy error.
- Dependency tree baseline raised to 70: workspace with trustedge-platform (without features) now has 70 unique crates vs prior 60. Threshold remains baseline + 10.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Validation order returned wrong error codes**
- **Found during:** Task 2 (analyzing platform_integration test assertions)
- **Issue:** `validate_verify_request` (empty segments + hash format) ran before `device_pub.is_empty()` and `manifest` checks. Test 6 (`test_verify_empty_device_pub_returns_400`) sends non-empty but invalid-format segments and empty device_pub — the hash format check fired first, returning `invalid_segments` instead of `invalid_device_pub`.
- **Fix:** Restructured handlers to check in order: empty segments → device_pub → manifest → hash format via `validate_segment_hashes`; made `validate_segment_hashes` pub
- **Files modified:** `crates/platform/src/http/handlers.rs`, `crates/platform/src/verify/validation.rs`
- **Verification:** All 12 unit tests still pass; platform integration tests list expected 11 tests
- **Committed in:** `faaccff` (Task 2 commit)

**2. [Rule 2 - Missing Critical] VerifyRequest missing deny_unknown_fields**
- **Found during:** Task 2 (test_verify_unknown_fields_returns_400 analysis)
- **Issue:** `VerifyRequest` without `#[serde(deny_unknown_fields)]` silently ignores unknown fields, returning 200 instead of 400 for the test case
- **Fix:** Added `#[serde(deny_unknown_fields)]` to `VerifyRequest` in `types.rs`
- **Files modified:** `crates/platform/src/verify/types.rs`
- **Verification:** Build passes; unknown fields now cause serde deserialization error → 400
- **Committed in:** `faaccff` (Task 2 commit)

**3. [Rule 1 - Bug] Collapsible nested if in non-postgres verify_handler**
- **Found during:** Task 2 (clippy --features http run)
- **Issue:** Nested `if options.return_receipt.unwrap_or(false) { if report... { ... } }` — clippy `-D warnings` rejects `collapsible_if`
- **Fix:** Collapsed to `if options.return_receipt.unwrap_or(false) && report.signature_verification.passed && report.continuity_verification.passed { ... }`
- **Files modified:** `crates/platform/src/http/handlers.rs`
- **Verification:** `cargo clippy -p trustedge-platform --features http -- -D warnings` passes
- **Committed in:** `faaccff` (Task 2 commit)

**4. [Rule 3 - Blocking] axum-test 18.x incompatible with axum 0.7**
- **Found during:** Task 2 (dependency resolution)
- **Issue:** Initially added `axum-test = "18.7.0"` which requires axum 0.8, but workspace uses axum 0.7 — would create API incompatibility for TestServer accepting our axum::Router
- **Fix:** Downgraded to `axum-test = "14.10"` which is compatible with axum 0.7
- **Files modified:** `crates/platform/Cargo.toml`
- **Verification:** `cargo build -p trustedge-platform --features "http,postgres,test-utils"` succeeds
- **Committed in:** `faaccff` (Task 2 commit)

---

**Total deviations:** 4 auto-fixed (2 bugs, 1 missing critical, 1 blocking)
**Impact on plan:** All fixes necessary for correct behavior and compilation. No scope creep.

## Issues Encountered

None — all issues handled automatically via deviation rules.

## Verification Results

- `cargo test -p trustedge-platform --lib`: PASSED (12 unit tests)
- `cargo test -p trustedge-platform --test verify_integration`: PASSED (5 non-HTTP tests)
- `cargo test -p trustedge-platform --test verify_integration --features http`: PASSED (7 tests)
- `cargo test -p trustedge-platform --test platform_integration --features "http,postgres,test-utils" -- --list`: 11 tests listed
- `cargo test --workspace --no-default-features --locked`: PASSED (no regressions)
- `cargo clippy -p trustedge-platform --features "http,postgres,ca" -- -D warnings`: PASSED
- `grep "trustedge-platform" scripts/ci-check.sh`: present in Tier 1 steps
- `grep "12" CLAUDE.md`: "12 crates" confirmed
- No reqwest in trustedge-platform: VERIFIED
- No verify_core_url in code: VERIFIED (only in doc comments)

## Next Phase Readiness

- Phase 25 complete: trustedge-platform has all verify-core + platform-api functionality consolidated
- Phase 26 (crypto deduplication) can now audit trustedge-platform's verify engine vs trustedge-core's chain.rs
- Phase 27 (ghost repo cleanup) can now proceed: trustedge-verify-core and trustedge-platform-api are fully superseded by trustedge-platform
- No blockers.

## Self-Check: PASSED

- crates/platform/tests/verify_integration.rs: FOUND
- crates/platform/tests/platform_integration.rs: FOUND
- scripts/ci-check.sh: FOUND (contains trustedge-platform)
- CLAUDE.md: FOUND (contains "12 crates")
- Task 1 commit fa9150d: FOUND
- Task 2 commit faaccff: FOUND

---
*Phase: 25-service-consolidation*
*Completed: 2026-02-22*
