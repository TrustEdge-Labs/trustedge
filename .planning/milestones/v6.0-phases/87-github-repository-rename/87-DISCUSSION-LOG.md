# Phase 87: GitHub Repository Rename - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in `87-CONTEXT.md` — this log preserves the alternatives considered.

**Date:** 2026-04-20
**Phase:** 87-github-repository-rename
**Areas discussed:** Straggler URL cleanup scope, Rename mechanics & sequencing, External service reconfig checklist, Post-rename verification gate

---

## Gray-area selection

**Question:** Which areas do you want to discuss for Phase 87 (operational — rename is a single click/gh command)?

| Option | Description | Selected |
|---|---|---|
| Straggler URL cleanup scope | Fix 3 tracked files (`.github/ISSUE_TEMPLATE/config.yml`, `.github/workflows/cla.yml`, `deploy/digitalocean/app.yaml`) that Phase 85/86 didn't scope | ✓ |
| Rename mechanics & sequencing | `gh` CLI vs web UI, who executes, pre-rename checks, order of operations | ✓ |
| External service reconfig checklist | DO App Platform, CLA Assistant, other services referencing repo URL | ✓ |
| Post-rename verification gate | What proves the rename worked — curl, git fetch, CI, DO deploy | ✓ |

**User's choice:** All 4 areas — rename is simple, but hygiene surfaces are worth nailing down.

---

## Straggler URL cleanup scope

### Q1: Fold the 3 tracked files into Phase 87?

| Option | Description | Selected |
|---|---|---|
| Fold all 3 into Phase 87 | One tidy cleanup commit updating `.github/ISSUE_TEMPLATE/config.yml`, `.github/workflows/cla.yml`, `deploy/digitalocean/app.yaml` | ✓ |
| Only `.github/` files, defer `deploy/` | Fix the GitHub-UI-visible files, leave DO config for Phase 88/89 | |
| None — strictly repo rename + git remote | Only `gh repo rename` + `git remote set-url`; stragglers resolve via GitHub redirect | |

**User's choice:** Fold all 3 into Phase 87.
**Notes:** Same hygiene bar as Phase 85/86; no redirect-dependent cruft.

### Q2: Any Phase 87 change to CHANGELOG.md / MIGRATION.md (hybrid treatment from Phase 86)?

| Option | Description | Selected |
|---|---|---|
| Leave both untouched | Historical entries describe what was shipped under old name; GitHub redirect keeps URLs working | ✓ |
| Add a post-rename breadcrumb to MIGRATION.md | Small note: "GitHub repo renamed on <date>; URLs before that resolve via redirect." | |
| Rewrite historical URLs to point at /sealedge | Consistent casing but rewrites history | |

**User's choice:** Leave both untouched.
**Notes:** Phase 86 D-02/D-03 hybrid treatment is final.

### Q3: Single commit or split for straggler fixes?

| Option | Description | Selected |
|---|---|---|
| Single atomic commit | `fix(87): point last tracked URL refs at /TrustEdge-Labs/sealedge` | ✓ |
| Two commits split (`.github/` vs `deploy/`) | More granular log, slightly more overhead | |

**User's choice:** Single atomic commit.

### Q4: Timing relative to GitHub rename?

| Option | Description | Selected |
|---|---|---|
| Before the GitHub rename | Commit + push on old repo name, then rename. New clones post-rename have clean URLs. | ✓ |
| After the GitHub rename | Rename first, then commit stragglers on renamed repo | |
| Doesn't matter (redirect covers both) | Pick whatever reads cleanest | |

**User's choice:** Before the GitHub rename.

---

## Rename mechanics & sequencing

### Q1: How should the actual rename operation be executed?

| Option | Description | Selected |
|---|---|---|
| You run `gh repo rename` | User executes `gh repo rename sealedge -R TrustEdge-Labs/trustedge` locally | ✓ |
| You use GitHub web UI | User opens settings → Repository name → type + click Rename | |
| Claude runs `gh repo rename` via Bash | Claude runs it directly via Bash tool | |

**User's choice:** You run `gh repo rename`.
**Notes:** Live external operation; user-driven matches careful-actions default. Claude provides the exact command.

### Q2: What checks must be green before triggering the rename?

| Option | Description | Selected |
|---|---|---|
| Main clean + pushed | `git status` clean, origin/main matches HEAD | ✓ |
| CI green on latest main | `gh run list --branch main --limit 1` shows success | ✓ |
| No open PRs / in-flight branches | Rebase or land open PRs before rename | ✓ |
| No scheduled workflows firing | Avoid mid-run rename on a scheduled Action | ✓ |

**User's choice:** All 4 pre-rename checks.

### Q3: Order of operations?

| Option | Description | Selected |
|---|---|---|
| Commit stragglers → push → rename → update local remote → verify | 5-step clean sequence, each step verifiable | ✓ |
| Rename first → commit stragglers → push to renamed repo | Straggler fixes land on `/sealedge` from the start | |

**User's choice:** Commit → push → rename → update → verify.

### Q4: Protocol for updated local git remote?

| Option | Description | Selected |
|---|---|---|
| Keep current HTTPS | Current origin is HTTPS; `gh auth` set up for HTTPS | ✓ |
| Switch to SSH | Use SSH key auth going forward | |

**User's choice:** Keep HTTPS.

---

## External service reconfig checklist

### Q1: Which external services need manual post-rename attention?

| Option | Description | Selected |
|---|---|---|
| DigitalOcean App Platform | `deploy/digitalocean/app.yaml` + DO control-panel source config; DO webhook may not auto-follow rename | ✓ |
| CLA Assistant GitHub App | `.github/workflows/cla.yml` URL refs + verify CLA Assistant still tracks post-rename | ✓ |
| Dependabot / GitHub built-in integrations | Managed inside repo; auto-follow. Sanity-check only. | ✓ |
| GitHub Marketplace Action listing | Phase 88 fully replaces this. Listed for completeness, NOT Phase 87 scope. | ✓ |

**User's choice:** All 4 noted (Marketplace flagged as Phase 88, not actioned here).

### Q2: Where to document the external-service checklist?

| Option | Description | Selected |
|---|---|---|
| Phase 87 PLAN.md task | Done-criteria enumerates DO + CLA + Dependabot sanity-check; evidence in 87-VERIFICATION.md | ✓ |
| Append to MIGRATION.md v6.0 section | Persistent reference in MIGRATION.md | |
| Both — PLAN.md task AND MIGRATION.md note | Slight duplication, covers both operational + reference angles | |

**User's choice:** Phase 87 PLAN.md task.

### Q3: DigitalOcean touch level?

| Option | Description | Selected |
|---|---|---|
| Update `app.yaml` + verify DO auto-deploys work | File update in straggler commit + verify DO control panel fires post-rename | ✓ |
| Update `app.yaml` only, skip live verify | Update declarative config; don't manually test DO deploy webhook | |
| Skip entirely, DO is out of scope | Leave DO for a later operational pass | |

**User's choice:** Update + verify live.

### Q4: CLA Assistant post-rename handling?

| Option | Description | Selected |
|---|---|---|
| Update workflow paths, verify next PR | URLs updated in straggler commit; verify CLA Assistant still comments on first post-rename PR | ✓ |
| Drop CLA Assistant reference, point to CLA.md directly | Simplify by removing the automation | |

**User's choice:** Update + verify on next PR.

---

## Post-rename verification gate

### Q1: What proves the rename worked before Phase 87 is called done?

| Option | Description | Selected |
|---|---|---|
| `curl -I` old URL shows 301 redirect | `curl -I https://github.com/TrustEdge-Labs/trustedge` returns 301 to /sealedge | ✓ |
| `git fetch` + test commit/push works | Confirms local remote + auth work against renamed URL | ✓ |
| CI workflow green on post-rename push | One CI run post-rename on renamed repo succeeds | ✓ |
| DO App auto-deploy fires + verify page live | End-to-end: push → DO deploy → verify page stays up | ✓ |

**User's choice:** All 4 verification checks required.

### Q2: Is verification a dedicated plan in Phase 87, or Phase 89?

| Option | Description | Selected |
|---|---|---|
| Dedicated plan in Phase 87 | One Phase 87 plan covers rename + verify; 87-VERIFICATION.md captures evidence | ✓ |
| Verification is Phase 89 | Phase 87 just does the rename; verification rolls into Phase 89 | |
| Split — rename-critical checks here, broader matrix in 89 | Phase 87 verifies rename integrity; Phase 89 owns full test matrix | |

**User's choice:** Dedicated plan in Phase 87.
**Notes:** Phase 89 still owns the broader VALID-01/02/03 matrix; Phase 87 only verifies rename-integrity.

### Q3: Failure handling if a verification check fails?

| Option | Description | Selected |
|---|---|---|
| Rollback + diagnose | `gh repo rename trustedge` (reversible via GitHub rename-history window), diagnose, retry | ✓ |
| Fix forward only | Don't plan rollback; fix the downstream issue with renamed repo in place | |
| Rollback only for redirect failure; fix-forward everything else | Hybrid: rollback on GitHub-side primitive failure, fix-forward for downstream config | |

**User's choice:** Rollback + diagnose.

### Q4: How is verification evidence captured?

| Option | Description | Selected |
|---|---|---|
| `87-VERIFICATION.md` with command outputs | Standard GSD pattern from Phases 83-86: curl/git fetch + CI run URL + DO deploy note | ✓ |
| Inline in SUMMARY.md, no separate file | Shorter paper trail, less auditable | |
| Screenshots + terminal output | More visual; more effort; useful for demo artifact | |

**User's choice:** 87-VERIFICATION.md with command outputs.

---

## Claude's Discretion

- Plan granularity (1 plan vs 3 plans) — planner's call based on complexity budget
- Commit message prefixes (`fix(87):` for stragglers, `chore(87):` for rename-adjacent)
- v6.0 release tag is NOT cut in Phase 87 (per v6.0 REQUIREMENTS.md out-of-scope)
- 30-second retry buffer for curl redirect if first attempt returns non-301 (GitHub edge cache lag)
- PR rebasing policy if a D-06 check catches an open PR — default is rebase-and-land before rename

## Deferred Ideas

- v6.0 release tag cutting — Phase 89 milestone-close
- New GitHub Action repo + Marketplace replacement + deprecate old listing — Phase 88
- Product-page content on trustedgelabs.com — Phase 88
- Full VALID-01/02/03 test matrix — Phase 89
- Permanent CI guard against `/TrustEdge-Labs/trustedge` URL regressions — optional backlog item
- Local working-directory path rename — out of scope; personal filesystem
