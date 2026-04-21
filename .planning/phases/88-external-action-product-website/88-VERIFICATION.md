<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: sealedge — Privacy and trust at the edge.
-->

# Phase 88 Verification — External Action & Product Website

**Phase:** 88-external-action-product-website
**Requirements:** EXT-02, EXT-03, EXT-04
**Date:** 2026-04-21
**Status:** partial — Plan 03 will prepend sections 1-8 (action rename + @v2 tag cut + marketplace). Plan 04 (this) contributes §9 below.

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
