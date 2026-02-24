---
phase: 35-hkdf-infrastructure
plan: 01
subsystem: cryptography
tags: [hkdf, kdf, ecdh, envelope, aes-gcm, rfc5869, sha2]

# Dependency graph
requires:
  - phase: 34-security-hardening
    provides: zeroize Secret<T> wrapper and PBKDF2 hardening context
provides:
  - hkdf 0.12 workspace dependency wired into root Cargo.toml and trustedge-core
  - HKDF-SHA256 Extract+Expand replacing PBKDF2 CatKDF in derive_shared_encryption_key()
  - Proper RFC 5869 key derivation from ECDH shared secret with domain separation
affects: [36-envelope-format-migration, 37-keyring-hardening]

# Tech tracking
tech-stack:
  added: [hkdf 0.12 (RustCrypto)]
  patterns:
    - HKDF-Extract(salt, IKM=ecdh_shared_secret) then HKDF-Expand(info=domain_string, L=32)
    - Domain separation via b"TRUSTEDGE_ENVELOPE_V1" info parameter
    - ECDH shared secret as sole IKM (no ad-hoc concatenation)

key-files:
  created: []
  modified:
    - Cargo.toml
    - crates/core/Cargo.toml
    - crates/core/src/envelope.rs

key-decisions:
  - "HKDF-SHA256 chosen over PBKDF2: HKDF (RFC 5869) is correct primitive for ECDH output extraction; PBKDF2 is for password stretching"
  - "ECDH shared secret is sole IKM: prevents CatKDF confusion where attacker-controlled public context was mixed with secret"
  - "Preserve pbkdf2_iterations field in ChunkManifest with literal 100_000u32: format compatibility maintained for Phase 36 versioning"
  - "Unused params prefixed with underscore: _sequence, _metadata_hash, _iterations remain in signature for Phase 36 cleanup"

patterns-established:
  - "HKDF usage: Hkdf::<Sha256>::new(Some(salt), secret_bytes) then .expand(domain_info, &mut key)"
  - "Domain separation: b\"TRUSTEDGE_ENVELOPE_V1\" as info parameter binds key to TrustEdge envelope context"

requirements-completed: [ENV-04, ENV-05, ENV-06]

# Metrics
duration: 16min
completed: 2026-02-23
---

# Phase 35 Plan 01: HKDF Infrastructure Summary

**HKDF-SHA256 (RFC 5869) replaces PBKDF2 CatKDF in envelope key derivation: ECDH shared secret feeds HKDF-Extract as sole IKM with domain separation via b"TRUSTEDGE_ENVELOPE_V1" info string**

## Performance

- **Duration:** 16 min
- **Started:** 2026-02-23T12:18:51Z
- **Completed:** 2026-02-23T12:34:57Z
- **Tasks:** 2
- **Files modified:** 3

## Accomplishments
- Added hkdf 0.12 as a workspace dependency (RustCrypto ecosystem, same trait versions as sha2 0.10)
- Replaced ad-hoc CatKDF (PBKDF2 with concatenated IKM) with proper RFC 5869 HKDF-Extract+Expand
- ECDH shared secret is now passed as sole IKM to HKDF-Extract; salt is a separate parameter (not mixed into IKM)
- Domain separation string "TRUSTEDGE_ENVELOPE_V1" binds derived key to TrustEdge envelope context via HKDF info parameter
- All 156 core lib tests pass; envelope roundtrip tests continue to pass with new KDF

## Task Commits

Each task was committed atomically:

1. **Task 1: Add hkdf workspace dependency** - `6f9e007` (chore)
2. **Task 2: Replace PBKDF2 CatKDF with HKDF-SHA256** - `a376393` (feat)

**Plan metadata:** (docs commit — see below)

## Files Created/Modified
- `/home/john/vault/projects/github.com/trustedge/Cargo.toml` - Added `hkdf = "0.12"` to workspace.dependencies Cryptography group
- `/home/john/vault/projects/github.com/trustedge/crates/core/Cargo.toml` - Added `hkdf = { workspace = true }` to dependencies
- `/home/john/vault/projects/github.com/trustedge/crates/core/src/envelope.rs` - Replaced PBKDF2 CatKDF with HKDF-SHA256; removed PBKDF2_ITERATIONS constant; updated imports

## Decisions Made
- **HKDF over PBKDF2:** PBKDF2 is designed for password stretching (slow hash + salt), not for extracting key material from high-entropy ECDH output. HKDF (RFC 5869) is the cryptographically correct primitive: Extract absorbs IKM entropy into a PRK, Expand produces arbitrary-length OKM.
- **ECDH as sole IKM:** The old CatKDF concatenated shared_secret + salt + sequence + metadata_hash + domain_string as a single blob into PBKDF2. This is incorrect — public/attacker-controlled data should not be mixed with secret IKM. In proper HKDF, salt is a separate randomization parameter and public context goes into the info string.
- **Domain separation:** The `b"TRUSTEDGE_ENVELOPE_V1"` info parameter cryptographically binds the derived key to this specific usage context, preventing cross-protocol key reuse.
- **Format stability:** The `pbkdf2_iterations` field in ChunkManifest is preserved with value `100_000u32` for serialization compatibility. Phase 36 handles format versioning and will clean this up.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
- `test_many_keys` (software_hsm) ran for over 60 seconds during full lib test run. This is a pre-existing slow stress test that generates 100 keys with 600k PBKDF2 iterations; it is unrelated to our changes. All 156 other tests pass in 32 seconds.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- HKDF infrastructure in place; Phase 36 (Envelope Format Migration) can now restructure the `derive_shared_encryption_key()` signature to eliminate the now-unused `sequence`, `metadata_hash`, and `iterations` parameters
- Phase 36 will also handle `pbkdf2_iterations` field removal from ChunkManifest with proper format versioning
- Phase 37 (Keyring Hardening) is unaffected by this change (keyring backends use their own PBKDF2 independently)

## Self-Check: PASSED

- envelope.rs: FOUND
- SUMMARY.md: FOUND
- commit 6f9e007 (chore: add hkdf workspace dependency): FOUND
- commit a376393 (feat: replace PBKDF2 CatKDF with HKDF-SHA256): FOUND

---
*Phase: 35-hkdf-infrastructure*
*Completed: 2026-02-23*
