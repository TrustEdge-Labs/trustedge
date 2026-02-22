# Phase 30: Repo Cleanup - Context

**Gathered:** 2026-02-22
**Status:** Ready for planning

<domain>
## Phase Boundary

Delete all orphaned GitHub repos from the TrustEdge-Labs org and update all documentation to reflect the 3-repo org structure (trustedge, trustedgelabs-website, shipsecure). No new capabilities — this is purely cleanup and documentation alignment.

</domain>

<decisions>
## Implementation Decisions

### Repo disposition
- Fully delete all orphaned repos — no archiving, no keeping read-only copies
- Claude audits the org to identify all repos beyond the 3 keepers (trustedge, trustedgelabs-website, shipsecure)
- Previously archived repos (billing, device, identity, ingestion, infra) also get fully deleted
- No concerns about external forks, stars, or references — all were internal scaffolds
- No special handling needed before deletion — everything needed has already been consolidated

### Documentation updates
- Remove the "Archived Service Repos" section from CLAUDE.md entirely
- Remove all references to the external trustedge-dashboard repo (dashboard is now in web/dashboard/)
- Full scrub: search all .md files and source code comments for stale repo references
- Update the GitHub org profile/README to reflect the 3-repo structure
- Only update docs within the trustedge repo — do NOT touch trustedgelabs-website or shipsecure

### Org description
- Do NOT touch the shipsecure repo in any way
- Only update docs within the trustedge repo itself
- GitHub org profile should list all 3 repos (trustedge, trustedgelabs-website, shipsecure) with brief roles

### Execution order
- Delete repos first, then update documentation to match the new state
- Claude executes `gh repo delete` commands directly (user will approve when prompted)
- Include a verification step: list org repos after deletion to confirm exactly 3 remain

### Claude's Discretion
- Exact wording of org profile description
- How to restructure CLAUDE.md sections after removing the archived repos table
- Whether to consolidate or remove other stale references found during the scrub

</decisions>

<specifics>
## Specific Ideas

No specific requirements — open to standard approaches.

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope.

</deferred>

---

*Phase: 30-repo-cleanup*
*Context gathered: 2026-02-22*
