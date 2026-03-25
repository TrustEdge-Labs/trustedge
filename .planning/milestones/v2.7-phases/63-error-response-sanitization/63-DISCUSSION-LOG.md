# Phase 63: Error Response Sanitization - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-03-25
**Phase:** 63-error-response-sanitization
**Areas discussed:** Error message wording, Logging level and detail

---

## Error Message Wording

| Option | Description | Selected |
|--------|-------------|----------|
| Category-only (Recommended) | "Cryptographic verification failed" / "Receipt generation failed" — no detail | ✓ |
| Generic + error code | "Verification failed (ERR_CRYPTO_001)" — reference code for support | |
| Fully generic | Just "Verification failed" for everything | |

**User's choice:** Category-only (Recommended)
**Notes:** Client knows WHAT failed (verification vs receipt), not WHY internally.

---

## Logging Level and Detail

| Option | Description | Selected |
|--------|-------------|----------|
| Keep existing warn! (Recommended) | warn! already captures full error. Just remove detail from client response. | ✓ |
| Upgrade to error! + chain | error! level with {:?} for full error chain | |
| Add tracing fields | Structured fields (device_pub, endpoint) for log correlation | |

**User's choice:** Keep existing warn! (Recommended)
**Notes:** Current logging is sufficient. Only change is to the client-facing message.

---

## Claude's Discretion

- Whether to use constants for generic messages or inline strings
- Test structure choices
- ValidationError field name retention

## Deferred Ideas

None — discussion stayed within phase scope.
