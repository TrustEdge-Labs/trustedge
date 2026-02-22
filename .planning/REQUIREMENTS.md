<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


# Requirements: TrustEdge

**Defined:** 2026-02-22
**Core Value:** A single, reliable trustedge-core library that owns all cryptographic operations — thin CLIs and WASM bindings are just frontends.

## v1.6 Requirements

Requirements for Final Consolidation milestone. Each maps to roadmap phases.

### Platform Server

- [ ] **PLAT-01**: Platform service runs as a standalone binary (`trustedge-platform-server`)
- [ ] **PLAT-02**: Server loads configuration from environment variables (PORT, DATABASE_URL, JWT_AUDIENCE)
- [ ] **PLAT-03**: Server boots Axum router via `trustedge-platform::create_router()`
- [ ] **PLAT-04**: Server handles graceful shutdown on SIGTERM/SIGINT

### Web Consolidation

- [ ] **WEB-01**: Dashboard source lives at `web/dashboard/` in the trustedge workspace
- [ ] **WEB-02**: Dashboard builds and runs from its new location
- [ ] **WEB-03**: Dashboard's hardcoded `types.ts` replaced with types generated from `trustedge-types` schemas

### Repo Cleanup

- [ ] **REPO-01**: 12 orphaned repos deleted from TrustEdge-Labs GitHub org
- [ ] **REPO-02**: CLAUDE.md updated to remove references to archived/deleted repos
- [ ] **REPO-03**: Documentation updated to reflect final repo structure (3 repos: trustedge, website, shipsecure)

## Future Requirements

### Deferred

- Pubky adapter merged into core protocols/pubky/ (feature-gated)
- Pubky-advanced hybrid encryption merged into core
- Prelude module for common imports
- Updated documentation with module-level security considerations

## Out of Scope

| Feature | Reason |
|---------|--------|
| Dashboard feature additions | Moving code only — no new UI features this milestone |
| Platform API changes | Server binary boots existing router — no endpoint modifications |
| trustedgelabs-website changes | Separate product site, not part of trustedge workspace |
| shipsecure integration | Separate product, not related to this consolidation |
| Type generation CI pipeline | Types are generated once; automated pipeline deferred to future |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| PLAT-01 | — | Pending |
| PLAT-02 | — | Pending |
| PLAT-03 | — | Pending |
| PLAT-04 | — | Pending |
| WEB-01 | — | Pending |
| WEB-02 | — | Pending |
| WEB-03 | — | Pending |
| REPO-01 | — | Pending |
| REPO-02 | — | Pending |
| REPO-03 | — | Pending |

**Coverage:**
- v1.6 requirements: 10 total
- Mapped to phases: 0
- Unmapped: 10

---
*Requirements defined: 2026-02-22*
*Last updated: 2026-02-22 after initial definition*
