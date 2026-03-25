---
phase: 61-ci-supply-chain-hardening
plan: "01"
subsystem: ci
tags: [security, ci, supply-chain, github-actions]
one_liner: "SHA-pin all GitHub Actions across 4 workflows, remove curl-pipe wasm-pack installer, replace archived actions-rs/toolchain with dtolnay/rust-toolchain"
dependency_graph:
  requires: []
  provides: [CISC-01, CISC-02, CISC-03]
  affects: [.github/workflows/wasm-tests.yml, .github/workflows/ci.yml, .github/workflows/semver.yml, .github/workflows/cla.yml]
tech_stack:
  added: []
  patterns: [sha-pinned-actions, cargo-binstall]
key_files:
  created: []
  modified:
    - .github/workflows/wasm-tests.yml
    - .github/workflows/ci.yml
    - .github/workflows/semver.yml
    - .github/workflows/cla.yml
decisions:
  - "Use taiki-e/install-action + cargo binstall for wasm-pack instead of curl|sh — verifiable SHA-pinned binary install"
  - "dtolnay/rust-toolchain uses targets: (plural) not target: and does not need override: true"
  - "All SHA pins include version comments (e.g., # v4) for human readability"
metrics:
  duration_seconds: 72
  completed_date: "2026-03-25"
  tasks_completed: 2
  tasks_total: 2
  files_modified: 4
---

# Phase 61 Plan 01: CI Supply Chain Hardening Summary

## One-Liner

SHA-pin all GitHub Actions across 4 workflows, remove curl-pipe wasm-pack installer, replace archived actions-rs/toolchain with dtolnay/rust-toolchain.

## What Was Built

Hardened all 4 GitHub Actions workflow files against supply chain attacks by closing 3 P0 security review findings:

**CISC-01 (curl|sh code execution):** Replaced `curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh` in both `wasm-size-check` and `wasm-build-check` jobs with `taiki-e/install-action@SHA # cargo-binstall` + `cargo binstall wasm-pack --no-confirm`. The installer is now a SHA-pinned action rather than an arbitrary shell script fetched at runtime.

**CISC-02 (tag-based action hijacking):** Every `uses:` line across all 4 workflows now references a full 40-character commit SHA with a version comment. Previously all actions used mutable tags (`@v4`, `@stable`, `@v2`, `@v2.6.1`) that could be overwritten by a compromised upstream repo. Pinned references:
- `actions/checkout@34e114876b0b11c390a56381ad16ebd13914f8d5 # v4` (6 occurrences)
- `dtolnay/rust-toolchain@631a55b12751854ce901bb631d5902ceb48146f7 # stable` (4 occurrences)
- `Swatinem/rust-cache@e18b497796c12c097a38f9edb9d0641fb99eee32 # v2` (2 occurrences)
- `taiki-e/install-action@5e35b238ee3d0a48aa25ae877adcff74a6ddd2e0 # cargo-audit` (1 occurrence)
- `taiki-e/install-action@1d6f37b1831936b8fe2d9feeea23d1da7f387001 # cargo-semver-checks` (1 occurrence)
- `taiki-e/install-action@0863870cb2b3e98da71d6f925d05ecf26c4b96c1 # cargo-binstall` (2 occurrences)
- `contributor-assistant/github-action@ca4a40a7d1004f18d9960b404b97e5f30a505a08 # v2.6.1` (1 occurrence)

**CISC-03 (archived actions-rs/toolchain):** Replaced `actions-rs/toolchain@v1` in both wasm-tests.yml jobs with `dtolnay/rust-toolchain@SHA`. Migrated `target:` parameter to `targets:` (plural) and removed `override: true` (not needed with dtolnay action).

## Commits

| Task | Description | Commit |
|------|-------------|--------|
| 1 | Harden wasm-tests.yml | 1be3e6a |
| 2 | SHA-pin ci.yml, semver.yml, cla.yml | 783849c |

## Verification Results

Post-execution comprehensive check:
- `grep -rn 'uses:' .github/workflows/ | grep -v '@[0-9a-f]\{40\}'` → empty (all pinned)
- `grep -rn 'curl.*|.*sh' .github/workflows/` → empty
- `grep -rn 'actions-rs' .github/workflows/` → empty
- All 4 files preserve copyright header `Copyright (c) 2025 TRUSTEDGE LABS LLC`

## Deviations from Plan

None — plan executed exactly as written.

## Known Stubs

None.

## Self-Check: PASSED

- `.github/workflows/wasm-tests.yml` — exists, verified
- `.github/workflows/ci.yml` — exists, verified
- `.github/workflows/semver.yml` — exists, verified
- `.github/workflows/cla.yml` — exists, verified
- Commit `1be3e6a` — verified in git log
- Commit `783849c` — verified in git log
