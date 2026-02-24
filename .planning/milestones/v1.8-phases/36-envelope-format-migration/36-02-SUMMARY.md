---
phase: 36-envelope-format-migration
plan: 02
subsystem: cryptography
tags: [hkdf, ecdh, aes-gcm, envelope, nonce, deterministic, backward-compat, versioning]

# Dependency graph
requires:
  - phase: 36-envelope-format-migration
    provides: v2 seal() with HKDF-once + deterministic counter nonces, Envelope.version field, hkdf_salt field, derive_shared_encryption_key() returning (key, nonce_prefix)
provides:
  - decrypt_chunk_v2(): HKDF-once key + deterministic nonce reconstruction (nonce_prefix || idx[1..4] || last_flag)
  - decrypt_chunk_v1(): per-chunk key_derivation_salt fallback with stored random nonce
  - unseal(): try-v2-first then fallback-to-v1; v2 key zeroized regardless of outcome
  - 5 new tests: v2 multi-chunk roundtrip, v2 single-chunk roundtrip, version field serialization, deterministic nonce uniqueness, v1 legacy fallback
  - Full backward-compat: v1 envelopes decrypt correctly via fallback path
affects: [37-keyring-hardening]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Try-v2-then-fallback-v1 unseal: derive key once for v2 attempt, zeroize before fallback decision, v1 fallback uses per-chunk derivation"
    - "V2 nonce reconstruction: nonce_prefix[0..8] || (sequence as u32).to_be_bytes()[1..4] || last_flag (0xFF/0x00)"
    - "V1 fallback: read nonce from NetworkChunk (stored random), derive key from ChunkManifest.key_derivation_salt"
    - "Zeroize v2 key unconditionally: encryption_key.zeroize() runs whether v2 decryption succeeds or falls back"

key-files:
  created: []
  modified:
    - crates/core/src/envelope.rs

key-decisions:
  - "try-then-fallback over version-field dispatch: more resilient to partially-migrated data; AES-GCM auth tag failure is the definitive v2 vs v1 discriminator"
  - "decrypt_chunk_v2 takes no SigningKey param: raw signing key not used inside the method; pre-derived encryption_key passed in eliminates clippy unused-param warning"
  - "V2 key zeroized before fallback decision: if v2 attempt fails we still zeroize, then v1 re-derives its own key per-chunk (each chunk zeroizes inline)"
  - "V1 fallback error is authoritative: if both v2 and v1 fail, return v1 error (more informative for genuinely corrupt data than v2 error)"

patterns-established:
  - "Envelope unseal flow: verify() -> sort chunks -> try v2 (derive-once + reconstruct nonces) -> zeroize v2 key -> on failure, fallback to v1 (per-chunk derive)"
  - "V1 legacy test pattern: seal v2 envelope for scaffolding, then rebuild Envelope struct fields directly with v1-style per-chunk encrypted chunks"

requirements-completed: [VER-02, TST-01, TST-02]

# Metrics
duration: 4min
completed: 2026-02-24
---

# Phase 36 Plan 02: Envelope Format Migration Summary

**Backward-compatible unseal() with try-v2-first (HKDF-once + deterministic nonce reconstruction) then v1-fallback (per-chunk salt), plus 5 new tests proving both paths work; all 21 envelope tests pass**

## Performance

- **Duration:** 4 min
- **Started:** 2026-02-24T00:06:47Z
- **Completed:** 2026-02-24T00:10:25Z
- **Tasks:** 2
- **Files modified:** 1

## Accomplishments
- Implemented `decrypt_chunk_v2()`: derives key once at envelope level, reconstructs deterministic nonce as `nonce_prefix[0..8] || chunk_index[1..4] (BE u32) || last_flag`; no SigningKey parameter (pre-derived key passed in to avoid clippy warning)
- Renamed existing `decrypt_chunk()` to `decrypt_chunk_v1()`: preserves per-chunk salt derivation from `ChunkManifest.key_derivation_salt`, uses stored random nonce from `NetworkChunk`
- Rewrote `unseal()` with try-v2-first then v1-fallback: derives v2 key once, attempts all chunks, zeroizes key regardless of outcome, falls back to v1 on any AES-GCM failure
- Added 5 new tests proving both paths: `test_v2_multi_chunk_roundtrip`, `test_v2_single_chunk_roundtrip`, `test_v2_envelope_version_field`, `test_v2_deterministic_nonces_are_unique`, `test_v1_legacy_fallback`
- All 21 envelope tests pass (16 pre-existing + 5 new); clippy clean with -D warnings

## Task Commits

Tasks 1 and 2 were committed atomically (tightly coupled — v1/v2 methods and their tests cannot compile independently):

1. **Task 1 + Task 2: v2 decrypt path + v1 fallback + 5 new tests** - `346360f` (feat)

**Plan metadata:** (docs commit — see below)

## Files Created/Modified
- `/home/john/vault/projects/github.com/trustedge/crates/core/src/envelope.rs` — Added decrypt_chunk_v2(), renamed decrypt_chunk() to decrypt_chunk_v1(), rewrote unseal() with try-then-fallback, added 5 new tests

## Decisions Made
- **try-then-fallback over version dispatch:** The plan specified this approach. AES-GCM authentication tag failure is the correct discriminator — if v2 nonce reconstruction produces the wrong nonce, decryption fails with a MAC error, which triggers the fallback cleanly.
- **No SigningKey in decrypt_chunk_v2:** The plan explicitly noted this — the raw signing key is not used inside decrypt_chunk_v2 (only the pre-derived encryption_key matters), and including it would trigger a clippy unused-parameter warning with -D warnings.
- **V2 key zeroized before fallback:** Even if v2 decryption fails, we zeroize the derived key before deciding to fall back. The v1 path re-derives per-chunk. This ensures key material lifetime is minimal in both code paths.
- **V1 test via direct struct construction:** The `test_v1_legacy_fallback` test seals a v2 envelope for scaffolding (metadata, verifying keys), then rebuilds the `Envelope` struct directly with v1-style per-chunk encrypted chunks, proving the fallback path is actually exercised.

## Deviations from Plan

None — plan executed exactly as written.

## Issues Encountered
- Tasks 1 and 2 could not be committed separately because the test for `test_v1_legacy_fallback` (Task 2) depends on `decrypt_chunk_v1` (Task 1), and both require the rewritten `unseal()`. Committed as a single atomic feat commit covering both tasks, same pattern as Plan 01.

## Next Phase Readiness
- Phase 36 is complete: seal() is v2-only, unseal() handles both v2 (primary) and v1 (fallback)
- Phase 37 (Keyring Hardening) can proceed independently — no envelope.rs changes needed
- VER-02, TST-01, TST-02 requirements fully addressed and tested

## Self-Check: PASSED

- crates/core/src/envelope.rs: FOUND
- SUMMARY.md: FOUND (this file)
- commit 346360f (feat(36-02): implement v2 decrypt path and v1 legacy fallback): FOUND

---
*Phase: 36-envelope-format-migration*
*Completed: 2026-02-24*
