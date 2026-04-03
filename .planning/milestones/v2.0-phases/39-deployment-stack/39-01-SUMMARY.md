<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


---
phase: 39-deployment-stack
plan: 01
subsystem: infra
tags: [docker, nginx, svelte, sveltekit, static-adapter, spa]

requires: []
provides:
  - SvelteKit dashboard configured with static adapter and SPA fallback routing
  - Dashboard Docker image (node:20-alpine builder + nginx:alpine runtime)
  - nginx.conf with try_files SPA routing and /healthz health endpoint
  - VITE_API_BASE build arg baked into static JS at Docker build time
affects: [39-02]

tech-stack:
  added: ["@sveltejs/adapter-static ^2.0.0", "nginx:alpine (runtime image)"]
  patterns: ["multi-stage Docker build for SPA (builder+nginx)", "VITE_ env baked at build time via ARG", "SPA fallback with try_files $uri $uri/ /index.html"]

key-files:
  created:
    - web/dashboard/src/routes/+layout.ts
    - web/dashboard/static/favicon.png
    - deploy/Dockerfile.dashboard
    - deploy/nginx.conf
  modified:
    - web/dashboard/package.json
    - web/dashboard/svelte.config.js
    - web/dashboard/package-lock.json

key-decisions:
  - "Use prerender=false (not true) in layout.ts -- dynamic route /receipts/[id] cannot be statically prerendered; SPA fallback in nginx handles all routing"
  - "Add static/favicon.png -- app.html referenced favicon.png but file was missing, causing prerender build failure"
  - "Docker build context is repo root (not web/dashboard/) -- matches platform Dockerfile convention, allows COPY of deploy/nginx.conf from same context"

patterns-established:
  - "Dashboard Docker: multi-stage node:20-alpine builder + nginx:alpine runtime, context at repo root"
  - "VITE_API_BASE as ARG in Dockerfile -- bakes API URL into static JS at build time, no runtime env injection needed"

requirements-completed: [DEPL-01, DEPL-03]

duration: 4min
completed: 2026-03-15
---

# Phase 39 Plan 01: Dashboard Containerization Summary

**SvelteKit dashboard converted to static SPA with adapter-static, served by nginx:alpine in a multi-stage Docker image with configurable VITE_API_BASE and /healthz health endpoint**

## Performance

- **Duration:** 4 min
- **Started:** 2026-03-15T20:53:21Z
- **Completed:** 2026-03-15T20:56:43Z
- **Tasks:** 2
- **Files modified:** 7

## Accomplishments
- SvelteKit switched from adapter-auto to adapter-static with `fallback: 'index.html'` for SPA routing
- Dashboard `npm run build` produces static files in `web/dashboard/build/`
- `deploy/Dockerfile.dashboard` multi-stage build: node:20-alpine builds, nginx:alpine serves
- `deploy/nginx.conf` provides SPA try_files fallback, aggressive asset caching (1y immutable), and /healthz endpoint
- Docker image verified: builds from repo root, runs, responds "ok" at /healthz

## Task Commits

Each task was committed atomically:

1. **Task 1: Switch SvelteKit to static adapter and add prerender layout** - `08c5adc` (feat)
2. **Task 2: Create Dockerfile.dashboard and nginx.conf for containerized dashboard** - `4778f64` (feat)

**Plan metadata:** (created below)

## Files Created/Modified
- `web/dashboard/package.json` - Replaced adapter-auto with adapter-static
- `web/dashboard/svelte.config.js` - Changed adapter import and added fallback: 'index.html'
- `web/dashboard/src/routes/+layout.ts` - Created: ssr=false, prerender=false for SPA mode
- `web/dashboard/static/favicon.png` - Created: minimal 1x1 PNG (was missing, blocked build)
- `web/dashboard/package-lock.json` - Updated by npm install
- `deploy/Dockerfile.dashboard` - Created: multi-stage build from repo root context
- `deploy/nginx.conf` - Created: SPA routing, static caching, /healthz

## Decisions Made
- Used `prerender = false` instead of plan's `prerender = true`: the dynamic route `/receipts/[id]` cannot be prerendered without explicit entries. For a SPA with nginx `try_files` fallback, `prerender = false` + `ssr = false` is the correct pattern. Static files still produced; nginx handles all routing.
- Added `static/favicon.png`: `app.html` referenced `%sveltekit.assets%/favicon.png` but the file didn't exist, causing a 404 during build prerendering. A minimal 1x1 transparent PNG was added.
- Docker build context set to repo root (not `web/dashboard/`): matches the existing platform Dockerfile convention and allows a single `COPY deploy/nginx.conf` instruction without copying nginx.conf into the dashboard directory.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Fixed missing favicon.png causing build failure**
- **Found during:** Task 1 (static adapter build)
- **Issue:** `app.html` references `favicon.png` but `web/dashboard/static/` did not exist; build exited 1 with "Error: 404 /favicon.png"
- **Fix:** Created `web/dashboard/static/favicon.png` with minimal 1x1 transparent PNG
- **Files modified:** web/dashboard/static/favicon.png
- **Verification:** `npm run build` exits 0, `build/index.html` exists
- **Committed in:** `08c5adc` (Task 1 commit)

**2. [Rule 1 - Bug] Set prerender=false instead of prerender=true**
- **Found during:** Task 1 (static adapter build)
- **Issue:** `prerender = true` with dynamic route `/receipts/[id]` causes "Error: routes were marked as prerenderable but were not prerendered" since there are no known IDs at build time
- **Fix:** Changed `+layout.ts` to `export const prerender = false`. The intent of the plan (static SPA output served by nginx) is fully preserved -- adapter-static still produces `build/` with `index.html` fallback
- **Files modified:** web/dashboard/src/routes/+layout.ts
- **Verification:** `npm run build` exits 0; `build/index.html` present; Docker build and container health check pass
- **Committed in:** `08c5adc` (Task 1 commit)

---

**Total deviations:** 2 auto-fixed (2 Rule 1 bugs)
**Impact on plan:** Both fixes required for build to succeed. Core objective (static SPA in Docker) delivered exactly as specified.

## Issues Encountered
None beyond the auto-fixed deviations above.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Dashboard Docker image (`trustedge-dashboard-test`) builds and runs successfully
- Ready for Plan 02: docker-compose.yml integrating platform-server, postgres, and dashboard containers
- VITE_API_BASE defaults to `http://localhost:3001` and can be overridden at `docker build` time via `--build-arg`

---
*Phase: 39-deployment-stack*
*Completed: 2026-03-15*

## Self-Check: PASSED
- All 7 key files verified present on disk
- Both task commits (08c5adc, 4778f64) verified in git log
