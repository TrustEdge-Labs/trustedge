# Phase 87: GitHub Repository Rename - Context

**Gathered:** 2026-04-20
**Status:** Ready for planning

<domain>
## Phase Boundary

Rename the monorepo from `TrustEdge-Labs/trustedge` to `TrustEdge-Labs/sealedge` on GitHub, update the local git remote to the new URL, and clean up the last few tracked files Phase 85/86 didn't scope so no `/TrustEdge-Labs/trustedge` URL references remain in live (non-planning, non-historical) source. Verify GitHub's auto-redirect is in place, CI still runs on the renamed repo, and external services (DigitalOcean App Platform, CLA Assistant) still function against the renamed repo.

**In scope:**
- Execute `gh repo rename sealedge -R TrustEdge-Labs/trustedge` from the user's machine
- Update local `git remote` origin URL from `/TrustEdge-Labs/trustedge.git` to `/TrustEdge-Labs/sealedge.git` (HTTPS)
- Fix 3 tracked files still containing `/TrustEdge-Labs/trustedge` URL strings that Phase 85/86 didn't scope:
  - `.github/ISSUE_TEMPLATE/config.yml` (3 URLs)
  - `.github/workflows/cla.yml` (2 URLs in CLA-assistant config)
  - `deploy/digitalocean/app.yaml` (`repo: TrustEdge-Labs/trustedge` source ref)
- Verify post-rename operation: GitHub 301 redirect, `git fetch` against renamed remote, one CI run green, DigitalOcean App Platform auto-deploy fires
- Document the external-service reconfig checklist in PLAN.md done-criteria

**Out of scope (explicit carve-outs):**
- `TrustEdge-Labs` GitHub org name — stays (per v6.0 memory)
- `trustedgelabs.com` domain — stays (Phase 88 updates product-page content)
- `trustedgelabs-website` repo name — stays (Phase 88 touches its content, not its name)
- CHANGELOG.md and MIGRATION.md historical entries — hybrid-treated in Phase 86; past-version entries stay verbatim (no URL rewrites)
- `RFC_K256_SUPPORT.md`, `improvement-plan.md`, `security-review-platform.md` — historical artifacts allowlisted in Phase 86 D-01a
- `actions/attest-sbom-action/**` — Phase 88 deprecates/replaces the Marketplace Action (no edits here)
- `.planning/**` — historical project-management artifacts
- Cutting the `v6.0` semver release tag — per v6.0 REQUIREMENTS.md out-of-scope list, the release tag is cut post-rename (Phase 89 milestone-close)
- Full test matrix (VALID-01/02/03) — Phase 89 owns this; Phase 87 verifies only the rename-integrity checks

</domain>

<decisions>
## Implementation Decisions

### Straggler URL cleanup scope

- **D-01:** Phase 87 sweeps the 3 tracked files Phase 85/86 didn't scope to `/TrustEdge-Labs/sealedge` (the last live `/TrustEdge-Labs/trustedge` references in tracked source outside `.planning/`, historical allowlists, and the Phase 88-owned Action repo):
  | File | Refs | Kind |
  |---|---|---|
  | `.github/ISSUE_TEMPLATE/config.yml` | 3 URLs (security advisories, discussions, README blob) | GitHub issue router |
  | `.github/workflows/cla.yml` | 2 URLs (`path-to-document` + CLA assistant prose) | CLA assistant config |
  | `deploy/digitalocean/app.yaml` | 1 ref (`repo: TrustEdge-Labs/trustedge`) | DO App Platform source |
- **D-02:** `CHANGELOG.md` and `MIGRATION.md` stay untouched in Phase 87. Phase 86's hybrid treatment (D-02/D-03 of 86-CONTEXT.md) is final — historical entries describe what was shipped under the old name; rewriting them would create historical inaccuracy. GitHub's redirect keeps any old URL references in those entries resolvable.
- **D-03:** The 3 straggler files get a single atomic commit: `fix(87): point last tracked URL refs at /TrustEdge-Labs/sealedge`. Clean break, easy to revert, matches Phase 85/86 single-commit-per-logical-step pattern.
- **D-04:** Straggler-fix commit lands **before** the GitHub rename operation — commit clean URL strings on the old repo name, push, then rename. New clones post-rename already have clean URLs; the tiny window between push and rename is covered by GitHub's redirect. Reverses to a no-op if rename is rolled back.

### Rename mechanics & sequencing

- **D-05:** **User runs the rename command** from their shell: `gh repo rename sealedge -R TrustEdge-Labs/trustedge`. Auth is already set up per `gh auth status` (HTTPS, scope: `repo`). Claude provides the exact command string and the pre-rename checklist; user confirms and executes. Not delegated to Claude's Bash tool because the rename is a live operation on a shared surface — user-driven matches the "careful actions" default.
- **D-06:** **Pre-rename gate — all 4 checks must be green** before triggering the rename:
  1. `git status` clean on `main`; `git log origin/main..HEAD` empty (main pushed to origin)
  2. Latest CI run on `main` is green (`gh run list --branch main --limit 1`)
  3. No open PRs requiring rebase (`gh pr list --state open`)
  4. No scheduled workflows currently firing (`gh run list --workflow=* --status in_progress`)
- **D-07:** **Order of operations** (commit → push → rename → update remote → verify):
  1. Commit the D-01 straggler fixes on `main`
  2. `git push origin main` (still against `/TrustEdge-Labs/trustedge`)
  3. Run pre-rename gate checks (D-06)
  4. `gh repo rename sealedge -R TrustEdge-Labs/trustedge`
  5. `git remote set-url origin https://github.com/TrustEdge-Labs/sealedge.git`
  6. Run verification gate (D-13)
- **D-08:** **Keep HTTPS** for the new origin URL — current remote is HTTPS (`https://github.com/TrustEdge-Labs/trustedge.git`) and `gh auth status` shows HTTPS as the active git protocol. Switching to SSH would be unrelated to the rename and adds risk (new auth path to validate).

### External service reconfig checklist

- **D-09:** Services that need post-rename attention (enumerated in PLAN.md done-criteria):
  - **DigitalOcean App Platform** — `deploy/digitalocean/app.yaml` + DO control-panel source config. The declarative file is updated in the straggler commit (D-01); the control-panel reference may or may not auto-follow the GitHub rename depending on how DO resolves the source. Verify auto-deploy fires post-rename; if not, reconfigure the source repo in DO's App settings.
  - **CLA Assistant GitHub App** — `.github/workflows/cla.yml` URLs updated in the straggler commit (D-01). Verify CLA Assistant still comments on the first post-rename PR; signatures are repo-ID-linked (not URL-linked), so should survive the rename.
  - **Dependabot + built-in GitHub integrations** — managed inside the repo; auto-follow the rename. Sanity-check only: confirm Dependabot still opens PRs, security alerts still fire. No config changes expected.
  - **GitHub Marketplace Action listing** — listed for completeness; **Phase 88 owns** the replacement action + deprecation of the old listing. Phase 87 does not touch `actions/attest-sbom-action/**`.
- **D-10:** External-service checklist lives in Phase 87's `PLAN.md` done-criteria (phase-scoped, one-time operational checklist). Not duplicated into `MIGRATION.md` — Phase 86's v6.0 MIGRATION section already covers the rename story for future readers; MIGRATION.md describes the user-facing break, not internal ops.
- **D-11:** **DO App Platform touch level**: update `deploy/digitalocean/app.yaml` `repo:` field **and** verify DO control-panel auto-deploy fires on a post-rename push to main. If DO lost the webhook after rename, reconfigure the source in the DO App settings — this is user-only work (Claude can't drive the DO UI).
- **D-12:** **CLA Assistant handling**: update `.github/workflows/cla.yml` URL strings (already covered by D-01 straggler commit) and verify the CLA Assistant GitHub App still comments on the next post-rename PR. Do not drop the CLA assistant integration — tangential cleanup, stay focused on rename.

### Post-rename verification gate

- **D-13:** **All 4 verification checks must pass** before Phase 87 is called done:
  1. `curl -I -s -o /dev/null -w '%{http_code} -> %{redirect_url}\n' https://github.com/TrustEdge-Labs/trustedge` returns `301 -> https://github.com/TrustEdge-Labs/sealedge` (GitHub built-in redirect working)
  2. `git fetch origin` and `git ls-remote origin` against the renamed URL succeed; a trivial push against the renamed remote works
  3. One CI workflow run on the renamed repo completes green (any trigger — push, manual dispatch, or scheduled run)
  4. DigitalOcean App Platform auto-deploy fires on the post-rename push; verify page (`verify.sealedge.dev` or current deploy URL) stays live
- **D-14:** **Phase 87 owns rename-integrity verification** via a dedicated plan (rename + verify in one plan, or a dedicated sub-plan if the planner decides to split). **Phase 89 owns** the broader VALID-01/02/03 matrix (full test suite, feature-matrix CI, WASM + dashboard + Docker stack). Phase 87 proves the rename operation itself succeeded; Phase 89 proves the product still works end-to-end under the new names.
- **D-15:** **Failure handling — rollback + diagnose**. GitHub repo renames are reversible via `gh repo rename trustedge -R TrustEdge-Labs/sealedge` within GitHub's rename-history window. If any D-13 check fails, rollback the rename, diagnose root cause, retry. PLAN.md includes the rollback command verbatim for fast execution. This is a clean-break-safe move because the only downstream state is (a) the repo name and (b) the local remote URL — both cheaply reversible.
- **D-16:** **Evidence capture**: `87-VERIFICATION.md` contains command outputs from D-13 (curl output with 301 line, `git fetch` output, CI run URL, DO auto-deploy timestamp / verify-page screenshot). Matches the Phase 83-86 verification pattern. No screenshots required; command output + URLs are sufficient.

### Claude's Discretion

- **Plan granularity:** Likely 1 plan covering the full rename + straggler commit + verification sequence (small phase, tight atomicity requirement). If the planner decides straggler cleanup + rename + verify is too much for one plan, split into:
  - Plan 01: Straggler URL fixes (the `.github/` + `deploy/` commit, D-01/D-03)
  - Plan 02: Rename execution (pre-checks + `gh repo rename` + remote update, D-06/D-07)
  - Plan 03: Verification gate (D-13 checks + 87-VERIFICATION.md, D-16)
- **Commit messages:** Straggler commit style: `fix(87): point last tracked URL refs at /TrustEdge-Labs/sealedge`. Rename-operation commit (if there's a post-rename commit to capture the remote URL change or small touch-up): `chore(87): rename GitHub repo trustedge → sealedge` or similar.
- **v6.0 release tag:** **NOT cut in Phase 87** — per v6.0 REQUIREMENTS.md out-of-scope list, the release tag is cut from the renamed repo after the rebrand lands. Expected to be cut at Phase 89 milestone close (or as a separate milestone-close step).
- **Timing window for redirect stabilization:** GitHub's rename redirect is typically immediate. If the D-13 curl check returns unexpected output, wait 30 seconds and retry once before declaring failure — GitHub's DNS / edge caches can occasionally lag.
- **PR rebasing:** If a D-06 pre-rename check catches an open PR, the planner's default response is to rebase-and-land it before the rename (don't leave work stranded). If the PR can't be landed quickly, defer the rename until the PR lands or the author agrees to rebase post-rename.

### Folded Todos

None — no backlog todos fold into this phase.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Milestone-level (v6.0 scope)
- `.planning/REQUIREMENTS.md` §Ext — EXT-01 maps to Phase 87 (GitHub monorepo rename + redirect + local remotes)
- `.planning/REQUIREMENTS.md` §"Out of Scope" — v6.0 release tag explicitly NOT cut on the old repo name (informs D-15 / Claude's Discretion)
- `.planning/PROJECT.md` §Current Milestone — v6.0 target, clean-break preference
- `.planning/ROADMAP.md` §"Phase 87: GitHub Repository Rename" (line 196-204) — goal + 3 success criteria

### Prior v6.0 phase decisions that carry forward
- `.planning/phases/83-crate-and-binary-rename/83-CONTEXT.md` — sealedge crate/binary naming establishes what the renamed repo hosts
- `.planning/phases/86-documentation-sweep/86-CONTEXT.md` §D-02/D-03 — CHANGELOG.md + MIGRATION.md hybrid treatment; past URLs in historical entries stay intact (reaffirmed here in D-02)
- `.planning/phases/86-documentation-sweep/86-CONTEXT.md` §D-01a — allowlisted historical artifacts (`improvement-plan.md`, `RFC_K256_SUPPORT.md`, `security-review-platform.md`) continue to be out-of-scope for URL sweeps
- `.planning/phases/86-documentation-sweep/86-CONTEXT.md` §D-06a — `actions/attest-sbom-action/**` is Phase 88, not touched here

### Memory / user context
- `~/.claude/projects/-home-john-vault-projects-github-com-trustedge/memory/project_v6_rebrand.md` — strict linear phase order (87 before 88/89), GitHub rename is the one-shot external operation
- `~/.claude/projects/-home-john-vault-projects-github-com-trustedge/memory/feedback_clean_break_compat.md` — no backward-compat shims; rename is a clean break backed by GitHub's auto-redirect (not a dual-URL maintenance path)

### Straggler source files (D-01 scope)
- `.github/ISSUE_TEMPLATE/config.yml` — 3 URLs: security advisories, discussions, README blob
- `.github/workflows/cla.yml` — 2 URLs: CLA Assistant `path-to-document` + prose in custom-message template
- `deploy/digitalocean/app.yaml` — `repo: TrustEdge-Labs/trustedge` source field; DO App Platform reads this to resolve source repo

### External service docs
- GitHub "About repository renaming" (docs.github.com) — confirms rename preserves issues/PRs/stars/watchers, creates a 301 redirect from the old URL, preserves webhooks and integrations keyed by repo ID (verifies D-13 / D-15 assumptions)
- `gh repo rename` docs — syntax, reversibility, required permissions (user already has `repo` scope per `gh auth status`)

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- **`gh` CLI already authenticated** — `gh auth status` confirms HTTPS protocol, `repo` + `workflow` + `read:org` scopes. `gh repo rename` works out of the box.
- **GitHub's built-in 301 redirect** on renamed repos — no manual redirect rule needed; verified by `curl -I` on the old URL post-rename (D-13).
- **CI workflows use `${{ github.repository }}` and relative refs** — `actions/checkout` and most Actions operations auto-follow the rename. Scanned `.github/workflows/*.yml` in prior phases: no hardcoded `/TrustEdge-Labs/trustedge` in workflow logic (only in `cla.yml` prose, which D-01 fixes).

### Established Patterns
- **Phase 85/86 URL sweep pattern** — repo-wide grep for `/TrustEdge-Labs/trustedge` with allowlist (`.planning/|actions/attest-sbom-action/|improvement-plan|RFC_K256_SUPPORT|security-review-platform`) surfaces exactly the 3 stragglers. After the D-01 commit, re-run the same grep; expected result: zero hits outside the allowlist.
- **Atomic-commit-per-logical-step** (Phase 83/84/85/86 carry-forward) — the straggler commit is one logical step; the rename is an external operation that isn't a commit itself; remote URL update is a local-git-config change (not a commit). Verification evidence goes in a single `87-VERIFICATION.md` at phase close.
- **Clean-break semantics** (v6.0 memory) — no dual-URL shim, no redirect-forwarding wrapper, no script to sync the two names. GitHub's built-in redirect is the one compatibility mechanism and it's free.

### Integration Points
- **DigitalOcean App Platform ↔ `deploy/digitalocean/app.yaml`** — DO resolves source via the `repo:` field. Updating the file is straightforward; verifying DO's control-panel reference still auto-deploys post-rename is the unknown (D-11 verification step).
- **CLA Assistant GitHub App ↔ `.github/workflows/cla.yml`** — the action references `path-to-document` (URL to CLA.md) and a prose message. URL updates in D-01 fix the strings; the GitHub App itself tracks signatures by repo ID (survives rename). Verify on first post-rename PR.
- **GitHub release asset URLs** — existing release assets from v3.0-v5.0 have URLs like `/TrustEdge-Labs/trustedge/releases/download/...`. These redirect automatically via GitHub's 301 mechanism. No manual relinking required.
- **Badge URLs in CHANGELOG.md / historical docs** — left intact per D-02; redirect covers them.

</code_context>

<specifics>
## Specific Ideas

- **Exact straggler-commit grep audit** — after the D-01 commit lands, run:
  ```
  git ls-files | xargs grep -n "TrustEdge-Labs/trustedge" 2>/dev/null \
    | grep -vE '^\.planning/|^\.factory/|actions/attest-sbom-action/|improvement-plan\.md|RFC_K256_SUPPORT\.md|security-review-platform\.md|CHANGELOG\.md|MIGRATION\.md'
  ```
  Expected: zero results. Any hits = planner expanded allowlist was wrong; fix before rename.
- **Rollback command kept in PLAN.md verbatim** — `gh repo rename trustedge -R TrustEdge-Labs/sealedge`. Copy-pasteable. Useful if D-13 redirect check fails immediately.
- **CI-green check uses `gh run list --branch main --limit 1`** — fast, deterministic. Blocks rename if not green.
- **DO App auto-deploy verification** — push a trivial touch-up commit (e.g., README whitespace nudge) post-rename; watch DO's deploy timeline in the App Platform UI. If DO doesn't fire, reconfigure source in DO App settings (user-only action).
- **"30-second retry" buffer for curl redirect** — GitHub's edge cache can occasionally lag on the 301 immediately post-rename. If first curl returns something other than 301, wait 30s and retry once before declaring failure.

</specifics>

<deferred>
## Deferred Ideas

- **v6.0 release tag cutting** — Phase 89 milestone-close or separate milestone-close step; per v6.0 REQUIREMENTS.md explicit out-of-scope.
- **New GitHub Action repo under sealedge naming + Marketplace replacement + deprecate old listing** — Phase 88 (EXT-02, EXT-03).
- **Product-page content refresh on trustedgelabs.com** — Phase 88 (EXT-04). Domain stays, content updates.
- **Full VALID-01/02/03 test matrix** — Phase 89 owns the full workspace test pass, feature-matrix CI, WASM + dashboard + Docker stack validation.
- **Permanent CI guard against `/TrustEdge-Labs/trustedge` URL regressions** — not required; one-time grep in D-13-style verification is sufficient. Add as backlog if desired.
- **Rename the local working-directory path** (`/home/john/vault/projects/github.com/trustedge` → `.../sealedge`) — out of scope; personal filesystem concern, not a repo-hygiene concern. Can happen anytime post-rename or never.

### Reviewed Todos (not folded)
None — no todos were considered for this phase.

</deferred>

---

*Phase: 87-github-repository-rename*
*Context gathered: 2026-04-20*
