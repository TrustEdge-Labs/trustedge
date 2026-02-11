<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


---
phase: 03-trst-core-integration
plan: 01
subsystem: trst-protocols
tags: [rename, restructure, protocols, wasm]
dependency_graph:
  requires: []
  provides: [trst-protocols-crate, domain-submodules, scoped-errors]
  affects: [trst-wasm, trst-cli]
tech_stack:
  added: []
  patterns: [domain-based-modules, scoped-error-types]
key_files:
  created:
    - crates/trst-protocols/src/archive/mod.rs
    - crates/trst-protocols/src/archive/chunks.rs
    - crates/trst-protocols/src/archive/signatures.rs
    - crates/trst-protocols/src/capture/mod.rs
    - crates/trst-protocols/src/capture/profile.rs
  modified:
    - crates/trst-protocols/Cargo.toml (renamed from trst-core)
    - crates/trst-protocols/src/lib.rs
    - crates/trst-protocols/src/archive/manifest.rs (moved and updated)
    - crates/trst-wasm/Cargo.toml
    - crates/trst-wasm/src/lib.rs
    - crates/trst-cli/Cargo.toml
    - Cargo.toml (workspace members)
decisions:
  - "Renamed trst-core to trst-protocols to better reflect purpose as protocol definitions"
  - "Structured into archive and capture domain submodules for clear separation"
  - "Renamed ManifestError to ManifestFormatError for scoped error naming consistency"
  - "Created ChunkFormatError, SignatureFormatError, ProfileFormatError for future use"
  - "Added backward compatibility alias (ManifestError) in lib.rs for transition period"
metrics:
  duration_minutes: 4.6
  tasks_completed: 2
  tests_passing: 6
  files_modified: 11
  commits: 2
  completed_at: "2026-02-10T14:38:08Z"
---

# Phase 03 Plan 01: Rename trst-core to trst-protocols Summary

**One-liner:** Renamed trst-core to trst-protocols with domain-based submodules (archive + capture) and scoped error types per submodule.

## What Was Done

### Task 1: Rename and Restructure
- Renamed `crates/trst-core/` to `crates/trst-protocols/` using git mv
- Updated package name to `trustedge-trst-protocols`
- Updated description to reflect protocol/format definitions purpose
- Created domain-based module hierarchy:
  - `archive/` - Archive format types (manifest, chunks, signatures)
  - `capture/` - Capture profile types (cam.video and future profiles)
- Moved manifest.rs to archive submodule with error type rename
- Created scoped error types for each submodule:
  - `ManifestFormatError` (renamed from ManifestError)
  - `ChunkFormatError` - for future chunk validation
  - `SignatureFormatError` - for future signature envelope parsing
  - `ProfileFormatError` - for future capture profile validation
- Updated lib.rs with new module structure and re-exports
- Added backward compatibility alias for ManifestError (temporary)
- All 5 manifest tests preserved and passing
- Added 1 doc test passing

### Task 2: Update trst-wasm
- Updated Cargo.toml dependency: `trustedge-trst-core` → `trustedge-trst-protocols`
- Updated all import statements in lib.rs
- Updated doc comments to reference trst-protocols
- No API surface changes: CamVideoManifest interface unchanged

## Verification Results

All verification checks passed:

1. `cargo check -p trustedge-trst-protocols` — ✓ Compiles successfully
2. `cargo check -p trustedge-trst-protocols --target wasm32-unknown-unknown` — ✓ WASM-safe
3. `cargo test -p trustedge-trst-protocols` — ✓ 5 unit tests + 1 doc test passing
4. `cargo check --workspace` — ✓ No workspace breakage
5. `cargo check -p trustedge-trst-wasm` — ✓ Compiles successfully
6. No stale trst-core references in trst-wasm or trst-protocols
7. Old `crates/trst-core/` directory no longer exists

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Updated trst-cli Cargo.toml dependency**
- **Found during:** Task 1 verification
- **Issue:** trst-cli still referenced trst-core in Cargo.toml, causing workspace-level cargo check to fail
- **Fix:** Updated trst-cli/Cargo.toml to use `trustedge-trst-protocols` instead of `trustedge-trst-core`
- **Files modified:** crates/trst-cli/Cargo.toml
- **Commit:** 05500f3 (included in Task 1)
- **Reason:** Plan mentioned trst-cli as affected but didn't explicitly list updating its Cargo.toml; this was necessary to complete the rename and allow verification to proceed

## Commits

| Task | Commit | Description |
|------|--------|-------------|
| 1 | 05500f3 | feat(03-01): rename trst-core to trst-protocols with domain submodules |
| 2 | 1f56a6b | feat(03-01): update trst-wasm to use trst-protocols |

## Files Modified

### Created (5)
- `crates/trst-protocols/src/archive/mod.rs` - Archive domain module with re-exports
- `crates/trst-protocols/src/archive/chunks.rs` - ChunkFormatError type
- `crates/trst-protocols/src/archive/signatures.rs` - SignatureFormatError type
- `crates/trst-protocols/src/capture/mod.rs` - Capture domain module with re-exports
- `crates/trst-protocols/src/capture/profile.rs` - ProfileFormatError type

### Modified (6)
- `Cargo.toml` - Updated workspace members list
- `crates/trst-protocols/Cargo.toml` - Renamed package, updated description
- `crates/trst-protocols/src/lib.rs` - New module structure with re-exports
- `crates/trst-protocols/src/archive/manifest.rs` - Moved from src/, renamed error type
- `crates/trst-cli/Cargo.toml` - Updated dependency reference
- `crates/trst-wasm/Cargo.toml` - Updated dependency reference
- `crates/trst-wasm/src/lib.rs` - Updated imports and doc comments

### Deleted (1)
- `crates/trst-core/` - Entire directory renamed to trst-protocols

## Technical Notes

### Module Structure
```
trst-protocols/
├── src/
│   ├── lib.rs              # Crate root with re-exports
│   ├── archive/
│   │   ├── mod.rs          # Archive domain re-exports
│   │   ├── manifest.rs     # CamVideoManifest + ManifestFormatError
│   │   ├── chunks.rs       # ChunkFormatError
│   │   └── signatures.rs   # SignatureFormatError
│   └── capture/
│       ├── mod.rs          # Capture domain re-exports
│       └── profile.rs      # ProfileFormatError
```

### Error Type Naming Convention
All error types follow the pattern `{Domain}FormatError`:
- `ManifestFormatError` - manifest parsing/validation errors
- `ChunkFormatError` - chunk structure errors
- `SignatureFormatError` - signature envelope errors
- `ProfileFormatError` - capture profile errors

This scoped naming prevents collisions and clearly indicates error domain.

### Backward Compatibility
The lib.rs includes `pub use ManifestFormatError as ManifestError` to maintain backward compatibility during the transition. This alias is marked `#[doc(hidden)]` and should be removed in a future phase once all consumers are updated.

### WASM Safety
All dependencies remain WASM-safe (serde, serde_json, thiserror only). Successfully compiles for `wasm32-unknown-unknown` target.

## Self-Check: PASSED

All created files verified:
- FOUND: crates/trst-protocols/src/archive/mod.rs
- FOUND: crates/trst-protocols/src/archive/manifest.rs
- FOUND: crates/trst-protocols/src/archive/chunks.rs
- FOUND: crates/trst-protocols/src/archive/signatures.rs
- FOUND: crates/trst-protocols/src/capture/mod.rs
- FOUND: crates/trst-protocols/src/capture/profile.rs
- FOUND: crates/trst-protocols/src/lib.rs
- FOUND: crates/trst-protocols/Cargo.toml

All commits verified:
- FOUND: 05500f3
- FOUND: 1f56a6b

Old directory removed:
- VERIFIED: crates/trst-core/ does not exist

All tests passing: 6 total (5 unit + 1 doc)
