<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


# Phase 67: Deployment Security - Context

**Gathered:** 2026-03-25
**Status:** Ready for planning

<domain>
## Phase Boundary

Dashboard nginx runs as non-root user. CI bundle credential guard (VITE_API_KEY grep) added to GitHub Actions ci.yml workflow. Two independent deployment security fixes.

</domain>

<decisions>
## Implementation Decisions

### nginx non-root (DEPL-01)
- **D-01:** Switch from `nginx:alpine` to `nginxinc/nginx-unprivileged:alpine` in `deploy/Dockerfile.dashboard` runtime stage. This image runs as uid 101 (nginx) by default and listens on port 8080 instead of 80.
- **D-02:** Update port mappings: internal container port changes from 80 to 8080. The `docker-compose.yml` dashboard service maps `"8080:8080"` instead of `"8080:80"`. HTTPS port 443 changes to 8443 internally.
- **D-03:** Update the healthcheck in both Dockerfile.dashboard and docker-compose.yml to use port 8080: `wget -qO- http://localhost:8080/healthz`.
- **D-04:** The `docker-entrypoint.sh` script needs adjustment — `nginx-unprivileged` can't write to `/etc/nginx/conf.d/` as non-root. Either: use `/tmp/` for generated SSL config, or pre-create the config location with correct permissions in the Dockerfile.
- **D-05:** The `EXPOSE` directive changes from `80 443` to `8080 8443`.

### CI bundle credential guard (DEPL-02)
- **D-06:** Add a new step to `.github/workflows/ci.yml` in the `lint` job (fast checks, no compilation) that builds the dashboard and greps the output for `VITE_API_KEY`. Mirror the logic from `scripts/ci-check.sh` Step 12.
- **D-07:** The step should: `cd web/dashboard && npm install && npm run build`, then `grep -r "VITE_API_KEY" build/` — if found, fail with an error message.
- **D-08:** This step runs only if `web/dashboard/package.json` exists (conditional on dashboard being present).

### Claude's Discretion
- Whether to use `nginx-unprivileged:alpine` or `nginx-unprivileged:stable-alpine` tag
- How to handle the entrypoint SSL config writing with non-root permissions
- Whether the CI step uses `npm ci` (lockfile strict) or `npm install`

</decisions>

<canonical_refs>
## Canonical References

No external specs — requirements fully captured in decisions above.

### Files to modify
- `deploy/Dockerfile.dashboard` — nginx base image + port changes (Finding 15)
- `deploy/docker-compose.yml` — dashboard port mapping + healthcheck (Finding 15)
- `deploy/docker-entrypoint.sh` — adapt for non-root nginx (Finding 15)
- `.github/workflows/ci.yml` — add bundle credential guard step (Finding 16)

### Reference patterns
- `scripts/ci-check.sh` Step 12 — existing local credential guard (Finding 16, lines 302-308)
- Phase 61 SHA-pinning — ci.yml already SHA-pinned, new step needs SHA-pinned actions too

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `scripts/ci-check.sh` Step 12 — exact grep logic to replicate in ci.yml
- `deploy/docker-entrypoint.sh` — existing TLS conditional logic needs adaptation
- `deploy/nginx.conf` and `deploy/nginx-ssl.conf.template` — may need port updates

### Established Patterns
- All CI workflow actions are SHA-pinned (Phase 61) — any new action refs must use full SHAs
- Dashboard healthcheck uses `wget -qO-` pattern
- docker-compose uses `condition: service_healthy` for dependencies

### Integration Points
- docker-compose dashboard service depends on platform-server healthcheck
- nginx serves static files from `/usr/share/nginx/html` — same path works in nginx-unprivileged
- The entrypoint writes SSL config conditionally — needs writable path as non-root

</code_context>

<specifics>
## Specific Ideas

No specific requirements — standard deployment hardening patterns.

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope.

</deferred>

---

*Phase: 67-deployment-security*
*Context gathered: 2026-03-25*
