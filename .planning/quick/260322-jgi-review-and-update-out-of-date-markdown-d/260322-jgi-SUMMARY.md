<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


---
phase: quick
plan: "01"
subsystem: documentation
tags: [docs, v2.4, maintenance]
dependency_graph:
  requires: []
  provides: [accurate-docs]
  affects: [SECURITY.md, CONTRIBUTING.md, docs/]
tech_stack:
  added: []
  patterns: []
key_files:
  created: []
  modified:
    - SECURITY.md
    - CONTRIBUTING.md
    - docs/developer/development.md
    - docs/developer/testing.md
    - docs/architecture.md
    - docs/technical/format.md
    - docs/README.md
decisions:
  - "coding-standards.md required no changes — already current"
  - "threat-model.md v1.7 mitigations table left intact — legitimate historical data"
  - "enterprise.md not in scope — no changes"
metrics:
  duration_minutes: 25
  completed_date: "2026-03-22"
  tasks_completed: 2
  tasks_total: 2
  files_modified: 7
---

# Quick Task 260322-jgi: Review and Update Out-of-Date Markdown Documentation

**One-liner**: Updated 7 markdown docs from v1.x era state (109-270 tests, trustedge-receipts, yourusername URLs) to accurate v2.4 state (406 tests, 9-crate workspace, TrustEdge-Labs org, encrypted keys, HKDF-SHA256).

## Tasks Completed

| Task | Name | Commit | Files |
|------|------|--------|-------|
| 1 | Update root-level docs (SECURITY.md, CONTRIBUTING.md) | f9ca4df | SECURITY.md, CONTRIBUTING.md |
| 2 | Update docs/ directory files | 3accd52 | docs/developer/development.md, docs/developer/testing.md, docs/architecture.md, docs/technical/format.md, docs/README.md |

## Changes Made

### SECURITY.md
- Version table: v1.7.x current -> v2.4.x current; v1.0-v1.6 legacy -> v1.0-v2.3 legacy
- Security status section header: v1.7 -> v2.4
- Added to implemented features: TRUSTEDGE-KEY-V1 encrypted key files (PBKDF2-HMAC-SHA256 600k iters + AES-256-GCM), HKDF-SHA256 envelope KDF, RSA OAEP-SHA256, 45+ dedicated security tests, multi-profile archive support
- Audit table: all TBD/Pending -> March 2026 reviewed
- "Next Audit: Planned for post-v1.0" -> "Previous Audit: v2.4 remediation completed March 2026"
- Last Updated: February 2026 -> March 2026; Document Version 3.0 -> 4.0

### CONTRIBUTING.md
- Current Status: v1.0 released -> v2.4 released (March 2026)
- Removed stale "Issue #16" reference (project tracking now via GSD workflow)

### docs/developer/development.md
- 144 Total Tests -> 406 Total Tests (with per-crate breakdown matching CLAUDE.md)
- Removed trustedge-receipts crate (merged into core in v1.5)
- Clone URL: yourusername/trustedge -> TrustEdge-Labs/trustedge
- cd trustedge/trustedge-core -> cd trustedge (workspace-level)
- Backends: added Software HSM and YubiKey alongside Keyring
- Development Roadmap (Phases 1-6 checkboxes) replaced with "Development History" summary of 53 phases across 14 milestones
- Fixed ci-check.sh path (was missing ./scripts/ prefix)
- Removed Phase 3 milestone reference and Issue #16 links

### docs/developer/testing.md
- "109 total tests" -> "406 total tests" in header
- Test Statistics block: 109 Total/86 Core/23 Receipt -> 406 Total with per-crate breakdown
- Quick Test Commands: updated to use ./scripts/ci-check.sh and correct -p flags
- Section 1 header: 86 Tests -> 160+ Tests
- Section 2 (Digital Receipt System): changed from trustedge-receipts crate to trustedge-core
- Test Categories: updated test commands to -p trustedge-core instead of trustedge-receipts
- Added new categories: types, platform, trst-cli

### docs/architecture.md
- "270+ automated tests" -> "406 automated tests"
- Updated per-category breakdown to match current workspace state
- Technology Stack: added HKDF-SHA256, RSA OAEP-SHA256, TRUSTEDGE-KEY-V1 key files
- Working with Archives: added keygen, --device-key, --device-pub, unwrap examples from CLAUDE.md

### docs/technical/format.md
- Status: Draft -> Stable
- Date: September 6, 2025 -> March 2026
- Fixed duplicate Section 9 numbering: second "Section 9" -> Section 10, Version History -> Section 11, References -> Section 12
- Added v2.0 version history entry: HKDF-SHA256 KDF, versioned envelope format, encrypted key files

### docs/README.md
- Removed broken link to WASM.md (file does not exist)
- Removed broken link to developer/claude.md (file does not exist)

## Deviations from Plan

### Minor scope reduction
**docs/developer/coding-standards.md**: The plan noted this as "light touch — generally OK." After reading the file, no stale references or outdated content was found. No changes were required.

## Known Stubs

None — all documentation now reflects accurate v2.4 state.

## Self-Check: PASSED

- [x] SECURITY.md updated: version table, security status, audit table
- [x] CONTRIBUTING.md updated: version, removed Issue #16
- [x] docs/developer/development.md: test counts, clone URL, roadmap, backends
- [x] docs/developer/testing.md: test counts, crate names, commands
- [x] docs/architecture.md: test counts, tech stack, archive commands
- [x] docs/technical/format.md: Status Stable, date March 2026, fixed Section numbering, v2.0 version history
- [x] docs/README.md: removed WASM.md and claude.md broken links
- [x] Commit f9ca4df: Task 1 (SECURITY.md + CONTRIBUTING.md)
- [x] Commit 3accd52: Task 2 (5 docs/ files)
- [x] No trustedge-receipts references remain in updated files
- [x] No yourusername references remain
- [x] No 109 Total / 144 Total / 86 Core / 23 Receipt stale counts remain
- [x] format.md Status is Stable
- [x] docs/README.md contains no links to WASM.md or claude.md
