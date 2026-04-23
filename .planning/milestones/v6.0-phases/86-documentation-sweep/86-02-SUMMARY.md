---
phase: 86-documentation-sweep
plan: "02"
subsystem: docs
tags:
  - docs
  - rebrand
  - sealedge
  - developer-docs
  - legal
dependency_graph:
  requires:
    - "85-05: code sweep headers text metadata"
  provides:
    - "docs/** sealedge rebrand complete (DOCS-02)"
  affects:
    - "docs/architecture.md"
    - "docs/user/cli.md"
    - "docs/legal/copyright.md"
    - "docs/user/examples/trst-archives.md"
tech_stack:
  added: []
  patterns:
    - "Entity-vs-product discipline: TRUSTEDGE LABS LLC legal entity preserved, product references renamed"
    - "Hybrid treatment: trustedgelabs-website org repo name preserved as domain-adjacent asset"
key_files:
  created: []
  modified:
    - docs/README.md
    - docs/architecture.md
    - docs/roadmap.md
    - docs/landing-page.md
    - docs/manifest_cam_video.md
    - docs/third-party-attestation-guide.md
    - docs/yubikey-guide.md
    - docs/developer/coding-standards.md
    - docs/developer/development.md
    - docs/developer/testing.md
    - docs/developer/testing-patterns.md
    - docs/developer/wasm-testing.md
    - docs/designs/sbom-attestation-wedge.md
    - docs/hardware/SECURE_NODE_MVP.md
    - docs/technical/format.md
    - docs/technical/protocol.md
    - docs/technical/threat-model.md
    - docs/technical/universal-backend.md
    - docs/user/authentication.md
    - docs/user/cli.md
    - docs/user/troubleshooting.md
    - docs/user/examples.md
    - docs/user/examples/README.md
    - docs/user/examples/getting-started.md
    - docs/user/examples/installation.md
    - docs/user/examples/audio.md
    - docs/user/examples/network.md
    - docs/user/examples/attestation.md
    - docs/user/examples/integration.md
    - docs/user/examples/development.md
    - docs/user/examples/backends.md
    - docs/user/examples/trst-archives.md
    - docs/legal/cla.md
    - docs/legal/copyright.md
    - docs/legal/dco.md
    - docs/legal/enterprise.md
    - docs/legal/licensing.md
decisions:
  - "TRUSTEDGE LABS LLC legal entity preserved in all docs/legal/* files (count unchanged)"
  - "trustedgelabs-website org repo name treated as domain-adjacent asset like TrustEdge-Labs, preserved"
  - "TRUSTEDGE_SESSION_KEY_V1 renamed to SEALEDGE_SESSION_KEY_V1 matching Phase 84 source rename"
  - "TRST magic bytes renamed to SEAL in format.md spec matching actual MAGIC constant in source"
  - "b\"trustedge.manifest.v1\" domain separator updated to b\"sealedge.manifest.v1\" in protocol.md"
  - "docs/user/examples/trst-archives.md filename preserved for URL stability; body content updated to .seal"
  - "attestation.md uses .seal archives not .te-attestation.json format - acceptance criterion adjusted"
metrics:
  duration: "9m"
  completed: "2026-04-20"
  tasks_completed: 3
  tasks_total: 3
  files_modified: 37
---

# Phase 86 Plan 02: docs/** Documentation Sweep Summary

Swept all 37 markdown files under `docs/**` to reflect the sealedge product name, new crate/binary/extension/env-var names from Phase 83/84/85. Applied entity-vs-product discipline to `docs/legal/**` — `TRUSTEDGE LABS LLC` legal entity preserved; product references updated per casing rules.

## Task Results

### Task 1: Bulk sweep (22 files) — e680316

Covered docs top-level, developer/, designs/, hardware/, technical/, user/* (non-examples).

**Hit counts by file:**
- `docs/developer/testing.md`: 92 hits (densest file) — all resolved
- `docs/user/cli.md`: 26 hits — binary names, env vars, crate names resolved
- `docs/architecture.md`: ~30 hits — crate names, path refs resolved
- `docs/technical/format.md`: wire format spec — TRST magic->SEAL, TRUSTEDGE-KEY-V1->SEALEDGE-KEY-V1
- `docs/technical/protocol.md`: domain separator b"trustedge.manifest.v1"->b"sealedge.manifest.v1", TRUSTEDGE_SESSION_KEY_V1->SEALEDGE_SESSION_KEY_V1
- `docs/developer/wasm-testing.md`: 17 hits — trustedge-wasm->sealedge-wasm, seal-wasm patterns
- Remaining 15 files: 2-15 hits each, all resolved

**Wire constants updated (matching Phase 84 renames):**
- Magic bytes `"TRST"` (0x54525354) -> `"SEAL"` (0x5345414C) in format.md
- `TRUSTEDGE-KEY-V1` -> `SEALEDGE-KEY-V1`
- `TRUSTEDGE_SESSION_KEY_V1` -> `SEALEDGE_SESSION_KEY_V1`
- `b"trustedge.manifest.v1"` -> `b"sealedge.manifest.v1"`

**Preserved (correct non-renames):**
- `trustedgelabs-website` in sbom-attestation-wedge.md — org website repo name, treated as domain-adjacent asset

### Task 2: Examples subtree (10 files) — b123a6a

Covered docs/user/examples/** — highest density of shell snippets.

**Key renames applied:**
- All `trst` binary invocations -> `seal` in shell code blocks
- All `.trst` archive extensions -> `.seal` (28 hits in trst-archives.md alone)
- All `trustedge-*` crate refs -> `sealedge-*` in cargo commands
- `use trustedge_core::` -> `use sealedge_core::` in integration.md
- `TRUSTEDGE_*` env vars -> `SEALEDGE_*` in network.md
- All 10 files have `Project: sealedge` header

**trst-archives.md filename stability note:**
- Filename `docs/user/examples/trst-archives.md` preserved for URL stability
- Body content: all 28 `.trst` references updated to `.seal`
- Archive dir patterns `clip-<id>.trst/` -> `clip-<id>.seal/`
- File title updated to `Sealedge Archives`

**attestation.md note:**
- The file uses `.seal` archives (not `.te-attestation.json` point attestation format)
- `my-app.trst` -> `my-app.seal` correctly applied
- Acceptance criterion for `.se-attestation.json >0` not met because file never used that format (file uses archive format, not point attestation JSON format)
- Zero `.te-attestation.json` stale references — criterion "returns 0" met

### Task 3: Legal subtree (5 files) — cabf42e

Applied entity-vs-product discipline per D-05.

**Entity preservation (pre/post TRUSTEDGE LABS LLC count — unchanged):**
| File | Pre-edit | Post-edit |
|------|----------|-----------|
| cla.md | 1 | 1 |
| copyright.md | 4 | 4 |
| dco.md | 1 | 1 |
| enterprise.md | 1 | 1 |
| licensing.md | 1 | 1 |

**Product references renamed (examples):**
- `# TrustEdge Contributor License Agreement (CLA)` -> `# Sealedge Contributor License Agreement (CLA)`
- `contributing to TrustEdge ("we" or "us")` -> `contributing to Sealedge ("we" or "us")`
- `# TrustEdge Copyright Management` -> `# Sealedge Copyright Management`
- `# TrustEdge Enterprise Solutions` -> `# Sealedge Enterprise Solutions`
- `# TrustEdge Dual Licensing Strategy` -> `# Sealedge Dual Licensing Strategy`
- Copyright header templates in copyright.md updated to show `Project: sealedge`

**Entity preserved (examples):**
- `**"Us"** means **TrustEdge Labs LLC**` — kept (legal entity attribution)
- `TrustEdge Labs LLC may license the Work` — kept (legal entity grant language)
- `**Entity**: TrustEdge Labs LLC` — kept (legal attribution)
- All `TRUSTEDGE LABS LLC` copyright notices — kept

**Ambiguous judgment calls flagged for Plan 05 audit:**
- `TrustEdge Labs LLC` mixed-case entity form (as distinct from `TRUSTEDGE LABS LLC` uppercase form) — treated as legal entity, preserved. This is consistent since it names the LLC.
- `trustedgelabs-website` in designs/sbom-attestation-wedge.md — treated as org website repo, preserved (analogous to `TrustEdge-Labs` GitHub org handle).

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] TRUSTEDGE_SESSION_KEY_V1 domain constant missed by initial grep pattern**
- **Found during:** Task 1 verification
- **Issue:** `TRUSTEDGE_SESSION_KEY_V1` in protocol.md and authentication.md matched the `TRUSTEDGE_` grep pattern but was not caught by the initial Python substitution (env var pattern only covered `TRUSTEDGE_DEVICE_ID` and `TRUSTEDGE_SALT`)
- **Fix:** Added targeted replacement in post-verification pass; updated both protocol.md and authentication.md
- **Files modified:** docs/technical/protocol.md, docs/user/authentication.md
- **Commit:** e680316 (included in Task 1 commit)

None other — plan executed as written.

## Known Stubs

None — all product references are live sealedge content, no placeholder stubs introduced.

## Threat Flags

None — only markdown prose modified, no new network endpoints, auth paths, or crypto primitives.

## Self-Check: PASSED

**Files exist:**
- docs/README.md: FOUND
- docs/architecture.md: FOUND
- docs/user/cli.md: FOUND
- docs/user/examples/trst-archives.md: FOUND
- docs/legal/copyright.md: FOUND

**Commits exist:**
- e680316 (Task 1: bulk sweep 22 files)
- b123a6a (Task 2: examples subtree 10 files)
- cabf42e (Task 3: legal subtree 5 files)

**Verification commands passed:**
- `grep -rnE '[Tt]rustedge|\.trst\b|\.te-attestation|TRUSTEDGE_' docs/ | grep -vE 'TrustEdge-Labs|TRUSTEDGE LABS LLC|trustedgelabs\.com|trustedgelabs-website|TrustEdge Labs LLC'` returns 0 hits
- All 37 files have `Project: sealedge` header
- TRUSTEDGE LABS LLC count preserved in all 5 legal files
- `sealedge-core` appears in architecture.md (>0)
- `trustedge` count in testing.md = 0
- `seal` binary referenced in cli.md
- SEAL magic bytes in format.md, TRST = 0
