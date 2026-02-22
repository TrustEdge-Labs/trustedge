<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-02-21)

**Core value:** A single, reliable trustedge-core library that owns all cryptographic operations — thin CLIs and WASM bindings are just frontends.
**Current focus:** Phase 25 - Service Consolidation

## Current Position

Phase: 25 of 27 in v1.5 (Service Consolidation)
Plan: 2 of 3 complete in current phase
Status: Phase 25 in progress
Last activity: 2026-02-21 — 25-02 complete (HTTP layer + database module, inline verification, no HTTP forwarding)

Progress: [████░░░░░░] ~35% (v1.5 milestone)

## Performance Metrics

**Cumulative (prior milestones):**
- v1.0: 8 phases, 17 plans, 31 tasks
- v1.1: 4 phases, 6 plans, 8 tasks
- v1.2: 2 phases, 4 plans, 9 tasks
- v1.3: 4 phases, 5 plans, 7 tasks
- v1.4: 5 phases, 5 plans, 10 tasks
- **Total: 23 phases, 37 plans, 65 tasks**

**v1.5 (in progress):**

| Phase | Plans | Tasks | Duration |
|-------|-------|-------|----------|
| 24. Type Centralization | 2 complete | 4 | 16 min |
| 25. Service Consolidation | 2 of 3 complete | 4 | 15 min |
| 26. Crypto Deduplication | TBD | - | - |
| 27. Ghost Repo Cleanup | TBD | - | - |

## Accumulated Context

### Decisions

- [v1.5 roadmap]: Type centralization precedes service consolidation — both platform-api and verify-core depend on te_shared types
- [v1.5 roadmap]: Crypto deduplication is its own phase after consolidation — merge the services first, then replace manual crypto
- [v1.5 roadmap]: Ghost repo cleanup (Phase 27) is independent of service work — can run anytime
- [v1.5 roadmap]: Dashboard (~139 LOC SvelteKit) deferred — separate technology, future milestone
- [Phase 24-type-centralization]: schemars 0.8 used (not 1.x) to preserve exact fixture match; no doc comments on structs (schemars includes them as 'description' field breaking fixture match)
- [Phase 24-type-centralization]: Keep local VerifyReport in trst-cli (out_of_order: bool vs OutOfOrder struct); migrate SegmentRef/VerifyOptions/VerifyRequest to shared types
- [Phase 25-01]: CA module is private (mod ca not pub mod ca) — Plan 02 exposes via HTTP layer
- [Phase 25-01]: Phase 26 labels replace all TODO markers in copied CA code; zero TODO markers remain
- [Phase 25-01]: BackendError (not anyhow::Error) in CAError::Backend — matches trustedge-core API
- [Phase 25-02]: sha2 and dotenvy available under both http and postgres features — both need them independently
- [Phase 25-02]: verify_core_url removed from AppState and Config — verification is inline via verify_to_report()
- [Phase 25-02]: Dual manifest digest: blake3 for receipt JWS construction, sha2 for DB storage (platform-api schema compat)
- [Phase 25-02]: postgres feature does NOT depend on http feature — they remain independent

### Pending Todos

None.

### Blockers/Concerns

- Phase 25 requires access to external repos (trustedge-platform-api, trustedge-verify-core)
- Phase 27 requires GitHub access to archive 6 repos
- Hardware tests require physical YubiKey 5 series (carried from prior milestones)
- RSA Marvin Attack advisory (RUSTSEC-2023-0071) risk-accepted (carried from v1.3)

## Session Continuity

Last session: 2026-02-21
Stopped at: Completed 25-02-PLAN.md (HTTP layer + database module)
Resume file: .planning/phases/25-service-consolidation/25-03-PLAN.md

---
*Last updated: 2026-02-21 after 25-02 completion*
