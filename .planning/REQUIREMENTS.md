<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->

# Requirements: TrustEdge

**Defined:** 2026-03-25
**Core Value:** Prove that data from an edge device has not been tampered with — from capture to verification — using cryptographic signatures, continuity chains, and verifiable receipts.

## v2.8 Requirements

Requirements for milestone v2.8 High Priority Hardening. Each maps to roadmap phases.

### Platform HTTP

- [x] **HTTP-01**: Rate limiter parses X-Forwarded-For from trusted proxies for per-client rate limiting behind reverse proxies
- [x] **HTTP-02**: 429 responses include Retry-After header per RFC 6585

### Key Material Safety

- [x] **KEY-01**: Auto-generated key files in `trst wrap` get 0600 Unix permissions (matching `keygen` behavior)
- [x] **KEY-02**: PrivateKey serde derives removed or key_bytes field made private to prevent accidental serialization

### Crypto Construction

- [ ] **CRYPT-01**: NetworkChunk::new() requires nonce as mandatory parameter (no zero-nonce default)

### CLI Hardening

- [x] **CLI-01**: All process::exit() calls in trst-cli replaced with proper error returns (11 call sites)
- [x] **CLI-02**: --chunk-size in trst-cli wrap has upper bound (256 MB ceiling) with clear error

### Deployment

- [ ] **DEPL-01**: Dashboard nginx runs as non-root user (nginx-unprivileged or USER directive)
- [ ] **DEPL-02**: CI bundle credential guard (VITE_API_KEY grep) added to GitHub Actions ci.yml workflow

## Future Requirements

None — all findings addressed in this milestone.

## Out of Scope

| Feature | Reason |
|---------|--------|
| Full WAF integration | Beyond rate limiting scope; future infrastructure work |
| Automated key rotation | Not flagged in review; separate feature |
| Container image scanning | Not flagged in review; future CI hardening |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| HTTP-01 | Phase 64 | Complete |
| HTTP-02 | Phase 64 | Complete |
| KEY-01 | Phase 65 | Complete |
| KEY-02 | Phase 65 | Complete |
| CRYPT-01 | Phase 66 | Pending |
| CLI-01 | Phase 66 | Complete |
| CLI-02 | Phase 66 | Complete |
| DEPL-01 | Phase 67 | Pending |
| DEPL-02 | Phase 67 | Pending |

**Coverage:**
- v2.8 requirements: 9 total
- Mapped to phases: 9
- Unmapped: 0

---
*Requirements defined: 2026-03-25*
*Last updated: 2026-03-25 after roadmap created*
