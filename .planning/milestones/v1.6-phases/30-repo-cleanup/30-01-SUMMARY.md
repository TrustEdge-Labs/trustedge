---
phase: 30-repo-cleanup
plan: 01
subsystem: infra
tags: [github, org-cleanup, repo-management]

# Dependency graph
requires:
  - phase: 29-dashboard-consolidation
    provides: trustedge-dashboard code consolidated into web/dashboard/ — safe to delete GitHub repo
  - phase: 25-platform-consolidation
    provides: platform-api and verify-core consolidated into trustedge-platform — safe to delete those GitHub repos
  - phase: 24-types-extraction
    provides: shared-libs consolidated into trustedge-types — safe to delete GitHub repo
provides:
  - TrustEdge-Labs GitHub org reduced to exactly 3 repos (trustedge, trustedgelabs-website, shipsecure)
  - 11 orphaned repos permanently deleted from GitHub
  - Org description updated to reflect 3-repo structure
affects: [30-02-plan, CLAUDE.md, documentation]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "GitHub org cleanup via gh CLI: gh repo delete TrustEdge-Labs/REPO --yes"
    - "Org metadata update via GitHub API: gh api -X PATCH /orgs/ORG -f description=..."

key-files:
  created: []
  modified: []

key-decisions:
  - "11 repos deleted (not just archived): billing-service, device-service, identity-service, infra, ingestion-service, platform-api, verify-core, shared-libs, dashboard, .github, trustedgelabs-brand-kit"
  - "Permanent deletion chosen over archival — all code already lives in the trustedge monorepo"
  - "Org description updated via GitHub API after .github profile repo deleted"

patterns-established: []

requirements-completed: [REPO-01]

# Metrics
duration: 5min
completed: 2026-02-22
---

# Phase 30 Plan 01: Repo Cleanup Summary

**Deleted 11 orphaned GitHub repos from TrustEdge-Labs org via gh CLI, leaving exactly 3 active repos: trustedge, trustedgelabs-website, shipsecure**

## Performance

- **Duration:** ~5 min
- **Started:** 2026-02-22
- **Completed:** 2026-02-22
- **Tasks:** 2 (1 auto + 1 checkpoint:human-verify)
- **Files modified:** 0 (all GitHub API operations — no local file changes)

## Accomplishments

- Deleted 11 orphaned repos from TrustEdge-Labs org via `gh repo delete --yes`
- Updated org description via GitHub API to reflect 3-repo structure
- Human verification confirmed: exactly 3 repos remain, no archived repos visible
- REPO-01 requirement satisfied

## Task Commits

This plan had no local file changes — all work was GitHub API operations. No per-task commits were made.

**Plan metadata:** committed with SUMMARY.md and state updates.

## Files Created/Modified

None — all work was remote GitHub org operations via gh CLI and GitHub API.

## Decisions Made

- Permanent deletion chosen over archival: all consolidated code already lives in the trustedge monorepo. Archived repos would still appear in the org and create confusion.
- The 5 scaffold repos (billing-service, device-service, identity-service, infra, ingestion-service) had been archived in v1.5 but were now permanently deleted.
- platform-api, verify-core, and shared-libs deleted — fully consolidated into trustedge-platform and trustedge-types in phases 24-25.
- trustedge-dashboard deleted — consolidated into `web/dashboard/` in phase 29.
- `.github` org profile repo deleted — org description set directly via GitHub API instead.
- `trustedgelabs-brand-kit` deleted — utility repo no longer needed.

## Deviations from Plan

None — plan executed exactly as written.

## Issues Encountered

None — all 11 repos deleted successfully. Human verification confirmed clean state.

## User Setup Required

None — no external service configuration required.

## Next Phase Readiness

- Phase 30 Plan 02: CLAUDE.md and documentation updates to remove references to deleted repos
- The org is clean; CLAUDE.md still references archived repos in its "Archived Service Repos" table and should be updated to reflect permanent deletion

## Self-Check: PASSED

- SUMMARY.md: FOUND at .planning/phases/30-repo-cleanup/30-01-SUMMARY.md
- STATE.md: Updated with Phase 30 Plan 01 position, decisions, session info
- ROADMAP.md: Updated 30-01 plan to complete, Phase 30 to 1/2 in progress
- REQUIREMENTS.md: REPO-01 marked complete
- No per-task commits (no local files changed — all GitHub API operations)

---
*Phase: 30-repo-cleanup*
*Completed: 2026-02-22*
