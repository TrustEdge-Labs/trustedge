# Phase 24: Type Centralization - Context

**Gathered:** 2026-02-21
**Status:** Ready for planning

<domain>
## Phase Boundary

Migrate te_shared wire types (~501 LOC from trustedge-shared-libs) into the main trustedge workspace as a standalone crate. Uuid and DateTime types adopt platform-api's implementation. JSON schema generation preserved with exact output compatibility. This phase delivers the types crate only -- service consolidation and crypto deduplication are separate phases.

</domain>

<decisions>
## Implementation Decisions

### Crate naming & placement
- Crate name: `trustedge-types`
- Directory: `crates/types/`
- Tier classification: Stable (Tier 1) -- blocking in CI, same status as trustedge-core
- Dependency relationship: trustedge-core depends on trustedge-types and re-exports. Downstream crates only need trustedge-core in their dependencies

### Type migration scope
- Scope determination deferred to researcher -- Claude examines te_shared contents and recommends what to include vs drop
- If types in te_shared overlap with types already in trustedge-core, core types win -- delete the te_shared duplicate
- Dependencies: match whatever te_shared currently depends on (bring deps along, trim later if needed)
- Must be WASM-compatible (wasm32-unknown-unknown target)

### Uuid & DateTime strategy
- Direct re-export of uuid::Uuid and chrono::DateTime -- no newtype wrappers
- DateTime library: chrono
- Uuid generation NOT included in types crate -- types crate only defines the Uuid type for field definitions. Generation happens in services/core
- Uuid crate version: upgrade to latest (don't pin to platform-api's version)

### Schema generation
- JSON schema generation is a default feature (not feature-gated)
- Schema output must be exact match with current shared-libs output
- Exposed as a library function (e.g., `trustedge_types::schema::generate()`)
- Snapshot regression test: store expected schema as fixture, test fails if output drifts
- Researcher should check how shared-libs' schema output is currently consumed (automated or documentation-only) -- unknown at decision time

### Claude's Discretion
- Internal module structure within the types crate
- Which specific te_shared types/helpers to include vs exclude (after research)
- Schema generation library choice (schemars or equivalent) -- must produce exact-match output
- How to handle any WASM-incompatible dependencies if encountered

</decisions>

<specifics>
## Specific Ideas

- Core depends on types, not the other way around -- preserves "monolith core + thin shells" architecture
- Types crate should be minimal: it's a leaf dependency, not a framework
- WASM compatibility matters for potential future browser verification use

</specifics>

<deferred>
## Deferred Ideas

None -- discussion stayed within phase scope

</deferred>

---

*Phase: 24-type-centralization*
*Context gathered: 2026-02-21*
