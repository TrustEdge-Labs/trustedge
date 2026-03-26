<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->

# Requirements: TrustEdge v3.0 Release Polish

**Defined:** 2026-03-26
**Core Value:** Prove that data from an edge device has not been tampered with — from capture to verification — using cryptographic signatures, continuity chains, and verifiable receipts.

## v3.0 Requirements

Requirements for the final polish before official v3.0 signed release. Each maps to roadmap phases.

### Platform Code Quality

- [x] **PLAT-01**: JWS receipt TTL is configurable via `RECEIPT_TTL_SECS` env var (default 3600 seconds)
- [x] **PLAT-02**: `/healthz` response omits exact crate version for unauthenticated callers
- [x] **PLAT-03**: Invalid `PORT` env var causes startup failure with clear error message instead of silently defaulting to 3001

### Core Crypto Hygiene

- [ ] **CORE-01**: `generate_aad()` uses `.expect("AAD serialization is infallible")` instead of `.unwrap()`
- [ ] **CORE-02**: `Envelope::hash()` returns `Result` instead of `unwrap_or_default()` that silently produces empty-input hash on failure

### Deployment Hardening

- [ ] **DEPL-01**: nginx security headers (X-Content-Type-Options, X-Frame-Options, Referrer-Policy, CSP) are present in all location blocks in both `nginx.conf` and `nginx-ssl.conf.template`
- [ ] **DEPL-02**: CSP `connect-src` directive includes the configured API origin (from `VITE_API_BASE`) so dashboard API calls are not blocked when re-enabled
- [ ] **DEPL-03**: Docker Compose database credentials use `env_file` or Docker secrets instead of inline plaintext password

### Release Documentation

- [ ] **DOCS-01**: README reflects current feature set, CLI commands, architecture, and v3.0 state
- [ ] **DOCS-02**: User-facing documentation (docs/, CLAUDE.md CLI tables, demo instructions) is current and consistent with codebase

## Future Requirements

None — v3.0 is the release milestone.

## Out of Scope

| Feature | Reason |
|---------|--------|
| Finding #9 (CAConfigBuilder placeholder) | Already resolved in v2.9 Phase 68 — builder panics on sentinel outside cfg(test) |
| New features | v3.0 is polish-only; new features go to v3.1+ |
| Post-quantum cryptography | Research phase only, no production use case |
| TPM support | No hardware to test against |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| PLAT-01 | Phase 71 | Complete |
| PLAT-02 | Phase 71 | Complete |
| PLAT-03 | Phase 71 | Complete |
| CORE-01 | Phase 72 | Pending |
| CORE-02 | Phase 72 | Pending |
| DEPL-01 | Phase 73 | Pending |
| DEPL-02 | Phase 73 | Pending |
| DEPL-03 | Phase 73 | Pending |
| DOCS-01 | Phase 74 | Pending |
| DOCS-02 | Phase 74 | Pending |

**Coverage:**
- v3.0 requirements: 10 total
- Mapped to phases: 10
- Unmapped: 0

---
*Requirements defined: 2026-03-26*
*Last updated: 2026-03-26 after roadmap creation*
