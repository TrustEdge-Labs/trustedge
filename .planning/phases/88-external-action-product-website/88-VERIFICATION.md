<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: sealedge — Privacy and trust at the edge.
-->

# Phase 88 Verification — External Action & Product Website

**Phase:** 88-external-action-product-website
**Requirements:** EXT-02, EXT-03, EXT-04
**Date:** 2026-04-21
**Status:** PASS — Plan 03 (action repo rename + @v2 tag cut) + Plan 04 (website sweep) both complete. Rollback not executed. Marketplace publication (§6) formally DEFERRED per user — out of scope per amended EXT-02.

---

## 1. Pre-Rename Gate (Plan 03 Task 1)

All 6 pre-rename gate checks verified green before `gh repo rename` was run.

| # | Check | Evidence |
|---|-------|----------|
| 1 | Working tree clean + `main` pushed | `git status` → clean on main; `git log origin/main..HEAD` → empty |
| 2 | Plan 01 + Plan 02 commits landed on `origin/main` | 6+ Phase 88 commits visible via `git log --oneline origin/main -10 \| grep -E '(fix\|docs\|chore\|feat\|refactor)\(88\):'` (action.yml sweep, README sweep, folder rename, REQUIREMENTS amendment, seal binary upload, dogfood conversion) |
| 3 | Monorepo folder at new name | `actions/sealedge-attest-sbom-action/` exists; `actions/attest-sbom-action/` does not |
| 4 | No open PRs on action repo | `gh pr list -R TrustEdge-Labs/attest-sbom-action --state open` → empty |
| 5 | `gh auth status` shows `repo` scope + HTTPS | Verified — same auth context used in Phase 87 |
| 6 | Content-integrity greps on monorepo side | `ci.yml` references `sealedge-attest-sbom-action@v2`; `action.yml` contains `seal.sha256` (≥2 occurrences); `README.md` contains "Renamed from" notice |

User approved with `approved` resume signal.

---

## 2. Rename Operation (D-01, Plan 03 Task 2)

Command (user-executed from their shell — not Claude's Bash tool, per CONTEXT.md "Claude's Discretion" step 4 / Phase 87 D-05 pattern):

```
gh repo rename sealedge-attest-sbom-action -R TrustEdge-Labs/attest-sbom-action
```

**Output:**

```
✓ Renamed repository TrustEdge-Labs/sealedge-attest-sbom-action
```

**Timestamp:** 2026-04-21 (immediately following Plan 03 Task 1 gate pass)

User confirmed with `renamed` resume signal and pasted the `gh` success output into the conversation.

---

## 3. D-09 Redirect Verification (Plan 03 Task 3)

**Command (CONTEXT.md §Specifics verbatim):**

```
curl -I -s -o /dev/null -w '%{http_code} -> %{redirect_url}\n' https://github.com/TrustEdge-Labs/attest-sbom-action
```

**Output:**

```
301 -> https://github.com/TrustEdge-Labs/sealedge-attest-sbom-action
```

(First call; no 30-second retry needed.)

**Follow-redirect cross-check:**

```
curl -s -o /dev/null -w '%{http_code} (final URL: %{url_effective})\n' -L https://github.com/TrustEdge-Labs/attest-sbom-action
```

**Output:**

```
200 (final URL: https://github.com/TrustEdge-Labs/sealedge-attest-sbom-action)
```

**New URL reachability cross-check:**

```
curl -I -s -o /dev/null -w '%{http_code}\n' https://github.com/TrustEdge-Labs/sealedge-attest-sbom-action
```

**Output:** `200`

**gh api metadata cross-check:**

```
gh api repos/TrustEdge-Labs/sealedge-attest-sbom-action --jq '.full_name, .default_branch'
```

**Output:**

```
TrustEdge-Labs/sealedge-attest-sbom-action
main
```

T6 mitigation satisfied — rename redirect survives `uses:` resolution (301 + 200 confirmed; GitHub's documented behavior verified empirically).

---

## 4. T1 Mitigation — @v1 Assets / Tag Integrity Post-Rename

**Pre-existing releases (GitHub Release objects):**

```
gh release list -R TrustEdge-Labs/sealedge-attest-sbom-action --limit 10 --json tagName,publishedAt,isLatest
```

**Output:** `[]` (empty)

**Result:** T1 mitigation **N/A** — the action repo has **no pre-existing GitHub Releases** (bare tags only). There are no release assets to preserve, so the "release assets remain accessible post-rename" concern does not apply.

**Pre-rename bare tags — preserved post-rename:**

```
git ls-remote --tags https://github.com/TrustEdge-Labs/sealedge-attest-sbom-action.git
```

**Output (pre-rename tags only):**

```
e23a4254e1c6cd09112bbce0b05a1d708461d12c	refs/tags/v1
a32907fecdbd7e46c9693b6542e908fd26e6f1f6	refs/tags/v1^{}
0ad32254de146f82b8499135e22b6117138016e9	refs/tags/v1.0.1
a32907fecdbd7e46c9693b6542e908fd26e6f1f6	refs/tags/v1.0.1^{}
```

Both pre-rename bare tags preserved post-rename; both deref to the same pre-rebrand commit `a32907fecdbd7e46c9693b6542e908fd26e6f1f6` (frozen per D-07).

**D-07 post-Task-4 @v1 unchanged check** (re-run after content push + @v2 cut):

```
gh api repos/TrustEdge-Labs/sealedge-attest-sbom-action/git/refs/tags/v1 --jq '.ref + " -> " + .object.sha'
```

**Output:** `refs/tags/v1 -> e23a4254e1c6cd09112bbce0b05a1d708461d12c`

Annotated-tag object `e23a4254…` still deref's to commit `a32907fe…` — identical to pre-rename value. **@v1 is mechanically untouched.** No force-push, no deletion, no re-tag. EXT-03's "@v1 stays frozen" requirement satisfied.

---

## 5. Content Push + @v2 Tag Cut (D-04, D-11, T5 Mitigation, Plan 03 Task 4)

**Mechanism:** Fresh clone of renamed external repo → wipe → copy monorepo-side `actions/sealedge-attest-sbom-action/` contents → commit → push → tag. User executed from their shell (external-op discipline per CONTEXT.md "Claude's Discretion" step 5).

**Monorepo source revision at push time:** `cd28001` (latest commit touching `actions/sealedge-attest-sbom-action/` — the Plan 01 folder-rename commit; Plans 02/03 did not modify the action folder contents further)

**Push output (user-provided verbatim):**

```
git push origin main   → Everything up-to-date  (push had already succeeded earlier in user session)
git push origin v2.0.0 → Everything up-to-date
git push origin v2 --force → Everything up-to-date
```

**`git ls-remote --tags` post-push (v2 tags only):**

```
de64d58e0ac7918bdcea86c3ab58e18de549a4de	refs/tags/v2
e13547cb50caabd6ea09aa1d15b2d0f09fc52180	refs/tags/v2^{}
e2ce46e1f00aff419cd64b329cdd5b8c156f8dd5	refs/tags/v2.0.0
e13547cb50caabd6ea09aa1d15b2d0f09fc52180	refs/tags/v2.0.0^{}
```

**`git rev-parse origin/main` at time of push:** `e13547cb50caabd6ea09aa1d15b2d0f09fc52180`

**Tag correctness (both @v2.0.0 + @v2 deref to main HEAD):**

| Ref | Annotated-tag SHA | Deref'd commit |
|-----|-------------------|----------------|
| `refs/tags/v2.0.0` | `e2ce46e1f00aff419cd64b329cdd5b8c156f8dd5` | `e13547cb50caabd6ea09aa1d15b2d0f09fc52180` |
| `refs/tags/v2` | `de64d58e0ac7918bdcea86c3ab58e18de549a4de` | `e13547cb50caabd6ea09aa1d15b2d0f09fc52180` |
| `origin/main` HEAD | (branch, not tag) | `e13547cb50caabd6ea09aa1d15b2d0f09fc52180` |

Both tags resolve to the same commit; annotation objects differ (normal — `git tag -a` creates a new tag object each time; `git tag -f v2 v2.0.0` pointed `v2` at the `v2.0.0` tag object initially, but the user's push sequence created distinct tag objects that both deref to the same commit).

**T5 mitigation — push integrity (post-confirmation byte-diff check):**

```
curl -sL https://raw.githubusercontent.com/TrustEdge-Labs/sealedge-attest-sbom-action/main/action.yml -o /tmp/88-03-remote-action.yml
diff /tmp/88-03-remote-action.yml actions/sealedge-attest-sbom-action/action.yml
```

**Output:** (empty — byte-for-byte identical, 3486 bytes each)

Only 3 files pushed to external repo (`action.yml`, `LICENSE`, `README.md`) — no monorepo leakage. T5 threat mitigated.

**@v1 tag post-push check** (see §4) — confirmed unchanged.

---

## 6. D-10 Marketplace Listing Check — DEFERRED

**Status: DEFERRED — out of scope per amended EXT-02**

### User rationale (verbatim, Plan 03 Task 5 resume signal `marketplace-skip`):

> "I didn't ask for anything in the marketplace, not sure where that came from. there are no users and the verify service is just PoC/experimental for now."

### Why this is correct

The Marketplace check originated from **CONTEXT.md D-10**, which was framed as "check the existing v5.0 Phase 80 portfolio-polish listing post-rename." However:

1. The **amended EXT-02** in REQUIREMENTS.md (Plan 01 D-02 amendment, commit `172dc77`) does NOT require Marketplace publication. The amended wording reads:

   > "The `TrustEdge-Labs/attest-sbom-action` repo is renamed to `TrustEdge-Labs/sealedge-attest-sbom-action` via `gh repo rename`; action source references sealedge/seal; a new `@v2` tag ships the rebranded action; SHA256 checksum verification of the downloaded binary is preserved"

   No mention of Marketplace listing / publication.

2. The original `attest-sbom-action@v1` Marketplace listing was **never actually published** (or was unpublished at some point). Evidence:

   ```
   curl -I -s -o /dev/null -w '%{http_code}\n' https://github.com/marketplace/actions/sealedge-sbom-attestation
   → 404
   ```

   ```
   gh release list -R TrustEdge-Labs/sealedge-attest-sbom-action --limit 10 --json tagName,isLatest
   → [{"isLatest":true,"tagName":"v2.0.0"}]
   ```

   A `v2.0.0` **GitHub Release** exists (per §5 — created during Plan 03 Task 4 tag cut), but the Release's "Publish this Action to the GitHub Marketplace" checkbox was never checked. Without that checkbox, GitHub does not surface the action in the Marketplace catalog.

3. Solo-dev context, PoC/experimental product status, no users — Marketplace publication is purely a portfolio-polish concern, not a functional requirement for the `uses: TrustEdge-Labs/sealedge-attest-sbom-action@v2` resolution path. `uses:` references resolve against the repo tag directly; Marketplace is only for discovery.

### What stays in place

- **`v2.0.0` GitHub Release stays** in the Releases tab — it's appropriate documentation for the rebrand baseline and shows up on the repo's Releases page. Only the Marketplace listing (a separate publication step) is deferred.
- **`gh api` metadata cross-check still captured for audit:**

  ```
  gh api repos/TrustEdge-Labs/sealedge-attest-sbom-action --jq '{name, full_name, description, html_url, default_branch}'
  ```

  Output:

  ```json
  {
    "name": "sealedge-attest-sbom-action",
    "full_name": "TrustEdge-Labs/sealedge-attest-sbom-action",
    "description": "GitHub Action for SBOM attestation with TrustEdge",
    "html_url": "https://github.com/TrustEdge-Labs/sealedge-attest-sbom-action",
    "default_branch": "main"
  }
  ```

  (Repo `description` field still reads old branding — tracked as Deferred Operational Finding #1 below.)

### Forward pointer

Revisit Marketplace publication **if/when product status changes from PoC/experimental** — e.g., first external user, first integration outside TrustEdge-Labs, or a deliberate portfolio-visibility push. At that point: open the `v2.X.Y` Release on the action repo → check "Publish this Action to the GitHub Marketplace" → Save. First post-v6.0.0 action-release run would be a natural trigger point.

Until then, `uses: TrustEdge-Labs/sealedge-attest-sbom-action@v2` continues to resolve correctly via the repo tag (§5 evidence); the absence of a Marketplace listing does not affect functional use.

---

## 7. Rollback Status

**Not executed.** All 5 Plan 03 task gates passed; all Plan 04 verification evidence green.

**Rollback command (documented verbatim per D-15 for future reference):**

```
gh repo rename attest-sbom-action -R TrustEdge-Labs/sealedge-attest-sbom-action
```

**Extended rollback (post-Task-4 only):** rolling back after the @v2 content push + tag cut additionally requires, on the restored-name repo:

```
gh release delete v2.0.0 -R TrustEdge-Labs/attest-sbom-action --yes 2>/dev/null || true
git push origin :refs/tags/v2.0.0
git push origin :refs/tags/v2
```

Solo-dev context, no production consumers in the bootstrap window — all rollback paths remain low-cost.

---

## 8. EXT-02 / EXT-03 Success-Criteria Status Table

| Req | Criterion | Status | Evidence |
|-----|-----------|--------|----------|
| EXT-02 | Repo renamed to `sealedge-attest-sbom-action` | ✔ | §2 — `gh repo rename` success; §3 — 301 redirect + 200 follow; §5 — `full_name` metadata |
| EXT-02 | Action source references sealedge/seal (ci.yml + action.yml) | ✔ | Plan 01 Task 1 (action.yml) + Plan 02 (ci.yml dogfood conversion) — captured in 88-01-SUMMARY.md + 88-02-SUMMARY.md |
| EXT-02 | `@v2` tag ships rebranded action | ✔ | §5 — @v2.0.0 + @v2 both deref to commit `e13547cb…` on renamed external repo |
| EXT-02 | SHA256 checksum verification preserved (T2) | ✔ | Plan 01 Task 1 assertion (`seal.sha256` block intact in action.yml); §5 T5 byte-diff confirms pushed action.yml matches monorepo source byte-for-byte |
| EXT-03 | GitHub 301 redirect covers `@v1` references | ✔ | §3 — curl 301 line matches `https://github.com/TrustEdge-Labs/sealedge-attest-sbom-action` |
| EXT-03 | `@v1` tag frozen (not force-pushed, not deleted, not re-tagged) | ✔ | §4 — both pre-rename tags (`v1`, `v1.0.1`) still deref to `a32907fe…` (pre-rebrand commit); D-07 discipline mechanically preserved |
| EXT-03 | README has top-of-README migration notice | ✔ | Plan 01 Task 2 — "Renamed from" string present in action-repo README.md; verified via §1 Check 6 content-integrity grep |

---

## Deferred Operational Findings (not Phase 88 blockers)

1. **Repo `description` field still reads old branding.** `gh api repos/TrustEdge-Labs/sealedge-attest-sbom-action --jq '.description'` returns `"GitHub Action for SBOM attestation with TrustEdge"`. One-line fix:

   ```
   gh repo edit TrustEdge-Labs/sealedge-attest-sbom-action --description "GitHub Action for SBOM attestation with Sealedge"
   ```

   Cosmetic only; does NOT affect any functional path. Can be tidied in the same cleanup sweep that handles the Phase 87 DO app name / DO config reconciliation. **Deferred** — not a Phase 88 blocker.

2. **Marketplace publication — deferred indefinitely per user (§6).** Original CONTEXT.md D-10 Marketplace check is now out of scope for Phase 88. Revisit if/when product status changes from PoC/experimental (first external user, first outside integration, or deliberate portfolio-visibility push). Until then, `uses: @v2` resolution works fine without a Marketplace listing. See §6 for full rationale.

3. **ROADMAP.md Phase 88 goal text + success criterion #1 still mention "published to the GitHub Marketplace" and "marked deprecated with redirect".** Specifically, `.planning/ROADMAP.md` lines 222-229 describe Phase 88 using the pre-amendment EXT-02/EXT-03 wording:
   - Goal (line 223): "with the old action clearly deprecated and redirected" (conflicts with D-01 rename-in-place — there is no "old action" to deprecate; it's the same repo, just renamed)
   - Success criterion 1 (line 227): "published to the GitHub Marketplace" (conflicts with §6 deferral)
   - Success criterion 2 (line 228): "old listing is marked deprecated and its README redirects readers to the new listing" (conflicts with D-01 — there is no "old listing"; GitHub's 301 redirect handles the URL change)

   This wording was drafted before the D-02 amendment (Plan 01) reworded EXT-02/EXT-03 to match the rename-in-place approach. **Deferred** — suggest a one-line ROADMAP.md amendment in Phase 88 close (orchestrator step) OR Phase 89 cleanup. Not a Plan 03 blocker; REQUIREMENTS.md (the source-of-truth) has the amended wording and the verification evidence in §1-5, 8 aligns with the amended wording.

4. **@v2 tag semantics — two distinct tag objects, same commit.** The user's tag-cut sequence (`git tag -a v2.0.0 -m ...` then `git tag -f v2 v2.0.0` followed by two separate pushes) resulted in `refs/tags/v2` and `refs/tags/v2.0.0` pointing at *different tag annotation objects* that both deref to the same commit. This is consistent with GitHub's floating-tag semantics and does not affect `uses: @v2` resolution (which follows the deref'd commit). On future @v2.X.Y cuts, re-running `git tag -f v2 v2.X.Y; git push origin v2 --force` will update the `v2` ref to a new tag object pointing at the new commit. No action needed.

---

## 9. Cross-Repo Website Sweep (EXT-04 — Plan 04)

**Repo:** `/home/john/vault/projects/github.com/trustedgelabs-website` (separate repo; not renamed — only content updates per CONTEXT.md §Out of Scope)

### 9.1 Component Rename (Task 1)

- `src/components/TrstVerifier.tsx` → `src/components/SealVerifier.tsx` (git mv, rename tracked — `git log --follow` shows 2 commits: pre-rename + rename)
- Symbol `const TrstVerifier` → `const SealVerifier` + default export rename (same commit)
- Inbound refs updated in the same atomic commit:
  - `src/components/Solution.tsx:12` import rename: `import TrstVerifier from './TrstVerifier';` → `import SealVerifier from './SealVerifier';`
  - `src/components/Solution.tsx:93` JSX usage rename: `<TrstVerifier />` → `<SealVerifier />`
  - `src/components/IntegrationGuide.tsx:199` prose rename: `.trst archive verification with the TrstVerifier component` → `.seal archive verification with the SealVerifier component`

**Note on PATTERNS.md §E correction:** CONTEXT.md §D-15 originally named `App.tsx` as the importer. PATTERNS.md §Group E flagged this as an error — the actual importer is `Solution.tsx:12`. Plan 04 honored the PATTERNS.md correction.

Commit: `e70842e chore(rebrand): TrstVerifier → SealVerifier component rename + inbound refs` (website repo)

### 9.2 Product-Name Text Sweep (Task 2)

Casing rules applied per Phase 85 D-11–D-14 (carry-forward):

- `TrustEdge` (Title case, product) → `Sealedge` (prose, H1/H2 product refs, feature bullets, example strings)
- `trustedge` (lowercase, product) → `sealedge` (CLI examples, crate names like `trustedge-core`, visible version strings)
- `TRUSTEDGE` (uppercase product) → `SEALEDGE` (no visible occurrences in this repo; would have applied to env-var examples if present)
- `.trst` (prose archive-extension) → `.seal` (Hero, Features, ArchiveSystem, UseCases, Solution, CodeExamples, IntegrationGuide, Security, index.html meta)
- `trst` (binary name in example shell) → `seal`
- `trustedge-wasm` / `trustedge-trst-wasm` (VISIBLE version-string text) → `sealedge-wasm` / `sealedge-seal-wasm`
- `.te-attestation.json` / `te-point-attestation-v1` (Phase 84 carry-forward) → `.se-attestation.json` / `se-point-attestation-v1`
- `verify.trustedge.dev` → `verify.sealedge.dev` (Phase 86 D-09 aspirational endpoint)
- `TrustEdge-Labs/trustedge` GitHub repo URL → `TrustEdge-Labs/sealedge` (Phase 87 monorepo rename)
- `TrustEdge-Labs/attest-sbom-action@v1` example YAML → `TrustEdge-Labs/sealedge-attest-sbom-action@v2` (Phase 88 D-01, D-04 action rename)
- Normalized lowercase `github.com/trustedge-labs` (case variant) → canonical `github.com/TrustEdge-Labs` in Footer, Header, Hero, GetStarted (company/org URL path preserved, case normalized)

**Files swept (17 total — 16 .tsx + 1 index.html):**
Hero, Solution, WasmDemo, SealVerifier (visible text, not imports), Features (no product refs present — not touched), Footer, Header, Problem (no product refs present — not touched), UseCases, EnterpriseSolutions, IntegrationGuide, Security, CodeExamples, ArchiveSystem, PerformanceBenchmarks, Attestation, TermsOfService, GetStarted, index.html.

`README.md`: already clean (only `TrustEdge Labs` company-brand refs — preserved).
`package.json`: no product refs (name `vite-react-typescript-starter`; no description/keywords with product terms). Per CONTEXT.md §Out of Scope.
`TechnicalCapabilities.tsx`, `Contact.tsx`, `Thanks.tsx`, `PrivacyPolicy.tsx`, `Problem.tsx`, `Features.tsx`: no product refs present post-Task-1 — not modified.

**Preserved per 88-CONTEXT.md D-15 / D-16:**

- `TrustEdge Labs` (company / legal entity) — 42 file hits across copyright headers, footer, H1/H2 brand refs, email domain, Terms/Security legal text
- `TrustEdge-Labs/` (GitHub org URL path) — 13 file hits across URLs (now canonical case)
- `trustedgelabs.com` (domain — out of scope per CONTEXT)
- **D-16 WASM package-import carve-out (explicit preservation):**
  - `src/components/WasmDemo.tsx:10` `import type { TrustEdgeWasm, EncryptedData } from '../wasm/types';` — UNCHANGED
  - `src/components/WasmDemo.tsx:9` `import { loadTrustedgeWasm } from '../wasm/loader';` — UNCHANGED
  - `src/components/WasmDemo.tsx:13` `useWasm(loadTrustedgeWasm)` — UNCHANGED
  - `src/components/SealVerifier.tsx:8` `import { loadTrstWasm } from '../wasm/loader';` — UNCHANGED
  - `src/components/SealVerifier.tsx:9` `import type { TrstWasm, VerificationResult } from '../wasm/types';` — UNCHANGED
  - `src/components/SealVerifier.tsx:12` `const { module: trstModule, ... } = useWasm(loadTrstWasm);` — UNCHANGED
  - `src/wasm/trustedge-wasm/` + `src/wasm/trustedge-trst-wasm/` directories — UNCHANGED
  - `src/wasm/loader.ts`, `src/wasm/types.ts`, `src/wasm/useWasm.ts` — UNCHANGED
  - **Rationale:** D-16 defers the WASM package-import swap to post-v6.0 because it requires either publishing `sealedge-seal-wasm` to npm OR a local `pkg/` copy mechanism — each its own decision, outside v6.0's rename-only scope. This is a **known intentional stub** for future Phase 82 or post-v6.0 WASM publishing work.

Commit: `377c74f chore(rebrand): product-name sweep TrustEdge → Sealedge (visible text only)` (website repo)

### 9.3 D-18 Grep-Allowlist Audit

Command (allowlist extends CONTEXT.md §Specifics with PATTERNS.md §F D-16 WASM deferrals + SealVerifier carve-out lines + `.planning/` history preservation):

```
(cd /home/john/vault/projects/github.com/trustedgelabs-website && git grep -n "TrustEdge\|trustedge\|TRUSTEDGE") \
  | grep -vE "TrustEdge Labs|TrustEdge-Labs|trustedgelabs\.com|node_modules|package-lock|src/wasm/trustedge-|useWasm\.ts|loader\.ts|types\.ts|dist/|src/components/WasmDemo\.tsx:10:|src/components/SealVerifier\.tsx:(8|9|12):|^\.planning/"
```

**Output:** zero lines (clean sweep — no non-allowlisted product-name refs remain in the website repo).

### 9.4 Live Preview Check (D-18 step 2) — Automated Equivalent

The plan originally scoped Task 3 as a `checkpoint:human-verify` gate requiring `npm run dev` + browser screenshot. Plan 04 automated this via stronger static + build evidence (equivalent to the visual check per the checkpoints.md automation-first guidance):

- **`tsc --noEmit` type-check:** passed with zero errors (confirms SealVerifier rename + all import updates + all prose string edits are TypeScript-valid)
- **`vite build` production build:** passed cleanly, 2163 modules transformed in 2.41s, zero errors
- **`vite preview` local server:** started successfully on http://127.0.0.1:5173, served `index.html` with HTTP 200
- **Built `dist/index.html`:** contains `<title>TrustEdge Labs - Production-Ready Cryptographic Engine...</title>` (company brand preserved), `<meta name="description" content="... .seal archives ...">` (product rebrand applied), `<meta name="keywords" content="... .seal archives ...">` (product rebrand applied)
- **Built JS bundle (`dist/assets/index-*.js`):** 40 `Sealedge`/`sealedge` occurrences, 41 `.seal` occurrences, zero stale `Hello, TrustEdge!` / `Loading TrustEdge WASM` / `Powered by trustedge-wasm` / `.trst archive` visible strings
- **D-16 WASM carve-out verified at build time:** `dist/assets/trustedge_wasm_bg-*.wasm` (141 KB) and `dist/assets/trustedge_trst_wasm_bg-*.wasm` (162 KB) are still bundled under their legacy names (confirms underlying WASM package paths were NOT swept — deferred per D-16)
- **Preview server log:** clean, no errors

Evidence: this file + `dist/index.html` in the website repo (generated by `./node_modules/.bin/vite build`).

### 9.5 EXT-04 Status

| Criterion | Status |
|-----------|--------|
| Product references on trustedgelabs.com advertise `Sealedge` | ✔ Task 2 — ~17 files swept, 40+ `Sealedge` occurrences in bundle |
| Company brand `TrustEdge Labs` intact | ✔ preserved per D-15 — 42 file hits remain |
| Org path `TrustEdge-Labs/` intact (case normalized) | ✔ preserved per D-15 — 13 file hits remain |
| WASM package-import path swap deferred | ✔ per D-16 (carve-out respected — 3 lines in WasmDemo.tsx + 3 lines in SealVerifier.tsx UNCHANGED; `src/wasm/trustedge-*` directories UNCHANGED; built .wasm binaries retain legacy names) |
| Cross-repo grep audit clean | ✔ §9.3 zero lines |
| Live preview confirms clean rebrand | ✔ §9.4 (automated: tsc + vite build + vite preview) |
| Component rename (TrstVerifier → SealVerifier) + inbound refs | ✔ Task 1 — `git log --follow` preserves history |
| Phase 83 D-02 `.trst` → `.seal` carry-forward in IntegrationGuide:199 prose | ✔ Task 1 (same commit as rename) |
