<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


---
phase: 64-platform-http-hardening
plan: 01
subsystem: api
tags: [rate-limiting, proxy, ipnet, axum, http-hardening]

# Dependency graph
requires: []
provides:
  - Proxy-aware rate limiting keyed on real client IP via X-Forwarded-For when peer is a trusted proxy CIDR
  - Retry-After: 1 header on all 429 Too Many Requests responses (RFC 6585)
  - TRUSTED_PROXIES env var support in create_router for production reverse-proxy deployments
affects: [platform-http, platform-server, docker-nginx-deploy]

# Tech tracking
tech-stack:
  added: [ipnet = "2" (optional, http feature)]
  patterns:
    - Proxy-aware IP extraction: trusted CIDR list + XFF header traversal (rightmost non-trusted IP)
    - Rate limiter keyed on effective_ip not peer_ip

key-files:
  created: []
  modified:
    - crates/platform/Cargo.toml
    - crates/platform/src/http/rate_limit.rs
    - crates/platform/src/http/router.rs
    - crates/platform/tests/verify_integration.rs

key-decisions:
  - "Use ipnet crate (already transitive dep) for CIDR containment checks — avoids re-implementing IP network math"
  - "Return Response (not Result<Response, StatusCode>) from rate_limit_middleware so Retry-After header can be set on 429"
  - "Trusted proxy config via TRUSTED_PROXIES env var (comma-separated CIDRs) — consistent with existing RATE_LIMIT_RPS pattern"
  - "Rightmost non-trusted IP from X-Forwarded-For chain — standard anti-spoofing convention"

patterns-established:
  - "Pattern: extract_client_ip helper is a pure function testable without Axum — receives HeaderMap + trusted CIDR slice"
  - "Pattern: env var parsing at router construction time, not at middleware execution time"

requirements-completed: [HTTP-01, HTTP-02]

# Metrics
duration: 25min
completed: 2026-03-25
---

# Phase 64 Plan 01: Platform HTTP Hardening — Rate Limiter Summary

**Proxy-aware per-client-IP rate limiting with TRUSTED_PROXIES CIDR config and RFC 6585 Retry-After: 1 header on 429 responses**

## Performance

- **Duration:** ~25 min
- **Started:** 2026-03-25T22:10:00Z
- **Completed:** 2026-03-25T22:35:00Z
- **Tasks:** 2
- **Files modified:** 4

## Accomplishments

- Added `extract_client_ip` helper that traverses X-Forwarded-For chains right-to-left, skipping trusted proxy CIDRs, to identify the real client IP behind a reverse proxy
- Updated `RateLimitState::new` to accept `Vec<ipnet::IpNet>` trusted proxies; router now reads `TRUSTED_PROXIES` env var (comma-separated CIDRs) at startup
- Changed `rate_limit_middleware` return type from `Result<Response, StatusCode>` to `Response`; 429 responses now include `Retry-After: 1` header per RFC 6585
- 9 new tests total: 6 unit tests (rate_limit.rs) + 3 integration tests (verify_integration.rs); all 27 integration tests pass

## Task Commits

Each task was committed atomically:

1. **Task 1: Implement proxy-aware IP extraction and Retry-After header in rate limiter** - `8453de3` (feat)
2. **Task 2: Add integration tests for proxy-aware rate limiting and Retry-After** - `7c55c7d` (test)

**Plan metadata:** (docs commit — see below)

## Files Created/Modified

- `crates/platform/Cargo.toml` - Added `ipnet = "2"` optional dep; added to `http` feature list
- `crates/platform/src/http/rate_limit.rs` - Added `trusted_proxies` field, `extract_client_ip` fn, Retry-After header, 6 unit tests
- `crates/platform/src/http/router.rs` - Reads `TRUSTED_PROXIES` env var; passes to `RateLimitState::new`
- `crates/platform/tests/verify_integration.rs` - Added Tests 16–18: retry_after, xff_trusted_proxy, xff_untrusted_proxy

## Decisions Made

- Used `ipnet` crate for CIDR containment (already a transitive dep in Cargo.lock — no new transitive dependencies)
- Changed middleware return type to `Response` rather than `Result<Response, StatusCode>` so custom headers can be set on the error response
- Rightmost non-trusted IP approach in XFF chain traversal — the de-facto standard to prevent client spoofing by injecting extra IPs at the left of the header

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

- `cargo fmt` reformatted two minor style issues (multi-line iterator chain and long `.expect()` call). Applied `cargo fmt` and re-verified. No functional changes.

## User Setup Required

To enable proxy-aware rate limiting in production, set the `TRUSTED_PROXIES` environment variable:

```
TRUSTED_PROXIES=10.0.0.0/8,172.16.0.0/12
```

When unset or empty, prior behavior is preserved (peer IP only).

## Next Phase Readiness

- Rate limiter now production-safe behind Docker nginx or any reverse proxy
- All 27 integration tests green; CI passes for platform crate
- Ready for remaining phase 64 plans

## Self-Check: PASSED

- `crates/platform/src/http/rate_limit.rs` — FOUND
- `crates/platform/src/http/router.rs` — FOUND
- `.planning/phases/64-platform-http-hardening/64-01-SUMMARY.md` — FOUND
- Commit `8453de3` — FOUND (feat: Task 1)
- Commit `7c55c7d` — FOUND (test: Task 2)
- All acceptance criteria patterns verified (trusted_proxies, extract_client_ip, retry-after, TRUSTED_PROXIES, ipnet)

---
*Phase: 64-platform-http-hardening*
*Completed: 2026-03-25*
