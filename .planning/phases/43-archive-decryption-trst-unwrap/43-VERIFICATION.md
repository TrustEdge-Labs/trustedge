---
phase: 43-archive-decryption-trst-unwrap
verified: 2026-03-17T00:00:00Z
status: passed
score: 5/5 must-haves verified
re_verification: false
---

# Phase 43: Archive Decryption (trst unwrap) Verification Report

**Phase Goal:** Users can recover original data from a .trst archive, completing the wrap/unwrap data lifecycle
**Verified:** 2026-03-17
**Status:** passed
**Re-verification:** No — initial verification

## Goal Achievement

### Observable Truths

| #  | Truth                                                                                                             | Status     | Evidence                                                                                           |
|----|-------------------------------------------------------------------------------------------------------------------|------------|----------------------------------------------------------------------------------------------------|
| 1  | User can run `trst unwrap <archive.trst> --device-key <key> --out <file>` and recover the exact original data    | ✓ VERIFIED | `handle_unwrap()` at line 692, `UnwrapCmd` struct at line 189, dispatched at line 265              |
| 2  | `trst wrap` derives encryption key from device signing key via HKDF (no hardcoded demo key in the codebase)      | ✓ VERIFIED | `derive_chunk_key(device_keypair.secret_bytes())` at line 346; grep for `0123456789abcdef` returns zero matches |
| 3  | `trst unwrap` verifies archive signature and continuity chain before producing any plaintext output               | ✓ VERIFIED | Signature check at line 710 exits code 10; continuity check at line 717 exits code 11; output write is at line 767 (after both checks) |
| 4  | A wrap-then-unwrap round-trip on arbitrary binary data produces byte-identical output                            | ✓ VERIFIED | `acceptance_unwrap_round_trip` and `acceptance_unwrap_generic_profile` both pass with `assert_eq!(original_data, recovered_data)` |
| 5  | `trst unwrap` on a tampered or incorrectly-keyed archive exits with a non-zero exit code and no plaintext output | ✓ VERIFIED | `acceptance_unwrap_wrong_key`, `acceptance_unwrap_tampered_manifest`, `acceptance_unwrap_missing_chunk` — all assert failure and output file absence |

**Score:** 5/5 truths verified

### Required Artifacts

| Artifact                                    | Expected                                              | Status     | Details                                                               |
|---------------------------------------------|-------------------------------------------------------|------------|-----------------------------------------------------------------------|
| `crates/core/src/crypto.rs`                 | `derive_chunk_key()` and `DeviceKeypair::secret_bytes()` | ✓ VERIFIED | Both functions present; 3 unit tests pass                             |
| `crates/core/src/lib.rs`                    | Re-export of `derive_chunk_key`                       | ✓ VERIFIED | Line 155: `derive_chunk_key` in pub use list                          |
| `crates/trst-cli/src/main.rs`               | `handle_unwrap()` + `UnwrapCmd` + nonce-prepended wrap | ✓ VERIFIED | All three present; no hardcoded key; `chunk_with_nonce` pattern at lines 370-376 |
| `crates/trst-cli/tests/acceptance.rs`       | 5 unwrap acceptance tests                             | ✓ VERIFIED | All 5 tests present and passing (24/24 total acceptance tests pass)   |

### Key Link Verification

| From                             | To                              | Via                                        | Status     | Details                                           |
|----------------------------------|---------------------------------|--------------------------------------------|------------|---------------------------------------------------|
| `main.rs (handle_wrap)`          | `crypto.rs`                     | `derive_chunk_key()` call at line 346      | ✓ WIRED    | Import at line 23, called with `secret_bytes()`   |
| `main.rs (handle_unwrap)`        | `crypto.rs`                     | `derive_chunk_key()` + `decrypt_segment()` at lines 724/754 | ✓ WIRED | Both imported line 23, both called               |
| `main.rs (handle_unwrap)`        | `archive.rs`                    | `read_archive()` + `validate_archive()`    | ✓ WIRED    | Both imported line 24, called at lines 700/717    |

### Requirements Coverage

| Requirement | Source Plan | Description                                                                              | Status      | Evidence                                                                 |
|-------------|-------------|------------------------------------------------------------------------------------------|-------------|--------------------------------------------------------------------------|
| UNWRAP-01   | 43-02       | User can run `trst unwrap` to decrypt and reassemble original data from a .trst archive  | ✓ SATISFIED | `handle_unwrap()` implemented; `acceptance_unwrap_round_trip` passes      |
| UNWRAP-02   | 43-01       | `trst wrap` derives encryption key via HKDF (replaces hardcoded demo key)                | ✓ SATISFIED | `derive_chunk_key()` in crypto.rs; no `0123456789abcdef` in codebase      |
| UNWRAP-03   | 43-02       | `trst unwrap` verifies archive integrity before decrypting                               | ✓ SATISFIED | Signature check (exit 10) + continuity check (exit 11) before file write; 3 error-path tests pass |
| UNWRAP-04   | 43-02       | Existing archives can be round-tripped (wrap → unwrap → identical data)                  | ✓ SATISFIED | `acceptance_unwrap_round_trip` + `acceptance_unwrap_generic_profile` pass with byte-identical assert |

No orphaned requirements — all four UNWRAP IDs appear in plan frontmatter and are accounted for.

### Anti-Patterns Found

No anti-patterns found in modified files (`crates/core/src/crypto.rs`, `crates/trst-cli/src/main.rs`, `crates/trst-cli/tests/acceptance.rs`).

The one `_ => {}` match arm at line 658 in main.rs is a legitimate discard of irrelevant error variants, not a stub.

### Human Verification Required

None. All success criteria are mechanically verifiable and confirmed via automated tests.

### Gaps Summary

No gaps. Phase goal fully achieved.

- `derive_chunk_key()` exists in `crates/core/src/crypto.rs`, is re-exported from `trustedge-core`, and has 3 passing unit tests.
- Hardcoded demo key `0123456789abcdef` is completely absent from the codebase.
- Chunk files are stored as `[nonce:24][ciphertext:N]` via `chunk_with_nonce`.
- `handle_unwrap()` implements verify-before-decrypt with correct exit codes (10 = sig fail, 11 = continuity fail, 1 = decrypt fail).
- All 5 unwrap acceptance tests pass. All 24 total acceptance tests pass with no regressions.

---

_Verified: 2026-03-17_
_Verifier: Claude (gsd-verifier)_
