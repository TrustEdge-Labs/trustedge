---
phase: 84-crypto-constants-file-extension
plan: "01"
subsystem: crypto-core
tags: [rebrand, crypto-constants, wire-format, clean-break, hkdf, aes-gcm]
one_liner: "Clean-break rename of two crypto wire-format constants: TRUSTEDGE-KEY-V1 → SEALEDGE-KEY-V1 (key file header) and TRUSTEDGE_ENVELOPE_V1 → SEALEDGE_ENVELOPE_V1 (HKDF domain), with 3 rejection tests proving legacy data is not silently accepted"
dependency_graph:
  requires: []
  provides: [REBRAND-03-crypto-constants]
  affects: [crates/core/src/crypto.rs, crates/core/src/envelope.rs, crates/seal-cli/tests/security_key_file_protection.rs]
tech_stack:
  added: []
  patterns: [shadow-const-test-module, hkdf-domain-separation-kat]
key_files:
  created: []
  modified:
    - crates/core/src/crypto.rs
    - crates/core/src/envelope.rs
    - crates/seal-cli/tests/security_key_file_protection.rs
decisions:
  - "D-01: Envelope version field stays at 2 — clean break achieved via AES-GCM tag failure, not version bump"
  - "D-02: Shadow-const test module pattern — legacy literals live only in #[cfg(test)] with zero production footprint"
  - "D-04: No dual-accept — is_encrypted_key_file() checks only SEALEDGE-KEY-V1 prefix, no fallback to old header"
metrics:
  duration_minutes: 15
  completed_date: "2026-04-18T22:45:56Z"
  tasks_completed: 1
  tasks_total: 1
  files_modified: 3
requirements: [REBRAND-03]
---

# Phase 84 Plan 01: Crypto Constants Rename Summary

## What Was Done

Renamed the two cryptographic wire-format constants from trustedge-branded to sealedge-branded values. The cryptographic scheme (HKDF-SHA256, AES-256-GCM, Ed25519, BLAKE3) is UNCHANGED — only the domain-separation strings and key-file magic header changed.

## Production Literal Changes

### crypto.rs line 28 — encrypted key file magic header

| Before | After |
|--------|-------|
| `const ENCRYPTED_KEY_HEADER: &str = "TRUSTEDGE-KEY-V1";` | `const ENCRYPTED_KEY_HEADER: &str = "SEALEDGE-KEY-V1";` |

Associated changes in the same file:
- `is_encrypted_key_file()` body: `b"TRUSTEDGE-KEY-V1\n"` → `b"SEALEDGE-KEY-V1\n"`
- Doc comment on `is_encrypted_key_file`: updated to reference new header
- Doc comment on `export_secret_encrypted`: updated output format example
- Existing `test_is_encrypted_key_file`: assert literal updated
- Existing `test_encrypted_key_format`: assert_eq and comment updated
- Existing `test_encrypted_key_rejects_low_iterations`: format! string updated

### envelope.rs line 103 — HKDF info byte literal

| Before | After |
|--------|-------|
| `let info = b"TRUSTEDGE_ENVELOPE_V1";` | `let info = b"SEALEDGE_ENVELOPE_V1";` |

Note: The comment on line 101 (`// The info parameter binds the derived key to the TrustEdge envelope v2 context.`) was intentionally left unchanged — brand-word prose in comments is Phase 86 scope per CONTEXT D-01.

## 3 New Clean-Break Rejection Tests

| Test Name | File | Purpose |
|-----------|------|---------|
| `test_old_header_rejected_cleanly` | `crates/core/src/crypto.rs` (mod tests) | Proves `is_encrypted_key_file()` returns false AND `import_secret_encrypted()` returns `InvalidKeyFormat("Expected header ...")` for a buffer prefixed with `b"TRUSTEDGE-KEY-V1"` |
| `test_old_domain_rejected_cleanly` | `crates/core/src/envelope.rs` (mod clean_break_tests) | Asserts the 32-byte AES key and 8-byte nonce prefix derived via HKDF under `OLD_ENVELOPE_DOMAIN` differ from the `SEALEDGE_ENVELOPE_V1` derivation — proving AES-GCM tag rejection of legacy envelopes |
| `test_old_domain_produces_distinct_okm` | `crates/core/src/envelope.rs` (mod clean_break_tests) | Direct KAT: HKDF-Expand over both info values with identical IKM+salt produces distinct 40-byte OKMs |

## Envelope Version Field — Unchanged

The 5 `assert_eq!(envelope.version, 2, ...)` test sites at lines 718, 751, 773, 785, 787 of envelope.rs were confirmed unchanged. The `version: 2` struct literal at line 186 was also left unchanged (per CONTEXT D-01).

```
grep -c 'version: 2,' crates/core/src/envelope.rs  → 1  (struct literal, unchanged)
grep -c 'envelope.version, 2' crates/core/src/envelope.rs  → 3  (test assertions, unchanged)
```

## Remaining Legacy Literal Occurrences

| File | Count | Location |
|------|-------|----------|
| `crates/core/src/crypto.rs` | 1 | Inside `test_old_header_rejected_cleanly` as `const OLD_KEY_HEADER: &[u8] = b"TRUSTEDGE-KEY-V1"` (shadow const, `#[cfg(test)]` only) |
| `crates/core/src/envelope.rs` | 1 | Inside `mod clean_break_tests` as `const OLD_ENVELOPE_DOMAIN: &[u8] = b"TRUSTEDGE_ENVELOPE_V1"` (shadow const, `#[cfg(test)]` only) |
| `crates/seal-cli/tests/security_key_file_protection.rs` | 0 | All fixtures fully realigned to `SEALEDGE-KEY-V1` |

## SEC-08 Test Fixture Updates

The following sites in `security_key_file_protection.rs` were updated from `TRUSTEDGE-KEY-V1` to `SEALEDGE-KEY-V1`:
- Line 9: file-level doc comment
- Line 92: `make_valid_encrypted_key` doc comment
- Line 103: `build_corrupted_key_file` byte literal
- Line 158: `sec_08_truncated_before_header_newline` doc comment + `b"SEALEDGE-KEY"` test data
- Line 169: `sec_08_truncated_after_header` doc comment + `b"SEALEDGE-KEY-V1\n"` test data
- Line 180: `sec_08_truncated_mid_json` doc comment + `b"SEALEDGE-KEY-V1\n{\"salt\":"` test data

## Commit

`f1b60e8` — refactor(84-01): rename crypto wire-format constants — TRUSTEDGE-* → SEALEDGE-*

## Validation Evidence

| Check | Result |
|-------|--------|
| `cargo fmt --check` | 0 (green) |
| `cargo clippy --workspace --all-targets -- -D warnings` | 0 (green) |
| `cargo check --workspace --locked` | 0 (green) |
| `cargo test --workspace --locked` | 0 (green) — 202 unit tests in sealedge-core all pass |
| `test_old_header_rejected_cleanly` | ok |
| `test_old_domain_rejected_cleanly` | ok |
| `test_old_domain_produces_distinct_okm` | ok |
| `test_is_encrypted_key_file` | ok |
| `test_encrypted_key_format` | ok |

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Removed unused `super::*` import in `clean_break_tests` module**
- **Found during:** cargo clippy run (Step 4)
- **Issue:** The `clean_break_tests` module used `use super::*` but only needed `hkdf::Hkdf` and `sha2::Sha256`, both of which were imported via their own explicit `use` statements. Clippy flagged `-D warnings` error: `unused import: super::*`.
- **Fix:** Removed `use super::*;` from `mod clean_break_tests`, keeping only `use hkdf::Hkdf;` and `use sha2::Sha256;`.
- **Files modified:** `crates/core/src/envelope.rs`
- **Commit:** Included in the atomic task commit `f1b60e8`

## Threat Surface Scan

No new network endpoints, auth paths, file access patterns, or schema changes introduced. The only changes are to in-memory constants and test modules. No threat flags.

## Self-Check: PASSED

- `crates/core/src/crypto.rs` — exists, contains new const
- `crates/core/src/envelope.rs` — exists, contains new info literal
- `crates/seal-cli/tests/security_key_file_protection.rs` — exists, fully realigned
- `.planning/phases/84-crypto-constants-file-extension/84-01-SUMMARY.md` — this file
- Commit `f1b60e8` — verified present in git log
