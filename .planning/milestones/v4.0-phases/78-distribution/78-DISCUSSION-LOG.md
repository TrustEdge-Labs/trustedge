# Phase 78: Distribution - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.

## Session: 2026-04-03

### Areas Selected
All 3 gray areas discussed: Landing page scope, Self-attestation CI step, GitHub Action design.

### Q&A

**Area: Landing page scope**
- Q: Content file in monorepo, full page in website repo, or Claude decides?
- A: **Content file in monorepo** — docs/landing-page.md. User deploys to trustedgelabs-website separately.

**Area: Self-attestation CI step**
- Q: Ephemeral key + document third-party, stored key + attest real project, or Claude decides?
- A: **Ephemeral key + document third-party** — throwaway keypair per CI run. Third-party demo is documentation, not actual attestation.

**Area: GitHub Action design**
- Q: New repo + composite action, subfolder + Docker action, or Claude decides?
- A: **New repo + composite action** — TrustEdge-Labs/attest-sbom-action, composite action, downloads pre-built trst binary.
