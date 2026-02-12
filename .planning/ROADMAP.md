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
- 🚧 **v1.2 Scope Reduction & Dependency Rationalization** - Phases 13-14 (in progress)

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

### 🚧 v1.2 Scope Reduction & Dependency Rationalization (In Progress)

**Milestone Goal:** Make TrustEdge maintainable by a solo developer — clear stable/experimental split, trimmed dependencies, reduced build and maintenance burden.

#### Phase 13: Crate Classification & Dependency Audit
**Goal**: Core crates clearly marked as stable, experimental crates marked as beta, and all dependencies documented with justification
**Depends on**: Phase 12 (v1.1 complete)
**Requirements**: CLSF-01, CLSF-02, CLSF-03, CLSF-04, DEPS-01, DEPS-02, DEPS-03, DEPS-04, DEPS-05
**Success Criteria** (what must be TRUE):
  1. All 5 core crates (core, cli, trst-protocols, trst-cli, trst-wasm) have stable metadata in Cargo.toml and README markers
  2. All 5 experimental crates (wasm, pubky, pubky-advanced, receipts, attestation) have experimental/beta metadata and clear warnings
  3. Workspace Cargo.toml documents the 2-tier crate classification
  4. Every dependency in core crates has documented justification
  5. Redundant and unused dependencies removed from core crates
**Plans**: 2 plans

Plans:
- [ ] 13-01-PLAN.md -- Crate classification: tier metadata in Cargo.toml + experimental README banners
- [ ] 13-02-PLAN.md -- Dependency audit: document justifications, remove unused, trim tokio, review reqwest

#### Phase 14: CI & Documentation
**Goal**: CI prioritizes core crates and documentation reflects the stable/experimental split
**Depends on**: Phase 13
**Requirements**: CI-01, CI-02, DOCS-01, DOCS-02
**Success Criteria** (what must be TRUE):
  1. CI pipeline runs comprehensive checks on core crates (experimental crates build but don't block)
  2. Dependency tree size baseline established and tracked in CI
  3. Root README clearly documents stable vs experimental crate split
  4. Each experimental crate README has prominent experimental/beta banner
**Plans**: TBD

Plans:
- [ ] 14-01: TBD (CI updates)
- [ ] 14-02: TBD (documentation)

## Progress

| Phase | Milestone | Plans Complete | Status | Completed |
|-------|-----------|----------------|--------|-----------|
| 1-8. [v1.0 phases] | v1.0 | 17/17 | Complete | 2026-02-11 |
| 9. Cleanup | v1.1 | 1/1 | Complete | 2026-02-11 |
| 10. Backend Rewrite | v1.1 | 2/2 | Complete | 2026-02-11 |
| 11. Test Infrastructure | v1.1 | 2/2 | Complete | 2026-02-11 |
| 12. CI Integration | v1.1 | 1/1 | Complete | 2026-02-11 |
| 13. Crate Classification & Dependency Audit | v1.2 | 0/TBD | Not started | - |
| 14. CI & Documentation | v1.2 | 0/TBD | Not started | - |

---
*Last updated: 2026-02-11 after v1.2 roadmap created*
