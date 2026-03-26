# Phase 64: Platform HTTP Hardening - Context

**Gathered:** 2026-03-25
**Status:** Ready for planning

<domain>
## Phase Boundary

Rate limiter correctly identifies clients behind reverse proxies using X-Forwarded-For from trusted sources, and 429 responses include a Retry-After header per RFC 6585. Only the rate_limit.rs middleware changes.

</domain>

<decisions>
## Implementation Decisions

### Trusted proxy configuration
- **D-01:** Add `TRUSTED_PROXIES` env var — comma-separated IPs/CIDRs (e.g., `"172.16.0.0/12,10.0.0.0/8"`). Only parse X-Forwarded-For when the direct TCP peer IP is in this list. When empty or unset, current behavior is preserved (use ConnectInfo peer IP only).
- **D-02:** Parse the trusted proxy list at startup in `RateLimitState::new()` or a separate config function. Store as `Vec<IpNet>` or similar for efficient subnet matching.
- **D-03:** When peer IP is trusted and X-Forwarded-For is present, use the rightmost non-trusted IP from the header (standard proxy chain parsing — rightmost entry is set by the last trusted proxy).
- **D-04:** When peer IP is not trusted or X-Forwarded-For is absent, fall back to ConnectInfo peer IP (current behavior). The 127.0.0.1 test fallback also stays.

### Retry-After header
- **D-05:** Return `Retry-After: 1` (fixed 1 second) on all 429 responses. Simple, predictable, matches per-second quota granularity.
- **D-06:** Change the error return from bare `Err(StatusCode::TOO_MANY_REQUESTS)` to a full `Response` with status 429 + Retry-After header. This requires changing the middleware return type slightly.

### Testing
- **D-07:** Add integration tests proving: (a) X-Forwarded-For is respected when peer is in TRUSTED_PROXIES, (b) X-Forwarded-For is ignored when peer is NOT trusted, (c) 429 response includes Retry-After header.

### Claude's Discretion
- Whether to use the `ipnet` crate or manual CIDR parsing
- Exact storage type for parsed proxy list (Vec<IpNet>, HashSet, etc.)
- Whether TRUSTED_PROXIES parsing goes in RateLimitState or a separate Config field

</decisions>

<canonical_refs>
## Canonical References

No external specs — requirements fully captured in decisions above and in the security review findings table.

### Files to modify
- `crates/platform/src/http/rate_limit.rs` — Rate limiter middleware with IP extraction and 429 response (Findings 8, 9)

### Related files
- `crates/platform/src/http/config.rs` — Config::from_env() where TRUSTED_PROXIES could be parsed
- `crates/platform/tests/verify_integration.rs` — Existing integration tests for the verify endpoint (has rate limit tests)

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `governor::DefaultKeyedRateLimiter<IpAddr>` already in use — no new rate limiter needed
- `ConnectInfo<SocketAddr>` extraction already present — add X-Forwarded-For as a second source
- Existing env var pattern: `env::var("RATE_LIMIT_RPS")` in config — follow for TRUSTED_PROXIES

### Established Patterns
- Rate limiter is applied as Axum middleware via `route_layer` on the /v1/verify sub-router
- 429 currently returned as bare `Err(StatusCode::TOO_MANY_REQUESTS)` — needs to become a Response with headers
- Test pattern uses `axum_test` crate with `create_test_app()` for HTTP integration tests

### Integration Points
- `rate_limit_middleware` is the only function that needs to change
- `RateLimitState` may need a `trusted_proxies` field
- Docker compose has nginx in front of platform-server — the typical TRUSTED_PROXIES value would be the Docker network CIDR

</code_context>

<specifics>
## Specific Ideas

No specific requirements — standard proxy-aware rate limiting patterns.

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope.

</deferred>

---

*Phase: 64-platform-http-hardening*
*Context gathered: 2026-03-25*
