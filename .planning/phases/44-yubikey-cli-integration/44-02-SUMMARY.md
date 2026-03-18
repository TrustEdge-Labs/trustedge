---
phase: 44-yubikey-cli-integration
plan: 02
subsystem: cli
tags: [yubikey, ecdsa-p256, rpassword, trst-cli, demo, acceptance-tests]

# Dependency graph
requires:
  - phase: 44-01
    provides: verify_manifest ECDSA P-256 support and ecdsa-p256 key/sig format

provides:
  - --backend software|yubikey flag on trst wrap command
  - Interactive PIN prompt via rpassword for YubiKey signing
  - ECDSA P-256 hardware signing via UniversalBackend in trst wrap
  - pub_key_to_device_id() helper for both ed25519 and ecdsa-p256 prefixes
  - demo.sh YubiKey auto-detection with optional hardware signing step
  - acceptance_backend_software_explicit regression test
  - acceptance_backend_unknown_fails error message test

affects: [45-any-future-trst-cli, demo-scripts]

# Tech tracking
tech-stack:
  added:
    - rpassword = "7" (interactive PIN prompt, no echo)
    - p256 (promoted from dev-dependencies to main dependencies)
    - yubikey feature flag on trustedge-trst-cli
    - p256::pkcs8::DecodePublicKey (SPKI DER parsing)
  patterns:
    - Feature-gated hardware path with clear runtime error when feature disabled
    - pub_key_to_device_id() strips prefix then takes first 6 bytes for hex device ID
    - YubiKey backend validation (--device-key required for chunk encryption) before keypair load

key-files:
  created: []
  modified:
    - crates/trst-cli/src/main.rs
    - crates/trst-cli/Cargo.toml
    - crates/trst-cli/tests/acceptance.rs
    - scripts/demo.sh

key-decisions:
  - "--device-key required with --backend yubikey because YubiKey cannot do symmetric encryption; software key still handles HKDF chunk encryption"
  - "pub_key_to_device_id() decodes base64 after prefix and uses first 6 raw key bytes regardless of algorithm"
  - "YubiKey step in demo.sh is auto-detected (ykman list), not controlled by --local flag; hardware presence determines whether step runs"
  - "p256 promoted to main dependencies (not just dev-deps) for use in yubikey signing path"

patterns-established:
  - "Backend dispatch pattern: match args.backend.as_str() with cfg-gated yubikey block and cfg(not) fallback bail"
  - "Device ID generation: prefix-agnostic helper fn supporting both ed25519: and ecdsa-p256: formats"
  - "Demo script YubiKey detection: ykman list | grep YubiKey, wrapped in command -v ykman guard"

requirements-completed: [YUBI-01, YUBI-03, YUBI-04]

# Metrics
duration: 20min
completed: 2026-03-18
---

# Phase 44 Plan 02: YubiKey CLI Integration Summary

**trst wrap --backend yubikey wires ECDSA P-256 hardware signing with interactive rpassword PIN prompt; demo.sh auto-detects YubiKey via ykman**

## Performance

- **Duration:** ~20 min
- **Started:** 2026-03-18T00:43:00Z
- **Completed:** 2026-03-18T01:03:00Z
- **Tasks:** 2
- **Files modified:** 4

## Accomplishments

- Added `--backend software|yubikey` and `--slot` flags to `trst wrap`; software remains backward-compatible default
- YubiKey signing path: rpassword PIN prompt, UniversalBackend ECDSA P-256 sign, SPKI DER -> SEC1 public key extraction
- Fixed device_id generation to be prefix-agnostic (both `ed25519:` and `ecdsa-p256:` formats)
- demo.sh detects YubiKey via `ykman list` and adds optional hardware signing step; friendly skip message when absent
- 2 new acceptance tests: `acceptance_backend_software_explicit` (ed25519 regression) and `acceptance_backend_unknown_fails` (error message)
- All 28 acceptance tests pass; both builds (with and without yubikey feature) compile cleanly

## Task Commits

1. **Task 1: Add --backend yubikey flag and PIN prompt to trst wrap** - `b36b7cb` (feat)
2. **Task 2: Update demo.sh with YubiKey auto-detection and acceptance tests** - `1b980de` (feat)

## Files Created/Modified

- `crates/trst-cli/src/main.rs` - Added backend dispatch, YubiKey signing path, pub_key_to_device_id() helper, --backend/--slot flags
- `crates/trst-cli/Cargo.toml` - Added yubikey feature, rpassword dep, p256 promoted from dev-deps
- `crates/trst-cli/tests/acceptance.rs` - Added acceptance_backend_software_explicit and acceptance_backend_unknown_fails
- `scripts/demo.sh` - YubiKey auto-detection, optional hardware signing step, dynamic TOTAL_STEPS

## Decisions Made

- `--device-key` is required with `--backend yubikey` because YubiKey PIV cannot perform symmetric crypto; the software key is still used for HKDF-based chunk encryption
- `pub_key_to_device_id()` extracts prefix, decodes base64, uses first 6 bytes - works for both key types without algorithm-specific logic
- YubiKey demo step is controlled by hardware auto-detection only, not by `--local` flag (demo.sh always skips server verification in local mode but YubiKey presence determines hardware step)

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Added `base64::Engine as _` import for base64 decode calls**
- **Found during:** Task 1 (building after implementation)
- **Issue:** `base64::engine::general_purpose::STANDARD.decode()` requires `Engine` trait in scope
- **Fix:** Added `use base64::Engine as _;` import
- **Files modified:** `crates/trst-cli/src/main.rs`
- **Verification:** `cargo build -p trustedge-trst-cli` compiles cleanly
- **Committed in:** `b36b7cb` (Task 1 commit, after cargo fmt)

**2. [Rule 3 - Blocking] Added `p256::pkcs8::DecodePublicKey` import (cfg-gated)**
- **Found during:** Task 1 (yubikey feature build)
- **Issue:** `p256::PublicKey::from_public_key_der()` requires `DecodePublicKey` trait in scope
- **Fix:** Added `#[cfg(feature = "yubikey")] use p256::pkcs8::DecodePublicKey;`
- **Files modified:** `crates/trst-cli/src/main.rs`
- **Verification:** `cargo build -p trustedge-trst-cli --features yubikey` compiles cleanly
- **Committed in:** `b36b7cb` (Task 1 commit, after cargo fmt)

---

**Total deviations:** 2 auto-fixed (both Rule 3 - blocking imports)
**Impact on plan:** Both fixes were compile errors from missing trait imports. No scope creep.

## Issues Encountered

- cargo fmt reordered the `#[cfg(feature = "yubikey")]` use blocks (DecodePublicKey moved before the core backends imports) — pre-commit hook caught it, ran fmt and recommitted cleanly.

## User Setup Required

None - no external service configuration required. YubiKey hardware tests remain hardware-gated (`#[ignore]` pattern).

## Next Phase Readiness

- Phase 44 complete: ECDSA P-256 verify (Plan 01) + YubiKey CLI integration (Plan 02) are both done
- Users can run `trst wrap --backend yubikey --device-key <key> --in <file> --out <archive>` with yubikey feature enabled
- demo.sh automatically includes the hardware signing step when a YubiKey 5 series is present

---
*Phase: 44-yubikey-cli-integration*
*Completed: 2026-03-18*
