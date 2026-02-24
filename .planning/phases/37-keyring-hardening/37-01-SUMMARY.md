---
phase: 37-keyring-hardening
plan: 01
subsystem: auth
tags: [pbkdf2, keyring, security, crypto, owasp]

# Dependency graph
requires: []
provides:
  - "Keyring backends with OWASP 2023 PBKDF2 parameters: 600k iterations, 32-byte salts"
  - "KeyContext and KeyDerivationContext defaults updated to 600k iterations"
  - "CLI salt validation updated to 32-byte requirement"
affects: [keyring, crypto, backends]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "600_000 PBKDF2 iterations as OWASP 2023 baseline across all keyring code paths"
    - "32-byte salts as minimum for PBKDF2 key derivation in keyring backends"

key-files:
  created: []
  modified:
    - crates/core/src/backends/keyring.rs
    - crates/core/src/backends/universal_keyring.rs
    - crates/core/src/backends/traits.rs
    - crates/core/src/backends/universal.rs
    - crates/core/src/bin/trustedge-client.rs

key-decisions:
  - "PBKDF2 iterations raised from 100k to 600k per OWASP 2023 PBKDF2-HMAC-SHA256 recommendation"
  - "Salt length raised from 16 to 32 bytes for both keyring backends"
  - "derive_key key_id ([u8; 16]) uses first 16 bytes of 32-byte salt as key isolator in CLI"

patterns-established:
  - "OWASP 2023 comment annotation: // OWASP 2023 recommended PBKDF2 iterations on iteration defaults"

requirements-completed: [KEY-01, KEY-02, KEY-03, KEY-04, TST-03]

# Metrics
duration: 17min
completed: 2026-02-24
---

# Phase 37 Plan 01: Keyring Hardening Summary

**PBKDF2 hardening across both keyring backends: 100k->600k iterations and 16->32 byte salts per OWASP 2023 PBKDF2-HMAC-SHA256 recommendation**

## Performance

- **Duration:** 17 min
- **Started:** 2026-02-24T12:09:45Z
- **Completed:** 2026-02-24T12:26:45Z
- **Tasks:** 2
- **Files modified:** 5

## Accomplishments

- Hardened PBKDF2 iteration count from 100,000 to 600,000 in both `keyring.rs` and `universal_keyring.rs` (OWASP 2023 PBKDF2-HMAC-SHA256 recommendation)
- Updated salt validation from 16 to 32 bytes in both backends and the CLI
- Updated `KeyContext::new` and `KeyDerivationContext::new` defaults to 600,000 iterations
- Updated test assertions in both files to match 32-byte salt error messages
- All 162 `cargo test -p trustedge-core --lib` tests pass with zero failures

## Task Commits

Each task was committed atomically:

1. **Task 1: Update PBKDF2 parameters in keyring backends and context structs** - `0c7f0cc` (feat) — includes test updates (salt validation messages and test vectors updated inline)
2. **Task 2: Update keyring tests for 32-byte salts and verify full test suite** — verified: 162/162 tests pass; no additional commit needed (test updates were part of Task 1 edit)

## Files Created/Modified

- `crates/core/src/backends/keyring.rs` - Salt validation 16->32 bytes, iterations default 100k->600k, test renamed and updated
- `crates/core/src/backends/universal_keyring.rs` - Matching salt/iteration updates, test vectors use vec![1; 32]
- `crates/core/src/backends/traits.rs` - KeyContext::new default iterations 100k->600k with OWASP comment
- `crates/core/src/backends/universal.rs` - KeyDerivationContext::new default iterations 100k->600k with OWASP comment
- `crates/core/src/bin/trustedge-client.rs` - Salt arg help text, 32-byte validation, key_id split pattern for derive_key

## Decisions Made

- PBKDF2 iterations raised from 100k to 600k — OWASP 2023 recommendation for PBKDF2-HMAC-SHA256 brute-force resistance
- Salt length raised from 16 to 32 bytes — matches NIST SP 800-132 minimum for PBKDF2
- `derive_key` takes `key_id: &[u8; 16]` (existing signature preserved per plan) — CLI passes first 16 bytes of 32-byte salt as key_id, which provides key isolation while keeping function signature unchanged

## Deviations from Plan

None - plan executed exactly as written.

The test updates for Task 2 (renaming `test_key_derivation_requires_16_byte_salt`, updating salt vectors) were applied inline during Task 1's file edits and included in the Task 1 commit. No separate Task 2 commit was needed.

## Issues Encountered

None.

## Next Phase Readiness

- Keyring PBKDF2 hardening complete — meets OWASP 2023 standards
- No blockers for subsequent plans in Phase 37

---
*Phase: 37-keyring-hardening*
*Completed: 2026-02-24*

## Self-Check: PASSED

- keyring.rs: FOUND, contains 600_000 and 32 bytes validation
- universal_keyring.rs: FOUND, contains 600_000 and 32 bytes validation
- traits.rs: FOUND, contains 600_000
- universal.rs: FOUND, contains 600_000
- trustedge-client.rs: FOUND, contains 32-byte validation
- SUMMARY.md: FOUND
- Commit 0c7f0cc: FOUND
- Test suite: 162 passed, 0 failed
