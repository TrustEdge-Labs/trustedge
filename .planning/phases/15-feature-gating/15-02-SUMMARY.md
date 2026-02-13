---
phase: 15-feature-gating
plan: 02
subsystem: ci-infrastructure
tags:
  - ci-testing
  - feature-gating
  - quality-assurance
dependency_graph:
  requires:
    - scripts/ci-check.sh
    - .github/workflows/ci.yml
    - git-attestation feature (15-01)
    - keyring feature (15-01)
  provides:
    - CI validation for git-attestation feature
    - CI validation for keyring feature
  affects:
    - Local CI script (ci-check.sh)
    - GitHub Actions CI pipeline
tech_stack:
  added: []
  patterns:
    - Feature-specific test steps in CI pipeline
    - Blocking CI tests for core features
    - Parallel feature validation in both local and GitHub CI
key_files:
  created: []
  modified:
    - scripts/ci-check.sh
    - .github/workflows/ci.yml
decisions:
  - git-attestation and keyring CI steps are blocking (not continue-on-error) since they are Tier 1 core features
  - No system dependencies required for git-attestation or keyring, so no conditional execution needed
  - Local script mirrors GitHub Actions CI behavior for consistency
  - Step renumbering applied sequentially after insertions to maintain clean numbering
metrics:
  duration: 97 seconds (1.6 minutes)
  completed: 2026-02-13T02:35:29Z
  tasks: 2
  commits: 2
---

# Phase 15 Plan 02: CI Pipeline Feature Testing

CI pipeline validates both default (no features) and feature-enabled builds for git-attestation and keyring, preventing silent regressions

## One-liner

Added dedicated clippy and test steps for git-attestation and keyring features to both local CI script and GitHub Actions

## Tasks Completed

### Task 1: Update local CI script with feature-gated test steps

**Files modified:**
- `scripts/ci-check.sh`

**Implementation:**
- Added Step 6: Clippy (trustedge-core with git-attestation)
- Added Step 7: Clippy (trustedge-core with keyring)
- Added Step 12: Tests (trustedge-core with git-attestation)
- Added Step 13: Tests (trustedge-core with keyring)
- Renumbered all subsequent steps sequentially (Steps 8-18)

**New clippy steps:**
```bash
step "Step 6: Clippy (trustedge-core with git-attestation)"
if cargo clippy --package trustedge-core --all-targets --features git-attestation -- -D warnings; then
    pass "clippy git-attestation"
else
    fail "clippy git-attestation"
fi

step "Step 7: Clippy (trustedge-core with keyring)"
if cargo clippy --package trustedge-core --all-targets --features keyring -- -D warnings; then
    pass "clippy keyring"
else
    fail "clippy keyring"
fi
```

**New test steps:**
```bash
step "Step 12: Tests (trustedge-core with git-attestation)"
if cargo test --package trustedge-core --features git-attestation --locked; then
    pass "git-attestation tests"
else
    fail "git-attestation tests"
fi

step "Step 13: Tests (trustedge-core with keyring)"
if cargo test --package trustedge-core --features keyring --locked; then
    pass "keyring tests"
else
    fail "keyring tests"
fi
```

**Verification:**
- ✓ `bash -n scripts/ci-check.sh` - syntax valid
- ✓ Script contains git-attestation clippy and test steps
- ✓ Script contains keyring clippy and test steps
- ✓ Step numbering is sequential (1-18, no gaps)

**Commit:** a9ba752

### Task 2: Update GitHub Actions CI with feature-gated test steps

**Files modified:**
- `.github/workflows/ci.yml`

**Implementation:**
- Added clippy (trustedge-core with git-attestation) step after yubikey clippy
- Added clippy (trustedge-core with keyring) step after git-attestation clippy
- Added tests (trustedge-core with git-attestation) step after yubikey tests
- Added tests (trustedge-core with keyring) step after git-attestation tests
- All new steps are blocking (no `continue-on-error: true`)
- No conditional execution (`if:` clauses) since these features have no system library dependencies

**New clippy steps:**
```yaml
- name: clippy (trustedge-core with git-attestation)
  run: cargo clippy --package trustedge-core --all-targets --features git-attestation -- -D warnings

- name: clippy (trustedge-core with keyring)
  run: cargo clippy --package trustedge-core --all-targets --features keyring -- -D warnings
```

**New test steps:**
```yaml
- name: tests (trustedge-core with git-attestation)
  run: cargo test --package trustedge-core --features git-attestation --locked --verbose

- name: tests (trustedge-core with keyring)
  run: cargo test --package trustedge-core --features keyring --locked --verbose
```

**Verification:**
- ✓ YAML syntax valid
- ✓ CI file contains git-attestation clippy and test steps
- ✓ CI file contains keyring clippy and test steps
- ✓ All new steps are blocking (no continue-on-error)
- ✓ Local ci-check.sh and GitHub CI are in sync

**Commit:** ffb82b8

## Deviations from Plan

None - plan executed exactly as written.

## Success Criteria

✓ ci-check.sh has dedicated clippy and test steps for git-attestation feature
✓ ci-check.sh has dedicated clippy and test steps for keyring feature
✓ ci.yml has dedicated clippy and test steps for git-attestation feature
✓ ci.yml has dedicated clippy and test steps for keyring feature
✓ All new CI steps are blocking (not continue-on-error)
✓ Local script and GitHub CI are in sync
✓ No syntax errors in either file

## Impact

**Regression prevention:**
- CI now validates that default build (no features) excludes git-attestation and keyring dependencies
- CI validates that feature-enabled builds compile and pass tests
- Prevents silent breakage where code accidentally becomes unconditional
- Prevents feature-gated code from breaking due to lack of testing

**CI coverage:**
- Each feature tested independently (isolates failures)
- Clippy ensures code quality for feature-specific code
- Tests run with `--locked` to catch Cargo.lock issues
- Existing `--all-features` step validates feature combinations
- cargo-hack powerset already tests all feature combinations (no change needed)

**Developer experience:**
- Local ci-check.sh mirrors GitHub CI behavior
- Developers can validate features before pushing
- Clear step names indicate what's being tested
- Blocking steps ensure broken features don't merge

**Build matrix:**
- Default build: no features (fastest, smallest)
- Feature builds: git-attestation, keyring (validated independently)
- Combined build: --all-features (validates no conflicts)
- Powerset: all 32 combinations (validates all permutations)

## Self-Check: PASSED

All created files exist:
- No new files created (only modifications)

All commits exist:
- ✓ FOUND: a9ba752 (Task 1: local CI script)
- ✓ FOUND: ffb82b8 (Task 2: GitHub Actions CI)

All modified files exist:
- ✓ FOUND: scripts/ci-check.sh
- ✓ FOUND: .github/workflows/ci.yml
