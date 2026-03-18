---
phase: 43-archive-decryption-trst-unwrap
plan: "02"
subsystem: trst-cli
tags: [trst, unwrap, decryption, acceptance-tests, round-trip]
dependency_graph:
  requires: ["43-01"]
  provides: ["trst-unwrap-command", "unwrap-acceptance-tests"]
  affects: ["crates/trst-cli/src/main.rs", "crates/trst-cli/tests/acceptance.rs"]
tech_stack:
  added: []
  patterns:
    - "verify-before-decrypt: signature + continuity checked before producing any plaintext"
    - "exit-code contract: 10=sig fail, 11=continuity fail, 1=decrypt fail"
    - "no-partial-output: process::exit() on all failure paths before file write"
key_files:
  created: []
  modified:
    - crates/trst-cli/src/main.rs
    - crates/trst-cli/tests/acceptance.rs
decisions:
  - "Used process::exit() on all error paths to ensure no partial output file is written"
  - "ProfileMetadata match extracts started_at for AAD reconstruction at unwrap time"
  - "validate_archive() called after signature check per plan spec (acceptable double-read)"
metrics:
  duration_seconds: 765
  completed_date: "2026-03-17"
  tasks_completed: 2
  tasks_total: 2
  files_modified: 2
---

# Phase 43 Plan 02: trst unwrap Command Summary

**One-liner:** `trst unwrap` command that verifies Ed25519 signature + BLAKE3 continuity chain before decrypting XChaCha20Poly1305 chunks and reassembling original data byte-identically.

## Tasks Completed

| Task | Name | Commit | Files |
|------|------|--------|-------|
| 1 | Add Unwrap CLI command and handle_unwrap() | 6f66039 | crates/trst-cli/src/main.rs |
| 2 | Add acceptance tests for unwrap round-trip and error cases | 4b680f7 | crates/trst-cli/tests/acceptance.rs |

## What Was Built

### Task 1: Unwrap CLI Command

Added `UnwrapCmd` struct, `Unwrap(UnwrapCmd)` enum variant, and `handle_unwrap()` function to `crates/trst-cli/src/main.rs`.

The function implements the security-first unwrap protocol:
1. Load device keypair from key file
2. Read archive (manifest + chunk bytes)
3. Verify Ed25519 signature — exit code 10 on failure, no output written
4. Validate BLAKE3 continuity chain — exit code 11 on failure, no output written
5. Derive XChaCha20Poly1305 chunk key via HKDF-SHA256 from signing key
6. Extract `started_at` from `ProfileMetadata` enum for AAD reconstruction
7. Decrypt all chunks in order, accumulate plaintext
8. Write output file only after all chunks successfully decrypted
9. Print summary (chunks, bytes, output path) to stderr

Decryption failure (wrong key causes AEAD authentication tag mismatch) exits with code 1 before any file write.

### Task 2: Acceptance Tests

Added 5 acceptance tests to `crates/trst-cli/tests/acceptance.rs` plus two helper functions (`run_unwrap`, `wrap_with_key`):

| Test | What It Verifies |
|------|-----------------|
| `acceptance_unwrap_round_trip` | cam.video wrap then unwrap produces byte-identical output (64KB) |
| `acceptance_unwrap_wrong_key` | Wrong device key: exits non-zero, output file not created |
| `acceptance_unwrap_tampered_manifest` | Tampered profile string: exits non-zero with "Signature: FAIL", output file not created |
| `acceptance_unwrap_generic_profile` | generic profile wrap-unwrap produces byte-identical output |
| `acceptance_unwrap_missing_chunk` | Deleted last chunk: exits non-zero, output file not created |

All 24 acceptance tests pass (19 pre-existing + 5 new).

## Verification Results

- `cargo test -p trustedge-trst-cli --test acceptance`: 24/24 passed
- `cargo clippy --workspace -- -D warnings`: clean
- `cargo build -p trustedge-trst-cli`: clean
- `trst unwrap --help`: shows correct usage

## Deviations from Plan

None — plan executed exactly as written.

## Self-Check: PASSED

- SUMMARY.md: FOUND
- Commit 6f66039 (feat: unwrap command): FOUND
- Commit 4b680f7 (test: acceptance tests): FOUND
- All 24 acceptance tests: PASSED
- clippy --workspace: CLEAN
