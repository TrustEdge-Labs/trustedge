# Phase 79: Self-Attestation CI - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-04-05
**Phase:** 79-self-attestation-ci
**Areas discussed:** Syft pinning strategy, Error handling policy

---

## Syft Pinning Strategy

| Option | Description | Selected |
|--------|-------------|----------|
| anchore/sbom-action@v0 | Official GitHub Action, SHA-pinned, handles install+SBOM generation in one step. Matches existing SHA-pinning pattern. | ✓ |
| Pin curl install to version | Keep curl install but pin a specific syft version tag. Simpler, but doesn't match SHA-pinned Actions pattern. | |
| Claude decides | Claude picks the best approach during planning | |

**User's choice:** anchore/sbom-action@v0
**Notes:** Matches the established pattern of SHA-pinning all GitHub Actions in ci.yml.

---

## Error Handling Policy

| Option | Description | Selected |
|--------|-------------|----------|
| continue-on-error: true | Attestation failures never block a release. Promote to required after 3+ successful releases. | ✓ |
| Required from the start | Attestation must pass for release to succeed. Riskier — external tool changes could block releases. | |
| Claude decides | Claude picks based on stability evidence during planning | |

**User's choice:** continue-on-error: true
**Notes:** Design doc specified this approach. Promotes to required after demonstrated stability.

---

## Claude's Discretion

- Exact SHA pin for anchore/sbom-action
- Whether to rename ephemeral.pub to build.pub
- Step ordering and comment cleanup in CI job

## Deferred Ideas

None.
