---
phase: 85-code-sweep-headers-text-metadata
plan: "01"
subsystem: core-crypto
tags: [rebrand, crypto, wire-format, clean-break, d02-tests]
dependency_graph:
  requires: []
  provides: [sealedge-crypto-domains, sealedge-magic-header]
  affects: [sealedge-core, all-crates-using-derive_chunk_key, all-crates-using-session-key, all-crates-using-chain, all-crates-using-format]
tech_stack:
  added: []
  patterns: [d02-shadow-const-rejection-tests, hkdf-sha256-domain-separation, blake3-derive-key-domain-separation]
key_files:
  created: []
  modified:
    - crates/core/src/crypto.rs
    - crates/core/src/auth.rs
    - crates/core/src/chain.rs
    - crates/core/src/format.rs
    - crates/core/tests/domain_separation_test.rs
    - crates/core/src/bin/inspect-seal.rs
    - crates/core/benches/crypto_benchmarks.rs
    - crates/core/src/vectors.rs
decisions:
  - "Clean-break rename of all 5 crypto wire-format byte literals in sealedge-core — no backward-compat decode path (matches Phase 84 precedent and v6.0 clean-break preference)"
  - "Golden BLAKE3 digest in vectors.rs updated to reflect new MAGIC bytes (expected cascade from wire-format change)"
  - "D-02 manifest domain tests placed at top-level in integration test file (not submodule) since cargo integration test discovery requires top-level #[test] fns"
  - "network_integration pre-existing failures (trustedge-server binary name) deferred — out of Plan 01 scope"
metrics:
  duration: "~30 minutes"
  completed: "2026-04-19T11:47:46Z"
  tasks_completed: 3
  files_modified: 8
---

# Phase 85 Plan 01: Core Crypto Byte-Literal Domain Rename Summary

**One-liner:** Clean-break rename of 5 sealedge-core wire-format crypto constants (HKDF info, BLAKE3 contexts, MAGIC) with 8 new D-02 rejection/KAT tests.

## Production Constant Renames

| File | Line | Old Value | New Value |
|------|------|-----------|-----------|
| `crates/core/src/crypto.rs` | 286 (doc) | `` `TRUSTEDGE_TRST_CHUNK_KEY` `` | `` `SEALEDGE_SEAL_CHUNK_KEY` `` |
| `crates/core/src/crypto.rs` | 291 | `b"TRUSTEDGE_TRST_CHUNK_KEY"` | `b"SEALEDGE_SEAL_CHUNK_KEY"` |
| `crates/core/src/auth.rs` | 320 | `"TRUSTEDGE_SESSION_KEY_V1"` | `"SEALEDGE_SESSION_KEY_V1"` |
| `crates/core/src/chain.rs` | 10 | `b"trustedge:genesis"` | `b"sealedge:genesis"` |
| `crates/core/src/format.rs` | 14 | `b"TRST"` | `b"SEAL"` |
| `crates/core/src/format.rs` | 141 | `b"trustedge.manifest.v1"` | `b"sealedge.manifest.v1"` |

## MAGIC Rename Cascades

| File | Change |
|------|--------|
| `crates/core/src/bin/inspect-seal.rs:40` | `println!("File format: TRST v{}")` → `println!("File format: SEAL v{}")` |
| `crates/core/benches/crypto_benchmarks.rs:364` | `data.starts_with(b"TRST")` → `data.starts_with(b"SEAL")` |
| `crates/core/benches/crypto_benchmarks.rs:365` | `"trustedge"` → `"sealedge"` (detected-format label) |
| `crates/core/src/vectors.rs:73-74` | Golden BLAKE3 digest updated: `162efe...` → `d43287...` (wire-format changed) |
| `crates/core/tests/domain_separation_test.rs:128` | `assert_eq!(MANIFEST_DOMAIN_SEP, b"trustedge.manifest.v1")` → `b"sealedge.manifest.v1"` |

## 8 New D-02 Clean-Break Tests

| Test Name | Location | What It Proves |
|-----------|----------|----------------|
| `test_old_chunk_key_domain_produces_distinct_okm` | `crypto.rs::tests::clean_break_chunk_key_tests` | HKDF-SHA256 with two different info values produces distinct 32-byte OKMs |
| `test_old_chunk_key_domain_rejected_cleanly` | `crypto.rs::tests::clean_break_chunk_key_tests` | Distinct AEAD keys imply XChaCha20Poly1305 tag failure on old-domain chunks |
| `test_old_session_key_domain_produces_distinct_okm` | `auth.rs::tests::clean_break_session_key_tests` | BLAKE3 derive_key with two contexts produces distinct 32-byte session keys |
| `test_old_session_key_domain_rejected_cleanly` | `auth.rs::tests::clean_break_session_key_tests` | Distinct session keys imply AEAD tag failure for legacy sessions |
| `test_old_genesis_seed_produces_distinct_hash` | `chain.rs::clean_break_genesis_tests` | BLAKE3(trustedge:genesis) != BLAKE3(sealedge:genesis) |
| `test_old_genesis_seed_rejected_cleanly` | `chain.rs::clean_break_genesis_tests` | First chain-block hash differs under two seeds — verifier rejects old-rooted chains |
| `test_old_manifest_domain_produces_distinct_signature` | `tests/domain_separation_test.rs` (top-level) | Ed25519 sig over two different domain prefixes produces distinct signature bytes |
| `test_old_manifest_domain_rejected_cleanly` | `tests/domain_separation_test.rs` (top-level) | Signature from old domain fails Ed25519 verification under new domain prefix |

All 8 tests pass. Total sealedge-core lib tests: 208. Total domain_separation integration tests: 9.

## Phase 84 Shadow Consts — Confirmed Preserved

| Const | File | Line | Value |
|-------|------|------|-------|
| `OLD_ENVELOPE_DOMAIN` | `crates/core/src/envelope.rs` | 858 | `b"TRUSTEDGE_ENVELOPE_V1"` |
| `OLD_KEY_HEADER` | `crates/core/src/crypto.rs` | 863 | `b"TRUSTEDGE-KEY-V1"` |

Verification: `grep -c 'OLD_ENVELOPE_DOMAIN' crates/core/src/envelope.rs` = 6; `grep -c 'OLD_KEY_HEADER' crates/core/src/crypto.rs` = 2.

## Legacy Byte-Literal Occurrences After Rename

| Legacy Value | Remaining Hits | Location |
|--------------|----------------|----------|
| `b"TRUSTEDGE_TRST_CHUNK_KEY"` | 1 | `crypto.rs` inside `mod clean_break_chunk_key_tests` (shadow const only) |
| `"TRUSTEDGE_SESSION_KEY_V1"` | 1 | `auth.rs` inside `mod clean_break_session_key_tests` (shadow const only) |
| `b"trustedge:genesis"` | 1 | `chain.rs` inside `mod clean_break_genesis_tests` (shadow const only) |
| `b"trustedge.manifest.v1"` | 1 | `domain_separation_test.rs` as top-level `OLD_MANIFEST_DOMAIN_SEP` const (test-only) |
| `b"TRST"` | 0 | Fully replaced — no shadow const (plain rename per CONTEXT D-02) |

## Commit

- Hash: `1086637` (108663765cd77bed7967269043fed5b1525efdda)
- Message: `refactor(85-01): crypto byte-literal domains renamed to sealedge`
- Files: 8 modified, 198 insertions, 12 deletions

## Validation Evidence

| Check | Result |
|-------|--------|
| `cargo fmt --check` | green |
| `cargo clippy --workspace --all-targets -- -D warnings` | green |
| `cargo check --workspace --locked` | green |
| `cargo test --workspace --locked` (core lib) | 208 passed, 0 failed |
| `cargo test -p sealedge-core --test domain_separation_test` | 9 passed, 0 failed |
| Phase 84 shadow consts preserved | confirmed |
| Experimental workspace `cargo check` | pre-existing rustc 1.95 requirement (not caused by this plan) |
| `network_integration` tests | pre-existing failure (trustedge-server binary name from Phase 83 rename, out of scope) |

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Golden BLAKE3 digest in vectors.rs needed updating**
- **Found during:** Task 3 full workspace test
- **Issue:** `vectors::tests::golden_trst_digest_is_stable` failed because the MAGIC rename (`b"TRST"` → `b"SEAL"`) changes the wire-format BLAKE3 hash. The test is designed to detect exactly this: it prints the new digest and instructs the developer to update it.
- **Fix:** Updated `GOLDEN_TRST_BLAKE3` from `162efe3e...` to `d432874a...` in `crates/core/src/vectors.rs`
- **Files modified:** `crates/core/src/vectors.rs`
- **Commit:** included in the same atomic commit `1086637`

**2. [Rule 1 - Deviation] D-02 manifest domain tests placed at file top-level (not in submodule)**
- **Found during:** Task 2 verification
- **Issue:** `#[cfg(test)] mod clean_break_manifest_domain_tests { ... }` inside an integration test file (`tests/domain_separation_test.rs`) is not discovered by cargo test — integration test runners only discover top-level `#[test]` functions.
- **Fix:** Moved the shadow const and two D-02 test functions to the top level of `domain_separation_test.rs` instead of nesting them in a submodule. Same cryptographic content, different structural placement.
- **Files modified:** `crates/core/tests/domain_separation_test.rs`
- **Commit:** included in the same atomic commit `1086637`

### Pre-existing Out-of-Scope Issues (not fixed)

- `network_integration` tests look for `target/debug/trustedge-server` (old binary name from Phase 83 rename). Pre-existing since Phase 83. Logged to deferred-items.
- Experimental workspace requires rustc >= 1.95.0 (`constant_time_eq@0.4.3` dependency). Pre-existing environment constraint unrelated to this plan.

## Self-Check: PASSED

- FOUND: `crates/core/src/crypto.rs` (modified, committed)
- FOUND: `crates/core/src/auth.rs` (modified, committed)
- FOUND: `crates/core/src/chain.rs` (modified, committed)
- FOUND: `crates/core/src/format.rs` (modified, committed)
- FOUND: `crates/core/tests/domain_separation_test.rs` (modified, committed)
- FOUND: `crates/core/src/bin/inspect-seal.rs` (modified, committed)
- FOUND: `crates/core/benches/crypto_benchmarks.rs` (modified, committed)
- FOUND: `crates/core/src/vectors.rs` (modified, committed)
- FOUND: commit `1086637` (108663765cd77bed7967269043fed5b1525efdda) in git log
- FOUND: SUMMARY.md at `.planning/phases/85-code-sweep-headers-text-metadata/85-01-SUMMARY.md`
- FOUND: SUMMARY committed at `5493f25`
