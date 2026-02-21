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

- [ ] **SVC-01**: platform-api and verify-core merged into a single `trustedge-platform` service crate in the main trustedge workspace
- [ ] **SVC-02**: trustedge-ca (Certificate Authority) from platform-api preserved as a workspace crate or module
- [ ] **SVC-03**: Combined REST API surface serves all existing endpoints (devices, receipts, verification, JWKS)
- [ ] **SVC-04**: All existing integration tests from both services pass in the consolidated crate

### Type Centralization

- [ ] **TYPE-01**: te_shared types live in the main trustedge workspace as a workspace crate
- [ ] **TYPE-02**: Uuid and DateTime types adopt platform-api's implementation
- [ ] **TYPE-03**: JSON schema generation capability preserved from shared-libs version

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
| SVC-01 | — | Pending |
| SVC-02 | — | Pending |
| SVC-03 | — | Pending |
| SVC-04 | — | Pending |
| TYPE-01 | — | Pending |
| TYPE-02 | — | Pending |
| TYPE-03 | — | Pending |
| CRYPTO-01 | — | Pending |
| CRYPTO-02 | — | Pending |
| REPO-01 | — | Pending |
| REPO-02 | — | Pending |

**Coverage:**
- v1.5 requirements: 11 total
- Mapped to phases: 0
- Unmapped: 11

---
*Requirements defined: 2026-02-21*
*Last updated: 2026-02-21 after initial definition*
