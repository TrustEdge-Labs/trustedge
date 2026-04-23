---
phase: 84-crypto-constants-file-extension
plan: "03"
subsystem: external-assets
tags: [rebrand, attestation-extension, html, github-action, deploy-docs]
one_liner: "Renamed .te-attestation.json to .se-attestation.json across HTML verifier UI, GitHub Action YAML+README, and DigitalOcean deployment README (10 sites, 4 files)"
dependency_graph:
  requires: ["84-01"]
  provides: ["REBRAND-04b-external-assets"]
  affects: ["web/verify/index.html", "actions/attest-sbom-action/action.yml", "actions/attest-sbom-action/README.md", "deploy/digitalocean/README-deploy.md"]
tech_stack:
  added: []
  patterns: ["clean-break rename", "include_str! HTML bundling"]
key_files:
  modified:
    - web/verify/index.html
    - actions/attest-sbom-action/action.yml
    - actions/attest-sbom-action/README.md
    - deploy/digitalocean/README-deploy.md
decisions:
  - "D-04 enforced: no dual-accept; file-input accept attribute (line 127) uses generic .json MIME type and was NOT modified"
  - "Brand-word TrustEdge preserved in h1 title (line 122) and action README prose per Phase 85/86 carve-out"
  - "Binary name trst in action README preserved unchanged per Phase 85/86 prose sweep scope"
metrics:
  duration: "~5 minutes"
  completed: "2026-04-18"
  tasks_completed: 1
  tasks_total: 1
  files_modified: 4
---

# Phase 84 Plan 03: Attestation Extension External Assets Summary

Renamed the `.te-attestation.json` attestation-file extension to `.se-attestation.json` across the monorepo external-facing assets. This is the external-assets portion of REBRAND-04b; the Rust sources and demo script portion is Plan 84-02 (parallel wave).

## Per-File Replacement Count

| File | Sites Changed | Legacy Remaining |
|------|--------------|-----------------|
| `web/verify/index.html` | 2 (lines 123, 126) | 0 |
| `actions/attest-sbom-action/action.yml` | 2 (lines 30, 89) | 0 |
| `actions/attest-sbom-action/README.md` | 5 (lines 71, 89, 99, 106, 117) | 0 |
| `deploy/digitalocean/README-deploy.md` | 1 (line 30) | 0 |
| **Total** | **10** | **0** |

## Confirmation Greps

Zero `.te-attestation.json` remaining in all 4 files (verified after commit):
- `web/verify/index.html`: 0 legacy, 2 new `.se-attestation.json`
- `actions/attest-sbom-action/action.yml`: 0 legacy, 2 new `.se-attestation.json`
- `actions/attest-sbom-action/README.md`: 0 legacy, 5 new `.se-attestation.json`
- `deploy/digitalocean/README-deploy.md`: 0 legacy, 1 new `.se-attestation.json`

## Brand-Word Carve-Out Confirmation

- `grep -c 'TrustEdge' web/verify/index.html` returned **2** (h1 title + subtitle preserved, Phase 86 scope)
- `grep -c 'TrustEdge' actions/attest-sbom-action/README.md` returned **10** (prose brand words preserved, Phase 86 scope)
- Binary name `trst` in action README preserved (Phase 85/86 scope)

## Commit

- **a85a3c0** — `refactor(84-03): rename .te-attestation.json -> .se-attestation.json in external assets`
  - 4 files changed, 10 insertions(+), 10 deletions(-)

## Validation Evidence

| Check | Exit Code | Notes |
|-------|-----------|-------|
| `cargo check --workspace --locked` | 0 | include_str! HTML bundling validated at compile time |
| `cargo fmt --check` | 0 | No formatting issues |
| `cargo clippy --workspace --all-targets -- -D warnings` | 0 | No warnings |
| `cargo test --workspace --locked` | 0 | All tests pass |

## REBRAND-04b Cross-Reference

Plan 84-02 covers the Rust sources (`point_attestation.rs`, `seal-cli/src/main.rs`, `seal-cli/tests/acceptance.rs`) and `scripts/demo-attestation.sh`. Together, Plans 84-02 and 84-03 fully close REBRAND-04b — the `.se-attestation.json` extension is now consistent across all monorepo source-of-truth files.

## Deviations from Plan

None — plan executed exactly as written. All 10 sites matched the enumerated interface spec. The file-input `accept` attribute on line 127 of `web/verify/index.html` correctly uses generic `.json,application/json` with no legacy extension reference, confirming no additional edit was required there.

## Threat Flags

None — edits are pure string-label changes in non-cryptographic files. No new network endpoints, auth paths, or trust boundaries introduced.

## Self-Check: PASSED

- web/verify/index.html exists and contains `.se-attestation.json` (2 occurrences)
- actions/attest-sbom-action/action.yml exists and contains `.se-attestation.json` (2 occurrences)
- actions/attest-sbom-action/README.md exists and contains `.se-attestation.json` (5 occurrences)
- deploy/digitalocean/README-deploy.md exists and contains `.se-attestation.json` (1 occurrence)
- Commit a85a3c0 confirmed in git log
