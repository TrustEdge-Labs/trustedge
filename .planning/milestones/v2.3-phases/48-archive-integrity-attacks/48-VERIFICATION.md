<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


---
phase: 48-archive-integrity-attacks
verified: 2026-03-20T23:03:38Z
status: passed
score: 4/4 must-haves verified
---

# Phase 48: Archive Integrity Attacks — Verification Report

**Phase Goal:** Any modification to a .trst archive is detected and rejected by trst verify
**Verified:** 2026-03-20T23:03:38Z
**Status:** passed
**Re-verification:** No — initial verification

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | Flipping any byte in an encrypted chunk causes trst verify to fail with exit code 11 | VERIFIED | `test_sec01_encrypted_chunk_byte_flip` passes; `test_sec01_unencrypted_chunk_byte_flip_last_byte` passes — both assert `.code(11).stderr(contains("hash mismatch"))` |
| 2 | Adding a spurious chunk file not referenced in the manifest causes trst verify to fail with exit code 11 and an error about unreferenced chunk files | VERIFIED | `test_sec02_injected_extra_chunk` passes — asserts `.code(11).stderr(contains("nreferenced chunk"))` |
| 3 | Swapping two chunk files causes trst verify to fail with exit code 11 | VERIFIED | `test_sec03_swap_adjacent_chunks` passes — asserts `.code(11).stderr(contains("hash mismatch"))` |
| 4 | Modifying any manifest field after signing causes trst verify to fail with exit code 10 | VERIFIED | `test_sec04_manifest_profile_change`, `test_sec04_manifest_device_id_change`, `test_sec04_manifest_segment_hash_change` all pass — assert `.code(10).stderr(contains("Signature verification failed"))` |

**Score:** 4/4 truths verified

### Required Artifacts

| Artifact | Provides | Status | Details |
|----------|----------|--------|---------|
| `crates/core/src/archive.rs` | Unreferenced chunk detection in `validate_archive()` | VERIFIED | Contains `HashSet<String>` expected-chunk check in `validate_archive()` at lines 127-143; `ArchiveError::UnreferencedChunk` returned on any `.bin` file not in manifest.segments; unit test `test_unreferenced_chunk_detected` confirms detection (8/8 archive unit tests pass) |
| `crates/trst-cli/tests/security_archive_integrity.rs` | Security tests for archive tampering detection | VERIFIED | 334 lines; 8 test functions covering all four attack vectors (SEC-01 x2, SEC-02 x2, SEC-03 x1, SEC-04 x3); all 8 tests pass |

### Key Link Verification

| From | To | Via | Status | Details |
|------|----|-----|--------|---------|
| `crates/core/src/archive.rs` | `crates/trst-cli/src/main.rs` | `validate_archive()` called after signature pass, errors exit 11 | WIRED | `validate_archive` imported at line 26; called at line 833 (post-signature, json-report path) and line 920 (plain-text path); errors routed through `output_continuity_error()` which emits "hash mismatch" / "Unreferenced chunk file" messages, then `process::exit(11)` |
| `crates/trst-cli/tests/security_archive_integrity.rs` | trst verify CLI | `assert_cmd Command::cargo_bin("trst")` | WIRED | `Command::cargo_bin("trst")` at lines 46, 77, 94, 124; `run_verify()` helper wires test assertions to actual CLI binary exit codes and stderr |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
|-------------|-------------|-------------|--------|----------|
| SEC-01 | 48-01 | Test that modifying any byte of an encrypted chunk causes trst verify to fail (AES-GCM auth tag detection) | SATISFIED | `test_sec01_encrypted_chunk_byte_flip` and `test_sec01_unencrypted_chunk_byte_flip_last_byte` both pass; BLAKE3 hash mismatch detected at exit 11 |
| SEC-02 | 48-01 | Test that injecting an extra chunk file into a .trst archive causes verification failure (BLAKE3 chain break) | SATISFIED | `validate_archive()` now scans chunks/ directory for unreferenced .bin files; `test_sec02_injected_extra_chunk` asserts exit 11 with "nreferenced chunk" in stderr; `test_sec02_injected_chunk_replacing_existing` asserts exit 11 with "hash mismatch" |
| SEC-03 | 48-01 | Test that reordering chunk files in a .trst archive causes verification failure (continuity chain) | SATISFIED | `test_sec03_swap_adjacent_chunks` passes; swap of chunks/00003.bin and chunks/00004.bin detected via BLAKE3 hash mismatch at exit 11 (BLAKE3 check runs before continuity chain; both would catch the error) |
| SEC-04 | 48-01 | Test that modifying manifest.json after signing causes signature verification failure | SATISFIED | Three tests each modify a different manifest field (profile, device id, segment blake3_hash) and all assert exit 10 with "Signature verification failed" in stderr |

**Orphaned requirements check:** REQUIREMENTS.md maps SEC-01, SEC-02, SEC-03, SEC-04 all to Phase 48. All four are claimed in plan 48-01 and verified. No orphaned requirements.

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
|------|------|---------|----------|--------|
| None | — | — | — | No TODOs, stubs, or placeholder patterns found in modified files |

Checked `crates/core/src/archive.rs`, `crates/core/src/error.rs`, `crates/trst-cli/src/main.rs`, and `crates/trst-cli/tests/security_archive_integrity.rs` — no red-flag patterns.

### Human Verification Required

None. All observable truths were verified programmatically via passing tests with specific exit codes and stderr assertions. No visual, real-time, or external-service behavior involved.

### Gaps Summary

No gaps. All must-haves verified:

1. `validate_archive()` in `archive.rs` detects unreferenced chunk files via `HashSet<String>` scan of the chunks/ directory — the SEC-02 system fix is real and substantive.
2. `ArchiveError::UnreferencedChunk` variant exists in `error.rs` with a proper thiserror display message and is handled exhaustively in both CLI match arms.
3. `test_unreferenced_chunk_detected` unit test in `archive.rs` creates an archive, writes `chunks/99999.bin`, calls `validate_archive()`, and asserts `Err(ArchiveError::UnreferencedChunk("99999.bin"))` — the test asserts failure, not just that the function runs.
4. All 8 security integration tests in `security_archive_integrity.rs` pass against the real `trst` binary, with exact exit code and stderr string assertions.
5. Existing 28 acceptance tests pass without regression.
6. Commits `3d14593` (archive.rs fix) and `ed93f1a` (security tests) both verified in git history.

**Note on SEC-03:** The ROADMAP success criterion says "continuity chain error" but the implementation correctly detects the swap via BLAKE3 hash mismatch (which runs before the continuity chain check). Both mechanisms would catch the reorder; BLAKE3 fires first. This is correct behavior and the test passes — the detection guarantee is satisfied.

---

_Verified: 2026-03-20T23:03:38Z_
_Verifier: Claude (gsd-verifier)_
