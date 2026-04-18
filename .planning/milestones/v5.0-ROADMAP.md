<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->

# Roadmap: TrustEdge

## Milestones

- ✅ **v1.0 Consolidation** - Phases 1-8 (shipped 2026-02-11)
- ✅ **v1.1 YubiKey Overhaul** - Phases 9-12 (shipped 2026-02-11)
- ✅ **v1.2 Scope Reduction** - Phases 13-14 (shipped 2026-02-12)
- ✅ **v1.3 Dependency Audit** - Phases 15-18 (shipped 2026-02-13)
- ✅ **v1.4 Placeholder Elimination** - Phases 19-23 (shipped 2026-02-13)
- ✅ **v1.5 Platform Consolidation** - Phases 24-27 (shipped 2026-02-22)
- ✅ **v1.6 Final Consolidation** - Phases 28-30 (shipped 2026-02-22)
- ✅ **v1.7 Security Hardening** - Phases 31-34 (shipped 2026-02-23)
- ✅ **v1.8 KDF Fix** - Phases 35-37 (shipped 2026-02-24)
- ✅ **v2.0 End-to-End Demo** - Phases 38-41 (shipped 2026-03-16)
- ✅ **v2.1 Data Lifecycle** - Phases 42-44 (shipped 2026-03-18)
- ✅ **v2.2 Security Remediation** - Phases 45-47 (shipped 2026-03-19)
- ✅ **v2.3 Security Testing** - Phases 48-51 (shipped 2026-03-21)
- ✅ **v2.4 Security Review Remediation** - Phases 52-53 (shipped 2026-03-22)
- ✅ **v2.5 Critical Security Fixes** - Phases 54-56 (shipped 2026-03-23)
- ✅ **v2.6 Security Hardening** - Phases 57-60 (shipped 2026-03-24)
- ✅ **v2.7 CI & Config Security** - Phases 61-63 (shipped 2026-03-25)
- ✅ **v2.8 High Priority Hardening** - Phases 64-67 (shipped 2026-03-26)
- ✅ **v2.9 Security Review P2 Remediation** - Phases 68-70 (shipped 2026-03-26)
- ✅ **v3.0 Release Polish** - Phases 71-74 (shipped 2026-03-27)
- ✅ **v4.0 SBOM Attestation Wedge** - Phases 75-78 (shipped 2026-04-03)
- 📋 **v5.0 Portfolio Polish** - Phases 79-82 (planned)

## Phases

<details>
<summary>✅ v1.0 through v4.0 (Phases 1-78) - SHIPPED</summary>

See `.planning/milestones/` for archived roadmaps and requirements.

**78 phases, 116 plans shipped across 21 milestones.**

</details>

### 📋 v5.0 Portfolio Polish (Planned)

**Milestone Goal:** Make the existing SBOM attestation work visible and discoverable — self-attesting CI releases, a published GitHub Action on the marketplace, a demo GIF embedded in the README, and a product landing page on trustedgelabs.com.

- [x] **Phase 79: Self-Attestation CI** - Wire up end-to-end self-attestation in the CI release workflow and archive te-prove design doc (completed 2026-04-05)
- [x] **Phase 80: GitHub Action Marketplace** - Publish `TrustEdge-Labs/attest-sbom-action@v1` to GitHub Marketplace as a standalone repo (completed 2026-04-05)
- [ ] **Phase 81: Demo GIF** - Record and embed demo GIF showing the full attest-sbom → verify-attestation flow in the README
- [ ] **Phase 82: Product Landing Page** - Ship product landing page on trustedgelabs.com with quick start and verifier link

## Phase Details

### Phase 79: Self-Attestation CI
**Goal**: Every TrustEdge GitHub release automatically attests its own binary, producing a `.te-attestation.json` and `build.pub` as downloadable release assets — with zero stored secrets.
**Depends on**: Phase 78 (v4.0 complete)
**Requirements**: CI-01, CI-02, CI-03, CI-04, HK-01
**Success Criteria** (what must be TRUE):
  1. After a release is published, the GitHub release page shows `.te-attestation.json` and `build.pub` as attached assets
  2. A user can download those assets and independently verify them with `trst verify-attestation` using the attached `build.pub` — no TrustEdge infrastructure needed
  3. The CI job generates a fresh Ed25519 keypair on every run; no signing key is stored in GitHub Secrets or the repository
  4. The CI job uses a pinned `anchore/sbom-action@v0` to generate a CycloneDX SBOM that is bound to the release binary
  5. The te-prove design doc is accessible in `.planning/ideas/` for future reference
**Plans:** 1/1 plans complete

Plans:
- [x] 79-01-PLAN.md — Fix self-attestation CI job and archive te-prove design doc

### Phase 80: GitHub Action Marketplace
**Goal**: Any project can add SBOM attestation to their CI with a single YAML snippet by installing `TrustEdge-Labs/attest-sbom-action@v1` from the GitHub Marketplace.
**Depends on**: Phase 79
**Requirements**: DIST-01, DIST-02, DIST-03, DIST-04
**Success Criteria** (what must be TRUE):
  1. A user can find `TrustEdge-Labs/attest-sbom-action` on the GitHub Marketplace and install it without leaving GitHub
  2. The action downloads the `trst` binary from GitHub Releases and verifies its SHA256 checksum before executing — it does not bundle a binary
  3. A user can configure the action with `sbom-path`, `binary-path`, `key-path`, and `trustedge-version` inputs in their workflow YAML
  4. The action README shows two working usage examples: one using a persistent signing key (stored as a secret) and one using an ephemeral key generated per-run
**Plans:** 1/1 plans complete

Plans:
- [x] 80-01-PLAN.md — Enhance action.yml (SHA256 verification), polish README (two usage examples), create separate repo and tag v1/v1.0.0, submit Marketplace listing

### Phase 81: Demo GIF
**Goal**: A developer landing on the TrustEdge README can immediately see what the product does — attest-sbom to verify-attestation — by watching an embedded GIF, without reading any prose.
**Depends on**: Phase 79
**Requirements**: VIS-01, VIS-03
**Success Criteria** (what must be TRUE):
  1. The README displays an embedded GIF that shows the complete attest-sbom → verify-attestation flow from a real terminal session
  2. The GIF is recorded from `scripts/demo.sh --local` (or the updated demo script) and reflects the actual CLI output — no staged or edited content
  3. The GIF is visible without clicking any links — embedded directly in the README at or near the top
**Plans**: TBD
**UI hint**: yes

### Phase 82: Product Landing Page
**Goal**: A recruiter or prospective user visiting trustedgelabs.com immediately understands what TrustEdge does, can run the quick start, and can reach the live verifier.
**Depends on**: Phase 81
**Requirements**: VIS-02
**Success Criteria** (what must be TRUE):
  1. A visitor to trustedgelabs.com can read a clear one-paragraph explanation of what TrustEdge does and who it is for
  2. The page includes a copy-pasteable quick start showing how to install `trst` and run `attest-sbom` in three or fewer commands
  3. The page links directly to the live public verifier so a visitor can verify an attestation without leaving the page context
  4. The page links to the GitHub Action marketplace listing so a visitor can add attestation to their own CI immediately
**Plans**: TBD
**UI hint**: yes

## Progress

| Phase | Milestone | Plans Complete | Status | Completed |
|-------|-----------|----------------|--------|-----------|
| 75. Core Attestation Library | v4.0 | 1/1 | Complete | 2026-04-02 |
| 76. CLI + Platform Endpoint | v4.0 | 2/2 | Complete | 2026-04-02 |
| 77. Verify Page + Deployment + Demo | v4.0 | 3/3 | Complete | 2026-04-03 |
| 78. Distribution | v4.0 | 2/2 | Complete | 2026-04-03 |
| 79. Self-Attestation CI | v5.0 | 1/1 | Complete   | 2026-04-05 |
| 80. GitHub Action Marketplace | v5.0 | 1/1 | Complete   | 2026-04-05 |
| 81. Demo GIF | v5.0 | 0/? | Not started | - |
| 82. Product Landing Page | v5.0 | 0/? | Not started | - |
