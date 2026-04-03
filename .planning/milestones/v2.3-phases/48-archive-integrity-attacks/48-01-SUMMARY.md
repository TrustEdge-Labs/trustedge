<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


---
phase: 48-archive-integrity-attacks
plan: "01"
subsystem: testing
tags: [archive, blake3, ed25519, security, trst, tampering, integrity]

# Dependency graph
requires: []
provides:
  - "ArchiveError::UnreferencedChunk variant in trustedge-core error.rs"
  - "Unreferenced chunk detection in validate_archive() (SEC-02 gap closed)"
  - "8 security integration tests in security_archive_integrity.rs (SEC-01 through SEC-04)"
affects: [48-02, 49, 50, 51]

# Tech tracking
tech-stack:
  added: []
  patterns: [assert_cmd black-box CLI testing, SEC-NN test naming convention for requirement traceability]

key-files:
  created:
    - crates/trst-cli/tests/security_archive_integrity.rs
  modified:
    - crates/core/src/error.rs
    - crates/core/src/archive.rs
    - crates/trst-cli/src/main.rs

key-decisions:
  - "UnreferencedChunk check placed in validate_archive() not read_archive() — validation logic belongs in validation, not data-loading"
  - "output_continuity_error() extended to emit specific message for UnreferencedChunk so test assertions can verify the exact failure type"
  - "wrap_encrypted_archive helper uses keygen --unencrypted + wrap --unencrypted to avoid passphrase prompts in CI"

patterns-established:
  - "SEC-NN_ test prefix convention: test names prefixed with requirement ID for direct traceability (test_sec01_, test_sec02_, etc.)"
  - "Black-box CLI testing: assert_cmd verifies exit codes (10/11/12) and stderr strings — tests are independent of internal implementation"

requirements-completed: [SEC-01, SEC-02, SEC-03, SEC-04]

# Metrics
duration: 4min
completed: "2026-03-20"
---

# Phase 48 Plan 01: Archive Integrity Attacks Summary

**validate_archive() now rejects unreferenced chunk files; 8 security tests prove .trst archives detect byte mutation, chunk injection, chunk reordering, and post-signing manifest modification**

## Performance

- **Duration:** 4 min
- **Started:** 2026-03-20T22:54:51Z
- **Completed:** 2026-03-20T22:59:00Z
- **Tasks:** 2
- **Files modified:** 4

## Accomplishments

- Closed SEC-02 gap: validate_archive() scans chunks/ directory and rejects any .bin file not in manifest.segments, returning ArchiveError::UnreferencedChunk
- Added ArchiveError::UnreferencedChunk variant with thiserror display, handled in all CLI match arms
- Created security_archive_integrity.rs with 8 tests covering all four threat model vectors (SEC-01 through SEC-04)
- All 28 acceptance tests and 16 integration tests continue to pass without regression

## Task Commits

Each task was committed atomically:

1. **Task 1: Fix validate_archive to detect unreferenced chunk files** - `3d14593` (fix)
2. **Task 2: Create archive integrity attack tests** - `ed93f1a` (feat)

## Files Created/Modified

- `crates/core/src/error.rs` - Added ArchiveError::UnreferencedChunk variant
- `crates/core/src/archive.rs` - Unreferenced chunk detection in validate_archive(); unit test test_unreferenced_chunk_detected
- `crates/trst-cli/src/main.rs` - Added UnreferencedChunk arm in exit-12 match; extended output_continuity_error() for specific unreferenced chunk message
- `crates/trst-cli/tests/security_archive_integrity.rs` - 8 new security tests (SEC-01 x2, SEC-02 x2, SEC-03 x1, SEC-04 x3)

## Decisions Made

- UnreferencedChunk check placed in validate_archive() not read_archive() — data-loading vs. validation separation
- output_continuity_error() extended with explicit "Unreferenced chunk file" branch so test stderr assertions are reliable
- Used wrap_encrypted_archive helper (keygen --unencrypted + wrap --unencrypted) for CI-compatible key handling without passphrase prompts

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] output_continuity_error() not emitting specific unreferenced chunk message**
- **Found during:** Task 2 (security tests) — test_sec02_injected_extra_chunk failed on stderr assertion
- **Issue:** When validate_archive() returned ArchiveError::UnreferencedChunk, output_continuity_error() fell through to generic "Continuity: FAIL" message without the specific error text
- **Fix:** Added explicit check for "Unreferenced chunk file" in the error string, emitting the full error message when detected
- **Files modified:** crates/trst-cli/src/main.rs
- **Verification:** All 8 security tests pass; 28 acceptance tests still pass
- **Committed in:** ed93f1a (Task 2 commit)

---

**Total deviations:** 1 auto-fixed (Rule 1 - bug in output formatting)
**Impact on plan:** Fix necessary for test assertions and user-visible diagnostics. No scope creep.

## Issues Encountered

None beyond the auto-fixed output formatting issue above.

## Next Phase Readiness

- SEC-01 through SEC-04 requirements fully verified with passing tests
- validate_archive() now enforces both hash integrity AND chunk set completeness
- Ready for Phase 49 (nonce uniqueness tests) — independent of this phase

---
*Phase: 48-archive-integrity-attacks*
*Completed: 2026-03-20*
