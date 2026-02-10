---
phase: 04-receipts-integration
plan: 01
subsystem: applications
tags: [receipts, migration, consolidation]
dependencies:
  requires: [envelope, layer4-scaffold]
  provides: [receipts-in-core]
  affects: [trustedge-core, trustedge-receipts]
tech_stack:
  added: []
  patterns: [re-export-facade, git-mv-history-preservation]
key_files:
  created:
    - crates/core/src/applications/receipts/mod.rs
    - crates/core/examples/receipts_demo.rs
    - crates/receipts/src/lib.rs (facade)
  modified:
    - crates/core/src/applications/mod.rs
    - crates/core/src/lib.rs
    - crates/receipts/Cargo.toml
decisions: []
metrics:
  duration_minutes: 4
  tasks_completed: 2
  files_moved: 2
  tests_migrated: 23
  loc_consolidated: 1281
  completed_at: 2026-02-10T22:15:47Z
---

# Phase 4 Plan 01: Move Receipts into Core Summary

**One-liner:** Digital receipt system (1,281 LOC, 23 tests) migrated from standalone crate into trustedge-core applications layer with backward-compatible facade.

## What Was Done

Consolidated the `trustedge-receipts` crate into `trustedge-core` by:

1. **Moved receipts implementation to core applications layer:**
   - Used `git mv` to preserve history: `crates/receipts/src/lib.rs` → `crates/core/src/applications/receipts/mod.rs`
   - Updated import from `trustedge_core::Envelope` to `crate::Envelope`
   - Updated doc comments to reference `trustedge_core` instead of `trustedge_receipts`
   - Added module declaration in `crates/core/src/applications/mod.rs`
   - Added re-exports in `crates/core/src/lib.rs` for public API

2. **Converted demo binary to cargo example:**
   - Moved `crates/receipts/src/bin/demo.rs` → `crates/core/examples/receipts_demo.rs`
   - Updated imports from `trustedge_receipts` to `trustedge_core`
   - Removed `[[bin]]` section from receipts Cargo.toml
   - Demo now runnable as: `cargo run -p trustedge-core --example receipts_demo`

3. **Created backward-compatibility facade:**
   - Replaced receipts crate with thin re-export facade
   - Facade re-exports: `Receipt`, `create_receipt`, `assign_receipt`, `extract_receipt`, `verify_receipt_chain`
   - Receipts crate now has 0 tests (all 23 migrated to core)
   - Maintains backward compatibility for existing consumers

## Test Results

**Before migration:**
- trustedge-receipts: 23 tests
- trustedge-core: 127 tests

**After migration:**
- trustedge-receipts: 0 tests (facade only)
- trustedge-core: 150 tests (127 + 23 receipts)

**Verification:**
```bash
✔ cargo test -p trustedge-core --lib -- applications::receipts  # 23 passed
✔ cargo test -p trustedge-receipts                               # 0 tests (facade)
✔ cargo run -p trustedge-core --example receipts_demo            # Demo executes
✔ cargo check --workspace                                        # Full workspace compiles
✔ cargo clippy --workspace -- -D warnings                        # Clean
```

## Architecture Impact

**Layer 4 (Applications) now contains:**
- `applications/receipts/` - Digital receipt system with cryptographic ownership chains
  - `Receipt` struct with issuer, beneficiary, amount, chain link
  - `create_receipt()` - Create origin receipts
  - `assign_receipt()` - Transfer ownership in chain
  - `extract_receipt()` - Deserialize and validate
  - `verify_receipt_chain()` - Validate ownership chain

**Public API preserved:**
- All receipt types and functions importable from `trustedge_core` crate root
- Backward compatibility maintained via re-export facade in `trustedge-receipts` crate
- No breaking changes for existing consumers

**Benefits:**
- Single source of truth for receipt logic
- Receipts can directly use internal core types (no cross-crate boundaries)
- Reduces compilation units (workspace now -1 crate dependency)
- Demo converted from binary to example (better discoverability)

## Deviations from Plan

None - plan executed exactly as written.

## Key Files Modified

| File | Change | Lines |
|------|--------|-------|
| `crates/core/src/applications/receipts/mod.rs` | Moved from receipts crate | 1,281 |
| `crates/core/src/applications/mod.rs` | Added receipts module declaration | +2 |
| `crates/core/src/lib.rs` | Added receipt re-exports | +3 |
| `crates/core/examples/receipts_demo.rs` | Moved from receipts binary | 159 |
| `crates/receipts/src/lib.rs` | Replaced with facade | -1,266 |
| `crates/receipts/Cargo.toml` | Removed [[bin]] section | -4 |

**Net consolidation:** 1,281 LOC of receipt logic now in core

## Commits

| Commit | Description |
|--------|-------------|
| `c7a7b87` | Move receipts into core applications layer |
| `5272bef` | Convert demo to example and create receipts facade |

## Self-Check: PASSED

**Created files verification:**
```bash
✔ crates/core/src/applications/receipts/mod.rs - FOUND (1,281 LOC)
✔ crates/core/examples/receipts_demo.rs - FOUND (159 lines)
✔ crates/receipts/src/lib.rs - FOUND (facade, 15 lines)
```

**Modified files verification:**
```bash
✔ crates/core/src/applications/mod.rs - Contains "pub mod receipts;"
✔ crates/core/src/lib.rs - Contains receipt re-exports
✔ crates/receipts/Cargo.toml - [[bin]] section removed
```

**Commits verification:**
```bash
✔ c7a7b87 - FOUND: feat(04-01): move receipts into core applications layer
✔ 5272bef - FOUND: feat(04-01): convert demo to example and create receipts facade
```

**Test verification:**
```bash
✔ 23 receipt tests passing in trustedge-core
✔ 0 tests in trustedge-receipts (facade only)
✔ Demo example runs successfully
✔ Full workspace compiles clean
```

All verifications passed.
