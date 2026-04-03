<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


---
phase: 74-release-documentation
plan: "01"
subsystem: documentation
tags: [readme, docs, release-polish, v3.0]
one_liner: "README.md updated to v3.0 (badge, quick-start env step, security posture); deploy/.env.example gains RECEIPT_TTL_SECS, CORS_ORIGINS, JWKS_KEY_PATH"

dependency_graph:
  requires: []
  provides: [DOCS-01]
  affects: [README.md, deploy/.env.example]

tech_stack:
  added: []
  patterns: ["accuracy-first documentation audit", "env.example as configuration reference"]

key_files:
  created: []
  modified:
    - README.md
    - deploy/.env.example

decisions:
  - "demo.sh already had --unencrypted in keygen and wrap; no changes needed"
  - "Added CORS_ORIGINS and JWKS_KEY_PATH to .env.example since both are documented in README security posture and referenced in platform code"

metrics:
  duration_minutes: 2
  completed_date: "2026-03-27"
  tasks_completed: 2
  files_modified: 2
---

# Phase 74 Plan 01: Release Documentation — README and Env Template Summary

Documentation accuracy audit for v3.0 release. Updated README.md and deploy/.env.example to reflect the v3.0 codebase state. Demo script was already accurate and required no changes.

## Tasks Completed

| Task | Name | Commit | Files |
|------|------|--------|-------|
| 1 | Audit and update README.md for v3.0 accuracy | aa4fde0 | README.md |
| 2 | Audit and update demo script and deploy env template | 9dc4dbc | deploy/.env.example |

## Changes Made

### README.md
- Updated version badge from v2.6 to v3.0
- Added `cp deploy/.env.example deploy/.env` step to Docker Quick Start (v3.0 requirement)
- Updated Security Posture section: bumped to v3.0, corrected test count from 423 to 406, added RECEIPT_TTL_SECS and PORT notes

### deploy/.env.example
- Added `RECEIPT_TTL_SECS` with default value (3600s) and description
- Added `CORS_ORIGINS` with usage instructions (comma-separated, disabled if unset)
- Added `JWKS_KEY_PATH` with container path note
- Improved `PORT` comment to note strict validation behavior

### scripts/demo.sh
- No changes needed — already uses `--unencrypted` in keygen and wrap, correct flag names, correct compose path

## Deviations from Plan

### Auto-added Missing Critical Functionality

**1. [Rule 2 - Missing] Added CORS_ORIGINS to deploy/.env.example**
- **Found during:** Task 2
- **Issue:** CORS_ORIGINS is documented in README security posture and implemented in platform/src/http/router.rs but was absent from .env.example
- **Fix:** Added CORS_ORIGINS with description matching actual platform behavior
- **Files modified:** deploy/.env.example
- **Commit:** 9dc4dbc

**2. [Rule 2 - Missing] Added JWKS_KEY_PATH to deploy/.env.example**
- **Found during:** Task 2
- **Issue:** JWKS_KEY_PATH referenced in README security posture and implemented in platform code but absent from .env.example
- **Fix:** Added JWKS_KEY_PATH with container path note and default behavior description
- **Files modified:** deploy/.env.example
- **Commit:** 9dc4dbc

## Verification Results

1. `grep -E "env.example|unencrypted|passphrase" README.md` — all patterns found
2. `bash -n scripts/demo.sh` — SYNTAX OK
3. `grep "RECEIPT_TTL_SECS" deploy/.env.example` — found `# RECEIPT_TTL_SECS=3600`
4. No removed or renamed CLI flags referenced in modified files

## Known Stubs

None.
