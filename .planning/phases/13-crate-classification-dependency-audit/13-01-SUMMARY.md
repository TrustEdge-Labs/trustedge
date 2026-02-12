---
phase: 13
plan: 01
subsystem: crate-organization
tags: [classification, metadata, documentation, facades]
dependency-graph:
  requires: []
  provides:
    - tier-metadata-in-cargo-toml
    - workspace-tier-documentation
    - experimental-readme-banners
    - stable-readme-markers
    - facade-reclassification
  affects:
    - all-10-workspace-crates
tech-stack:
  added: []
  patterns:
    - cargo-package-metadata
    - tier-based-classification
    - readme-banners
key-files:
  created:
    - crates/trustedge-cli/README.md
    - crates/trst-protocols/README.md
    - crates/trst-cli/README.md
  modified:
    - Cargo.toml
    - crates/core/Cargo.toml
    - crates/trustedge-cli/Cargo.toml
    - crates/trst-protocols/Cargo.toml
    - crates/trst-cli/Cargo.toml
    - crates/trst-wasm/Cargo.toml
    - crates/wasm/Cargo.toml
    - crates/pubky/Cargo.toml
    - crates/pubky-advanced/Cargo.toml
    - crates/receipts/Cargo.toml
    - crates/attestation/Cargo.toml
    - crates/core/README.md
    - crates/trst-wasm/README.md
    - crates/wasm/README.md
    - crates/pubky/README.md
    - crates/pubky-advanced/README.md
    - crates/receipts/README.md
    - crates/attestation/README.md
decisions:
  - decision: "Use [package.metadata.trustedge] for tier classification"
    rationale: "Cargo metadata is standard mechanism for crate-specific metadata, queryable via cargo metadata command"
  - decision: "Reclassify facade crates from deprecated to experimental"
    rationale: "v1.2 is 'mark not destroy' - no deletion, just clear maintenance boundaries"
  - decision: "Use README banners for user-visible tier communication"
    rationale: "Users see README first on crates.io and GitHub, need immediate clarity on maintenance status"
metrics:
  duration: "~6 minutes"
  tasks-completed: 3
  commits: 3
  files-changed: 21
  completed-date: "2026-02-12"
---

# Phase 13 Plan 01: Crate Classification with Tier Metadata

**One-liner:** Established 2-tier crate classification (5 stable, 5 experimental) with Cargo metadata, workspace documentation, and README banners for clear maintenance boundaries.

## What Was Built

### Task 1: Tier Metadata and Workspace Documentation
- Added `[package.metadata.trustedge]` sections to all 10 crates
- 5 stable crates: `tier = "stable"`, `maintained = true`
- 5 experimental crates: `tier = "experimental"`, `maintained = false`
- Documented 2-tier system in workspace Cargo.toml with inline comments
- Reclassified facade crates (receipts, attestation) from deprecated to experimental
- Updated descriptions and keywords for receipts and attestation crates

### Task 2: Experimental Crate Banners
- Added prominent "EXPERIMENTAL" banners to 5 tier 2 crate READMEs
- wasm: directs users to trustedge-trst-wasm for browser verification
- pubky/pubky-advanced: community-contributed, no maintenance commitment
- receipts/attestation: replaced deprecation notices with experimental status
- Banners clarify tier classification and maintenance expectations

### Task 3: Stable Crate Markers
- Added "STABLE" banners to existing READMEs (core, trst-wasm)
- Created new READMEs with stable markers for 3 crates:
  - trustedge-cli: main CLI binary
  - trst-protocols: archive format definitions
  - trst-cli: archive CLI binary
- All 5 stable crate READMEs now have "Tier 1 (Stable)" marker

## Requirements Satisfied

- **CLSF-01**: 5 core crates marked stable in Cargo.toml metadata and README markers
- **CLSF-02**: 5 experimental crates marked experimental in Cargo.toml metadata and README banners
- **CLSF-03**: Workspace Cargo.toml documents the 2-tier classification
- **CLSF-04**: Facade crates reclassified from deprecated to experimental

## Verification Results

All verification criteria passed:

1. ✅ `cargo check --workspace` passes (no TOML errors)
2. ✅ All 10 crate Cargo.toml files have `[package.metadata.trustedge]` with correct tier (5 stable, 5 experimental, 0 missing)
3. ✅ Workspace Cargo.toml has tier documentation comment block
4. ✅ Facade crates (receipts, attestation) have experimental descriptions, not deprecated
5. ✅ All 5 experimental crate READMEs have "EXPERIMENTAL" banner
6. ✅ All 5 stable crate Cargo.toml files have `tier = "stable"`
7. ✅ All 5 stable crate READMEs have "Tier 1 (Stable)" marker

## Deviations from Plan

None - plan executed exactly as written.

## Commits

| Hash | Message |
|------|---------|
| 383b787 | feat(13-01): add tier metadata and reclassify facade crates |
| b4d1877 | docs(13-01): add experimental banners to tier 2 crate READMEs |
| 11e2002 | docs(13-01): add stable markers to tier 1 crate READMEs |

## Impact

**Maintainability:** Clear boundaries between production-committed (stable) and community/experimental crates. Solo developer can focus on 5 stable crates with confidence.

**User clarity:** README banners and Cargo.toml metadata provide immediate visibility into maintenance status. Users know which crates have maintenance commitment.

**Foundation for Phase 14:** Tier classification enables CI prioritization (stable: full tests + build; experimental: build-only).

**Facade crates:** Reclassified from deprecated to experimental with no maintenance commitment. Users directed to trustedge-core for production use.

## Self-Check: PASSED

**Created files exist:**
- ✅ crates/trustedge-cli/README.md
- ✅ crates/trst-protocols/README.md
- ✅ crates/trst-cli/README.md

**Commits exist:**
- ✅ 383b787 (tier metadata)
- ✅ b4d1877 (experimental banners)
- ✅ 11e2002 (stable markers)

**Tier metadata verification:**
- ✅ 5 stable crates: core, trustedge-cli, trst-protocols, trst-cli, trst-wasm
- ✅ 5 experimental crates: wasm, pubky, pubky-advanced, receipts, attestation

**README verification:**
- ✅ All 5 experimental crates have "EXPERIMENTAL" banner
- ✅ All 5 stable crates have "Tier 1 (Stable)" marker

**Workspace documentation:**
- ✅ Cargo.toml has tier classification comment block

All claims verified. Plan executed successfully.
