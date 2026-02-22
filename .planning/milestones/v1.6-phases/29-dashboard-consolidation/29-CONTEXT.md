# Phase 29: Dashboard Consolidation - Context

**Gathered:** 2026-02-22
**Status:** Ready for planning

<domain>
## Phase Boundary

Move the trustedge-dashboard SvelteKit app from the TrustEdge-Labs/trustedge-dashboard GitHub repo into `web/dashboard/` in the monorepo. Replace hardcoded TypeScript type definitions with types generated from `trustedge-types` schemars schemas. The dashboard must build and run from its new location with no manual adjustments.

No new dashboard features. No CI integration for the dashboard. No Docker containerization of the dashboard.

</domain>

<decisions>
## Implementation Decisions

### Import strategy
- Copy files from TrustEdge-Labs/trustedge-dashboard repo (no git history preserved — clean break)
- Standard exclusions: skip node_modules, .git, dist/build artifacts, .env files — source files and configs only
- After successful move, the trustedge-dashboard repo will be deleted (handled by Phase 30)

### Type generation
- Pipeline: Rust binary/test dumps JSON Schema from schemars, then json-schema-to-typescript converts to TypeScript
- Output: `web/dashboard/src/lib/types.ts` (replaces existing hand-written types file in place)
- JSON Schema intermediate files are ephemeral — only the final types.ts is committed
- Include a `scripts/generate-types.sh` shell script so types can be regenerated when Rust types change
- No npm dependency on Rust crates — generated types.ts is self-contained

### Path and config changes
- API base URL hardcoded to localhost:3001 for dev — production config is out of scope this milestone
- No npm workspace dependency on trustedge-types — type generation is the bridge
- Researcher should examine the dashboard repo for any .env or config files that need updating for the new location

### Dev workflow
- Standard SvelteKit workflow: `cd web/dashboard && npm run dev` — no root-level npm scripts
- CLAUDE.md updated with a Dashboard section in Build & Test Commands
- No dashboard CI checks this milestone — focus on getting the move done correctly
- docker-compose.yml stays backend-only (postgres + platform-server) — dashboard runs natively with npm

### Claude's Discretion
- Exact json-schema-to-typescript configuration and options
- How to handle any API proxy/middleware patterns found in the dashboard
- Package.json adjustments needed for the new location
- Whether to keep or update the dashboard's existing README

</decisions>

<specifics>
## Specific Ideas

- Dashboard is ~29 SvelteKit files (noted during v1.5 archival decision)
- trustedge-types already uses schemars 0.8 for JSON Schema generation (confirmed in v1.5)
- Success criteria from ROADMAP.md are explicit: `npm run dev` and `npm run build` must succeed, and `web/dashboard/src/lib/types.ts` must be generated from schemas

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope

</deferred>

---

*Phase: 29-dashboard-consolidation*
*Context gathered: 2026-02-22*
