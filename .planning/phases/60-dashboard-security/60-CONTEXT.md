# Phase 60: Dashboard Security - Context

**Gathered:** 2026-03-24
**Status:** Ready for planning

<domain>
## Phase Boundary

Remove `VITE_API_KEY` from the dashboard client-side bundle. Dashboard communicates with public platform endpoints only — no Bearer token needed.

</domain>

<decisions>
## Implementation Decisions

### API key removal
- **D-01:** Remove `VITE_API_KEY` entirely — delete from `config.ts`, `.env.example`, and all references.
- **D-02:** Remove the `Authorization: Bearer` header from `api.ts` — dashboard only accesses public endpoints.
- **D-03:** Remove the `apiKey` field from `ApiClient` constructor and the `config` object.
- **D-04:** Update the config warning in `+page.svelte` — remove the API key configuration check and instructions. The dashboard only needs `VITE_API_BASE` to be configured.

### Scope limitation
- **D-05:** Dashboard only accesses public endpoints: `/v1/verify` (POST), `/.well-known/jwks.json` (GET), `/healthz` (GET). The postgres-protected routes (`/v1/devices`, `/v1/receipts`) are admin-only and do not need a browser UI.
- **D-06:** If the dashboard currently has pages that call protected endpoints (devices, receipts), those pages should show a "not available in this deployment" message or be removed.

### Build-time check
- **D-07:** Add a CI/build-time check that greps the built bundle for `VITE_API_KEY` to prevent re-introduction. Can be a simple `grep -r "VITE_API_KEY" web/dashboard/build/` check in CI or a npm script.

### Claude's Discretion
- Whether to keep the receipts page with read-only public access or remove it
- Exact CI check mechanism (npm script vs ci-check.sh step)
- Whether to add a comment in config.ts explaining why there's no API key

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Dashboard files
- `web/dashboard/src/lib/config.ts` — `apiKey: import.meta.env.VITE_API_KEY || ''` (line 3)
- `web/dashboard/src/lib/api.ts` — `ApiClient` class with `apiKey` field (line 11), Bearer header (line 23)
- `web/dashboard/src/routes/+page.svelte` — `config.apiKey` check (line 8), configuration warning UI (lines 21-25)
- `web/dashboard/src/routes/receipts/+page.svelte` — `Bearer YOUR_API_KEY` in example curl (line 136)
- `web/dashboard/.env.example` — `VITE_API_KEY=your-api-key-here` (line 2)

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- SvelteKit static adapter already configured — no SSR, pure client-side
- `config.ts` is the single configuration source — clean removal point
- `api.ts` ApiClient is the single HTTP client — clean header removal point

### Established Patterns
- Vite `import.meta.env.VITE_*` for build-time env injection
- `npm run build` produces static files in `build/`
- `npm run check` for TypeScript validation

### Integration Points
- `config.ts` → consumed by `api.ts` and `+page.svelte`
- `api.ts` → consumed by all route pages (devices, receipts, etc.)
- `.env.example` → documentation for deployment

</code_context>

<specifics>
## Specific Ideas

No specific requirements — straightforward removal of client-side credentials.

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope.

</deferred>

---

*Phase: 60-dashboard-security*
*Context gathered: 2026-03-24*
