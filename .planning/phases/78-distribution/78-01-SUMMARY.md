---
phase: 78-distribution
plan: "01"
subsystem: docs
tags: [documentation, landing-page, sbom-attestation, distribution]
dependency_graph:
  requires: []
  provides: [landing-page-content, third-party-attestation-guide]
  affects: [trustedgelabs-website, prospect-conversations]
tech_stack:
  added: []
  patterns: [mpl-2.0-copyright-header, markdown-docs]
key_files:
  created:
    - docs/landing-page.md
    - docs/third-party-attestation-guide.md
  modified: []
key_decisions:
  - "Landing page differentiates on infrastructure independence vs GitHub Attestations (locked to GitHub) and Sigstore (complex PKI)"
  - "Third-party guide presents ephemeral keys as the default CI pattern — no secrets to rotate"
  - "Both documents include MPL-2.0 copyright headers consistent with project convention"
metrics:
  duration: "87 seconds"
  completed: "2026-04-03T14:02:48Z"
  tasks: 2
  files: 2
requirements_met:
  - DIST-03
  - DIST-05
---

# Phase 78 Plan 01: Distribution Documentation Summary

**One-liner:** Product landing page and third-party SBOM attestation guide covering 3-command quick start, GitHub Action, and manual CI workflows.

## What Was Built

Two documentation files in `docs/` targeting DevSecOps engineers evaluating EU CRA compliance tools:

**docs/landing-page.md** — Product landing page content ready to deploy to trustedgelabs-website. Contains:
- Problem-first headline focused on supply chain tamper-evidence
- Differentiation table vs GitHub Attestations and Sigstore/cosign
- 3-command quick start (keygen, attest-sbom, verify-attestation)
- GitHub Action YAML snippet
- Links to public verifier, GitHub Action repo, source repo, and integration guide
- What's-in-an-attestation description

**docs/third-party-attestation-guide.md** — Standalone integration guide for any project. Contains:
- Prerequisites (trst binary, syft)
- Complete manual workflow: keygen through gh release upload (5 steps, all copy-paste)
- CI workflow: GitHub Action one-liner AND manual steps for non-GitHub CI
- Verification section: local CLI, public verifier web UI, direct API
- Attestation field table (format, binary_hash, sbom_hash, sbom_content, signature, nonce, timestamp)
- Encrypted and YubiKey key options for production devices

## Deviations from Plan

None — plan executed exactly as written.

## Known Stubs

None — both documents contain complete, accurate content derived from the actual CLI commands in `scripts/demo-attestation.sh` and `crates/trst-cli/src/main.rs`.

## Self-Check: PASSED

- [x] docs/landing-page.md exists and contains all required sections
- [x] docs/third-party-attestation-guide.md exists with manual and CI workflows
- [x] Commits 6ffd09c and 4d47080 verified in git log
- [x] All automated verification checks passed
