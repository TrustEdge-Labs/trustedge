# Requirements: TrustEdge

**Defined:** 2026-02-12
**Core Value:** A single, reliable trustedge-core library that owns all cryptographic operations — maintainable by a solo developer.

## v1.3 Requirements

Requirements for dependency audit and rationalization. Each maps to roadmap phases.

### Feature Gating

- [x] **GATE-01**: git2 dependency is behind an opt-in `git-attestation` feature flag (not compiled by default)
- [x] **GATE-02**: keyring dependency is behind an opt-in `keyring` feature flag (not compiled by default)
- [x] **GATE-03**: All code using git2 compiles only when `git-attestation` feature is enabled
- [x] **GATE-04**: All code using keyring compiles only when `keyring` feature is enabled
- [x] **GATE-05**: CI tests both with and without git-attestation and keyring features

### Dependency Removal

- [x] **REM-01**: cargo-machete run across all 10 crates with results documented
- [x] **REM-02**: All genuinely unused dependencies removed from Cargo.toml files
- [x] **REM-03**: Workspace-level dependencies not referenced by any crate are removed

### Security Audit

- [x] **SEC-01**: cargo-audit runs clean (no known vulnerabilities in dependency tree)
- [x] **SEC-02**: Any advisories are either fixed (version bump) or documented with risk acceptance
- [x] **SEC-03**: cargo-audit added to CI pipeline as a blocking check

### Documentation

- [x] **DOC-01**: DEPENDENCIES.md updated to cover all 10 crates (not just 5 stable)
- [x] **DOC-02**: Every dependency has a one-line justification
- [x] **DOC-03**: Security-critical dependencies (crypto, TLS, key storage) have detailed rationale

## Future Requirements

### Dependency Upgrades

- **UPG-01**: Upgrade outdated dependencies to latest compatible versions
- **UPG-02**: Pin dependency versions for reproducible builds

## Out of Scope

| Feature | Reason |
|---------|--------|
| Removing experimental crates | Scope reduction already done in v1.2; experimental crates preserved for future |
| Upgrading major dependency versions | Would require API changes; separate milestone |
| Adding new dependencies | This milestone is about reducing, not adding |
| no_std support | Requires separate milestone with broader architectural changes |

## Traceability

Which phases cover which requirements. Updated during roadmap creation.

| Requirement | Phase | Status |
|-------------|-------|--------|
| GATE-01 | Phase 15 | Complete |
| GATE-02 | Phase 15 | Complete |
| GATE-03 | Phase 15 | Complete |
| GATE-04 | Phase 15 | Complete |
| GATE-05 | Phase 15 | Complete |
| REM-01 | Phase 16 | Complete |
| REM-02 | Phase 16 | Complete |
| REM-03 | Phase 16 | Complete |
| SEC-01 | Phase 17 | Complete |
| SEC-02 | Phase 17 | Complete |
| SEC-03 | Phase 17 | Complete |
| DOC-01 | Phase 18 | Complete |
| DOC-02 | Phase 18 | Complete |
| DOC-03 | Phase 18 | Complete |

**Coverage:**
- v1.3 requirements: 14 total
- Mapped to phases: 14
- Unmapped: 0

**Coverage validation:** ✓ 100% (all requirements mapped to exactly one phase)

---
*Requirements defined: 2026-02-12*
*Last updated: 2026-02-13 after Phase 18 completion*
