---
phase: 44-yubikey-cli-integration
plan: 01
subsystem: crypto
tags: [p256, ecdsa, ed25519, signature-verification, trst, archive]

# Dependency graph
requires:
  - phase: 43-trst-encryption
    provides: verify_manifest() in crypto.rs, trst verify CLI
provides:
  - verify_manifest() dispatches on ed25519: and ecdsa-p256: prefixes
  - ECDSA P-256 archive verification end-to-end via trst verify CLI
  - Acceptance tests for both valid and invalid P-256 signatures
affects: [44-02-yubikey-wrap, platform-verify]

# Tech tracking
tech-stack:
  added: []  # p256 crate already present; only ecdsa Signer/Verifier trait usage added
  patterns:
    - "Signature algorithm dispatch: prefix-based (ed25519: vs ecdsa-p256:) in verify_manifest()"
    - "P-256 public keys encoded as SEC1 uncompressed bytes in ecdsa-p256:<base64> format"
    - "P-256 signatures encoded as DER in ecdsa-p256:<base64> format"
    - "p256::ecdsa::SigningKey::sign() hashes with SHA-256 internally (no pre-hashing)"

key-files:
  created: []
  modified:
    - crates/core/src/crypto.rs
    - crates/trst-cli/src/main.rs
    - crates/trst-cli/tests/acceptance.rs
    - crates/trst-cli/Cargo.toml

key-decisions:
  - "ECDSA P-256 public key format: ecdsa-p256:<base64_sec1_uncompressed> (65 bytes uncompressed)"
  - "ECDSA P-256 signature format: ecdsa-p256:<base64_der> (DER-encoded ECDSA signature)"
  - "p256 crate Signer/Verifier trait handles SHA-256 hashing internally; pass canonical_bytes directly"
  - "Bare keys without prefix in --device-pub are assumed to be Ed25519 (backward compat)"
  - "resign_manifest_p256() must set device.public_key BEFORE calling to_canonical_bytes()"

patterns-established:
  - "Multi-algorithm signature dispatch: check signature prefix first, then validate key/sig pair"
  - "Algorithm mismatch (ed25519: sig + ecdsa-p256: key) returns Err(InvalidSignatureFormat)"

requirements-completed: [YUBI-02]

# Metrics
duration: 21min
completed: 2026-03-18
---

# Phase 44 Plan 01: ECDSA P-256 Multi-Algorithm Signature Verification Summary

**ECDSA P-256 signature verification added to verify_manifest() with prefix dispatch and end-to-end trst verify CLI acceptance tests**

## Performance

- **Duration:** 21 min
- **Started:** 2026-03-18T00:19:07Z
- **Completed:** 2026-03-18T00:40:14Z
- **Tasks:** 2
- **Files modified:** 4

## Accomplishments

- verify_manifest() now dispatches on "ed25519:" vs "ecdsa-p256:" signature prefix, returning Err for unknown algorithms
- ECDSA P-256 verification using p256::ecdsa::VerifyingKey with SEC1 public keys and DER-encoded signatures
- Two acceptance tests: valid P-256 archive (exit 0) and wrong P-256 key (exit 10 + "Signature verification failed")
- trst verify --device-pub now passes through ecdsa-p256: prefixed keys without adding ed25519: prefix
- All 17 crypto unit tests and 26 acceptance tests pass; clippy --workspace -D warnings clean

## Task Commits

1. **Task 1: Extend verify_manifest() for ECDSA P-256 dispatch** - `34fd51f` (feat)
2. **Task 2: Add ECDSA P-256 acceptance test for trst verify** - `053a54e` (feat)

**Plan metadata:** (pending)

_Note: TDD tasks had RED (failing tests added first), then GREEN (implementation), then REFACTOR (clippy strip_prefix fixes)_

## Files Created/Modified

- `crates/core/src/crypto.rs` - Added verify_manifest_ecdsa_p256(), refactored verify_manifest() with algorithm dispatch, 4 new unit tests
- `crates/trst-cli/src/main.rs` - Updated handle_verify() prefix logic and VerifyCmd help text
- `crates/trst-cli/tests/acceptance.rs` - Added acceptance_verify_ecdsa_p256, acceptance_verify_ecdsa_p256_wrong_key, resign_manifest_p256() helper
- `crates/trst-cli/Cargo.toml` - Added p256 to [dev-dependencies]

## Decisions Made

- ECDSA P-256 public key format: "ecdsa-p256:<base64_sec1_uncompressed>" (65 bytes, 0x04 prefix)
- ECDSA P-256 signature format: "ecdsa-p256:<base64_der>" (variable-length DER encoding)
- p256 crate Signer/Verifier handles SHA-256 hashing internally; canonical_bytes passed directly (not pre-hashed)
- Bare base64 keys to --device-pub default to ed25519: for backward compatibility

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Fixed resign_manifest_p256() key ordering bug**
- **Found during:** Task 2 (acceptance test implementation)
- **Issue:** resign_manifest_p256() computed canonical_bytes BEFORE setting manifest.device.public_key to the P-256 key, so the signed bytes included the old Ed25519 public_key but the manifest on disk had the P-256 key. Verification failed because the on-disk manifest produced different canonical bytes.
- **Fix:** Moved manifest.device.public_key update to before to_canonical_bytes() call in the helper function
- **Files modified:** crates/trst-cli/tests/acceptance.rs
- **Verification:** acceptance_verify_ecdsa_p256 passes with exit 0
- **Committed in:** 053a54e (Task 2 commit)

---

**Total deviations:** 1 auto-fixed (Rule 1 - Bug)
**Impact on plan:** Single logic ordering bug in test helper. No scope creep, no architectural changes.

## Issues Encountered

None beyond the auto-fixed bug above.

## Next Phase Readiness

- verify_manifest() is ready to accept P-256 signatures from YubiKey hardware
- Plan 02 can now wire YubiKey ECDSA P-256 signing into trst wrap
- Signature format is established: ecdsa-p256:<base64_sec1_uncompressed> for keys, ecdsa-p256:<base64_der> for signatures

---
*Phase: 44-yubikey-cli-integration*
*Completed: 2026-03-18*
