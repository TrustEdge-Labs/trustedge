# Phase 70: Deployment Hardening - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-03-26
**Phase:** 70-deployment-hardening
**Areas discussed:** CSP header policy, HSTS parameters, HTTP redirect behavior

---

## CSP Header Policy

| Option | Description | Selected |
|--------|-------------|----------|
| Moderate | default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; connect-src 'self'; img-src 'self' data:; font-src 'self'. Allows inline styles for SvelteKit. | ✓ |
| Strict | No unsafe-inline. May break SvelteKit scoped styles. | |
| You decide | Claude picks based on dashboard needs. | |

**User's choice:** Moderate (Recommended)
**Notes:** SvelteKit static builds use inline styles; unsafe-inline needed for style-src.

---

## HSTS Parameters

| Option | Description | Selected |
|--------|-------------|----------|
| Conservative | max-age=31536000 (1 year), no includeSubDomains, no preload. Safe for self-hosted. | ✓ |
| Aggressive | max-age=63072000 (2 years), includeSubDomains, preload. Hard to undo. | |
| You decide | Claude picks settings. | |

**User's choice:** Conservative (Recommended)
**Notes:** Self-hosted deployments — operators control DNS, no need for preload list.

---

## HTTP Redirect Behavior

| Option | Description | Selected |
|--------|-------------|----------|
| 301 redirect all | Server block on port 8080 that 301s to HTTPS 8443. Healthz excluded for docker probes. | ✓ |
| 308 permanent redirect | Preserves HTTP method. More correct for APIs but dashboard is GET-only. | |
| You decide | Claude picks strategy. | |

**User's choice:** 301 redirect all (Recommended)
**Notes:** Standard approach. /healthz excluded so docker-compose healthchecks aren't broken.

## Claude's Discretion

- Exact placement of add_header directives (server vs location level)

## Deferred Ideas

None.
