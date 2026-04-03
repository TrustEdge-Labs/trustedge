---
phase: 78-distribution
plan: 02
subsystem: infra
tags: [github-actions, ci, sbom, attestation, cyclonedx, composite-action, syft]

requires:
  - phase: 78-distribution/78-01
    provides: trst attest-sbom CLI command with --binary/--sbom/--device-key/--device-pub/--out/--unencrypted flags

provides:
  - Composite GitHub Action (actions/attest-sbom-action/) for one-line SBOM attestation in any CI pipeline
  - Self-attestation CI job in .github/workflows/ci.yml that produces .te-attestation.json on v* tag releases

affects: [release-pipeline, third-party-adoption, ci]

tech-stack:
  added: [syft (SBOM generator, installed at runtime in CI), gh cli (release asset upload)]
  patterns:
    - Composite GitHub Action with conditional ephemeral-key generation
    - Release-gated CI job (startsWith(github.ref, 'refs/tags/v'))

key-files:
  created:
    - actions/attest-sbom-action/action.yml
    - actions/attest-sbom-action/README.md
    - actions/attest-sbom-action/LICENSE
  modified:
    - .github/workflows/ci.yml

key-decisions:
  - "Ephemeral Ed25519 keypair per release — no stored signing secrets in GitHub"
  - "Action downloads trst from GitHub Releases (not Docker) matching D-11"
  - "self-attestation job depends on build-and-test so attestation only runs after tests pass"

patterns-established:
  - "Composite action: download binary at runtime, generate ephemeral key if none provided, produce output path"

requirements-completed: [CI-01, DIST-04]

duration: 12min
completed: 2026-04-01
---

# Phase 78 Plan 02: GitHub Action and CI Self-Attestation Summary

**Composite GitHub Action for one-line SBOM attestation plus TrustEdge CI self-attestation on every v* release tag**

## Performance

- **Duration:** 12 min
- **Started:** 2026-04-01T20:01:37Z
- **Completed:** 2026-04-01T20:13:00Z
- **Tasks:** 2
- **Files modified:** 4

## Accomplishments

- Created `actions/attest-sbom-action/` composite action: downloads trst binary, generates ephemeral Ed25519 keypair when no key provided, runs `attest-sbom`, outputs attestation path
- Added `self-attestation` job to CI that triggers on `refs/tags/v*`, depends on `build-and-test`, builds trst, installs syft, generates SBOM and attestation, uploads `.te-attestation.json` as release asset
- Full MPL-2.0 license and usage documentation for the action

## Task Commits

1. **Task 1: Create composite GitHub Action for SBOM attestation** - `f3e66fc` (feat)
2. **Task 2: Add self-attestation step to CI release workflow** - `cf4ee4e` (feat)

**Plan metadata:** (to be added after state update)

## Files Created/Modified

- `actions/attest-sbom-action/action.yml` - Composite action: download trst, keygen if no key, attest-sbom
- `actions/attest-sbom-action/README.md` - Usage examples, inputs/outputs table, verification instructions
- `actions/attest-sbom-action/LICENSE` - Full MPL-2.0 text
- `.github/workflows/ci.yml` - Added `self-attestation` job (47 lines) gated on v* tags

## Decisions Made

- Ephemeral keypair approach: generates a fresh Ed25519 key per release so no persistent signing keys are stored in GitHub Secrets. Attestation is still verifiable via the embedded public key in the attestation JSON.
- Action uses `runner.temp` for all generated files to avoid workspace pollution.
- `self-attestation` job has `permissions: contents: write` at the job level (not workflow level) to follow least-privilege pattern.
- Existing jobs (lint, build-and-test, security) left completely unchanged.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required. The CI job uses `GITHUB_TOKEN` (automatically provided by GitHub Actions) for release asset uploads.

## Next Phase Readiness

- Composite action is ready for the user to push to `TrustEdge-Labs/attest-sbom-action` repository for marketplace listing (out of scope for this plan per verification criteria)
- Self-attestation will run automatically on the next `git tag v*` push and will upload `trst.te-attestation.json` to the GitHub release

---
*Phase: 78-distribution*
*Completed: 2026-04-01*
