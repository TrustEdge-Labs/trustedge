---
phase: 26-crypto-deduplication
plan: 01
subsystem: crypto
tags: [trustedge-core, blake3, ed25519, chain, crypto, platform, verification]

# Dependency graph
requires:
  - phase: 25-service-consolidation
    provides: trustedge-platform crate with verify engine and HTTP handlers
provides:
  - trustedge-core as always-on dependency of trustedge-platform
  - engine.rs using trustedge_core::crypto::verify_manifest for Ed25519 verification
  - engine.rs using trustedge_core::chain::{genesis, chain_next} for BLAKE3 chaining
  - handlers.rs using trustedge_core::chain::segment_hash for BLAKE3 digest
  - CA module with all Future: markers (renamed from Phase 26:)
affects: [26-crypto-deduplication, 27-ghost-repo-cleanup]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "trustedge-core owns all cryptographic operations — platform delegates to core's chain and crypto modules"
    - "format_b3() helper in engine.rs encodes [u8; 32] to 'b3:BASE64' using standard base64 alphabet"
    - "Ed25519 signature format bridging: manifest stores raw base64, core expects 'ed25519:BASE64' — prepend prefix at call site"

key-files:
  created: []
  modified:
    - crates/platform/Cargo.toml
    - crates/platform/src/verify/engine.rs
    - crates/platform/src/http/handlers.rs
    - crates/platform/src/ca/api.rs
    - crates/platform/src/ca/auth.rs
    - crates/platform/src/ca/database.rs
    - crates/platform/src/ca/service.rs

key-decisions:
  - "trustedge-core moved to always-on dependency (was optional/ca-gated) — all platform features need crypto primitives"
  - "format_b3() helper uses BASE64 crate's STANDARD encoder (not core's internal base64_encode) for consistent wire format with existing callers"
  - "Ed25519 signature in manifest JSON stored as raw base64 (no prefix); prepend 'ed25519:' before calling trustedge_core::crypto::verify_manifest()"
  - "CA module doc comments referencing 'Phase 26' also renamed to 'Future:' for consistency"

patterns-established:
  - "Crypto delegation pattern: platform calls trustedge_core::chain::segment_hash() and trustedge_core::chain::{genesis, chain_next} — never blake3 directly"
  - "Signature verification delegation: platform calls trustedge_core::crypto::verify_manifest() — never ed25519_dalek directly"

requirements-completed: [CRYPTO-01, CRYPTO-02]

# Metrics
duration: 2min
completed: 2026-02-22
---

# Phase 26 Plan 01: Crypto Deduplication Summary

**trustedge-core promoted to always-on platform dependency, replacing direct blake3 and ed25519-dalek calls with trustedge_core::chain and trustedge_core::crypto delegation; 19 CA Phase 26 markers renamed to Future:**

## Performance

- **Duration:** 2 min
- **Started:** 2026-02-22T01:30:34Z
- **Completed:** 2026-02-22T01:34:10Z
- **Tasks:** 2
- **Files modified:** 7

## Accomplishments
- trustedge-core promoted from optional (ca-gated) to always-on dependency in trustedge-platform
- engine.rs verify_signature() now calls trustedge_core::crypto::verify_manifest() — no ed25519_dalek
- engine.rs chain functions now use trustedge_core::chain::{genesis, chain_next} — no blake3::Hasher
- handlers.rs compute_manifest_digest_blake3() now uses trustedge_core::chain::segment_hash()
- All 19 "// Phase 26:" markers in CA module renamed to "// Future:"
- All 12 unit tests and 7 integration tests pass, clippy clean

## Task Commits

Each task was committed atomically:

1. **Task 1: Replace engine.rs crypto with trustedge-core and update Cargo.toml** - `2ef9fa1` (feat)
2. **Task 2: Replace handlers.rs blake3 digest and rename CA Phase 26 markers** - `b8ac29e` (feat)

**Plan metadata:** (docs commit follows)

## Files Created/Modified
- `crates/platform/Cargo.toml` - Added trustedge-core as always-on dep; removed from ca feature list
- `crates/platform/src/verify/engine.rs` - Removed blake3::Hasher and ed25519_dalek imports; added trustedge_core delegation
- `crates/platform/src/http/handlers.rs` - Replaced blake3::hash() with trustedge_core::chain::segment_hash()
- `crates/platform/src/ca/api.rs` - Renamed 6 Phase 26: markers to Future:
- `crates/platform/src/ca/auth.rs` - Renamed 3 Phase 26: markers to Future:
- `crates/platform/src/ca/database.rs` - Renamed 5 Phase 26: markers to Future:
- `crates/platform/src/ca/service.rs` - Renamed 6 Phase 26: markers to Future:

## Decisions Made
- trustedge-core moved to always-on: verify engine needs core's crypto even without the `ca` feature — no reason to keep it optional
- format_b3() helper in engine.rs uses `base64` crate's STANDARD encoder (not core's private `base64_encode`) to match existing wire format that callers decode with the same encoder
- Ed25519 bridge: manifest JSON stores raw base64 signature (no prefix), core's verify_manifest expects "ed25519:BASE64" — prepend prefix at call site rather than changing wire format

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - Missing Critical] Updated CA module doc comments referencing Phase 26**
- **Found during:** Task 2 (CA marker rename)
- **Issue:** auth.rs and database.rs module doc comments (`//!`) referenced "Phase 26 implements..." — not covered by the `// Phase 26:` comment pattern but still incorrect references
- **Fix:** Updated two module doc comments to say "Future: implements..." for consistency
- **Files modified:** crates/platform/src/ca/auth.rs, crates/platform/src/ca/database.rs
- **Verification:** grep -rn "Phase 26" crates/platform/src/ returns no results
- **Committed in:** b8ac29e (Task 2 commit)

---

**Total deviations:** 1 auto-fixed (missing critical — doc comment cleanup)
**Impact on plan:** Minor — doc comments only, no functional changes. Complete consistency in Phase 26 reference removal.

## Issues Encountered
None - plan executed cleanly with no blocking issues.

## Next Phase Readiness
- Phase 26 Plan 01 complete: crypto deduplication foundation established
- engine.rs and handlers.rs are now fully decoupled from direct blake3/ed25519_dalek
- CA module cleanup complete — all Phase 26 time-specific markers gone
- Ready for Phase 26 Plan 02 (if any) or Phase 27

---
*Phase: 26-crypto-deduplication*
*Completed: 2026-02-22*

## Self-Check: PASSED

- FOUND: crates/platform/src/verify/engine.rs
- FOUND: crates/platform/src/http/handlers.rs
- FOUND: .planning/phases/26-crypto-deduplication/26-01-SUMMARY.md
- FOUND: commit 2ef9fa1 (Task 1)
- FOUND: commit b8ac29e (Task 2)
- FOUND: commit ec1f61f (Plan metadata)
