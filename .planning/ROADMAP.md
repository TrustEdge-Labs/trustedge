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
- 📋 **v4.0 SBOM Attestation Wedge** - Phases 75-78 (planned)

## Phases

<details>
<summary>✅ v1.0 through v3.0 (Phases 1-74) - SHIPPED</summary>

See `.planning/milestones/` for archived roadmaps and requirements.

**74 phases, 108 plans shipped across 20 milestones.**

</details>

### 📋 v4.0 SBOM Attestation Wedge (Planned)

**Milestone Goal:** Deliver a lightweight, infrastructure-independent SBOM attestation capability — from core crypto format through CLI, platform endpoint, public verifier, and GitHub Action — so teams can prove software bill of materials integrity independent of their CI provider.

- [x] **Phase 75: Core Attestation Library** - PointAttestation struct, signing, verification, and canonical serialization (completed 2026-04-02)
- [x] **Phase 76: CLI + Platform Endpoint** - `trst attest-sbom`, `trst verify-attestation`, and `POST /v1/verify-attestation` (completed 2026-04-02)
- [ ] **Phase 77: Verify Page + Deployment + Demo** - Static HTML verifier, public deployment, and demo script
- [ ] **Phase 78: Distribution** - Landing page, self-attestation in CI, third-party demo, GitHub Action

## Phase Details

### Phase 75: Core Attestation Library
**Goal**: The PointAttestation type exists in trustedge-core with correct cryptographic properties — deterministic canonical serialization, Ed25519 signing, BLAKE3 hashing, random nonce, and timestamp — enabling everything else to be built on top of it.
**Depends on**: Phase 74 (v3.0 complete)
**Requirements**: ATTEST-01, ATTEST-02, ATTEST-03
**Success Criteria** (what must be TRUE):
  1. A developer can construct a PointAttestation from a binary path and SBOM path and sign it with an Ed25519 key, producing a `.te-attestation.json` file
  2. A developer can verify a `.te-attestation.json` file's signature using only the public key and get a clear pass/fail result
  3. Canonical JSON serialization is deterministic: signing the same inputs twice produces the same bytes to sign (signature excluded from payload)
  4. Verification optionally accepts the original binary and SBOM files and independently checks their BLAKE3 hashes match the attestation document
**Plans:** 1/1 plans complete
Plans:
- [x] 75-01-PLAN.md -- PointAttestation types, signing, verification, canonical serialization, and tests

### Phase 76: CLI + Platform Endpoint
**Goal**: Users can create and verify attestations end-to-end from the command line, and the platform server exposes a network verification endpoint that returns a JWS receipt.
**Depends on**: Phase 75
**Requirements**: CLI-01, CLI-02, CLI-03, PLAT-01
**Success Criteria** (what must be TRUE):
  1. User can run `trst attest-sbom --binary <path> --sbom <path> --device-key <key>` and receive a `.te-attestation.json` output file
  2. User can run `trst verify-attestation <attestation> --device-pub <pub>` and see a clear verified/failed result with relevant details (key, timestamp, hashes)
  3. Running `trst attest-sbom` against a 0-byte binary, a non-JSON SBOM, or a binary over 256 MB exits with a clear error message and non-zero exit code
  4. `POST /v1/verify-attestation` accepts an attestation document and returns a signed JWS receipt on success, or a structured error on failure
**Plans:** 2/2 plans complete
Plans:
- [x] 76-01-PLAN.md — CLI attest-sbom and verify-attestation subcommands with acceptance tests
- [x] 76-02-PLAN.md — Platform POST /v1/verify-attestation endpoint with integration tests

### Phase 77: Verify Page + Deployment + Demo
**Goal**: Anyone with a `.te-attestation.json` file can verify it in a browser via a public URL, and the full flow from keygen to verified receipt runs in under 60 seconds via a demo script.
**Depends on**: Phase 76
**Requirements**: PLAT-02, PLAT-03, DIST-01, DIST-02
**Success Criteria** (what must be TRUE):
  1. A user can visit the public verifier URL, upload a `.te-attestation.json` file, and see a receipt displaying the binary hash, SBOM contents, signing key, and timestamp
  2. The verify page handles network errors and timeouts with user-visible error messages (no silent failures or blank states)
  3. A user can upload an attestation plus a binary and a public key to the verify page for independent third-party verification without relying on a stored key
  4. Running `./scripts/demo-attestation.sh` completes keygen → attest-sbom → verify end-to-end in under 60 seconds with no manual steps
**Plans:** 3 plans
Plans:
- [ ] 77-01-PLAN.md — Static HTML verify page + platform route serving at GET /verify
- [ ] 77-02-PLAN.md — DigitalOcean App Platform deployment configuration
- [ ] 77-03-PLAN.md — End-to-end SBOM attestation demo script

### Phase 78: Distribution
**Goal**: TrustEdge's SBOM attestation capability is publicly reachable and self-demonstrating — the product landing page links to the verifier and GitHub Action, TrustEdge attests its own release builds, and a ready-to-use GitHub Action exists for one-line CI integration.
**Depends on**: Phase 77
**Requirements**: DIST-03, DIST-04, DIST-05, CI-01
**Success Criteria** (what must be TRUE):
  1. The trustedgelabs-website landing page has a quick start section, a link to the public verifier, and a link to the GitHub Action
  2. TrustEdge GitHub Releases include a `.te-attestation.json` alongside the release binary, generated automatically in CI
  3. A prospect can independently verify the TrustEdge self-attestation using only the public verifier URL and the release artifact — no TrustEdge infrastructure required
  4. A third-party project can add SBOM attestation to its release workflow with a single YAML snippet using `trustedge/attest-sbom-action@v1`
**Plans**: TBD

## Progress

| Phase | Milestone | Plans Complete | Status | Completed |
|-------|-----------|----------------|--------|-----------|
| 75. Core Attestation Library | v4.0 | 1/1 | Complete   | 2026-04-02 |
| 76. CLI + Platform Endpoint | v4.0 | 2/2 | Complete   | 2026-04-02 |
| 77. Verify Page + Deployment + Demo | v4.0 | 0/3 | Planning complete | - |
| 78. Distribution | v4.0 | 0/? | Not started | - |
