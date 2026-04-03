<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


# Phase 55: Platform HTTP Hardening - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-03-23
**Phase:** 55-platform-http-hardening
**Areas discussed:** Body size limit, Rate limiting approach, Signing key storage

---

## Body Size Limit

| Option | Description | Selected |
|--------|-------------|----------|
| 2 MB global (Recommended) | Apply RequestBodyLimitLayer to all routes via build_base_router(). 2 MB covers any realistic verification payload. Simplest — one layer, consistent behavior. | ✓ |
| 5 MB on /v1/verify only | Larger limit, scoped to the verify endpoint only. More permissive but requires per-route layer. | |
| 10 MB global | Very generous limit. Covers edge cases but higher OOM risk under load. | |

**User's choice:** 2 MB global
**Notes:** None.

---

## Rate Limiting Approach

| Option | Description | Selected |
|--------|-------------|----------|
| tower-governor in-process (Recommended) | Add governor crate + tower-governor middleware. Per-IP rate limiting on /v1/verify. Returns 429 Too Many Requests. Works without external infra. Can be configured via env var. | ✓ |
| Nginx rate limiting only | Configure rate limiting in the nginx reverse proxy. No Rust code changes. But only protects Docker deployments, not standalone binary. | |
| Both layers | In-process governor + nginx. Defense in depth but more complexity. | |

**User's choice:** tower-governor in-process
**Notes:** None.

---

## Signing Key Storage

| Option | Description | Selected |
|--------|-------------|----------|
| Env var path + ephemeral default (Recommended) | Read key path from JWKS_KEY_PATH env var. Default to a runtime temp directory (not target/dev/). Key generated fresh each startup if file doesn't exist. For production: mount a persistent path. | ✓ |
| Env var path + encrypt at rest | Same as above but also encrypt the key file with a passphrase from JWKS_KEY_PASSPHRASE env var. More secure but adds complexity. | |
| In-memory only | Never persist the signing key to disk. Generate fresh on each startup. Simplest but receipts unverifiable after restart. | |

**User's choice:** Env var path + ephemeral default
**Notes:** None.

---

## Claude's Discretion

- Exact governor configuration (burst size, quota algorithm)
- Rate limit response headers
- Temp directory strategy
- Error response body format for 413 and 429

## Deferred Ideas

None — discussion stayed within phase scope.
