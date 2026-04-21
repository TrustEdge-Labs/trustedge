---
phase: 88-external-action-product-website
plan: 01
subsystem: external-action
tags:
  - rebrand
  - action-source
  - rename-in-place
  - grep-audit
  - phase-88
one_liner: "Rename-in-place sweep of the attest-sbom GitHub Action source: 22 action.yml substrings + README top-notice + product/URL sweep + monorepo folder `git mv` + REQUIREMENTS §Ext amendment — delivered as 4 atomic commits; T2 SHA256 + T4 ephemeral keygen mitigations preserved structurally."
dependency_graph:
  requires: []
  provides:
    - sealedge-attest-sbom-action-source
    - ext-02-wording-aligned
    - ext-03-wording-aligned
  affects:
    - actions/sealedge-attest-sbom-action/
    - .planning/REQUIREMENTS.md
tech-stack:
  added: []
  patterns:
    - rename-in-place-backed-by-github-301-redirect
    - atomic-commit-per-logical-step
    - targeted-Edit-over-sed-for-YAML
    - git-mv-for-folder-rename
    - allowlisted-grep-audit
key-files:
  created: []
  modified:
    - actions/sealedge-attest-sbom-action/action.yml
    - actions/sealedge-attest-sbom-action/README.md
    - .planning/REQUIREMENTS.md
  renamed:
    - actions/attest-sbom-action/ → actions/sealedge-attest-sbom-action/
decisions:
  - "Honored CONTEXT.md D-02 verbatim replacement text for EXT-02/EXT-03 (the authoritative source) even though one plan-level acceptance criterion was over-specified against it — CONTEXT.md wins on conflict (Rule 1 deviation, documented below)"
  - "Used per-Edit substring swaps (preferred over sed) per Phase 87 Plan 01 Task 1 discipline — deterministic outcomes, no YAML reflow, line count preserved (98 before, 98 after)"
  - "T2 mitigation (SHA256 checksum verification) preserved structurally: `sha256sum.*runner.temp.*/seal` grep returns 1, `seal.sha256` fetch/compare logic byte-for-byte unchanged aside from filename swap"
  - "T4 mitigation (ephemeral keygen) preserved structurally: `seal\" keygen` grep returns exactly 1, step ordering + `if: inputs.key == ''` guard unchanged, env var rename TRST_KEY/TRST_PUB → SEAL_KEY/SEAL_PUB is pure substitution"
metrics:
  duration: "3m 31s"
  completed: "2026-04-21"
  tasks: "4/4"
  commits: 4
---

# Phase 88 Plan 01: External Action Source In-Place Rewrite + Monorepo Folder Rename + REQUIREMENTS §Ext Amendment Summary

## What Landed

Four atomic commits on the current branch, each scoped to a single logical step:

| # | Commit | Type | Files | Scope |
|---|--------|------|-------|-------|
| 1 | `b511b47` | `fix(88-01)` | `actions/attest-sbom-action/action.yml` | 22-line-local substitution: trst→seal binary + seal-version input + /TrustEdge-Labs/sealedge/releases URL base + Sealedge metadata + SEAL_KEY/SEAL_PUB env vars + Project: sealedge header. SHA256 verification block + ephemeral keygen step structurally preserved. Line count unchanged (98). |
| 2 | `d2f676d` | `docs(88-01)` | `actions/attest-sbom-action/README.md` | Prepend top-of-README "Renamed from `TrustEdge-Labs/attest-sbom-action`" notice (1 blockquote paragraph, ~3 lines per D-08). Sweep product-name / URL / example-YAML substrings per D-15: `uses:` example lines → `@v2`, secret-name `TRUSTEDGE_KEY` → `SEALEDGE_KEY`, `trst` binary in shell examples → `seal`, verify.trustedge.dev → verify.sealedge.dev, links to /TrustEdge-Labs/trustedge → /TrustEdge-Labs/sealedge. Company brand `TrustEdge Labs` and org path `TrustEdge-Labs/` preserved. |
| 3 | `cd28001` | `chore(88-01)` | `actions/attest-sbom-action/` → `actions/sealedge-attest-sbom-action/` | `git mv` — pure folder rename. 3 files relocated (action.yml, README.md, LICENSE). 100% rename similarity score. `git log --follow` walks through the rename (5 commits of pre-rename history visible). No orphan refs anywhere under `.github/`, `scripts/`, `docs/`, or root. |
| 4 | `172dc77` | `docs(88-01)` | `.planning/REQUIREMENTS.md` | 2-line replacement on lines 39-40 (EXT-02 + EXT-03) with CONTEXT.md §Specifics D-02 verbatim text. EXT-01, EXT-04, VALID-01 and the rest of the requirements document untouched. Diff: exactly 2 insertions + 2 deletions. |

## Threat Model Mitigation Evidence

The plan's `<threat_model>` specifies `mitigate` dispositions for T1/T2/T4 (T-88-01 SHA256, T-88-02 ephemeral keygen, T-88-03 orphan refs). Evidence:

| Threat | Mitigation | Evidence |
|---|---|---|
| T-88-01 (T2 carry) — SHA256 verification | Preserve `sha256sum` invocation against the downloaded binary | `grep -c 'sha256sum.*runner\.temp.*/seal' actions/sealedge-attest-sbom-action/action.yml` → **1** (expected ≥1). Verification block shape (fetch `seal.sha256`, `awk '{print $1}'`, `sha256sum ... | awk '{print $1}'`, `if [ "$EXPECTED_HASH" != "$ACTUAL_HASH" ]`, `exit 1`) byte-for-byte preserved aside from filename swap `trst.sha256` → `seal.sha256` and binary path `trst` → `seal`. |
| T-88-02 (T4 carry) — Ephemeral keygen | Preserve `seal keygen` invocation + `if: inputs.key == ''` guard + env var writes | `grep -c 'seal\" keygen' actions/sealedge-attest-sbom-action/action.yml` → **1** (expected exactly 1). Step `name: Generate ephemeral keypair (if no key provided)` untouched, step ordering untouched, `if:` guard untouched. Env var rename `TRST_KEY`/`TRST_PUB` → `SEAL_KEY`/`SEAL_PUB` is pure substitution — downstream `Create SBOM attestation` step reads the renamed vars (lines 93-94 substitution aligned). |
| T-88-03 — Orphan refs | Zero references to old path in code/config | `grep -rnE 'actions/attest-sbom-action([/[:space:]"\]]|$)' --include='*.yml' --include='*.md' --include='*.toml' --include='*.sh' .` (excluding `.planning/`, `.factory/`, `.claude/` historical / worktree metadata) → **0 hits**. ci.yml has never referenced the action folder by path (uses inline shell); Plan 02 will later add a `uses:` reference to the renamed external repo. |
| T-88-04 — Audit trail | Commit message cites D-02, explains rationale | Commit `172dc77` body explicitly references 88-CONTEXT.md D-02 and states "no scope change". `git blame` on lines 39-40 of `.planning/REQUIREMENTS.md` will surface this rationale directly. |

## Plan-Level Success Criteria Status

All 7 criteria from the plan's `<success_criteria>` block verified:

1. ✔ `actions/sealedge-attest-sbom-action/action.yml` exists with all 22 substitutions landed; SHA256 + keygen preserved; 98 lines (no reflow).
2. ✔ `actions/sealedge-attest-sbom-action/README.md` exists with top-of-README "Renamed from" notice + full sweep.
3. ✔ Old `actions/attest-sbom-action/` folder absent; `git log --follow` walks through the rename (5 commits pre-rename visible).
4. ✔ No tracked file in `.github/`, `scripts/`, `docs/`, or root contains `actions/attest-sbom-action` — `.planning/` historical context files allowlisted.
5. ✔ `.planning/REQUIREMENTS.md` §Ext EXT-02 and EXT-03 rewritten per CONTEXT.md §Specifics D-02 verbatim.
6. ✔ All 4 commits landed: `fix(88-01):`, `docs(88-01):`, `chore(88-01):`, `docs(88-01):` — atomic per logical step.
7. ✔ Plan 02 (ci.yml changes) and Plan 03 (external op) unblocked; Plan 04 (website sweep) independent.

## Deviations from Plan

### [Rule 1 - Plan-internal inconsistency] Over-specified acceptance criterion for Task 4

- **Found during:** Task 4 assertion battery
- **Issue:** The plan's Task 4 acceptance criterion listed `grep -c 'sealedge-attest-sbom-action' .planning/REQUIREMENTS.md` returns **at least 2** ("EXT-02 explicit name + EXT-03 explicit name"). The verbatim CONTEXT.md §Specifics D-02 replacement text quoted inside the same plan contains the literal string `sealedge-attest-sbom-action` **only once** (in EXT-02's `TrustEdge-Labs/sealedge-attest-sbom-action` repo-rename phrase). EXT-03 intentionally references the **OLD** name `attest-sbom-action@v1` to explain what the 301 redirect covers.
- **Resolution:** Delivered the verbatim CONTEXT.md D-02 text (authoritative source wins over the over-specified acceptance-criterion count). All other Task 4 assertions pass: `marketplace listing marked deprecated` → 0, `GitHub's built-in 301 redirect covers existing` → 1, `pre-rebrand @v1 tag stays frozen` → 1, EXT-01/EXT-04/VALID-01 each appear exactly 1× (spot-check confirms only EXT-02/03 lines were touched).
- **Files modified:** None beyond the intended 2-line swap.
- **Commit:** `172dc77` (Task 4) — diff is exactly 2 insertions + 2 deletions as the plan specified elsewhere in the `<verify>` block.

No other deviations. Tasks 1-3 executed exactly as written. No authentication gates encountered. No checkpoints encountered.

## Pointers Forward

- **Plan 02 (`88-02-PLAN.md`)** — extends `.github/workflows/ci.yml`'s self-attest release job to upload `seal` + `seal.sha256` (D-12) and converts the inline attest steps into a `uses: TrustEdge-Labs/sealedge-attest-sbom-action@v2` reference (D-14 dogfood). Unblocked by this plan's content + rename landing.
- **Plan 03 (`88-03-PLAN.md`)** — executes the external `gh repo rename sealedge-attest-sbom-action -R TrustEdge-Labs/attest-sbom-action` op + cuts `@v2.0.0` + floating `@v2` tag on the renamed action repo + checks Marketplace listing refresh (D-01, D-04, D-10, D-11). Depends on Plan 01's monorepo folder rename + content sweep being on `main` so the post-rename action source tree is coherent.
- **Plan 04 (`88-04-PLAN.md`)** — cross-repo website sweep (`TrstVerifier.tsx` → `SealVerifier.tsx` rename, product-name text updates across 19+ TSX + 3 metadata files in `/home/john/vault/projects/github.com/trustedgelabs-website`). **Independent** of Plans 02/03 — can run in parallel with Plan 02 (same wave) since it touches a different repo entirely.

## Self-Check: PASSED

Claims verified on disk + in git:

- ✔ `actions/sealedge-attest-sbom-action/action.yml` — FOUND
- ✔ `actions/sealedge-attest-sbom-action/README.md` — FOUND
- ✔ `actions/sealedge-attest-sbom-action/LICENSE` — FOUND
- ✔ `actions/attest-sbom-action/` — NOT present (expected)
- ✔ Commit `b511b47` (Task 1) — FOUND in `git log`
- ✔ Commit `d2f676d` (Task 2) — FOUND in `git log`
- ✔ Commit `cd28001` (Task 3) — FOUND in `git log`
- ✔ Commit `172dc77` (Task 4) — FOUND in `git log`
- ✔ T2 mitigation — `grep -c 'sha256sum.*runner\.temp.*/seal' actions/sealedge-attest-sbom-action/action.yml` = 1
- ✔ T4 mitigation — `grep -c 'seal\" keygen' actions/sealedge-attest-sbom-action/action.yml` = 1
