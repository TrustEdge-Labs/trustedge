# Phase 25: Service Consolidation - Context

**Gathered:** 2026-02-21
**Status:** Ready for planning

<domain>
## Phase Boundary

Merge trustedge-platform-api (~4,173 LOC, Axum/PostgreSQL REST API with CA, 11 integration tests) and trustedge-verify-core (~1,685 LOC, verification service with manual crypto/chaining, 17 tests) into a single `trustedge-platform` crate in the main workspace. All endpoints preserved, all 28 tests passing. Manual crypto code from verify-core comes over as-is -- Phase 26 replaces it with trustedge-core.

</domain>

<decisions>
## Implementation Decisions

### CA placement
- CA is a private module inside trustedge-platform (`mod ca`, not `pub mod ca`)
- Not a separate workspace crate -- keeps the service self-contained
- CA internals can be simplified during merge if redundancy is found
- Researcher should check whether CA uses trustedge-core for crypto or has its own -- if own crypto, Phase 26 handles that

### Database dependencies
- PostgreSQL/sqlx is feature-gated: `--features postgres` enables the DB layer. Default build compiles without DB
- Integration tests requiring a database are `#[ignore]` by default, matching the YubiKey test pattern
- sqlx query style: match whatever platform-api currently uses (compile-time macros or runtime strings)
- Database migrations: match platform-api's current structure (include in crate or keep separate -- preserve as-is)

### Crate classification & dependencies
- Tier 1 (Stable) classification -- blocking in CI from day one
- Heavy dependencies (Axum, sqlx, tower, hyper) added as workspace-level deps in root Cargo.toml
- Dependency tree threshold: raise baseline to accommodate platform deps (the growth is intentional)
- Library only -- no binary/main.rs. Server startup is handled externally, not by this crate

### Migration approach
- Researcher decides module structure (lift-and-shift vs restructure) after examining both codebases
- Clean copy -- no git history preservation. History lives in original repos
- verify-core's manual crypto/chaining code comes over as-is. Phase 26 replaces it with trustedge-core primitives
- Source repos are local clones at:
  - `~/vault/projects/github.com/trustedge-platform-api/`
  - `~/vault/projects/github.com/trustedge-verify-core/`
  - `~/vault/projects/github.com/trustedge-shared-libs/` (for reference, already centralized in Phase 24)

### Claude's Discretion
- Internal module organization (routes/, models/, db/, etc. vs preserving original structure)
- Axum router composition and middleware strategy
- How to unify configuration between the two services
- Test organization and fixture management

</decisions>

<specifics>
## Specific Ideas

- Feature-gating the DB layer means the crate compiles and can be tested (non-DB parts) without a PostgreSQL instance
- #[ignore] pattern for DB tests matches the established YubiKey precedent -- consistent test philosophy
- Library-only crate keeps deployment flexible -- a thin binary or the main CLI can start the server

</specifics>

<deferred>
## Deferred Ideas

None -- discussion stayed within phase scope

</deferred>

---

*Phase: 25-service-consolidation*
*Context gathered: 2026-02-21*
