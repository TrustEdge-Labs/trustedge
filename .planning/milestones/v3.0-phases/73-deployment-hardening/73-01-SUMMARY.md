<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


---
phase: 73-deployment-hardening
plan: "01"
subsystem: infra
tags: [nginx, docker-compose, security-headers, csp, credentials]

# Dependency graph
requires: []
provides:
  - nginx security headers repeated in all location blocks (DEPL-01)
  - CSP connect-src includes http://localhost:3001 in all nginx configs (DEPL-02)
  - Docker Compose credentials sourced from env_file, not inline plaintext (DEPL-03)
  - deploy/.env gitignored to prevent credential leaks
affects: [74-docs-sweep]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "nginx: repeat all security headers in every location block that uses add_header (inheritance does not work)"
    - "docker-compose: env_file for secrets, environment: block only for non-secret config"

key-files:
  created: []
  modified:
    - deploy/nginx.conf
    - deploy/nginx-ssl.conf.template
    - deploy/docker-compose.yml
    - deploy/.env.example
    - .gitignore

key-decisions:
  - "HTTP redirect server healthz uses minimal CSP (default-src 'self' only) since it serves no JS and connect-src is not applicable"
  - "env_file references deploy/.env so cp .env.example .env works immediately with dev defaults"
  - "POSTGRES_USER and POSTGRES_DB stay inline in environment: (not secrets); only POSTGRES_PASSWORD and DATABASE_URL moved to env_file"

patterns-established:
  - "nginx security header pattern: all 4 headers (X-Content-Type-Options, X-Frame-Options, Referrer-Policy, CSP) repeated in every location block"
  - "SSL locations additionally include HSTS; HTTP redirect locations do not"

requirements-completed: [DEPL-01, DEPL-02, DEPL-03]

# Metrics
duration: 2min
completed: "2026-03-27"
---

# Phase 73 Plan 01: Deployment Hardening Summary

**nginx security headers fixed in all location blocks with CSP connect-src for localhost:3001, and Docker Compose credentials moved from inline plaintext to env_file**

## Performance

- **Duration:** ~2 min
- **Started:** 2026-03-27T19:16:50Z
- **Completed:** 2026-03-27T19:18:36Z
- **Tasks:** 2
- **Files modified:** 5

## Accomplishments

- Fixed nginx header inheritance issue: all 4 security headers now repeated in every location block that uses add_header (static assets and healthz in both configs)
- Added `http://localhost:3001` to CSP connect-src in all applicable nginx location blocks so dashboard API calls are not browser-blocked
- Added HSTS to SSL location blocks; HTTP redirect server correctly has no HSTS
- Moved POSTGRES_PASSWORD and DATABASE_URL from inline docker-compose.yml to env_file, eliminating plaintext credentials in source-controlled files
- Updated deploy/.env.example with active dev defaults so `cp .env.example .env` works immediately
- Added `deploy/.env` to .gitignore to prevent real credentials being committed

## Task Commits

1. **Task 1: Fix nginx security header inheritance and CSP connect-src** - `96244b3` (fix)
2. **Task 2: Move Docker Compose credentials to env_file** - `05cb7d5` (fix)

## Files Created/Modified

- `deploy/nginx.conf` - Security headers repeated in static assets and healthz location blocks; CSP connect-src updated
- `deploy/nginx-ssl.conf.template` - Security headers + HSTS repeated in SSL location blocks; HTTP redirect healthz has minimal CSP (no HSTS)
- `deploy/docker-compose.yml` - env_file directive added to postgres and platform-server; inline POSTGRES_PASSWORD and DATABASE_URL removed
- `deploy/.env.example` - Updated header, active credential defaults (POSTGRES_PASSWORD, DATABASE_URL), usage instructions updated
- `.gitignore` - Added `deploy/.env` entry

## Decisions Made

- HTTP redirect server healthz uses `Content-Security-Policy "default-src 'self'"` (not the full CSP with connect-src) since the redirect server serves no JavaScript and connect-src is meaningless there. This satisfies the acceptance criteria of 3 (not 4) matches for connect-src in nginx-ssl.conf.template.
- POSTGRES_USER and POSTGRES_DB remain inline in `environment:` since they are not secrets; only the password-bearing variables go to env_file.

## Deviations from Plan

None - plan executed exactly as written. The HTTP redirect healthz CSP decision aligns with the plan's acceptance criteria note: "(SSL server + 2 SSL locations; HTTP redirect healthz has no CSP with connect-src needed)".

## Issues Encountered

None.

## User Setup Required

Operators must create `deploy/.env` before running docker-compose:
```bash
cp deploy/.env.example deploy/.env
# Edit POSTGRES_PASSWORD and DATABASE_URL for non-local deployments
docker compose -f deploy/docker-compose.yml up --build
```

Dev defaults in `.env.example` work as-is for local/demo use.

## Next Phase Readiness

- All DEPL requirements complete (DEPL-01, DEPL-02, DEPL-03)
- Phase 74 (docs sweep) can proceed — all deployment hardening fixes are landed

---
*Phase: 73-deployment-hardening*
*Completed: 2026-03-27*
