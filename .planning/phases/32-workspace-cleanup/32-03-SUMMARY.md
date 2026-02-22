---
phase: 32-workspace-cleanup
plan: 03
subsystem: infra
tags: [ci, documentation, workspace-cleanup, scripts]

# Dependency graph
requires:
  - phase: 32-01
    provides: facade crates deleted from workspace
  - phase: 32-02
    provides: pubky crates isolated to crates/experimental/ standalone workspace
provides:
  - CI scripts free of deleted/moved crate references and tiered logic
  - Documentation reflecting actual 9-crate root workspace
affects: []

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Single-tier CI: all workspace crates covered by one clippy/test pass (--workspace flag)"
    - "Dep tree baseline 70 matches actual post-cleanup count in both ci-check.sh and ci.yml"

key-files:
  created: []
  modified:
    - scripts/ci-check.sh
    - .github/workflows/ci.yml
    - CLAUDE.md
    - README.md
    - DEPENDENCIES.md
    - FEATURES.md
    - MIGRATION.md

key-decisions:
  - "MIGRATION.md retains historical crate names (trustedge-receipts, trustedge-attestation) in migration guidance sections — these are educational references, not active crate names, and the file is not scanned by CI"
  - "Dep tree baseline kept at 70 in both ci-check.sh and ci.yml — actual count matches baseline exactly"
  - "ci.yml uses --workspace flag for clippy and tests, replacing the explicit -p list — ensures trustedge-types and trustedge-platform are covered (they were missing from ci.yml core crates list)"

patterns-established:
  - "After crate removal: rewrite entire affected sections rather than surgical removal — cleaner and avoids stale context"

requirements-completed: [WRK-02]

# Metrics
duration: 5min
completed: 2026-02-22
---

# Phase 32 Plan 03: CI and Documentation Cleanup Summary

**CI scripts simplified to single-tier workspace passes; all documentation sections rewritten to reflect 9-crate root workspace with pubky moved to crates/experimental/ and facade crates fully removed**

## Performance

- **Duration:** 5 min
- **Started:** 2026-02-22T18:33:41Z
- **Completed:** 2026-02-22T18:39:12Z
- **Tasks:** 2
- **Files modified:** 7

## Accomplishments
- Removed tiered CI logic (core blocking, experimental non-blocking) from both ci-check.sh and ci.yml — replaced with single `--workspace` clippy and test passes
- Removed all references to trustedge-receipts, trustedge-attestation, trustedge-pubky, trustedge-pubky-advanced from all CI scripts
- Updated dep tree baseline in ci.yml from 60 to 70 (was stale since v1.2; actual count is 70)
- CLAUDE.md architecture section rewritten: 9 crates listed (was 12 with facades and pubky), experimental workspace note added
- DEPENDENCIES.md rewritten: removed Experimental Tier section and all 4 deleted/moved crate sections; added trustedge-types section; updated workspace dep summary (removed pubky/x25519-dalek/hkdf)
- README.md crate table and directory tree rewritten to match actual workspace
- FEATURES.md: removed trustedge-attestation symbols feature section
- MIGRATION.md: noted that facade crates have been fully removed as of v1.7 (not just deprecated)

## Task Commits

Each task was committed atomically:

1. **Task 1: Simplify CI scripts (ci-check.sh and ci.yml)** - `e63afcf` (chore)
2. **Task 2: Rewrite documentation sections** - `f6e52e7` (docs)

**Plan metadata:** _(docs commit follows)_

## Files Created/Modified
- `scripts/ci-check.sh` - Removed tiered Step 4 and Step 11; single `--workspace` clippy and test passes
- `.github/workflows/ci.yml` - Removed tiered clippy/test steps; updated dep tree baseline from 60 to 70; added trustedge-types/trustedge-platform via `--workspace`
- `CLAUDE.md` - Architecture Overview: 9 crates (was 12), removed facade and pubky sections, added experimental workspace note
- `README.md` - Rewritten crate table and directory tree; removed 2-tier classification section
- `DEPENDENCIES.md` - Removed Experimental Tier section and 4 crate sections; added trustedge-types section; updated security rationale numbering
- `FEATURES.md` - Removed trustedge-attestation symbols feature section
- `MIGRATION.md` - Updated timeline to note facade removal is complete (v1.7)

## Decisions Made
- MIGRATION.md retains historical crate names in migration guidance — these are educational "migrate FROM" references, not active crate references, and MIGRATION.md is not scanned by CI
- Dep tree baseline kept at 70 (matches actual count exactly post-cleanup)
- `--workspace` flag used in ci.yml clippy/test to ensure trustedge-types and trustedge-platform are covered (they were absent from the old explicit `-p` list in ci.yml)

## Deviations from Plan

None — plan executed exactly as written.

## Issues Encountered
None.

## User Setup Required
None — no external service configuration required.

## Next Phase Readiness
- Phase 32 complete: all 3 plans executed
- All CI scripts and documentation are consistent with the post-cleanup 9-crate root workspace
- No references to deleted facades or moved pubky crates remain in any CI or developer-facing doc

---
*Phase: 32-workspace-cleanup*
*Completed: 2026-02-22*
