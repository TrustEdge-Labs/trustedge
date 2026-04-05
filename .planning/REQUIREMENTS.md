<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->

# Requirements: TrustEdge v5.0 Portfolio Polish

**Defined:** 2026-04-05
**Core Value:** Make the existing SBOM attestation work visible and discoverable.

## v5.0 Requirements

Requirements for this milestone. Each maps to roadmap phases.

### CI/CD

- [ ] **CI-01**: Self-attestation job runs end-to-end on every GitHub release, producing `.te-attestation.json` and `build.pub` as release assets
- [ ] **CI-02**: Self-attestation generates SBOM via pinned `anchore/sbom-action@v0` and binds it to the release binary
- [ ] **CI-03**: Self-attestation uses ephemeral Ed25519 keypair per build (no stored secrets)
- [ ] **CI-04**: Self-attestation job uses `continue-on-error: true` until promoted to required

### Distribution

- [ ] **DIST-01**: GitHub Action published to marketplace as `TrustEdge-Labs/attest-sbom-action@v1` (separate repo)
- [ ] **DIST-02**: Action downloads trst binary from GitHub Releases with SHA256 verification
- [ ] **DIST-03**: Action accepts inputs: sbom-path, binary-path, key-path, trustedge-version
- [ ] **DIST-04**: Action README includes usage example with persistent key and ephemeral key patterns

### Visibility

- [ ] **VIS-01**: README demo GIF shows the full attest-sbom → verify-attestation flow
- [ ] **VIS-02**: Product landing page live on trustedgelabs.com with quick start and verifier link
- [ ] **VIS-03**: Demo GIF recorded from `scripts/demo.sh --local` (or updated script if needed)

### Housekeeping

- [ ] **HK-01**: te-prove design doc archived in `.planning/ideas/` with office hours context notes

## Future Requirements

Deferred to future milestones. Tracked but not in current roadmap.

### Security Enhancements
- **SEC-01**: OIDC/Sigstore integration for identity-bound attestations
- **SEC-02**: SPDX SBOM format support (in addition to CycloneDX)

### Product Features
- **PROD-01**: C2PA compatibility for media profiles (cam.video, audio) — first amendment auditor use case
- **PROD-02**: Verification badge endpoint for README embedding
- **PROD-03**: SBOM diff/drift detection between attested versions
- **PROD-04**: te-prove — FOSS supply chain trust policy engine (parked, see .planning/ideas/)

## Out of Scope

Explicitly excluded. Documented to prevent scope creep.

| Feature | Reason |
|---------|--------|
| te-prove implementation | No demand evidence, undefined target user, FOMO-driven — parked as idea |
| New cryptographic features | This milestone is visibility/polish, not new capabilities |
| Dashboard changes | Dashboard is functional, not part of portfolio polish scope |
| Multi-tenant platform features | Premature — no users yet |

## Traceability

Which phases cover which requirements. Updated during roadmap creation.

| Requirement | Phase | Status |
|-------------|-------|--------|
| CI-01 | Phase 79 | Pending |
| CI-02 | Phase 79 | Pending |
| CI-03 | Phase 79 | Pending |
| CI-04 | Phase 79 | Pending |
| HK-01 | Phase 79 | Pending |
| DIST-01 | Phase 80 | Pending |
| DIST-02 | Phase 80 | Pending |
| DIST-03 | Phase 80 | Pending |
| DIST-04 | Phase 80 | Pending |
| VIS-01 | Phase 81 | Pending |
| VIS-03 | Phase 81 | Pending |
| VIS-02 | Phase 82 | Pending |

**Coverage:**
- v5.0 requirements: 12 total
- Mapped to phases: 12
- Unmapped: 0 ✓

---
*Requirements defined: 2026-04-05*
*Last updated: 2026-04-05 — traceability populated after roadmap creation*
