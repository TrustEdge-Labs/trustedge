<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
-->

# Phase 87 Verification — GitHub Repository Rename

**Phase:** 87-github-repository-rename
**Requirement:** EXT-01
**Date:** 2026-04-21
**Status:** PASS — all 3 ROADMAP success criteria and all 4 D-13 verification checks satisfied. Rollback not executed.

---

## D-06 Pre-Rename Gate

All 4 gate checks verified green before `gh repo rename` was run.

| # | Check | Evidence |
|---|-------|----------|
| 1 | Working tree clean + `main` pushed | `HEAD == origin/main == d1965bc`; `git log origin/main..HEAD` empty |
| 2 | Latest CI on main is green | Workflow `CI` run `24721623300` on `f4a02ea` (direct ancestor of `d1965bc`): `completed`/`success`; CodeQL run `24722894758` on `d1965bc` itself: `completed`/`success` |
| 3 | No open PRs | `gh pr list --state open` → `[]` |
| 4 | No in-progress workflows | `gh run list --status in_progress` → `[]` (after waiting for CodeQL on `d1965bc` to complete) |

**Note:** This project's `ci.yml` only triggers on `pull_request`, tag push (`v*`), and `workflow_dispatch` — not on regular main pushes. D-06 criterion 2 was satisfied by manually triggering `ci.yml` via `workflow_dispatch` against `f4a02ea` (the HEAD after the Phase 01 straggler commit + the cargo fmt + rustls-webpki CVE fixes). The `d1965bc` docs-only commit on top was accepted as a code-state no-op on that verified base.

---

## D-05 Rename Operation

Command executed by user from their shell:

```
gh repo rename sealedge -R TrustEdge-Labs/trustedge
```

**Output:**

```
✓ Renamed repository TrustEdge-Labs/sealedge
```

**Timestamp:** 2026-04-21 (immediately following the D-06 gate pass)

**Rollback command (NOT executed — documented verbatim per D-15 for future reference):**

```
gh repo rename trustedge -R TrustEdge-Labs/sealedge
git remote set-url origin https://github.com/TrustEdge-Labs/trustedge.git
```

**Incident during rename:** User initially pasted the rollback command (`gh repo rename trustedge -R TrustEdge-Labs/sealedge`) before the rename had occurred — GitHub returned `HTTP 404: Not Found (https://api.github.com/repos/TrustEdge-Labs/sealedge)` since `/sealedge` did not yet exist. No state change; user re-ran with the correct rename command and it succeeded. The rollback-vs-rename command pair is symmetric by design; ordering confusion is a real human-factors risk and is captured here for future reference.

---

## D-13 §1 — GitHub 301 Redirect

**Command (CONTEXT.md §D-13 §1 verbatim):**

```
curl -I -s -o /dev/null -w '%{http_code} -> %{redirect_url}\n' https://github.com/TrustEdge-Labs/trustedge
```

**Output:**

```
301 -> https://github.com/TrustEdge-Labs/sealedge
```

**Follow-redirect cross-check:**

```
curl -s -o /dev/null -w '%{http_code} (final: %{url_effective})\n' -L https://github.com/TrustEdge-Labs/trustedge
```

**Output:**

```
200 (final: https://github.com/TrustEdge-Labs/sealedge)
```

No 30-second retry buffer needed — redirect was in place on the first call. ROADMAP §Phase 87 success criterion #1 satisfied.

---

## D-13 §2 — Local Remote + Fetch/Push

**Before (pre-update):** `https://github.com/TrustEdge-Labs/trustedge.git`
**After (post-update):** `https://github.com/TrustEdge-Labs/sealedge.git`

**Command:**

```
git remote set-url origin https://github.com/TrustEdge-Labs/sealedge.git
git remote get-url origin
```

**`git fetch origin`:** success (no errors; picked up a new dependabot branch `dependabot/cargo/cargo-39cecc6af4` that had landed on the renamed remote)

**`git ls-remote origin HEAD`:** `d1965bc03c0056b3d75203fbccb400a6433ab2df	HEAD`

**`git ls-remote origin | head -5`:**

```
d1965bc03c0056b3d75203fbccb400a6433ab2df	HEAD
2e5054087c7e9a45df9a551aa382411d4ddc0aab	refs/heads/admin/fixdco-signoff
eda956778eb68b148652c02011a60707e491bb5c	refs/heads/cla-signatures
a295f29c3830bf61df6946c89f400aa8df81b6d1	refs/heads/dependabot/cargo/cargo-3169503097
cd012d97b09360f39af6f8337de55de8c03ca0d6	refs/heads/dependabot/cargo/cargo-39cecc6af4
```

**Verification-trigger push (Task 5a):** `git push origin main` landed `d0968b8` on `sealedge.git`. Push output included `To https://github.com/TrustEdge-Labs/sealedge.git / d1965bc..d0968b8  main -> main` — explicit confirmation that the push went to the renamed remote.

HTTPS retained per D-08. ROADMAP §Phase 87 success criterion #2 satisfied.

---

## D-13 §3 — CI Green on Renamed Repo

**Verification commit SHA:** `d0968b8e66b00c015b9d524949c674daa3c03aad` (empty commit, `chore(87): post-rename CI + DO verification trigger`)

**CI run triggered:** Manual `workflow_dispatch` on `ci.yml` against `main` (since `ci.yml` doesn't auto-fire on main pushes per this project's workflow config)
**Run URL:** https://github.com/TrustEdge-Labs/sealedge/actions/runs/24724694867
**Workflow:** CI
**Status:** completed
**Conclusion:** success

**CodeQL also ran automatically** on `d0968b8`:
**Run URL:** https://github.com/TrustEdge-Labs/sealedge/actions/runs/24723883409
**Status:** completed
**Conclusion:** success

**Key signal:** the run URLs are now under `https://github.com/TrustEdge-Labs/sealedge/actions/...` — GitHub Actions follows the renamed repo without any workflow file changes required.

---

## D-13 §4 — Deployed Service Still Live (REFRAMED)

**Original assumption:** "DigitalOcean App Platform fires auto-deploy on the post-rename push; verify page stays live."

**Reframed finding (discovered during execution):** The live DO App Platform config for `trustedge-verifier` does **not** use a GitHub-source auto-deploy. It uses **Image Repository** source (`trustedge-verifier`, tag `v4.0`, digest `9401b92`). The `deploy/digitalocean/app.yaml` file in the repo declares a `github:` source block, but the live-running app was switched to image-registry source at some point and never reverted.

This means the rename had **zero bearing** on the DO deployment — DO wasn't watching git at all. The original D-13 §4 phrasing ("auto-deploy fires on push") therefore doesn't apply. What the rename CAN impact for DO is: whether the deployed service stays reachable under its existing URL.

**Verification:** DO-hosted service remained reachable throughout the rename.

**DO app URL:** `https://seashell-app-jbfi4.ondigitalocean.app/`
**`/healthz` HTTP status:** 200
**`/healthz` body:** `{"status":"OK","timestamp":"2026-04-21T13:53:10.635151224+00:00"}`
**TTFB:** 96ms (healthy)

**Follow-on observation:** `/` returns 404 — there is no static verify page served at root by the currently-deployed binary. This is not a regression from the rename; it is the existing deployment's behavior. If / was expected to serve a verify page, the deployed Docker image is the subset that exposes only the API (e.g., `/healthz`) and not the static frontend. Capturing this as a deferred todo (see "Deferred operational findings" below).

**Conclusion on D-13 §4:** The deployed service survived the rename — `/healthz` returns 200 post-rename with fresh timestamp. ROADMAP §Phase 87 success criterion #3 (in-repo references resolve against the renamed repo) is satisfied independently via the Plan 01 D-13 §3 grep audit, not via DO.

---

## External Integrations Post-Rename

| Integration | Status | Notes |
|-------------|--------|-------|
| GitHub built-in 301 redirect | ✔ | Verified via curl (D-13 §1) |
| GitHub Actions (CI, WASM, CodeQL) | ✔ | Run URLs now under `/TrustEdge-Labs/sealedge/actions/...` |
| Dependabot | ✔ | Auto-followed (new branch `dependabot/cargo/cargo-39cecc6af4` appeared on renamed remote during `git fetch`) |
| CLA Assistant | 🟡 Untested | Signatures are repo-ID-linked per D-12; will be verified on first post-rename PR |
| DigitalOcean App Platform | 🟡 N/A | Service stayed live; DO uses image-registry source not git, so the rename was orthogonal (see D-13 §4 reframe above) |

---

## ROADMAP §Phase 87 Success Criteria

| # | Criterion | Status |
|---|-----------|--------|
| 1 | Repo accessible at `/TrustEdge-Labs/sealedge`; old URL 301 redirects | ✔ PASS — D-13 §1 |
| 2 | Local origin points to new URL; `git push`/`git pull` work without manual fixes | ✔ PASS — D-13 §2 |
| 3 | In-repo references resolve against the renamed repo | ✔ PASS — Plan 01 grep audit clean + D-13 §2 fetch/push against `sealedge.git` |

---

## Rollback Status

**Not executed.** All 4 D-13 checks passed. Rollback command retained verbatim (D-15 pattern):

```
gh repo rename trustedge -R TrustEdge-Labs/sealedge
git remote set-url origin https://github.com/TrustEdge-Labs/trustedge.git
```

The 404 during the user's initial command confusion (pasting the rollback form before the rename had occurred) left no state change on GitHub.

---

## Deferred Operational Findings (not Phase 87 blockers)

1. **`deploy/digitalocean/app.yaml` ↔ live DO config mismatch** — the repo's declarative spec (`github.repo: TrustEdge-Labs/sealedge`) describes a git-source deploy, but the live DO app uses image-registry source (`trustedge-verifier:v4.0`). Neither is wrong on its own; they just don't match. Reconciliation path options: (a) commit the current live state back into `app.yaml` (image-registry section), (b) re-point DO to the git source, (c) delete `app.yaml` if it's purely aspirational. Pick one in a future operational cleanup.

2. **DO app's `name:` field `trustedge-verifier`** — internal DO app name is unchanged. Renaming the DO app is explicitly out of Phase 87 scope per D-11. If branding consistency across DO matters, a follow-up can rename the DO app; not required for product function.

3. **DO app root `/` returns 404** — not caused by the rename. The deployed image apparently doesn't serve a verify page at root. If the product surface requires a verify-page frontend, a follow-up is needed to rebuild/redeploy a variant that serves both `/healthz` AND `/verify`.

4. **CLA Assistant untested post-rename** — will be validated on the first post-rename PR. Expected to work (signatures repo-ID-linked); flag here for operational awareness.

---

## Supporting Git State Post-Close

**HEAD at phase-close push:** `d0968b8` (verification-trigger commit) — Plan 02 Task 7 adds the `docs(87):` commit on top containing this VERIFICATION.md and the Plan 02 SUMMARY.

**origin/main at phase-close:** `d0968b8` (before Task 7 commit pushes)

**Remote URL:** `https://github.com/TrustEdge-Labs/sealedge.git` (HTTPS retained per D-08)

---

*Phase: 87-github-repository-rename*
*Verification completed: 2026-04-21*
*EXT-01 satisfied; Phase 88 unblocked.*
