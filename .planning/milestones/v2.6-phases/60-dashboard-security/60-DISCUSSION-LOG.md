# Phase 60: Dashboard Security - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.

**Date:** 2026-03-24
**Phase:** 60-dashboard-security
**Areas discussed:** Auth approach

---

## Auth Approach

| Option | Description | Selected |
|--------|-------------|----------|
| Remove auth entirely (Recommended) | Dashboard only reads public endpoints. Remove VITE_API_KEY, remove Bearer header. | ✓ |
| Server-side proxy | Node/Bun server holds API key. Complex, adds runtime. | |
| Session-based login | Full auth flow. Significant scope expansion. | |

**User's choice:** Remove auth entirely

---

## Claude's Discretion

- Receipts page handling
- CI check mechanism
- Config comments

## Deferred Ideas

None.
