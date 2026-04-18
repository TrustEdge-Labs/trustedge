---
gsd_state_version: 1.0
milestone: v6.0
milestone_name: Sealedge Rebrand
status: defining_requirements
stopped_at: v6.0 milestone initialized — defining requirements
last_updated: "2026-04-18T00:00:00.000Z"
last_activity: 2026-04-18 -- Milestone v6.0 Sealedge Rebrand started
progress:
  total_phases: 0
  completed_phases: 0
  total_plans: 0
  completed_plans: 0
  percent: 0
---

<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->

# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-04-18)

**Core value:** Prove that data from an edge device has not been tampered with — from capture to verification — using cryptographic signatures, continuity chains, and verifiable receipts.
**Current focus:** Defining v6.0 Sealedge Rebrand requirements

## Current Position

Phase: Not started (defining requirements)
Plan: —
Status: Defining requirements
Last activity: 2026-04-18 — Milestone v6.0 Sealedge Rebrand started

## Performance Metrics

**Cumulative (all milestones):**

- v1.0–v4.0: 78 phases, 116 plans shipped
- v5.0: 2 phases shipped (79-80), 2 punted (81-82 → post-rename roadmap)

**v6.0 (current):** 0/? phases — roadmap pending

## Accumulated Context

### Decisions

- v6.0 is a trademark-driven rename from "trustedge" to "sealedge" — clean break, no legacy compat
- TrustEdge-Labs org/brand stays; only the product is renamed
- `trst` CLI also gets renamed (pick new short name during planning)
- Crypto constants (TRUSTEDGE-KEY-V1, TRUSTEDGE_ENVELOPE_V1) get replaced with sealedge equivalents — no backward-compat decrypt path (solo dev, no production users)
- v5.0 Phase 81 (demo GIF) and Phase 82 (landing page) punted to post-rename roadmap — makes more sense to produce after the rebrand lands
- v5.0 decisions carried forward:
  - Self-attestation uses ephemeral Ed25519 key per build, OIDC/Sigstore deferred
  - GitHub Action required separate repo for marketplace listing (platform constraint)
  - Design doc: ~/.gstack/projects/TrustEdge-Labs-trustedge/john-main-design-20260405-085506.md

### Pending Todos

- C2PA compatibility for media profiles (cam.video, audio) — first amendment auditor use case (P2)
- Verification badge endpoint for README embedding (P3)
- SBOM diff/drift detection between attested versions (P3)
- te-prove: FOSS supply chain trust policy engine (parked — see .planning/ideas/)
- Post-rename: record demo GIF and embed in README (was v5.0 Phase 81)
- Post-rename: ship product landing page on trustedgelabs.com (was v5.0 Phase 82)

### Blockers/Concerns

None.

### Quick Tasks Completed

| # | Description | Date | Commit | Directory |
|---|-------------|------|--------|-----------|
| 260414-aib | Resolve GitHub Dependabot issues | 2026-04-14 | 8d01d53 | [260414-aib-resolve-github-dependabot-issues](./quick/260414-aib-resolve-github-dependabot-issues/) |

## Session Continuity

Last session: 2026-04-18T00:00:00.000Z
Stopped at: v6.0 initialized — requirements definition in progress
Resume file: .planning/REQUIREMENTS.md

---
*Last updated: 2026-04-18 — v6.0 Sealedge Rebrand milestone started*
