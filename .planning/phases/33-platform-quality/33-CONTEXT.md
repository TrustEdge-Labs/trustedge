# Phase 33: Platform Quality - Context

**Gathered:** 2026-02-22
**Status:** Ready for planning

<domain>
## Phase Boundary

Harden the platform HTTP layer: deduplicate verify handler validation into a single always-compiled path, replace permissive CORS on verify-only builds with restrictive policy, and clarify the CA module's status as library-only with no HTTP exposure. No new capabilities — this is internal quality work on existing code.

</domain>

<decisions>
## Implementation Decisions

### CORS policy
- Non-postgres (verify-only) build: same-origin only — no cross-origin requests allowed
- Postgres build: keep hardcoded localhost allowlist (localhost:3000, localhost:8080) — sufficient for dashboard dev
- Postgres build: restrict allowed headers to Content-Type, Authorization, and Accept (replace current `Any`)
- No env-var configurability for now — hardcoded origins are fine for the foreseeable future

### CA module disposition
- Document as library-only — no HTTP routes wired into create_router()
- Add a brief note about potential future HTTP exposure behind `ca` feature flag
- Annotate each sub-module (api.rs, database.rs, auth.rs, service.rs) with current status and usage
- Refactor api.rs Axum handlers to plain service functions — remove Axum coupling from library-only module

### Verify handler deduplication
- Extract shared validation logic (~50 lines) into a public function — available to downstream crates and integration tests
- Keep first-error-wins behavior (return on first validation failure, don't collect all)
- Device lookup stays inline in postgres handler — it's postgres-specific, not shared
- Also extract receipt construction logic — deduplicate both validation and receipt building across both handler variants

### Claude's Discretion
- Exact function signatures and module placement for extracted validation/receipt functions
- Whether to create a new sub-module or add to existing validation.rs
- Error type design for the extracted functions

</decisions>

<specifics>
## Specific Ideas

No specific requirements — open to standard approaches

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope

</deferred>

---

*Phase: 33-platform-quality*
*Context gathered: 2026-02-22*
