---
gsd_state_version: 1.0
milestone: v4.0
milestone_name: SBOM Attestation Wedge
status: ready to plan
stopped_at: Phase 75
last_updated: "2026-04-01T22:30:00.000Z"
last_activity: 2026-04-01
progress:
  total_phases: 4
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
**Current focus:** v4.0 SBOM Attestation Wedge — Phase 75 ready to plan

## Current Position

Phase: 75 of 78 (Core Attestation Library)
Plan: —
Status: Ready to plan
Last activity: 2026-04-01 — Roadmap created for v4.0

Progress: [░░░░░░░░░░] 0%

## Performance Metrics

**Cumulative (all milestones):**

- v1.0–v3.0: 74 phases, 108 plans shipped

**v4.0 (current):** 0/4 phases complete

## Accumulated Context

### Decisions

- v4.0 uses lightweight point attestation format (`.te-attestation.json`) NOT .trst archives for SBOM use case
- .trst format reserved for streaming data (video, audio, continuous sensor readings)
- Point attestation: detached Ed25519 signature over canonical JSON (BLAKE3 hashes + nonce + timestamp)
- Platform gets new `/v1/verify-attestation` endpoint (separate from `/v1/verify` for .trst archives)
- CycloneDX JSON only for Phase 1 (SPDX deferred to future milestone)
- Public verifier: in-memory backend, ephemeral receipts, acceptable for demo stage
- Phase 78 (landing page) touches trustedgelabs-website repo — separate from main monorepo

### Pending Todos

- C2PA compatibility for media profiles (cam.video, audio) — first amendment auditor use case (P2)
- Verification badge endpoint for README embedding (P3)
- SBOM diff/drift detection between attested versions (P3)

### Blockers/Concerns

None.

## Session Continuity

Last session: 2026-04-01
Stopped at: Roadmap created — Phase 75 ready to plan
Resume file: None

---
*Last updated: 2026-04-01 — v4.0 roadmap created*
