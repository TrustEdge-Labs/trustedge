---
phase: 31-secret-hardening
plan: 02
subsystem: backends
tags: [secret, zeroize, security, yubikey, software-hsm, config-hardening]

# Dependency graph
requires:
  - phase: 31-01
    provides: "Secret<T> wrapper type with zeroize, redacted Debug, expose_secret()"
provides:
  - "YubiKeyConfig hardened: no Serialize/Deserialize, pin is Secret<String>, builder pattern, redacted Debug"
  - "SoftwareHsmConfig hardened: no Serialize/Deserialize, default_passphrase is Secret<String>, builder pattern, redacted Debug"
  - "Builder APIs: YubiKeyConfig::builder() and SoftwareHsmConfig::builder()"
  - "All call sites updated: integration tests, examples, demo binary, platform CA service"
affects: [yubikey-backend, software-hsm, platform-ca, trust-boundary]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Config structs holding secrets: no serde derives, private secret fields, builder-only construction"
    - "Debug redaction via manual fmt::Debug impl using f.debug_struct().field(name, &'[REDACTED]')"
    - "Secret access: expose_secret() called only at the usage site, never stored or logged"

key-files:
  created: []
  modified:
    - crates/core/src/backends/yubikey.rs
    - crates/core/src/backends/software_hsm.rs
    - crates/core/src/bin/software-hsm-demo.rs
    - crates/core/tests/software_hsm_integration.rs
    - crates/core/tests/yubikey_integration.rs
    - crates/core/examples/verify_yubikey.rs
    - crates/core/examples/verify_yubikey_custom_pin.rs
    - crates/platform/src/ca/service.rs

key-decisions:
  - "Builder pattern chosen over public struct fields — prevents accidental bypass of Secret<T> wrapping"
  - "pin() and default_passphrase() getters return &str via expose_secret() — minimal exposure surface"
  - "YubiKeySigningKeyPair.pin kept as Option<String> (extracted once for use with rcgen) — not stored across calls"
  - "platform/ca/service.rs stale fields (pkcs11_module_path, slot) removed and replaced with correct builder API"

patterns-established:
  - "Config hardening pattern: remove serde, private secret fields, builder constructor, manual Debug with [REDACTED]"
  - "Secret getter pattern: pub fn field_name(&self) -> &str returning expose_secret().as_str()"

requirements-completed: [SEC-01, SEC-02, SEC-03]

# Metrics
duration: 8min
completed: 2026-02-22
---

# Phase 31 Plan 02: Config Secret Hardening Summary

**YubiKeyConfig and SoftwareHsmConfig hardened with Secret<String> fields, redacted Debug, builder pattern, and no Serialize/Deserialize — compiler blocks accidental PIN/passphrase serialization**

## Performance

- **Duration:** 8 min
- **Started:** 2026-02-22T17:36:22Z
- **Completed:** 2026-02-22T17:44:18Z
- **Tasks:** 2
- **Files modified:** 8

## Accomplishments
- YubiKeyConfig.pin is now `Option<Secret<String>>` (private field), accessible only via `pin()` getter
- SoftwareHsmConfig.default_passphrase is now `Secret<String>` (private field), accessible only via `default_passphrase()` getter
- Both structs have manual Debug impls that print `[REDACTED]` for secret fields
- Both structs removed `#[derive(Serialize, Deserialize)]` — serialization attempts are compile errors
- Builder APIs added for both configs with fluent methods
- All call sites updated across 8 files: tests, examples, demo binary, platform CA service
- Fixed stale platform/ca/service.rs which referenced non-existent fields `pkcs11_module_path` and `slot`
- 6 new security tests added (3 per struct: debug redaction, builder sets fields, builder defaults)

## Task Commits

Each task was committed atomically:

1. **Task 1: Harden YubiKeyConfig** - `78ba192` (feat)
2. **Task 2: Harden SoftwareHsmConfig** - `b5882f2` (feat)

## Files Created/Modified
- `crates/core/src/backends/yubikey.rs` - YubiKeyConfig hardened, builder added, all .pin field accesses updated
- `crates/core/src/backends/software_hsm.rs` - SoftwareHsmConfig hardened, builder added, all .default_passphrase accesses updated
- `crates/core/src/bin/software-hsm-demo.rs` - Config construction updated to builder pattern
- `crates/core/tests/software_hsm_integration.rs` - create_test_hsm_setup + readonly_config updated to builder
- `crates/core/tests/yubikey_integration.rs` - create_test_config + wrong_pin test updated to builder
- `crates/core/examples/verify_yubikey.rs` - Config construction updated to builder pattern
- `crates/core/examples/verify_yubikey_custom_pin.rs` - Config construction updated to builder (handles None pin via conditional)
- `crates/platform/src/ca/service.rs` - Removed non-existent fields, updated to correct builder API

## Decisions Made
- Builder pattern chosen over public struct fields to prevent accidental bypass of `Secret<T>` wrapping
- `pin()` and `default_passphrase()` getters return `&str` via `expose_secret()` — minimal exposure surface, caller responsible for not logging
- `YubiKeySigningKeyPair.pin` kept as `Option<String>` (extracted once at construction via `pin()`) — rcgen requires owned value, not stored beyond the signing call
- Stale `pkcs11_module_path` and `slot` fields in platform CA service removed as part of fixing this function (Rule 1 - Bug)

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Fixed stale fields in platform/ca/service.rs create_yubikey_ca_service()**
- **Found during:** Task 1 (Harden YubiKeyConfig)
- **Issue:** Function used `pkcs11_module_path` and `slot` fields that never existed on `YubiKeyConfig` — stale code from platform consolidation era
- **Fix:** Removed both non-existent fields, converted to builder pattern with correct fields (`.pin()`, `.default_slot()`, `.verbose()`)
- **Files modified:** `crates/platform/src/ca/service.rs`
- **Verification:** `cargo build -p trustedge-platform --features "ca,yubikey"` passes
- **Committed in:** `78ba192` (Task 1 commit)

---

**Total deviations:** 1 auto-fixed (Rule 1 - Bug)
**Impact on plan:** Fix was necessary for correctness. The plan explicitly mentioned this file needed updating.

## Issues Encountered
None — all tasks completed cleanly.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Secret hardening for config structs complete (SEC-01, SEC-02, SEC-03)
- Both backend configs now prevent accidental serialization of secret material
- Zeroization on drop confirmed via Secret<T> which uses ZeroizeOnDrop
- Ready for Phase 32 (next phase in v1.7 roadmap)

---
*Phase: 31-secret-hardening*
*Completed: 2026-02-22*
