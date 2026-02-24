---
phase: 36-envelope-format-migration
verified: 2026-02-24T00:30:00Z
status: passed
score: 9/9 must-haves verified
re_verification: false
---

# Phase 36: Envelope Format Migration Verification Report

**Phase Goal:** Envelope encryption uses HKDF-once key derivation with deterministic counter nonces, and the format version field enables backward-compatible decryption of both old (PBKDF2-per-chunk) and new (HKDF-once) envelopes
**Verified:** 2026-02-24T00:30:00Z
**Status:** passed
**Re-verification:** No — initial verification

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | `seal()` derives the AES-256-GCM encryption key exactly once per envelope via HKDF-Extract + Expand, not once per chunk | VERIFIED | `derive_shared_encryption_key()` called once before chunk loop at envelope.rs:166-167; chunk loop at lines 174-186 passes pre-derived `encryption_key` into `create_encrypted_chunk()` — no derive call inside the loop |
| 2 | Per-chunk nonces are deterministic counters (8-byte prefix || 3-byte chunk_index BE || 1-byte last_flag), not random | VERIFIED | Lines 340-344: `nonce[0..8].copy_from_slice(nonce_prefix)`, `nonce[8..11].copy_from_slice(&idx_be[1..4])`, `nonce[11] = if is_last_chunk { 0xFF } else { 0x00 }` — exactly 12 bytes, all deterministic |
| 3 | Envelope struct carries `version: u8` and `hkdf_salt: [u8; 32]` as top-level serde fields | VERIFIED | Lines 39-43: `#[serde(default = "default_envelope_version")] version: u8` and `#[serde(default)] hkdf_salt: [u8; 32]` both present on `Envelope` struct |
| 4 | New envelopes are always version 2 | VERIFIED | Line 192: `Envelope { version: 2, hkdf_salt, ... }` in `seal()` return; test `test_v2_multi_chunk_roundtrip` asserts `assert_eq!(envelope.version, 2)` |
| 5 | Decrypting a v2 envelope with the correct recipient key returns the original plaintext | VERIFIED | `decrypt_chunk_v2()` at lines 463-512 reconstructs nonce deterministically and decrypts via AES-256-GCM; `test_v2_multi_chunk_roundtrip` and `test_v2_single_chunk_roundtrip` both pass |
| 6 | Decrypting a v1 (legacy per-chunk salt) envelope succeeds via try-then-fallback without modifying stored data | VERIFIED | `unseal()` at lines 241-290 tries v2 first, zeroizes v2 key, then falls back to `decrypt_chunk_v1()` which reads `manifest.key_derivation_salt` (per-chunk) and stored nonce from `NetworkChunk`; `test_v1_legacy_fallback` passes |
| 7 | All 8 (now 16 pre-existing) envelope tests pass without regression | VERIFIED | `cargo test -p trustedge-core --lib -- envelope` reports 21 passed, 0 failed |
| 8 | A new multi-chunk round-trip test encrypts and decrypts a multi-chunk payload using the v2 format | VERIFIED | `test_v2_multi_chunk_roundtrip` at line 783 uses 3*DEFAULT_CHUNK_SIZE+500 payload, asserts version==2, 4 chunks, byte-for-byte match |
| 9 | A dedicated v1 legacy fallback test constructs a v1-format envelope and verifies the fallback decrypt path returns correct plaintext | VERIFIED | `test_v1_legacy_fallback` at line 922 builds a genuine v1 envelope (per-chunk salts, random nonces, nonzero pbkdf2_iterations), calls `unseal()`, asserts payload byte-for-byte match |

**Score:** 9/9 truths verified

### Required Artifacts

| Artifact | Expected | Status | Details |
|----------|----------|--------|---------|
| `crates/core/src/envelope.rs` | v2 envelope seal + HKDF-once key derivation + deterministic nonces | VERIFIED | 1033 lines; contains `derive_shared_encryption_key()`, `seal()`, `create_encrypted_chunk()`, `decrypt_chunk_v2()`, `decrypt_chunk_v1()`, `unseal()`, and 21 tests |
| `crates/core/src/envelope.rs` | `version: u8` field with serde default | VERIFIED | Line 39-40: `#[serde(default = "default_envelope_version")] version: u8` with `fn default_envelope_version() -> u8 { 1 }` at line 24 |
| `crates/core/src/envelope.rs` | `fn decrypt_chunk_v2` | VERIFIED | Line 463: `fn decrypt_chunk_v2(&self, chunk: &NetworkChunk, encryption_key: &[u8; 32], nonce_prefix: &[u8; 8], is_last_chunk: bool) -> Result<Vec<u8>>` |

### Key Link Verification

| From | To | Via | Status | Details |
|------|----|-----|--------|---------|
| `Envelope::seal` | `derive_shared_encryption_key` | called once before chunk loop, returns 40 bytes (32 key + 8 nonce prefix) | WIRED | Lines 166-167: single call before `payload.chunks()` loop at line 174; `hkdf.expand` at line 111 with 40-byte output |
| `Envelope::create_encrypted_chunk` | nonce construction | deterministic counter from nonce_prefix + chunk_index + last_flag | WIRED | Lines 340-344: `nonce[0..8].copy_from_slice(nonce_prefix)`, `nonce[8..11].copy_from_slice(&idx_be[1..4])`, `nonce[11] = if is_last_chunk { 0xFF } else { 0x00 }` |
| `Envelope::unseal` | `decrypt_chunk_v2` / `decrypt_chunk_v1` | try v2 first, if decryption error fall back to v1 | WIRED | Lines 245-290: IIFE captures v2 attempt result, `encryption_key.zeroize()` at 257, `if v2_result.is_ok()` at 270 guards early return, v1 fallback runs at 275-278 |
| `decrypt_chunk_v2` | `derive_shared_encryption_key` | uses envelope-level hkdf_salt (called once, nonce reconstructed from prefix + index) | WIRED | Lines 242-243: `derive_shared_encryption_key(decryption_key, &sender_public_key, &self.hkdf_salt)` in `unseal()` before v2 attempt; the `hkdf_salt` field is used as the salt parameter |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
|-------------|-------------|-------------|--------|----------|
| ENV-01 | 36-01-PLAN.md | Envelope encryption derives keys from ECDH shared secrets using HKDF-SHA256, not PBKDF2 | SATISFIED | `derive_shared_encryption_key()` uses `Hkdf::<Sha256>::new()` at line 104; no PBKDF2 in envelope.rs |
| ENV-02 | 36-01-PLAN.md | Encryption key derived once per envelope via HKDF-Extract + Expand, not per chunk | SATISFIED | Single `derive_shared_encryption_key()` call before the chunk loop in `seal()` (line 166) |
| ENV-03 | 36-01-PLAN.md | Per-chunk nonces use deterministic counter mode (NoncePrefix || chunk_index || last_flag) | SATISFIED | Lines 340-344 in `create_encrypted_chunk()`; mirrored in `decrypt_chunk_v2()` at lines 481-484 |
| VER-01 | 36-01-PLAN.md | Envelope format includes version field to distinguish v1 from v2 | SATISFIED | `version: u8` field with `#[serde(default = "default_envelope_version")]`; `seal()` always writes `version: 2` |
| VER-02 | 36-02-PLAN.md | Decryption path supports both v1 and v2 envelope formats | SATISFIED | try-then-fallback in `unseal()` (lines 241-290); `test_v1_legacy_fallback` exercises actual v1 fallback path |
| TST-01 | 36-02-PLAN.md | All existing envelope tests pass with updated KDF architecture | SATISFIED | 21 tests pass (0 failed); all 16 pre-existing tests included |
| TST-02 | 36-02-PLAN.md | Multi-chunk encryption/decryption verified end-to-end with new HKDF-based format | SATISFIED | `test_v2_multi_chunk_roundtrip` seals 3*DEFAULT_CHUNK_SIZE+500 bytes, asserts 4 chunks, byte-for-byte match after unseal |

**Orphaned requirement check:** REQUIREMENTS.md maps ENV-01, ENV-02, ENV-03, VER-01, VER-02, TST-01, TST-02 to Phase 36. All 7 are claimed in plan frontmatter (36-01 claims ENV-01/02/03/VER-01; 36-02 claims VER-02/TST-01/TST-02). No orphaned requirements.

**Out-of-scope note:** ENV-04, ENV-05, ENV-06 are mapped to Phase 35 (HKDF infrastructure), which is the preceding phase. KEY-01 through KEY-04 and TST-03 are mapped to Phase 37 (Keyring Hardening), correctly deferred.

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
|------|------|---------|----------|--------|
| None found | — | — | — | — |

Scanned `crates/core/src/envelope.rs` for TODO/FIXME/PLACEHOLDER markers, empty returns, and stub handlers. None detected. The `#[allow(clippy::too_many_arguments)]` at line 324 is justified — `create_encrypted_chunk` takes 7 parameters required for v2 encryption and is a private method.

### Human Verification Required

None. All truths are verifiable through code inspection and test output. The test suite comprehensively exercises both the v2 path and the v1 fallback path with byte-for-byte payload assertions.

### Gaps Summary

No gaps. All 9 observable truths verified. All 7 requirement IDs from PLAN frontmatter satisfied with direct code evidence. Both commits (59e6106, 346360f) confirmed present in git log. `cargo test -p trustedge-core --lib -- envelope` reports 21 passed, 0 failed. `cargo clippy -p trustedge-core -- -D warnings` is clean.

---

_Verified: 2026-02-24T00:30:00Z_
_Verifier: Claude (gsd-verifier)_
