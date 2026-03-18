---
phase: 45-rsa-oaep-migration
plan: 01
subsystem: crypto
tags: [rsa, oaep, sha256, padding, security, audit]

# Dependency graph
requires: []
provides:
  - RSA OAEP-SHA256 encrypt and decrypt in asymmetric.rs (replaces PKCS#1 v1.5)
  - Clean cargo-audit config without RUSTSEC-2023-0071 risk acceptance
affects: [46-kdf-hardening, 47-trst-key-encryption]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "RSA operations use Oaep::new::<sha2::Sha256>() for all encrypt/decrypt"

key-files:
  created: []
  modified:
    - crates/core/src/asymmetric.rs
    - .cargo/audit.toml

key-decisions:
  - "Migrated RSA encrypt/decrypt from PKCS#1 v1.5 (Pkcs1v15Encrypt) to OAEP-SHA256 (Oaep::new::<sha2::Sha256>()) to eliminate Marvin Attack timing sidechannel"
  - "Removed RUSTSEC-2023-0071 entirely from audit.toml ignore list — advisory not flagged by cargo-audit after OAEP migration (yubikey transitive dep rsa 0.7.2 not currently triggering advisory)"

patterns-established:
  - "OAEP-SHA256: all RSA key wrapping operations in TrustEdge use Oaep::new::<sha2::Sha256>(), never Pkcs1v15Encrypt"

requirements-completed: [RSA-01, RSA-02]

# Metrics
duration: 19min
completed: 2026-03-18
---

# Phase 45 Plan 01: RSA OAEP Migration Summary

**RSA PKCS#1 v1.5 padding replaced with OAEP-SHA256 in asymmetric.rs, eliminating the Marvin Attack timing sidechannel (RUSTSEC-2023-0071) from TrustEdge's direct RSA usage**

## Performance

- **Duration:** 19 min
- **Started:** 2026-03-18T01:32:03Z
- **Completed:** 2026-03-18T01:51:23Z
- **Tasks:** 2
- **Files modified:** 2

## Accomplishments
- Replaced `Pkcs1v15Encrypt` with `Oaep::new::<sha2::Sha256>()` in both `rsa_encrypt_key` and `rsa_decrypt_key` in `asymmetric.rs`
- RSA round-trip test (`test_rsa_key_encryption`) passes with OAEP padding; all 169 trustedge-core unit tests pass
- Removed RUSTSEC-2023-0071 risk acceptance from `.cargo/audit.toml` with a comment documenting the removal rationale

## Task Commits

Each task was committed atomically:

1. **Task 1: Replace PKCS#1 v1.5 with OAEP-SHA256 in RSA encrypt and decrypt** - `0a45a21` (fix)
2. **Task 2: Remove RUSTSEC-2023-0071 from cargo-audit risk-accepted list** - `1358948` (chore)

**Plan metadata:** (docs commit follows)

## Files Created/Modified
- `crates/core/src/asymmetric.rs` - `rsa_encrypt_key` and `rsa_decrypt_key` now use `Oaep::new::<sha2::Sha256>()` instead of `Pkcs1v15Encrypt`
- `.cargo/audit.toml` - RUSTSEC-2023-0071 removed from ignore list; comment added documenting removal rationale

## Decisions Made
- Used `Oaep::new::<sha2::Sha256>()` (OAEP with SHA-256 as both the hash and MGF hash) — standard, well-tested configuration provided by the `rsa` crate's `Oaep` type
- Removed RUSTSEC-2023-0071 entirely rather than scoping to yubikey-only rationale — cargo-audit does not currently flag the advisory for the yubikey transitive dep (rsa 0.7.2), so no ignore entry is needed at this time

## Deviations from Plan

None — plan executed exactly as written.

## Issues Encountered
- `test_many_keys` in the software HSM integration test suite takes ~8 minutes to complete (generates many keys iteratively). This is a pre-existing behavior, not related to this change.

## User Setup Required

None — no external service configuration required.

## Next Phase Readiness
- RSA OAEP migration complete; RUSTSEC-2023-0071 resolved for TrustEdge's direct usage
- Phase 46 (KDF Hardening) and Phase 47 (trst key encryption) can proceed
- If yubikey crate updates in the future cause RUSTSEC-2023-0071 to re-appear in cargo-audit, re-add with a yubikey-scoped rationale

---
*Phase: 45-rsa-oaep-migration*
*Completed: 2026-03-18*
