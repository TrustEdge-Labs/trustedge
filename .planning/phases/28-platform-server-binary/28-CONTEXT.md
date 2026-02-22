# Phase 28: Platform Server Binary - Context

**Gathered:** 2026-02-22
**Status:** Ready for planning

<domain>
## Phase Boundary

Create a standalone binary crate `crates/platform-server` that boots the `trustedge-platform` Axum HTTP server. The binary is a thin entry point — all routing, handlers, and business logic live in `trustedge-platform`. Includes deployment artifacts (Dockerfile, docker-compose).

</domain>

<decisions>
## Implementation Decisions

### Configuration loading
- .env file + env vars via dotenvy — load .env if present, env vars override
- If DATABASE_URL is not set, start in verify-only mode (stateless endpoints: /v1/verify, JWKS, healthz) — no device/receipt endpoints
- Default PORT: 3001 (matches existing Config::from_env() default)
- Ship .env.example with all vars documented (PORT, DATABASE_URL, JWT_AUDIENCE with explanatory comments)

### Startup behavior
- Migrations are a separate subcommand (`trustedge-platform-server migrate`), NOT auto-run on start
- Minimal banner on startup: name, version, port, mode (verify-only vs full/postgres)
- Print active routes/endpoints on startup — show which routes are live based on enabled features
- Use clap for CLI structure with subcommands: `serve` (default), `migrate`

### Feature composition
- Default features: `http` + `postgres` + `openapi`
- CA available as opt-in feature (`--features ca`)
- Binary name: `trustedge-platform-server`
- Direct dependency on both `trustedge-core` and `trustedge-platform`
- Verify-only mode is a runtime decision (no DATABASE_URL), not a compile-time feature

### Deployment model
- Multi-stage Dockerfile: builder stage compiles Rust, runtime stage is minimal (debian-slim)
- docker-compose.yml with postgres + platform-server (backend only — no dashboard)
- All deployment artifacts live in `deploy/` directory at repo root
- Dashboard runs natively with npm, not containerized in compose

### Claude's Discretion
- Exact banner format and styling
- tracing vs println for startup output
- Dockerfile base image selection (debian-slim vs alpine vs distroless)
- docker-compose postgres version and volume configuration
- clap derive vs builder API

</decisions>

<specifics>
## Specific Ideas

- The platform crate already exposes `create_router(AppState)`, `AppState`, `Config::from_env()`, `create_connection_pool()`, `run_migrations()` — main.rs should be thin, wiring these together
- Verify-only mode should feel intentional, not degraded — it's a valid deployment for stateless verification services

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope

</deferred>

---

*Phase: 28-platform-server-binary*
*Context gathered: 2026-02-22*
