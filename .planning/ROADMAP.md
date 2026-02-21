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
- 🚧 **v1.5 Platform Consolidation** - Phases 24-27 (in progress)

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

### 🚧 v1.5 Platform Consolidation (In Progress)

**Milestone Goal:** Consolidate external service repos into the main trustedge workspace, mandate trustedge-core for all crypto, and prune empty scaffold repos.

- [ ] **Phase 24: Type Centralization** - Migrate te_shared wire types into the main workspace as a standalone crate
- [ ] **Phase 25: Service Consolidation** - Merge trustedge-platform-api and trustedge-verify-core into a single trustedge-platform crate
- [ ] **Phase 26: Crypto Deduplication** - Replace manual crypto/chaining in the merged service with trustedge-core primitives
- [ ] **Phase 27: Ghost Repo Cleanup** - Archive 6 empty scaffold repos and document their intended scope

## Phase Details

### Phase 24: Type Centralization
**Goal**: Shared wire types live in the main trustedge workspace, consumed by all service crates
**Depends on**: Nothing (first phase of this milestone)
**Requirements**: TYPE-01, TYPE-02, TYPE-03
**Success Criteria** (what must be TRUE):
  1. A `trustedge-shared` (or equivalent) crate exists in the main workspace and builds successfully
  2. Uuid and DateTime types match platform-api's implementation — no conflicting type definitions across crates
  3. JSON schema generation for wire types works and produces output equivalent to the shared-libs version
  4. platform-api and verify-core source can reference the workspace crate without import errors
**Plans**: TBD

### Phase 25: Service Consolidation
**Goal**: platform-api and verify-core run as a single unified service in the main workspace
**Depends on**: Phase 24
**Requirements**: SVC-01, SVC-02, SVC-03, SVC-04
**Success Criteria** (what must be TRUE):
  1. A single `trustedge-platform` crate in the main workspace compiles and starts a server
  2. All device, receipt, verification, and JWKS endpoints respond correctly to requests
  3. The Certificate Authority from trustedge-ca is callable from within the consolidated crate
  4. All 11 integration tests from platform-api and all 17 tests from verify-core pass in the new crate
**Plans**: TBD

### Phase 26: Crypto Deduplication
**Goal**: The consolidated service uses only trustedge-core for cryptography — no parallel hand-rolled implementations remain
**Depends on**: Phase 25
**Requirements**: CRYPTO-01, CRYPTO-02
**Success Criteria** (what must be TRUE):
  1. The manual crypto and chaining code that existed in verify-core is deleted from the codebase
  2. Verification logic calls `trustedge_core::chain` and `trustedge_core::crypto` directly — no reimplemented equivalents
  3. All verification tests continue to pass using the core-backed implementation
**Plans**: TBD

### Phase 27: Ghost Repo Cleanup
**Goal**: The six empty scaffold repos are archived and their intended scope is recorded
**Depends on**: Nothing (independent of other phases)
**Requirements**: REPO-01, REPO-02
**Success Criteria** (what must be TRUE):
  1. All 6 repos (audit, billing-service, device-service, identity-service, infra, ingestion-service) are archived on GitHub and no longer accept pushes
  2. A document in the main workspace records what each ghost repo was intended to become, so the scope is not lost
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
| 24. Type Centralization | v1.5 | 0/TBD | Not started | - |
| 25. Service Consolidation | v1.5 | 0/TBD | Not started | - |
| 26. Crypto Deduplication | v1.5 | 0/TBD | Not started | - |
| 27. Ghost Repo Cleanup | v1.5 | 0/TBD | Not started | - |

---
*Last updated: 2026-02-21 after v1.5 roadmap creation*
