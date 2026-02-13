# Requirements: TrustEdge

**Defined:** 2026-02-12
**Core Value:** A single, reliable trustedge-core library that owns all cryptographic operations — maintainable by a solo developer.

## v1.3 Requirements

Requirements for dependency audit and rationalization. Each maps to roadmap phases.

### Feature Gating

- [ ] **GATE-01**: git2 dependency is behind an opt-in `git-attestation` feature flag (not compiled by default)
- [ ] **GATE-02**: keyring dependency is behind an opt-in `keyring` feature flag (not compiled by default)
- [ ] **GATE-03**: All code using git2 compiles only when `git-attestation` feature is enabled
- [ ] **GATE-04**: All code using keyring compiles only when `keyring` feature is enabled
- [ ] **GATE-05**: CI tests both with and without git-attestation and keyring features

### Dependency Removal

- [ ] **REM-01**: cargo-machete run across all 10 crates with results documented
- [ ] **REM-02**: All genuinely unused dependencies removed from Cargo.toml files
- [ ] **REM-03**: Workspace-level dependencies not referenced by any crate are removed

### Security Audit

- [ ] **SEC-01**: cargo-audit runs clean (no known vulnerabilities in dependency tree)
- [ ] **SEC-02**: Any advisories are either fixed (version bump) or documented with risk acceptance
- [ ] **SEC-03**: cargo-audit added to CI pipeline as a blocking check

### Documentation

- [ ] **DOC-01**: DEPENDENCIES.md updated to cover all 10 crates (not just 5 stable)
- [ ] **DOC-02**: Every dependency has a one-line justification
- [ ] **DOC-03**: Security-critical dependencies (crypto, TLS, key storage) have detailed rationale

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
| GATE-01 | — | Pending |
| GATE-02 | — | Pending |
| GATE-03 | — | Pending |
| GATE-04 | — | Pending |
| GATE-05 | — | Pending |
| REM-01 | — | Pending |
| REM-02 | — | Pending |
| REM-03 | — | Pending |
| SEC-01 | — | Pending |
| SEC-02 | — | Pending |
| SEC-03 | — | Pending |
| DOC-01 | — | Pending |
| DOC-02 | — | Pending |
| DOC-03 | — | Pending |

**Coverage:**
- v1.3 requirements: 14 total
- Mapped to phases: 0
- Unmapped: 14

---
*Requirements defined: 2026-02-12*
*Last updated: 2026-02-12 after initial definition*
