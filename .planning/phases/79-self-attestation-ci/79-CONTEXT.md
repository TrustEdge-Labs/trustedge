# Phase 79: Self-Attestation CI - Context

**Gathered:** 2026-04-05
**Status:** Ready for planning

<domain>
## Phase Boundary

Wire up end-to-end self-attestation in the CI release workflow so every GitHub release produces verifiable attestation artifacts. Also archive the te-prove design doc for future reference. The self-attestation CI job already exists at `.github/workflows/ci.yml:171` — this phase fixes gaps and completes the wiring.

</domain>

<decisions>
## Implementation Decisions

### SBOM Generation
- **D-01:** Replace the unpinned `curl | sh` syft install with `anchore/sbom-action@v0` (SHA-pinned). This matches the existing pattern of SHA-pinning all GitHub Actions in ci.yml.
- **D-02:** The sbom-action handles both syft installation and SBOM generation in one step, replacing the current two-step install+run.

### Release Asset Uploads
- **D-03:** Upload `ephemeral.pub` (or rename to `build.pub`) as a release asset alongside `.te-attestation.json`. A verifier needs both files to independently verify the attestation.

### Error Handling
- **D-04:** Add `continue-on-error: true` to the self-attestation job. Attestation failures should never block a legitimate release. Promote to a required check after 3+ successful releases confirm stability.

### Housekeeping
- **D-05:** Move `te-prove-design-doc.md` from repo root to `.planning/ideas/te-prove-design-doc.md`. Add a brief context note referencing the April 5 office hours session (no demand evidence, FOMO-driven, parked).

### Claude's Discretion
- Exact SHA pin for anchore/sbom-action (look up latest stable release)
- Whether to rename `ephemeral.pub` to `build.pub` in the workflow for clarity
- Any additional cleanup of the CI job (step ordering, comments)

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### CI Workflow
- `.github/workflows/ci.yml` — The existing self-attestation job (lines 171-216), all SHA-pinned actions, and the build-and-test dependency chain

### Design Context
- `~/.gstack/projects/TrustEdge-Labs-trustedge/john-main-design-20260405-085506.md` — Approved design doc with full deliverable specs, key management decisions, trust model honesty note
- `~/.gstack/projects/TrustEdge-Labs-trustedge/ceo-plans/2026-04-01-sbom-attestation-wedge.md` — CEO plan accepting self-attestation as a scope expansion

### Requirements
- `.planning/REQUIREMENTS.md` — CI-01 through CI-04 and HK-01 requirement definitions

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- Self-attestation CI job already exists (`.github/workflows/ci.yml:171-216`) — builds trst, generates ephemeral keypair, creates SBOM, creates attestation, uploads to release
- `trst keygen --unencrypted` — generates ephemeral Ed25519 keypair
- `trst attest-sbom` — creates the attestation binding SBOM to binary
- All GitHub Actions already SHA-pinned (actions/checkout, dtolnay/rust-toolchain, Swatinem/rust-cache)

### Established Patterns
- SHA-pinned GitHub Actions with version comments (e.g., `@34e114876b...  # v4`)
- Release-tag trigger: `if: startsWith(github.ref, 'refs/tags/v')`
- `permissions: contents: write` for release asset uploads
- `gh release upload --clobber` for idempotent asset uploads

### Integration Points
- The self-attestation job runs after `build-and-test` via `needs: build-and-test`
- Release assets uploaded via `gh release upload` with GITHUB_TOKEN

</code_context>

<specifics>
## Specific Ideas

No specific requirements — the design doc and existing CI job define the approach clearly.

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope.

</deferred>

---

*Phase: 79-self-attestation-ci*
*Context gathered: 2026-04-05*
