---
phase: 86-documentation-sweep
plan: 03
subsystem: docs
tags: [rebrand, crate-readmes, github-templates, deploy, web-demo, scripts]

requires:
  - phase: 83-crate-and-binary-rename
    provides: final crate directory names (crates/cli, crates/core, crates/seal-cli, etc.)
  - phase: 84-crypto-constants-file-extension
    provides: .seal archive extension, .se-attestation.json
  - phase: 85-code-sweep-headers-text-metadata
    provides: casing rules, TrustEdge-Labs/TRUSTEDGE LABS LLC preservation discipline
provides:
  - All crate-level READMEs and internal tech docs describe the product as sealedge
  - examples/cam.video/README.md uses new binary/crate/extension names throughout
  - .github/ PR template and repo README updated
  - deploy/digitalocean/README-deploy.md uses sealedge-verifier docker image
  - web/demo/README.md uses new wasm crate and seal archive references
  - scripts/README.md and scripts/project/README.md brand-aligned
affects: [phase-88 (external-action-website), phase-89 (final-validation)]

tech-stack:
  added: []
  patterns: []

key-files:
  created:
    - .planning/phases/86-documentation-sweep/86-03-SUMMARY.md
  modified:
    - crates/cli/README.md
    - crates/core/README.md
    - crates/core/AUTHENTICATION.md
    - crates/core/BENCHMARKS.md
    - crates/core/PERFORMANCE.md
    - crates/core/SOFTWARE_HSM_TEST_REPORT.md
    - crates/seal-cli/README.md
    - crates/seal-protocols/README.md
    - crates/seal-wasm/README.md
    - crates/wasm/README.md
    - crates/experimental/pubky/README.md
    - crates/experimental/pubky-advanced/README.md
    - examples/cam.video/README.md
    - deploy/digitalocean/README-deploy.md
    - web/demo/README.md
    - .github/pull_request_template.md
    - .github/README.md
    - scripts/README.md
    - scripts/project/README.md
  excluded:
    - actions/attest-sbom-action/README.md  # D-06a — Phase 88 replaces the Action
    - crates/seal-wasm/pkg-bundler/README.md  # already sealedge-branded from prior work
    - crates/wasm/pkg-bundler/README.md  # already sealedge-branded from prior work

key-decisions:
  - "crates/seal-wasm/pkg-bundler/README.md and crates/wasm/pkg-bundler/README.md required no edits — they were already sealedge-branded before this phase started"
  - "examples/cam.video/Cargo.toml package name is sealedge-cam-video-examples — README updated to match"
  - "scripts/README.md trustedge-core/ bench path updated to crates/core/ (Phase 83 moved the directory)"

patterns-established:
  - "Sed-batch substitution pattern: one-pass sed with ordered substitutions (specific before general) reliably sweeps prose-heavy READMEs with no MPL header-body collision"

requirements-completed: [DOCS-01, DOCS-02, DOCS-04]

duration: recovery-inline
completed: 2026-04-20
---

# Phase 86 Plan 03: Crate READMEs + Other Markdown Sweep — Summary

**19 markdown files across crate READMEs, .github/ templates, deploy guides, web demo, and scripts now describe the product as sealedge; 3 files confirmed out of scope or already clean.**

## Performance

- **Duration:** Recovery-inline (initial parallel agent hit Bash permission gate before any commit; orchestrator completed remaining work via sequential inline execution)
- **Completed:** 2026-04-20
- **Tasks:** 2/2
- **Files modified:** 19
- **Files confirmed already clean (no edit needed):** 2

## Accomplishments

### Task 1 — Crate READMEs + crate-internal tech docs (14 files, 12 edited)

All crate-level README.md files updated:
- `crates/cli/README.md`, `crates/core/README.md`, `crates/seal-cli/README.md`, `crates/seal-protocols/README.md`
- `crates/seal-wasm/README.md`, `crates/wasm/README.md`
- `crates/experimental/pubky/README.md`, `crates/experimental/pubky-advanced/README.md`

crates/core internal docs:
- `AUTHENTICATION.md`, `BENCHMARKS.md`, `PERFORMANCE.md`, `SOFTWARE_HSM_TEST_REPORT.md`

No-op verified for pkg-bundler READMEs (already sealedge-branded from prior work).

Every MPL-2.0 header now reads `Project: sealedge — Privacy and trust at the edge.` with `GitHub: https://github.com/TrustEdge-Labs/sealedge`. Crate names, binary names, file extensions, env-var prefixes, and wire-format constants updated per Phase 83/84 mapping. `TrustEdge-Labs` / `TRUSTEDGE LABS LLC` / `trustedgelabs.com` preserved.

### Task 2 — Non-crate markdown (7 files)

- `examples/cam.video/README.md` — `sealedge-seal-cli` package references, `sealedge-cam-video-examples` package, `.seal` archive references throughout, updated CLI documentation link to `crates/seal-cli/`
- `deploy/digitalocean/README-deploy.md` — `sealedge-verifier` docker image name
- `web/demo/README.md` — `crates/seal-wasm` wasm-pack build path, `sealedge-seal-cli` package, `.seal` archive UI references, "Sealedge P0 WASM Demo" title
- `.github/pull_request_template.md` — MPL header updated
- `.github/README.md` — MPL header, "# Sealedge Documentation" title
- `scripts/README.md` — MPL header, "# Sealedge Scripts" title, "Sealedge project management", `crates/core/` bench path (Phase 83 directory move)
- `scripts/project/README.md` — MPL header, "utilities for Sealedge"

## crates/wasm vs crates/seal-wasm investigation (Claude's Discretion)

Both `crates/wasm/` and `crates/seal-wasm/` exist in the workspace. Each has its own README.md and pkg-bundler/README.md. Neither was deleted in this phase per CONTEXT.md guidance ("Do not delete directories in Phase 86; flag in SUMMARY if one is a rename leftover"). Both directories contain real package manifests and source trees. Planner should investigate in a follow-up cleanup phase whether one is a Phase 83 rename leftover that can be consolidated.

## Scope excluded

- `actions/attest-sbom-action/README.md` — D-06a explicitly out of scope for Phase 86. Phase 88 replaces the old Marketplace Action entirely; editing its README before deletion would be wasted work.

## Verification

Per-file grep post-commit, for all 19 modified files:
```
grep -nE '[Tt]rustedge|\.trst\b|\.te-attestation|TRUSTEDGE_' $f | grep -vE 'TrustEdge-Labs|TRUSTEDGE LABS LLC|trustedgelabs\.com'
```
→ returns empty for every file.

All touched files have `Project: sealedge` in their MPL-2.0 header (the 2 no-edit pkg-bundler files already had this).

## Commits

- `4f84c85` — Task 1: 12 crate-level READMEs + internal tech docs
- `e54124d` — Task 2: examples + deploy + web demo + .github + scripts README sweep

## Self-Check: PASSED
