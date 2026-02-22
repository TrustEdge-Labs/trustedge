<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-02-22)

**Core value:** A single, reliable trustedge-core library that owns all cryptographic operations — thin CLIs and WASM bindings are just frontends.
**Current focus:** v1.6 Final Consolidation

## Current Position

Phase: 29 — Dashboard Consolidation
Plan: 02 (complete)
Status: Phase 29 complete (2 of 2 plans done)
Last activity: 2026-02-22 — 29-02 TypeScript types generated from trustedge-types JSON schemas

```
v1.6 Progress: [■■■       ] 2/3 phases (phases 28-29 complete, phase 30 pending)
```

## Performance Metrics

**Cumulative (all milestones):**
- v1.0: 8 phases, 17 plans, 31 tasks
- v1.1: 4 phases, 6 plans, 8 tasks
- v1.2: 2 phases, 4 plans, 9 tasks
- v1.3: 4 phases, 5 plans, 7 tasks
- v1.4: 5 phases, 5 plans, 10 tasks
- v1.5: 4 phases, 8 plans, 16 tasks
- **Total (through v1.5): 27 phases, 45 plans, 81 tasks**

**v1.6 (in progress):**
- 3 phases planned
- Plans: 4 complete (28-01, 28-02, 29-01, 29-02)
- Tasks: 7 complete

| Phase | Plan | Duration | Tasks | Files |
|-------|------|----------|-------|-------|
| 28    | 01   | 4 min    | 2     | 5     |
| 28    | 02   | 2 min    | 2     | 3     |
| 29    | 01   | 5 min    | 2     | 22    |
| 29    | 02   | 3 min    | 1     | 8     |

## Accumulated Context

### Decisions

- Phase 28 before 29: Server binary and dashboard move are independent; numbered for natural delivery order (infra before web)
- Phase 30 after 29: trustedge-dashboard repo deletion must follow successful dashboard move into `web/dashboard/`
- Platform server binary goes in `crates/platform-server/` — thin main.rs, all routing in trustedge-platform
- Dashboard types generated from trustedge-types schemars 0.8 schemas (no new tooling, one-time generation)
- [Phase 28]: postgres is always compiled into platform-server binary (default feature); verify-only mode is not a compile-time decision
- [Phase 28]: platform-server test guards in verify_integration and platform_integration needed fixing when postgres feature became active workspace-wide
- [Phase 28-02]: debian:bookworm-slim chosen over alpine for Dockerfile runtime stage — glibc compatibility with sqlx native-tls; postgres:16-alpine for compose service
- [Phase 29]: vitePreprocess() required in svelte.config.js for TypeScript in Svelte 4 script blocks
- [Phase 29]: import.meta.env.VITE_* used for env vars in SvelteKit dashboard — $env/dynamic/public only allows PUBLIC_-prefixed vars
- [Phase 29]: Separate import type statements required in Svelte 4 — inline type modifier not supported by acorn parser
- [Phase 29]: json-schema-to-typescript as devDependency in web/dashboard; DashboardReceipt naming to avoid collision with generated Receipt; Node.js dedup embedded in generate-types.sh for cross-schema interface deduplication

### Pending Todos

None.

### Blockers/Concerns

- Hardware tests require physical YubiKey 5 series (carried from prior milestones)
- RSA Marvin Attack advisory (RUSTSEC-2023-0071) risk-accepted (carried from v1.3)
- WEB-03 type generation: RESOLVED in 29-02 — types.ts now generated from crates/types/tests/fixtures/*.json

## Session Continuity

Last session: 2026-02-22
Stopped at: Completed 29-02-PLAN.md
Resume at: Phase 30 (trustedge-dashboard repo deletion)

---
*Last updated: 2026-02-22 after 29-02 type generation complete*
