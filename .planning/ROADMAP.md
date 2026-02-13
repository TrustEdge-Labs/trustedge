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
- 🚧 **v1.3 Dependency Audit & Rationalization** - Phases 15-18 (in progress)

## Overview

v1.3 hardens the dependency tree across all 10 crates by making heavy optional dependencies opt-in (git2, keyring), removing unused dependencies, running security audits, and documenting every remaining dependency with justification.

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

### 🚧 v1.3 Dependency Audit & Rationalization (In Progress)

**Milestone Goal:** Minimize and harden the dependency tree — feature-gate heavy optional deps, remove unused deps, run security audit, document every remaining dependency.

- [ ] **Phase 15: Feature Gating** - Move git2 and keyring behind opt-in feature flags
- [ ] **Phase 16: Dependency Audit** - Remove genuinely unused dependencies from workspace
- [ ] **Phase 17: Security Hardening** - Ensure dependency tree has no known vulnerabilities
- [ ] **Phase 18: Documentation** - Document every dependency across all 10 crates

## Phase Details

### Phase 15: Feature Gating
**Goal**: Heavy optional dependencies (git2, keyring) compile only when explicitly requested
**Depends on**: Phase 14 (v1.2 complete)
**Requirements**: GATE-01, GATE-02, GATE-03, GATE-04, GATE-05
**Success Criteria** (what must be TRUE):
  1. Running `cargo build --workspace` does not compile git2 or keyring
  2. Running `cargo build --workspace --features git-attestation` compiles git2 and attestation code
  3. Running `cargo build --workspace --features keyring` compiles keyring and backend code
  4. CI pipeline tests both default build (no features) and feature-enabled builds
  5. All tests pass with and without optional features enabled
**Plans**: TBD

Plans:
- TBD (created during plan-phase)

### Phase 16: Dependency Audit
**Goal**: Remove genuinely unused dependencies from workspace
**Depends on**: Phase 15 (feature gating changes dep tree)
**Requirements**: REM-01, REM-02, REM-03
**Success Criteria** (what must be TRUE):
  1. cargo-machete runs against all 10 crates with results documented
  2. All genuinely unused dependencies are removed from crate Cargo.toml files
  3. All workspace-level dependencies not referenced by any crate are removed
  4. cargo build --workspace and cargo test --workspace still pass
**Plans**: TBD

Plans:
- TBD (created during plan-phase)

### Phase 17: Security Hardening
**Goal**: Dependency tree has no known vulnerabilities
**Depends on**: Phase 16 (audit on final dep tree)
**Requirements**: SEC-01, SEC-02, SEC-03
**Success Criteria** (what must be TRUE):
  1. Running `cargo audit` reports no known vulnerabilities in dependency tree
  2. Any security advisories are either fixed via version bumps or documented with risk acceptance
  3. CI pipeline runs `cargo audit` as a blocking check on every PR
**Plans**: TBD

Plans:
- TBD (created during plan-phase)

### Phase 18: Documentation
**Goal**: Every dependency across all 10 crates is documented with justification
**Depends on**: Phase 17 (document final state)
**Requirements**: DOC-01, DOC-02, DOC-03
**Success Criteria** (what must be TRUE):
  1. DEPENDENCIES.md covers all 10 crates (not just 5 stable crates from v1.2)
  2. Every dependency has a one-line justification in DEPENDENCIES.md
  3. Security-critical dependencies (crypto, TLS, key storage) have detailed rationale beyond one-line
**Plans**: TBD

Plans:
- TBD (created during plan-phase)

## Progress

**Execution Order:**
Phases execute in numeric order: 15 → 16 → 17 → 18

| Phase | Milestone | Plans Complete | Status | Completed |
|-------|-----------|----------------|--------|-----------|
| 1-8. [v1.0 phases] | v1.0 | 17/17 | Complete | 2026-02-11 |
| 9. Cleanup | v1.1 | 1/1 | Complete | 2026-02-11 |
| 10. Backend Rewrite | v1.1 | 2/2 | Complete | 2026-02-11 |
| 11. Test Infrastructure | v1.1 | 2/2 | Complete | 2026-02-11 |
| 12. CI Integration | v1.1 | 1/1 | Complete | 2026-02-11 |
| 13. Crate Classification & Dependency Audit | v1.2 | 2/2 | Complete | 2026-02-12 |
| 14. CI & Documentation | v1.2 | 2/2 | Complete | 2026-02-12 |
| 15. Feature Gating | v1.3 | 0/? | Not started | - |
| 16. Dependency Audit | v1.3 | 0/? | Not started | - |
| 17. Security Hardening | v1.3 | 0/? | Not started | - |
| 18. Documentation | v1.3 | 0/? | Not started | - |

---
*Last updated: 2026-02-12 after v1.3 roadmap creation*
