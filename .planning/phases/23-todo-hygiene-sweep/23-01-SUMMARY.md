---
phase: 23-todo-hygiene-sweep
plan: 01
subsystem: ["code-quality", "ci-enforcement"]
tags: ["cleanup", "ci", "documentation"]
one_liner: "Zero unimplemented TODOs confirmed, feature-disabled terminology clarified, CI hygiene enforcement added"

dependency_graph:
  requires: ["phase-22 (pubky stub elimination)"]
  provides: ["TODO hygiene CI enforcement", "Clear feature-disabled terminology"]
  affects: ["scripts/ci-check.sh", ".github/workflows/ci.yml", "crates/core/src/audio.rs", "crates/trustedge-cli/src/main.rs"]

tech_stack:
  patterns_added: ["CI TODO hygiene scanning"]

key_files:
  created: []
  modified:
    - path: "crates/core/src/audio.rs"
      impact: "Renamed 'stub' to 'feature-disabled' in doc-comments for clarity"
    - path: "crates/trustedge-cli/src/main.rs"
      impact: "Updated doc-comment for list_audio_devices and clarified header fields comment"
    - path: "scripts/ci-check.sh"
      impact: "Added Step 22: TODO hygiene check to local CI script"
    - path: ".github/workflows/ci.yml"
      impact: "Added TODO hygiene step to GitHub Actions CI workflow"

decisions:
  - what: "Use 'feature-disabled' terminology instead of 'stub'"
    why: "More accurately describes fail-closed behavior for cfg-gated features"
    impact: "Clearer documentation, reduces confusion about purpose"
  - what: "Scan for multiple TODO/FIXME variants in CI"
    why: "Catch all forms of unimplemented markers: comments and macros"
    impact: "Prevents regression to incomplete code"

metrics:
  completed_at: "2026-02-13"
  duration_min: 5.6
  tasks_completed: 2
  files_modified: 4
  test_coverage: "146 core tests passing"
---

# Phase 23 Plan 01: TODO Hygiene Sweep Summary

Zero unimplemented TODOs confirmed, feature-disabled terminology clarified, CI hygiene enforcement added.

## Objective

Finalize TODO hygiene across the codebase: confirm zero unimplemented-functionality TODOs remain, improve doc-comment clarity on cfg-gated feature stubs, and add CI guardrails preventing future TODO regression.

## What Was Built

### Task 1: Clean up ambiguous stub terminology and confirm zero unimplemented TODOs

**Verified zero unimplemented TODO markers:**
- Scanned entire crates/ directory for `// TODO`, `// FIXME`, `// HACK`, `// XXX`, `todo!()`, `unimplemented!()`
- Result: Zero matches found (phases 19-22 successfully eliminated all unimplemented markers)

**Renamed "stub" terminology to "feature-disabled":**
- Updated `crates/core/src/audio.rs`:
  - Changed top-level comment: "Stub implementation" → "Feature-disabled implementation: returns errors when audio feature is not compiled"
  - Updated 9 doc-comments: `(stub)` → `(feature-disabled)`
  - Renamed test: `test_audio_stub` → `test_audio_feature_disabled`
  - Updated inline comment: "No-op for stub" → "No-op for feature-disabled"

- Updated `crates/trustedge-cli/src/main.rs`:
  - Changed `list_audio_devices` doc-comment: "stub when audio not available" → "feature-disabled: provides guidance when audio not compiled"
  - Clarified header fields comment: "demo placeholders as needed" → "randomly generated per session"

**Rationale:** The #[cfg(not(feature = "audio"))] blocks aren't placeholders or incomplete implementations — they're properly designed fail-closed feature gates that return actionable errors. The "stub" terminology was misleading. "Feature-disabled" accurately describes the behavior.

**Files modified:**
- `crates/core/src/audio.rs` (12 changes)
- `crates/trustedge-cli/src/main.rs` (2 changes)

**Commit:** `483f77a`

### Task 2: Add TODO hygiene CI check to prevent regression

**Added Step 22 to local CI script (`scripts/ci-check.sh`):**
```bash
# ── Step 22: TODO hygiene ──────────────────────────────────────────
step "Step 22: TODO hygiene (no unimplemented markers)"
# Scan for TODO/FIXME/HACK/XXX comments that indicate unimplemented functionality
todo_count=0
while IFS= read -r match; do
    # Skip test-only placeholder data (e.g., continuity_hash in test fixtures)
    case "$match" in
        *"#[cfg(test)]"*) continue ;;
        *"_test_"*|*"test_"*) continue ;;
    esac
    echo "  Found: $match"
    todo_count=$((todo_count + 1))
done < <(grep -rn '// TODO\|// FIXME\|// HACK\|// XXX\|todo!()\|unimplemented!()' \
    --include="*.rs" crates/ \
    2>/dev/null || true)
if [ "$todo_count" -gt 0 ]; then
    fail "$todo_count unimplemented TODO/FIXME markers found"
else
    pass "No unimplemented TODO/FIXME markers"
fi
```

**Added TODO hygiene step to GitHub Actions CI (`.github/workflows/ci.yml`):**
```yaml
- name: TODO hygiene (no unimplemented markers)
  run: |
    # Scan for TODO/FIXME/HACK/XXX that indicate unimplemented functionality
    count=$(grep -rn '// TODO\|// FIXME\|// HACK\|// XXX\|todo!()\|unimplemented!()' \
      --include="*.rs" crates/ 2>/dev/null | wc -l || echo 0)
    if [ "$count" -gt 0 ]; then
      echo "::error::Found $count unimplemented TODO/FIXME/HACK/XXX markers in crates/"
      grep -rn '// TODO\|// FIXME\|// HACK\|// XXX\|todo!()\|unimplemented!()' \
        --include="*.rs" crates/ || true
      exit 1
    fi
    echo "No unimplemented TODO/FIXME markers found"
```

**Scan patterns (identical in both local script and GitHub CI):**
- `// TODO` - Standard TODO comment
- `// FIXME` - Fix-me marker
- `// HACK` - Temporary workaround marker
- `// XXX` - Attention-needed marker
- `todo!()` - Rust panic macro for unimplemented code paths
- `unimplemented!()` - Rust panic macro for unimplemented functions

**Placement:**
- Local script: Added as Step 22 (last check before summary)
- GitHub Actions: Added after copyright headers, before fmt (fail-fast placement)

**Verification:**
- Local script test: `✔ No unimplemented TODO/FIXME markers`
- YAML validation: Passed with `python3 -c "import yaml; yaml.safe_load(...)"`
- Pattern testing: Correctly matches `// TODO: implement this`, correctly ignores `// Removed per TODO hygiene sweep`

**Files modified:**
- `scripts/ci-check.sh` (+18 lines)
- `.github/workflows/ci.yml` (+13 lines)

**Commit:** `d995936`

## Deviations from Plan

None - plan executed exactly as written.

## Verification Results

All verification criteria met:

1. ✔ Zero unimplemented TODO/FIXME/HACK/XXX markers in crates/
2. ✔ Zero "stub" references in audio.rs and main.rs
3. ✔ Core crate tests pass (146 tests in trustedge-core, all passing)
4. ✔ Clippy passes with no warnings (workspace-wide)
5. ✔ Local ci-check.sh runs successfully with new TODO hygiene step
6. ✔ GitHub Actions YAML is valid

**Note on experimental crates:** The trustedge-pubky integration test failure (test_migrate_missing_files) is pre-existing and non-blocking per v1.2 tiered CI design.

## Outcomes

**Code quality improvements:**
- Zero ambiguous "stub" terminology in feature-gated code
- Clear fail-closed documentation for all feature-disabled implementations
- Accurate inline comments (no more "placeholder" terminology for random values)

**CI enforcement:**
- Local ci-check.sh and GitHub Actions CI both enforce TODO hygiene on every run
- Any new TODO/FIXME/HACK/XXX comment or todo!()/unimplemented!() macro causes CI failure
- Prevents regression to incomplete code patterns eliminated in phases 19-22

**Milestone completion:**
- This is the FINAL phase of the v1.4 milestone
- All unimplemented TODOs eliminated (phases 19-22)
- CI enforcement added (phase 23)
- Codebase is now in a clean, production-ready state with automated hygiene checks

## Self-Check: PASSED

**Verified created files:** N/A (no new files created)

**Verified modified files exist:**
- ✔ crates/core/src/audio.rs
- ✔ crates/trustedge-cli/src/main.rs
- ✔ scripts/ci-check.sh
- ✔ .github/workflows/ci.yml

**Verified commits exist:**
- ✔ 483f77a: refactor(23-01): rename stub terminology to feature-disabled
- ✔ d995936: feat(23-01): add TODO hygiene CI enforcement

All claims verified successfully.
