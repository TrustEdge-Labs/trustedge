<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


# Requirements: TrustEdge

**Defined:** 2026-02-11
**Core Value:** A single, reliable trustedge-core library that owns all cryptographic operations — maintainable by a solo developer.

## v1.2 Requirements

Requirements for scope reduction and dependency rationalization.

### Classification

- [ ] **CLSF-01**: Core crates (core, cli, trst-protocols, trst-cli, trst-wasm) marked stable in Cargo.toml metadata and README
- [ ] **CLSF-02**: Experimental crates (wasm, pubky, pubky-advanced, receipts, attestation) marked as experimental/beta in Cargo.toml metadata and README
- [ ] **CLSF-03**: Workspace Cargo.toml documents crate tiers (stable vs experimental)
- [ ] **CLSF-04**: Facade crates (receipts, attestation) reclassified from "deprecated with timeline" to "experimental, no maintenance commitment"

### Dependencies

- [ ] **DEPS-01**: Full dependency audit for 5 core crates — document each dep with justification
- [ ] **DEPS-02**: Remove unused or redundant dependencies from core crates
- [ ] **DEPS-03**: Consolidate duplicate crypto dependencies where crates pull the same libs directly instead of through core
- [ ] **DEPS-04**: Trim tokio feature flags from "full" to only what's actually used
- [ ] **DEPS-05**: Review and potentially remove reqwest from trst-cli (archive tool shouldn't need HTTP client)

### CI

- [x] **CI-01**: CI pipeline prioritizes core crates — experimental crates build but don't block merge
- [x] **CI-02**: Dependency tree size tracked (baseline established, regressions caught)

### Documentation

- [x] **DOCS-01**: Root README reflects stable/experimental split
- [x] **DOCS-02**: Each experimental crate README has clear "experimental/beta" banner

## Future Requirements

Deferred to subsequent milestones.

### Deferred

- Pubky adapter merged into core protocols/pubky/ (feature-gated)
- Pubky-advanced hybrid encryption merged into core
- Prelude module for common imports
- Updated documentation with module-level security considerations
- Key generation and attestation (yubikey crate API limitations)
- RSA certificate generation

## Out of Scope

Explicitly excluded. Documented to prevent scope creep.

| Feature | Reason |
|---------|--------|
| Deleting experimental crates | Goal is to mark, not destroy — rebuild later would be wasted effort |
| Rewriting code to drop dependencies | Moderate audit — don't rewrite working code for marginal dep savings |
| New cryptographic capabilities | This is a reduction milestone, not a feature milestone |
| TPM support | Premature, no hardware to test against |
| Post-quantum cryptography | Research phase only |
| no_std support | Requires separate milestone |

## Traceability

Which phases cover which requirements. Updated during roadmap creation.

| Requirement | Phase | Status |
|-------------|-------|--------|
| CLSF-01 | Phase 13 | Complete |
| CLSF-02 | Phase 13 | Complete |
| CLSF-03 | Phase 13 | Complete |
| CLSF-04 | Phase 13 | Complete |
| DEPS-01 | Phase 13 | Complete |
| DEPS-02 | Phase 13 | Complete |
| DEPS-03 | Phase 13 | Complete |
| DEPS-04 | Phase 13 | Complete |
| DEPS-05 | Phase 13 | Complete |
| CI-01 | Phase 14 | Complete |
| CI-02 | Phase 14 | Complete |
| DOCS-01 | Phase 14 | Complete |
| DOCS-02 | Phase 14 | Complete |

**Coverage:**
- v1.2 requirements: 13 total
- Mapped to phases: 13/13 ✓
- Unmapped: 0

**Phase mapping:**
- Phase 13 (Crate Classification & Dependency Audit): 9 requirements
- Phase 14 (CI & Documentation): 4 requirements

---
*Requirements defined: 2026-02-11*
*Last updated: 2026-02-12 after Phase 14 complete — all v1.2 requirements satisfied*
