---
phase: 52-code-hardening
plan: "02"
subsystem: crypto
tags: [envelope, receipts, cli, security, file-permissions, nonce]

# Dependency graph
requires:
  - phase: 52-code-hardening
    provides: Phase context and AUTH-02, KEYF-01, KEYF-02 requirements
provides:
  - beneficiary() and issuer() return Result<VerifyingKey> — no panics on invalid key bytes
  - Nonce construction rejects chunk index >= 2^24 with explicit error in both seal and unseal
  - Generated key files set to 0600 permissions on Unix via PermissionsExt
affects: [52-03, 53-testing, any callers of envelope beneficiary/issuer methods]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Result-returning getters: public API methods that decode bytes return Result, not panic"
    - "let Ok(x) = expr else { return false; } for bool-returning functions with fallible calls"
    - "#[cfg(unix)] blocks for platform-specific file permission enforcement"

key-files:
  created: []
  modified:
    - crates/core/src/envelope.rs
    - crates/core/src/applications/receipts/mod.rs
    - crates/trst-cli/src/main.rs

key-decisions:
  - "beneficiary()/issuer() return Result<VerifyingKey> — callers decide how to handle invalid bytes"
  - "Test assertions use .expect(\"test\") pattern — test code panicking is acceptable per D-12"
  - "MAX_CHUNK_INDEX as a module-level const referenced in both seal/unseal paths"
  - "Permission set on out_key only, not out_pub — public key files are safe to be world-readable"
  - "Non-unix platforms get stderr warning rather than a hard error — cross-platform compatibility"

patterns-established:
  - "Result-returning getters: public API methods that decode bytes return Result, not panic"

requirements-completed: [AUTH-02, KEYF-01, KEYF-02]

# Metrics
duration: 15min
completed: 2026-03-22
---

# Phase 52 Plan 02: Code Hardening — Panic Elimination and Key File Permissions Summary

**Eliminated panicking unwrap/expect from two public envelope methods and guarded nonce construction against chunk index overflow; generated secret key files now get 0600 OS-level permissions on Unix**

## Performance

- **Duration:** ~15 min
- **Started:** 2026-03-22T02:30:00Z
- **Completed:** 2026-03-22T02:45:00Z
- **Tasks:** 2
- **Files modified:** 3

## Accomplishments

- `Envelope::beneficiary()` and `Envelope::issuer()` changed from panicking methods to `Result<VerifyingKey>` — eliminates AUTH-02 finding
- `const MAX_CHUNK_INDEX: u64 = 16_777_215` added; both `create_encrypted_chunk()` and `decrypt_chunk_v2()` return an explicit error if chunk index >= 2^24 — closes KEYF-02 (nonce overflow silent truncation)
- `trst keygen` now calls `std::fs::set_permissions(&args.out_key, Permissions::from_mode(0o600))` after writing the secret key on Unix — closes KEYF-01
- All 174 core library tests pass; 28 acceptance tests pass; clippy clean on both crates

## Task Commits

1. **Task 1: Eliminate panics from envelope methods and guard nonce overflow** - `8b18b91` (fix)
2. **Task 2: Enforce 0600 permissions on generated key files** - `78a0785` (fix)

## Files Created/Modified

- `crates/core/src/envelope.rs` — added MAX_CHUNK_INDEX const, changed beneficiary/issuer return types to Result, added overflow guards in create_encrypted_chunk and decrypt_chunk_v2, updated test assertions with .expect("test")
- `crates/core/src/applications/receipts/mod.rs` — updated assign_receipt caller to use `?`, updated verify_receipt_chain to use `let Ok()` pattern, updated 4 test assertion sites
- `crates/trst-cli/src/main.rs` — added `#[cfg(unix)] use std::os::unix::fs::PermissionsExt`, added permission block with 0o600 and non-unix warning after secret key write

## Decisions Made

- Test assertions use `.expect("test")` — per D-12, test code panicking is acceptable and `.expect()` in tests is the conventional pattern
- `let Ok(x) = expr else { return false; }` used in `verify_receipt_chain` because that function returns `bool` (not `Result`) — cannot use `?`
- Non-unix platforms emit a `eprintln!` warning rather than failing — prevents breaking CI on non-Linux/macOS platforms

## Deviations from Plan

None — plan executed exactly as written.

## Issues Encountered

None.

## Known Stubs

None — no stubs or placeholder values introduced.

## Next Phase Readiness

- AUTH-02, KEYF-01, KEYF-02 requirements are closed
- Phase 53 (error-path tests) can now test the new Result-returning envelope methods and the nonce overflow guard
- Any downstream callers of `beneficiary()` or `issuer()` outside this crate will need updating (WASM, platform) — scan before 53

---
*Phase: 52-code-hardening*
*Completed: 2026-03-22*
