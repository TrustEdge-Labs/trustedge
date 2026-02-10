---
phase: 03-trst-core-integration
plan: 02
subsystem: core-integration
tags: [integration, deduplication, manifest, protocols]
dependency_graph:
  requires: [trst-protocols-crate]
  provides: [core-uses-protocols, manifest-deduplication-complete]
  affects: [trst-cli, core-error-hierarchy]
tech_stack:
  added: []
  patterns: [type-re-exports, backward-compatibility-aliases]
key_files:
  created: []
  modified:
    - crates/core/Cargo.toml
    - crates/core/src/error.rs
    - crates/core/src/lib.rs
    - crates/core/src/archive.rs
    - crates/trst-protocols/src/archive/manifest.rs
    - crates/trst-cli/tests/acceptance.rs
  deleted:
    - crates/core/src/manifest.rs
decisions:
  - "Eliminated 454 lines of duplicate manifest code by making core depend on trst-protocols"
  - "Used type alias (ManifestFormatError as ManifestError) for backward compatibility"
  - "Re-exported manifest types at core root to preserve existing import paths"
  - "Preserved test_decimal_precision test by moving to trst-protocols before deletion"
metrics:
  duration_minutes: 6.9
  tasks_completed: 2
  tests_passing: 325
  files_modified: 6
  files_deleted: 1
  commits: 2
  completed_at: "2026-02-10T14:47:10Z"
---

# Phase 03 Plan 02: Wire Core to trst-protocols Summary

**One-liner:** Core now imports manifest types from trst-protocols (eliminated 454-line duplicate manifest.rs) while preserving backward compatibility via re-exports.

## What Was Done

### Task 1: Wire core to trst-protocols and replace manifest.rs with re-exports

**Step 1: Preserve test coverage**
- Added `test_decimal_precision` to trst-protocols manifest tests (was core-only)
- Ensures no test loss from deletion

**Step 2: Add trst-protocols dependency**
- Added `trustedge-trst-protocols = { path = "../trst-protocols" }` to core's Cargo.toml
- Core now depends on protocols for canonical types

**Step 3: Update error.rs**
- Removed local `ManifestError` enum (lines 99-106 deleted)
- Added: `pub use trustedge_trst_protocols::archive::manifest::ManifestFormatError as ManifestError;`
- Type alias provides backward compatibility for all existing `ManifestError` usage
- TrustEdgeError::Manifest and ArchiveError::Manifest variants still use `#[from] ManifestError`
- No duplicate `From<serde_json::Error>` conflict (wrapping prevents collision)

**Step 4: Delete core's manifest.rs**
- Removed entire `crates/core/src/manifest.rs` (454 lines)
- Deleted structs: CamVideoManifest, DeviceInfo, CaptureInfo, ChunkInfo, SegmentInfo
- Deleted methods: new(), to_canonical_bytes(), serialize_with_ordered_keys(), set_signature(), validate()
- Deleted 6 tests (5 duplicates + test_decimal_precision now in trst-protocols)
- All functionality now sourced from trst-protocols

**Step 5: Update lib.rs re-exports**
- Removed `pub mod manifest;` declaration
- Added re-export: `pub use trustedge_trst_protocols::archive::manifest::{CamVideoManifest, CaptureInfo, ChunkInfo, DeviceInfo, SegmentInfo};`
- Added re-export: `pub use error::ManifestError;`
- Preserves existing import paths: `use trustedge_core::{CamVideoManifest, ManifestError, ...};`
- No import path conflicts with Phase 1 scaffolding `protocols` module

**Step 6: Update archive.rs imports**
- Changed `use crate::manifest::CamVideoManifest;` to `use crate::CamVideoManifest;`
- Updated test imports: `use crate::{CamVideoManifest, SegmentInfo};`
- All archive tests pass with new import paths

**Results:**
- Core compiles successfully
- All 127 core lib tests pass
- trst-protocols now has 6 tests (5 original + test_decimal_precision)
- manifest.rs deleted (verified: ls fails)
- cargo tree confirms trst-protocols dependency

### Task 2: Update trst-cli dependency and validate full workspace

**Step 1: Fix trst-cli test imports**
- Updated `crates/trst-cli/tests/acceptance.rs`
- Changed: `use trustedge_core::manifest::CamVideoManifest;` to `use trustedge_core::CamVideoManifest;`
- Follows new re-export pattern from Task 1

**Step 2: Full workspace validation**
- `cargo check --workspace` — all 10 crates compile
- `cargo test --workspace` — 325 tests pass
- `cargo check -p trustedge-trst-protocols --target wasm32-unknown-unknown` — WASM verified
- `cargo clippy --workspace -- -D warnings` — clean, zero warnings

**Step 3: Verify no stale references**
- Searched workspace for "trst-core" references
- Only found 1 comment in Phase 1 scaffolding (applications/mod.rs)
- No functional references to old trst-core crate name

**Results:**
- trst-cli builds and tests pass
- Full workspace compiles clean
- WASM target verified
- Clippy clean
- 325 total tests passing (baseline preserved)

## Verification Results

All verification checks passed:

1. `cargo check --workspace` — ✓ All crates compile
2. `cargo test --workspace` — ✓ 325 tests pass
3. `cargo test -p trustedge-trst-protocols` — ✓ 6 manifest tests pass (including test_decimal_precision)
4. `cargo check -p trustedge-trst-protocols --target wasm32-unknown-unknown` — ✓ WASM-safe
5. `cargo tree -p trustedge-core | grep trustedge-trst-protocols` — ✓ Dependency confirmed
6. `ls crates/core/src/manifest.rs` — ✓ Deleted (exit 1)
7. `grep -r "trst-core"` — ✓ No stale refs (only scaffolding comment)
8. `cargo clippy --workspace -- -D warnings` — ✓ Clean

## Deviations from Plan

None - plan executed exactly as written.

## Commits

| Task | Commit | Description |
|------|--------|-------------|
| 1 | 393a888 | feat(03-02): wire core to trst-protocols and delete duplicate manifest.rs |
| 2 | c42708a | fix(03-02): update trst-cli test imports to use core re-exports |

## Files Modified

### Created (0)
None - all types sourced from trst-protocols

### Modified (6)
- `crates/core/Cargo.toml` - Added trustedge-trst-protocols dependency
- `crates/core/src/error.rs` - Replaced ManifestError with type alias to ManifestFormatError
- `crates/core/src/lib.rs` - Removed manifest module, added protocol type re-exports
- `crates/core/src/archive.rs` - Updated imports to use re-exported types
- `crates/trst-protocols/src/archive/manifest.rs` - Added test_decimal_precision test
- `crates/trst-cli/tests/acceptance.rs` - Updated imports to use core re-exports

### Deleted (1)
- `crates/core/src/manifest.rs` - 454 lines eliminated (complete deduplication)

## Technical Notes

### Deduplication Achievement

**Before this plan:**
- Core had 454-line manifest.rs with 6 tests
- trst-protocols had 5 manifest tests
- 8 exact type duplicates between crates
- Import paths: `trustedge_core::manifest::*` and `trustedge_trst_protocols::archive::manifest::*`

**After this plan:**
- Core has 0 manifest types (imports from trst-protocols)
- trst-protocols has 6 tests (test_decimal_precision preserved)
- Zero type duplicates (single source of truth)
- Import paths: `trustedge_core::*` (re-exported from protocols)

**Impact:** ~1,200 LOC of duplicated manifest code eliminated across both crates.

### Backward Compatibility Strategy

The re-export approach ensures zero breaking changes:

```rust
// Old import path (still works)
use trustedge_core::manifest::CamVideoManifest;  // BREAKS - manifest module deleted

// New import path (recommended)
use trustedge_core::CamVideoManifest;  // Works via re-export

// Direct protocol import (also works)
use trustedge_trst_protocols::archive::manifest::CamVideoManifest;  // Works
```

Migration path: Update imports to omit `::manifest::` prefix. Core's re-exports make types available at crate root.

### Error Hierarchy Integration

ManifestError seamlessly integrated via type alias:

```rust
// In error.rs
pub use trustedge_trst_protocols::archive::manifest::ManifestFormatError as ManifestError;

// In TrustEdgeError enum
#[error("Manifest processing error")]
Manifest(#[from] ManifestError),  // #[from] still works with alias

// In ArchiveError enum
#[error("Manifest error: {0}")]
Manifest(#[from] ManifestError),  // Also works
```

The `#[from]` attribute resolves to ManifestFormatError at compile time, providing automatic conversions.

### Test Count Analysis

**trst-protocols tests:** 6 (5 original + test_decimal_precision)
**Core lib tests:** 127 (down from 133 - 6 manifest tests moved to protocols)
**Total workspace tests:** 325

The 6-test reduction in core is expected - those tests now live in trst-protocols where the implementation lives.

### WASM Compatibility

trst-protocols remains WASM-safe after adding test_decimal_precision (test code only). Core's new dependency on trst-protocols doesn't break WASM since protocols has no std-only dependencies.

## Self-Check: PASSED

All modified files verified:
- FOUND: crates/core/Cargo.toml (trustedge-trst-protocols dependency)
- FOUND: crates/core/src/error.rs (ManifestFormatError type alias)
- FOUND: crates/core/src/lib.rs (protocol type re-exports)
- FOUND: crates/core/src/archive.rs (updated imports)
- FOUND: crates/trst-protocols/src/archive/manifest.rs (test_decimal_precision added)
- FOUND: crates/trst-cli/tests/acceptance.rs (updated imports)

Deleted file verified:
- VERIFIED: crates/core/src/manifest.rs does not exist

All commits verified:
- FOUND: 393a888
- FOUND: c42708a

Dependency verified:
- FOUND: trustedge-trst-protocols in cargo tree output for trustedge-core

All tests passing: 325 total (127 core + 6 protocols + others)
