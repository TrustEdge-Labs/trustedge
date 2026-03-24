---
phase: 59-cli-deploy-hardening
plan: 02
subsystem: infra
tags: [nginx, docker, tls, https, ssl, deploy]

# Dependency graph
requires: []
provides:
  - Conditional HTTPS termination in nginx dashboard container via envsubst template
  - docker-entrypoint.sh that activates port 443 SSL block only when SSL_CERT_PATH and SSL_KEY_PATH are set
  - deploy/nginx-ssl.conf.template with TLSv1.2/1.3 server block
  - docker-compose port 8443:443 exposed with documented SSL env vars and cert volume mount
  - .env.example SSL_CERT_PATH/SSL_KEY_PATH documentation
affects: [deploy, dashboard, docker-compose]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "envsubst conditional nginx config: template file + shell entrypoint generates ssl.conf only when cert paths set"
    - "docker-entrypoint.sh pattern: sh -based entrypoint that conditionally augments nginx config before exec"

key-files:
  created:
    - deploy/nginx-ssl.conf.template
    - deploy/docker-entrypoint.sh
  modified:
    - deploy/Dockerfile.dashboard
    - deploy/docker-compose.yml
    - deploy/.env.example

key-decisions:
  - "envsubst + shell entrypoint pattern: nginx cannot conditionally include server blocks based on env vars at runtime; shell entrypoint generates ssl.conf only when both cert vars are non-empty"
  - "HTTP port 80 unchanged: no forced redirect; HTTPS coexists as opt-in via separate server block"
  - "Port 8443:443 exposed in compose: host port 8443 avoids privileged port conflict in development"

patterns-established:
  - "Pattern 1: Conditional nginx TLS via docker-entrypoint.sh — set SSL_CERT_PATH and SSL_KEY_PATH to activate HTTPS, leave unset for HTTP-only"

requirements-completed: [DEPL-01]

# Metrics
duration: 5min
completed: 2026-03-24
---

# Phase 59 Plan 02: TLS Termination for nginx Dashboard Summary

**Conditional HTTPS termination added to the nginx dashboard container via envsubst template and docker-entrypoint.sh; HTTP port 80 unchanged; port 443 opt-in when SSL_CERT_PATH and SSL_KEY_PATH are set**

## Performance

- **Duration:** ~5 min
- **Started:** 2026-03-24T13:55:00Z
- **Completed:** 2026-03-24T13:57:29Z
- **Tasks:** 2
- **Files modified:** 5

## Accomplishments

- Created `deploy/nginx-ssl.conf.template` with a `server { listen 443 ssl; ... }` block using `${SSL_CERT_PATH}` and `${SSL_KEY_PATH}` placeholders (TLSv1.2/1.3, HIGH ciphers, SPA fallback, health check)
- Created `deploy/docker-entrypoint.sh` that conditionally runs envsubst to generate `/etc/nginx/conf.d/ssl.conf` only when both SSL env vars are non-empty; otherwise HTTP-only
- Updated `deploy/Dockerfile.dashboard` to copy the TLS template and entrypoint script, expose port 443, and set the ENTRYPOINT
- Updated `deploy/docker-compose.yml` to expose port `8443:443`, document SSL env vars (commented), and document cert volume mount (commented)
- Updated `deploy/.env.example` to document `SSL_CERT_PATH` and `SSL_KEY_PATH` with usage instructions

## Task Commits

Each task was committed atomically:

1. **Task 1: Add nginx TLS template and docker-entrypoint script** - `8e041aa` (feat)
2. **Task 2: Wire TLS into Dockerfile.dashboard, docker-compose, and .env.example** - `67fc2d8` (feat)

## Files Created/Modified

- `deploy/nginx-ssl.conf.template` - HTTPS server block on port 443 with SSL_CERT_PATH/SSL_KEY_PATH envsubst placeholders
- `deploy/docker-entrypoint.sh` - Shell entrypoint that conditionally activates HTTPS via envsubst at container startup
- `deploy/Dockerfile.dashboard` - Added TLS template copy, entrypoint, and EXPOSE 443
- `deploy/docker-compose.yml` - Port 8443:443 exposed; SSL env vars and cert volume mount documented (commented)
- `deploy/.env.example` - TLS/HTTPS section with SSL_CERT_PATH and SSL_KEY_PATH documented

## Decisions Made

- envsubst + shell entrypoint pattern chosen over multi-stage template includes — nginx cannot conditionally include server blocks based on env vars at runtime; shell entrypoint is the idiomatic nginx:alpine solution
- HTTP port 80 left completely unchanged — no forced redirect; HTTPS is additive and opt-in
- Port 8443 used as host port for 443 to avoid requiring privileged ports in development environments

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None.

## User Setup Required

To enable HTTPS on the dashboard container:

1. Set `SSL_CERT_PATH` and `SSL_KEY_PATH` environment variables on the dashboard service (to paths inside the container)
2. Mount the certificate directory as a read-only volume: `- /path/to/your/certs:/certs:ro`
3. Access via port 8443 (maps to container port 443)
4. HTTP on port 8080 continues to work unchanged

## Next Phase Readiness

- TLS termination infra complete for dashboard nginx container
- API key removal from dashboard client-side bundle is a separate plan (59-03 or similar)
- Deploy stack is production-ready for HTTPS when certificates are provided

## Self-Check: PASSED

- `deploy/nginx-ssl.conf.template` exists and contains `ssl_certificate`
- `deploy/docker-entrypoint.sh` exists and contains `SSL_CERT_PATH` conditional and `envsubst`
- `deploy/Dockerfile.dashboard` contains `nginx-ssl.conf.template`, `docker-entrypoint.sh`, and `ENTRYPOINT`
- `deploy/docker-compose.yml` contains `443` and `SSL_CERT_PATH`
- `deploy/.env.example` contains `SSL_CERT_PATH` and `SSL_KEY_PATH`
- `deploy/nginx.conf` unchanged with `listen 80`
- Commits `8e041aa` and `67fc2d8` exist

---
*Phase: 59-cli-deploy-hardening*
*Completed: 2026-03-24*
