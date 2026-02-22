<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


# Requirements: TrustEdge v1.5 Platform Consolidation

**Defined:** 2026-02-21
**Core Value:** A single, reliable trustedge-core library that owns all cryptographic operations — thin CLIs and WASM bindings are just frontends.

## v1.5 Requirements

### Service Consolidation

- [x] **SVC-01**: platform-api and verify-core merged into a single `trustedge-platform` service crate in the main trustedge workspace
- [x] **SVC-02**: trustedge-ca (Certificate Authority) from platform-api preserved as a workspace crate or module
- [x] **SVC-03**: Combined REST API surface serves all existing endpoints (devices, receipts, verification, JWKS)
- [x] **SVC-04**: All existing integration tests from both services pass in the consolidated crate

### Type Centralization

- [x] **TYPE-01**: te_shared types live in the main trustedge workspace as a workspace crate
- [x] **TYPE-02**: Uuid and DateTime types adopt platform-api's implementation
- [x] **TYPE-03**: JSON schema generation capability preserved from shared-libs version

### Crypto Deduplication

- [ ] **CRYPTO-01**: Manual crypto and chaining code in verify-core deleted
- [ ] **CRYPTO-02**: Verification logic uses `trustedge_core::chain` and `trustedge_core::crypto` instead of hand-rolled implementations

### Ghost Repo Cleanup

- [ ] **REPO-01**: 6 empty scaffold repos archived on GitHub (audit, billing-service, device-service, identity-service, infra, ingestion-service)
- [ ] **REPO-02**: Consolidated service design documents what functionality the ghost repos intended, for future reference

## Future Requirements

### Dashboard Integration

- **DASH-01**: SvelteKit dashboard (~139 LOC) consolidated into workspace or separate deployment
- **DASH-02**: Dashboard API client updated to point at consolidated platform service

## Out of Scope

| Feature | Reason |
|---------|--------|
| New API endpoints | Consolidation only — no new functionality |
| Database migrations | Preserve existing PostgreSQL schema as-is |
| Dashboard frontend | Separate technology (SvelteKit), defer to future milestone |
| Microservice split | Reviewer explicitly said "monolithic service until scale dictates otherwise" |
| YubiKey v1.1 rewrite | Already shipped in v1.1 milestone |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| TYPE-01 | Phase 24 | Complete |
| TYPE-02 | Phase 24 | Complete |
| TYPE-03 | Phase 24 | Complete |
| SVC-01 | Phase 25 | Complete |
| SVC-02 | Phase 25 | Complete |
| SVC-03 | Phase 25 | Complete |
| SVC-04 | Phase 25 | Complete |
| CRYPTO-01 | Phase 26 | Pending |
| CRYPTO-02 | Phase 26 | Pending |
| REPO-01 | Phase 27 | Pending |
| REPO-02 | Phase 27 | Pending |

**Coverage:**
- v1.5 requirements: 11 total
- Mapped to phases: 11
- Unmapped: 0

---
*Requirements defined: 2026-02-21*
*Last updated: 2026-02-21 after roadmap creation*
