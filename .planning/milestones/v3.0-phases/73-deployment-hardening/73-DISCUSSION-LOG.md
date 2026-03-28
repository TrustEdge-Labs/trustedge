# Phase 73: Deployment Hardening - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-03-27
**Phase:** 73-deployment-hardening
**Areas discussed:** None (user chose "Skip — all straightforward")

---

## Gray Areas Presented

| Area | Description | Selected |
|------|-------------|----------|
| CSP connect-src strategy | How to handle VITE_API_BASE in CSP | Not selected |
| Docker secrets approach | env_file vs Docker secrets vs .env reference | Not selected |
| Skip — all straightforward | nginx headers mechanical, CSP/Docker have standard solutions | ✓ |

**User's choice:** Skip discussion — all three findings have standard, well-understood solutions.

## Claude's Discretion

- nginx header include snippet vs inline repetition
- CSP connect-src approach (build-time vs runtime vs hardcoded)
- .env.example content (dev defaults vs placeholders)

## Deferred Ideas

None
