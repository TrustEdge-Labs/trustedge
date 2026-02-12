---
phase: 14-ci-documentation
plan: 01
subsystem: ci-testing
tags: [ci, quality, dependency-tracking]

dependency-graph:
  requires: []
  provides:
    - tiered-ci-pipeline
    - dependency-tree-tracking
  affects:
    - ci-workflow
    - local-ci-script

tech-stack:
  added: []
  patterns:
    - tiered-validation-pipeline
    - non-blocking-experimental-checks
    - dependency-tree-size-baseline

key-files:
  created: []
  modified:
    - .github/workflows/ci.yml
    - scripts/ci-check.sh

decisions:
  - decision: Split CI into core (blocking) and experimental (non-blocking) tiers
    rationale: Prevents experimental crate issues from blocking core development while maintaining build visibility
    alternatives: [all-blocking, all-non-blocking, separate-workflows]
    timestamp: 2026-02-12
  - decision: Baseline dependency tree at 60 unique crates with +10 threshold
    rationale: Establishes early warning system for dependency bloat without hard-blocking
    alternatives: [no-tracking, hard-limit, percentage-based]
    timestamp: 2026-02-12

metrics:
  duration_seconds: 115
  files_modified: 2
  tests_added: 0
  completed_date: 2026-02-12
---

# Phase 14 Plan 01: Tiered CI Pipeline Summary

**One-liner:** Implemented tiered CI validation with core crates blocking merge and experimental crates non-blocking, plus dependency tree size baseline tracking at 60 unique crates.

## What Was Built

### Tiered CI Pipeline

**Core crates (Tier 1 - blocking):**
- trustedge-core
- trustedge-cli
- trustedge-trst-protocols
- trustedge-trst-cli
- trustedge-trst-wasm
- cam-video-example

**Experimental crates (Tier 2 - non-blocking):**
- trustedge-wasm
- trustedge-pubky
- trustedge-pubky-advanced
- trustedge-receipts
- trustedge-attestation

### Changes to `.github/workflows/ci.yml`

1. **Split clippy step**: Replaced single workspace clippy with two steps:
   - "clippy (core crates)" - blocking, no continue-on-error
   - "clippy (experimental crates - non-blocking)" - has `continue-on-error: true`

2. **Split test step**: Replaced single workspace test with two steps:
   - "tests (core crates)" - blocking, no continue-on-error
   - "tests (experimental crates - non-blocking)" - has `continue-on-error: true`

3. **Added dependency tree check**: New step after build that:
   - Counts unique crates via `cargo tree --workspace --depth 1 --prefix none --no-dedupe`
   - Establishes baseline of 60 unique crates
   - Warns (non-blocking) if count exceeds baseline + 10

4. **Preserved all existing steps**: audio, yubikey, all-features, cargo-hack, WASM, semver unchanged

### Changes to `scripts/ci-check.sh`

1. **Added `warn()` function**: New counter for non-blocking failures (WARN)

2. **Updated Step 3 (Clippy)**: Split into:
   - Core crates clippy (blocking with `fail()`)
   - Experimental crates clippy (non-blocking with `warn()`)

3. **Updated Step 7 (Build + test)**: Split into:
   - Workspace build (unchanged)
   - Core crate tests (blocking with `fail()`)
   - Experimental crate tests (non-blocking with `warn()`)

4. **Added Step 14 (Dependency tree)**: New step that:
   - Counts dependency tree size
   - Compares against baseline of 60
   - Warns (non-blocking) if exceeds threshold

5. **Updated summary output**: Now shows "Results: X passed, Y failed, Z warnings, W skipped"

## Deviations from Plan

None - plan executed exactly as written.

## Testing & Verification

**Verification checks performed:**
1. ✔ `bash -n scripts/ci-check.sh` - no syntax errors
2. ✔ ci.yml validates as valid YAML
3. ✔ Core crate steps do NOT have `continue-on-error`
4. ✔ Experimental crate steps HAVE `continue-on-error: true`
5. ✔ Dependency tree baseline set in both files (60)
6. ✔ All existing CI functionality preserved (audio, yubikey, WASM, semver, cargo-hack)

**Baseline establishment:**
- Current dependency tree: 60 unique crates
- Threshold: 70 crates (baseline + 10)
- Warning trigger: non-blocking information for developers

## Commits

| Task | Commit | Description |
|------|--------|-------------|
| 1 | 342ada0 | feat(14-01): implement tiered CI pipeline |
| 2 | f8e114a | feat(14-01): add dependency tree size baseline and tracking |

## Impact

**Developer workflow:**
- Core crate failures block PR merge (as before)
- Experimental crate failures visible but don't block
- Dependency growth tracked automatically
- Local ci-check.sh mirrors CI behavior

**CI reliability:**
- Core product stability ensured
- Experimental features get build visibility
- Dependency bloat prevention system established
- No regression in existing CI coverage

**Maintenance:**
- If experimental crate needs stabilization → move to core tier
- If dependency tree legitimately grows → update baseline in both files
- Warning-only approach allows flexibility without blocking development

## Self-Check: PASSED

**Files created:** None (all modifications)

**Files modified:** All exist
- ✔ FOUND: .github/workflows/ci.yml
- ✔ FOUND: scripts/ci-check.sh

**Commits:** All exist
- ✔ FOUND: 342ada0
- ✔ FOUND: f8e114a

## Success Criteria Met

- [x] CI-01: Core crates get comprehensive blocking CI; experimental crates build but don't block
- [x] CI-02: Dependency tree size baseline established and tracked
- [x] No regressions in existing CI behavior
- [x] Local ci-check.sh mirrors CI tiered approach

## Next Steps

None required. Plan complete. Ready to proceed to 14-02 (Documentation updates).
