# Phase 67: Deployment Security - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.

**Date:** 2026-03-25
**Phase:** 67-deployment-security
**Areas discussed:** nginx non-root approach

---

## nginx Non-Root Approach

| Option | Description | Selected |
|--------|-------------|----------|
| nginxinc/nginx-unprivileged (Recommended) | Official non-root image, listens 8080, runs as uid 101 | ✓ |
| Add USER directive to nginx:alpine | Keep image, add manual user config | |
| Custom distroless | Minimal attack surface but loses shell for entrypoint | |

**User's choice:** nginxinc/nginx-unprivileged (Recommended)

---

## Claude's Discretion

- Exact nginx-unprivileged tag variant
- Entrypoint SSL config adaptation for non-root
- npm ci vs npm install in CI step

## Deferred Ideas

None.
