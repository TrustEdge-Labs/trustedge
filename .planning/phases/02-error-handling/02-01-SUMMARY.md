<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


---
phase: 02-error-handling
plan: 01
subsystem: error-handling
tags: [error-hierarchy, refactor, foundation]
dependency_graph:
  requires: [01-foundation]
  provides: [unified-error-module, error-namespace-resolution]
  affects: [hybrid, envelope_v2_bridge, pubky]
tech_stack:
  added: [thiserror error hierarchy]
  patterns: [unified error enum, subsystem error types]
key_files:
  created:
    - crates/core/src/error.rs
  modified:
    - crates/core/src/lib.rs
    - crates/core/src/hybrid.rs
    - crates/core/src/envelope_v2_bridge.rs
    - crates/pubky/src/lib.rs
decisions:
  - slug: error-hierarchy-structure
    summary: "TrustEdgeError as top-level enum with 9 variants (7 subsystem + Io + Json)"
    rationale: "Unified error type enables clean error propagation across all subsystems"
  - slug: hybrid-error-rename
    summary: "Renamed hybrid.rs TrustEdgeError to HybridEncryptionError"
    rationale: "Eliminates namespace collision with unified error.rs TrustEdgeError"
  - slug: asymmetric-backend-error-string
    summary: "Changed AsymmetricError::BackendError from anyhow::Error to String"
    rationale: "anyhow::Error with #[from] cannot be nested in thiserror enum hierarchy"
metrics:
  duration: "4 minutes"
  completed_date: "2026-02-10"
  task_count: 2
  test_count: 348
  test_result: "348 passed, 0 failed"
---

# Phase 02 Plan 01: Error Hierarchy Foundation Summary

**One-liner:** Created unified error.rs module with TrustEdgeError hierarchy and resolved hybrid.rs namespace collision by renaming to HybridEncryptionError.

## Overview

This plan established the foundational error hierarchy for Phase 2's error consolidation work. It created a new `error.rs` module containing the top-level `TrustEdgeError` enum and all 7 subsystem error types, then resolved the naming conflict by renaming `hybrid.rs`'s `TrustEdgeError` to `HybridEncryptionError`.

## Tasks Completed

### Task 1: Create error.rs with unified error hierarchy

**Commit:** `96cb2ff`

Created `crates/core/src/error.rs` with:
- **TrustEdgeError** — Top-level enum with 9 variants: `Crypto`, `Backend`, `Transport`, `Archive`, `Manifest`, `Chain`, `Asymmetric`, `Io`, `Json`
- **CryptoError** — 7 variants (copied from crypto.rs)
- **ChainError** — 3 variants (copied from chain.rs)
- **AsymmetricError** — 4 variants (adapted from asymmetric.rs with BackendError changed to String)
- **ManifestError** — 2 variants (copied from manifest.rs)
- **ArchiveError** — 9 variants (copied from archive.rs)
- **BackendError** — NEW, 5 variants for backend operations
- **TransportError** — NEW, 6 variants for transport layer

All errors use `thiserror::Error`. Module declared in `lib.rs` (no public re-exports yet).

**Files modified:**
- `crates/core/src/error.rs` (created, 179 lines)
- `crates/core/src/lib.rs` (added `pub mod error;`)

### Task 2: Rename hybrid.rs TrustEdgeError to HybridEncryptionError

**Commit:** `8ea9e0d`

Renamed the error type throughout the codebase:
- `hybrid.rs`: Enum definition + all function return types + error construction sites
- `lib.rs`: Updated public re-export
- `envelope_v2_bridge.rs`: Updated imports and all usage sites
- `pubky/src/lib.rs`: Updated error variant from `trustedge_core::TrustEdgeError` to `HybridEncryptionError`

**Verification:** `grep -r "pub enum TrustEdgeError" crates/` returns exactly one match (error.rs only).

**Files modified:**
- `crates/core/src/hybrid.rs` (27 replacements)
- `crates/core/src/lib.rs` (re-export updated)
- `crates/core/src/envelope_v2_bridge.rs` (6 replacements)
- `crates/pubky/src/lib.rs` (1 replacement)

## Deviations from Plan

None — plan executed exactly as written.

## Technical Details

### Error Hierarchy Design

```
TrustEdgeError (top-level)
├── Crypto(CryptoError)          — Encryption/signing operations
├── Backend(BackendError)        — HSM/keyring/hardware operations
├── Transport(TransportError)    — Network communication
├── Archive(ArchiveError)        — .trst archive operations
├── Manifest(ManifestError)      — Manifest serialization
├── Chain(ChainError)            — Continuity chain validation
├── Asymmetric(AsymmetricError)  — Public key operations
├── Io(std::io::Error)           — File system operations
└── Json(serde_json::Error)      — JSON serialization
```

### Multiple #[from] Paths

The hierarchy has multiple `#[from]` conversions for `std::io::Error` and `serde_json::Error`:
- `ArchiveError::Io(#[from] std::io::Error)` creates `From<io::Error> for ArchiveError`
- `TrustEdgeError::Io(#[from] std::io::Error)` creates `From<io::Error> for TrustEdgeError`
- `TransportError::Io(#[from] std::io::Error)` creates `From<io::Error> for TransportError`

These do NOT conflict because they are on different types. The compiler generates separate `From` implementations for each.

### AsymmetricError Adaptation

The original `AsymmetricError::BackendError(#[from] anyhow::Error)` was changed to `BackendError(String)` because:
- `anyhow::Error` with `#[from]` creates `From<anyhow::Error> for AsymmetricError`
- When `AsymmetricError` is nested in `TrustEdgeError` with `#[from]`, the error message is preserved via `Display` but the source chain is lost
- Using `String` is clearer and avoids nested `anyhow` types in a `thiserror` hierarchy

## Verification

- `cargo build --workspace` — Compiles with no errors
- `cargo test --workspace` — 348 tests passed, 0 failed
- `grep -r "pub enum TrustEdgeError" crates/` — Returns exactly 1 match (error.rs)
- Name collision resolved: `TrustEdgeError` is exclusively in `error.rs`, hybrid uses `HybridEncryptionError`

## Next Steps

**Plan 02:** Migrate existing code to use new error types (update function signatures, convert error construction sites).

**Plan 03:** Update public API re-exports and add workspace-level error re-exports.

## Self-Check: PASSED

**Created files exist:**
```
FOUND: crates/core/src/error.rs
```

**Modified files exist:**
```
FOUND: crates/core/src/lib.rs
FOUND: crates/core/src/hybrid.rs
FOUND: crates/core/src/envelope_v2_bridge.rs
FOUND: crates/pubky/src/lib.rs
```

**Commits exist:**
```
FOUND: 96cb2ff (feat(02-01): create unified error hierarchy module)
FOUND: 8ea9e0d (refactor(02-01): rename hybrid TrustEdgeError to HybridEncryptionError)
```

**Workspace compiles:** YES (cargo build --workspace succeeded)

**Tests pass:** YES (348 tests passed, 0 failed)

**Namespace collision resolved:** YES (grep returns 1 match for "pub enum TrustEdgeError")
