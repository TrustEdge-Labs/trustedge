# Roadmap: TrustEdge Consolidation

## Overview

TrustEdge consolidates from 10 crates to a monolithic core with thin shells. This roadmap transforms a scattered workspace into a focused library by merging duplicated functionality (receipts, attestation, archives) into trustedge-core while preserving all 150+ tests and maintaining WASM compatibility. The journey follows dependency order: establish foundation and unified errors, merge WASM-safe code first, integrate dependent systems in order, consolidate feature flags, preserve backward compatibility, and validate exhaustively.

## Phases

**Phase Numbering:**
- Integer phases (1, 2, 3): Planned milestone work
- Decimal phases (2.1, 2.2): Urgent insertions (marked with INSERTED)

Decimal phases appear between their surrounding integers in numeric order.

- [x] **Phase 1: Foundation** - Baseline and module hierarchy ✓
- [x] **Phase 2: Error Handling** - Unified error types ✓
- [x] **Phase 3: trst-core Integration** - WASM-safe manifest types ✓
- [x] **Phase 4: Receipts Integration** - Digital receipt system ✓
- [x] **Phase 5: Attestation Integration** - Software attestation ✓
- [x] **Phase 6: Feature Flags** - Consolidate feature architecture ✓
- [ ] **Phase 7: Backward Compatibility** - Re-export facades
- [ ] **Phase 8: Validation** - Comprehensive testing

## Phase Details

### Phase 1: Foundation
**Goal**: Establish baseline metrics and module hierarchy before making changes
**Depends on**: Nothing (first phase)
**Requirements**: FOUND-01, FOUND-02, FOUND-03
**Success Criteria** (what must be TRUE):
  1. Dependency graph visualization exists showing all cross-crate dependencies
  2. Test inventory baseline documents exact count per crate (150+ total accounted for)
  3. Layered module skeleton exists in trustedge-core (primitives/backends/protocols/applications/transport/io)
  4. API surface snapshot captured for semver validation
**Plans**: 4 plans

Plans:
- [x] 01-01-PLAN.md — Install analysis tools and integrate into CI
- [x] 01-02-PLAN.md — Create layered module hierarchy scaffolding
- [x] 01-03-PLAN.md — Generate test baseline and workspace dependency graph
- [x] 01-04-PLAN.md — Conduct duplication audit and capture API baselines

### Phase 2: Error Handling
**Goal**: Unified error type hierarchy across all crates
**Depends on**: Phase 1
**Requirements**: ERR-01, ERR-02, ERR-03
**Success Criteria** (what must be TRUE):
  1. Single TrustEdgeError enum exists with subsystem variants (Crypto, Backend, Transport, Archive, Manifest)
  2. All 10+ duplicate error types consolidated into hierarchy
  3. Library code uses thiserror, CLI binaries use anyhow propagation
  4. Error conversion paths preserve context (no information loss)
**Plans**: 3 plans

Plans:
- [x] 02-01-PLAN.md — Create unified error hierarchy and resolve name collision
- [x] 02-02-PLAN.md — Migrate core module error definitions to error.rs
- [x] 02-03-PLAN.md — Migrate backend traits to BackendError and finalize public API

### Phase 3: trst-core Integration
**Goal**: Archive manifest types merged into core while preserving WASM compatibility
**Depends on**: Phase 2
**Requirements**: INTG-01, INTG-02
**Success Criteria** (what must be TRUE):
  1. Manifest types exist in trustedge-core applications/archives/manifest/
  2. Duplicate ManifestError between core and trst-core resolved into unified type
  3. WASM build succeeds (cargo check --target wasm32-unknown-unknown)
  4. trst-cli and trst-wasm updated to import from core (no functionality loss)
**Plans**: 2 plans

Plans:
- [x] 03-01-PLAN.md — Rename trst-core to trst-protocols and restructure into domain submodules
- [x] 03-02-PLAN.md — Wire core to trst-protocols, delete duplicate manifest.rs, update consumers

### Phase 4: Receipts Integration
**Goal**: Digital receipt system merged into core
**Depends on**: Phase 3
**Requirements**: INTG-03
**Success Criteria** (what must be TRUE):
  1. Receipt logic (1,281 LOC) exists in trustedge-core applications/receipts/
  2. All 23 receipt tests preserved and passing
  3. Receipt operations available through core API (no separate crate needed)
  4. No circular dependencies introduced (verified by cargo-modules)
**Plans**: 1 plan

Plans:
- [x] 04-01-PLAN.md — Move receipts into core, convert demo to example, validate workspace

### Phase 5: Attestation Integration
**Goal**: Software attestation merged into core
**Depends on**: Phase 4
**Requirements**: INTG-04
**Success Criteria** (what must be TRUE):
  1. Attestation logic exists in trustedge-core applications/attestation/
  2. Attestation tests preserved and passing
  3. Envelope integration unified (no feature flag drift)
  4. Provenance tracking available through core API
**Plans**: 1 plan

Plans:
- [x] 05-01-PLAN.md — Move attestation into core, convert binaries to examples, validate workspace

### Phase 6: Feature Flags
**Goal**: Unified feature flag architecture preventing combinatorial explosion
**Depends on**: Phase 5
**Requirements**: FEAT-01, FEAT-02
**Success Criteria** (what must be TRUE):
  1. Features organized into categories (backend, platform, format)
  2. CI matrix tests critical combinations (default, yubikey, audio, all-features)
  3. Feature documentation exists explaining when to use each flag
  4. No per-subsystem feature flags like receipt-ops (avoid explosion)
**Plans**: 2 plans

Plans:
- [x] 06-01-PLAN.md — Add feature documentation, doc(cfg) annotations, and docs.rs metadata
- [x] 06-02-PLAN.md — Add all-features CI testing, downstream feature checks, and WASM verification

### Phase 7: Backward Compatibility
**Goal**: Preserve public API surface during transition
**Depends on**: Phase 6
**Requirements**: COMPAT-01, COMPAT-02
**Success Criteria** (what must be TRUE):
  1. Deprecated re-export facades created for merged crates (receipts, attestation, trst-core)
  2. Migration guide documents import path changes (old -> new)
  3. All thin shells (trustedge-cli, trst-cli, wasm, trst-wasm) build successfully
  4. Deprecation warnings visible but not breaking (6-month migration window)
**Plans**: TBD

Plans:
- [ ] TBD

### Phase 8: Validation
**Goal**: Comprehensive validation of consolidation preserving all functionality
**Depends on**: Phase 7
**Requirements**: VAL-01, VAL-02, VAL-03
**Success Criteria** (what must be TRUE):
  1. Test count validation passes (150+ tests preserved, exact match to baseline)
  2. WASM build succeeds (cargo check --target wasm32-unknown-unknown)
  3. No API breakage detected (cargo semver-checks passes)
  4. YubiKey hardware integration documented (manual test protocol if hardware unavailable)
  5. Build time measured and within acceptable bounds (<2x baseline)
**Plans**: TBD

Plans:
- [ ] TBD

## Progress

**Execution Order:**
Phases execute in numeric order: 1 -> 2 -> 3 -> 4 -> 5 -> 6 -> 7 -> 8

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. Foundation | 4/4 | ✓ Complete | 2026-02-10 |
| 2. Error Handling | 3/3 | ✓ Complete | 2026-02-09 |
| 3. trst-core Integration | 2/2 | ✓ Complete | 2026-02-10 |
| 4. Receipts Integration | 1/1 | ✓ Complete | 2026-02-10 |
| 5. Attestation Integration | 1/1 | ✓ Complete | 2026-02-10 |
| 6. Feature Flags | 2/2 | ✓ Complete | 2026-02-10 |
| 7. Backward Compatibility | 0/TBD | Not started | - |
| 8. Validation | 0/TBD | Not started | - |
