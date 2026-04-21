# Phase 88: External Action & Product Website - Context

**Gathered:** 2026-04-21
**Status:** Ready for planning

<domain>
## Phase Boundary

Align Sealedge's external distribution surface with the v6.0 rebrand — the GitHub Action and the product references on `trustedgelabs.com` — using the same clean-break rename-in-place pattern that Phase 87 applied to the monorepo. Solo dev, no production consumers, so GitHub's built-in 301 redirect is the one compatibility mechanism.

**In scope:**
- Rename `TrustEdge-Labs/attest-sbom-action` → `TrustEdge-Labs/sealedge-attest-sbom-action` in-place via `gh repo rename` (GitHub's 301 redirect covers existing `uses:` references)
- Update action source in-place to reference sealedge/seal: rename `trst` binary fetch → `seal`, rename `trst-version` input → `seal-version`, update action `name`/`description` metadata, update README product-name references, update release-URL base from `/TrustEdge-Labs/trustedge/releases` → `/TrustEdge-Labs/sealedge/releases`
- Rename in-repo folder `actions/attest-sbom-action/` → `actions/sealedge-attest-sbom-action/` (canonical source-of-truth stays in the sealedge monorepo)
- Cut `@v2` tag (plus floating `@v2`) on the renamed action repo; `@v1` stays frozen as pre-rebrand behavior
- Add short "Renamed from attest-sbom-action" notice near the top of the new README
- Add `seal` + `seal.sha256` upload steps to the existing self-attest release job in `.github/workflows/ci.yml` (so the action has a real upstream artifact to download)
- Convert `.github/workflows/ci.yml`'s self-attest job to use `uses: TrustEdge-Labs/sealedge-attest-sbom-action@v2` (dogfoods the action; Phase 89's first v6.0.0 release run proves end-to-end function)
- Cross-repo: update product-name references in `trustedgelabs-website` (`/home/john/vault/projects/github.com/trustedgelabs-website`) — minimal rename-only swap, including renaming `TrstVerifier.tsx` → `SealVerifier.tsx` and text updates to `WasmDemo.tsx` and other product-referencing components
- Amend REQUIREMENTS.md EXT-02/EXT-03 wording to match the rename-in-place approach (single repo, renamed; no two-repo migration story)
- Verification: `88-VERIFICATION.md` captures (1) `curl -I` output showing 301 redirect for old action URL, (2) Marketplace listing title/description screenshot after rename, (3) grep-clean audit of trustedgelabs-website, (4) live preview screenshot of website post-update

**Out of scope (explicit carve-outs):**
- `TrustEdge-Labs` GitHub org name — stays (v6.0 memory)
- `trustedgelabs.com` domain name — stays (only content updates)
- `trustedgelabs-website` repo name — stays
- Broader website copy refresh (new hero, value-prop rewrite, features overhaul) — reserved for Phase 82 (v5.0 punted, executes post-v6.0)
- Publishing `sealedge-seal-wasm` to npm / swapping the website's WASM demo to import the new package — deferred; website keeps current WASM plumbing structure with updated component names + branding only
- Re-tagging or deleting the old `@v1` tag on the renamed action repo — stays frozen as pre-rebrand behavior
- Cutting `v6.0.0` release tag on the sealedge monorepo — Phase 89 owns this (per v6.0 REQUIREMENTS.md out-of-scope list)
- Full VALID-01/02/03 matrix (feature-matrix CI, WASM + dashboard + Docker end-to-end) — Phase 89 owns this
- Stale `seal.te-attestation.json` extension in `ci.yml` line 212/220 (pre-existing gap from Phase 84 that Phase 88 may touch incidentally when extending the release-upload step, but not a Phase 88 deliverable by itself)

</domain>

<decisions>
## Implementation Decisions

### New action: repo rename & versioning

- **D-01:** **Rename in-place, not two-repo migration.** Execute `gh repo rename sealedge-attest-sbom-action -R TrustEdge-Labs/attest-sbom-action`. GitHub's 301 redirect covers the old URL AND existing `uses: TrustEdge-Labs/attest-sbom-action@v1` references in any consumer workflow. Matches Phase 87's repo-rename pattern and the v6.0 clean-break philosophy — no customers to migrate, so a separate-repo ceremony would be pure overhead.
- **D-02:** **Amend EXT-02 / EXT-03 during planning** to reflect the rename-in-place approach. Current requirement wording ("new repo", "old listing marked deprecated with README redirect to the new listing") was drafted assuming a two-repo migration and no longer matches the execution plan. Update `.planning/REQUIREMENTS.md` in Phase 88 so auditors don't find a mismatch.
- **D-03:** **Renamed repo name: `sealedge-attest-sbom-action`** — direct product-prefix parallel to the old `attest-sbom-action` name. Clearest signal of the rebrand.
- **D-04:** **First post-rename version tag: `@v2`** (plus floating `@v2`). `@v1` already exists on the repo from the pre-rebrand lineage and is kept frozen. `@v2.0.0` + `@v2` ship the rebranded action. Semver signals the user-visible change (repo name, binary name, binary download URL).
- **D-05:** **Behavior parity is 1:1** — same SHA256-optional-with-warning-if-missing logic as `@v1`. EXT-02 explicitly requires "equivalent functionality ... including SHA256 checksum verification." No behavior changes in v6.0 per project_v6_rebrand.md.

### In-monorepo folder layout

- **D-06:** **Rename `actions/attest-sbom-action/` → `actions/sealedge-attest-sbom-action/`** in the sealedge monorepo. Keep as canonical source-of-truth. Action-repo contents pushed from this folder for each release cut. One place to edit.

### Old @v1 tag & README

- **D-07:** **Leave `@v1` frozen.** Zero churn on the pre-rebrand tag — it continues to point at its original commit and fetches `trst` from old trustedge releases (which redirect to sealedge releases and still host pre-v6.0 `trst` artifacts). Users pinned to `@v1` keep working on old behavior; migrating = update the `uses:` repo name AND bump to `@v2`. SHA-pinned users unaffected.
- **D-08:** **Post-rename README carries a short top-of-README migration notice** — 1-3 lines stating the repo was renamed from `attest-sbom-action` as part of v6.0, `@v1` is pre-rebrand behavior, `@v2+` uses sealedge naming, GitHub redirects handle old `uses:` lines. Doesn't dominate the README. No full migration section with before/after examples (over-scoped for solo-dev with no consumers).

### Verification of redirect and Marketplace listing

- **D-09:** **Old `uses: TrustEdge-Labs/attest-sbom-action@v1` resolution is verified via one-shot curl redirect check**, captured in `88-VERIFICATION.md`:
  ```
  curl -I -s -o /dev/null -w '%{http_code} -> %{redirect_url}\n' \
    https://github.com/TrustEdge-Labs/attest-sbom-action
  ```
  Expected: `301 -> https://github.com/TrustEdge-Labs/sealedge-attest-sbom-action`. No need for a live `uses: @v1` workflow run — GitHub's redirect for renamed-repo action references is documented behavior.
- **D-10:** **Marketplace listing check post-rename.** Open the listing URL after the rename, confirm title is "Sealedge SBOM Attestation" (from updated `action.yml name:` field on `@v2`) and the description matches. If the listing didn't auto-refresh, re-publish manually via the GitHub UI (user-only step). Document the check (and any manual re-publish action) in `88-VERIFICATION.md`.

### Tag-timing & release-pipeline changes

- **D-11:** **Cut `@v2` during Phase 88, before Phase 89 cuts v6.0.0.** Phase 88 ships the rebranded `@v2` tag pointing at `latest` releases. `@v2` is not end-to-end functional until Phase 89 cuts v6.0.0 with `seal` binary uploaded. Phase 89's final validation includes the first green release run as implicit smoke test of `@v2`.
- **D-12:** **Phase 88 adds `seal` + `seal.sha256` upload steps to `.github/workflows/ci.yml`'s self-attest release job.** Current job uploads only `*.te-attestation.json` + `build.pub` — no `seal` binary. The new action can't download what isn't there. Extend the existing job (or add a companion step) to `gh release upload` the binary + SHA256 file with `--clobber`. EXT-02 explicitly requires SHA256 checksum verification to be functional, which requires the checksum file to exist in releases.
- **D-13:** **Input rename: `trst-version` → `seal-version`, default `latest`.** Input name matches the renamed binary. `latest` resolves to the most recent sealedge release. Consumers wanting pre-v6.0 behavior can pin to `@v1` (old action tag) or pin `seal-version` explicitly.
- **D-14:** **Convert `ci.yml` self-attest job to use `uses: TrustEdge-Labs/sealedge-attest-sbom-action@v2`.** Currently the job builds and attests inline. Dogfooding the action:
  - Proves `@v2` works end-to-end on the first v6.0.0 release run (Phase 89)
  - Re-establishes the Phase 79/80 self-attestation pattern against the rebranded action
  - Reduces inline workflow code (action encapsulates the attest-sbom logic)
  Evidence captured in `88-VERIFICATION.md`: `ci.yml` diff + first green post-v6.0.0-release run URL.

### Product-page content on trustedgelabs.com (EXT-04)

- **D-15:** **Content scope: minimal product-name swap only.** Find/replace product references ('TrustEdge' → 'Sealedge' when referring to the PRODUCT) in `/home/john/vault/projects/github.com/trustedgelabs-website`. Keep company-brand 'TrustEdge Labs' references intact. Rename `TrstVerifier.tsx` → `SealVerifier.tsx` and update any product-name text in `WasmDemo.tsx`, `Hero.tsx`, `Features.tsx`, `CodeExamples.tsx`, `UseCases.tsx`, `EnterpriseSolutions.tsx`, `Footer.tsx`, `IntegrationGuide.tsx`, `Security.tsx`, `Solution.tsx`, `Problem.tsx`, `PerformanceBenchmarks.tsx`, `TechnicalCapabilities.tsx`, `ArchiveSystem.tsx`, `Contact.tsx`, `GetStarted.tsx`, `PrivacyPolicy.tsx`, `TermsOfService.tsx`, `Thanks.tsx`. No hero-copy refresh, no value-prop rewrite — that's reserved for Phase 82 post-v6.0.
- **D-16:** **WASM demo integration: Phase 88 updates component names + visible text only. WASM package path swap (`trustedge-trst-wasm` → `sealedge-seal-wasm`) is a deferred follow-up.** The package-import swap requires either publishing `sealedge-seal-wasm` to npm OR a local `pkg/` copy mechanism — each its own decision, each outside v6.0's rename-only scope. Current WASM plumbing stays structurally intact; only the text/branding around it updates. Flag as deferred idea in this CONTEXT.md.
- **D-17:** **Cross-repo work organized as a dedicated Phase 88 plan** (tentatively `88-03-PLAN.md` or similar, at planner's discretion). Plan lives in sealedge's `.planning/` but its atomic commits land in `/home/john/vault/projects/github.com/trustedgelabs-website`. Executor operates against both repos. Matches the cross-surface pattern Phase 87 used (local operations + external GitHub operation).
- **D-18:** **EXT-04 verified via grep audit + live preview screenshot**, captured in `88-VERIFICATION.md`:
  1. Grep the trustedgelabs-website repo for stale product-name references with an allowlist for company-brand 'TrustEdge Labs' mentions (mirror Phase 85/86 sweep discipline)
  2. Run `npm run dev` locally (or check the deployed Cloudflare Pages preview URL), screenshot the home page and key components
  Both forms of evidence go into the verification file.

### Claude's Discretion

- **Plan granularity:** Likely 3-4 plans — (a) rename monorepo folder + update action source in-place (`actions/sealedge-attest-sbom-action/`), (b) extend `ci.yml` release job with seal + seal.sha256 upload AND convert self-attest to `uses: sealedge-attest-sbom-action@v2`, (c) execute the action repo rename + cut `@v2` tag + Marketplace check, (d) trustedgelabs-website cross-repo content update. Planner may split or merge as coherent atomic commits allow.
- **EXT-02/03 amendment style:** Planner drafts the reworded requirements in the `REQUIREMENTS.md` update commit. Commit message something like `docs(88): amend EXT-02/03 to match rename-in-place approach`. Atomic, small, separate from the execution commits.
- **Seal binary upload: match existing artifact conventions.** Whatever naming the prior `trst` release artifacts used (e.g., `trst` + `trst.sha256` uncompressed binary, or a platform-specific tarball) — the `seal` upload mirrors that shape. Planner confirms by checking existing release-asset history via `gh release view` during planning.
- **Rename commit sequencing** (mirrors Phase 87 pattern):
  1. Update `actions/attest-sbom-action/` contents in-place to reference sealedge/seal (commit on main, pushed to the still-old-named repo)
  2. Rename monorepo folder `actions/attest-sbom-action/` → `actions/sealedge-attest-sbom-action/` (commit on main)
  3. Update `ci.yml` to upload seal + seal.sha256 AND switch to `uses: TrustEdge-Labs/sealedge-attest-sbom-action@v2` (commit on main)
  4. `gh repo rename sealedge-attest-sbom-action -R TrustEdge-Labs/attest-sbom-action` (external GitHub op, user-driven, not a commit)
  5. Cut `@v2.0.0` tag on the renamed action repo; push floating `@v2`
  6. Update trustedgelabs-website content in the separate repo (parallel atomic commits)
  7. Marketplace check + curl redirect check + grep audit + screenshot → `88-VERIFICATION.md`
- **Recommended rollback** — `gh repo rename attest-sbom-action -R TrustEdge-Labs/sealedge-attest-sbom-action` (same reversibility property as Phase 87's monorepo rename, within GitHub's rename-history window). Kept in PLAN.md verbatim.
- **No release-tag v6.0.0 cut in Phase 88** — per v6.0 REQUIREMENTS.md. The `@v2` action tag is what Phase 88 cuts; the v6.0.0 monorepo release is Phase 89 milestone close.

### Folded Todos

None — no backlog todos match this phase.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Milestone-level (v6.0 scope)
- `.planning/REQUIREMENTS.md` §Ext — EXT-02 / EXT-03 / EXT-04 (amendment to EXT-02/03 wording is a Phase 88 deliverable; see D-02)
- `.planning/REQUIREMENTS.md` §"Out of Scope" — v6.0 release tag NOT cut in Phase 88 (informs D-11)
- `.planning/PROJECT.md` §Current Milestone — v6.0 target, clean-break preference
- `.planning/ROADMAP.md` §"Phase 88: External Action & Product Website" (line 210-218) — goal + 3 success criteria

### Prior v6.0 phase decisions that carry forward
- `.planning/phases/83-crate-and-binary-rename/83-CONTEXT.md` §D-01 — `trst` binary renamed to `seal` (drives `trst-version` → `seal-version` input rename in D-13)
- `.planning/phases/83-crate-and-binary-rename/83-CONTEXT.md` §D-02 — `.trst` extension → `.seal` (parallel to binary rename)
- `.planning/phases/84-crypto-constants-file-extension/84-CONTEXT.md` — `.te-attestation.json` → `.se-attestation.json` file extension (informs action output-path naming)
- `.planning/phases/85-code-sweep-headers-text-metadata/85-CONTEXT.md` — Phase 85 sweep completed product-name strings in workspace code; action source under `actions/attest-sbom-action/` was explicitly carved out (Phase 88 owns it)
- `.planning/phases/86-documentation-sweep/86-CONTEXT.md` §D-06a — `actions/attest-sbom-action/**` explicit Phase 88 carve-out
- `.planning/phases/87-github-repository-rename/87-CONTEXT.md` — rename-in-place pattern (monorepo); same `gh repo rename` + GitHub-301-redirect mechanism reused in D-01
- `.planning/phases/87-github-repository-rename/87-CONTEXT.md` §Out of Scope — `actions/attest-sbom-action/**` deferred to Phase 88 (this phase)

### Memory / user context
- `~/.claude/projects/-home-john-vault-projects-github-com-trustedge/memory/project_v6_rebrand.md` — GitHub Action rename is part of v6.0; solo dev with no customers; company brand 'TrustEdge Labs' stays but product is Sealedge
- `~/.claude/projects/-home-john-vault-projects-github-com-trustedge/memory/feedback_clean_break_compat.md` — no backward-compat shims; rename-in-place is a clean break backed by GitHub's auto-redirect (not a dual-repo maintenance path)

### Source files touched by Phase 88
- `actions/attest-sbom-action/action.yml` — composite action YAML; renames `trst` binary fetch URL, input name `trst-version`, action `name`/`description`, output file extension
- `actions/attest-sbom-action/README.md` — product-name refs, example YAML snippets (`uses: TrustEdge-Labs/attest-sbom-action@v1` → `TrustEdge-Labs/sealedge-attest-sbom-action@v2`), verify.trustedge.dev → verify.sealedge.dev if applicable, links to TrustEdge repo → sealedge repo
- `.github/workflows/ci.yml` lines 171-222 — self-attest release job; extend with `seal`/`seal.sha256` upload steps AND convert the attest step to use `uses: TrustEdge-Labs/sealedge-attest-sbom-action@v2`

### Cross-repo source (trustedgelabs-website)
- `/home/john/vault/projects/github.com/trustedgelabs-website/src/App.tsx` — app entry point; check for product-name references
- `/home/john/vault/projects/github.com/trustedgelabs-website/src/components/TrstVerifier.tsx` — rename file to `SealVerifier.tsx`; update imports
- `/home/john/vault/projects/github.com/trustedgelabs-website/src/components/WasmDemo.tsx` — text/branding updates only; WASM package import swap is deferred (D-16)
- `/home/john/vault/projects/github.com/trustedgelabs-website/src/components/{Hero,Features,Footer,Problem,Solution,UseCases,EnterpriseSolutions,IntegrationGuide,Security,CodeExamples,ArchiveSystem,PerformanceBenchmarks,TechnicalCapabilities,Contact,GetStarted,Thanks,PrivacyPolicy,TermsOfService}.tsx` — product-name text updates per D-15 grep-driven sweep
- `/home/john/vault/projects/github.com/trustedgelabs-website/index.html` — title / meta tags / any product-name refs
- `/home/john/vault/projects/github.com/trustedgelabs-website/README.md` — product-name refs
- `/home/john/vault/projects/github.com/trustedgelabs-website/package.json` — description / keywords (if applicable)

### External service docs
- GitHub "About repository renaming" (docs.github.com) — confirms `gh repo rename` preserves issues/PRs/tags/releases, creates 301 redirect for old URL, AND redirects `uses:` action references to the renamed repo (verifies D-01/D-09 assumptions)
- GitHub Actions Marketplace — listing auto-refreshes when repo is renamed and when `action.yml` `name:`/`description:` fields change on the latest tag (verifies D-10; manual re-publish is the fallback)

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- **Phase 87's `gh repo rename` + verification pattern** — exact same command shape reused here (`gh repo rename <new> -R <owner>/<old>`), same curl-301 check for redirect, same 30-second retry buffer if first curl returns unexpected output.
- **Existing `actions/attest-sbom-action/action.yml` composite workflow** — already implements SHA256 verification (lines 49-62), ephemeral keygen path (lines 64-73), persistent key path (lines 75-82), and attest step (lines 84-98). Phase 88 edits these in-place; does not rewrite the action from scratch.
- **`anchore/sbom-action@v0.24.0` for SBOM generation** — used in `ci.yml` line 191; the new `sealedge-attest-sbom-action@v2` continues to document this composition pattern in its README (same example structure as old README, just with updated `uses:` lines).

### Established Patterns
- **Clean-break rename-in-place** (v6.0, carried from Phase 87) — single-repo rename backed by GitHub's 301 redirect, no dual-maintenance path, no deprecation-of-separate-old-repo ceremony. Applied to the action here for the same reason it applied to the monorepo there: solo dev, no production users.
- **Atomic commit per logical step** (Phase 83-87 carry-forward) — content update, folder rename, ci.yml changes, external rename, tag cut, cross-repo website updates each land as their own commit. Verification evidence in a single `88-VERIFICATION.md` at phase close.
- **Grep-clean audit with allowlist** (Phase 85/86/87 pattern) — post-update sweep for `trustedge` references on trustedgelabs-website mirrors the monorepo's repo-wide grep-with-allowlist audit. Allowlist retains company-brand "TrustEdge Labs" mentions; catches anything else.

### Integration Points
- **Marketplace listing ↔ action repo `action.yml` `name:` + `description:` fields** — listing title and description come from the latest tag's `action.yml` metadata. Updating `name: 'Sealedge SBOM Attestation'` and bumping the description on `@v2` should auto-refresh the listing. D-10 verifies; manual re-publish is the fallback.
- **ci.yml self-attest job ↔ sealedge-attest-sbom-action@v2** — converting the inline attest logic to `uses: TrustEdge-Labs/sealedge-attest-sbom-action@v2` requires the action to exist AND a `seal` binary to be in releases. Both are Phase 88 deliverables (via D-11, D-12). First green release run after v6.0.0 is cut (Phase 89) proves the integration.
- **trustedgelabs-website WasmDemo ↔ WASM crate distribution** — current website likely imports a `trustedge-trst-wasm` artifact. Phase 88 defers the actual package-import swap (D-16); only text/branding in `WasmDemo.tsx` updates. Package plumbing is a post-v6.0 follow-up.
- **Company-brand 'TrustEdge Labs' vs product-name 'TrustEdge'** — the website has both; Phase 88 sweep must distinguish these (company stays, product renames). Grep allowlist pattern per D-18.

</code_context>

<specifics>
## Specific Ideas

- **Exact post-rename curl audit** (D-09 verification):
  ```
  curl -I -s -o /dev/null -w '%{http_code} -> %{redirect_url}\n' \
    https://github.com/TrustEdge-Labs/attest-sbom-action
  ```
  Expected: `301 -> https://github.com/TrustEdge-Labs/sealedge-attest-sbom-action`. 30-second retry if first call returns unexpected output (GitHub edge cache lag pattern from Phase 87).

- **Rollback command kept in PLAN.md verbatim** — `gh repo rename attest-sbom-action -R TrustEdge-Labs/sealedge-attest-sbom-action`. Copy-pasteable. Useful if D-09 redirect check fails.

- **Pre-rename gate checks** (reuse Phase 87 D-06 pattern adapted for the action repo):
  1. `git status` clean on the sealedge monorepo main; straggler commits (D-15 in-place content update + D-16 folder rename + D-17 ci.yml changes) pushed
  2. No open PRs on the old `attest-sbom-action` repo (`gh pr list -R TrustEdge-Labs/attest-sbom-action --state open`)
  3. `gh auth status` shows `repo` scope active (already verified in Phase 87)

- **Grep audit query for trustedgelabs-website** (D-18 verification):
  ```
  git -C /home/john/vault/projects/github.com/trustedgelabs-website ls-files \
    | xargs grep -n "TrustEdge\|trustedge\|TRUSTEDGE" 2>/dev/null \
    | grep -vE "TrustEdge Labs|TrustEdge-Labs|trustedgelabs\.com|node_modules|package-lock"
  ```
  Expected: zero results outside the allowlist after cross-repo plan lands.

- **Seal binary upload step** (D-12) — append to existing `ci.yml` self-attest release job (line 215-222 area). Companion commands:
  ```
  sha256sum ./target/release/seal | awk '{print $1 "  seal"}' > seal.sha256
  gh release upload "${{ github.ref_name }}" \
    ./target/release/seal \
    seal.sha256 \
    --clobber
  ```

- **EXT-02/03 reworded language suggestion** (D-02 guidance to planner):
  - EXT-02 → "The TrustEdge-Labs/attest-sbom-action repo is renamed to TrustEdge-Labs/sealedge-attest-sbom-action via `gh repo rename`; action source references sealedge/seal; a new `@v2` tag ships the rebranded action; SHA256 checksum verification of the downloaded binary is preserved"
  - EXT-03 → "GitHub's built-in 301 redirect covers existing `uses: TrustEdge-Labs/attest-sbom-action@v1` references; the pre-rebrand `@v1` tag stays frozen; the post-rename README carries a short notice pointing readers to `@v2` and the renamed repo"

</specifics>

<deferred>
## Deferred Ideas

- **WASM package publishing (`sealedge-seal-wasm` to npm)** — prerequisite to updating the website's WasmDemo to import the new package name directly. Currently the website likely uses a local-pkg copy or the old `trustedge-trst-wasm` name. Deferred to post-v6.0 — its own decision tree (npm publish ceremony, scoped package naming, version cadence).
- **WasmDemo.tsx package-import swap on trustedgelabs-website** — D-16 defers this. Once WASM publishing is decided, swap the imports. Flag as v7.0 or post-v6.0 polish.
- **Broader trustedgelabs.com copy refresh** — new hero copy, updated value-prop, feature section rewrite. This is Phase 82 territory (v5.0 punted, executes post-v6.0 per v6.0 memory).
- **Demo GIF re-record on the rebranded product** — Phase 81 territory (v5.0 punted, executes post-v6.0).
- **Publishing sealedge crates to crates.io** — out-of-scope per v6.0 REQUIREMENTS.md; separate post-v6.0 decision.
- **Stale `seal.te-attestation.json` file extension in ci.yml lines 212/220** — pre-existing Phase 84 cleanup gap. Phase 88 may touch incidentally while editing ci.yml for D-12/D-14, or may leave it for Phase 89's VALID-03 sweep. Noted to avoid surprise during planning.
- **v6.0.0 monorepo release tag cutting** — Phase 89 milestone close.
- **Permanent CI guard against stale `trustedge` product-name references on trustedgelabs-website** — not required; one-time grep (D-18) is sufficient. Add as backlog if a repeating drift pattern emerges.

### Reviewed Todos (not folded)
None — no todos were considered for this phase.

</deferred>

---

*Phase: 88-external-action-product-website*
*Context gathered: 2026-04-21*
