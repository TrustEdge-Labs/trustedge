<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


---
phase: 70-deployment-hardening
plan: 01
subsystem: infra
tags: [nginx, github-actions, security-headers, hsts, csp, ci]

# Dependency graph
requires: []
provides:
  - Least-privilege CI token scope for wasm-tests.yml (contents: read only)
  - Security headers on all nginx.conf responses (X-Content-Type-Options, X-Frame-Options, Referrer-Policy, CSP)
  - HSTS header on all SSL nginx responses (Strict-Transport-Security max-age=31536000)
  - HTTP-to-HTTPS 301 redirect server block on port 8080 with /healthz passthrough
affects: [docker, deploy, dashboard, ci]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Nginx security headers at server block level with always parameter (applies to error responses too)"
    - "HTTP redirect server block co-existing with SSL server block in template"
    - "envsubst safety: nginx $var (no braces) passes through; only ${VAR} is substituted"

key-files:
  created: []
  modified:
    - .github/workflows/wasm-tests.yml
    - deploy/nginx.conf
    - deploy/nginx-ssl.conf.template

key-decisions:
  - "HSTS conservative: max-age=31536000 only, no includeSubDomains, no preload — avoids locking subdomains before they are ready"
  - "HTTP redirect server on port 8080 with /healthz exception — docker-compose health probes must not be redirected"
  - "CSP uses unsafe-inline for style-src — SvelteKit dashboard requires it for component styles"

patterns-established:
  - "Security headers pattern: add_header ... always at server block level, not in location blocks"
  - "nginx template envsubst safety: use $var (no braces) for nginx variables; only ${VAR} (curly braces) is expanded by envsubst"

requirements-completed: [DEPL-01, DEPL-02, DEPL-03]

# Metrics
duration: 10min
completed: 2026-03-26
---

# Phase 70 Plan 01: Deployment Hardening Summary

**Least-privilege CI permissions and complete browser security headers (CSP, HSTS, X-Frame, X-Content-Type, Referrer-Policy) added to all nginx configs with HTTP-to-HTTPS redirect**

## Performance

- **Duration:** 10 min
- **Started:** 2026-03-26T20:25:00Z
- **Completed:** 2026-03-26T20:35:00Z
- **Tasks:** 2
- **Files modified:** 3

## Accomplishments

- Added `permissions: contents: read` to wasm-tests.yml — matches ci.yml and semver.yml pattern, closes DEPL-01
- Added four security headers to deploy/nginx.conf at server block level with `always` parameter — closes DEPL-02
- Added five security headers (including HSTS) to SSL server block in nginx-ssl.conf.template — closes DEPL-03
- Added HTTP-to-HTTPS redirect server block on port 8080 with /healthz passthrough for docker-compose health probes

## Task Commits

Each task was committed atomically:

1. **Task 1: Add permissions block to wasm-tests.yml and security headers to nginx.conf** - `5a93ff0` (feat)
2. **Task 2: Add security headers, HSTS, and HTTP redirect to nginx-ssl.conf.template** - `6f8ac56` (feat)

**Plan metadata:** (docs commit follows)

## Files Created/Modified

- `.github/workflows/wasm-tests.yml` - Added `permissions: contents: read` between env and jobs blocks
- `deploy/nginx.conf` - Added X-Content-Type-Options, X-Frame-Options, Referrer-Policy, Content-Security-Policy headers at server level
- `deploy/nginx-ssl.conf.template` - Added all four security headers plus Strict-Transport-Security, added HTTP redirect server block with /healthz exception

## Decisions Made

- HSTS conservative: `max-age=31536000` only, no `includeSubDomains`, no `preload` — avoids locking subdomains prematurely
- HTTP redirect server block listens on port 8080 with `/healthz` returning 200 — docker-compose health probes hit port 8080 and must not be redirected
- CSP includes `style-src 'self' 'unsafe-inline'` — required by SvelteKit for component styles

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None. The envsubst safety concern documented in the plan (nginx `$host` vs shell `${VAR}`) was handled correctly — the new redirect block uses `$host` and `$request_uri` (no curly braces), which pass through envsubst untouched. Only the pre-existing `${SSL_CERT_PATH}` and `${SSL_KEY_PATH}` are expanded.

## User Setup Required

None - no external service configuration required. Changes are config file only; take effect on next container rebuild.

## Next Phase Readiness

All three P2 deployment hardening findings (DEPL-01, DEPL-02, DEPL-03) are closed. Phase 70 is complete.

---
*Phase: 70-deployment-hardening*
*Completed: 2026-03-26*
