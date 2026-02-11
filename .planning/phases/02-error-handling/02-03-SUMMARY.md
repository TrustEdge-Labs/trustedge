<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


---
phase: 02-error-handling
plan: 03
subsystem: error-handling
tags: [thiserror, anyhow, backend-traits, structured-errors]

# Dependency graph
requires:
  - phase: 02-01
    provides: Unified error.rs with TrustEdgeError and 7 subsystem error enums
  - phase: 02-02
    provides: Module error migrations completed (crypto, chain, manifest, asymmetric, archive)
provides:
  - Backend traits return Result<T, BackendError> instead of anyhow::Result
  - Public API exposes TrustEdgeError, BackendError, TransportError
  - CLI binaries automatically convert BackendError to anyhow via ? operator
affects: [03-manifest-deduplication, yubikey-integration, backend-development]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - Backend trait methods use Result<T, BackendError> for structured error handling
    - CLI binaries use ? operator for automatic BackendError -> anyhow conversion
    - Error mapping: KeyNotFound, UnsupportedOperation, HardwareError, OperationFailed

key-files:
  created: []
  modified:
    - crates/core/src/backends/traits.rs
    - crates/core/src/backends/universal.rs
    - crates/core/src/backends/software_hsm.rs
    - crates/core/src/backends/universal_keyring.rs
    - crates/core/src/backends/universal_registry.rs
    - crates/core/src/backends/keyring.rs
    - crates/core/src/backends/yubikey.rs
    - crates/pubky/src/lib.rs
    - crates/pubky/src/mock.rs
    - crates/trustedge-cli/src/main.rs
    - crates/core/src/lib.rs

key-decisions:
  - "Backend traits use BackendError (not anyhow) - library code requires structured errors"
  - "CLI binaries use ? operator for auto-conversion - BackendError implements std::error::Error"
  - "Semantic error mapping: KeyNotFound for missing keys, UnsupportedOperation for unsupported ops"

patterns-established:
  - "Backend error handling: map anyhow errors to BackendError variants in trait implementations"
  - "Error context preservation: use format!() to add context when converting to BackendError"

# Metrics
duration: 6min
completed: 2026-02-10
---

# Phase 02 Plan 03: Backend Error Migration Summary

**Backend traits migrated from anyhow to structured BackendError with semantic error variants and automatic CLI conversion**

## Performance

- **Duration:** 6 minutes
- **Started:** 2026-02-10T02:59:22Z
- **Completed:** 2026-02-10T03:05:56Z
- **Tasks:** 2
- **Files modified:** 11

## Accomplishments

- Backend traits (KeyBackend, UniversalBackend) return Result<T, BackendError> instead of anyhow::Result
- All 7 backend implementations updated (keyring, universal_keyring, software_hsm, universal_registry, yubikey, pubky, pubky-mock)
- Public API exposes TrustEdgeError, BackendError, TransportError through lib.rs re-exports
- CLI binaries automatically convert BackendError to anyhow via ? operator
- All 348+ workspace tests pass, clippy clean

## Task Commits

Each task was committed atomically:

1. **Task 1: Migrate backend trait signatures to BackendError** - `402bc5a` (feat)
2. **Task 2: Update lib.rs re-exports and run final workspace verification** - `1cde54b` (feat)

## Files Created/Modified

- `crates/core/src/backends/traits.rs` - KeyBackend trait methods return Result<T, BackendError>
- `crates/core/src/backends/universal.rs` - UniversalBackend trait methods return Result<T, BackendError>
- `crates/core/src/backends/keyring.rs` - Updated KeyBackend impl, mapped errors to BackendError variants
- `crates/core/src/backends/universal_keyring.rs` - Updated UniversalBackend impl with BackendError
- `crates/core/src/backends/software_hsm.rs` - Updated perform_operation and list_keys with BackendError
- `crates/core/src/backends/universal_registry.rs` - Updated perform_operation return type
- `crates/core/src/backends/yubikey.rs` - Updated both feature-gated and non-feature impls
- `crates/pubky/src/lib.rs` - Updated PubkyBackend trait impl
- `crates/pubky/src/mock.rs` - Updated MockPubkyBackend trait impl
- `crates/trustedge-cli/src/main.rs` - Fixed backend.derive_key() call to use ? operator
- `crates/core/src/lib.rs` - Added pub use error::{TrustEdgeError, BackendError, TransportError}

## Decisions Made

- **Semantic error mapping:** Used KeyNotFound for missing keys, UnsupportedOperation for unsupported operations, HardwareError for YubiKey failures, OperationFailed for generic failures
- **Automatic conversion in CLIs:** Backend errors convert to anyhow::Error via ? operator because BackendError implements std::error::Error through thiserror
- **Context preservation:** Used format!() to add context when converting anyhow errors to BackendError

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None - compilation and tests passed on first attempt after all migrations.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Phase 2 (Error Handling) is now complete - all success criteria met:
  * Single TrustEdgeError enum with 7 subsystem variants (CryptoError, ChainError, AsymmetricError, ManifestError, ArchiveError, BackendError, TransportError)
  * All duplicate error types consolidated into error.rs
  * Library code uses thiserror, CLI binaries use anyhow
  * Error conversion paths preserve context via #[from] and #[source] attributes
- Backend traits now use structured errors - ready for Phase 3 (Manifest Deduplication)
- YubiKey integration can proceed with clean BackendError handling

---
*Phase: 02-error-handling*
*Completed: 2026-02-10*

## Self-Check: PASSED

All claims verified:
- Modified files exist and contain expected changes
- Commits 402bc5a and 1cde54b exist in git history
- Error re-exports present in lib.rs
- All 348+ workspace tests pass
