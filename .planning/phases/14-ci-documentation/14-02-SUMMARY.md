---
phase: 14
plan: 02
subsystem: documentation
tags: [readme, classification, tier-system, user-documentation]
dependency-graph:
  requires:
    - phase-13-tier-classification
    - phase-13-readme-banners
  provides:
    - root-readme-crate-classification
    - visible-tier-documentation
  affects:
    - root-readme
    - user-onboarding
tech-stack:
  added: []
  patterns:
    - tier-based-documentation
    - structured-crate-tables
key-files:
  created: []
  modified:
    - README.md
decisions: []
metrics:
  duration: "74 seconds (~1 minute)"
  tasks-completed: 2
  commits: 1
  files-changed: 1
  completed-date: "2026-02-12"
---

# Phase 14 Plan 02: Root README Crate Classification

**One-liner:** Updated root README with visible 2-tier crate classification section and enhanced crate table, making stable vs experimental split immediately clear to users.

## What Was Built

### Task 1: Add Crate Classification to Root README
- Added v1.2 version history entry with crate classification, dependency audit, and tiered CI
- Created new "Crate Classification" section with 2-tier comparison table
  - Tier 1 (Stable): 5 crates with full CI and active maintenance
  - Tier 2 (Experimental): 5 crates with build-only CI and no maintenance commitment
- Enhanced existing "Crate Overview" table with new "Tier" column
- Clearly labeled all 10 crates as "Stable" or "Experimental"
- Positioned classification section prominently after architecture diagram, before crate table

### Task 2: Verify Experimental Crate README Banners
- Verified all 5 experimental crate READMEs retain "EXPERIMENTAL" banners from Phase 13
- Confirmed no regression: wasm, pubky, pubky-advanced, receipts, attestation all have banners
- No file changes needed (verification task only)

## Requirements Satisfied

- **DOCS-01**: Root README clearly documents stable vs experimental crate split with dedicated section
- **DOCS-02**: All experimental crate READMEs have experimental/beta banner (verified, no regression)
- README changes are additive (no existing content removed or altered)

## Verification Results

All verification criteria passed:

1. ✅ README.md contains "Crate Classification" section with tier table
2. ✅ README.md crate overview table includes "Tier" column
3. ✅ "Tier 1" appears for 5 stable crates (core, cli, trst-protocols, trst-cli, trst-wasm)
4. ✅ "Experimental" appears for 5 experimental crates
5. ✅ v1.2 mentioned in version history
6. ✅ All 5 experimental crate READMEs have "EXPERIMENTAL" banner (grep verified)
7. ✅ No unrelated README content changed

## Deviations from Plan

None - plan executed exactly as written.

## Commits

| Hash | Message |
|------|---------|
| e5e6a8d | docs(14-02): add crate classification section to root README |

## Impact

**User clarity:** Users landing on the root README immediately see the 2-tier classification system before diving into individual crates. The new section answers "Which crates should I use for production?" upfront.

**Onboarding improvement:** New contributors and users can quickly identify stable crates (core, CLI tools) vs experimental/community crates without needing to read individual READMEs.

**Consistency:** Root README now aligns with the Cargo.toml metadata and individual crate READMEs established in Phase 13, creating a cohesive tier system across all documentation.

**Foundation for Phase 14:** Documentation updates support the tiered CI pipeline implementation (Plan 14-01) by making tier boundaries visible to all stakeholders.

## Self-Check: PASSED

**Modified files exist:**
- ✅ README.md exists and contains changes

**Commits exist:**
- ✅ e5e6a8d (crate classification section)

**Content verification:**
- ✅ "Crate Classification" section present at line 151
- ✅ "Tier 1" appears in classification table
- ✅ "v1.2 Scope Reduction" appears in version history at line 120
- ✅ Crate overview table has 4 columns (Crate, Purpose, Tier, Documentation)
- ✅ All 5 stable crates labeled "Stable" in table
- ✅ All 5 experimental crates labeled "Experimental" in table

**Experimental banner verification:**
- ✅ crates/wasm/README.md has "EXPERIMENTAL" banner
- ✅ crates/pubky/README.md has "EXPERIMENTAL" banner
- ✅ crates/pubky-advanced/README.md has "EXPERIMENTAL" banner
- ✅ crates/receipts/README.md has "EXPERIMENTAL" banner
- ✅ crates/attestation/README.md has "EXPERIMENTAL" banner

All claims verified. Plan executed successfully.
