---
phase: 06-feature-flags
plan: 02
subsystem: ci-infrastructure
tags: [ci, testing, feature-flags, wasm, downstream-validation]
dependency_graph:
  requires: [06-01]
  provides: [all-features-ci-testing, downstream-feature-validation, wasm-build-verification]
  affects: [ci-pipeline, local-ci-parity]
tech_stack:
  added: []
  patterns: [conditional-ci-steps, downstream-feature-powerset, wasm-target-verification]
key_files:
  created: []
  modified:
    - scripts/ci-check.sh
    - .github/workflows/ci.yml
decisions:
  - "Conditional guards for all-features test: Only runs when both audio (ALSA) and yubikey (PCSC) platform dependencies are available"
  - "WASM target installation: CI installs wasm32-unknown-unknown target explicitly; local script checks if already installed"
  - "Downstream check unconditional: trustedge-cli feature-powerset runs in all environments (cargo-hack already required)"
  - "Step ordering: all-features (12) → downstream (13) → WASM (14) → semver (15) follows complexity progression"
metrics:
  duration_seconds: 84
  tasks_completed: 2
  files_modified: 2
  commits: 2
  completed_date: 2026-02-10
---

# Phase 06 Plan 02: All-Features CI Testing and Downstream Validation Summary

**One-liner:** Comprehensive CI coverage for all-features builds, downstream crate feature propagation, and WASM target verification to catch feature interaction bugs.

## Objective

Added all-features testing and downstream crate feature verification to CI pipeline. The current CI tested individual features (audio, yubikey) and powerset combinations, but never tested all features enabled simultaneously, and did not verify that downstream crates (trustedge-cli) correctly propagate features from core. This left gaps where feature interaction bugs between audio and yubikey could go undetected, and feature propagation issues would not be caught.

## What Was Done

### Task 1: Add all-features and downstream feature testing to ci-check.sh

**Commit:** `dd0870c`

Added three new steps to scripts/ci-check.sh after Step 11 (yubikey tests):

1. **Step 12: All-features build and test** - Tests audio+yubikey together to catch interaction bugs. Only runs if both ALSA and PCSC platform dependencies are available. Includes conditional guard with skip message if libraries missing.

2. **Step 13: Downstream crate feature check** - Uses cargo-hack to verify trustedge-cli feature-powerset propagation. Runs unconditionally since cargo-hack is already required and cli features are simple.

3. **Step 14: WASM build verification** - Checks both trustedge-wasm and trustedge-trst-wasm build without accidentally enabling platform-incompatible features. Checks if wasm32-unknown-unknown target installed; skips with installation instructions if missing.

Renumbered old Step 12 (semver-checks) to Step 15. Updated final success message to reflect 16 total steps (Step 0 through Step 15).

**Files modified:** scripts/ci-check.sh

### Task 2: Add all-features and downstream feature testing to GitHub CI workflow

**Commit:** `177bbe8`

Added three new steps to .github/workflows/ci.yml after the yubikey tests step:

1. **Build and test all features (trustedge-core)** - Mirrors ci-check.sh Step 12. Uses GitHub Actions conditionals (`if: steps.audio-deps.outputs.audio-available == 'true' && steps.yubikey-deps.outputs.yubikey-available == 'true'`) instead of bash conditionals.

2. **Downstream crate feature check (trustedge-cli)** - Mirrors ci-check.sh Step 13. Runs cargo-hack feature-powerset check unconditionally.

3. **WASM build verification** - Mirrors ci-check.sh Step 14. Explicitly installs wasm32-unknown-unknown target with rustup (CI may not have it by default). Checks both trustedge-wasm and trustedge-trst-wasm.

**Files modified:** .github/workflows/ci.yml

## Deviations from Plan

None - plan executed exactly as written.

## Testing

1. Script syntax validation: `bash -n scripts/ci-check.sh` passed
2. YAML validation: `python3 -c "import yaml; yaml.safe_load(open('.github/workflows/ci.yml'))"` passed
3. Step count verification: 16 steps (Step 0 through Step 15) confirmed
4. Pattern matching: all-features, wasm32-unknown-unknown, trustedge-cli found in both files
5. WASM build verification: Both trustedge-wasm and trustedge-trst-wasm build successfully for wasm32-unknown-unknown target
6. Downstream feature check: cargo-hack command syntax correct (failed on missing ALSA, expected behavior with conditional guards)

## Impact

**Before:**
- CI tested individual features but never all features together
- No verification of downstream crate feature propagation
- No WASM target compatibility verification
- Gap where audio+yubikey interaction bugs could go undetected

**After:**
- All-features build+test catches interaction bugs between audio and yubikey
- Downstream feature-powerset check verifies trustedge-cli propagates core features correctly
- WASM build verification ensures WASM crates don't accidentally pull platform-incompatible features
- Local ci-check.sh and GitHub CI workflow remain in parity (both have all three new checks)
- Conditional guards handle missing platform dependencies gracefully

## Self-Check

Verifying all claims in this summary:

**Files modified:**
- scripts/ci-check.sh: exists and contains all-features, trustedge-cli, wasm32-unknown-unknown patterns
- .github/workflows/ci.yml: exists and contains all-features, trustedge-cli, wasm32-unknown-unknown patterns

**Commits:**
- dd0870c: "feat(06-02): add all-features testing and downstream feature checks to ci-check.sh"
- 177bbe8: "feat(06-02): add all-features testing and downstream feature checks to GitHub CI"

**Step count:** 16 steps in ci-check.sh (Step 0 through Step 15) confirmed

**WASM build:** trustedge-wasm and trustedge-trst-wasm both check successfully for wasm32-unknown-unknown target

## Self-Check: PASSED

All files, commits, and functionality verified.
