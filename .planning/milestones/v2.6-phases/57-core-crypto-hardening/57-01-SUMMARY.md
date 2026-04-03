<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


---
phase: 57-core-crypto-hardening
plan: 01
subsystem: crypto
tags: [zeroize, pbkdf2, key-material, memory-safety, rust]

# Dependency graph
requires: []
provides:
  - PrivateKey with Zeroize derive and manual Drop impl that zeroizes key_bytes
  - ClientAuthResult with Zeroize derive and manual Drop impl that zeroizes session_key
  - SessionInfo with Zeroize derive and manual Drop impl that zeroizes session_key
  - SymmetricKey with Zeroize derive and manual Drop impl that zeroizes inner [u8;32]
  - import_secret_encrypted() rejects key files with < 600,000 PBKDF2 iterations
affects: [all phases using PrivateKey, SessionInfo, ClientAuthResult, SymmetricKey]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Zeroize derive + manual Drop impl for structs that Clone (cannot use ZeroizeOnDrop)"
    - "#[zeroize(skip)] on non-key fields whose types don't implement Zeroize"
    - "PBKDF2_MIN_ITERATIONS guard immediately after iterations parsing in import path"

key-files:
  created: []
  modified:
    - crates/core/src/asymmetric.rs
    - crates/core/src/auth.rs
    - crates/core/src/hybrid.rs
    - crates/core/src/crypto.rs

key-decisions:
  - "Use #[derive(Zeroize)] + manual impl Drop (not ZeroizeOnDrop) — avoids conflict with Clone derive (decision D-01)"
  - "Skip non-key fields with #[zeroize(skip)] when their types (AsymmetricAlgorithm, ServerCertificate) don't implement Zeroize"
  - "Place PBKDF2 iteration count guard immediately after parsing, before nonce length check — fail early on weak key files"
  - "Use result.err().unwrap() in test to extract CryptoError without requiring T: Debug on DeviceKeypair"

patterns-established:
  - "Zeroize derive + manual Drop: preferred pattern for key-holding structs that also need Clone"
  - "PBKDF2 iteration guard: all import paths must check >= PBKDF2_MIN_ITERATIONS before using iterations"

requirements-completed: [CORE-01, CORE-02]

# Metrics
duration: 34min
completed: 2026-03-24
---

# Phase 57 Plan 01: Core Crypto Hardening Summary

**Zeroize-on-drop added to four key-holding structs (PrivateKey, ClientAuthResult, SessionInfo, SymmetricKey) and 600k PBKDF2 minimum enforced at import boundary in import_secret_encrypted()**

## Performance

- **Duration:** 34 min
- **Started:** 2026-03-24T01:02:37Z
- **Completed:** 2026-03-24T01:36:52Z
- **Tasks:** 2
- **Files modified:** 4

## Accomplishments

- Four key-holding structs now zeroize their key material on drop: `PrivateKey.key_bytes`, `ClientAuthResult.session_key`, `SessionInfo.session_key`, `SymmetricKey.0`
- `import_secret_encrypted()` now rejects key files with fewer than 600,000 PBKDF2 iterations with a clear error message including the actual iteration count
- New test `test_encrypted_key_rejects_low_iterations` constructs a synthetic low-iteration key file and verifies rejection
- All 184 trustedge-core lib tests pass (183 pre-existing + 1 new)

## Task Commits

Each task was committed atomically:

1. **Task 1: Add Zeroize and Drop to four key-holding structs** - `daa1b11` (feat)
2. **Task 2: Enforce 600k PBKDF2 minimum in import_secret_encrypted** - `5a9272f` (feat)
3. **Format fix for crypto.rs test code** - `fa2ee74` (chore)

**Plan metadata:** (see final metadata commit)

## Files Created/Modified

- `crates/core/src/asymmetric.rs` - Added `use zeroize::Zeroize`, `#[derive(Zeroize)]` + `#[zeroize(skip)]` on non-key fields, `impl Drop for PrivateKey`
- `crates/core/src/auth.rs` - Added `#[derive(Zeroize)]` + `#[zeroize(skip)]` on non-key fields and `impl Drop` for both `ClientAuthResult` and `SessionInfo`
- `crates/core/src/hybrid.rs` - Added `use zeroize::Zeroize`, `#[derive(Zeroize)]` on `SymmetricKey`, `impl Drop for SymmetricKey`
- `crates/core/src/crypto.rs` - Added `iterations < PBKDF2_MIN_ITERATIONS` guard + new test `test_encrypted_key_rejects_low_iterations`

## Decisions Made

- **D-01 Zeroize pattern**: Used `#[derive(Zeroize)]` + manual `impl Drop` (not `ZeroizeOnDrop`) for all four structs, matching the existing `DeviceKeypair` pattern and avoiding conflict with the `Clone` derive that `ZeroizeOnDrop` cannot coexist with.
- **D-02 Field skipping**: Applied `#[zeroize(skip)]` to all non-key fields (`algorithm`, `key_id`, `session_id`, `client_public_key`, `client_identity`, `created_at`, `expires_at`, `authenticated`, `server_certificate`) because their types don't implement `Zeroize`. Only key material fields are zeroized.
- **D-03 Test error extraction**: Used `result.err().unwrap()` instead of `result.unwrap_err()` to extract `CryptoError` in the new test — `DeviceKeypair` doesn't implement `Debug`, which `unwrap_err()` and `expect_err()` both require.

## Deviations from Plan

None — plan executed exactly as written. The only minor deviation was using `result.err().unwrap()` in the test code instead of `result.unwrap_err()` as shown in the plan, because `DeviceKeypair` doesn't implement `Debug`. This is Rule 1 (auto-fix bug) — the plan's code wouldn't compile.

### Auto-fixed Issues

**1. [Rule 1 - Bug] Fixed test_encrypted_key_rejects_low_iterations to compile without T: Debug**
- **Found during:** Task 2 (PBKDF2 enforcement and test)
- **Issue:** Plan's test used `result.unwrap_err()` and `result.expect_err()` which require `T: Debug`. `DeviceKeypair` does not implement `Debug` (intentional — redacts key material).
- **Fix:** Changed to `result.err().unwrap()` — extracts the error via `Option<E>`, no `T: Debug` bound.
- **Files modified:** crates/core/src/crypto.rs
- **Verification:** `cargo test -p trustedge-core --lib test_encrypted_key` — 5 tests pass
- **Committed in:** `fa2ee74` (format fix commit)

---

**Total deviations:** 1 auto-fixed (Rule 1 compile error)
**Impact on plan:** Trivial fix, no behavior change, no scope creep.

## Issues Encountered

None beyond the compile error documented above.

## Known Stubs

None — all changes are complete and functional.

## User Setup Required

None — no external service configuration required.

## Next Phase Readiness

- CORE-01 and CORE-02 requirements satisfied
- Key-holding structs now memory-safe at drop time
- import_secret_encrypted enforces minimum at import boundary
- Ready for remaining Phase 57 plans (OrgContext fix, key logging, CORS, TLS, dashboard API key)

---
*Phase: 57-core-crypto-hardening*
*Completed: 2026-03-24*
