# Phase 70: Deployment Hardening - Context

**Gathered:** 2026-03-26
**Status:** Ready for planning

<domain>
## Phase Boundary

Three deployment hardening changes: (1) add explicit `permissions: contents: read` to wasm-tests.yml, (2) add security headers to nginx.conf, and (3) add HSTS + HTTP-to-HTTPS redirect to nginx-ssl.conf.template.

</domain>

<decisions>
## Implementation Decisions

### CI Permissions (DEPL-01)
- **D-01:** Add `permissions: contents: read` block to `.github/workflows/wasm-tests.yml` at workflow level (between the `env:` and `jobs:` blocks), matching the pattern in `ci.yml:10-11` and `semver.yml:9-10`.

### Nginx Security Headers (DEPL-02)
- **D-02:** Add these headers to `deploy/nginx.conf` in the main `server` block (above or within the `location /` block so they apply globally):
  - `X-Content-Type-Options: nosniff`
  - `X-Frame-Options: DENY`
  - `Referrer-Policy: strict-origin-when-cross-origin`
  - `Content-Security-Policy: default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; connect-src 'self'; img-src 'self' data:; font-src 'self'`
- **D-03:** The same security headers should also be added to the SSL template (`deploy/nginx-ssl.conf.template`) so HTTPS-served pages also get them.

### HSTS + HTTP Redirect (DEPL-03)
- **D-04:** Add `Strict-Transport-Security: max-age=31536000` header to the SSL server block in `deploy/nginx-ssl.conf.template`. Conservative — no includeSubDomains, no preload.
- **D-05:** Add a second `server` block in `deploy/nginx-ssl.conf.template` listening on port 8080 that returns `301` redirect to `https://$host:8443$request_uri` for all paths. This block only exists in the SSL template (not in the plain nginx.conf).
- **D-06:** Exclude `/healthz` from the redirect — docker-compose health probes hit port 8080 and must not be redirected. Add `location /healthz { return 200 "ok\n"; add_header Content-Type text/plain; }` in the redirect server block before the catch-all redirect.

### Claude's Discretion
- Exact placement of `add_header` directives within the nginx config (server block level vs location level) — as long as they apply to all responses.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### CI Workflows
- `.github/workflows/wasm-tests.yml` — Missing permissions block (finding 5)
- `.github/workflows/ci.yml:10-11` — Reference pattern for permissions block
- `.github/workflows/semver.yml:9-10` — Reference pattern for permissions block

### Nginx Configs
- `deploy/nginx.conf` — Plain HTTP dashboard config, missing security headers (finding 6)
- `deploy/nginx-ssl.conf.template` — SSL template, missing HSTS and redirect (finding 7)

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- ci.yml and semver.yml already have the exact `permissions: contents: read` pattern to copy
- nginx-ssl.conf.template already has `ssl_protocols TLSv1.2 TLSv1.3` — HSTS header goes alongside

### Established Patterns
- nginx.conf uses `add_header` at location level for Cache-Control and Content-Type — security headers should go at server level to apply globally
- SSL template uses envsubst for `${SSL_CERT_PATH}` and `${SSL_KEY_PATH}` — no envsubst needed for static HSTS header

### Integration Points
- `deploy/docker-entrypoint.sh` processes the SSL template via envsubst — the redirect server block must use literal values (no ${} vars) or be part of the envsubst pipeline
- Docker-compose healthcheck hits port 8080 `/healthz` — redirect must not break this

</code_context>

<specifics>
## Specific Ideas

No specific requirements — standard deployment hardening following security review findings.

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope.

</deferred>

---

*Phase: 70-deployment-hardening*
*Context gathered: 2026-03-26*
