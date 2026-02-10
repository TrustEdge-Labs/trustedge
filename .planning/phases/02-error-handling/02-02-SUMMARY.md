---
phase: 02-error-handling
plan: 02
subsystem: error-handling
status: complete
completed_at: 2026-02-10T02:57:13Z
duration_minutes: 6
executor: sonnet

tags:
  - error-consolidation
  - refactoring
  - module-migration
  - backward-compatible

dependency_graph:
  requires:
    - phase: 02
      plan: 01
      artifacts: ["crates/core/src/error.rs with unified error enums"]
  provides:
    - artifact: "crates/core/src/crypto.rs"
      capability: "imports CryptoError from error.rs"
    - artifact: "crates/core/src/chain.rs"
      capability: "imports ChainError from error.rs"
    - artifact: "crates/core/src/manifest.rs"
      capability: "imports ManifestError from error.rs"
    - artifact: "crates/core/src/asymmetric.rs"
      capability: "imports AsymmetricError from error.rs"
    - artifact: "crates/core/src/archive.rs"
      capability: "imports ArchiveError, ManifestError, ChainError from error.rs"
  affects:
    - "All crates importing error types from trustedge-core (backward compatible)"

tech_stack:
  added: []
  patterns:
    - "pub use crate::error::*Error re-exports for backward compatibility"
    - "Single source of truth: all error enums defined in error.rs only"

key_files:
  created: []
  modified:
    - path: "crates/core/src/crypto.rs"
      changes: "Removed CryptoError enum (7 variants), added pub use crate::error::CryptoError"
      lines_changed: -19
    - path: "crates/core/src/chain.rs"
      changes: "Removed ChainError enum (3 variants), added pub use crate::error::ChainError"
      lines_changed: -8
    - path: "crates/core/src/manifest.rs"
      changes: "Removed ManifestError enum (2 variants), added pub use crate::error::ManifestError"
      lines_changed: -5
    - path: "crates/core/src/asymmetric.rs"
      changes: "Removed AsymmetricError enum (4 variants), added pub use crate::error::AsymmetricError"
      lines_changed: -14
    - path: "crates/core/src/archive.rs"
      changes: "Removed ArchiveError enum (10 variants), added pub use imports from error.rs"
      lines_changed: -24

decisions:
  - summary: "Use pub use for re-exports to maintain backward compatibility"
    rationale: "lib.rs expects error types at module level (e.g., pub use crypto::CryptoError). Using pub use ensures trustedge_core::crypto::CryptoError path still resolves."
    alternatives: "Could have updated all import paths, but that would break external users."
    result: "Zero breaking changes for dependent crates."

  - summary: "No conversion code needed for AsymmetricError::BackendError"
    rationale: "error.rs version uses String (not anyhow::Error), but asymmetric.rs had no actual usages of the BackendError variant with anyhow conversions."
    result: "Clean migration with no code changes beyond enum removal."

metrics:
  duration: "6 minutes"
  tasks_completed: 2
  files_modified: 5
  lines_removed: 70
  lines_added: 6
  tests_passing: "348+ workspace tests"
  commits: 2
---

# Phase 02 Plan 02: Module Error Migration Summary

Migrated all core module error definitions to use unified error.rs types.

**One-liner:** Removed duplicate error enum definitions from 5 core modules (crypto, chain, manifest, asymmetric, archive) and replaced with pub use imports from error.rs — maintaining backward compatibility and zero test regressions.

## Objective

Remove duplicate error enum definitions from individual modules and consolidate them into error.rs. After this plan, all error types in trustedge-core are defined in exactly one place (error.rs), while function signatures and public behavior remain identical.

## What Was Built

### Task 1: Leaf Error Module Migration
**Commit:** `44ffa85`

Migrated three leaf modules that have no cross-dependencies on other TrustEdge error types:

1. **crypto.rs:**
   - Removed 7-variant CryptoError enum definition
   - Added `pub use crate::error::CryptoError;`
   - All functions returning `Result<_, CryptoError>` unchanged
   - 133 trustedge-core tests pass

2. **chain.rs:**
   - Removed 3-variant ChainError enum definition
   - Added `pub use crate::error::ChainError;`
   - Continuity chain validation functions unchanged

3. **manifest.rs:**
   - Removed 2-variant ManifestError enum definition
   - Added `pub use crate::error::ManifestError;`
   - Manifest serialization functions unchanged

**Result:** Clean drop-in replacement. Error types identical, just imported from different location.

### Task 2: Dependent Error Module Migration
**Commit:** `ed10885`

Migrated two modules with error types that depend on other error types:

1. **asymmetric.rs:**
   - Removed 4-variant AsymmetricError enum definition
   - Added `pub use crate::error::AsymmetricError;`
   - No conversion code needed (BackendError variant unused)
   - All key generation and ECDH functions unchanged

2. **archive.rs:**
   - Removed 10-variant ArchiveError enum definition
   - Added `pub use crate::error::{ArchiveError, ChainError, ManifestError};`
   - Imports all dependent error types from error.rs
   - Archive read/write/validation functions unchanged

**Verification:**
- `cargo build --workspace` compiles successfully
- `cargo test --workspace` passes all 348+ tests
- Backward compatibility verified: `trustedge_core::crypto::CryptoError` path still resolves
- No breaking changes for dependent crates (receipts, attestation, trst-cli)

## Deviations from Plan

None — plan executed exactly as written. All error enum definitions removed, pub use re-exports added, zero test regressions.

## Verification Results

All verification criteria met:

- [x] `cargo build --workspace` compiles without errors
- [x] `cargo test --workspace` passes all 348+ tests
- [x] No error enum definitions remain in crypto.rs, chain.rs, manifest.rs, asymmetric.rs, archive.rs
- [x] All 5 modules have `pub use crate::error::*Error;` re-exports
- [x] `trustedge_core::crypto::CryptoError` path still resolves (backward compatible)

Test output shows:
- trustedge-core: 133 tests passed
- trustedge-receipts: 23 tests passed
- trustedge-trst-cli: 7 acceptance tests passed
- All other crates build and test successfully

## Dependencies

**Depends on:**
- Plan 02-01: Error hierarchy foundation (error.rs with all enum definitions)

**Blocks:**
- Plan 02-03: Function signature migration (can now safely migrate Result types)

## Key Files Modified

| File | Change | Impact |
|------|--------|--------|
| crates/core/src/crypto.rs | Removed CryptoError enum (7 variants) | -19 lines |
| crates/core/src/chain.rs | Removed ChainError enum (3 variants) | -8 lines |
| crates/core/src/manifest.rs | Removed ManifestError enum (2 variants) | -5 lines |
| crates/core/src/asymmetric.rs | Removed AsymmetricError enum (4 variants) | -14 lines |
| crates/core/src/archive.rs | Removed ArchiveError enum (10 variants) | -24 lines |

**Total:** 70 lines removed, 6 lines added (pub use statements). Net reduction: 64 lines.

## Technical Notes

### Backward Compatibility Pattern

The `pub use crate::error::*Error;` pattern ensures backward compatibility:

```rust
// In crypto.rs
pub use crate::error::CryptoError;  // Re-export at module level

// In lib.rs (unchanged)
pub use crypto::{..., CryptoError, ...};

// External users (unchanged)
use trustedge_core::crypto::CryptoError;  // Still works!
use trustedge_core::error::CryptoError;   // Also works!
```

This means zero breaking changes for dependent crates.

### AsymmetricError::BackendError Variant

The error.rs version changed `BackendError(#[from] anyhow::Error)` to `BackendError(String)`. This could have required conversion code, but analysis showed asymmetric.rs had no actual usages of the variant with anyhow conversions. Functions return `anyhow::Result` at the function level, but don't convert to AsymmetricError::BackendError. Clean migration with no code changes needed.

### Import Consolidation in archive.rs

archive.rs imports multiple error types from error.rs:

```rust
pub use crate::error::{ArchiveError, ChainError, ManifestError};
```

This consolidates what were previously separate imports:
- `ManifestError` was from `crate::manifest`
- `ChainError` was from `crate::chain`
- Now both come from `crate::error` (single source of truth)

## Success Criteria Met

- [x] All 5 core error types consolidated into error.rs (single source of truth)
- [x] Zero test regressions across the entire workspace
- [x] Backward-compatible module paths preserved via pub use re-exports
- [x] No function signature changes (same return types)

## Self-Check: PASSED

**Created files:** None (modification-only plan)

**Modified files:**
- [x] crates/core/src/crypto.rs exists
- [x] crates/core/src/chain.rs exists
- [x] crates/core/src/manifest.rs exists
- [x] crates/core/src/asymmetric.rs exists
- [x] crates/core/src/archive.rs exists

**Commits:**
- [x] 44ffa85 exists (Task 1: leaf module migration)
- [x] ed10885 exists (Task 2: dependent module migration)

**Verification:**
- [x] No error enum definitions remain in migrated modules
- [x] All modules have pub use re-exports
- [x] Workspace builds successfully
- [x] All 348+ tests pass
- [x] Backward compatibility verified

All checks passed. Plan 02-02 executed successfully.
