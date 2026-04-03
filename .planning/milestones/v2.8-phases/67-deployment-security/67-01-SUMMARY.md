<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


---
phase: 67-deployment-security
plan: "01"
subsystem: deployment
tags: [docker, nginx, non-root, security, ci, credential-guard]
dependency_graph:
  requires: []
  provides: [DEPL-01, DEPL-02]
  affects: [deploy/Dockerfile.dashboard, deploy/docker-compose.yml, deploy/docker-entrypoint.sh, deploy/nginx.conf, deploy/nginx-ssl.conf.template, .github/workflows/ci.yml]
tech_stack:
  added: [nginxinc/nginx-unprivileged:alpine, actions/setup-node@v4]
  patterns: [non-root container, SHA-pinned CI actions, credential leak guard]
key_files:
  created: []
  modified:
    - deploy/Dockerfile.dashboard
    - deploy/docker-compose.yml
    - deploy/docker-entrypoint.sh
    - deploy/nginx.conf
    - deploy/nginx-ssl.conf.template
    - .github/workflows/ci.yml
decisions:
  - Use /tmp/nginx-ssl/ for runtime SSL config output (writable by non-root uid 101)
  - Store SSL template at /etc/nginx/nginx-ssl.conf.template (not conf.d to avoid nginx parsing it)
  - Static ssl-include.conf in conf.d picks up /tmp/nginx-ssl/*.conf at nginx startup
  - SHA-pin actions/setup-node at 49933ea5 (v4.4.0, verified via GitHub API)
  - Use npm ci --ignore-scripts for security in CI build step
metrics:
  duration: "85s"
  completed: "2026-03-26"
  tasks_completed: 2
  files_modified: 6
---

# Phase 67 Plan 01: Deployment Security Hardening Summary

**One-liner:** Non-root nginx via nginx-unprivileged:alpine on ports 8080/8443 plus CI credential leak guard for dashboard bundle.

## Tasks Completed

| # | Task | Commit | Files |
|---|------|--------|-------|
| 1 | Switch dashboard container to non-root nginx (DEPL-01) | 4efb634 | deploy/Dockerfile.dashboard, deploy/docker-compose.yml, deploy/docker-entrypoint.sh, deploy/nginx.conf, deploy/nginx-ssl.conf.template |
| 2 | Add CI bundle credential guard to GitHub Actions (DEPL-02) | 39dfb68 | .github/workflows/ci.yml |

## What Was Built

### Task 1: Non-root Nginx Container (DEPL-01)

Replaced `nginx:alpine` base image with `nginxinc/nginx-unprivileged:alpine` which runs as uid 101 (non-root) and listens on unprivileged ports by default.

Changes:
- `deploy/Dockerfile.dashboard`: New base image, EXPOSE 8080 8443, healthcheck on port 8080, no chmod needed (entrypoint is 100755 in git)
- `deploy/nginx.conf`: `listen 8080` (was 80)
- `deploy/nginx-ssl.conf.template`: `listen 8443 ssl` (was 443)
- `deploy/docker-compose.yml`: ports `8080:8080` and `8443:8443`, healthcheck targets `localhost:8080`
- `deploy/docker-entrypoint.sh`: writes SSL config to `/tmp/nginx-ssl/ssl.conf` (non-root writable); reads template from `/etc/nginx/nginx-ssl.conf.template`

SSL config routing at runtime: the Dockerfile creates a static `/etc/nginx/conf.d/ssl-include.conf` containing `include /tmp/nginx-ssl/*.conf;`. If SSL env vars are set, the entrypoint renders the template to `/tmp/nginx-ssl/ssl.conf` which nginx picks up via the include. If not set, the directory is empty and nginx silently ignores the glob include.

### Task 2: CI Bundle Credential Guard (DEPL-02)

Added two steps to the `lint` job in `.github/workflows/ci.yml`:

1. `Setup Node.js for dashboard checks` — SHA-pinned `actions/setup-node@49933ea5288caeca8642d1e84afbd3f7d6820020 # v4` (v4.4.0, verified via `gh api repos/actions/setup-node/git/ref/tags/v4`)
2. `Dashboard bundle credential guard` — runs `npm ci --ignore-scripts && npm run build`, then greps the build output for `VITE_API_KEY`. Fails with `::error::` annotation on detection.

Both steps are conditional on `hashFiles('web/dashboard/package.json') != ''` (per D-08 requirement). The grep pattern mirrors `scripts/ci-check.sh` Step 12 exactly.

## Decisions Made

- **Non-root SSL write path:** Used `/tmp/nginx-ssl/` rather than `/etc/nginx/conf.d/` for SSL config at runtime, even though nginx-unprivileged makes conf.d writable. Explicit `/tmp/` path is more predictable and avoids any image version variability.
- **SSL template location:** Moved template to `/etc/nginx/nginx-ssl.conf.template` (outside conf.d) to prevent nginx from attempting to parse the template file as a config on startup.
- **actions/setup-node SHA:** Verified `49933ea5288caeca8642d1e84afbd3f7d6820020` via `gh api repos/actions/setup-node/git/ref/tags/v4` — this is v4.4.0, the latest stable v4 release.

## Deviations from Plan

### Auto-fixed Issues

None — plan executed exactly as written, with the following note:

The plan's `<interfaces>` section contained an incorrect SHA for `actions/setup-node` (`cdca7365b2dadb8aad0a33bc7601856ffabcc48e` = v4.3.0, and `49933ea5288caeca8642195f572a2b2b9f8a9ba7` which is malformed). Per the `<important_note>` in the prompt, the SHA was verified via `gh api repos/actions/setup-node/git/ref/tags/v4 --jq .object.sha` returning `49933ea5288caeca8642d1e84afbd3f7d6820020` (v4.4.0). This is the SHA used in the commit.

## Known Stubs

None.

## Self-Check: PASSED

Files created/modified:
- FOUND: /home/john/vault/projects/github.com/trustedge/deploy/Dockerfile.dashboard
- FOUND: /home/john/vault/projects/github.com/trustedge/deploy/docker-compose.yml
- FOUND: /home/john/vault/projects/github.com/trustedge/deploy/docker-entrypoint.sh
- FOUND: /home/john/vault/projects/github.com/trustedge/deploy/nginx.conf
- FOUND: /home/john/vault/projects/github.com/trustedge/deploy/nginx-ssl.conf.template
- FOUND: /home/john/vault/projects/github.com/trustedge/.github/workflows/ci.yml

Commits:
- 4efb634: feat(67-01): switch dashboard container to non-root nginx (DEPL-01)
- 39dfb68: feat(67-01): add CI bundle credential guard to lint job (DEPL-02)
