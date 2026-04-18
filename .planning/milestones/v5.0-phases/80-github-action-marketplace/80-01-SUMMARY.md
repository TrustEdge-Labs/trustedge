---
phase: 80-github-action-marketplace
plan: 01
subsystem: infra
tags: [github-actions, marketplace, sbom, attestation, sha256, branding]

# Dependency graph
requires: []
provides:
  - "action.yml with SHA256 binary verification and Marketplace branding block"
  - "README.md with two copy-pasteable usage examples (ephemeral and persistent key)"
affects: [github-marketplace-listing]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "SHA256 graceful degradation: verify if checksum file present, warn-and-continue if absent"
    - "Ephemeral vs persistent key pattern documented for CI and device-identity use cases"

key-files:
  created: []
  modified:
    - actions/attest-sbom-action/action.yml
    - actions/attest-sbom-action/README.md

key-decisions:
  - "SHA256 mismatch is a hard failure (exit 1) per T-80-01 threat mitigation"
  - "Missing checksum file is graceful degradation with warning (D-04)"
  - "Branding block uses shield icon + blue color for Marketplace discoverability"
  - "README uses anchore/sbom-action in both examples for consistency"

patterns-established:
  - "BASE_URL pattern: construct URL from version, then derive checksum URL with .sha256 suffix"

requirements-completed: [DIST-01, DIST-02, DIST-03, DIST-04]

# Metrics
duration: 12min
completed: 2026-04-05
---

# Phase 80 Plan 01: GitHub Action Marketplace Summary

**Composite action.yml hardened with SHA256 binary verification (exit 1 on mismatch, warn-and-continue if no checksum), branding added for Marketplace listing, and README rewritten with two copy-pasteable examples (ephemeral key and TRUSTEDGE_KEY secret patterns)**

## Performance

- **Duration:** ~12 min
- **Started:** 2026-04-05T14:50:00Z
- **Completed:** 2026-04-05T15:02:00Z
- **Tasks:** 2 of 3 (Task 3 is a checkpoint awaiting orchestrator)
- **Files modified:** 2

## Accomplishments

- action.yml download step rewritten to verify SHA256 checksum before executing trst
- Hard fail on hash mismatch protects against tampered binary (T-80-01 mitigation)
- Branding block (shield icon, blue) added — required field for GitHub Marketplace listing
- README rewritten with Example 1 (ephemeral key, recommended for CI) and Example 2 (persistent key via GitHub Secret)
- "What you get" section explains local-only attestation output with no network calls (D-07)
- How it works step 1 updated to mention SHA256 verification

## Task Commits

1. **Task 1: Add SHA256 verification to action.yml** - `bb86deb` (feat)
2. **Task 2: Polish README with two named usage examples** - `accca7e` (feat)
3. **Task 3: Create separate repo, push, tag, submit to Marketplace** - CHECKPOINT (pending orchestrator)

## Files Created/Modified

- `actions/attest-sbom-action/action.yml` - Added SHA256 verification step with graceful degradation, added branding block for Marketplace
- `actions/attest-sbom-action/README.md` - Rewrote Usage section with two named examples, added "What you get" section, updated "How it works" step 1

## Decisions Made

- SHA256 mismatch exits with code 1 (hard failure) — correct behavior per T-80-01 threat register
- Missing `trst.sha256` file degrades gracefully with a warning — preserves compatibility with older releases that predate checksum publishing (D-04)
- Both README examples use `anchore/sbom-action@v0` for the SBOM generation step (replacing the syft `run:` step from the old README) — cleaner, consistent with Marketplace ecosystem norms
- Dropped "Upload as workflow artifact" section from old README — covered by persistent key example's upload step

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None.

## Checkpoint: Task 3 Pending

Task 3 requires creating a separate public GitHub repo under the `TrustEdge-Labs` org. This
cannot be executed from the worktree (gh commands work against the calling user's org access
from the main repo context). The orchestrator must run these commands:

```bash
# Step 1: Create the separate repo (public, under TrustEdge-Labs org)
gh repo create TrustEdge-Labs/attest-sbom-action \
  --public \
  --description "Attest a binary artifact with its CycloneDX SBOM using TrustEdge — one YAML line, cryptographic proof." \
  --homepage "https://github.com/TrustEdge-Labs/trustedge"

# Step 2: Clone into a temp directory
WORK_DIR=$(mktemp -d)
gh repo clone TrustEdge-Labs/attest-sbom-action "$WORK_DIR/attest-sbom-action"

# Step 3: Copy files from monorepo (use absolute paths)
MONOREPO=/home/john/vault/projects/github.com/trustedge
cp "$MONOREPO/actions/attest-sbom-action/action.yml" "$WORK_DIR/attest-sbom-action/"
cp "$MONOREPO/actions/attest-sbom-action/README.md"  "$WORK_DIR/attest-sbom-action/"
cp "$MONOREPO/actions/attest-sbom-action/LICENSE"    "$WORK_DIR/attest-sbom-action/"

# Step 4: Initial commit and push
cd "$WORK_DIR/attest-sbom-action"
git add action.yml README.md LICENSE
git commit -m "feat: publish TrustEdge SBOM attestation action v1.0.0"
git push origin main

# Step 5: Tag v1 and v1.0.0
git tag v1.0.0
git tag v1
git push origin v1.0.0
git push origin v1

# Step 6: Confirm tags are live
gh api repos/TrustEdge-Labs/attest-sbom-action/git/refs/tags --jq '.[].ref'
echo "Done. Repo: https://github.com/TrustEdge-Labs/attest-sbom-action"
```

After tags are live, present the user with Marketplace submission instructions:
- Visit: https://github.com/TrustEdge-Labs/attest-sbom-action
- Settings > GitHub Marketplace section (or https://github.com/marketplace/actions/new)
- Accept GitHub Marketplace Developer Agreement if not yet done
- Submit for review (1-2 business days per D-08)
- Note: action works immediately via `uses: TrustEdge-Labs/attest-sbom-action@v1`

## User Setup Required

Task 3 requires the orchestrator to:
1. Run the repo creation and push commands above
2. Present the user with Marketplace submission steps
3. Collect user confirmation ("approved" signal from plan's resume-signal)

## Next Phase Readiness

- action.yml and README.md in monorepo are ready to be copied to the separate repo
- After Task 3 checkpoint completes, the action is immediately usable as `@v1`
- Marketplace listing pending review (D-08)

---
*Phase: 80-github-action-marketplace*
*Completed: 2026-04-05 (Tasks 1-2; Task 3 at checkpoint)*

## Self-Check

### Files exist:
- actions/attest-sbom-action/action.yml: FOUND
- actions/attest-sbom-action/README.md: FOUND

### Commits exist:
- bb86deb (Task 1): FOUND
- accca7e (Task 2): FOUND
