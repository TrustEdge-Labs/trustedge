---
phase: 25-service-consolidation
plan: 02
subsystem: platform
tags: [rust, axum, sqlx, postgres, ed25519, blake3, sha2, tower-http, cors]

# Dependency graph
requires:
  - phase: 25-01
    provides: trustedge-platform crate with verify engine and CA module skeleton

provides:
  - database/queries.rs: 10 PostgreSQL CRUD functions for multi-tenant platform data
  - migrations/001_create_multi_tenant_schema.sql: full schema (organizations, users, api_keys, devices, verifications, receipts, policies)
  - http/handlers.rs: verify_handler (inline, no HTTP forwarding), register_device, get_receipt, jwks, health
  - http/auth.rs: Bearer token auth middleware (SHA-256 hash lookup), generate_token
  - http/router.rs: create_router() with CORS + TraceLayer + auth middleware
  - http/state.rs: consolidated AppState (no verify_core_url)
  - http/config.rs: Config from env vars (no verify_core_url)

affects:
  - 25-03 (binary crate that wires AppState and starts the HTTP server)
  - 26-crypto-deduplication (uses trustedge-platform verify module)

# Tech tracking
tech-stack:
  added:
    - sha2 0.10 (token hashing + manifest digest for DB; available under http and postgres features)
    - dotenvy 0.15 (env var loading; available under http and postgres features)
    - bcrypt 0.15 (postgres-gated; available for future password hashing needs)
  patterns:
    - Inline verification: verify_handler calls verify_to_report() directly (reqwest eliminated)
    - Dual manifest digest: blake3 for receipt construction, sha2 for DB storage compatibility
    - Feature-gated handlers: postgres-gated handlers use Extension<OrgContext> from auth middleware
    - Independent feature gates: http and postgres can be used separately or together

key-files:
  created:
    - crates/platform/migrations/001_create_multi_tenant_schema.sql
    - crates/platform/src/database/mod.rs
    - crates/platform/src/database/queries.rs
    - crates/platform/src/http/mod.rs
    - crates/platform/src/http/state.rs
    - crates/platform/src/http/config.rs
    - crates/platform/src/http/auth.rs
    - crates/platform/src/http/handlers.rs
    - crates/platform/src/http/router.rs
  modified:
    - crates/platform/src/lib.rs (replaced placeholders with real pub mod declarations)
    - crates/platform/Cargo.toml (sha2 + dotenvy moved to http feature; test-utils feature added)

key-decisions:
  - "sha2 and dotenvy available under both http and postgres features — both need them independently"
  - "verify_core_url removed from AppState and Config — verification is inline via verify_to_report()"
  - "Dual manifest digest strategy: blake3 for receipt JWS construction, sha2 for DB storage (platform-api schema compatibility)"
  - "parse_jws_claims gated behind postgres feature — only used by get_receipt_handler"
  - "postgres feature does NOT depend on http feature — they remain independent"

requirements-completed: [SVC-01, SVC-03]

# Metrics
duration: 5min
completed: 2026-02-21
---

# Phase 25 Plan 02: HTTP Layer and Database Module Summary

**Axum HTTP layer with inline verification (no HTTP forwarding), Bearer token auth, PostgreSQL CRUD, and CORS+TraceLayer router**

## Performance

- **Duration:** 5 min
- **Started:** 2026-02-21T23:59:09Z
- **Completed:** 2026-02-21T00:04:00Z
- **Tasks:** 2
- **Files modified:** 11

## Accomplishments

- Database module: 10 CRUD functions ported from trustedge-platform-api covering organizations, API keys, devices, verifications, and receipts
- PostgreSQL migration SQL: 7-table multi-tenant schema (organizations, users, api_keys, devices, verifications, receipts, policies) with 12 performance indexes
- HTTP layer: Axum router combining verify-service endpoints (verify, jwks, health) and platform-api endpoints (devices, receipts) in one `create_router()` function
- Key consolidation: `verify_handler` calls `verify_to_report()` directly — reqwest dependency eliminated, no HTTP forwarding to a separate verify-core service
- JWKS endpoint serves from local `KeyManager` — no proxy
- Feature gates work independently: `http` (stateless verify mode) and `postgres` (full multi-tenant mode) can be combined or used separately

## Task Commits

Each task was committed atomically:

1. **Task 1: Create database module and migrations** - `fdb386c` (feat)
2. **Task 2: Create HTTP layer with consolidated router and handlers** - `46d31ee` (feat)

## Files Created/Modified

- `crates/platform/migrations/001_create_multi_tenant_schema.sql` - Multi-tenant PostgreSQL schema (7 tables, 12 indexes)
- `crates/platform/src/database/mod.rs` - Database module root with re-exports
- `crates/platform/src/database/queries.rs` - 10 CRUD functions using sqlx runtime queries
- `crates/platform/src/http/mod.rs` - HTTP module root with re-exports
- `crates/platform/src/http/state.rs` - Consolidated AppState (db_pool postgres-gated, KeyManager always-on)
- `crates/platform/src/http/config.rs` - Config from env vars (database_url postgres-gated, no verify_core_url)
- `crates/platform/src/http/auth.rs` - Bearer token auth middleware (postgres-gated) + token utilities (always-on)
- `crates/platform/src/http/handlers.rs` - All HTTP handlers; verify_handler with inline verification
- `crates/platform/src/http/router.rs` - Axum router with CORS + TraceLayer; auth middleware postgres-gated
- `crates/platform/src/lib.rs` - Placeholder comments replaced with real pub mod declarations
- `crates/platform/Cargo.toml` - sha2 + dotenvy now available under both http and postgres features

## Decisions Made

- `sha2` and `dotenvy` are available under both `http` and `postgres` features: `auth.rs` uses sha2 for token hashing and `config.rs` uses dotenvy for env loading — both are HTTP-layer files that need these deps regardless of postgres.
- `verify_core_url` removed from `AppState` and `Config` — this is the key consolidation change. The verify handler now calls `verify_to_report()` directly.
- Dual manifest digest: BLAKE3 used for receipt JWS construction (consistent with verify-service convention); SHA-256 used for DB storage (compatible with existing platform-api `manifest_digest` column schema).
- `postgres` feature does NOT depend on `http` feature — they remain independent. You can use postgres without the HTTP layer (e.g., for migration tooling) or http without postgres (stateless verification mode).

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] sha2 and dotenvy not available under http feature**
- **Found during:** Task 2 (http feature compilation)
- **Issue:** `auth.rs` imports `sha2` and `config.rs` imports `dotenvy`, but both deps were only in the `postgres` feature. Building with just `--features http` failed.
- **Fix:** Moved sha2 and dotenvy to be available under both `http` and `postgres` features in Cargo.toml
- **Files modified:** `crates/platform/Cargo.toml`
- **Committed in:** `46d31ee`

**2. [Rule 1 - Bug] serde_json::Value has no is_empty() method**
- **Found during:** Task 2 (http feature compilation)
- **Issue:** Platform-api source used `req.manifest.is_empty()` where manifest was `String`. In the consolidated crate, manifest is `serde_json::Value` — no `is_empty()` method exists.
- **Fix:** Replaced with `request.manifest.is_null() || request.manifest == serde_json::Value::Object(Default::default())`
- **Files modified:** `crates/platform/src/http/handlers.rs`
- **Committed in:** `46d31ee`

**3. [Rule 2 - Missing Critical] parse_jws_claims needed postgres gate**
- **Found during:** Task 2 (clippy warning in http-only build)
- **Issue:** `parse_jws_claims` was defined ungated but only called from postgres-gated `get_receipt_handler`, causing a dead_code warning in http-only builds
- **Fix:** Added `#[cfg(feature = "postgres")]` to `parse_jws_claims`
- **Files modified:** `crates/platform/src/http/handlers.rs`
- **Committed in:** `46d31ee`

---

**Total deviations:** 3 auto-fixed (2 bugs, 1 missing feature gate)
**Impact on plan:** All fixes necessary for correct compilation across all feature combinations. No scope creep.

## Verification Results

- `cargo build -p trustedge-platform` (no features): PASSED
- `cargo build -p trustedge-platform --features http` (stateless mode): PASSED
- `cargo build -p trustedge-platform --features postgres`: PASSED
- `cargo build -p trustedge-platform --features "http,postgres"`: PASSED
- `cargo build -p trustedge-platform --features "http,postgres,ca"`: PASSED
- `cargo clippy -p trustedge-platform --features "http,postgres" -- -D warnings`: PASSED
- `cargo test -p trustedge-platform --lib`: PASSED (12/12 tests)
- No `reqwest` in crate: VERIFIED
- No `verify_core_url` in code: VERIFIED

## Next Phase Readiness

- Plan 03 (binary crate) can now wire `AppState` and call `create_router()` to start the HTTP server
- `create_connection_pool` and `run_migrations` are available for startup logic
- `Config::from_env()` is ready for environment-based configuration
- No blockers.

## Self-Check: PASSED

- crates/platform/migrations/001_create_multi_tenant_schema.sql: FOUND
- crates/platform/src/database/queries.rs: FOUND
- crates/platform/src/http/handlers.rs: FOUND
- crates/platform/src/http/router.rs: FOUND
- Task 1 commit fdb386c: FOUND
- Task 2 commit 46d31ee: FOUND

---
*Phase: 25-service-consolidation*
*Completed: 2026-02-21*
