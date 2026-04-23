---
phase: 86-documentation-sweep
plan: 01
subsystem: docs
tags: [rebrand, markdown, migration, changelog, copyright-headers]

requires:
  - phase: 83-crate-and-binary-rename
    provides: final sealedge-* crate names, seal binary, .seal extension
  - phase: 84-crypto-constants-file-extension
    provides: SEALEDGE-KEY-V1, SEALEDGE_ENVELOPE_V1, .se-attestation.json
  - phase: 85-code-sweep-headers-text-metadata
    provides: casing rules (D-11 through D-14), TrustEdge-Labs/TRUSTEDGE LABS LLC preservation
provides:
  - 11 root-level .md files (9 fully swept + 2 hybrid-treated) describing the product as sealedge
  - Forward-looking MIGRATION.md v6.0 section with complete rename map and concrete re-wrap/re-keygen commands
  - CHANGELOG.md top-of-file notice explaining pre-v6.0 entry brand
  - Readable audit trail for any user hitting the v6.0 clean-break failure modes
affects: [phase-87 (github-rename), phase-88 (external-action-website), phase-89 (final-validation)]

tech-stack:
  added: []
  patterns: [hybrid historical-preservation (CHANGELOG/MIGRATION), entity-vs-product discipline]

key-files:
  created:
    - .planning/phases/86-documentation-sweep/86-01-SUMMARY.md
  modified:
    - README.md
    - CLAUDE.md
    - DEPENDENCIES.md
    - SECURITY.md
    - CONTRIBUTING.md
    - FEATURES.md
    - WASM.md
    - YUBIKEY_VERIFICATION.md
    - GEMINI.md
    - CHANGELOG.md
    - MIGRATION.md

key-decisions:
  - "Pre-v6.0 CHANGELOG entries preserved verbatim — v5.0.0 continues to describe TrustEdge-Labs/attest-sbom-action@v1 and trst.te-attestation.json as shipped"
  - "MIGRATION.md v6.0 section uses Phase 83's actual final crate/binary names (sealedge-seal-cli, seal binary) rather than generic placeholders"
  - "trustedgelabs-website repo-name reference in CLAUDE.md preserved (sibling repo, not in v6.0 rename scope)"

patterns-established:
  - "Hybrid treatment: header/intro updated; historical version entries frozen verbatim with top-of-file notice explaining former name"
  - "v6.0 migration section: complete rename table, clean-break failure modes, concrete upgrade commands under new binary names"

requirements-completed: [DOCS-01]

duration: recovery-inline
completed: 2026-04-20
---

# Phase 86 Plan 01: Root .md Sweep — Summary

**11 root-level markdown files now describe the product as sealedge, with CHANGELOG/MIGRATION applying hybrid historical preservation plus a new MIGRATION v6.0 clean-break section.**

## Performance

- **Duration:** Recovery-inline (initial parallel agent hit Bash permission gate mid-plan; orchestrator completed remaining work via sequential inline execution)
- **Completed:** 2026-04-20
- **Tasks:** 3/3
- **Files modified:** 11

## Accomplishments

### Task 1 — 9-file straight-substitution sweep

Every MPL-2.0 header now reads `Project: sealedge — Privacy and trust at the edge.` with `GitHub: https://github.com/TrustEdge-Labs/sealedge`. Every product identifier (`trustedge` / `TrustEdge` / `TRUSTEDGE_*` / `.trst` / `.te-attestation.json`) updated per Phase 83/84/85 mapping. `TrustEdge-Labs` org, `TRUSTEDGE LABS LLC` entity, `trustedgelabs.com` domain, and generic "trust"/"edge" vocabulary preserved.

### Task 2 — CHANGELOG.md hybrid

Header, title, and a new top-of-file notice updated to sealedge. Past version entries (v5.0.0, v4.0.0, v3.0.0, ...) frozen verbatim so that historical references to `trst.te-attestation.json`, `TrustEdge-Labs/attest-sbom-action@v1`, and `ephemeral.pub → build.pub` describe what actually shipped. Notice points readers at `MIGRATION.md` §"v6.0" for the rename map.

### Task 3 — MIGRATION.md hybrid + v6.0 section

Header and title updated to sealedge. New forward-looking intro paragraph. New top-level `## v6.0: trustedge → sealedge rebrand — clean break` section inserted before the historical 0.2.x→0.3.x section containing:

- Full rename table (22 rows) covering crate prefix, binaries, file extensions, env-var prefix, wire-format constants (envelope/chunk-key/session-key/genesis/manifest/magic/X25519/V2 session/V2 audio), and repo URL
- Clean-break behavior statement enumerating the expected failure modes under v6.0 tooling
- Concrete upgrade commands: `sealedge-seal-cli -- keygen` / `wrap` / `attest-sbom`, env-var export changes, Cargo.toml update, `git remote set-url`
- Expected-error reference for users hitting `archive magic mismatch — expected SEAL, got TRST` and similar

Historical `## Migrating from 0.2.x to 0.3.x: Crate Consolidation` section preserved verbatim with its "TrustEdge workspace consolidation project" proper noun.

## Deviations from plan

- **Execution mode:** The plan was originally dispatched to a parallel worktree executor but that agent hit a Bash permission denial after Task 1's file edits. The orchestrator recovered by completing Tasks 2 and 3 inline in the main working tree. Net result matches the plan's intent; only the execution path deviated. All acceptance criteria verified per task.
- **crates/wasm directory note:** Not a deviation of this plan, but surfaced during Plan 03 execution — flagged there.

## Verification

All acceptance criteria from the plan verified post-commit:

- `grep -l "Project: trustedge " {9 files}` → empty
- `grep -l "TrustEdge-Labs/trustedge" {9 files}` → empty
- `grep -l "TRUSTEDGE_DEVICE_ID\|TRUSTEDGE_SALT\|TRUSTEDGE-KEY-V1\|TRUSTEDGE_ENVELOPE_V1" {9 files}` → empty
- `grep -l "\.trst\b\|\.te-attestation" {9 files}` → empty
- CHANGELOG.md: new title + notice + all historical version entries intact
- MIGRATION.md: 12 acceptance criteria all pass (title, v6.0 section position, keygen/wrap commands, env vars, wire constants, legal entity, historical section, header, historical proper noun)

## Commits

- `c3e1979` — Task 1: 9 root .md files straight-substitution sweep
- `70e765a` — Task 2: CHANGELOG.md hybrid treatment
- `693bbc0` — Task 3: MIGRATION.md hybrid + v6.0 section

## Self-Check: PASSED
