# Requirements: TrustEdge

**Defined:** 2026-03-26
**Core Value:** Prove that data from an edge device has not been tampered with — from capture to verification — using cryptographic signatures, continuity chains, and verifiable receipts.

## v2.9 Requirements

Requirements for security review P2 remediation. Each maps to roadmap phases.

### Insecure Defaults

- [ ] **DFLT-01**: CAConfig::default() cannot produce a usable config with placeholder JWT secret — either remove Default impl or add runtime guard
- [ ] **DFLT-02**: SoftwareHsmConfig::default() cannot use "changeme123!" passphrase outside test builds — require explicit passphrase or panic on demo default

### Code Quality

- [ ] **QUAL-01**: Regex in validate_segment_hashes() compiled once via std::sync::LazyLock, not per-request
- [ ] **QUAL-02**: trst-cli emits stderr warning when --unencrypted flag is used, noting security implications

### Deployment Hardening

- [ ] **DEPL-01**: wasm-tests.yml has explicit permissions: contents: read block matching ci.yml and semver.yml
- [ ] **DEPL-02**: nginx.conf includes X-Content-Type-Options, X-Frame-Options, Referrer-Policy, and Content-Security-Policy headers
- [ ] **DEPL-03**: nginx-ssl.conf.template adds Strict-Transport-Security header and redirects HTTP to HTTPS when TLS is enabled

## Future Requirements

None — this is a targeted remediation milestone.

## Out of Scope

| Feature | Reason |
|---------|--------|
| Full security audit re-run | These are specific findings, not a new audit |
| P3 findings | Lower priority items deferred to future milestone |
| New feature development | Focus is remediation only |

## Traceability

Which phases cover which requirements. Updated during roadmap creation.

| Requirement | Phase | Status |
|-------------|-------|--------|
| DFLT-01 | Phase 68 | Pending |
| DFLT-02 | Phase 68 | Pending |
| QUAL-01 | Phase 69 | Pending |
| QUAL-02 | Phase 69 | Pending |
| DEPL-01 | Phase 70 | Pending |
| DEPL-02 | Phase 70 | Pending |
| DEPL-03 | Phase 70 | Pending |

**Coverage:**
- v2.9 requirements: 7 total
- Mapped to phases: 7
- Unmapped: 0 ✓

---
*Requirements defined: 2026-03-26*
*Last updated: 2026-03-26 after roadmap creation*
