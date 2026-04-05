# Phase 80: GitHub Action Marketplace - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-04-05
**Phase:** 80-github-action-marketplace
**Areas discussed:** None (user skipped — all gaps are mechanical)

---

## Analysis

The existing action at `actions/attest-sbom-action/` (action.yml, README.md, LICENSE) was built in Phase 78. Gaps identified:
- DIST-01: Action in monorepo, needs separate repo for marketplace
- DIST-02: No SHA256 verification on binary download
- DIST-03: Input naming already exists (binary, sbom, key, trst-version)
- DIST-04: README needs persistent vs ephemeral key examples side-by-side

User chose "All clear, skip discussion" — the fixes are straightforward file copy + enhancement tasks.

## Claude's Discretion

- SHA256 verification script implementation details
- Marketplace listing categories
- Whether to add CONTRIBUTING.md

## Deferred Ideas

None.
