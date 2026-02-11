<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


---
phase: 01-foundation
plan: 02
subsystem: trustedge-core
tags:
  - architecture
  - scaffolding
  - layer-hierarchy
dependency-graph:
  requires: []
  provides:
    - Layer directory structure (primitives, protocols, applications, io)
    - Layer contract documentation
    - Module declarations in lib.rs
  affects:
    - crates/core/src/lib.rs (module declarations)
tech-stack:
  added: []
  patterns:
    - 6-layer architecture pattern
    - Layer contract documentation
key-files:
  created:
    - crates/core/src/primitives/mod.rs
    - crates/core/src/protocols/mod.rs
    - crates/core/src/applications/mod.rs
    - crates/core/src/io/mod.rs
  modified:
    - crates/core/src/lib.rs
decisions:
  - Flat layout (no src/layers/ parent directory)
  - Module named `io` (no conflict with std::io in practice)
  - Doc examples marked as `ignore` (future APIs don't exist yet)
metrics:
  duration: 404 seconds
  completed: 2026-02-10T02:04:15Z
---

# Phase 01 Plan 02: Layer Hierarchy Scaffolding Summary

**One-liner:** Created 6-layer directory structure in trustedge-core with documented contracts defining what code belongs in each layer and strict dependency rules.

## What Was Built

Established the foundational directory structure for the layered architecture consolidation. Created 4 new module directories (primitives, protocols, applications, io) alongside 2 existing directories (backends, transport) to form the complete 6-layer hierarchy.

Each layer has comprehensive documentation specifying:
- What belongs in the layer (contract)
- Allowed dependencies (CAN import)
- Forbidden dependencies (NEVER imports)
- Future contents after Phase 2-8 migration
- Usage examples

## Tasks Completed

### Task 1: Create layer directories with contract documentation
**Commit:** 03cbb45
**Files:**
- `crates/core/src/primitives/mod.rs` (87 lines)
- `crates/core/src/protocols/mod.rs` (97 lines)
- `crates/core/src/applications/mod.rs` (81 lines)
- `crates/core/src/io/mod.rs` (73 lines)

Created 4 new directories with fully documented mod.rs files. Each module includes:
- MPL-2.0 copyright header
- Layer contract with dependency rules
- Post-consolidation migration plans
- Usage examples (marked `ignore` for future APIs)
- Status note: "Phase 1 scaffolding - no code yet"

**Layer definitions:**
- **primitives/** (Layer 1): Pure crypto primitives, no business logic
- **protocols/** (Layer 3): Wire formats, envelopes, chains, manifests
- **applications/** (Layer 4): Business logic (receipts, attestation, auth)
- **io/** (Layer 6): I/O adapters (audio, archives)

### Task 2: Declare new modules in lib.rs
**Commit:** db2e713
**Files:** `crates/core/src/lib.rs` (6 lines added)

Added 4 `pub mod` declarations to lib.rs:
```rust
// Layer hierarchy (Phase 1 scaffolding -- populated in later phases)
pub mod primitives;
pub mod protocols;
pub mod applications;
pub mod io;
```

Verified full workspace builds and all 150+ tests pass with no regressions.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Fixed doctest compilation errors**
- **Found during:** Task 2 verification
- **Issue:** Doc examples in layer mod.rs files failed compilation because they reference future APIs that don't exist yet (encrypt_segment, Receipt, etc.)
- **Fix:** Added `ignore` attribute to all doc examples (changed ````rust` to ````rust,ignore`)
- **Files modified:** All 4 layer mod.rs files
- **Commit:** 8e63d4d
- **Rationale:** Doctests compile by default even with `no_run`. Since these are scaffolding modules showing future APIs, `ignore` prevents compilation errors while preserving documentation value.

## Verification Results

All verification criteria met:

1. All 4 directories exist with mod.rs files
2. Each mod.rs contains layer contract documentation with "NEVER imports" rules
3. Each mod.rs documents post-consolidation contents
4. lib.rs declares all 4 new modules
5. `cargo build --workspace` compiles successfully
6. `cargo test --workspace` passes all 150+ tests
   - trustedge-core: 133 tests passed
   - trustedge-receipts: 23 tests passed
   - trustedge-trst-cli: 7 acceptance tests passed
   - All other crates: tests passed

## Self-Check: PASSED

**Created files verification:**
```
FOUND: crates/core/src/primitives/mod.rs
FOUND: crates/core/src/protocols/mod.rs
FOUND: crates/core/src/applications/mod.rs
FOUND: crates/core/src/io/mod.rs
```

**Commits verification:**
```
FOUND: 03cbb45 (Task 1 - layer directories)
FOUND: db2e713 (Task 2 - lib.rs declarations)
FOUND: 8e63d4d (Deviation fix - doctest ignore)
```

**Build verification:**
```
cargo build --workspace: SUCCESS
cargo test --workspace: ALL TESTS PASSED
```

## Impact

**Immediate:**
- 4 new empty modules in trustedge-core
- No code moves yet (Phase 1 is scaffolding only)
- Zero behavior changes
- Documentation roadmap for Phase 2-8 consolidation

**Future phases:**
- Layer contracts define clear boundaries for code migration
- Dependency rules prevent circular dependencies
- Post-consolidation documentation guides where each existing module will move

## Next Steps

Phase 01 Plan 03 will continue foundation work according to the roadmap. The layer hierarchy is now ready to receive code during Phase 2-8 consolidation.

**Key architectural decisions preserved:**
- Flat layout (directories sit alongside existing backends/ and transport/)
- 6-layer hierarchy: primitives -> backends -> protocols -> applications -> transport -> io
- Each layer has clear contract and dependency rules
- No src/layers/ parent directory (per user decision)

