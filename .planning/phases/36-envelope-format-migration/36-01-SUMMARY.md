---
phase: 36-envelope-format-migration
plan: 01
subsystem: cryptography
tags: [hkdf, ecdh, aes-gcm, envelope, nonce, deterministic, versioning, serde]

# Dependency graph
requires:
  - phase: 35-hkdf-infrastructure
    provides: HKDF-SHA256 Extract+Expand replacing PBKDF2 CatKDF, hkdf 0.12 workspace dep
provides:
  - Envelope struct with version:u8 and hkdf_salt:[u8;32] fields (serde defaults for v1 compat)
  - derive_shared_encryption_key() returning 40-byte OKM split as ([u8;32] key, [u8;8] nonce_prefix)
  - v2 seal() path: single HKDF call per envelope, deterministic counter nonces
  - Nonce layout: nonce_prefix[0..8] || chunk_index[1..4] (BE u32) || last_flag (0xFF/0x00)
  - ChunkManifest key_derivation_salt and pbkdf2_iterations zeroed in v2 (serde compat preserved)
affects: [36-02-decrypt-path, 37-keyring-hardening]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "HKDF-once per envelope: derive (key, nonce_prefix) from single 40-byte OKM expansion before chunk loop"
    - "Deterministic nonce: nonce_prefix[0..8] || chunk_index[1..4] (low 3 bytes of BE u32) || last_flag byte"
    - "Format versioning: Envelope.version:u8 with serde default=1 for v1 backward compat"
    - "OKM zeroize: 40-byte buffer zeroized immediately after split into key+prefix copies"
    - "Envelope-level key zeroize: encryption_key zeroized after all chunks sealed, not per-chunk"

key-files:
  created: []
  modified:
    - crates/core/src/envelope.rs

key-decisions:
  - "Single HKDF derivation per envelope: eliminates per-chunk key derivation overhead; 32-byte key + 8-byte nonce prefix from 40-byte OKM"
  - "Deterministic nonce: nonce_prefix[0..8] || chunk_index[1..4] (BE) || last_flag avoids RNG calls per chunk and makes nonce construction auditable"
  - "serde defaults for backward compat: version defaults to 1u8, hkdf_salt defaults to [0;32] enabling old v1 envelopes to deserialize"
  - "ChunkManifest fields zeroed in v2: key_derivation_salt=[0;32] and pbkdf2_iterations=0 maintain serde shape without carrying stale data"
  - "OKM buffer zeroized immediately: 40-byte intermediate buffer cleared before returning (key, prefix) copies"
  - "Envelope-level encryption key zeroized after chunk loop: moved from per-chunk to per-envelope level in v2"

patterns-established:
  - "V2 seal flow: generate hkdf_salt → derive_once → for each chunk: build_nonce(prefix, idx, last) → encrypt → zeroize key after loop"
  - "Nonce index encoding: (sequence as u32).to_be_bytes()[1..4] extracts low 3 bytes for 24-bit counter (16M chunk limit)"

requirements-completed: [ENV-01, ENV-02, ENV-03, VER-01]

# Metrics
duration: 4min
completed: 2026-02-23
---

# Phase 36 Plan 01: Envelope Format Migration Summary

**v2 envelope seal path with single HKDF-SHA256 derivation (40-byte OKM) producing one AES-256-GCM key and one 8-byte nonce prefix used for deterministic counter nonces across all chunks**

## Performance

- **Duration:** 4 min
- **Started:** 2026-02-23T23:49:44Z
- **Completed:** 2026-02-23T23:53:50Z
- **Tasks:** 2
- **Files modified:** 1

## Accomplishments
- Added `version: u8` and `hkdf_salt: [u8; 32]` to Envelope struct with serde defaults enabling v1 backward compat (old envelopes default to version=1, hkdf_salt=[0;32])
- Rewrote `derive_shared_encryption_key()` to return `([u8; 32], [u8; 8])` — 40-byte OKM split into encryption key + nonce prefix; intermediate OKM buffer zeroized before returning
- Rewrote `seal()` to generate one random `hkdf_salt` and call derive once before the chunk loop (v2 path)
- Rewrote `create_encrypted_chunk()` to accept pre-derived `encryption_key` and `nonce_prefix`; constructs deterministic 12-byte nonce: `prefix[0..8] || idx[1..4] (BE u32) || last_flag`
- ChunkManifest fields `key_derivation_salt` and `pbkdf2_iterations` zeroed in v2 (serde shape preserved for Plan 02 format compat)
- All 16 envelope tests pass including roundtrip, multi-chunk, wrong-key, and third-party-cannot-decrypt

## Task Commits

Both tasks were committed atomically (tightly coupled — struct changes and seal rewrite both required for compilation):

1. **Task 1 + Task 2: v2 envelope format implementation** - `59e6106` (feat)

**Plan metadata:** (docs commit — see below)

## Files Created/Modified
- `/home/john/vault/projects/github.com/trustedge/crates/core/src/envelope.rs` — Envelope struct + derive function + seal() + create_encrypted_chunk() + decrypt_chunk() minimal compile fix

## Decisions Made
- **Single HKDF derivation:** The previous code derived a new key per chunk using a per-chunk random salt. This is wasteful (ECDH per chunk) and cryptographically unnecessary. One ECDH + HKDF per envelope is the correct pattern (matching Tink AES-GCM-HKDF streaming model).
- **Deterministic nonces:** Per-chunk random nonces eliminate the ability to audit nonce uniqueness. Deterministic counter nonces are provably unique within an envelope (given unique HKDF salt) and support verification.
- **OKM split at 40 bytes:** 32 bytes for AES-256-GCM key + 8 bytes for nonce prefix. 8-byte prefix gives 2^64 envelope uniqueness combined with the random `hkdf_salt`.
- **Minimal decrypt_chunk fix:** Plan 02 will implement full v1/v2 dispatch. For this plan, `decrypt_chunk()` was minimally updated to use the new 3-argument `derive_shared_encryption_key()` signature, with a version check to select the correct salt (`hkdf_salt` for v2, `key_derivation_salt` for v1). This keeps compilation clean without breaking the test suite.

## Deviations from Plan

**1. [Rule 1 - Compile fix] Minimal decrypt_chunk update to match new function signature**
- **Found during:** Task 1 (changing derive_shared_encryption_key signature)
- **Issue:** Old call site in decrypt_chunk() passed 6 arguments to the now-3-argument function; would prevent cargo check from passing
- **Fix:** Updated decrypt_chunk() call to use new 3-argument signature with version check to select salt (hkdf_salt for v2, key_derivation_salt for v1). Plan 02 will replace with full v1/v2 dispatch including correct nonce reconstruction.
- **Files modified:** crates/core/src/envelope.rs (decrypt_chunk)
- **Verification:** cargo check passes, all 16 tests pass
- **Committed in:** 59e6106

---

**Total deviations:** 1 auto-fixed (compile fix required by signature change)
**Impact on plan:** Compile fix is minimal scope — decrypt_chunk now uses new signature with basic version dispatch. Plan 02's full decrypt path will supersede this code.

## Issues Encountered
- Tasks 1 and 2 could not be committed separately because the old call sites (create_encrypted_chunk, decrypt_chunk) used the old 6-argument signature. Both had to be updated for cargo check to pass. Committed as a single atomic feat commit covering both tasks.

## Next Phase Readiness
- Plan 02 (decrypt path) can now implement full v1/v2 dispatch: for v2 envelopes, derive once using hkdf_salt and reconstruct deterministic nonce from nonce_prefix + chunk index; for v1 envelopes, use per-chunk key_derivation_salt with the legacy path
- The minimal decrypt_chunk fix in this plan correctly routes by version, but does not yet reconstruct the v2 nonce — that is Plan 02's work
- All other envelope tests continue to pass

## Self-Check: PASSED

- crates/core/src/envelope.rs: FOUND
- SUMMARY.md: FOUND (this file)
- commit 59e6106 (feat: implement v2 envelope format): FOUND

---
*Phase: 36-envelope-format-migration*
*Completed: 2026-02-23*
