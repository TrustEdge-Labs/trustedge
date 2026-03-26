# Phase 71: Platform Code Quality - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-03-26
**Phase:** 71-platform-code-quality
**Areas discussed:** Receipt TTL config, Healthz response, PORT failure mode

---

## Receipt TTL Config

| Option | Description | Selected |
|--------|-------------|----------|
| RECEIPT_TTL_SECS env var | Add to Config struct, default 3600s. Follows existing pattern (PORT, DATABASE_URL, JWT_AUDIENCE) | ✓ |
| Config struct field only | No env var — set via builder. More explicit but less ops-friendly | |
| You decide | Claude picks best-fit approach | |

**User's choice:** RECEIPT_TTL_SECS env var (Recommended)
**Notes:** Follows existing Config::from_env() pattern

---

## Healthz Response

| Option | Description | Selected |
|--------|-------------|----------|
| Remove version field entirely | Return only status + timestamp. Eliminates fingerprinting. | ✓ |
| Replace with generic string | e.g. version: "ok" or "3.x" — field stays but imprecise | |
| Keep version, gate behind auth | Authenticated callers see version, unauthenticated get status+timestamp | |

**User's choice:** Remove version field entirely (Recommended)
**Notes:** Simplest fix, version still available via binary --version

---

## PORT Failure Mode

| Option | Description | Selected |
|--------|-------------|----------|
| Hard fail with error | If PORT is set but unparseable, exit with clear error. Missing PORT defaults to 3001. | ✓ |
| Warn and use default | Log warning, fall back to 3001. More forgiving but masks misconfiguration. | |

**User's choice:** Hard fail with error (Recommended)
**Notes:** "If you set it, you meant something."

---

## Claude's Discretion

- Threading receipt_ttl from Config to sign_receipt_jws
- Test coverage for RECEIPT_TTL_SECS env var parsing

## Deferred Ideas

None
