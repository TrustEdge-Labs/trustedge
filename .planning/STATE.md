---
gsd_state_version: 1.0
milestone: v4.0
milestone_name: SBOM Attestation Wedge
status: defining requirements
stopped_at: null
last_updated: "2026-04-01T22:30:00.000Z"
last_activity: 2026-04-01
progress:
  total_phases: 0
  completed_phases: 0
  total_plans: 0
  completed_plans: 0
---

<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->

# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-04-01)

**Core value:** Prove that data from an edge device has not been tampered with — from capture to verification — using cryptographic signatures, continuity chains, and verifiable receipts.
**Current focus:** v4.0 SBOM Attestation Wedge — defining requirements

## Current Position

Phase: Not started (defining requirements)
Plan: —
Status: Defining requirements
Last activity: 2026-04-01 — Milestone v4.0 started

## Performance Metrics

**Cumulative (all milestones):**

- v1.0: 8 phases, 17 plans
- v1.1: 4 phases, 6 plans
- v1.2: 2 phases, 4 plans
- v1.3: 4 phases, 5 plans
- v1.4: 5 phases, 5 plans
- v1.5: 4 phases, 8 plans
- v1.6: 3 phases, 6 plans
- v1.7: 4 phases, 10 plans
- v1.8: 3 phases, 4 plans
- v2.0: 4 phases, 8 plans
- v2.1: 3 phases, 6 plans
- v2.2: 3 phases, 5 plans
- v2.3: 4 phases, 4 plans
- v2.4: 2 phases, 3 plans
- v2.5: 3 phases, 4 plans
- v2.6: 4 phases, 5 plans
- v2.7: 3 phases, 3 plans
- v2.8: 4 phases, 5 plans
- v2.9: 3 phases, 3 plans
- v3.0: 4 phases, 5 plans
- **Total shipped: 74 phases, 108 plans**

## Accumulated Context

### Decisions

- v4.0 uses lightweight point attestation format (`.te-attestation.json`) NOT .trst archives for SBOM use case
- .trst format reserved for streaming data (video, audio, continuous sensor readings)
- Point attestation: detached Ed25519 signature over canonical JSON (BLAKE3 hashes + nonce + timestamp)
- Platform gets new `/v1/verify-attestation` endpoint (separate from `/v1/verify` for .trst archives)
- CycloneDX JSON only for Phase 1 (SPDX deferred to Phase 2)
- SBOM treated as opaque JSON payload (no schema validation in Phase 1)
- Differentiation vs GitHub Attestations: infrastructure-independent, cross-CI, hardware key support
- Public verifier: in-memory backend, ephemeral receipts, acceptable for demo stage
- Design doc: ~/.gstack/projects/TrustEdge-Labs-trustedge/john-main-design-20260401-172433.md
- CEO plan: docs/designs/sbom-attestation-wedge.md (promoted to repo)

### Pending Todos

- C2PA compatibility for media profiles (cam.video, audio) — first amendment auditor use case (P2)
- Verification badge endpoint for README embedding (P3)
- SBOM diff/drift detection between attested versions (P3)

### Blockers/Concerns

None.

## Session Continuity

Last session: 2026-04-01
Stopped at: Milestone v4.0 initialized
Resume file: None

---
*Last updated: 2026-04-01 — v4.0 milestone started*
