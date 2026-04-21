---
phase: 88-external-action-product-website
plan: 03
subsystem: infra
tags: [rebrand, external-operation, gh-repo-rename, release-tag, verification, phase-88, marketplace-deferred]

# Dependency graph
requires:
  - phase: 88-external-action-product-website
    provides: "Plan 01 (action source rewrite + folder rename + REQUIREMENTS amendment); Plan 02 (ci.yml seal binary upload + dogfood conversion to @v2)"
  - phase: 87-github-repository-rename
    provides: "The rename-in-place pattern (gh repo rename + 301 redirect verification) reused here; pre-rename gate checks adapted to the action repo"
provides:
  - "Renamed external action repo: TrustEdge-Labs/attest-sbom-action → TrustEdge-Labs/sealedge-attest-sbom-action (gh repo rename, D-01)"
  - "GitHub 301 redirect verified (curl 301 + follow-redirect 200 + gh api metadata cross-check, D-09)"
  - "@v2.0.0 + floating @v2 tags cut on renamed external repo (D-04, D-11) — both deref to commit e13547cb on the rebrand-baseline commit"
  - "@v1 tag mechanically preserved (frozen per D-07 — not force-pushed, not deleted, not re-tagged)"
  - "T5 mitigation verified: only action.yml, LICENSE, README.md pushed to external repo (3 files, byte-for-byte match to monorepo source)"
  - "88-VERIFICATION.md finalized with 9 sections (pre-rename gate, rename op, D-09 redirect, T1 asset integrity, content push + tag cut, Marketplace DEFERRED, rollback, EXT-02/EXT-03 status table, cross-repo website sweep)"
  - "Formal deferral of Marketplace publication — user-driven decision; out of scope per amended EXT-02 / EXT-03"
affects: [phase-89, phase-90, future-action-release-runs]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Fresh-clone-and-copy external-repo push mechanism (T5 mitigation pattern for cross-repo pushes)"
    - "External-op user-driven execution with verbatim command + rollback in PLAN.md (Phase 87 D-05 carry-forward)"
    - "Formal deferral-with-rationale pattern: user overrides out-of-scope work, captured verbatim in VERIFICATION.md with discovery evidence + forward-pointer"

key-files:
  created:
    - ".planning/phases/88-external-action-product-website/88-03-SUMMARY.md"
  modified:
    - ".planning/phases/88-external-action-product-website/88-VERIFICATION.md"

key-decisions:
  - "Marketplace publication formally DEFERRED per user (out-of-scope per amended EXT-02). v2.0.0 GitHub Release stays in place; only the Marketplace listing publication step is deferred indefinitely."
  - "Amended EXT-02/EXT-03 wording (Plan 01 D-02 amendment, commit 172dc77) is the source-of-truth for success criteria; ROADMAP.md Phase 88 goal text carries pre-amendment wording and is logged as a deferred finding."
  - "Task 4 used an isolated fresh-clone-and-copy mechanism (T5 mitigation): mktemp -d → gh repo clone → wipe → cp -a monorepo/actions/sealedge-attest-sbom-action/. → commit → push. Mechanically impossible to leak monorepo files."
  - "Task 3 ran the D-09 curl 301 check without needing the 30-second retry buffer — first call returned the expected redirect line."

patterns-established:
  - "External-repo cross-push T5 mitigation: fresh clone + wipe + dot-suffix cp = zero monorepo leakage risk"
  - "User-driven external op with structured resume signals (approved, renamed, pushed-and-tagged, marketplace-skip) at each checkpoint"
  - "Deferral-with-full-rationale: when user overrides a plan's out-of-scope work, VERIFICATION.md captures verbatim quote + discovery evidence + forward-pointer + what-stays-in-place, so the decision is auditable indefinitely"

requirements-completed: [EXT-02, EXT-03]

# Metrics
duration: ~2h 15m (across multiple checkpoint sessions — pre-rename gate, rename op, content push + tag cut, Marketplace deferral, VERIFICATION.md finalization)
completed: 2026-04-21
---

# Phase 88 Plan 03: External Action Repo Rename + @v2 Tag Cut (Marketplace Deferred) Summary

**External repo renamed to TrustEdge-Labs/sealedge-attest-sbom-action, rebrand baseline pushed via T5-isolated clone mechanism, @v2.0.0 + floating @v2 tags cut, @v1 frozen, 301 redirect verified end-to-end — Marketplace publication formally deferred per user.**

## Performance

- **Duration:** ~2h 15m (multi-session due to 5 user-gated checkpoints)
- **Started:** 2026-04-21 (earlier session)
- **Completed:** 2026-04-21T22:00Z (this session — wrap-up after `marketplace-skip` signal)
- **Tasks:** 5 (all gated; all resumed successfully; final task closed with deferral decision)
- **Files modified:** 1 in-monorepo (`88-VERIFICATION.md`) + 1 in-external-repo (`action.yml` + `LICENSE` + `README.md` as a single commit — 3 files, new commit on renamed external repo main)

## Accomplishments

- External repo `TrustEdge-Labs/attest-sbom-action` renamed in-place to `TrustEdge-Labs/sealedge-attest-sbom-action` via `gh repo rename` (D-01). User executed from their shell per Phase 87 D-05 pattern.
- GitHub 301 redirect verified end-to-end: `curl -I` returns `301 -> .../sealedge-attest-sbom-action`; `curl -L` follow-redirect returns `200` at the new URL. T6 threat mitigated (rename redirect survives `uses:` resolution).
- Rebrand baseline content pushed to external repo's `main` using the T5-mitigation isolated-clone mechanism (mktemp → clone → wipe → copy → commit → push). External repo tree contains exactly 3 files (`action.yml`, `LICENSE`, `README.md`) — zero monorepo leakage. Byte-diff against monorepo source returns empty (identical).
- `@v2.0.0` + floating `@v2` tags cut on external repo; both deref to the rebrand-baseline commit `e13547cb…`. `@v1` tag mechanically untouched (still deref's to pre-rebrand commit `a32907fe…`) — D-07 frozen-@v1 discipline preserved.
- T1 mitigation resolved: action repo has no pre-existing GitHub Release objects (bare tags only); T1 "release assets remain accessible post-rename" threat is N/A.
- `88-VERIFICATION.md` finalized with 9 sections capturing all evidence artifacts plus the rollback command verbatim.
- Marketplace publication formally deferred per user decision — not required by amended EXT-02, not blocking plan close.

## Task Commits

Plan 03 commits (landed across multiple user sessions):

1. **Task 1 (Pre-rename gate — `approved`):** No commit (checkpoint gate only; user-visible check output)
2. **Task 2 (`gh repo rename` — `renamed`):** No in-monorepo commit (external GitHub operation; evidence captured in VERIFICATION.md §2)
3. **Task 3 (D-09 redirect + T1 asset check — `auto`):** No in-monorepo commit (evidence-only; captured in VERIFICATION.md §3 + §4)
4. **Task 4 (Content push + @v2 tag cut — `pushed-and-tagged`):** No in-monorepo commit (external repo gains one commit `e13547cb…` on its main; evidence captured in VERIFICATION.md §5)
5. **Task 5 (Marketplace check + VERIFICATION.md write — `marketplace-skip`):** `cc60fd6` — `docs(88-03): finalize verification — Marketplace deferred per user (out of amended EXT-02 scope)`

**Plan is predominantly checkpoint-gated** — 4 of 5 tasks are user-gated external ops with no in-monorepo commits. The only in-monorepo artifact from Plan 03 is the VERIFICATION.md file, committed as `cc60fd6`.

## Files Created/Modified

**In-monorepo:**
- `.planning/phases/88-external-action-product-website/88-VERIFICATION.md` — full 9-section evidence file; finalized in commit `cc60fd6`
- `.planning/phases/88-external-action-product-website/88-03-SUMMARY.md` — this file (commit pending)

**External repo `TrustEdge-Labs/sealedge-attest-sbom-action`:**
- New commit `e13547cb…` on `main` containing `action.yml`, `LICENSE`, `README.md` (byte-for-byte match to monorepo `actions/sealedge-attest-sbom-action/`)
- New annotated tag `v2.0.0` (tag object `e2ce46e1`, deref → `e13547cb`)
- New annotated tag `v2` (tag object `de64d58e`, deref → `e13547cb`)
- `v1` and `v1.0.1` tags untouched (still deref → `a32907fe`)

## Decisions Made

**1. Marketplace publication formally DEFERRED (Task 5 — user-driven).**

Rationale (user's verbatim quote, captured in VERIFICATION.md §6):

> "I didn't ask for anything in the marketplace, not sure where that came from. there are no users and the verify service is just PoC/experimental for now."

User's reasoning is correct: the amended EXT-02 (REQUIREMENTS.md, Plan 01 D-02 amendment commit `172dc77`) does NOT require Marketplace publication. The Marketplace check came from CONTEXT.md D-10, which was framed as checking the *existing* portfolio-polish listing post-rename. Discovery during this session showed:

- `curl -I https://github.com/marketplace/actions/sealedge-sbom-attestation` → 404 (Marketplace listing never existed under the new slug)
- `gh release list -R TrustEdge-Labs/sealedge-attest-sbom-action` shows `v2.0.0` Release exists, but the Release's "Publish this Action to the GitHub Marketplace" checkbox was never checked
- Solo-dev context, PoC/experimental product status, zero users → Marketplace publication is pure portfolio polish, not a functional requirement

`uses: TrustEdge-Labs/sealedge-attest-sbom-action@v2` continues to resolve correctly via the repo tag (§5 evidence); Marketplace listing is for discovery, not for resolution. Deferred indefinitely — revisit if/when product status changes.

**2. v2.0.0 GitHub Release stays in place.**

The Release object (not the Marketplace listing — different concepts) is appropriate documentation in the Releases tab and shows up on the repo's Releases page. Only the Marketplace listing publication step is deferred.

**3. ROADMAP.md inconsistency logged as deferred finding (not fixed in Plan 03).**

`.planning/ROADMAP.md` lines 222-229 describe Phase 88 using pre-amendment wording (mentions "published to the GitHub Marketplace", "old listing marked deprecated", "README redirects readers to the new listing"). This conflicts with:
- D-01 rename-in-place (no "old action" / "old listing" — it's the same repo, just renamed)
- §6 Marketplace deferral (not published)

REQUIREMENTS.md (the true source-of-truth) has the amended wording from commit `172dc77`. ROADMAP.md was not touched during the D-02 amendment commit. One-line ROADMAP.md fix deferred to Phase 88 close (orchestrator step) or Phase 89 cleanup — not a Plan 03 blocker.

## Deviations from Plan

### Gate-driven modifications to Task 5 scope

**1. [Rule 4 — Architectural] Task 5 Marketplace check replaced with formal deferral**

- **Found during:** Task 5 (Marketplace listing check)
- **Issue:** CONTEXT.md D-10 and PLAN.md Task 5 expected a Marketplace listing visual check. Investigation revealed the Marketplace listing was never actually published — `https://github.com/marketplace/actions/sealedge-sbom-attestation` returned 404; the `v2.0.0` Release's "Publish to Marketplace" checkbox was never checked. The amended EXT-02/EXT-03 wording (REQUIREMENTS.md post-`172dc77`) does not require Marketplace publication. User was presented with options at the checkpoint and responded `marketplace-skip — defer`.
- **Fix:** §6 of VERIFICATION.md rewritten from pending-placeholder to DEFERRED status with user's verbatim rationale, discovery evidence, "what stays in place" clarification (Release stays; only Marketplace listing deferred), and forward-pointer (revisit post-PoC).
- **Files modified:** `.planning/phases/88-external-action-product-website/88-VERIFICATION.md` (§6 rewritten, §8 status table confirmed, Deferred Findings extended)
- **Verification:** `gh api repos/TrustEdge-Labs/sealedge-attest-sbom-action` confirms repo is at new name with correct metadata (except repo-description field — tracked separately); `uses: @v2` resolution works via repo tag independent of Marketplace listing.
- **Committed in:** `cc60fd6` (Task 5 VERIFICATION.md finalization commit)

This is a Rule 4 "architectural" change because it redefines a task's out-of-scope boundary based on user judgment. It was handled via checkpoint escalation (Task 5 original resume signals included `marketplace-ok | marketplace-republished | marketplace-broken | marketplace-skip`), so the deferral path was pre-authorized by the plan's structure — not a deviation from the plan's *structure*, but a deviation from the plan's *expected outcome* (which anticipated a green auto-refresh check).

---

**Total deviations:** 1 (scope-boundary redefinition via user checkpoint)
**Impact on plan:** Cleaner than anticipated — the deferral path was pre-authorized by the checkpoint structure, and the amended REQUIREMENTS.md already supports the decision. No scope creep; no blocking issues.

## Issues Encountered

**1. Stashed WIP on main branch at Plan 03 start.**

The user had in-flight edits to `web/demo/index.html`, `web/verify/index.html`, and `crates/experimental/pubky/tests/integration_tests.rs` (straggler rebrand edits + gsd-sdk state/config noise) that were stashed before Plan 03's pre-rename gate could pass. Stash entry: `stash@{0}: On main: phase-88-plan-03-gate: straggler rebrand edits …`. The stash will be restored post-plan-close (see "Next Phase Readiness").

**2. Three distinct tag objects for what could have been one (§5 observation).**

User's tag-cut sequence (`git tag -a v2.0.0 -m ...` then `git tag -f v2 v2.0.0` followed by two separate pushes) resulted in `refs/tags/v2` and `refs/tags/v2.0.0` pointing at *different tag annotation objects* that both deref to the same commit. Not a bug — consistent with GitHub's floating-tag semantics and does not affect `uses: @v2` resolution. Documented in VERIFICATION.md Deferred Finding #4 for future reference.

## User Setup Required

None - no external service configuration required by this plan.

(External operations were limited to the `gh repo rename` + external-repo content push + tag cut, all executed during the checkpoint gates in Task 2 and Task 4 by the user. No lingering environment variables, secrets, or dashboard config needed.)

## Next Phase Readiness

**Ready:**

- Phase 88 Plans 01, 02, 03, 04 all closed. Phase 88 is ready for orchestrator close once STATE.md and ROADMAP.md checkboxes are updated.
- EXT-02 and EXT-03 requirements fully satisfied under the amended wording (`docs(88-01): amend EXT-02/EXT-03 to match rename-in-place approach`, commit `172dc77`).
- EXT-04 satisfied via Plan 04 (website sweep), per 88-04-SUMMARY.md.
- External action repo is live at https://github.com/TrustEdge-Labs/sealedge-attest-sbom-action with @v2.0.0 + @v2 tags; 301 redirect from the old URL is verified; `uses: TrustEdge-Labs/sealedge-attest-sbom-action@v2` is ready to be exercised by Phase 89's first v6.0.0 release run.

**Forward-pointers:**

- **Phase 89 (Final Validation):** Phase 89's first green post-v6.0.0 release run proves end-to-end function of the dogfooded `@v2` action (D-11, D-14). If that run succeeds, @v2 is implicitly smoke-tested in production.
- **Phase 88 close (orchestrator):** ROADMAP.md Phase 88 goal text + success criterion #1 need a one-line amendment to match the amended EXT-02/EXT-03 (pre-amendment wording mentions "published to the GitHub Marketplace" / "old listing marked deprecated"; both conflict with the rename-in-place approach + Marketplace deferral). Tracked as VERIFICATION.md Deferred Finding #3.
- **Post-plan-close (this session):** Attempted `git stash pop` to restore user's stashed WIP (straggler rebrand edits on `web/demo/`, `web/verify/`, and `crates/experimental/pubky/`). **Pop failed with merge conflict on `.claude/settings.local.json`** — the stashed version would overwrite uncommitted working-tree changes in that file. **The stash entry is preserved** (`stash@{0}`) — no data loss. Resolve manually when convenient:
  1. Decide whether to keep the working-tree `.claude/settings.local.json` or the stashed version (`git stash show -p stash@{0} -- .claude/settings.local.json`)
  2. Either commit or discard the working-tree `.claude/settings.local.json`, then re-run `git stash pop`
  3. The stash contains the intended straggler rebrand edits — restoring it is worth doing before Phase 89 picks up.

**Deferred (not blocking):**

1. Repo description field on `TrustEdge-Labs/sealedge-attest-sbom-action` still reads "GitHub Action for SBOM attestation with TrustEdge" — one-line `gh repo edit --description "..."` fix; bundle with other cleanup sweeps.
2. Marketplace publication — deferred indefinitely per user; revisit post-PoC.
3. ROADMAP.md Phase 88 goal text / success-criterion wording — one-line amendment; orchestrator or Phase 89 cleanup.
4. @v2 tag semantics (two distinct tag objects deref'ing to same commit) — cosmetic only; no action needed.

---
*Phase: 88-external-action-product-website*
*Plan: 03 (external rename + @v2 tag cut + Marketplace deferral + VERIFICATION.md)*
*Completed: 2026-04-21*

## Self-Check: PASSED

- `88-VERIFICATION.md` exists — verified via file-stat
- `88-03-SUMMARY.md` exists — verified via file-stat (this file)
- `cc60fd6` commit exists in git log (`docs(88-03): finalize verification — Marketplace deferred per user ...`) — verified via `git log --all | grep cc60fd6`
- `172dc77` commit (Plan 01 REQUIREMENTS amendment) exists in git log — verified via `git log --all | grep 172dc77`
- VERIFICATION.md §6 contains "DEFERRED" + user's verbatim quote — verified via `Read`
- VERIFICATION.md §8 status table shows ✔ for all EXT-02 / EXT-03 criteria — verified via `Read`
- VERIFICATION.md Deferred Findings section includes Marketplace deferral (#2), repo-description field (#1), ROADMAP.md wording inconsistency (#3) — verified via `Read`
