<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


---
phase: 55-platform-http-hardening
plan: 01
subsystem: api
tags: [axum, tower-http, governor, rate-limiting, body-limit, http, security]

requires:
  - phase: 54-transport-security
    provides: platform HTTP layer with Axum, CORS, TraceLayer, integration test infrastructure

provides:
  - RequestBodyLimitLayer (2 MB) on all HTTP routes to prevent body-flood OOM DoS
  - Per-IP rate limiting middleware (governor) applied exclusively to /v1/verify
  - RATE_LIMIT_RPS env var for configurable rate limit (default 10 req/sec per IP)
  - into_make_service_with_connect_info wiring in platform-server for ConnectInfo extraction
  - 4 integration tests proving 413, not-413, 429, and healthz-unthrottled behaviors

affects:
  - 55-02 (JWKS key path configurability - same platform HTTP layer)
  - platform-server deployment configuration

tech-stack:
  added:
    - governor 0.10 (dashmap feature) — keyed per-IP rate limiter
    - tower_http::limit::RequestBodyLimitLayer — body size enforcement
  patterns:
    - Rate limiting via nested sub-router with route_layer (only /v1/verify throttled)
    - ConnectInfo fallback to 127.0.0.1 for test-safe middleware
    - Env var read at router construction time (RATE_LIMIT_RPS)

key-files:
  created:
    - crates/platform/src/http/rate_limit.rs
  modified:
    - crates/platform/Cargo.toml
    - crates/platform/src/http/mod.rs
    - crates/platform/src/http/router.rs
    - crates/platform-server/src/main.rs
    - crates/platform/tests/verify_integration.rs

key-decisions:
  - "governor with dashmap feature for keyed per-IP rate limiter (no custom data structures)"
  - "rate limiter applied via nested sub-router route_layer — not a global layer — preserving healthz/jwks unthrottled"
  - "ConnectInfo fallback to 127.0.0.1 when absent enables test coverage without real TCP connections"
  - "RATE_LIMIT_RPS read at router construction time; tests set env var before creating app"
  - "RequestBodyLimitLayer placed after with_state() in both postgres and non-postgres code paths"

patterns-established:
  - "Route-scoped rate limiting: create sub-router, apply route_layer, merge into base router before with_state"
  - "Test-safe middleware: accept Option<ConnectInfo<SocketAddr>> and fall back to localhost"

requirements-completed: [HTTP-01, HTTP-02]

duration: 10min
completed: 2026-03-23
---

# Phase 55 Plan 01: Platform HTTP Hardening — Body Limit and Rate Limiting Summary

**2 MB RequestBodyLimitLayer on all routes + governor-based per-IP rate limiter on /v1/verify only, with 4 integration tests proving 413, 429, and healthz-unthrottled behaviors**

## Performance

- **Duration:** ~10 min
- **Started:** 2026-03-23T15:37:00Z
- **Completed:** 2026-03-23T15:47:41Z
- **Tasks:** 2
- **Files modified:** 6

## Accomplishments

- Added governor 0.10 (dashmap) as optional dep under the `http` feature in Cargo.toml
- Created `rate_limit.rs` with `RateLimitState` and `rate_limit_middleware` — falls back to 127.0.0.1 when `ConnectInfo` absent (test-safe)
- Restructured `router.rs` to apply `RequestBodyLimitLayer(2MB)` on all routes and per-IP rate limit only on `/v1/verify` via nested sub-router
- Updated `platform-server/src/main.rs` to use `into_make_service_with_connect_info::<SocketAddr>()` (required for ConnectInfo extraction at runtime)
- Added 4 integration tests: `test_body_limit_413`, `test_body_under_limit_not_413`, `test_rate_limit_429`, `test_healthz_not_rate_limited`
- All 22 platform integration tests pass (18 existing + 4 new)

## Task Commits

Each task was committed atomically:

1. **Task 1: Add governor, create rate_limit.rs, wire body limit and rate limiter into router** - `6f188b6` (feat)
2. **Task 2: Add integration tests for body limit (413) and rate limiting (429)** - `2f720d5` (test)

**Plan metadata:** `(pending final commit)` (docs: complete plan)

## Files Created/Modified

- `crates/platform/src/http/rate_limit.rs` - New: `RateLimitState`, `rate_limit_middleware` using governor keyed limiter
- `crates/platform/Cargo.toml` - Added `governor = { version = "0.10", features = ["dashmap"], optional = true }` under http feature
- `crates/platform/src/http/mod.rs` - Added `pub mod rate_limit;`
- `crates/platform/src/http/router.rs` - Added `RequestBodyLimitLayer(2MB)`, rate-limited verify sub-router; moved `/v1/verify` to `create_router`
- `crates/platform-server/src/main.rs` - Changed `axum::serve` to `into_make_service_with_connect_info::<SocketAddr>()`, added rate limit log
- `crates/platform/tests/verify_integration.rs` - Added 4 new integration tests (tests 10-13)

## Decisions Made

- Used governor with `dashmap` feature for the keyed rate limiter — `dashmap` is the recommended concurrent hash map for governor's keyed limiter
- Applied rate limiter only to `/v1/verify` via a nested sub-router with `route_layer`, so `/healthz` and `/.well-known/jwks.json` remain completely unthrottled
- `ConnectInfo` made `Option<ConnectInfo<SocketAddr>>` so middleware works without a real TCP connection (test environments using `tower::ServiceExt::oneshot`)
- `RATE_LIMIT_RPS` is read at router construction time; rate limit tests set env var before calling `create_router`, then immediately remove it

## Deviations from Plan

None — plan executed exactly as written. The `tower-http` `limit` feature was already present in the workspace Cargo.toml so no additional feature activation was needed.

## Issues Encountered

- First build attempt failed with `could not find limit in tower_http` — resolved by discovering the workspace `tower-http` already had `features = ["cors", "trace", "limit"]`; after the initial compile error from cached state, subsequent builds succeeded cleanly.

## User Setup Required

None — no external service configuration required.

## Next Phase Readiness

- HTTP hardening complete for body flood and verify-endpoint CPU abuse
- Platform server correctly wires `into_make_service_with_connect_info` for production ConnectInfo extraction
- Ready for Phase 55 Plan 02 (JWKS key path configurability)

---
*Phase: 55-platform-http-hardening*
*Completed: 2026-03-23*
