<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


# Phase 73: Deployment Hardening - Context

**Gathered:** 2026-03-27
**Status:** Ready for planning

<domain>
## Phase Boundary

Fix nginx security header inheritance in sub-locations, update CSP connect-src for dashboard API origin, and move Docker Compose credentials out of plaintext inline values.

</domain>

<decisions>
## Implementation Decisions

### nginx Security Headers (DEPL-01)
- **D-01:** Repeat the 4 security headers (X-Content-Type-Options, X-Frame-Options, Referrer-Policy, CSP) in every location block that has its own `add_header` directive. Affected blocks:
  - `nginx.conf`: `location ~*` (static assets, line 19-22), `location /healthz` (line 25-29)
  - `nginx-ssl.conf.template`: `location ~*` (static assets, line 26-29), `location /healthz` (line 32-36), and the HTTP redirect server's `location /healthz` (line 45-49)
  - Also add HSTS to the SSL template's sub-locations that already have it at server level.

### CSP connect-src (DEPL-02)
- **D-02:** Update CSP `connect-src 'self'` to include the API origin so dashboard API calls work when re-enabled. Approach is Claude's discretion — options include build-time `VITE_API_BASE` substitution or envsubst at container start or simply adding `http://localhost:3001` alongside `'self'`.

### Docker Compose Credentials (DEPL-03)
- **D-03:** Move `POSTGRES_PASSWORD` and `DATABASE_URL` credentials from inline plaintext in `docker-compose.yml` to an `env_file` reference (e.g., `deploy/.env.example` with documented values, actual `.env` gitignored). Update the compose comment that says "No .env file needed" since that's no longer true.

### Claude's Discretion
- Whether to use an nginx `include` snippet for shared headers vs repeating inline
- Exact CSP connect-src approach (build-time vs runtime vs hardcoded)
- Whether `.env.example` should contain the dev defaults or placeholder values

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### nginx configs
- `deploy/nginx.conf` — Dashboard nginx config with security headers at server level (line 8-11) and sub-locations that override them (lines 19-22, 25-29)
- `deploy/nginx-ssl.conf.template` — SSL nginx config with HSTS, same sub-location override issue (lines 26-29, 32-36, 45-49)

### Docker Compose
- `deploy/docker-compose.yml` — Full stack compose with inline POSTGRES_PASSWORD (line 18) and DATABASE_URL (line 35)

### Prior v2.9 work
- Phase 70 added security headers at server level and HSTS — this phase completes the inheritance fix

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- Server-level `add_header` block in nginx.conf (lines 8-11) — template for what to repeat in sub-locations
- `deploy/.env.example` does NOT exist yet — needs creation

### Established Patterns
- nginx configs use `always` flag on all add_header directives
- Docker Compose uses inline environment values throughout
- `VITE_API_BASE` is a build arg on the dashboard service (line 56)

### Integration Points
- Dashboard Dockerfile.dashboard likely uses VITE_API_BASE at build time
- Docker Compose healthcheck on platform-server uses wget to /healthz
- The HTTP redirect server in nginx-ssl.conf.template has its own /healthz that also needs headers

</code_context>

<specifics>
## Specific Ideas

No specific requirements — standard nginx and Docker Compose patterns.

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope

</deferred>

---

*Phase: 73-deployment-hardening*
*Context gathered: 2026-03-27*
