# Phase 71: Platform Code Quality - Context

**Gathered:** 2026-03-26
**Status:** Ready for planning

<domain>
## Phase Boundary

Make platform configuration explicit and fail-safe: configurable JWS receipt TTL via env var, remove version fingerprint from /healthz, and fail on invalid PORT instead of silently defaulting.

</domain>

<decisions>
## Implementation Decisions

### Receipt TTL Configuration
- **D-01:** Add `RECEIPT_TTL_SECS` env var to `Config::from_env()` in `platform/src/http/config.rs`, following the existing pattern for PORT, DATABASE_URL, and JWT_AUDIENCE. Default remains 3600 seconds. The value flows through `Config` struct → `AppState` → `sign_receipt_jws()`.

### Healthz Response
- **D-02:** Remove the `version` field from `HealthResponse` struct entirely. Return only `{"status":"OK","timestamp":"..."}`. No auth gating, no generic replacement string — just drop the field. The `HealthResponse` struct in `verify/types.rs` loses the `version` field.

### PORT Failure Mode
- **D-03:** If `PORT` env var is set but unparseable, hard fail with a clear error message and non-zero exit. If `PORT` is not set at all, default to 3001 as today. Logic: "if you set it, you meant something."

### Claude's Discretion
- How to thread receipt_ttl through from Config to sign_receipt_jws — via AppState field or parameter is Claude's call
- Whether to add a test for the new RECEIPT_TTL_SECS env var parsing

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Platform HTTP config
- `crates/platform/src/http/config.rs` — Config::from_env() with PORT, DATABASE_URL, JWT_AUDIENCE pattern
- `crates/platform/src/http/handlers.rs` — health_handler() at line 51, verify_handler implementations

### Receipt signing
- `crates/platform/src/verify/signing.rs` — sign_receipt_jws() with hardcoded TTL at line 29

### Types
- `crates/platform/src/verify/types.rs` — HealthResponse struct with version field at line 43

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `Config::from_env()` in `config.rs` — established pattern for reading env vars with defaults and error handling
- `AppState` struct — carries `KeyManager` and JWKS state; natural place for receipt_ttl config

### Established Patterns
- Env var reading: `env::var("NAME").unwrap_or_else(|_| default)` for optional vars
- Database URL enforces release-mode requirement via `cfg!(debug_assertions)` — model for strict validation
- Config flows via Axum `State<AppState>` extractor

### Integration Points
- `sign_receipt_jws(receipt, key_manager)` signature needs receipt_ttl parameter (or access to config)
- `health_handler()` is a standalone function — removing version field is isolated change
- `HealthResponse` may be referenced in integration tests — check verify_integration tests

</code_context>

<specifics>
## Specific Ideas

No specific requirements — standard env var pattern, straightforward struct modification, and parse error propagation.

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope

</deferred>

---

*Phase: 71-platform-code-quality*
*Context gathered: 2026-03-26*
