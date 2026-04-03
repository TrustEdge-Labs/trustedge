<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->

# Requirements: TrustEdge v4.0

**Defined:** 2026-04-01
**Core Value:** Prove that data from an edge device has not been tampered with — from capture to verification — using cryptographic signatures, continuity chains, and verifiable receipts.

## v4.0 Requirements

Requirements for SBOM Attestation Wedge milestone. Each maps to roadmap phases.

### Attestation Format

- [x] **ATTEST-01**: AttestationDocument struct with Ed25519 signing, BLAKE3 hashing, random nonce, and timestamp
- [x] **ATTEST-02**: Signature verification of attestation documents (validates signature, optionally checks hashes against provided files)
- [x] **ATTEST-03**: Canonical JSON serialization for deterministic signing (signature excluded from canonicalized payload)

### CLI

- [ ] **CLI-01**: User can run `trst attest-sbom` to bind a CycloneDX JSON SBOM to a binary artifact, producing a `.te-attestation.json` file
- [ ] **CLI-02**: User can run `trst verify-attestation` to verify an attestation document locally, with optional binary/SBOM hash checking
- [ ] **CLI-03**: CLI rejects 0-byte binaries, non-JSON SBOMs, and binaries >256MB with clear error messages

### Platform

- [x] **PLAT-01**: Platform exposes `POST /v1/verify-attestation` endpoint that verifies point attestations and returns a JWS receipt
- [ ] **PLAT-02**: Static HTML verification page accepts attestation upload, displays receipt with SBOM contents, binary hash, key, timestamp, and handles network errors/timeouts
- [ ] **PLAT-03**: Verification page supports third-party verification via the attestation's embedded public key (self-contained attestation documents are independently verifiable without separate key upload)

### Distribution

- [ ] **DIST-01**: Platform server deployed to a public URL (in-memory backend, rate limited, ephemeral receipts)
- [x] **DIST-02**: Demo script runs end-to-end in under 60 seconds (keygen → attest-sbom → verify)
- [ ] **DIST-03**: Product landing page on trustedgelabs-website with quick start, verifier link, and GitHub Action link
- [ ] **DIST-04**: TrustEdge attests its own release builds in CI, with .te-attestation.json in GitHub Releases
- [ ] **DIST-05**: Third-party attestation demo prepared for prospect conversations

### CI Integration

- [ ] **CI-01**: GitHub Action `trustedge/attest-sbom-action@v1` runs `trst attest-sbom` on release builds with one-line YAML integration

## Future Requirements

Deferred to future milestones. Tracked but not in current roadmap.

### Format Extensions

- **FMT-01**: SPDX SBOM format support alongside CycloneDX
- **FMT-02**: SBOM schema validation (verify CycloneDX/SPDX structure, not just JSON)
- **FMT-03**: SBOM/binary mismatch detection (SBOM lists packages not in binary)

### Media Provenance

- **MEDIA-01**: C2PA sidecar manifest output for cam.video profile
- **MEDIA-02**: C2PA sidecar manifest output for audio profile
- **MEDIA-03**: First amendment auditor use case (prove what was said and when)

### Platform Enhancements

- **PLAT-04**: SQLite persistence for receipts (survives restarts)
- **PLAT-05**: Verification badge endpoint (Shields.io-style SVG)
- **PLAT-06**: SBOM diff/drift detection between attested versions

## Out of Scope

| Feature | Reason |
|---------|--------|
| PostgreSQL/multi-tenant for public verifier | Demo stage, in-memory sufficient |
| Auth/user accounts on public verifier | Open verification is the value proposition |
| Container image signing (Sigstore overlap) | Focus on firmware/non-container binaries |
| GitLab CI component | Defer until GitHub Action validated |
| WASM browser verification for attestation | Server-side verification sufficient for Phase 1 |
| TPM support | No hardware to test, premature |
| Post-quantum cryptography | No production use case yet |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| ATTEST-01 | Phase 75 | Complete |
| ATTEST-02 | Phase 75 | Complete |
| ATTEST-03 | Phase 75 | Complete |
| CLI-01 | Phase 76 | Pending |
| CLI-02 | Phase 76 | Pending |
| CLI-03 | Phase 76 | Pending |
| PLAT-01 | Phase 76 | Complete |
| PLAT-02 | Phase 77 | Pending |
| PLAT-03 | Phase 77 | Pending |
| DIST-01 | Phase 77 | Pending |
| DIST-02 | Phase 77 | Complete |
| DIST-03 | Phase 78 | Pending |
| DIST-04 | Phase 78 | Pending |
| DIST-05 | Phase 78 | Pending |
| CI-01 | Phase 78 | Pending |

**Coverage:**
- v4.0 requirements: 15 total
- Mapped to phases: 15
- Unmapped: 0 ✓

---
*Requirements defined: 2026-04-01*
*Last updated: 2026-04-01 after roadmap creation*
