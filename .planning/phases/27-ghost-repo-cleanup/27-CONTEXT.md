# Phase 27: Ghost Repo Cleanup - Context

**Gathered:** 2026-02-22
**Status:** Ready for planning

<domain>
## Phase Boundary

Archive 6 empty scaffold repos on GitHub (TrustEdge-Labs org) and document what each was intended to become. These repos were created for a SaaS-type platform for commercial use and licensing as part of the open-core business model. The repos are: audit, billing-service, device-service, identity-service, infra, ingestion-service.

</domain>

<decisions>
## Implementation Decisions

### Scope documentation
- Add a section to CLAUDE.md (not a standalone file)
- Brief intent only: 1-2 sentences per repo in table format
- Include where the intended functionality lives now (or note "not implemented, deferred")
- Names only, no GitHub links
- Original context: these were intended as microservices for a commercial SaaS platform (open-core model)

### Archive process
- Repos are under github.com/TrustEdge-Labs/
- Update each repo's README with a simple redirect notice before archiving: "Archived. See [trustedge](link) main workspace."
- No content transfer — archive as-is
- Archive via GitHub API after README update

### Repo investigation
- Infer intent from repo names — no deep commit history research needed
- Before archiving, do a quick API check per repo (file count, commit count) to verify they're truly scaffolds
- If a repo has meaningful code: flag it, skip archiving that repo, continue with the others
- Don't archive repos with substantial content without manual review

### Claude's Discretion
- Exact wording of the CLAUDE.md section header and table columns
- How to determine "meaningful code" threshold (e.g., file count, LOC)
- Order of repos in the documentation table

</decisions>

<specifics>
## Specific Ideas

- The repos were part of a planned microservice architecture for TrustEdge's commercial offering — audit trail, billing, device management, identity/auth, infrastructure, and data ingestion
- The consolidation milestone (v1.5) moved toward a monolithic approach, making these separate repos unnecessary

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope

</deferred>

---

*Phase: 27-ghost-repo-cleanup*
*Context gathered: 2026-02-22*
