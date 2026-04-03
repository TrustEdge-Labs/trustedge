<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


---
phase: 47-key-protection-at-rest
plan: 01
subsystem: crypto
tags: [pbkdf2, aes-gcm, ed25519, key-encryption, passphrase]

# Dependency graph
requires: []
provides:
  - "DeviceKeypair::export_secret_encrypted() — passphrase-encrypted key export (TRUSTEDGE-KEY-V1 format)"
  - "DeviceKeypair::import_secret_encrypted() — passphrase-authenticated key import with wrong-passphrase rejection"
  - "is_encrypted_key_file() — detection function distinguishing encrypted from plaintext key files"
affects: [47-02, 47-03]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "TRUSTEDGE-KEY-V1 format: header line + JSON metadata line (salt/nonce/iterations) + ciphertext bytes"
    - "PBKDF2-SHA256 at 600k iterations for passphrase-based key derivation (matches keyring backend baseline)"
    - "AES-256-GCM for symmetric encryption of 32-byte Ed25519 secret keys"
    - "Derived key zeroized after use with zeroize::Zeroize"

key-files:
  created: []
  modified:
    - crates/core/src/crypto.rs
    - crates/core/src/lib.rs

key-decisions:
  - "Used existing CryptoError::EncryptionFailed and DecryptionFailed variants (not new EncryptionError/DecryptionError) to avoid error enum churn"
  - "AesGcmNonce::from_slice() used instead of AesNonce::<Aes256Gcm> — type parameter on AesNonce is the NonceSize not the cipher type"
  - "600k PBKDF2 iterations matches v1.8 keyring backend hardening (OWASP 2023 minimum 300k)"
  - "is_encrypted_key_file is a standalone function (not method) since it operates on raw bytes before a keypair exists"

patterns-established:
  - "TRUSTEDGE-KEY-V1 header: enables format detection before attempting decryption"

requirements-completed: [KEY-01]

# Metrics
duration: 17min
completed: 2026-03-19
---

# Phase 47 Plan 01: Encrypted Key File Support Summary

**PBKDF2-SHA256 (600k iterations) + AES-256-GCM encrypted key export/import with TRUSTEDGE-KEY-V1 magic header and wrong-passphrase rejection via AES-GCM authentication tag**

## Performance

- **Duration:** 17 min
- **Started:** 2026-03-19T01:51:10Z
- **Completed:** 2026-03-19T02:07:41Z
- **Tasks:** 1 (TDD: 2 commits — RED test + GREEN implementation)
- **Files modified:** 2

## Accomplishments
- `DeviceKeypair::export_secret_encrypted(passphrase)` encrypts the 32-byte Ed25519 secret key using PBKDF2-SHA256 (600k iterations, 32-byte random salt) to derive an AES-256-GCM key, then encrypts the secret; output is the TRUSTEDGE-KEY-V1 format (header + JSON metadata + ciphertext)
- `DeviceKeypair::import_secret_encrypted(data, passphrase)` decrypts and reconstructs the keypair; wrong passphrase returns `CryptoError::DecryptionFailed` (AES-GCM authentication tag failure — no garbage output possible)
- `is_encrypted_key_file(data)` detects the TRUSTEDGE-KEY-V1 header prefix for pre-flight format disambiguation
- All 173 `trustedge-core` lib tests pass; clippy clean; no regressions

## Task Commits

Each task was committed atomically:

1. **Task 1 RED: failing tests for encrypted key export/import** - `69720c4` (test)
2. **Task 1 GREEN: implement export_secret_encrypted, import_secret_encrypted, is_encrypted_key_file** - `02a2ebd` (feat)

## Files Created/Modified
- `crates/core/src/crypto.rs` - Added ENCRYPTED_KEY_HEADER const, export_secret_encrypted(), import_secret_encrypted() methods, is_encrypted_key_file() function, and 4 unit tests
- `crates/core/src/lib.rs` - Added is_encrypted_key_file to pub use re-exports

## Decisions Made
- Used existing `CryptoError::EncryptionFailed` and `DecryptionFailed` variants rather than adding new `EncryptionError`/`DecryptionError` variants as the plan suggested; the existing variants have identical semantics and adding variants would require updating match arms elsewhere
- Used `AesGcmNonce::from_slice()` (typed as `aes_gcm::Nonce`) instead of `AesNonce::<Aes256Gcm>` — the type parameter for `aes_gcm::Nonce<_>` is the nonce size (U12), not the cipher type
- 600k PBKDF2 iterations matches the v1.8 keyring hardening target (OWASP 2023); the plan specified 600k explicitly

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Used correct AesGcmNonce type (not AesNonce::<Aes256Gcm>)**
- **Found during:** Task 1 GREEN (compilation)
- **Issue:** Plan's code sample used `AesNonce::<Aes256Gcm>::from_slice()` which fails — the `Nonce<_>` type parameter is the nonce size (U12 via generic-array), not the cipher type
- **Fix:** Changed to `AesGcmNonce::from_slice(&nonce_bytes)` where `AesGcmNonce` is aliased from `aes_gcm::Nonce`
- **Files modified:** crates/core/src/crypto.rs
- **Verification:** Compilation succeeded; all 4 new tests pass
- **Committed in:** 02a2ebd

---

**Total deviations:** 1 auto-fixed (Rule 1 - type error in plan's code sample)
**Impact on plan:** Fix necessary for compilation. No scope creep.

## Issues Encountered
- `test_many_keys` (pre-existing software_hsm test, 100 key generations) runs slowly (~9 min) due to 600k-iteration PBKDF2 in the HSM backend — this is pre-existing behavior, unrelated to this plan's changes

## Next Phase Readiness
- `DeviceKeypair::export_secret_encrypted()` and `import_secret_encrypted()` are ready for use by the `trst keygen` CLI (Plan 47-02)
- `is_encrypted_key_file()` is re-exported from lib.rs for the CLI to detect file format before prompting for passphrase

---
*Phase: 47-key-protection-at-rest*
*Completed: 2026-03-19*
