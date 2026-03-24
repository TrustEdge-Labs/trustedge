# Requirements: TrustEdge v2.6

**Defined:** 2026-03-23
**Core Value:** Prove that data from an edge device has not been tampered with — from capture to verification — using cryptographic signatures, continuity chains, and verifiable receipts.

## v2.6 Requirements

Requirements for P1 security hardening. Each maps to roadmap phases.

### Core Crypto

- [x] **CORE-01**: `PrivateKey` (asymmetric.rs), `SessionInfo.session_key` (auth.rs), `ClientAuthResult.session_key` (auth.rs), and `SymmetricKey` (hybrid.rs) implement `Zeroize` and `ZeroizeOnDrop`
- [x] **CORE-02**: `import_secret_encrypted()` rejects key files with PBKDF2 iteration count below 600,000

### Platform

- [ ] **PLAT-01**: `/v1/verify` handler works correctly in postgres mode without requiring `OrgContext` from auth middleware
- [ ] **PLAT-02**: CORS allowed origins are configurable via `CORS_ORIGINS` environment variable (not hardcoded to localhost)

### CLI

- [ ] **CLI-01**: `trustedge-cli` does not print encryption key to stderr unless `--show-key` flag is explicitly provided

### Deploy

- [ ] **DEPL-01**: nginx configuration supports TLS termination (HTTPS on port 443) with configurable certificate paths

### Dashboard

- [ ] **DASH-01**: Dashboard does not embed `VITE_API_KEY` in the client-side JavaScript bundle; API authentication uses a server-proxied approach or is removed

## Out of Scope

| Feature | Reason |
|---------|--------|
| mTLS client certificates | Infrastructure-level, not application code — separate initiative |
| Key rotation automation | Good practice but not a P1 vulnerability |
| WAF/DDoS protection | Infrastructure concern beyond application hardening |
| Full OAuth/OIDC for dashboard | Over-engineered for current deployment model |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| CORE-01 | Phase 57 | Complete |
| CORE-02 | Phase 57 | Complete |
| PLAT-01 | Phase 58 | Pending |
| PLAT-02 | Phase 58 | Pending |
| CLI-01 | Phase 59 | Pending |
| DEPL-01 | Phase 59 | Pending |
| DASH-01 | Phase 60 | Pending |

**Coverage:**
- v2.6 requirements: 7 total
- Mapped to phases: 7
- Unmapped: 0 ✓

---
*Requirements defined: 2026-03-23*
*Last updated: 2026-03-23 after roadmap created*
