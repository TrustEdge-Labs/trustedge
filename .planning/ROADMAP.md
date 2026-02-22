<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


# Roadmap: TrustEdge

## Milestones

- ✅ **v1.0 Consolidation** - Phases 1-8 (shipped 2026-02-11)
- ✅ **v1.1 YubiKey Integration Overhaul** - Phases 9-12 (shipped 2026-02-11)
- ✅ **v1.2 Scope Reduction** - Phases 13-14 (shipped 2026-02-12)
- ✅ **v1.3 Dependency Audit** - Phases 15-18 (shipped 2026-02-13)
- ✅ **v1.4 Placeholder Elimination** - Phases 19-23 (shipped 2026-02-13)
- ✅ **v1.5 Platform Consolidation** - Phases 24-27 (shipped 2026-02-22)
- **v1.6 Final Consolidation** - Phases 28-30 (active)

## Phases

<details>
<summary>✅ v1.0 Consolidation (Phases 1-8) - SHIPPED 2026-02-11</summary>

Consolidated TrustEdge from 10 scattered crates into monolithic core with thin CLI/WASM shells. Zero API breaking changes, 98.6% test retention (343 tests), WASM compatibility preserved. Eliminated ~2,500 LOC duplication, removed 21 unused dependencies. Established 6-layer architecture, unified error types, migrated receipts and attestation into core, deprecated facade crates with 6-month migration window.

**See:** `.planning/milestones/v1.0-ROADMAP.md` for full phase details.

</details>

<details>
<summary>✅ v1.1 YubiKey Integration Overhaul (Phases 9-12) - SHIPPED 2026-02-11</summary>

Deleted broken YubiKey backend (8,117 lines) and rewrote from scratch with fail-closed design, battle-tested libraries only (yubikey crate stable API, rcgen for X.509), comprehensive test suite (18 simulation + 9 hardware), and unconditional CI validation on every PR.

**See:** `.planning/milestones/v1.1-ROADMAP.md` for full phase details.

</details>

<details>
<summary>✅ v1.2 Scope Reduction (Phases 13-14) - SHIPPED 2026-02-12</summary>

Made TrustEdge maintainable by a solo developer — 2-tier crate classification (stable/experimental), full dependency audit with documentation, trimmed tokio features, tiered CI pipeline (core blocking, experimental non-blocking), dependency tree size tracking, and updated README with crate classification.

**See:** `.planning/milestones/v1.2-ROADMAP.md` for full phase details.

</details>

<details>
<summary>✅ v1.3 Dependency Audit & Rationalization (Phases 15-18) - SHIPPED 2026-02-13</summary>

Hardened the dependency tree across all 10 crates — feature-gated heavy optional deps (git2, keyring), removed unused deps via cargo-machete, integrated cargo-audit into CI, and documented every remaining dependency with justification in DEPENDENCIES.md.

**See:** `.planning/milestones/v1.3-ROADMAP.md` for full phase details.

</details>

<details>
<summary>✅ v1.4 Placeholder Elimination (Phases 19-23) - SHIPPED 2026-02-13</summary>

Removed all placeholder code, incomplete features, and insecure defaults. Secured QUIC TLS by default, removed dead code and stubs from core and Pubky crates, enforced zero-TODO hygiene with CI enforcement on every push/PR.

**See:** `.planning/milestones/v1.4-ROADMAP.md` for full phase details.

</details>

<details>
<summary>✅ v1.5 Platform Consolidation (Phases 24-27) - SHIPPED 2026-02-22</summary>

Consolidated external service repos (platform-api, verify-core, shared-libs) into the main trustedge workspace. Created trustedge-types crate for shared wire types, merged platform-api and verify-core into unified trustedge-platform crate, replaced all manual crypto with trustedge-core primitives, and archived 5 scaffold repos with scope documentation.

**See:** `.planning/milestones/v1.5-ROADMAP.md` for full phase details.

</details>

### v1.6 Final Consolidation

- [x] **Phase 28: Platform Server Binary** - Create `crates/platform-server` binary crate that boots trustedge-platform via Axum (completed 2026-02-22)
- [x] **Phase 29: Dashboard Consolidation** - Move trustedge-dashboard into `web/dashboard/` and replace hardcoded types with generated schemas (completed 2026-02-22)
- [ ] **Phase 30: Repo Cleanup** - Delete 12 orphaned GitHub repos and update all documentation to reflect 3-repo org structure

## Phase Details

### Phase 28: Platform Server Binary
**Goal**: The platform service runs as a deployable standalone binary
**Depends on**: Nothing (trustedge-platform crate exists from v1.5)
**Requirements**: PLAT-01, PLAT-02, PLAT-03, PLAT-04
**Success Criteria** (what must be TRUE):
  1. Running `trustedge-platform-server` starts an Axum HTTP server on the configured port
  2. Server reads PORT, DATABASE_URL, and JWT_AUDIENCE from environment variables without code changes
  3. Server routes all requests through `trustedge_platform::create_router()` — no routing logic in main.rs
  4. Sending SIGTERM or SIGINT to the process causes graceful shutdown with no abrupt connection drops
**Plans**: 2 plans

Plans:
- [ ] 28-01-PLAN.md — Create platform-server binary crate (Cargo.toml, main.rs, workspace registration)
- [ ] 28-02-PLAN.md — Create deployment artifacts (Dockerfile, docker-compose.yml, .env.example)

### Phase 29: Dashboard Consolidation
**Goal**: The dashboard lives in the monorepo and uses types generated from Rust schemas
**Depends on**: Nothing (trustedge-types schemas available from v1.5)
**Requirements**: WEB-01, WEB-02, WEB-03
**Success Criteria** (what must be TRUE):
  1. `web/dashboard/` contains all dashboard source files and the repo contains no references to an external dashboard location
  2. `npm run dev` and `npm run build` succeed from `web/dashboard/` with no manual path adjustments
  3. The file `web/dashboard/src/lib/types.ts` is generated from `trustedge-types` JSON schemas — no hand-written TypeScript interface definitions remain for types that exist in trustedge-types
**Plans**: 2 plans

Plans:
- [ ] 29-01-PLAN.md — Move dashboard files into web/dashboard/ and update CLAUDE.md
- [ ] 29-02-PLAN.md — Create type generation script and replace hand-written types with generated schemas

### Phase 30: Repo Cleanup
**Goal**: The TrustEdge-Labs GitHub org contains only the three active repos
**Depends on**: Phase 29 (trustedge-dashboard repo deleted only after dashboard is moved in)
**Requirements**: REPO-01, REPO-02, REPO-03
**Success Criteria** (what must be TRUE):
  1. The TrustEdge-Labs GitHub org lists exactly 3 repos: trustedge, trustedgelabs-website, shipsecure
  2. CLAUDE.md contains no references to the 12 deleted repos (no stale repo links, no archived-repo tables for deleted repos)
  3. Documentation accurately states the org has 3 repos and describes the scope of each
**Plans**: TBD

## Progress

| Phase | Milestone | Plans Complete | Status | Completed |
|-------|-----------|----------------|--------|-----------|
| 1-8. [v1.0 phases] | v1.0 | 17/17 | Complete | 2026-02-11 |
| 9. Cleanup | v1.1 | 1/1 | Complete | 2026-02-11 |
| 10. Backend Rewrite | v1.1 | 2/2 | Complete | 2026-02-11 |
| 11. Test Infrastructure | v1.1 | 2/2 | Complete | 2026-02-11 |
| 12. CI Integration | v1.1 | 1/1 | Complete | 2026-02-11 |
| 13. Crate Classification | v1.2 | 2/2 | Complete | 2026-02-12 |
| 14. CI & Documentation | v1.2 | 2/2 | Complete | 2026-02-12 |
| 15. Feature Gating | v1.3 | 2/2 | Complete | 2026-02-12 |
| 16. Dependency Audit | v1.3 | 1/1 | Complete | 2026-02-13 |
| 17. Security Hardening | v1.3 | 1/1 | Complete | 2026-02-13 |
| 18. Documentation | v1.3 | 1/1 | Complete | 2026-02-13 |
| 19. QUIC Security Hardening | v1.4 | 1/1 | Complete | 2026-02-13 |
| 20. Dead Code Removal | v1.4 | 1/1 | Complete | 2026-02-13 |
| 21. Core Stub Elimination | v1.4 | 1/1 | Complete | 2026-02-13 |
| 22. Pubky Stub Elimination | v1.4 | 1/1 | Complete | 2026-02-13 |
| 23. TODO Hygiene Sweep | v1.4 | 1/1 | Complete | 2026-02-13 |
| 24. Type Centralization | v1.5 | 2/2 | Complete | 2026-02-21 |
| 25. Service Consolidation | v1.5 | 3/3 | Complete | 2026-02-22 |
| 26. Crypto Deduplication | v1.5 | 2/2 | Complete | 2026-02-22 |
| 27. Ghost Repo Cleanup | v1.5 | 1/1 | Complete | 2026-02-22 |
| 28. Platform Server Binary | v1.6 | 2/2 | Complete | 2026-02-22 |
| 29. Dashboard Consolidation | 2/2 | Complete    | 2026-02-22 | - |
| 30. Repo Cleanup | v1.6 | 0/? | Not started | - |

---
*Last updated: 2026-02-22 after phase 29 planning complete*
