# Phase 86: Documentation Sweep - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-04-20
**Phase:** 86-documentation-sweep
**Areas discussed:** Root doc scope, Historical text, docs/ subdir scope, External refs

---

## Gray Area Selection

| Area | Presented | Selected |
|---|---|---|
| Root doc scope | ✓ | ✓ |
| Historical text | ✓ | ✓ |
| docs/ subdir scope | ✓ | ✓ |
| External refs | ✓ | ✓ |

**User selected all 4 areas via multiSelect.**

---

## Root doc scope

| Option | Description | Selected |
|---|---|---|
| All product/user-facing (Recommended) | In: README, CLAUDE, DEPENDENCIES, SECURITY, CONTRIBUTING, FEATURES, WASM, YUBIKEY_VERIFICATION, MIGRATION, GEMINI, CHANGELOG. Exclude: improvement-plan, RFC_K256_SUPPORT, security-review-platform as internal planning/review artifacts kept for historical trace. | ✓ |
| Strictly the 4 named | Only README, CLAUDE, DEPENDENCIES, SECURITY per literal DOCS-01 wording. | |
| Everything at root | All 14 .md files including improvement-plan, RFC_K256_SUPPORT, security-review-platform. | |

**User's choice:** All product/user-facing (Recommended)
**Notes:** Captured as D-01 (in-scope list) + D-01a (explicitly out-of-scope historical artifacts) in CONTEXT.md. Planner's repo-wide grep check (D-13) will allowlist the 3 excluded files.

---

## Historical text (CHANGELOG.md + MIGRATION.md)

| Option | Description | Selected |
|---|---|---|
| Hybrid (Recommended) | Header/intro and going-forward text use sealedge. Past version entries preserved verbatim. Add short note at top acknowledging pre-v6.0 entries refer to the former name. | ✓ |
| Full rewrite | Rewrite all entries to sealedge form. Clean brand, but historical inaccuracy. | |
| Freeze fully | Leave CHANGELOG and MIGRATION untouched. Preserves history but ~71 refs would remain visible. | |

**User's choice:** Hybrid (Recommended)
**Notes:** Captured as D-02 (CHANGELOG hybrid) in CONTEXT.md. Rationale: mirrors the user's clean-break semantics for wire formats (old data rejected cleanly) while preserving historical record.

### Follow-up: Add new v6.0 section to MIGRATION.md?

| Option | Description | Selected |
|---|---|---|
| Add section (Recommended) | New top entry documenting the trustedge→sealedge rebrand, rename mappings, and clean-break semantics (no auto-migration). Tells a user how to re-wrap/re-keygen. | ✓ |
| Skip — rebrand only | No net-new migration content. | |

**User's choice:** Add section (Recommended)
**Notes:** Captured as D-03 in CONTEXT.md. Content specified: v6.0 section lists all rename mappings, states no backward-compat path, gives concrete re-wrap/re-keygen commands under the new binary names.

---

## docs/ subdir scope

| Option | Description | Selected |
|---|---|---|
| All of docs/ (Recommended) | Everything under docs/ — README, technical/, developer/, designs/, hardware/, legal/, roadmap.md, landing-page.md, manifest_cam_video.md, third-party-attestation-guide.md, yubikey-guide.md. | ✓ |
| Skip historical/design docs | Out: docs/roadmap.md + docs/designs/sbom-attestation-wedge.md. Preserves design-doc accuracy. | |
| Strictly DOCS-02 list | Only architecture.md + cli.md + development.md + testing.md + user/*. | |

**User's choice:** All of docs/ (Recommended)
**Notes:** Captured as D-04 in CONTEXT.md. D-04a resolves DOCS-02's ambiguous `docs/cli.md` reference (actual file is `docs/user/cli.md`). D-05 adds entity-vs-product discipline for `docs/legal/` subtree following Phase 85 D-03 pattern.

### Follow-up: Other markdown surfaces (crate READMEs, .github/, deploy/, web/demo/)

| Option | Description | Selected |
|---|---|---|
| All except the old Action (Recommended) | In: crate READMEs, examples/cam.video/README, deploy/digitalocean/, web/demo/, .github/pull_request_template, .github/README. Out: actions/attest-sbom-action/README (Phase 88 replaces). | ✓ |
| All including old Action | Also update the old Action README. | |
| Only what's explicitly named | Only examples/cam.video/README.md. | |

**User's choice:** All except the old Action (Recommended)
**Notes:** Captured as D-06 in CONTEXT.md. D-06a explicitly excludes `actions/attest-sbom-action/README.md` — Phase 88 owns it.

---

## External refs

### Repo URL (TrustEdge-Labs/trustedge → TrustEdge-Labs/sealedge)

| Option | Description | Selected |
|---|---|---|
| Rename now (Recommended) | Switch all repo URL references in docs to /sealedge. GitHub redirect covers the Phase 86→Phase 87 window. | ✓ |
| Leave until Phase 87 | Keep docs pointing at /trustedge until Phase 87 renames the repo. | |

**User's choice:** Rename now (Recommended)
**Notes:** Captured as D-11 in CONTEXT.md. Same rationale as Phase 85 D-05 for `.rs` headers — GitHub's built-in redirect covers the window.

### verify.trustedge.dev domain

| Option | Description | Selected |
|---|---|---|
| Rename to verify.sealedge.dev (Recommended) | Replace with aspirational/placeholder sealedge domain. No DNS change today. | ✓ |
| Rename to localhost:3001 | Match actual default demo endpoint. | |
| Leave verify.trustedge.dev | Treat as out-of-scope external surface. | |

**User's choice:** Rename to verify.sealedge.dev (Recommended)
**Notes:** Captured as D-09 in CONTEXT.md. `scripts/demo-attestation.sh:30` line is the primary site. `trustedgelabs.com` references stay (domain doesn't rename per v6.0 memory).

---

## Final confirmation

| Option | Description | Selected |
|---|---|---|
| I'm ready for context (Recommended) | Write CONTEXT.md. Rustdoc enforcement, plan granularity, doc ordering go under Claude's Discretion. | ✓ |
| Rustdoc enforcement | Discuss ci-check scripted grep vs one-time phase-close grep vs no automated check. | |
| Plan granularity | Discuss how to split into plans. | |
| Something else | Another area. | |

**User's choice:** I'm ready for context (Recommended)
**Notes:** Rustdoc enforcement captured under Claude's Discretion (one-time phase-close grep is sufficient; permanent ci-check hook is optional). Plan granularity captured as 5-plan suggested split.

---

## Claude's Discretion

- **Plan granularity** — 5-plan split: root `.md` (Plan 01), `docs/**` (Plan 02), other `.md` surfaces (Plan 03), rustdoc (Plan 04), scripts + examples + final grep audit (Plan 05). Planner may split further.
- **Rustdoc enforcement mechanism** — one-time `grep -rni 'trustedge' target/doc/` at phase close is sufficient. Permanent ci-check.sh hook is nice-to-have, not required.
- **Commit granularity** — `cargo build --workspace --locked` green at every commit boundary. `cargo test --doc --workspace` required gate after Plan 04.
- **crates/wasm vs crates/seal-wasm investigation** — planner verifies during Plan 01 whether one is a Phase 83 rename leftover; flag to cleanup todo if so.
- **CHANGELOG v6.0 entry** — planner may draft stub or leave for Phase 89 milestone close.
- **Copyright year** — stays at 2025 (Phase 85 D-06 carry-forward).

## Deferred Ideas

- GitHub Action repo + Marketplace listing update → Phase 88
- Product website (`trustedgelabs.com`) content refresh → Phase 88
- GitHub repository rename operation → Phase 87
- Final validation across full test matrix + CI + E2E → Phase 89
- `crates/wasm/` directory cleanup if leftover → follow-up cleanup phase or todo
- Permanent ci-check.sh grep guard for DOCS-03 → backlog item if desired
- Copyright year bump to 2026 → separate annual sweep if ever needed
