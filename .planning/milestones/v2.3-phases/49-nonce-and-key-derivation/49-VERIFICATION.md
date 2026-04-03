<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


---
phase: 49-nonce-and-key-derivation
verified: 2026-03-20T00:00:00Z
status: passed
score: 3/3 must-haves verified
re_verification: false
---

# Phase 49: Nonce and Key Derivation Verification Report

**Phase Goal:** Users have concrete evidence that TrustEdge never reuses nonces within or across archives, and that HKDF key derivation is key-bound
**Verified:** 2026-03-20
**Status:** passed
**Re-verification:** No — initial verification

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | All chunk nonces within a single archive are unique (no nonce reuse) | VERIFIED | `test_sec05_all_chunk_nonces_unique_within_archive` and `test_sec05_nonce_not_all_zeros` pass: 16-chunk archive inspected, HashSet dedup confirms zero collisions, zero-nonce guard passes |
| 2 | Same plaintext + same device key produces different nonces across two separate archives | VERIFIED | `test_sec06_same_plaintext_same_key_different_nonces` passes: identical 64KB input wrapped twice with same key file; chunk 00000.bin nonces differ |
| 3 | HKDF derivation with different device keys produces different chunk encryption keys | VERIFIED | `test_sec07_different_device_keys_produce_different_chunk_keys`, `test_sec07_same_device_key_produces_same_chunk_key`, and `test_sec07_hkdf_produces_full_entropy_key` all pass; `derive_chunk_key()` called directly from `trustedge_core` |

**Score:** 3/3 truths verified

### Required Artifacts

| Artifact | Expected | Status | Details |
|----------|----------|--------|---------|
| `crates/trst-cli/tests/security_nonce_key_derivation.rs` | Security tests for nonce uniqueness and HKDF key binding | VERIFIED | 328 lines, 6 tests (2 SEC-05, 1 SEC-06, 3 SEC-07), MPL-2.0 header present, fully substantive |

### Key Link Verification

| From | To | Via | Status | Details |
|------|----|-----|--------|---------|
| `crates/trst-cli/tests/security_nonce_key_derivation.rs` | `crates/core/src/crypto.rs` | `derive_chunk_key()` re-exported from `trustedge_core` | WIRED | `use trustedge_core::derive_chunk_key;` at line 27; function defined at `crypto.rs:271`; re-exported at `lib.rs:155` |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
|-------------|-------------|-------------|--------|----------|
| SEC-05 | 49-01-PLAN.md | Test that nonces across chunks within a single archive are unique (no nonce reuse) | SATISFIED | `test_sec05_all_chunk_nonces_unique_within_archive` + `test_sec05_nonce_not_all_zeros` — HashSet dedup over 16 chunks, zero-nonce guard |
| SEC-06 | 49-01-PLAN.md | Test that the same plaintext encrypted twice with the same device key produces different nonces | SATISFIED | `test_sec06_same_plaintext_same_key_different_nonces` — two wrap invocations with shared key file, nonces asserted not-equal |
| SEC-07 | 49-01-PLAN.md | Test that HKDF derivation with different device keys produces different encryption keys | SATISFIED | Three tests: different-inputs-differ, same-input-deterministic, full-entropy sanity check |

REQUIREMENTS.md marks all three complete under Phase 49 at lines 77-79.

### Anti-Patterns Found

None. No TODO, FIXME, placeholder, stub, or empty-return patterns detected in the test file.

### Human Verification Required

None. All success criteria are programmatically verifiable (nonce bytes, key bytes, test pass/fail). The 6 tests ran and passed under `cargo test -p trustedge-trst-cli --test security_nonce_key_derivation`.

### Gaps Summary

No gaps. The single artifact exists, is fully substantive (328 lines, 6 real test functions with meaningful assertions), and the key link to `derive_chunk_key()` in `trustedge_core` is wired via direct import. Commit `ee004dc` is present in the repository. All three requirements in REQUIREMENTS.md are marked complete and attributed to Phase 49. The plan declared 7 tests in its done-criteria but the task body enumerated 6 named functions — the SUMMARY correctly documents this as an off-by-one in the plan prose, not a missing test. Six tests cover all three requirements without gap.

---

_Verified: 2026-03-20_
_Verifier: Claude (gsd-verifier)_
