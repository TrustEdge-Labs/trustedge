# Phase 64: Platform HTTP Hardening - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-03-25
**Phase:** 64-platform-http-hardening
**Areas discussed:** Trusted proxy config, Retry-After value

---

## Trusted Proxy Configuration

| Option | Description | Selected |
|--------|-------------|----------|
| TRUSTED_PROXIES env var (Recommended) | Comma-separated IPs/CIDRs. Only parse X-Forwarded-For when peer is trusted. Empty = disabled. | ✓ |
| Always trust X-Forwarded-For | Parse header unconditionally. Simple but spoofable. | |
| axum-client-ip crate | Established crate with configurable trust chain. More deps. | |

**User's choice:** TRUSTED_PROXIES env var (Recommended)

---

## Retry-After Value

| Option | Description | Selected |
|--------|-------------|----------|
| Fixed 1 second (Recommended) | Simple, predictable. Matches per-second quota granularity. | ✓ |
| Computed from governor | Use governor's until_ready() for exact wait time. More complex. | |
| Configurable via env | RATE_LIMIT_RETRY_AFTER env var. Over-engineered. | |

**User's choice:** Fixed 1 second (Recommended)

---

## Claude's Discretion

- CIDR parsing crate choice (ipnet or manual)
- Storage type for proxy list
- Config integration approach

## Deferred Ideas

None — discussion stayed within phase scope.
