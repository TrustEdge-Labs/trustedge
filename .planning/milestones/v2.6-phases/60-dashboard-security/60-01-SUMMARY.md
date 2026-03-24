---
phase: 60-dashboard-security
plan: 01
subsystem: ui
tags: [svelte, typescript, dashboard, security, credentials, ci]

# Dependency graph
requires: []
provides:
  - Dashboard config.ts without apiKey field
  - ApiClient without Authorization header
  - receipts and devices pages show admin-only notice instead of calling protected endpoints
  - CI Step 12 greps built dashboard bundle for VITE_API_KEY and fails if found
affects: [dashboard, ci-check]

# Tech tracking
tech-stack:
  added: []
  patterns: [public-endpoint-only dashboard, static admin notice for protected endpoints, CI bundle credential scan]

key-files:
  created: []
  modified:
    - web/dashboard/src/lib/config.ts
    - web/dashboard/src/lib/api.ts
    - web/dashboard/src/routes/+page.svelte
    - web/dashboard/src/routes/receipts/+page.svelte
    - web/dashboard/src/routes/devices/+page.svelte
    - web/dashboard/.env.example
    - scripts/ci-check.sh

key-decisions:
  - "Dashboard accesses public endpoints only (/v1/verify, /.well-known/jwks.json, /healthz) — no Bearer token needed"
  - "Receipts and devices pages replaced with static admin notice — calling protected endpoints from client bundle was incorrect"
  - "CI Step 12 builds dashboard and greps bundle for VITE_API_KEY; fails CI if credential found"

patterns-established:
  - "Public-endpoint-only dashboard: config has no credential fields; api client sends no Authorization header"
  - "Protected-endpoint pages show static admin notice with curl command for direct API access"

requirements-completed: [DASH-01]

# Metrics
duration: 2min
completed: 2026-03-24
---

# Phase 60 Plan 01: Dashboard Security Summary

**Removed VITE_API_KEY from SvelteKit dashboard bundle: config, api client, and two protected-endpoint pages replaced; CI bundle grep guard added**

## Performance

- **Duration:** 2 min
- **Started:** 2026-03-24T14:54:08Z
- **Completed:** 2026-03-24T14:55:47Z
- **Tasks:** 2
- **Files modified:** 7

## Accomplishments
- Eliminated VITE_API_KEY from all dashboard source files — credential can no longer appear in the client-side JavaScript bundle
- Removed Authorization header from ApiClient — dashboard only calls unauthenticated public endpoints
- Replaced receipts and devices pages (which called postgres-protected endpoints) with static "Admin access required" notices showing direct API curl commands
- Added CI Step 12 that builds the dashboard and greps the built bundle for VITE_API_KEY, failing CI if found

## Task Commits

Each task was committed atomically:

1. **Task 1: Remove VITE_API_KEY from config, api client, home page, and env.example** - `df13dc3` (fix)
2. **Task 2: Update protected-endpoint pages and add CI bundle guard** - `f92375c` (fix)

**Plan metadata:** (final docs commit)

## Files Created/Modified
- `web/dashboard/src/lib/config.ts` - Removed apiKey field; apiBase is the only config
- `web/dashboard/src/lib/api.ts` - Removed apiKey field, constructor assignment, and Authorization header; updated 401 message
- `web/dashboard/src/routes/+page.svelte` - Removed API key check banner (onMount + config import + error div)
- `web/dashboard/src/routes/receipts/+page.svelte` - Replaced full data-fetching UI with static admin notice
- `web/dashboard/src/routes/devices/+page.svelte` - Replaced device management UI with static admin notice
- `web/dashboard/.env.example` - Removed VITE_API_KEY line
- `scripts/ci-check.sh` - Added Step 12: dashboard bundle credential check

## Decisions Made
- Dashboard accesses public endpoints only — no API key needed in client bundle
- Receipts and devices pages replaced entirely with static notices rather than conditionally hiding — cleaner and eliminates all protected API calls
- CI Step 12 skips gracefully if node is not installed or build fails (warns rather than fails), to avoid breaking CI on minimal environments

## Deviations from Plan

None — plan executed exactly as written.

## Issues Encountered

None — TypeScript check passes with 0 errors (1 pre-existing CSS warning about unused `.code` selector on home page, unrelated to this plan).

## User Setup Required

None — no external service configuration required. Existing VITE_API_BASE env var is unchanged; VITE_API_KEY should be removed from any existing `.env.local` files.

## Next Phase Readiness

- Dashboard security hardening (DASH-01) complete
- v2.6 milestone all 5 plans now complete (phases 57-60)
- CI guard permanently prevents re-introduction of API key in dashboard bundle

---
*Phase: 60-dashboard-security*
*Completed: 2026-03-24*
