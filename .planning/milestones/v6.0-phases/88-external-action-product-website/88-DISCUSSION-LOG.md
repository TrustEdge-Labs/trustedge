# Phase 88: External Action & Product Website - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-04-21
**Phase:** 88-external-action-product-website
**Areas discussed:** New action location & naming, Old action deprecation depth, Binary-release dependency, Product website scope (EXT-04)

---

## New action: location & naming

### Initial gray-area reframe

After the first question was presented with options for a separate new repo, the user pushed back:

> "why can't the same changes made in trustedge repo be made in attest-sbom-action? solo builder and dev, there are no customers or users"

**Reframe accepted:** apply Phase 87's rename-in-place pattern to the action repo. No two-repo migration story. GitHub's 301 redirect covers old `uses:` references. EXT-02/EXT-03 wording amended to match.

### Q1: Rename-in-place vs two-repo migration?

| Option | Description | Selected |
|--------|-------------|----------|
| Rename in-place + amend requirements | Rename existing repo via `gh repo rename`, update content in-place, amend EXT-02/03 wording | ✓ |
| Rename in-place, keep req wording as-is | Same execution, interpret "new repo" = renamed repo | |
| Two-repo migration | Create new repo, deprecate old, README redirect | |

**User's choice:** Rename in-place, amend EXT-02/03 during planning.

### Q2: Renamed repo name?

| Option | Description | Selected |
|--------|-------------|----------|
| sealedge-attest-sbom-action | Product-prefix parallel to old name | ✓ |
| seal-attest-sbom-action | Use CLI binary short name as prefix | |
| sealedge-sbom-action | Drop 'attest-' | |

**User's choice:** sealedge-attest-sbom-action.

### Q3: First version tag?

| Option | Description | Selected |
|--------|-------------|----------|
| @v2 (plus floating @v2) | Signal user-visible breaking change; @v1 stays frozen | ✓ |
| Fresh @v1 | Force @v1 reuse; breaks SHA-pinned users | |
| @v1.1.0 minor bump | Stay on @v1 major; hides breaking change | |

**User's choice:** @v2.

### Q4: Existing monorepo folder fate?

| Option | Description | Selected |
|--------|-------------|----------|
| Canonical source in monorepo, rename to match | Rename `actions/attest-sbom-action/` → `actions/sealedge-attest-sbom-action/`, keep as source-of-truth | ✓ |
| Delete from monorepo | Move source to external action repo only | |
| Mirror both | Drift-risk pattern | |

**User's choice:** Canonical source in monorepo.

### Q5: Behavior parity?

| Option | Description | Selected |
|--------|-------------|----------|
| 1:1 equivalent (SHA256 optional with warning) | Same behavior as old action; rename-only | ✓ |
| Tighten SHA256 to required | Behavior change — scope creep for v6.0 | |
| 1:1 + rename-only text updates | Effectively same as option 1 | |

**User's choice:** 1:1 equivalent.

---

## Old action deprecation depth

### Q1: Old @v1 tag fate?

| Option | Description | Selected |
|--------|-------------|----------|
| Leave @v1 frozen; @v2 is canonical rebranded | Zero churn; redirects preserve old consumers | ✓ |
| Re-tag @v1 to emit deprecation warning | Force-push @v1; breaks SHA pins | |
| Delete @v1 entirely | Force users off; destructive | |

**User's choice:** Leave @v1 frozen.

### Q2: Post-rename README content?

| Option | Description | Selected |
|--------|-------------|----------|
| Short top-of-README notice | 1-3 line "renamed from attest-sbom-action" callout | ✓ |
| Full migration section with before/after | Dedicated ## Migration section | |
| No migration section | Clean-break in README too | |

**User's choice:** Short top-of-README notice.

### Q3: Verification of old `uses:` redirect?

| Option | Description | Selected |
|--------|-------------|----------|
| One-shot curl redirect check in 88-VERIFICATION.md | Confirm 301 from old URL; rely on GitHub docs | ✓ |
| Curl + live workflow run using @v1 | Prove end-to-end; higher setup | |
| No verification | Trust GitHub's documented behavior | |

**User's choice:** One-shot curl redirect check.

### Q4: Marketplace listing auto-update?

| Option | Description | Selected |
|--------|-------------|----------|
| Check first, re-publish if needed | Phase 88 verification step; manual fallback | ✓ |
| Assume auto-update works | Minimal ceremony | |
| Force manual re-publish | Always click-through GH UI | |

**User's choice:** Check first, re-publish if needed.

---

## Binary-release dependency

### Q1: When to cut @v2 tag?

| Option | Description | Selected |
|--------|-------------|----------|
| During Phase 88, before Phase 89 v6.0.0 release | Ship @v2 pointing at `latest`; smoke-tested by Phase 89 | ✓ |
| Defer @v2 tag to Phase 89 | Phase 88 stops at rename + content only | |
| Cut @v2 only once v6.0.0 exists; reorder phases | Move all tag work to Phase 89 | |

**User's choice:** During Phase 88, before Phase 89.

### Q2: Add seal binary upload to ci.yml release job?

| Option | Description | Selected |
|--------|-------------|----------|
| Yes, Phase 88 adds seal + seal.sha256 upload | Action needs real upstream binary to download | ✓ |
| Defer to Phase 89 final validation | Phase 88 ships non-functional @v2 until then | |
| Check release history first | Inspect prior `trst` uploads before deciding | |

**User's choice:** Yes, Phase 88 adds seal + seal.sha256 upload.

### Q3: Version-input rename and default?

| Option | Description | Selected |
|--------|-------------|----------|
| Rename `trst-version` → `seal-version`, default `latest` | Input name matches renamed binary | ✓ |
| `seal-version`, default pinned v6.0.0 | Higher friction on every minor release | |
| Keep `trst-version` input name, default `latest` | Preserves diff but confusing (binary is `seal`) | |

**User's choice:** Rename to `seal-version`, default `latest`.

### Q4: E2E verification of @v2?

| Option | Description | Selected |
|--------|-------------|----------|
| Convert ci.yml self-attest to `uses: sealedge-attest-sbom-action@v2` | Dogfood the action; Phase 89's first release proves it | ✓ |
| Keep ci.yml inline; add separate test workflow | Minimal change to existing self-attest | |
| Don't verify in Phase 88; defer to Phase 89 VALID-03 | Lowest effort | |

**User's choice:** Convert ci.yml self-attest job.

---

## Product website scope (EXT-04)

### Q1: Content change scope?

| Option | Description | Selected |
|--------|-------------|----------|
| Minimal: product-name swap only | Rename-only; keep company brand; scope-safe | ✓ |
| Minimal + update WASM demo package imports | Add WASM npm/local-pkg swap | |
| Broader refresh with rebranded copy, new hero | That's Phase 82 territory; scope creep | |

**User's choice:** Minimal product-name swap only.

### Q2: WasmDemo integration?

| Option | Description | Selected |
|--------|-------------|----------|
| Phase 88 updates component names + imports; WASM package swap deferred | Text/branding only; plumbing is a follow-up | ✓ |
| Phase 88 swaps WASM package imports inline | Adds complexity to cross-repo phase | |
| Don't touch WasmDemo; flag as follow-up only | Leave stale names; visibly inconsistent | |

**User's choice:** Update component names + imports; WASM package path swap deferred.

### Q3: Cross-repo work organization?

| Option | Description | Selected |
|--------|-------------|----------|
| Dedicated Phase 88 plan commits directly to website repo | Executor operates against both repos; atomic | ✓ |
| Hand-off checklist; user commits manually | Loses executor atomicity guarantee | |
| Executor creates branch + PR; user reviews & merges | More ceremony | |

**User's choice:** Dedicated Phase 88 plan commits to website repo.

### Q4: EXT-04 verification method?

| Option | Description | Selected |
|--------|-------------|----------|
| Grep audit + live preview screenshot | Source-tree clean AND visual evidence | ✓ |
| Grep audit only, no screenshot | Lower-effort; no visual proof | |
| Live preview + manual walkthrough, no formal grep | Harder to audit later | |

**User's choice:** Grep audit + live preview screenshot.

---

## Claude's Discretion

- Plan granularity (3-4 plans at planner's discretion): in-repo folder/content update, ci.yml release-job extensions + self-attest conversion, action repo rename + @v2 tag cut + Marketplace check, cross-repo trustedgelabs-website content update
- EXT-02/03 amendment language (suggested wording in CONTEXT.md specifics)
- Seal-binary release-artifact naming (mirror prior `trst` shape — uncompressed binary + .sha256 companion, or whatever prior releases established)
- Commit sequencing mirrors Phase 87: local content + ci.yml changes first, external `gh repo rename`, tag cut, parallel cross-repo website commits, verification last
- No v6.0.0 monorepo release in Phase 88 — that's Phase 89 milestone close

## Deferred Ideas

- WASM package publishing (`sealedge-seal-wasm` to npm) — post-v6.0 decision tree
- WasmDemo.tsx package-import swap — depends on WASM publishing decision
- Broader trustedgelabs.com copy refresh — Phase 82 territory
- Demo GIF re-record — Phase 81 territory
- Publishing sealedge crates to crates.io — post-v6.0 decision
- Stale `seal.te-attestation.json` extension in ci.yml — pre-existing Phase 84 gap; touch incidentally or leave for Phase 89
- v6.0.0 monorepo release tag — Phase 89 milestone close
- Permanent CI guard against stale product-name refs on website — not required; backlog if drift emerges
