---
phase: 49-nonce-and-key-derivation
plan: "01"
subsystem: testing
tags: [security, nonce, hkdf, xchacha20, blake3, trst-cli]

# Dependency graph
requires:
  - phase: 48-archive-integrity-attacks
    provides: security test patterns (assert_cmd, tempfile, SEC-NN_ naming, wrap_unencrypted_archive helper)
provides:
  - 6 passing security tests proving nonce uniqueness within/across archives and HKDF key-binding
  - SEC-05, SEC-06, SEC-07 requirement coverage with named test functions
affects: [50-key-file-format, 51-receipt-binding]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "SEC-NN_ prefix naming convention for requirement-traceable test functions"
    - "read_nonce(path) helper extracts 24-byte nonce prefix from chunk files"
    - "collect_chunk_paths() helper returns sorted chunk paths from archive"
    - "HashSet<[u8; 24]> deduplication for nonce uniqueness proofs"

key-files:
  created:
    - crates/trst-cli/tests/security_nonce_key_derivation.rs
  modified: []

key-decisions:
  - "6 tests cover 3 requirements (not 7 as originally estimated in done criteria — plan task body lists 6 distinct test names)"
  - "wrap_with_key() helper excluded as unused — duplicate test for SEC-06 uses inline Command::cargo_bin calls to keep each archive wrap explicit and self-documenting"

patterns-established:
  - "Security test files follow assert_cmd + tempfile black-box pattern, copy helpers locally rather than sharing across test crates"
  - "Unit-level HKDF tests use derive_chunk_key() directly via trustedge_core re-export"
  - "Nonce extraction pattern: read first 24 bytes of chunk file (on-disk format is [nonce:24][ciphertext:N])"

requirements-completed: [SEC-05, SEC-06, SEC-07]

# Metrics
duration: 35min
completed: 2026-03-20
---

# Phase 49 Plan 01: Nonce and Key Derivation Security Tests Summary

**Six passing tests prove XChaCha20 nonce uniqueness within/across .trst archives and HKDF-SHA256 key-binding via direct trustedge_core::derive_chunk_key() calls**

## Performance

- **Duration:** 35 min
- **Started:** 2026-03-20T23:14:04Z
- **Completed:** 2026-03-20T23:48:44Z
- **Tasks:** 2
- **Files modified:** 1

## Accomplishments

- Created `security_nonce_key_derivation.rs` with 6 security tests covering SEC-05, SEC-06, SEC-07
- SEC-05: HashSet dedup proves all 16 chunk nonces within a single archive are unique; zero-nonce sanity check guards against uninitialized nonce bugs
- SEC-06: Two wraps of identical plaintext with identical device key produce different chunk-00000 nonces (random nonce generation proof)
- SEC-07: Three unit tests call `derive_chunk_key()` directly — different inputs yield different keys, same input yields same key (deterministic), output is 32 non-degenerate bytes
- Zero regressions: all 28 trst-cli tests (acceptance + security_archive_integrity + security_nonce_key_derivation) pass

## Task Commits

Each task was committed atomically:

1. **Task 1: Create nonce uniqueness and HKDF key derivation security tests** - `ee004dc` (test)
2. **Task 2: Verify no regressions across workspace** - no commit needed (no files modified)

**Plan metadata:** (this commit)

## Files Created/Modified

- `/home/john/vault/projects/github.com/trustedge/crates/trst-cli/tests/security_nonce_key_derivation.rs` - 6 security tests for SEC-05 (nonce uniqueness within archive), SEC-06 (nonce uniqueness across archives), SEC-07 (HKDF key binding)

## Decisions Made

- Used 6 tests rather than 7 (the plan task body enumerates 6 distinct test function names; the done criteria mentioning "7" was an off-by-one in the plan)
- Removed `wrap_with_key()` helper after noting it was dead code (Rule 3 proactive cleanup); SEC-06 test uses inline `Command::cargo_bin` calls which are clearer for a 2-archive comparison
- HKDF full-entropy test uses `[0x42u8; 32]` as canonical test vector (unambiguous, not confused with edge-case inputs)

## Deviations from Plan

None - plan executed exactly as written. One dead-code warning for an unused helper was discovered and removed inline (minor cleanup, not a deviation).

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- SEC-05, SEC-06, SEC-07 are fully verified; Phase 49 test coverage is complete
- Phase 50 (key-file format testing) can proceed; it uses the same assert_cmd + tempfile pattern
- No blockers

---
*Phase: 49-nonce-and-key-derivation*
*Completed: 2026-03-20*
