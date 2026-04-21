# Phase 87 Plan 01 — Straggler URL Cleanup SUMMARY

**Date:** 2026-04-21
**Phase:** 87-github-repository-rename
**Plan:** 01 — Straggler URL cleanup commit + grep audit + push (pre-rename)
**Status:** Complete
**Primary commit:** `301d989` (`fix(87): point last tracked URL refs at /TrustEdge-Labs/sealedge`)
**Aux commits:** `9980fa1` (cargo fmt pre-existing regression), `f4a02ea` (rustls-webpki CVE bump)
**Pushed to:** `origin/main` (still `TrustEdge-Labs/trustedge` — rename is Plan 02)

---

## What shipped

Six literal URL substitutions across 3 tracked files:

```
 .github/ISSUE_TEMPLATE/config.yml | 6 +++---
 .github/workflows/cla.yml         | 4 ++--
 deploy/digitalocean/app.yaml      | 2 +-
 3 files changed, 6 insertions(+), 6 deletions(-)
```

| File | Line(s) | Change |
|------|---------|--------|
| `.github/ISSUE_TEMPLATE/config.yml` | 8, 11, 14 | `TrustEdge-Labs/trustedge` → `TrustEdge-Labs/sealedge` (security advisories, discussions, README blob URLs) |
| `.github/workflows/cla.yml` | 30, 49 | `TrustEdge-Labs/trustedge` → `TrustEdge-Labs/sealedge` (CLA Assistant `path-to-document` + prose) |
| `deploy/digitalocean/app.yaml` | 18 | `repo: TrustEdge-Labs/trustedge` → `repo: TrustEdge-Labs/sealedge` (DO App Platform source field; indentation preserved) |

**NOT modified** (per D-01 scope + D-02 hybrid treatment + D-06a carve-out):
- `CHANGELOG.md`, `MIGRATION.md` — hybrid historical treatment (Phase 86 D-02/D-03)
- `improvement-plan.md`, `RFC_K256_SUPPORT.md`, `security-review-platform.md` — historical artifacts (Phase 86 D-01a allowlist)
- `actions/attest-sbom-action/**` — Phase 88 replaces the whole Action
- `deploy/digitalocean/app.yaml:12` `name: trustedge-verifier` — DO's internal app-name field, not a repo URL (explicitly preserved per D-11; DO-app rename is out of Phase 87 scope)
- Copyright headers in the 3 files (`Project: trustedge — Privacy and trust at the edge.`) — Phase 85 owned copyright-header sweeps; the 3 files were not in Phase 85's scope and updating them now would expand Phase 87 beyond D-01

---

## D-13 §3 verbatim allowlisted grep audit

Command run (per 87-CONTEXT.md §Specifics lines 162-165):

```
git ls-files | xargs grep -n "TrustEdge-Labs/trustedge" 2>/dev/null \
  | grep -vE '^\.planning/|^\.factory/|actions/attest-sbom-action/|improvement-plan\.md|RFC_K256_SUPPORT\.md|security-review-platform\.md|CHANGELOG\.md|MIGRATION\.md'
```

**Output:** (empty — zero lines) ✓

**Result:** D-13 success criterion #3 met. Every remaining `TrustEdge-Labs/trustedge` reference in the repo is either in the archived `.planning/` milestone history, the Phase-88-owned `actions/attest-sbom-action/**` surface, or one of the 5 explicitly allowlisted historical `.md` artifacts.

Sanity-check (`git ls-files | xargs grep -l "TrustEdge-Labs/trustedge"` without the allowlist filter) returns only allowlisted paths, confirming no hits outside the expected surface.

---

## Pre-push hook interactions

The project's pre-push hook (`cargo fmt --check`, `cargo audit`, dashboard build) blocked the initial push twice. Both blockers were unrelated to the Phase 87 URL substitutions; both fixed in separate scoped commits.

### Blocker 1: cargo fmt regression (commit `9980fa1`)

`crates/platform/tests/verify_integration.rs:1548` had drifted out of rustfmt compliance in a prior-phase commit. Single-line rewrap, no semantic change, scoped to the one file. Commit message: `chore: cargo fmt pre-existing regression on verify_integration.rs`.

### Blocker 2: rustls-webpki CVEs (commit `f4a02ea`)

Two advisories published 2026-04-14 flag `rustls-webpki 0.103.10`:
- **RUSTSEC-2026-0098** — name constraints for URI names were incorrectly accepted
- **RUSTSEC-2026-0099** — name constraints were accepted for certificates asserting a wildcard name

Fix: `cargo update -p rustls-webpki --precise 0.103.12`. Only Cargo.lock changed (2 insertions, 2 deletions). `cargo audit` post-bump reports zero errors (only informational unmaintained-crate warnings for bincode/derivative/instant — already long-term-accepted). `cargo check --workspace` green post-bump. Commit message: `chore(security): bump rustls-webpki 0.103.10 → 0.103.12 (RUSTSEC-2026-0098, -0099)`.

Both blockers routed through user confirmation before each separate commit was made (no hook bypass, no scope creep into the primary Phase 87 commit).

---

## Acceptance criteria verification

| Criterion | Status |
|-----------|--------|
| `grep -c "TrustEdge-Labs/trustedge" .github/ISSUE_TEMPLATE/config.yml` = 0 | ✓ |
| `grep -c "TrustEdge-Labs/sealedge" .github/ISSUE_TEMPLATE/config.yml` = 3 | ✓ |
| `grep -c "TrustEdge-Labs/trustedge" .github/workflows/cla.yml` = 0 | ✓ |
| `grep -c "TrustEdge-Labs/sealedge" .github/workflows/cla.yml` = 2 | ✓ |
| `grep -c "TrustEdge-Labs/trustedge" deploy/digitalocean/app.yaml` = 0 | ✓ |
| `grep -c "TrustEdge-Labs/sealedge" deploy/digitalocean/app.yaml` = 1 | ✓ |
| `grep -n "repo:" deploy/digitalocean/app.yaml` shows `18:      repo: TrustEdge-Labs/sealedge` (6-space indent) | ✓ |
| `git diff --stat` on straggler commit shows `3 files changed, 6 insertions(+), 6 deletions(-)` | ✓ |
| D-13 §3 verbatim audit returns zero lines | ✓ |
| Commit subject exactly `fix(87): point last tracked URL refs at /TrustEdge-Labs/sealedge` | ✓ |
| `git rev-parse HEAD` == `git rev-parse origin/main` | ✓ (`f4a02ea`) |
| CHANGELOG.md and MIGRATION.md NOT modified | ✓ |
| No files outside 3-file scope in the primary Phase 87 commit | ✓ (aux commits are separately-scoped chore fixes) |

---

## Current repo state post-push

- **Local HEAD:** `f4a02ea`
- **Remote `origin/main`:** `f4a02ea` (still pointing at `TrustEdge-Labs/trustedge.git` — rename is Plan 02)
- **Commit sequence on main (most recent first):**
  ```
  f4a02ea chore(security): bump rustls-webpki 0.103.10 → 0.103.12 (RUSTSEC-2026-0098, -0099)
  9980fa1 chore: cargo fmt pre-existing regression on verify_integration.rs
  301d989 fix(87): point last tracked URL refs at /TrustEdge-Labs/sealedge
  65f63d2 docs(87): create phase 87 plans — GitHub repository rename (2 plans)
  b0800b9 docs(state): record phase 87 context session
  ```
- Working tree has the pre-existing .claude/, web/demo/, web/verify/, experimental/pubky dirty state that was present at phase start — none of those files are in Phase 87 scope.

---

## Pointer forward — Plan 02

Per D-04 / D-07 step 1-2, the straggler commit had to land + push BEFORE the GitHub rename operation. That precondition is now met.

**Next:** Plan 02 executes the 4-check pre-rename gate (D-06), you run `gh repo rename sealedge -R TrustEdge-Labs/trustedge`, I update the local `origin` URL, then the 4-check verification gate (D-13 all 4 checks) and 87-VERIFICATION.md evidence capture.

Rollback if anything goes wrong: `gh repo rename trustedge -R TrustEdge-Labs/sealedge` + `git remote set-url origin https://github.com/TrustEdge-Labs/trustedge.git` (verbatim in Plan 02).

---

*Phase: 87-github-repository-rename*
*Plan: 01 — straggler URL cleanup*
*Completed: 2026-04-21*
