<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


# Phase 39: Deployment Stack - Context

**Gathered:** 2026-03-15
**Status:** Ready for planning

<domain>
## Phase Boundary

Users can start the entire TrustEdge platform with a single `docker-compose up` command from the deploy/ directory. All three services (platform server, postgres, dashboard) start, connect, and become healthy without manual configuration. This phase updates the existing deploy/ artifacts — it does not add production hardening, CI/CD, or TLS termination.

</domain>

<decisions>
## Implementation Decisions

### Dashboard Containerization
- Static build + nginx: SvelteKit static adapter, multi-stage Dockerfile (node builder -> nginx:alpine runtime)
- Dockerfile lives at `deploy/Dockerfile.dashboard` alongside existing Dockerfile
- Switch SvelteKit from auto adapter to static adapter (`@sveltejs/adapter-static`)
- Dashboard added as third service in docker-compose.yml

### Schema Initialization
- Platform server runs `migrate` then `serve` via docker-compose command override: `["sh", "-c", "trustedge-platform-server migrate && trustedge-platform-server serve"]`
- Migrations are idempotent — safe to run on every container restart (use IF NOT EXISTS / CREATE OR REPLACE)
- No separate migration step required from the user

### Networking & Ports
- Browser-direct API access: dashboard JS fetches from localhost:3001, platform-server on its own port
- Dashboard exposed on port 8080 (nginx)
- Platform server on port 3001 (existing)
- Postgres on port 5432 exposed to host (existing, useful for debugging)
- CORS already configured on platform server — no proxy needed

### Developer Experience
- Zero-config: all env vars have sensible defaults inline in docker-compose.yml — no .env file copy required for demo
- .env.example kept for customization reference
- Build from source on first `docker-compose up` (Rust compile is slow but one-time, Docker layer caching handles rebuilds)
- No pre-built images — demo users build locally
- Health checks on all three services (postgres already has one; add to platform-server and dashboard)
- No startup script wrapper — `docker-compose ps` shows health status

### Claude's Discretion
- Nginx config for SvelteKit static files (fallback routing for SPA)
- Health check commands and intervals for platform-server and dashboard
- Whether to add a .dockerignore for the dashboard build context
- Exact env var defaults in docker-compose.yml

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Existing deployment artifacts
- `deploy/Dockerfile` — Current platform-server multi-stage build (debian-slim, --features postgres)
- `deploy/docker-compose.yml` — Current postgres + platform-server setup (no dashboard yet)
- `deploy/.env.example` — Environment variable documentation (DATABASE_URL, PORT, JWT_AUDIENCE)

### Platform server
- `crates/platform-server/src/main.rs` — CLI with serve/migrate subcommands, Config::from_env()
- `crates/platform/migrations/001_create_multi_tenant_schema.sql` — Schema migration SQL

### Dashboard
- `web/dashboard/src/lib/config.ts` — API connection config (VITE_API_BASE defaults to localhost:3001)
- `web/dashboard/package.json` — Build scripts, current adapter configuration

### Requirements
- `.planning/REQUIREMENTS.md` — DEPL-01 through DEPL-04 define success criteria

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `deploy/Dockerfile`: Multi-stage Rust build pattern — can be referenced for dashboard Dockerfile
- `deploy/docker-compose.yml`: Postgres service with healthcheck already defined
- `trustedge-platform-server migrate` command: Already implemented, uses sqlx migrations
- `web/dashboard/src/lib/config.ts`: API base URL configurable via VITE_API_BASE env var

### Established Patterns
- Multi-stage Docker builds (builder -> slim runtime)
- Environment-based configuration (Config::from_env() in platform-server)
- Health checks via pg_isready for postgres
- Platform server has /healthz endpoint

### Integration Points
- `deploy/docker-compose.yml` — Add dashboard service, update platform-server command
- `deploy/Dockerfile.dashboard` — New file for dashboard container
- `web/dashboard/svelte.config.js` — Switch to static adapter
- `web/dashboard/package.json` — Add @sveltejs/adapter-static dependency
- Platform server /healthz — Health check target for docker-compose

</code_context>

<specifics>
## Specific Ideas

- The current docker-compose.yml explicitly says "Dashboard runs natively: cd web/dashboard && npm run dev" — this phase replaces that with containerized dashboard
- Platform server Dockerfile already builds with `--features postgres` which is correct for the full stack
- `VITE_API_BASE` must be set at build time (SvelteKit static build bakes env vars in) — needs build arg in Dockerfile

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope.

</deferred>

---

*Phase: 39-deployment-stack*
*Context gathered: 2026-03-15*
