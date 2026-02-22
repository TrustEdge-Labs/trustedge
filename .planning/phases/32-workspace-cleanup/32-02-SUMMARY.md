---
phase: 32-workspace-cleanup
plan: 02
subsystem: infra
tags: [cargo-workspace, pubky, dependency-isolation, experimental]

# Dependency graph
requires:
  - phase: 32-01
    provides: facade crates removed, workspace cleaned of receipts/attestation
provides:
  - Experimental workspace at crates/experimental/ isolating pubky crates
  - Root workspace free of pubky members and pubky-only workspace deps
  - Cargo.lock reduced by 61 packages (697 → 636)
affects: [32-03, ci-check.sh]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Experimental crates get their own workspace root (crates/experimental/Cargo.toml) with explicit dep versions"
    - "Experimental workspace gitignores its own Cargo.lock to avoid root workspace contamination"

key-files:
  created:
    - crates/experimental/Cargo.toml
    - crates/experimental/.gitignore
  modified:
    - Cargo.toml (root workspace — removed pubky members + pubky-only workspace deps)
    - Cargo.lock (root — reduced from 697 to 636 packages)
    - crates/experimental/pubky/Cargo.toml (workspace refs → explicit versions, path → ../../core)
    - crates/experimental/pubky-advanced/Cargo.toml (workspace refs → explicit versions, path → ../../core)

key-decisions:
  - "Experimental workspace uses no [workspace.dependencies] — each crate pins its own explicit versions to avoid coupling"
  - "rsa retained in root workspace.dependencies — trustedge-core uses it in asymmetric.rs (not pubky-only)"
  - "x25519-dalek and hkdf removed from root workspace.dependencies — only used by pubky crates"
  - "pubky = '0.5.4' removed from root workspace.dependencies — only used by pubky crates"
  - "Tier 1/Tier 2 classification replaced with flat list + experimental note pointing to crates/experimental/"

patterns-established:
  - "Experimental/community crates: separate workspace at crates/experimental/ with own Cargo.toml"
  - "Dep isolation: experimental workspace ignores its Cargo.lock, root Cargo.lock stays clean"

requirements-completed: [WRK-03, WRK-04]

# Metrics
duration: 3min
completed: 2026-02-22
---

# Phase 32 Plan 02: Isolate Pubky Crates into Experimental Workspace Summary

**Pubky crates moved to crates/experimental/ standalone workspace with explicit dep pins; root Cargo.lock reduced by 61 packages (697 → 636) by eliminating pubky, x25519-dalek, hkdf transitive deps**

## Performance

- **Duration:** 3 min
- **Started:** 2026-02-22T18:27:51Z
- **Completed:** 2026-02-22T18:31:16Z
- **Tasks:** 2
- **Files modified:** 6

## Accomplishments
- Created crates/experimental/ as a standalone Cargo workspace with pubky and pubky-advanced crates
- Removed pubky crates from root workspace members; removed pubky, x25519-dalek, hkdf from workspace.dependencies
- Root Cargo.lock shrank from 697 to 636 packages (-61 pubky transitive deps)
- All 28 experimental crate tests pass (7 pubky unit + 11 pubky integration + 10 pubky-advanced unit)
- All 154 trustedge-core lib tests continue to pass

## Task Commits

Each task was committed atomically:

1. **Task 1: Create experimental workspace and move pubky crates** - `3ba8688` (feat)
2. **Task 2: Clean root workspace deps and verify dep tree reduction** - `8851916` (feat)

**Plan metadata:** _(docs commit follows)_

## Files Created/Modified
- `crates/experimental/Cargo.toml` - Standalone workspace definition for pubky crates
- `crates/experimental/.gitignore` - Excludes /target/ and Cargo.lock from git
- `crates/experimental/pubky/Cargo.toml` - Replaced workspace refs with explicit versions; path = ../../core
- `crates/experimental/pubky-advanced/Cargo.toml` - Replaced workspace refs with explicit versions; path = ../../core
- `Cargo.toml` - Removed pubky/pubky-advanced members, pubky/x25519-dalek/hkdf workspace deps, updated tier comments
- `Cargo.lock` - Reduced from 697 to 636 packages

## Decisions Made
- Experimental workspace uses no `[workspace.dependencies]` — each crate pins its own explicit versions to avoid coupling the experimental workspace to the root
- `rsa` retained in root workspace.dependencies because `trustedge-core/src/asymmetric.rs` uses it directly (not pubky-only)
- `x25519-dalek` and `hkdf` removed from root — confirmed no non-pubky crates depend on them
- Tier 1/Tier 2 crate classification replaced with a flat list noting experimental crates live in crates/experimental/

## Deviations from Plan

None — plan executed exactly as written. The cargo fmt pre-commit hook required that root Cargo.toml be updated before task 1 could be committed (because `cargo fmt --all` resolves workspace members), but both tasks were still committed as separate atomic commits in the correct logical order.

## Issues Encountered
- Pre-commit hook ran `cargo fmt --all` which requires a valid root workspace. Since task 1 moved the pubky crates but root Cargo.toml still listed them, cargo fmt failed. Solution: updated root Cargo.toml (task 2) before committing task 1. Both tasks still committed atomically in separate commits.
- cargo-machete reports pre-existing unused deps in trustedge-types, trustedge-platform, and trustedge-platform-server — these are out-of-scope for this plan and logged as deferred items.

## User Setup Required
None — no external service configuration required.

## Next Phase Readiness
- Plan 03 (CI update) can now update the dependency baseline count in ci-check.sh Step 21
- Experimental workspace is buildable and testable at crates/experimental/
- Root workspace is clean and all CI checks should pass

---
*Phase: 32-workspace-cleanup*
*Completed: 2026-02-22*
