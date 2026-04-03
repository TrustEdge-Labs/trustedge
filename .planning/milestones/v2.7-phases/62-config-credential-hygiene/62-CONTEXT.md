<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


# Phase 62: Config & Credential Hygiene - Context

**Gathered:** 2026-03-25
**Status:** Ready for planning

<domain>
## Phase Boundary

Production deployments cannot start with hardcoded or placeholder credentials. DATABASE_URL requires explicit configuration in release builds, PostgreSQL is not exposed to the host network, and CAConfig rejects its placeholder JWT secret before reaching production code.

</domain>

<decisions>
## Implementation Decisions

### DATABASE_URL enforcement
- **D-01:** Gate the hardcoded fallback behind `cfg!(debug_assertions)`: debug builds keep the default `postgres://postgres:password@localhost:5432/trustedge` for dev convenience, release builds return `Err("DATABASE_URL must be set")` instead.
- **D-02:** Modify `Config::from_env()` in `crates/platform/src/http/config.rs:31`. The `#[cfg(feature = "postgres")]` gate stays — this only applies when postgres feature is enabled.
- **D-03:** Add a test that verifies release-mode behavior by calling the error path directly (can't compile-test `cfg!(debug_assertions)` easily, but can test the error message is produced when the env var is unset and debug_assertions is false).

### PostgreSQL port exposure
- **D-04:** Remove `ports: - "5432:5432"` from the postgres service in `deploy/docker-compose.yml`. The platform-server connects via the internal Docker network (`postgres:5432`), so no host binding is needed.
- **D-05:** Keep the comment noting that developers who need direct DB access can add `ports:` back manually or use `docker compose exec postgres psql`.

### CA placeholder JWT secret
- **D-06:** Add validation in `CAConfigBuilder::build()` that panics (or returns Result) if `jwt_secret` equals `"your-secret-key"`. This catches placeholder usage at construction time.
- **D-07:** Update existing tests in `ca/mod.rs` to explicitly set a test-specific JWT secret (e.g., `"test-jwt-secret-do-not-use-in-prod"`) instead of relying on `CAConfig::default()` values.
- **D-08:** Keep `CAConfig::default()` and `CAConfigBuilder::default()` as they are — the guard is at `build()` time, not at Default impl time. This preserves the builder pattern where you start from defaults and override.

### Claude's Discretion
- Whether `CAConfigBuilder::build()` should panic or return `Result<CAConfig, Error>` (both are acceptable — panic is simpler given current usage is tests-only)
- Exact wording of the DATABASE_URL error message
- Whether to add a dev-mode comment in docker-compose.yml about re-enabling the postgres port

</decisions>

<canonical_refs>
## Canonical References

No external specs — requirements fully captured in decisions above and in the security review findings table.

### Files to modify
- `crates/platform/src/http/config.rs` — Config::from_env() with DATABASE_URL fallback (Finding 4)
- `deploy/docker-compose.yml` — postgres service with host port binding (Finding 5)
- `crates/platform/src/ca/mod.rs` — CAConfig::default() and CAConfigBuilder with placeholder JWT (Finding 7)

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `Secret<T>` wrapper already used for `jwt_secret` in CAConfig — no new secret handling needed
- `anyhow::Result` already used in `Config::from_env()` — error return is natural

### Established Patterns
- Env-var config with `env::var().unwrap_or_else()` in config.rs — modify the unwrap_or_else closure
- `CAConfigBuilder` pattern with `.build()` — add validation before constructing CAConfig
- JWKS_KEY_PATH env var pattern from v2.5 Phase 55 — similar env-var enforcement approach

### Integration Points
- `Config::from_env()` is called from `crates/platform-server/src/main.rs` at startup — error here prevents server boot
- `CAConfigBuilder::build()` is called in tests within `ca/mod.rs` — tests must be updated to pass explicit JWT secret
- docker-compose.yml postgres service is referenced by platform-server via `DATABASE_URL` connection string using Docker internal DNS (`postgres:5432`)

</code_context>

<specifics>
## Specific Ideas

No specific requirements — standard credential hygiene patterns.

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope.

</deferred>

---

*Phase: 62-config-credential-hygiene*
*Context gathered: 2026-03-25*
