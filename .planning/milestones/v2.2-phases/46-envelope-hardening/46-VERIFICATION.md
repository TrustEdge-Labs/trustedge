<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


---
phase: 46-envelope-hardening
verified: 2026-03-19T02:00:00Z
status: passed
score: 7/7 must-haves verified
---

# Phase 46: Envelope Hardening Verification Report

**Phase Goal:** The v1 envelope format is removed and PBKDF2 iteration minimums are enforced everywhere
**Verified:** 2026-03-19T02:00:00Z
**Status:** PASSED
**Re-verification:** No — initial verification

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | seal() produces v2 envelopes exclusively (no v1 code path exists) | VERIFIED | `version: 2` set at line 183; no v1 seal branch anywhere in envelope.rs |
| 2 | unseal() decrypts v2 envelopes without any v1 fallback branch | VERIFIED | Lines 214-253: straight-line v2 decrypt with single HKDF call, no conditional branching |
| 3 | No v1 decrypt function exists in the codebase | VERIFIED | grep across all crates for `decrypt_chunk_v1`, `v1_payload`, `default_envelope_version`, `test_v1_legacy_fallback` returns zero results |
| 4 | All envelope tests pass using v2-only code paths | VERIFIED | 20 envelope tests pass (summary reports 169 core tests total, all passing) |
| 5 | Any PBKDF2 call with fewer than 300,000 iterations returns an error | VERIFIED | assert! at builder level in both `KeyDerivationContext::with_iterations` and `KeyContext::with_iterations`; backend guard in `derive_key_internal` and `derive_key` before pbkdf2_hmac call |
| 6 | Default iteration count remains 600,000 | VERIFIED | `unwrap_or(600_000)` in both keyring backends; constant defined separately from minimum |
| 7 | Existing code that uses defaults continues to work unchanged | VERIFIED | Default path bypasses minimum check (600k > 300k); no changes to call sites that use defaults |

**Score:** 7/7 truths verified

### Required Artifacts

| Artifact | Expected | Status | Details |
|----------|----------|--------|---------|
| `crates/core/src/envelope.rs` | v2-only envelope seal/unseal, contains `fn unseal` | VERIFIED | Contains `fn unseal` at line 214; `version: 2` at line 183; no v1 code; `#[serde(default)]` removed from both `version` and `hkdf_salt` fields |
| `crates/core/src/backends/universal.rs` | PBKDF2 minimum enforcement, contains `300_000` | VERIFIED | `PBKDF2_MIN_ITERATIONS = 300_000` constant at line 19; used 3 times (const, assert condition, assert message) |
| `crates/core/src/backends/keyring.rs` | PBKDF2 minimum enforcement, contains `300_000` | VERIFIED | Backend guard at lines 84-88 before pbkdf2_hmac call using `PBKDF2_MIN_ITERATIONS` |
| `crates/core/src/backends/universal_keyring.rs` | PBKDF2 minimum enforcement, contains `300_000` | VERIFIED | Backend guard at lines 78-82 before pbkdf2_hmac call using `PBKDF2_MIN_ITERATIONS` |
| `crates/core/src/backends/traits.rs` | PBKDF2 minimum enforcement in KeyContext::with_iterations, contains `300_000` | VERIFIED | `assert!` at lines 73-75 using `crate::backends::universal::PBKDF2_MIN_ITERATIONS` |

### Key Link Verification

| From | To | Via | Status | Details |
|------|----|-----|--------|---------|
| `envelope.rs unseal()` | `derive_shared_encryption_key` | single HKDF call, no fallback | WIRED | Line 230-231: single unconditional call; no `if version == 1` branch present anywhere in the function |
| `universal.rs` | `KeyDerivationContext::with_iterations` | validation at builder level | WIRED | `assert!(iterations >= PBKDF2_MIN_ITERATIONS, ...)` at line 80-84 |
| `universal_keyring.rs` | `derive_key_internal` | validation before pbkdf2_hmac call | WIRED | Guard at lines 78-82: `if iterations < PBKDF2_MIN_ITERATIONS { return Err(...) }` |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
|-------------|------------|-------------|--------|----------|
| ENV-01 | 46-01 | v1 envelope format removed (user decision: remove entirely, stronger than deprecation warning) | SATISFIED | Zero occurrences of `decrypt_chunk_v1`, `default_envelope_version`, `v1_payload`, `test_v1_legacy_fallback` across all crates; `#[serde(default)]` removed from `version` and `hkdf_salt` fields |
| ENV-02 | 46-01 | seal() always produces v2 envelopes | SATISFIED | `version: 2` hardcoded at line 183 of envelope.rs; confirmed by test at line 691 asserting `envelope.version == 2` |
| KDF-01 | 46-02 | Any PBKDF2 usage enforces minimum 300,000 iterations | SATISFIED | Belt-and-suspenders: assert! at builder level (universal.rs, traits.rs) + error return at backend execution level (universal_keyring.rs, keyring.rs); two rejection tests with `#[should_panic]` |

Note on ENV-01: REQUIREMENTS.md text reads "v1 envelope format is deprecated — unseal() logs a deprecation warning" but the CONTEXT.md user decision explicitly overrides this to full removal ("stronger than deprecation"). The implementation satisfies the stronger interpretation — code is gone, not just warned about.

### Anti-Patterns Found

No anti-patterns detected in modified files:
- No TODO/FIXME/PLACEHOLDER markers found in changed files
- No stub return values (empty arrays, null, etc.)
- No sub-minimum iteration calls outside `#[should_panic]` rejection tests
- Sub-minimum calls in `universal_keyring.rs` (1000 iterations) and `universal.rs` (50_000 iterations) are correctly located inside `#[should_panic]` test functions — these are intentional negative test cases, not stubs

### Human Verification Required

None — all behavioral claims are verifiable via code inspection:
- The v2-only path is a structural property of the code (no conditional branches on version)
- PBKDF2 minimum is enforced at two independent code points before any crypto operation runs
- Rejection tests use `#[should_panic]` which is compiler-verified behavior

### Commit Verification

| Commit | Description | Status |
|--------|-------------|--------|
| `8aff537` | feat(46-01): remove v1 envelope format entirely | Present in git log |
| `2c40152` | feat(46-02): enforce PBKDF2 minimum 300k iterations at builder and backend level | Present in git log |

### Gaps Summary

No gaps. All three requirements (ENV-01, ENV-02, KDF-01) are satisfied with evidence in the codebase:

- v1 envelope format is entirely absent from the codebase (zero grep matches across all crates)
- unseal() is a clean straight-line v2 decrypt (lines 214-253, no branching on version)
- seal() sets `version: 2` hardcoded (line 183)
- PBKDF2 minimum 300,000 is enforced at four independent code points: two builders (assert!) and two backends (error return)
- Default 600,000 iteration count is unchanged; all existing callers using defaults are unaffected

---

_Verified: 2026-03-19T02:00:00Z_
_Verifier: Claude (gsd-verifier)_
