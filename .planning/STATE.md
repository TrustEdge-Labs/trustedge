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
**Current focus:** Phase 27 - Ghost Repo Cleanup — COMPLETE (v1.5 milestone complete)

## Current Position

Phase: 27 of 27 in v1.5 (Ghost Repo Cleanup) — COMPLETE
Plan: 1 of 1 complete in current phase
Status: Phase 27 complete — 5 scaffold repos archived on GitHub; CLAUDE.md documents archived service repo intent
Last activity: 2026-02-22 — 27-01 complete (5 repos archived, CLAUDE.md updated)

Progress: [██████████] 100% (v1.5 milestone COMPLETE)

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
| 25. Service Consolidation | 3 of 3 complete | 6 | 24 min |
| 26. Crypto Deduplication | 2 of 2 complete | 4 | 21 min |
| 27. Ghost Repo Cleanup | 1 of 1 complete | 2 | 2 min |

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
- [Phase 25-03]: axum-test 14.x used (not 18.x) — must match axum 0.7 workspace version
- [Phase 25-03]: Ordered validation: empty segments → device_pub → manifest → hash format (ensures correct error codes per test expectations)
- [Phase 25-03]: #[serde(deny_unknown_fields)] on VerifyRequest — strict API hygiene, rejects unknown fields with 400
- [Phase 25-03]: Dep tree baseline raised to 70 — platform crate adds transitive workspace deps
- [Phase 26-01]: trustedge-core moved to always-on dependency (was optional/ca-gated) — all platform features need crypto primitives
- [Phase 26-01]: format_b3() helper uses BASE64 crate's STANDARD encoder (not core's internal base64_encode) for consistent wire format
- [Phase 26-01]: Ed25519 signature bridge — manifest JSON stores raw base64; prepend 'ed25519:' before calling trustedge_core::crypto::verify_manifest()
- [Phase 26-02]: trustedge-core re-exports ed25519_dalek::{SigningKey, VerifyingKey} for JWKS key management; ed25519-dalek retained in dev-dependencies for test fixtures
- [Phase 27-01]: Plan listed 6 repos by short names but actual repos use trustedge- prefix; no trustedge-audit exists — 5 repos archived instead of 6
- [Phase 27-01]: trustedge-dashboard has meaningful SvelteKit code (29 files) — not archived; trustedge-platform-api/shared-libs/verify-core excluded (consolidated codebase from v1.5)

### Pending Todos

None.

### Blockers/Concerns

- Hardware tests require physical YubiKey 5 series (carried from prior milestones)
- RSA Marvin Attack advisory (RUSTSEC-2023-0071) risk-accepted (carried from v1.3)

## Session Continuity

Last session: 2026-02-22
Stopped at: Completed 27-01-PLAN.md — Phase 27 Ghost Repo Cleanup complete; v1.5 milestone complete
Resume file: .planning/phases/27-ghost-repo-cleanup/27-01-SUMMARY.md

---
*Last updated: 2026-02-22 after Phase 27 Plan 01 completion — v1.5 COMPLETE*
