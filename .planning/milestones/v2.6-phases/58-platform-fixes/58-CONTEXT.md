# Phase 58: Platform Fixes - Context

**Gathered:** 2026-03-24
**Status:** Ready for planning

<domain>
## Phase Boundary

Fix `/v1/verify` handler in postgres mode (OrgContext extraction fails on public route) and make CORS origins configurable via environment variable.

</domain>

<decisions>
## Implementation Decisions

### Postgres verify fix
- **D-01:** Make `OrgContext` extraction optional in `verify_handler` — change `Extension(org_ctx): Extension<OrgContext>` to `org_ctx: Option<Extension<OrgContext>>` at handlers.rs:118.
- **D-02:** If `OrgContext` is present (request came through auth middleware), use `org_ctx.org_id` for tenant-scoped operations. If absent (public verify), operate in tenant-agnostic mode.
- **D-03:** This is the smallest change — `/v1/verify` stays as a public route in `build_base_router()`, no routing changes needed.

### CORS configuration
- **D-04:** Read CORS origins from `CORS_ORIGINS` environment variable, comma-separated. Example: `CORS_ORIGINS=https://app.example.com,https://admin.example.com`.
- **D-05:** When `CORS_ORIGINS` is not set, fall back to `http://localhost:3000,http://localhost:8080` (dev default, preserves current behavior).
- **D-06:** Applies to postgres builds only — non-postgres builds continue to use `CorsLayer::new()` (same-origin only).
- **D-07:** Parse each origin with `.parse::<HeaderValue>()`, skip invalid entries with a warning log.

### Claude's Discretion
- Whether to log the active CORS origins at startup
- Error handling for malformed CORS_ORIGINS entries
- Whether verify_handler should log when OrgContext is absent (useful for debugging)

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Verify handler
- `crates/platform/src/http/handlers.rs` — `verify_handler` postgres variant (line 116), `OrgContext` extraction (line 118), also `register_device_handler` (line 280) and `get_receipt_handler` (line 306) which correctly use OrgContext behind auth
- `crates/platform/src/http/auth.rs` — `OrgContext` struct (line 23), `auth_middleware` (line 58) that injects it

### CORS and router
- `crates/platform/src/http/router.rs` — `build_base_router()` (line 33), `create_router()` (line 44), hardcoded CORS origins (lines 80-81), non-postgres CorsLayer::new() (line 112)
- `crates/platform-server/src/main.rs` — Server startup, env var reading pattern

### Prior CORS hardening
- Phase 33 (v1.7) established: CorsLayer::new() for non-postgres, restricted headers for postgres

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `std::env::var()` pattern already used in main.rs for PORT, DATABASE_URL, JWT_AUDIENCE, JWKS_KEY_PATH, RATE_LIMIT_RPS
- `CorsLayer` from `tower-http` already imported in router.rs
- `Option<Extension<T>>` is a standard axum extractor pattern — no new dependencies

### Established Patterns
- Feature-gated code: `#[cfg(feature = "postgres")]` blocks in handlers.rs and router.rs
- Env var with fallback: `std::env::var("PORT").unwrap_or_else(|_| "3001".to_string())`
- CORS layer applied at router level in `create_router()`, not per-route

### Integration Points
- `verify_handler` postgres variant at handlers.rs:116 — OrgContext extraction change
- `create_router()` postgres block at router.rs:72-104 — CORS layer construction
- `main.rs` startup — CORS_ORIGINS env var logging

</code_context>

<specifics>
## Specific Ideas

No specific requirements — standard fixes with clear patterns.

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope.

</deferred>

---

*Phase: 58-platform-fixes*
*Context gathered: 2026-03-24*
