<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


# Phase 55: Platform HTTP Hardening - Context

**Gathered:** 2026-03-23
**Status:** Ready for planning

<domain>
## Phase Boundary

Protect HTTP platform endpoints against body-flood DoS, CPU-exhaustion abuse via /v1/verify, and plaintext signing key leakage. Add request body size limit, rate limiting middleware, and configurable signing key storage.

</domain>

<decisions>
## Implementation Decisions

### Body size limit
- **D-01:** Apply a 2 MB global `RequestBodyLimitLayer` via `build_base_router()`. This covers all routes consistently — `/v1/verify`, `/.well-known/jwks.json`, `/healthz`, and any postgres-mode routes.
- **D-02:** Use `tower_http::limit::RequestBodyLimitLayer` which is already available via the `tower-http` dependency.

### Rate limiting
- **D-03:** Use `governor` crate + `tower_governor` middleware for in-process, per-IP rate limiting on `/v1/verify`.
- **D-04:** Return HTTP 429 Too Many Requests when rate limit is exceeded.
- **D-05:** Rate limit should be configurable via environment variable (e.g., `RATE_LIMIT_RPS` defaulting to 10 requests per second per IP).
- **D-06:** Rate limiting applies to `/v1/verify` only — `/healthz` and `/.well-known/jwks.json` should remain unthrottled.

### Signing key storage
- **D-07:** Read signing key file path from `JWKS_KEY_PATH` environment variable. Remove all hardcoded `"target/dev/"` references.
- **D-08:** Default to a runtime temp directory (e.g., system temp dir) when `JWKS_KEY_PATH` is not set — never default to `target/dev/`.
- **D-09:** Key is generated fresh on startup if the file at the configured path doesn't exist. For production: user mounts a persistent path.
- **D-10:** The JWKS public key file (`jwks.json`) should follow the same pattern — derive its path from `JWKS_KEY_PATH` or co-locate with the signing key.
- **D-11:** Set 0600 Unix permissions on the signing key file (consistent with the pattern established in v2.4 Phase 52 for device key files).

### Claude's Discretion
- Exact governor configuration (burst size, quota algorithm)
- Whether to add rate limit headers (X-RateLimit-Remaining, etc.)
- Temp directory strategy (std::env::temp_dir vs platform-specific)
- Error response body format for 413 and 429 responses

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Platform HTTP layer
- `crates/platform/src/http/router.rs` — `build_base_router()` (line 33) is the single source of truth for routes. `create_router()` (line 44) composes the full router. `CorsLayer` and `TraceLayer` already applied here.
- `crates/platform/src/http/handlers.rs` — verify_handler implementation
- `crates/platform/src/http/state.rs` — AppState struct definition

### JWKS key management
- `crates/platform/src/verify/jwks.rs` — `KeyManager::new()` (line 34), `save_to_file()` (line 80), `write_jwks_file()` (line 97), `rotate()` (line 146). All 4 locations hardcode `target/dev/` paths.

### Platform server binary
- `crates/platform-server/src/main.rs` — Server startup, reads env vars (PORT, DATABASE_URL, JWT_AUDIENCE). JWKS_KEY_PATH should follow same pattern.

### Prior security hardening
- `.planning/phases/33-platform-quality/33-CONTEXT.md` — CORS hardening decisions from v1.7
- `.planning/phases/34-platform-testing/34-CONTEXT.md` — Platform integration test patterns

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `tower-http` already in deps — `RequestBodyLimitLayer` available without new dependency
- `build_base_router()` is the single place to add global middleware layers
- Server binary already reads env vars via `std::env::var()` pattern for PORT, DATABASE_URL, JWT_AUDIENCE
- v2.4 Phase 52 established the 0600 permissions pattern for key files — reusable in jwks.rs

### Established Patterns
- Middleware layering: `.layer(CorsLayer::new()).layer(TraceLayer::new_for_http())` in router.rs
- Environment-driven config: `std::env::var("PORT").unwrap_or_else(|_| "3001".to_string())`
- Feature-gated routes: `#[cfg(feature = "postgres")]` blocks in create_router
- base64 key storage: `StoredKey` struct in jwks.rs uses serde + base64 for key serialization

### Integration Points
- `build_base_router()` — where body limit layer goes (global)
- `/v1/verify` route — where rate limit layer goes (route-specific)
- `KeyManager::new()` — where env var path loading goes
- `KeyManager::save_to_file()` and `write_jwks_file()` — where path parameterization goes
- `crates/platform-server/src/main.rs` — where new env vars get documented in startup banner

</code_context>

<specifics>
## Specific Ideas

No specific requirements — standard hardening patterns apply.

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope.

</deferred>

---

*Phase: 55-platform-http-hardening*
*Context gathered: 2026-03-23*
