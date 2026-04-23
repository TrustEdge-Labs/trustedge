---
phase: 89-final-validation
plan: 03
subsystem: validation
tags: [validation, release, tag-cut, milestone-close, sealedge, rebrand, ci, self-attestation]

requires:
  - phase: 89-final-validation
    plan: 02
    provides: "89-VERIFICATION.md §2.1/2.2 drafted; wasm-tests + semver dispatch runs green"

provides:
  - "RELEASE-NOTES-v6.0.0.md: phase-local release-notes body committed before tag cut"
  - "v6.0.0 annotated tag pushed to TrustEdge-Labs/sealedge"
  - "GitHub release v6.0.0 published with --latest flag"
  - "Tag-push CI run 24817620668: conclusion success (all 4 jobs including self-attestation)"
  - "self-attestation job: sealedge-attest-sbom-action@v2 dogfood verified end-to-end"
  - "Release assets: seal, seal.sha256, seal-sbom.cdx.json, seal.se-attestation.json, ephemeral.pub"
  - "89-VERIFICATION.md §2.3 populated; Status flipped to PASS; §5 criterion #3 PASS"
  - "VALID-02 closed (final evidence gap)"

affects:
  - 89-04
  - v6.0-milestone-close

tech-stack:
  added: []
  patterns:
    - "B-3 commit ordering: RELEASE-NOTES committed pre-tag, VERIFICATION.md updated post-tag (separate atomic commits)"
    - "Option A force-update: two inline fix commits, tag force-updated twice, documented in §4"
    - "syft file: syntax for binary SBOM generation (replaces anchore/sbom-action path: parameter)"

key-files:
  created:
    - ".planning/phases/89-final-validation/RELEASE-NOTES-v6.0.0.md"
    - ".planning/phases/89-final-validation/89-03-SUMMARY.md"
  modified:
    - ".planning/phases/89-final-validation/89-VERIFICATION.md"
    - ".github/workflows/ci.yml"

key-decisions:
  - "Option A force-update chosen over v6.0.1: two self-attestation bugs fixed inline, tag force-updated twice, no production consumers to disrupt"
  - "syft file: syntax: replaced anchore/sbom-action path: parameter (directory-only) with direct syft CLI using file: prefix for binary scanning"
  - "ephemeral.pub naming: sealedge-attest-sbom-action@v2 generates ephemeral keypair named ephemeral.pub when no key: input provided; serves D-15 build.pub role"

metrics:
  duration: ~3h (including three CI run cycles + pre-push hook overhead per run)
  tasks_completed: 3 / 3 (Task 1: RELEASE-NOTES + commit; Task 2: tag+push+release+CI; Task 3: VERIFICATION.md)
  files_touched: 4
  commits: 5 (ab0c98a RELEASE-NOTES, f7e747b syft fix, 9ae2da9 upload fix, 5f0e59c VERIFICATION.md, plus this SUMMARY)
  completed_date: 2026-04-23

requirements-completed:
  - VALID-02
---

# Phase 89 Plan 03: v6.0.0 Tag Cut + Release + VALID-02 Close Summary

**v6.0.0 cut and released on TrustEdge-Labs/sealedge; tag-push CI green across all 4 jobs including sealedge-attest-sbom-action@v2 self-attestation dogfood; VALID-02 §2.3 evidence captured; milestone close ready for Plan 04**

## Performance

- **Duration:** ~3 hours (three CI run cycles; each tag-push triggers ~20 min CI + pre-push hook overhead)
- **Started:** 2026-04-22
- **Completed:** 2026-04-23
- **Tasks:** 3 (Task 1: RELEASE-NOTES; Task 2: tag+push+release+watch; Task 3: VERIFICATION.md)
- **Files modified:** 4

## Accomplishments

- Created `.planning/phases/89-final-validation/RELEASE-NOTES-v6.0.0.md` covering 7 breaking-change areas + MIGRATION.md link; all 10 MIGRATION.md keyword coverage checks passed
- Ran D-06 6-point pre-tag gate checklist (all 6 green, including Gate 5 confirmed no post-Plan-01 regressions)
- Created annotated tag `v6.0.0` on commit `ab0c98a`, pushed to origin (2026-04-22T23:39:02Z)
- Created GitHub release at https://github.com/TrustEdge-Labs/sealedge/releases/tag/v6.0.0
- Resolved two D-14 hybrid-gate bugs in ci.yml self-attestation job via Option A force-update (see Deviations)
- Final tag-push CI run 24817620668: conclusion success; self-attestation job all steps green
- sealedge-attest-sbom-action@v2 dogfood verified: binary download + SHA256 check + ephemeral keygen + SBOM attestation + upload all succeeded
- Release assets confirmed: 5/5 D-15 expected asset categories present
- Populated 89-VERIFICATION.md §2.3 with tag/release/CI evidence; flipped Status to PASS; §5 criterion #3 from PARTIAL to PASS

## Pre-Tag Gate Checklist Outcomes

| Gate | Check | Outcome |
|------|-------|---------|
| 1 | Working tree clean + main pushed | PASS — RELEASE-NOTES committed in Task 1 per B-3 |
| 2 | validate-v6.sh exit 0 (Plan 02 Task 1) | PASS — 1052 tests, 8/8 gates |
| 3 | Latest main CI run green | PASS — run 24724694867 (success) |
| 4 | wasm-tests.yml + semver.yml dispatch green | PASS — runs 24786501926 + 24786499197 |
| 5 | D-10 straggler grep: no post-Plan-01 regressions | PASS — 0 new product-name regressions in crates/deploy/scripts/web/.github since Plan 01 |
| 6 | No open PRs | PASS (1 unrelated Dependabot PR: rand 0.9.2→0.9.3) |

## Tag Cut + Release Timeline

| Event | Time | Detail |
|-------|------|--------|
| Task 1 commit (RELEASE-NOTES) | 2026-04-22 | `ab0c98a` — B-3 pre-tag Gate 1 commit |
| First tag push | 2026-04-22T23:39:02Z | `git push origin v6.0.0` → triggered CI run 24808314283 |
| GitHub release created | 2026-04-22T23:36:21Z | `gh release create v6.0.0 --notes-file ... --latest` |
| Fix 1: syft file: syntax | 2026-04-22 | `f7e747b` — Rule 1 bug fix for SBOM generation |
| Tag force-update 1 | 2026-04-22 | `dfb07a5 → 0f83bf2` → triggered CI run 24811716778 |
| Fix 2: upload sbom+pub | 2026-04-22 | `9ae2da9` — Rule 2 missing upload step |
| Tag force-update 2 | 2026-04-23 | `0f83bf2 → 88c6cc2` → triggered CI run 24817620668 |
| Final CI run complete | 2026-04-23T05:17:56Z | All 4 jobs success |

## CI Run Summary

| Run ID | Commit | Conclusion | Self-attestation |
|--------|--------|------------|-----------------|
| 24808314283 | `ab0c98a` | success (overall) | FAILURE — syft path: bug |
| 24811716778 | `f7e747b` | success (overall) | success — sbom+pub missing from upload |
| 24817620668 | `9ae2da9` | success | success — all steps green (authoritative) |

**Authoritative run:** https://github.com/TrustEdge-Labs/sealedge/actions/runs/24817620668

## Release Assets

| Asset | Size | Status |
|-------|------|--------|
| `seal` | 3,525,760 bytes | D-15 required — present |
| `seal.sha256` | 71 bytes | D-15 required — present |
| `seal-sbom.cdx.json` | 484 bytes | D-15 required — present |
| `seal.se-attestation.json` | 660 bytes | D-15 required — present |
| `ephemeral.pub` | 53 bytes | D-15 build.pub equivalent — present |

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] anchore/sbom-action path: parameter does not accept binary files**
- **Found during:** Task 2, CI run 24808314283 self-attestation job
- **Issue:** `anchore/sbom-action` with `path: ./target/release/seal` passes the binary file as a directory to syft; syft exits 1 with "not a directory source"
- **Fix:** Replaced `uses: anchore/sbom-action` step with `run:` step that installs syft directly and uses `syft scan file:./target/release/seal -o cyclonedx-json > seal-sbom.cdx.json`
- **Files modified:** `.github/workflows/ci.yml`
- **Commit:** `f7e747b`

**2. [Rule 2 - Missing functionality] Upload step only uploaded attestation, not SBOM or public key**
- **Found during:** Task 2, CI run 24811716778 (self-attestation success but only 3/5 assets present)
- **Issue:** The "Upload attestation to release" step only uploaded `${{ steps.attest.outputs.attestation-path }}`; D-15 requires `seal-sbom.cdx.json` and `build.pub` (named `ephemeral.pub` by the action) also be uploaded
- **Fix:** Extended upload command to include `seal-sbom.cdx.json` and `${SEAL_PUB}` (the ephemeral public key path from the action)
- **Files modified:** `.github/workflows/ci.yml`
- **Commit:** `9ae2da9`

**3. [Option A - Force-update] Tag force-updated twice per CONTEXT.md D-06 recovery**
- Both fixes applied as `fix(89):` commits; tag force-updated to each new HEAD
- Solo-dev context, no production consumers, force-update is acceptable per D-06 / CONTEXT.md §Claude's-Discretion
- Documented in 89-VERIFICATION.md §4 with full audit trail

### B-3 Commit Structure: Satisfied

Three distinct atomic commits (excluding fix commits):
1. `ab0c98a` — `docs(89): add RELEASE-NOTES-v6.0.0.md body` (pre-tag Gate 1 commit)
2. External: `git tag -a v6.0.0` + `git push origin v6.0.0` (no Claude commit)
3. `5f0e59c` — `docs(89): close VALID-02 §2.3 — v6.0.0 tag cut + self-attestation dogfood green` (post-tag-push)

## Self-Check: PASSED

| Check | Result |
|-------|--------|
| RELEASE-NOTES-v6.0.0.md exists, ≥15 lines, has MPL-2.0 header | FOUND (24 lines) |
| RELEASE-NOTES has 7 breaking-change bullets | FOUND (grep -c '^- ': 7) |
| MIGRATION.md coverage: 0 MISSING keywords | PASSED (all 10 keywords present) |
| v6.0.0 tag exists on origin | CONFIRMED (`git ls-remote --tags origin v6.0.0`: `88c6cc2...refs/tags/v6.0.0`) |
| GitHub release exists at /releases/tag/v6.0.0 | CONFIRMED |
| Authoritative CI run 24817620668: conclusion success | CONFIRMED |
| self-attestation job: success | CONFIRMED (all 11 steps green) |
| Release assets: 5/5 present | CONFIRMED (seal, seal.sha256, seal-sbom.cdx.json, seal.se-attestation.json, ephemeral.pub) |
| VERIFICATION.md Status: PASS | CONFIRMED (grep: 1 match) |
| VERIFICATION.md §5 criterion #3: PASS (no PARTIAL) | CONFIRMED (grep PARTIAL: 0) |
| VERIFICATION.md §2.3 has run URL | CONFIRMED (actions/runs/24817620668) |
| Commit `ab0c98a` (RELEASE-NOTES) exists | CONFIRMED |
| Commit `f7e747b` (syft fix) exists | CONFIRMED |
| Commit `9ae2da9` (upload fix) exists | CONFIRMED |
| Commit `5f0e59c` (VERIFICATION.md close) exists | CONFIRMED |
