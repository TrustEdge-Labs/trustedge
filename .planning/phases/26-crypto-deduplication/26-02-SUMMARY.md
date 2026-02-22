---
phase: 26-crypto-deduplication
plan: 02
subsystem: crypto
tags: [rust, ed25519-dalek, blake3, trustedge-core, trustedge-platform, cargo, re-export]

# Dependency graph
requires:
  - phase: 26-01
    provides: engine.rs and handlers.rs delegating all crypto to trustedge-core; CA Future: markers; trustedge-core as always-on dep

provides:
  - ed25519_dalek::{SigningKey, VerifyingKey} re-exported from trustedge-core for downstream crates
  - trustedge-platform Cargo.toml with blake3 and ed25519-dalek removed from production [dependencies]
  - jwks.rs imports SigningKey/VerifyingKey through trustedge-core re-exports

affects:
  - 27-ghost-repo-cleanup
  - future phases using trustedge-platform

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Crypto re-export pattern: trustedge-core re-exports ed25519_dalek types for downstream crates to use without direct dependency"
    - "Single crypto source: trustedge-platform has exactly one crypto dependency (trustedge-core); no parallel crypto dep trees"

key-files:
  created: []
  modified:
    - crates/core/src/lib.rs
    - crates/platform/src/verify/jwks.rs
    - crates/platform/Cargo.toml
    - crates/platform/src/http/handlers.rs
    - DEPENDENCIES.md

key-decisions:
  - "trustedge-core re-exports ed25519_dalek::{SigningKey, VerifyingKey} for JWKS key management — minimal surface, no new transitive deps"
  - "ed25519-dalek retained in trustedge-platform [dev-dependencies] for test fixture creation (integration tests generate signing keys directly)"

patterns-established:
  - "Crypto consolidation pattern: downstream crates use trustedge_core::{SigningKey, VerifyingKey} not ed25519_dalek::{SigningKey, VerifyingKey}"

requirements-completed: [CRYPTO-01, CRYPTO-02]

# Metrics
duration: 19min
completed: 2026-02-22
---

# Phase 26 Plan 02: Crypto Deduplication — Platform Dep Removal Summary

**trustedge-core re-exports SigningKey/VerifyingKey; blake3 and ed25519-dalek removed from trustedge-platform production deps, leaving trustedge-core as sole crypto dependency**

## Performance

- **Duration:** 19 min
- **Started:** 2026-02-22T01:36:49Z
- **Completed:** 2026-02-22T01:56:08Z
- **Tasks:** 2
- **Files modified:** 5

## Accomplishments

- Added `pub use ed25519_dalek::{SigningKey, VerifyingKey}` re-exports to `trustedge-core/src/lib.rs`
- Updated `jwks.rs` to import `SigningKey`/`VerifyingKey` via `trustedge_core::` instead of `ed25519_dalek::` directly
- Removed `blake3` and `ed25519-dalek` from `trustedge-platform`'s `[dependencies]` section
- All 265+ workspace tests pass; `cargo clippy --workspace -- -D warnings` is clean
- Updated DEPENDENCIES.md to document the crypto deduplication and single-source crypto pattern

## Task Commits

Each task was committed atomically:

1. **Task 1: Add ed25519-dalek re-exports to trustedge-core and update jwks.rs** - `b68832c` (feat)
2. **Task 2: Remove blake3/ed25519-dalek from production deps and run full validation** - `3fe7871` (feat)

**Plan metadata:** (final docs commit)

## Files Created/Modified

- `crates/core/src/lib.rs` - Added `pub use ed25519_dalek::{SigningKey, VerifyingKey}` re-exports
- `crates/platform/src/verify/jwks.rs` - Changed import from `ed25519_dalek` to `trustedge_core`
- `crates/platform/Cargo.toml` - Removed `blake3` and `ed25519-dalek` from `[dependencies]`; `ed25519-dalek` remains in `[dev-dependencies]`
- `crates/platform/src/http/handlers.rs` - Fixed needless borrow in `compute_manifest_digest_blake3` (auto-fix)
- `DEPENDENCIES.md` - Updated trustedge-platform section: removed stale blake3/ed25519-dalek entries, added trustedge-core as sole crypto dep, added crypto deduplication note

## Decisions Made

- `trustedge-core` re-exports `ed25519_dalek::{SigningKey, VerifyingKey}` — minimal addition since core already depends on ed25519-dalek; adds zero new transitive dependencies
- `ed25519-dalek` retained in `[dev-dependencies]` — integration tests in `verify_integration.rs` create signing keys directly for test fixture generation

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Fixed needless borrow clippy warning in handlers.rs**
- **Found during:** Task 2 (full validation — running ci-check.sh)
- **Issue:** `BASE64.encode(&hash)` triggers `clippy::needless-borrows-for-generic-args` warning when building with `--features "http,postgres,ca"`. The `encode()` method accepts anything implementing `AsRef<[u8]>`, so `&hash` is unnecessary when `hash: [u8; 32]` already implements it.
- **Fix:** Changed `BASE64.encode(&hash)` to `BASE64.encode(hash)` in `compute_manifest_digest_blake3`
- **Files modified:** `crates/platform/src/http/handlers.rs`
- **Verification:** `cargo clippy -p trustedge-platform --features "http,postgres,ca" -- -D warnings` passes clean
- **Committed in:** `3fe7871` (Task 2 commit)

---

**Total deviations:** 1 auto-fixed (Rule 1 - bug fix)
**Impact on plan:** The clippy fix was a pre-existing warning exposed when building with all features. Required fix for CI compliance. No scope creep.

## Issues Encountered

- `cargo fmt` pre-commit hook reordered the `use trustedge_core` import alphabetically (after `use std`) — expected behavior; re-staged formatted files and committed.
- The remaining `ci-check.sh` failures (`cargo audit`, `cargo-hack feature powerset`, `downstream feature check`) are all pre-existing environment issues: ALSA audio library not available on this machine (hardware limitation), and pre-existing sqlx/pubky advisory warnings. None caused by this plan's changes.

## Next Phase Readiness

- Phase 26 crypto deduplication is complete: trustedge-platform has one crypto dependency (trustedge-core)
- Phase 27 (Ghost Repo Cleanup) requires GitHub access to archive 6 repos — independent of crypto work
- The `SigningKey`/`VerifyingKey` re-export pattern is established for future downstream crates

## Self-Check: PASSED

- FOUND: `.planning/phases/26-crypto-deduplication/26-02-SUMMARY.md`
- FOUND: commit `b68832c` (Task 1)
- FOUND: commit `3fe7871` (Task 2)
- PASS: no direct `ed25519_dalek` or `blake3` imports in `crates/platform/src/`

---
*Phase: 26-crypto-deduplication*
*Completed: 2026-02-22*
