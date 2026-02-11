---
phase: 09-cleanup
plan: 01
subsystem: infrastructure
tags: [yubikey, cleanup, technical-debt, security]

# Dependency graph
requires:
  - phase: 01-foundation through 08-validation
    provides: v1.0 consolidation complete
provides:
  - Deleted 3,263-line broken YubiKey backend implementation
  - Deleted 8 YubiKey test files (hardware detection, integration, simulation)
  - Deleted 8 YubiKey example files and 1 demo binary
  - Removed all placeholder keys and manual DER encoding patterns
  - Clean slate for v1.1 YubiKey rewrite
affects: [10-backend-rewrite, 11-transport-integration, 12-validation]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "yubikey dependency preserved (version 0.7, optional) without 'untested' feature flag"
    - "yubikey feature flag preserved for v1.1 reuse"
    - "Zero placeholder keys or manual DER encoding in backends/transport"

key-files:
  created: []
  modified:
    - crates/core/src/backends/mod.rs
    - crates/core/src/backends/universal_registry.rs
    - crates/core/src/transport/quic.rs
    - crates/core/src/lib.rs
    - crates/core/Cargo.toml

key-decisions:
  - "Scorched-earth deletion: removed entire YubiKey implementation (backend, tests, examples) rather than incremental fixes"
  - "Preserved yubikey dependency and feature flag for v1.1 rewrite"
  - "Removed 'untested' feature flag from yubikey dependency (use only stable API in v1.1)"
  - "Updated docs to mark yubikey as experimental/v1.1 rewrite in progress"

patterns-established:
  - "CLEAN-04 pattern: zero placeholder keys, zero manual DER encoding in backends/transport"
  - "Feature flag preservation: keep dependency and feature for future use even when implementation deleted"

# Metrics
duration: 5min
completed: 2026-02-11
---

# Phase 9 Plan 1: YubiKey Cleanup Summary

**Deleted 8,117 lines of broken YubiKey code: 3,263-line backend, 8 tests, 8 examples, 1 binary, all placeholder keys and manual DER encoding**

## Performance

- **Duration:** 5 minutes
- **Started:** 2026-02-11T23:37:26Z
- **Completed:** 2026-02-11T23:42:49Z
- **Tasks:** 2
- **Files modified:** 23 (18 deleted, 5 edited)

## Accomplishments
- Deleted entire broken YubiKey implementation (3,263-line backend, 8 test files, 8 example files, 1 demo binary)
- Removed all placeholder keys (hardcoded Ed25519, ECDSA P-256 keys) from quic.rs
- Removed all manual DER encoding patterns (to_der, encode_asn1, build_tbs, etc.)
- Module system compiles cleanly with no orphaned imports or references
- Full workspace tests pass (343 tests)

## Task Commits

Each task was committed atomically:

1. **Task 1: Delete YubiKey files and update module references** - `3e81f5b` (chore)
2. **Task 2: Clean quic.rs YubiKey code and run full verification** - `e890999` (chore)

## Files Created/Modified

**Deleted (18 files, 8,117 lines):**
- `crates/core/src/backends/yubikey.rs` - 3,263-line backend implementation
- `crates/core/tests/yubikey_certificate_debug.rs` - Certificate debug tests
- `crates/core/tests/yubikey_hardware_detection.rs` - Hardware detection tests
- `crates/core/tests/yubikey_hardware_tests.rs` - Hardware integration tests
- `crates/core/tests/yubikey_integration.rs` - Integration test suite
- `crates/core/tests/yubikey_piv_analysis.rs` - PIV applet analysis tests
- `crates/core/tests/yubikey_simulation_tests.rs` - Simulation tests
- `crates/core/tests/yubikey_strict_hardware.rs` - Strict hardware tests
- `crates/core/tests/yubikey_real_operations.rs` - Real hardware operation tests
- `crates/core/examples/yubikey_certificate_demo.rs` - Certificate demo
- `crates/core/examples/yubikey_demo.rs` - Basic demo
- `crates/core/examples/yubikey_enhanced_cert_demo.rs` - Enhanced certificate demo
- `crates/core/examples/yubikey_hardware_signing_demo.rs` - Hardware signing demo
- `crates/core/examples/yubikey_phase2c_demo.rs` - Phase 2c demo
- `crates/core/examples/yubikey_pubkey_demo.rs` - Public key demo
- `crates/core/examples/yubikey_quic_demo.rs` - QUIC integration demo
- `crates/core/examples/yubikey_quic_hardware_demo.rs` - QUIC hardware demo
- `crates/core/src/bin/yubikey-demo.rs` - Demo binary

**Modified (5 files):**
- `crates/core/src/backends/mod.rs` - Removed pub mod yubikey and YubiKey type exports
- `crates/core/src/backends/universal_registry.rs` - Removed YubiKeyBackend import and registration block
- `crates/core/src/transport/quic.rs` - Removed 325 lines of placeholder code (create_placeholder_private_key, create_demo_private_key, etc.)
- `crates/core/src/lib.rs` - Updated docs to mark yubikey as experimental/v1.1 rewrite
- `crates/core/Cargo.toml` - Removed 'untested' feature flag from yubikey dependency, removed yubikey-demo binary section

## Decisions Made

1. **Scorched-earth deletion over incremental fixes:** External review found critical security issues (placeholder keys, manual DER encoding, silent fallbacks). Safer to delete everything and rewrite from scratch in v1.1.

2. **Preserved yubikey dependency and feature flag:** Keep yubikey crate (version 0.7, optional) and feature flag definition for v1.1 reuse. Removed 'untested' feature flag to use only stable API.

3. **Updated documentation to mark as experimental:** lib.rs docs now state "YubiKey PIV support (experimental, v1.1 rewrite in progress)" to set expectations.

## Deviations from Plan

None - plan executed exactly as written. No bugs encountered, no blocking issues, no architectural changes needed.

## Issues Encountered

**Audio feature compilation:** `cargo check --features audio` failed due to missing ALSA system libraries (libasound2-dev). This is a system dependency issue, not a code issue. Default and yubikey features compile successfully.

## Verification Results

**CLEAN-01:** yubikey.rs backend deleted (confirmed: ls returns no result)

**CLEAN-02:** All 8 YubiKey test files deleted (confirmed: ls yubikey_*.rs returns 0 files)

**CLEAN-03:** 'untested' feature flag removed from Cargo.toml (confirmed: grep returns no hits)

**CLEAN-04:** Zero placeholder patterns in backends/transport:
- `grep "placeholder"` returns only legitimate hits (archive.rs manifest field, attestation git hash fallback)
- `grep "create_placeholder_private_key|create_demo_private_key|encode_asn1|build_tbs"` returns zero hits
- `grep "fake_key|dummy_key|placeholder_key"` returns zero hits
- `grep 'cfg(feature = "yubikey")'` in quic.rs returns zero hits

**Build verification:**
- `cargo check` passes (default, no features)
- `cargo check --features yubikey` passes (deps present, no impl)
- `cargo test --workspace` passes (343 tests, 0 failures)

## Next Phase Readiness

**Ready for Phase 10 (Backend Rewrite):**
- Clean slate: zero YubiKey code in codebase
- Dependencies preserved: yubikey crate and feature flag ready for v1.1
- Module system clean: no orphaned imports or references
- Test suite passes: baseline quality maintained

**Blockers:** None

**Notes for Phase 10:**
- rcgen custom signer API callback pattern needs investigation during planning
- PKCS#11 key attribute extraction may vary by YubiKey firmware version
- Hardware tests require physical YubiKey 5 series with PIV applet enabled
- PCSC daemon (pcscd) must be running for hardware operations

## Self-Check: PASSED

**Files deleted:**
- ✔ yubikey.rs deleted
- ✔ yubikey-demo.rs deleted
- ✔ All 8 test files deleted
- ✔ All 8 example files deleted

**Commits exist:**
- ✔ 3e81f5b (Task 1)
- ✔ e890999 (Task 2)

**Modified files exist:**
- ✔ backends/mod.rs
- ✔ backends/universal_registry.rs
- ✔ transport/quic.rs
- ✔ lib.rs
- ✔ Cargo.toml

---
*Phase: 09-cleanup*
*Completed: 2026-02-11*
