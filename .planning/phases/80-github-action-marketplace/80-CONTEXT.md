# Phase 80: GitHub Action Marketplace - Context

**Gathered:** 2026-04-05
**Status:** Ready for planning

<domain>
## Phase Boundary

Publish `TrustEdge-Labs/attest-sbom-action@v1` to the GitHub Marketplace as a separate repo. The action already exists at `actions/attest-sbom-action/` in the monorepo — this phase creates the separate repo, adds SHA256 binary verification, polishes the README, and submits to the marketplace.

</domain>

<decisions>
## Implementation Decisions

### Repo Strategy
- **D-01:** Create a new public repo `TrustEdge-Labs/attest-sbom-action` on GitHub using the `gh` CLI.
- **D-02:** Copy `action.yml`, `README.md`, and `LICENSE` from `actions/attest-sbom-action/` in the monorepo. The monorepo copy remains as a reference but the marketplace listing points to the separate repo.
- **D-03:** Tag `v1` and `v1.0.0` on the separate repo after pushing.

### Binary Verification
- **D-04:** Add SHA256 checksum verification to the binary download step. Download the checksum file from GitHub Releases (e.g., `trst.sha256`) alongside the binary, verify with `sha256sum --check`. If no checksum file exists in the release, skip verification with a warning (graceful degradation for older releases).
- **D-05:** Input names stay as-is: `binary`, `sbom`, `key`, `trst-version`. These are cleaner than the design doc's `sbom-path`/`binary-path`/`key-path` naming.

### README Polish
- **D-06:** Add two explicit side-by-side usage examples: "Ephemeral key (recommended for CI)" and "Persistent key (stored as GitHub Secret)". The persistent key example shows `echo "${{ secrets.TRUSTEDGE_KEY }}" > build.key` pattern.
- **D-07:** Add a note explaining that the action outputs a local `.te-attestation.json` file. Submitting to a TrustEdge Platform instance for a signed receipt is an optional follow-on step.

### Marketplace Listing
- **D-08:** Accept the GitHub Marketplace Developer Agreement and submit the listing. Budget 1-2 business days for review. The action is usable via direct `uses:` reference before listing is approved.

### Claude's Discretion
- Exact SHA256 verification script implementation
- Whether to add a CONTRIBUTING.md to the separate repo
- Marketplace listing categories and description text

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Existing Action
- `actions/attest-sbom-action/action.yml` — The existing composite action to copy and enhance
- `actions/attest-sbom-action/README.md` — Existing README to polish
- `actions/attest-sbom-action/LICENSE` — MPL-2.0 license to copy

### Design Context
- `~/.gstack/projects/TrustEdge-Labs-trustedge/john-main-design-20260405-085506.md` — Approved design doc, section "2. GitHub Action"

### CI Reference
- `.github/workflows/ci.yml` — Self-attestation job shows how the action is used internally (lines 171-216)

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `actions/attest-sbom-action/action.yml` — Complete composite action with download, keygen, and attestation steps
- `actions/attest-sbom-action/README.md` — Usage docs with minimal and full examples
- `actions/attest-sbom-action/LICENSE` — MPL-2.0

### Established Patterns
- Binary download from GitHub Releases: `curl -fsSL` with version interpolation
- Ephemeral keypair generation via `trst keygen --unencrypted`
- Output via `$GITHUB_OUTPUT` for `attestation-path`
- Environment variables via `$GITHUB_ENV` for key paths

### Integration Points
- The separate repo's `action.yml` references `TrustEdge-Labs/trustedge` releases for binary downloads
- Self-attestation job in ci.yml already uses the internal action as a reference pattern

</code_context>

<specifics>
## Specific Ideas

No specific requirements — the existing action and design doc define the approach clearly.

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope.

</deferred>

---

*Phase: 80-github-action-marketplace*
*Context gathered: 2026-04-05*
