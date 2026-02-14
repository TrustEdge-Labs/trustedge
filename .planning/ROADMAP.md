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
- 🚧 **v1.4 Placeholder Elimination** - Phases 19-23 (in progress)

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

### 🚧 v1.4 Placeholder Elimination (In Progress)

**Milestone Goal:** Remove all placeholder code, incomplete features, and insecure defaults — if it doesn't work, it doesn't exist in the codebase.

#### Phase 19: QUIC Security Hardening
**Goal**: Secure QUIC TLS by default
**Depends on**: Phase 18
**Requirements**: QUIC-01, QUIC-02
**Success Criteria** (what must be TRUE):
  1. QUIC client enforces proper TLS certificate verification by default
  2. Insecure TLS skip is only available when insecure-tls feature flag is enabled
  3. CI validates that default build rejects invalid certificates
  4. Developer documentation clearly warns about insecure-tls feature being development-only
**Plans**: 1 plan

Plans:
- [ ] 19-01-PLAN.md -- Add insecure-tls feature flag, refactor QUIC TLS to secure-by-default, update CI

#### Phase 20: Dead Code Removal
**Goal**: Remove legacy and unused code from core crate
**Depends on**: Phase 19
**Requirements**: DEAD-01, DEAD-02, DEAD-03, DEAD-04
**Success Criteria** (what must be TRUE):
  1. No legacy server functions remain in trustedge-server.rs
  2. No reserved/unimplemented functions in universal_keyring.rs
  3. ProcessingSession contains only active fields
  4. Every #[allow(dead_code)] attribute either has a documented justification or the code is deleted
  5. Cargo build produces no dead_code warnings
**Plans**: TBD

Plans:
- [ ] 20-01: TBD

#### Phase 21: Core Stub Elimination
**Goal**: Remove incomplete features from trustedge-core
**Depends on**: Phase 20
**Requirements**: STUB-01, STUB-02, STUB-03
**Success Criteria** (what must be TRUE):
  1. envelope_v2_bridge.rs is deleted from codebase
  2. Software HSM advertises only implemented hash variants (no Blake2b)
  3. YubiKey generate_key returns actionable error message directing users to external tools
  4. All tests pass after stub removal
**Plans**: TBD

Plans:
- [ ] 21-01: TBD

#### Phase 22: Pubky Stub Elimination
**Goal**: Remove placeholders from experimental Pubky crates
**Depends on**: Phase 21
**Requirements**: PUBK-01, PUBK-02, PUBK-03, PUBK-04
**Success Criteria** (what must be TRUE):
  1. No unimplemented CLI commands remain in trustedge-pubky
  2. discover_identities either removed or returns proper "not implemented" error
  3. Placeholder migrate command removed from CLI
  4. batch_resolve TODO comments either resolved or documented as known limitations
  5. Pubky integration tests pass with only implemented functionality
**Plans**: TBD

Plans:
- [ ] 22-01: TBD

#### Phase 23: TODO Hygiene Sweep
**Goal**: Zero unimplemented functionality TODOs
**Depends on**: Phase 22
**Requirements**: TODO-01
**Success Criteria** (what must be TRUE):
  1. Zero TODO comments indicating unimplemented functionality remain in codebase
  2. Informational TODOs (future optimizations) only exist where current code works correctly
  3. All TODOs have clear context (what works now, what's deferred, why)
  4. CI validation confirms no new unimplemented TODOs can be added
**Plans**: TBD

Plans:
- [ ] 23-01: TBD

## Progress

**Execution Order:**
Phases execute in numeric order: 19 → 20 → 21 → 22 → 23

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
| 19. QUIC Security Hardening | v1.4 | 0/0 | Not started | - |
| 20. Dead Code Removal | v1.4 | 0/0 | Not started | - |
| 21. Core Stub Elimination | v1.4 | 0/0 | Not started | - |
| 22. Pubky Stub Elimination | v1.4 | 0/0 | Not started | - |
| 23. TODO Hygiene Sweep | v1.4 | 0/0 | Not started | - |

---
*Last updated: 2026-02-13 after v1.4 roadmap creation*
