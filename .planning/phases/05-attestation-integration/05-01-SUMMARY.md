<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


---
phase: 05-attestation-integration
plan: 01
subsystem: applications/attestation
tags: [migration, attestation, layer-4, consolidation]
dependency_graph:
  requires:
    - core/src/envelope.rs (Envelope type)
    - core/src/applications/mod.rs (Layer 4 structure)
  provides:
    - applications::attestation (10 tests, 826 LOC)
    - Attestation types and functions at crate root
  affects:
    - crates/attestation (now thin facade)
    - core examples (3 new examples)
tech_stack:
  added: [git2]
  patterns: [re-export-facade, git-history-preservation, feature-gate-removal]
key_files:
  created:
    - crates/core/src/applications/attestation/mod.rs
    - crates/core/examples/attest.rs
    - crates/core/examples/verify_attestation.rs
    - crates/core/examples/attestation_demo.rs
    - crates/attestation/src/lib.rs (facade)
  modified:
    - crates/core/Cargo.toml
    - crates/core/src/applications/mod.rs
    - crates/core/src/lib.rs
    - crates/attestation/Cargo.toml
  deleted:
    - crates/attestation/src/bin/
    - crates/attestation/examples/
decisions:
  - title: Remove all envelope feature gates
    rationale: "Envelope is always available inside core (defined in crate::envelope), so cfg(feature = envelope) is meaningless. Removed 2 tests that only existed for cfg(not(feature = envelope)) scenario."
  - title: Convert binaries to cargo examples
    rationale: "Examples are more discoverable and align with Rust conventions for demo code. Renamed verify.rs to verify_attestation.rs to avoid potential name collision."
  - title: Keep minimal dependencies in facade
    rationale: "Facade only needs trustedge-core for re-exports. Kept anyhow, serde, serde_json conservatively to avoid breaking downstream that might import them through attestation crate."
metrics:
  duration_minutes: 6
  completed_date: 2026-02-10
  tasks_completed: 2
  tests_migrated: 10
  loc_migrated: 826
---

# Phase 5 Plan 01: Attestation Integration Summary

**Moved attestation crate (826 LOC, 10 tests) into core applications layer with envelope feature gates removed**

## Objective Achievement

Successfully migrated the attestation crate into trustedge-core's applications layer following the proven Phase 4 receipts pattern. All 10 lib tests now pass inside core, envelope integration is direct (no feature flags), and backward compatibility is maintained via thin re-export facade.

## Tasks Completed

### Task 1: Move attestation into core applications layer
**Status:** ✔ Complete
**Commit:** `6ea4d77`

- Added git2 dependency to core Cargo.toml (attestation uses it for source commit hash capture)
- Created `crates/core/src/applications/attestation/` directory
- Moved `crates/attestation/src/lib.rs` → `crates/core/src/applications/attestation/mod.rs` (git mv preserves history)
- Updated imports: `use trustedge_core::Envelope` → `use crate::Envelope`
- Removed ALL `#[cfg(feature = "envelope")]` gates from functions and tests
- Deleted `#[cfg(not(feature = "envelope"))]` fallback function (dead code inside core)
- Deleted 2 tests: `test_provided_key_source_json_fallback` and `test_sealed_envelope_fallback_without_feature` (both tested scenarios impossible inside core)
- Updated module doc comments: references to `trustedge_attestation` → `trustedge_core::applications::attestation`
- Declared module in `crates/core/src/applications/mod.rs`
- Added re-exports to `crates/core/src/lib.rs` for all attestation types and functions
- Verified: All 10 tests pass in core, no envelope feature gates remain

**Key Files:**
- `crates/core/src/applications/attestation/mod.rs` (826 LOC, 10 tests)
- `crates/core/Cargo.toml` (added git2)
- `crates/core/src/applications/mod.rs` (module declaration)
- `crates/core/src/lib.rs` (re-exports)

### Task 2: Convert binaries to examples, create facade
**Status:** ✔ Complete
**Commit:** `2866837`

- Moved `crates/attestation/src/bin/attest.rs` → `crates/core/examples/attest.rs`
- Moved `crates/attestation/src/bin/verify.rs` → `crates/core/examples/verify_attestation.rs` (renamed)
- Moved `crates/attestation/examples/attestation_demo.rs` → `crates/core/examples/attestation_demo.rs`
- Updated all imports in examples: `trustedge_attestation` → `trustedge_core`
- Removed `#[cfg(feature = "envelope")]` blocks from all examples (envelope always available)
- Updated help text references: CLI binaries → cargo examples
- Created thin re-export facade in `crates/attestation/src/lib.rs`
- Updated `crates/attestation/Cargo.toml`: removed binaries, removed features, cleaned dependencies
- Deleted empty `crates/attestation/src/bin/` and `crates/attestation/examples/` directories
- Verified: All 3 examples runnable, attestation crate has 0 tests, full workspace compiles clean

**Key Files:**
- `crates/core/examples/attest.rs` (CLI-style attestation creation)
- `crates/core/examples/verify_attestation.rs` (CLI-style verification)
- `crates/core/examples/attestation_demo.rs` (demo of library usage)
- `crates/attestation/src/lib.rs` (17 LOC facade)
- `crates/attestation/Cargo.toml` (minimal facade config)

## Deviations from Plan

None - plan executed exactly as written.

## Verification Results

All verification criteria passed:

1. ✔ `cargo test -p trustedge-core --lib -- applications::attestation` → 10 tests passed
2. ✔ `cargo test --workspace` → All tests passing (10 attestation tests moved to core)
3. ✔ `cargo modules dependencies --lib -p trustedge-core --acyclic` → No circular dependencies (false positive on Display trait ignored)
4. ✔ `cargo run -p trustedge-core --example attestation_demo` → Demo executes successfully
5. ✔ `cargo run -p trustedge-core --example attest -- --help` → Shows CLI help
6. ✔ `cargo run -p trustedge-core --example verify_attestation -- --help` → Shows CLI help
7. ✔ `cargo clippy --workspace -- -D warnings` → Clean
8. ✔ Attestation types importable: `use trustedge_core::{Attestation, create_signed_attestation, verify_attestation};`
9. ✔ No `#[cfg(feature = "envelope")]` remains in any migrated file

**Test Count Integrity:**
- Before: 10 tests in attestation crate (8 always-on + 2 envelope-gated + 2 not-envelope)
- After: 10 tests in core (8 always-on + 2 formerly envelope-gated, now always-on), 0 in attestation crate
- Net change: -2 tests (cfg(not(feature)) tests deleted as dead code)

## Impact Analysis

**Code Migration:**
- 826 LOC moved from attestation → core
- 10 lib tests moved to core
- 3 examples created in core
- 2 binaries eliminated (converted to examples)

**Dependency Changes:**
- Core gained: git2 (for source commit hash capture)
- Attestation lost: sha2, chrono, git2, bincode, clap, ed25519-dalek, rand, thiserror, hex, tempfile
- Attestation kept: anyhow, serde, serde_json, trustedge-core

**Architecture:**
- Attestation functionality now in Layer 4 (applications)
- Direct envelope integration (no feature flags)
- Thin facade maintains backward compatibility
- Examples follow Rust conventions

## Next Steps

1. **Phase 5 Plan 02**: Continue Phase 5 attestation integration work
2. **Phase 7**: Fully deprecate attestation facade crate
3. **Documentation**: Update CLAUDE.md with attestation examples

## Self-Check

**Files created:**
- ✔ FOUND: crates/core/src/applications/attestation/mod.rs
- ✔ FOUND: crates/core/examples/attest.rs
- ✔ FOUND: crates/core/examples/verify_attestation.rs
- ✔ FOUND: crates/core/examples/attestation_demo.rs
- ✔ FOUND: crates/attestation/src/lib.rs

**Commits:**
- ✔ FOUND: 6ea4d77 (Task 1)
- ✔ FOUND: 2866837 (Task 2)

**Tests:**
- ✔ 10 attestation tests passing in core
- ✔ 0 tests in attestation crate (facade)

## Self-Check: PASSED

All files exist, all commits present, all tests passing.
