# Phase 86: Documentation Sweep - Context

**Gathered:** 2026-04-20
**Status:** Ready for planning

<domain>
## Phase Boundary

Every human-readable document in the repo describes the product as **sealedge** with the new crate/binary/extension/env-var names so a cold reader never sees conflicting or stale brand references. Covers:

- Root-level `.md` files that describe the product or guide contributors/users
- Developer docs under `docs/**` (all subtrees — technical, developer, designs, hardware, legal, user, plus top-level docs/*.md)
- Crate-level `README.md` files under `crates/**/`
- Rustdoc comments (`///` on items, `//!` on modules) — the doc-comment sweep Phase 85 explicitly deferred here
- Scripts and examples: `scripts/*.sh` residual prose + `examples/cam.video/` content
- Other top-level `.md` surfaces: `.github/` templates, `deploy/digitalocean/`, `web/demo/`

**In scope (boundary with Phase 85 and Phase 87/88):**
- Phase 85 = compiled-binary strings and inline `//` comments in `.rs` source. Phase 86 = everything else that a human reads (markdown, rustdoc, script echo prose Phase 85 missed).
- Phase 86 updates the repository URL to `https://github.com/TrustEdge-Labs/sealedge` in docs now; Phase 87 performs the actual GitHub rename and GitHub's built-in redirect covers the gap (same rationale as Phase 85 D-05 for `.rs` headers).
- Phase 88 replaces the GitHub Action repo and updates the product website; Phase 86 does not touch `actions/attest-sbom-action/README.md`.

**Out of scope (explicit carve-outs):**
- `TrustEdge-Labs` GitHub org name — stays (per v6.0 memory and Phase 85 D-02 carry-forward)
- `TRUSTEDGE LABS LLC` legal entity in copyright lines — stays (Phase 85 D-03 carry-forward)
- `trustedgelabs.com` domain — stays (v6.0 memory: domain does not rename; only its product-page *content* changes, in Phase 88)
- `actions/attest-sbom-action/README.md` — Phase 88 deprecates/replaces the whole action; no edits here
- Internal planning/review artifacts at root: `improvement-plan.md`, `RFC_K256_SUPPORT.md`, `security-review-platform.md` — historical artifacts, not product docs
- `.planning/**` directory — discuss/plan/verify artifacts are historical internal records
- GitHub repository rename operation itself — Phase 87
- Net-new features or re-organization — rename/rewording only

</domain>

<decisions>
## Implementation Decisions

### Root-level `.md` scope (DOCS-01 expansion)

- **D-01:** Phase 86 sweeps the following 11 root-level `.md` files — DOCS-01's named four plus seven product/user-facing peers:
  | File | Category | DOCS-01 named? |
  |---|---|---|
  | `README.md` | Front door | ✓ |
  | `CLAUDE.md` | AI guide | ✓ |
  | `DEPENDENCIES.md` | Dep audit | ✓ |
  | `SECURITY.md` | Vuln policy | ✓ |
  | `CONTRIBUTING.md` | Contributor guide | — expanded |
  | `FEATURES.md` | Product features | — expanded |
  | `WASM.md` | WASM usage guide | — expanded |
  | `YUBIKEY_VERIFICATION.md` | Hardware guide | — expanded |
  | `MIGRATION.md` | Version migration | — expanded; also see D-03 |
  | `GEMINI.md` | Peer AI guide to CLAUDE.md | — expanded |
  | `CHANGELOG.md` | Version history | — expanded with hybrid treatment (D-02) |

- **D-01a:** Explicitly **out of Phase 86 scope** (historical internal artifacts; planner's grep allowlist excludes these):
  - `improvement-plan.md`
  - `RFC_K256_SUPPORT.md`
  - `security-review-platform.md`

  These are kept in the repo for historical trace but do not represent product documentation. A repo-wide `grep -i trustedge` success check must treat these as allowlisted.

### Historical text treatment (CHANGELOG.md + MIGRATION.md)

- **D-02:** **Hybrid treatment** for `CHANGELOG.md`:
  - Copyright/MPL header, file title, intro paragraph → `sealedge`
  - A short notice at top: *"Entries before v6.0 refer to the project under its former name, trustedge. See v6.0 for the rebrand."*
  - **Past version entries preserved verbatim** — `v5.0.0` continues to describe `TrustEdge-Labs/attest-sbom-action@v1`, `trst.te-attestation.json`, `ephemeral.pub`→`build.pub` as shipped. Rewriting would create historical inaccuracy (describing artifacts that never existed under the new names).
  - Future entries (v6.0 onward) use `sealedge` throughout.
  - **Rationale:** matches the user's clean-break semantics for wire formats (old data is rejected cleanly, not silently rewritten) while preserving the historical record of what actually shipped.

- **D-03:** **Hybrid treatment** for `MIGRATION.md`:
  - Header/intro text → `sealedge`
  - Past version migration entries preserved verbatim
  - **Add a new top-level section for v6.0:** `## v6.0: trustedge → sealedge rebrand — clean break`. Content:
    - Lists all rename mappings: crate prefix `trustedge-*` → `sealedge-*`, binaries (`trustedge`→`sealedge`, `trst`→`seal`, `trustedge-server`→`sealedge-server`, `trustedge-client`→`sealedge-client`, `trustedge-platform-server`→`sealedge-platform-server`), file extensions (`.trst`→`.seal`, `.te-attestation.json`→`.se-attestation.json`), env-var prefix (`TRUSTEDGE_*`→`SEALEDGE_*`), wire-format constants (`TRUSTEDGE-KEY-V1`→`SEALEDGE-KEY-V1`, etc.)
    - States explicitly: "no backward-compat migration path; existing `.trst` archives, `.se-attestation.json` files, `TRUSTEDGE-KEY-V1` encrypted keys, and active TCP/QUIC sessions fail cleanly under the new magic/domain constants"
    - Tells a user how to re-wrap / re-keygen under the new binary names (concrete commands matching Phase 83 final naming)
  - **Rationale:** any user who upgrades past v6.0 and tries to read prior data will hit magic-byte or domain-tag failures; documenting the break keeps the error message pointable.

### docs/ subdirectory scope (DOCS-02 expansion)

- **D-04:** Phase 86 sweeps **all of `docs/**`** — every subtree, not just the four files named in DOCS-02. This is a superset:
  - `docs/README.md` — TOC / entry point
  - `docs/architecture.md`, `docs/cli.md`* — (*DOCS-02 names `cli.md`; the repo has `docs/user/cli.md` — same file under user/, see D-04a)
  - `docs/roadmap.md` — historical project roadmap
  - `docs/landing-page.md` — marketing copy
  - `docs/manifest_cam_video.md` — format spec
  - `docs/third-party-attestation-guide.md`, `docs/yubikey-guide.md` — user guides
  - `docs/developer/` — `coding-standards.md`, `development.md`, `testing.md`, `testing-patterns.md`, `wasm-testing.md`
  - `docs/designs/sbom-attestation-wedge.md` — design doc for shipped feature (apply D-02-style hybrid if the doc narrates past intent: header/prose update; design history preserved)
  - `docs/hardware/SECURE_NODE_MVP.md` — hardware guide
  - `docs/legal/` — `cla.md`, `copyright.md`, `dco.md`, `enterprise.md`, `licensing.md` (see D-05 for legal-specific handling)
  - `docs/technical/` — `format.md`, `protocol.md`, `threat-model.md`, `universal-backend.md`
  - `docs/user/` — `authentication.md`, `cli.md`, `troubleshooting.md`, `examples.md`, `examples/*` (getting-started, installation, audio, network, attestation, integration, development, backends, trst-archives, README)

- **D-04a:** Resolution for DOCS-02's `docs/cli.md` reference: no such file exists at `docs/cli.md`; the intended target is `docs/user/cli.md`. Planner treats these as the same entry.

- **D-05:** `docs/legal/copyright.md` and any other legal doc under `docs/legal/` follow Phase 85 D-03 discipline:
  - `TRUSTEDGE LABS LLC` legal entity name **stays** wherever it appears in attribution language
  - Product references (non-attribution) update from `trustedge`/`TrustEdge` → `sealedge`/`Sealedge` per casing rules (D-11 below)
  - Any phrase of the form "the TrustEdge project" becomes "the Sealedge project"; "TRUSTEDGE LABS LLC" stays
  - If legal docs explicitly cite past product names in legal-binding text, preserve them and add a clarifying note (follow the design-doc hybrid pattern)

### Other markdown surfaces (outside root and docs/)

- **D-06:** Phase 86 also sweeps:
  - `crates/*/README.md` — all crate-level READMEs under `crates/cli/`, `crates/core/`, `crates/seal-cli/`, `crates/seal-protocols/`, `crates/seal-wasm/`, `crates/wasm/`, `crates/experimental/pubky/`, `crates/experimental/pubky-advanced/` (and any other crate READMEs discovered at plan-01 grep time). Note: `crates/wasm/` and `crates/seal-wasm/` both appear in the scout — investigate whether one is a Phase 83 rename leftover.
  - `crates/core/AUTHENTICATION.md`, `crates/core/BENCHMARKS.md`, `crates/core/PERFORMANCE.md`, `crates/core/SOFTWARE_HSM_TEST_REPORT.md` — crate-internal technical docs
  - `examples/cam.video/README.md` — DOCS-04 explicit
  - `deploy/digitalocean/README-deploy.md` — operator deploy guide
  - `web/demo/README.md` — demo README
  - `.github/pull_request_template.md` — PR template (pasted into every new PR, brand-visible)
  - `.github/README.md` — repo/org meta README

- **D-06a:** **Out of scope** (Phase 88 owns):
  - `actions/attest-sbom-action/README.md` — Phase 88 deprecates/replaces the whole Marketplace Action; no point editing ahead of deletion

### Rustdoc sweep (DOCS-03)

- **D-07:** Sweep every `///` and `//!` doc comment in `.rs` source across `crates/**/*.rs` and `examples/**/*.rs` (including experimental crates) for `trustedge`/`TrustEdge`/`TRUSTEDGE_*` brand references. Scout count: ~188 lines to review (excluding legal-entity matches). Includes:
  - MPL-2.0 header rustdoc lines (`/// Project: trustedge` → `/// Project: sealedge`, `/// GitHub: https://github.com/TrustEdge-Labs/trustedge` → `/// GitHub: https://github.com/TrustEdge-Labs/sealedge`) — Phase 85 handled the block-comment form (`// Project:` in MPL headers using `//`), but some files use `///` rustdoc form instead; those belong here
  - Module-level `//!` that names the crate or describes the product (e.g., `crates/core/src/lib.rs` examples referencing `trustedge-core = ...`, `crates/wasm/src/lib.rs` importing from `'trustedge-wasm'`, etc.)
  - `///` item-level docs that reference crate/binary/env-var names in prose or examples

- **D-08:** Apply the same casing rules from Phase 85 D-11–D-14:
  - Product name in prose: `sealedge` (lowercase)
  - Brand word at sentence start: `Sealedge`
  - Constants/env vars in docstring examples: `SEALEDGE_*`
  - Crate/binary names in rustdoc code blocks: match the actual renamed identifier (e.g., `sealedge-core`, `seal`, `sealedge-wasm`)

### Scripts + examples sweep (DOCS-04 + Phase 85 carry-forward)

- **D-09:** Scripts sweep:
  - `scripts/demo-attestation.sh:30` — `ENDPOINT="https://verify.trustedge.dev"` → **`https://verify.sealedge.dev`** (aspirational/placeholder; no live DNS change)
  - `scripts/fast-bench.sh:13` — comment already references "sealedge core crate" but still mentions the rename history; update prose for sealedge-final consistency
  - `scripts/consolidate-docs.sh:44` — `Project: trustedge — Privacy and trust at the edge.` (copyright-header substitution template in script body) → `Project: sealedge — Privacy and trust at the edge.`
  - `scripts/project/add-copyright.sh:33` — checks `copyright.*trustedge labs llc` (this is a legal-entity match, keep as-is; but verify the script's *output* template doesn't write `trustedge` anywhere)
  - Any additional `scripts/*.sh` prose found during plan-01 repo-wide grep

- **D-10:** `examples/cam.video/` sweep:
  - `examples/cam.video/README.md` — prose sweep
  - `examples/cam.video/record_and_wrap.rs` + `verify_cli.rs` — rustdoc `///` / `//!` sweep (covered by D-07 via the `examples/**/*.rs` glob) plus any literal string/prose inside the binary

- **D-11:** External references carry-forward from Phase 85:
  - `https://github.com/TrustEdge-Labs/trustedge` → `https://github.com/TrustEdge-Labs/sealedge` in every doc and rustdoc line (GitHub redirect covers the Phase 86→Phase 87 window)
  - `trustedgelabs.com` references — **leave as-is** (domain does not rename per v6.0 memory; Phase 88 updates the *content* there)

### Brand casing (reaffirmed from Phase 85)

- **D-12:** Apply Phase 85 D-11 through D-14 casing rules uniformly across all Phase 86 surfaces:
  - Prose / CLI help / log strings → `sealedge`
  - UI labels / titles / sentence-start → `Sealedge`
  - Constants / env vars → `SEALEDGE_*`
  - Generic English words containing "trust"/"edge" (e.g., "trust boundary", "edge device") — stay
  - `TrustEdge-Labs` (hyphenated org), `TRUSTEDGE LABS LLC` (legal entity) — stay

### Success criterion verification

- **D-13:** After Phase 86 lands, the following grep returns only `TrustEdge-Labs` (org), `TRUSTEDGE LABS LLC` (legal entity), and allowlisted historical artifacts (D-01a) — no other hits across non-planning source:
  ```
  grep -rin "trustedge" --include='*.md' --include='*.rs' --include='*.sh' --include='*.toml' --include='*.svelte' --include='*.ts' --include='*.json' --include='*.html' . \
    | grep -vE 'TrustEdge-Labs|TRUSTEDGE LABS LLC|\.planning/|\.git/|target/|node_modules/|\.claude/worktrees/|improvement-plan\.md|RFC_K256_SUPPORT\.md|security-review-platform\.md|actions/attest-sbom-action/'
  ```
  Expected: empty result (or commented-out/historical-record entries we explicitly accept — planner enumerates any exceptions during plan-01).

- **D-14:** Rustdoc rendering check: `cargo doc --workspace --no-deps` succeeds and the generated `target/doc/` HTML output is grep-clean for stray `trustedge` under the same allowlist. Planner chooses enforcement mechanism (one-time phase-close manual grep is sufficient; a permanent ci-check.sh hook is optional).

### Claude's Discretion

- **Plan granularity:** The planner splits Phase 86 along natural documentation boundaries. Suggested split (planner's call):
  - Plan 01: Root-level `.md` sweep (D-01 list, ~11 files) with hybrid treatment (D-02/D-03) applied to CHANGELOG + MIGRATION
  - Plan 02: `docs/**` sweep (D-04, all subtrees)
  - Plan 03: Other `.md` surfaces (D-06) — crate READMEs, `.github/`, `deploy/`, `web/demo/`, `examples/cam.video/README.md`
  - Plan 04: Rustdoc sweep (D-07, D-08) across `crates/**/*.rs` and `examples/**/*.rs`
  - Plan 05: Scripts + examples `.rs` + final grep audit (D-09, D-10) + verify D-13/D-14 clean at phase close
- **File-count budget:** Plan 02 and Plan 04 are the largest; planner may split further if complexity budget is exceeded.
- **Commit-granularity:** `cargo build --workspace --locked` green at every commit boundary (Phase 83/84/85 carry-forward). For doc-only commits this is trivially satisfied, but rustdoc renames in macro-heavy crates can trip doc tests — run `cargo test --doc --workspace` on any plan that touches rustdoc.
- **Rustdoc enforcement mechanism:** One-time `grep -rni 'trustedge' target/doc/` at phase close is sufficient. A permanent hook in `ci-check.sh` is nice-to-have, not required; planner decides based on plan budget.
- **`crates/wasm/` vs `crates/seal-wasm/`:** The scout found both paths. Planner verifies during plan-01 whether one is a Phase 83 rename leftover that should also be deleted in this phase's cleanup, or whether they're distinct crates. Do not rename or delete directories in Phase 86 — flag to a follow-up plan/phase if cleanup is needed.
- **Copyright year:** Stays at 2025 per Phase 85 D-06 carry-forward.
- **"Trust" / "edge" as domain vocabulary:** Preserve in prose (e.g., "trust boundary", "edge device", "edge computing") — these are not brand words.

### Folded Todos

None — no backlog todos fold into this phase.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Milestone-level (v6.0 scope)
- `.planning/REQUIREMENTS.md` §"Docs (documentation sweep)" — DOCS-01 (root docs), DOCS-02 (developer docs), DOCS-03 (code doc comments / rustdoc), DOCS-04 (scripts + examples)
- `.planning/PROJECT.md` §"Current Milestone" — v6.0 target, clean-break preference
- `.planning/ROADMAP.md` §"Phase 86: Documentation Sweep" (line 180) — goal + 4 success criteria

### Phase 83/84/85 decisions that carry forward
- `.planning/phases/83-crate-and-binary-rename/83-CONTEXT.md` — final binary names (`sealedge`, `seal`, `sealedge-server`, `sealedge-client`, `sealedge-platform-server`), crate names (`sealedge-*`), file extension (`.seal`)
- `.planning/phases/84-crypto-constants-file-extension/84-CONTEXT.md` — wire-format constant renames, `.se-attestation.json` extension
- `.planning/phases/85-code-sweep-headers-text-metadata/85-CONTEXT.md` — D-11/D-12/D-13 casing rules (reaffirmed in D-12 above), D-16 (long-form content = Phase 86), D-18 (rustdoc = Phase 86), D-19 (scripts prose mostly Phase 85 with carve-outs to Phase 86)

### Memory / user context
- `~/.claude/projects/-home-john-vault-projects-github-com-trustedge/memory/project_v6_rebrand.md` — scope boundaries, reserved phase numbers, clean-break rationale
- `~/.claude/projects/-home-john-vault-projects-github-com-trustedge/memory/feedback_clean_break_compat.md` — user's clean-break preference

### Surface starting points for the planner

**Root `.md` files (D-01 scope, 11 files):**
- `README.md`, `CLAUDE.md`, `DEPENDENCIES.md`, `SECURITY.md` — DOCS-01 named four
- `CONTRIBUTING.md`, `FEATURES.md`, `WASM.md`, `YUBIKEY_VERIFICATION.md`, `GEMINI.md` — expanded product/user-facing
- `CHANGELOG.md` (hybrid per D-02), `MIGRATION.md` (hybrid + new v6.0 section per D-03)

**Root `.md` files explicitly OUT of scope (D-01a, planner allowlists in grep check):**
- `improvement-plan.md`, `RFC_K256_SUPPORT.md`, `security-review-platform.md`

**docs/ tree (D-04, all subtrees):**
- `docs/README.md`, `docs/architecture.md`, `docs/roadmap.md`, `docs/landing-page.md`, `docs/manifest_cam_video.md`, `docs/third-party-attestation-guide.md`, `docs/yubikey-guide.md`
- `docs/developer/*` (5 files)
- `docs/designs/sbom-attestation-wedge.md`
- `docs/hardware/SECURE_NODE_MVP.md`
- `docs/legal/*` (5 files — apply D-05 entity-vs-product discipline)
- `docs/technical/*` (4 files)
- `docs/user/*` (including `docs/user/examples/*`)

**Other `.md` surfaces (D-06):**
- `crates/cli/README.md`, `crates/core/README.md`, `crates/seal-cli/README.md`, `crates/seal-protocols/README.md`, `crates/seal-wasm/README.md`, `crates/seal-wasm/pkg-bundler/README.md`, `crates/wasm/README.md`, `crates/wasm/pkg-bundler/README.md`, `crates/experimental/pubky/README.md`, `crates/experimental/pubky-advanced/README.md`
- `crates/core/AUTHENTICATION.md`, `crates/core/BENCHMARKS.md`, `crates/core/PERFORMANCE.md`, `crates/core/SOFTWARE_HSM_TEST_REPORT.md`
- `examples/cam.video/README.md`
- `deploy/digitalocean/README-deploy.md`
- `web/demo/README.md`
- `.github/pull_request_template.md`, `.github/README.md`

**Rustdoc (D-07, ~188 lines):**
- `crates/**/*.rs` — all source files; planner's first task is repo-wide grep `///.*[Tt]rustedge|//!.*[Tt]rustedge` with the legal-entity filter
- `examples/**/*.rs` — example programs

**Scripts + examples (D-09, D-10):**
- `scripts/demo-attestation.sh:30` — verify.trustedge.dev → verify.sealedge.dev
- `scripts/fast-bench.sh:13` — prose tidy
- `scripts/consolidate-docs.sh:44` — copyright-header template
- `scripts/project/add-copyright.sh:33` — verify legal-entity match is intentional
- `examples/cam.video/record_and_wrap.rs`, `verify_cli.rs`
- `examples/cam.video/Cargo.toml`, `device.key`, `device.pub` (binary files — no edits)

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- **Phase 85's `scripts/fix-copyright.sh`** is already adapted for `Project: sealedge` headers in `.rs` source. The `scripts/consolidate-docs.sh` script (D-09) contains a similar copyright-template string that missed Phase 85's sweep — update as a plain string edit.
- **Phase 85's grep-allowlist regex pattern** is the template for D-13's success check. Copy the `TrustEdge-Labs|TRUSTEDGE LABS LLC|\.planning/|…` exclusion into the planner's verification step.
- **`cargo doc --workspace --no-deps`** exists in the CLAUDE.md build commands section — D-14 reuses this as the rustdoc render check.

### Established Patterns
- **Hybrid historical-preservation pattern** (D-02/D-03) mirrors the Phase 85 approach to `.planning/milestones/` artifacts — new top-level narrative uses the new brand; historical records are frozen accurately. Pattern is legible to readers and preserves audit trail.
- **Casing rules (D-12)** — already applied consistently through Phase 85. Phase 86 reapplies the same four-way distinction (`sealedge` / `Sealedge` / `SEALEDGE_*` / preserve "trust"+"edge" generic).
- **Rustdoc doc-test interaction** — rustdoc code blocks marked with ```rust are compiled as doc tests. Renaming `use trustedge_core::…` to `use sealedge_core::…` in a rustdoc example updates both prose and the doc-test itself. Run `cargo test --doc --workspace` after Plan 04 to catch any doc-test regressions from rename mismatches.

### Integration Points
- **`include_str!` macro ↔ markdown content** — some crate roots embed README content into the lib (`include_str!("../README.md")` pattern). Phase 86 edits flow through the next cargo build. Planner verifies during plan-03 enumeration.
- **`cargo doc` output ↔ crate prose** — module-level `//!` doc in `crates/core/src/lib.rs` or `crates/wasm/src/lib.rs` renders as the crate landing page on docs.rs (if ever published). D-07 sweep updates these in-place.
- **GitHub redirect ↔ repo URL references** — Phase 85 D-05 established that GitHub's built-in redirect covers the Phase 85→Phase 87 window. Same mechanism covers Phase 86's URL edits — all repo URLs written as `/TrustEdge-Labs/sealedge` continue to resolve until Phase 87 renames the repo, after which they resolve without redirect.

</code_context>

<specifics>
## Specific Ideas

- **CHANGELOG.md v6.0 entry** — Phase 86 may leave the v6.0 entry stub (or may draft it). If drafted, content is: `## [6.0.0] - <date> — Sealedge Rebrand (trademark-driven clean break)` with bullet list of the renames from D-03. If stubbed, Phase 89 finalizes it as part of milestone close. Planner's call.
- **Doc-test regression check** — after Plan 04, `cargo test --doc --workspace` is a required verification gate. Rustdoc renames in macro-heavy crates historically trip doc-test compilation; include the gate in Plan 04's done-criteria.
- **Legal doc line-by-line review** — `docs/legal/*` (5 files) and `docs/legal/copyright.md` specifically need a closer read than bulk sed-style sweeps. Entity vs product distinction (D-05) requires judgment per occurrence. Planner allocates extra time to this subset of Plan 02.
- **crates/wasm vs crates/seal-wasm investigation** — noted in Claude's Discretion. If planner confirms one is a Phase 83 leftover, flag to a cleanup todo or small follow-up plan; do not delete directories in Phase 86.
- **Grep criterion 5 verification command** — exact command in D-13. Saved here for copy-paste during plan verification steps.

</specifics>

<deferred>
## Deferred Ideas

- **GitHub Action repo + Marketplace listing update** — Phase 88. `actions/attest-sbom-action/README.md` explicitly excluded from Phase 86.
- **Product website (`trustedgelabs.com`) content refresh** — Phase 88. Domain stays, content updates.
- **GitHub repository rename operation** — Phase 87. Phase 86 writes new URLs; Phase 87 does the GitHub-side rename.
- **Final validation across the full test matrix + CI + E2E** — Phase 89.
- **`crates/wasm/` directory cleanup** if it turns out to be a Phase 83 rename leftover — follow-up cleanup phase or ad-hoc todo (depending on planner's plan-01 enumeration).
- **Permanent ci-check.sh grep guard** for DOCS-03 rustdoc cleanliness — nice-to-have; one-time check at phase close is sufficient. Add as backlog item if desired.
- **Copyright year bump to 2026** — not a rename concern; handle as a separate annual sweep if ever needed.

### Reviewed Todos (not folded)
None — no todos were considered for this phase.

</deferred>

---

*Phase: 86-documentation-sweep*
*Context gathered: 2026-04-20*
