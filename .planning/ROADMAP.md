<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->

# Roadmap: TrustEdge

## Milestones

- ✅ **v1.0 Consolidation** - Phases 1-8 (shipped 2026-02-11)
- ✅ **v1.1 YubiKey Overhaul** - Phases 9-12 (shipped 2026-02-11)
- ✅ **v1.2 Scope Reduction** - Phases 13-14 (shipped 2026-02-12)
- ✅ **v1.3 Dependency Audit** - Phases 15-18 (shipped 2026-02-13)
- ✅ **v1.4 Placeholder Elimination** - Phases 19-23 (shipped 2026-02-13)
- ✅ **v1.5 Platform Consolidation** - Phases 24-27 (shipped 2026-02-22)
- ✅ **v1.6 Final Consolidation** - Phases 28-30 (shipped 2026-02-22)
- ✅ **v1.7 Security Hardening** - Phases 31-34 (shipped 2026-02-23)
- ✅ **v1.8 KDF Fix** - Phases 35-37 (shipped 2026-02-24)
- ✅ **v2.0 End-to-End Demo** - Phases 38-41 (shipped 2026-03-16)
- ✅ **v2.1 Data Lifecycle** - Phases 42-44 (shipped 2026-03-18)
- ✅ **v2.2 Security Remediation** - Phases 45-47 (shipped 2026-03-19)
- ✅ **v2.3 Security Testing** - Phases 48-51 (shipped 2026-03-21)
- ✅ **v2.4 Security Review Remediation** - Phases 52-53 (shipped 2026-03-22)
- ✅ **v2.5 Critical Security Fixes** - Phases 54-56 (shipped 2026-03-23)
- ✅ **v2.6 Security Hardening** - Phases 57-60 (shipped 2026-03-24)
- ✅ **v2.7 CI & Config Security** - Phases 61-63 (shipped 2026-03-25)
- ✅ **v2.8 High Priority Hardening** - Phases 64-67 (shipped 2026-03-26)
- ✅ **v2.9 Security Review P2 Remediation** - Phases 68-70 (shipped 2026-03-26)
- ✅ **v3.0 Release Polish** - Phases 71-74 (shipped 2026-03-27)
- ✅ **v4.0 SBOM Attestation Wedge** - Phases 75-78 (shipped 2026-04-03)
- ✅ **v5.0 Portfolio Polish** - Phases 79-82 (partial — 79-80 shipped 2026-04-05; 81-82 punted to post-rename)
- ✅ **v6.0 Sealedge Rebrand** - Phases 83-89 (shipped 2026-04-22)

## Phases

<details>
<summary>✅ v1.0 through v4.0 (Phases 1-78) - SHIPPED</summary>

See `.planning/milestones/` for archived roadmaps and requirements.

**78 phases, 116 plans shipped across 21 milestones.**

</details>

### ✅ v5.0 Portfolio Polish (Partial — 2 of 4 phases shipped)

- [x] **Phase 79: Self-Attestation CI** - Wire up end-to-end self-attestation in the CI release workflow (completed 2026-04-05)
- [x] **Phase 80: GitHub Action Marketplace** - Publish `TrustEdge-Labs/attest-sbom-action@v1` to GitHub Marketplace (completed 2026-04-05)
- [ ] **Phase 81: Demo GIF** - Record and embed demo GIF in README (punted — execute after v6.0 rebrand lands)
- [ ] **Phase 82: Product Landing Page** - Ship product landing page on trustedgelabs.com (punted — execute after v6.0 rebrand lands)

### ✅ v6.0 Sealedge Rebrand — SHIPPED (v6.0.0 released 2026-04-22)

**Milestone Goal:** Rename the product from "trustedge" to "sealedge" end-to-end — repo, crates, binaries, internal constants, docs, functions — clean break with no legacy compatibility path, while the TrustEdge-Labs org/brand retains its identity.

- [x] **Phase 83: Crate & Binary Rename** - Rename all workspace crates `trustedge-*` → `sealedge-*` and all CLI binaries (including `trst` → `seal`) to sealedge equivalents (completed 2026-04-18 — 7 plans, 7 commits)
- [x] **Phase 84: Crypto Constants & File Extension** - Replace `TRUSTEDGE-KEY-V1` / `TRUSTEDGE_ENVELOPE_V1` with sealedge equivalents (clean break, no backward-compat decrypt) and rename `.te-attestation.json` file extension to the sealedge form everywhere (completed 2026-04-18)
- [x] **Phase 85: Code Sweep — Headers, Text, Metadata** - Update all copyright/license headers, user-facing text (errors, logs, help, env vars, UI labels), and Cargo.toml metadata (description, repository, homepage, documentation) across the workspace (completed 2026-04-19 — 6 plans, ~25 commits)
- [x] **Phase 86: Documentation Sweep** - Update root project docs (README, CLAUDE.md, DEPENDENCIES.md, SECURITY.md), developer docs (docs/**), code doc comments (`///`, `//!`), and scripts/examples to reflect sealedge naming (completed 2026-04-20 — 5 plans)
- [x] **Phase 87: GitHub Repository Rename** - Rename monorepo `TrustEdge-Labs/trustedge` → `TrustEdge-Labs/sealedge` with GitHub's automatic redirect enabled; update local git remotes (completed 2026-04-21)
- [x] **Phase 88: External Action & Product Website** - Publish new GitHub Action repo under sealedge naming with SHA256 checksum verification; deprecate old `attest-sbom-action` marketplace listing with redirect; update product references on trustedgelabs.com (completed 2026-04-21)
- [x] **Phase 89: Final Validation** - Full workspace test suite, all GitHub Actions workflows, WASM + dashboard + Docker stack all green end-to-end under the new names (completed 2026-04-22)

See `.planning/milestones/v6.0-phases/` for archived phase artifacts.

## Phase Details

### Phase 79: Self-Attestation CI
**Goal**: Every TrustEdge GitHub release automatically attests its own binary, producing a `.te-attestation.json` and `build.pub` as downloadable release assets — with zero stored secrets.
**Depends on**: Phase 78 (v4.0 complete)
**Requirements**: CI-01, CI-02, CI-03, CI-04, HK-01
**Success Criteria** (what must be TRUE):
  1. After a release is published, the GitHub release page shows `.te-attestation.json` and `build.pub` as attached assets
  2. A user can download those assets and independently verify them with `trst verify-attestation` using the attached `build.pub` — no TrustEdge infrastructure needed
  3. The CI job generates a fresh Ed25519 keypair on every run; no signing key is stored in GitHub Secrets or the repository
  4. The CI job uses a pinned `anchore/sbom-action@v0` to generate a CycloneDX SBOM that is bound to the release binary
  5. The te-prove design doc is accessible in `.planning/ideas/` for future reference
**Plans:** 1/1 plans complete

Plans:
- [x] 79-01-PLAN.md — Fix self-attestation CI job and archive te-prove design doc

### Phase 80: GitHub Action Marketplace
**Goal**: Any project can add SBOM attestation to their CI with a single YAML snippet by installing `TrustEdge-Labs/attest-sbom-action@v1` from the GitHub Marketplace.
**Depends on**: Phase 79
**Requirements**: DIST-01, DIST-02, DIST-03, DIST-04
**Success Criteria** (what must be TRUE):
  1. A user can find `TrustEdge-Labs/attest-sbom-action` on the GitHub Marketplace and install it without leaving GitHub
  2. The action downloads the `trst` binary from GitHub Releases and verifies its SHA256 checksum before executing — it does not bundle a binary
  3. A user can configure the action with `sbom-path`, `binary-path`, `key-path`, and `trustedge-version` inputs in their workflow YAML
  4. The action README shows two working usage examples: one using a persistent signing key (stored as a secret) and one using an ephemeral key generated per-run
**Plans:** 1/1 plans complete

Plans:
- [x] 80-01-PLAN.md — Enhance action.yml (SHA256 verification), polish README (two usage examples), create separate repo and tag v1/v1.0.0, submit Marketplace listing

### Phase 81: Demo GIF (Punted — post-rename)
**Goal**: A developer landing on the README can immediately see what the product does — attest-sbom to verify-attestation — by watching an embedded GIF, without reading any prose.
**Depends on**: Phase 89 (v6.0 rebrand validated)
**Requirements**: VIS-01, VIS-03
**Success Criteria** (what must be TRUE):
  1. The README displays an embedded GIF that shows the complete attest-sbom → verify-attestation flow from a real terminal session
  2. The GIF is recorded from `scripts/demo.sh --local` (or the updated demo script) and reflects the actual CLI output — no staged or edited content
  3. The GIF is visible without clicking any links — embedded directly in the README at or near the top
**Plans:** 4 plans

Plans:
- [x] 88-01-PLAN.md — Action source + monorepo folder rename + REQUIREMENTS amendment (wave 1, autonomous, EXT-02/EXT-03)
- [x] 88-02-PLAN.md — ci.yml release-job extension (seal + seal.sha256 upload) + dogfood conversion to @v2 (wave 2, autonomous, EXT-02)
- [x] 88-03-PLAN.md — External gh repo rename + @v2 tag cut + Marketplace check + 88-VERIFICATION.md (wave 3, checkpoints, EXT-02/EXT-03)
- [x] 88-04-PLAN.md — Cross-repo trustedgelabs-website product-name sweep + SealVerifier rename + live-preview check (wave 1, checkpoints, EXT-04)
**UI hint**: yes

### Phase 82: Product Landing Page (Punted — post-rename)
**Goal**: A recruiter or prospective user visiting trustedgelabs.com immediately understands what Sealedge does, can run the quick start, and can reach the live verifier.
**Depends on**: Phase 81
**Requirements**: VIS-02
**Success Criteria** (what must be TRUE):
  1. A visitor to trustedgelabs.com can read a clear one-paragraph explanation of what Sealedge does and who it is for
  2. The page includes a copy-pasteable quick start showing how to install the Sealedge CLI and run its attestation command in three or fewer commands
  3. The page links directly to the live public verifier so a visitor can verify an attestation without leaving the page context
  4. The page links to the GitHub Action marketplace listing so a visitor can add attestation to their own CI immediately
**Plans:** 4 plans

Plans:
- [x] 88-01-PLAN.md — Action source + monorepo folder rename + REQUIREMENTS amendment (wave 1, autonomous, EXT-02/EXT-03)
- [x] 88-02-PLAN.md — ci.yml release-job extension (seal + seal.sha256 upload) + dogfood conversion to @v2 (wave 2, autonomous, EXT-02)
- [x] 88-03-PLAN.md — External gh repo rename + @v2 tag cut + Marketplace check + 88-VERIFICATION.md (wave 3, checkpoints, EXT-02/EXT-03)
- [x] 88-04-PLAN.md — Cross-repo trustedgelabs-website product-name sweep + SealVerifier rename + live-preview check (wave 1, checkpoints, EXT-04)
**UI hint**: yes

### Phase 83: Crate & Binary Rename
**Goal**: The entire Cargo workspace presents as sealedge — every crate is named `sealedge-*` (including `sealedge-seal-*` for the former `trustedge-trst-*` archive crates), every binary target is a sealedge-derived name (`trst` → `seal`, `trustedge` → `sealedge`, etc.), the `.trst` archive file extension is renamed to `.seal`, and the workspace still builds and tests green end-to-end.
**Depends on**: Phase 80 (v5.0 shipped portion complete)
**Requirements**: REBRAND-01, REBRAND-02, REBRAND-04a
**Success Criteria** (what must be TRUE):
  1. `cargo metadata` shows no package whose name starts with `trustedge-` — every workspace member is `sealedge-*`
  2. `cargo build --workspace --release` produces only sealedge-named binaries; no binary target in the workspace retains a `trustedge`-derived name (including the former `trst` → `seal`)
  3. Inter-crate dependencies in every Cargo.toml reference the new `sealedge-*` crate names; `cargo check --workspace` compiles cleanly
  4. All existing workspace tests still pass under the new crate/binary names (`cargo test --workspace` green)
  5. Archive files are written and read with the `.seal` extension across the archive CLI, core library, examples, and tests; no `.trst` literal remains in production code paths
**Plans:** 7/7 plans complete (completed 2026-04-18)

Plans:
- [x] 83-01-PLAN.md — Rename 6 root crates trustedge-* → sealedge-* (f38bd31)
- [x] 83-02-PLAN.md — Rename trst-family → sealedge-seal-*, trst → seal binary (92e8243)
- [x] 83-03-PLAN.md — Sweep .trst → .seal archive extension in Rust code (59ebcd2)
- [x] 83-04-PLAN.md — Rename dashboard + WASM npm packages + JS bindings (586644e)
- [x] 83-05-PLAN.md — Update CI workflows + shell scripts for sealedge naming (7322536)
- [x] 83-06-PLAN.md — Rename experimental pubky crates (4408d13)
- [x] 83-07-PLAN.md — Final verification + human checkpoint (fbe8ba8)

### Phase 84: Crypto Constants & File Extension
**Goal**: Wire-format constants and on-disk file extensions announce the product as sealedge — cleanly broken from the old trustedge-labelled values, with no backward-compatibility decrypt path for data encrypted under the old constants.
**Depends on**: Phase 83
**Requirements**: REBRAND-03, REBRAND-04b
**Success Criteria** (what must be TRUE):
  1. The encrypted key file header string is `SEALEDGE-KEY-V1` (not `TRUSTEDGE-KEY-V1`); keygen and unwrap produce and consume only the new header
  2. The HKDF domain-separation info parameter in envelope v2 is `SEALEDGE_ENVELOPE_V1` (not `TRUSTEDGE_ENVELOPE_V1`); envelopes sealed under the old constant intentionally fail to unseal
  3. Attestation files are written and read with the sealedge-branded extension (e.g. `.se-attestation.json`) across CLI subcommands, the platform endpoint, the GitHub Action, and the verify HTML page
  4. A targeted test proves that data produced with the old `TRUSTEDGE-*` constants is rejected cleanly (not silently decrypted) — confirming the clean break
**Plans:** 3 plans

Plans:
- [x] 84-01-PLAN.md — Rename ENCRYPTED_KEY_HEADER + HKDF info byte literal; add D-02 clean-break rejection tests
- [x] 84-02-PLAN.md — Sweep .te-attestation.json → .se-attestation.json in Rust sources + demo script
- [x] 84-03-PLAN.md — Sweep .te-attestation.json → .se-attestation.json in external assets (HTML, Action, deploy README)
**Status:** Complete (2026-04-18)

### Phase 85: Code Sweep — Headers, Text, Metadata
**Goal**: Every human-readable string emitted from the codebase or written in its source says "sealedge" — copyright headers, error messages, log lines, CLI help text, env var prefixes, dashboard UI labels, and Cargo.toml metadata URLs all match the new brand.
**Depends on**: Phase 84
**Requirements**: REBRAND-05, REBRAND-06, REBRAND-07
**Success Criteria** (what must be TRUE):
  1. Every `.rs` file's MPL-2.0 header reads `Project: sealedge` — a repo-wide grep for `Project: trustedge` returns zero results
  2. CLI help output, error messages, and log lines visible to a user contain no `trustedge` strings; environment variable prefixes are `SEALEDGE_*` (not `TRUSTEDGE_*`)
  3. Every Cargo.toml `description`, `repository`, `homepage`, and `documentation` field points at sealedge naming and the new repo URL
  4. The SvelteKit dashboard UI (titles, headings, labels, footer) renders "Sealedge" in place of "TrustEdge" in all user-facing copy
  5. A repo-wide grep (outside of archived `.planning/milestones/` history) for case-insensitive `trustedge` returns only intentional references to the `TrustEdge-Labs` org/brand
**Plans:** 6 plans
**UI hint**: yes

Plans:
- [x] 85-01-PLAN.md — Core crypto byte-literal domain rename + D-02 clean-break tests (chunk key, session key, genesis seed, manifest domain, MAGIC)
- [x] 85-02-PLAN.md — Experimental pubky-advanced crypto byte-literal rename + D-02 clean-break tests + audio demo header
- [x] 85-03-PLAN.md — Cargo.toml metadata sweep (repository URLs, docs URLs, descriptions, workspace comment block)
- [x] 85-04-PLAN.md — MPL-2.0 headers across all .rs and .sh files (Project: sealedge + new GitHub URL) + fix-copyright.sh templates
- [x] 85-05-PLAN.md — Production-code prose sweep (env vars, clap #[command(name=)], CLI help/error/log strings, inline // comments, test vectors, scripts echo prose)
- [x] 85-06-PLAN.md — SvelteKit dashboard UI compiled text (titles, headings, nav, footer, meta, package.json metadata)
**Status:** Complete (2026-04-19)

### Phase 86: Documentation Sweep
**Goal**: All project documentation — root docs, developer docs, code doc comments, and scripts — describes the product as sealedge, so a new reader never sees conflicting or stale brand references.
**Depends on**: Phase 85
**Requirements**: DOCS-01, DOCS-02, DOCS-03, DOCS-04
**Success Criteria** (what must be TRUE):
  1. README.md, CLAUDE.md, DEPENDENCIES.md, and SECURITY.md (if present) describe the product as sealedge throughout, with updated crate names, binary names, and install/usage snippets
  2. Developer docs under `docs/` (architecture.md, cli.md, development.md, testing.md, user/*) reflect the new crate names, binary names, file extensions, and env var prefixes
  3. `cargo doc --workspace --no-deps` produces rustdoc output where module-level and item-level doc comments render "sealedge" — no stale `trustedge` references remain in the rendered docs
  4. Scripts in `scripts/` and examples under `examples/cam.video/` invoke the new binary names and reference the new attestation file extension
**Plans:** 5 plans
- [ ] 86-01-PLAN.md — Root .md sweep (11 files) with hybrid treatment of CHANGELOG.md + MIGRATION.md v6.0 section
- [ ] 86-02-PLAN.md — docs/** sweep (37 files across docs/, developer/, designs/, hardware/, legal/, technical/, user/)
- [ ] 86-03-PLAN.md — Crate READMEs + .github/ + deploy/ + web/demo/ + examples/cam.video/README.md + scripts/*.md (21 files)
- [ ] 86-04-PLAN.md — Rustdoc /// and //! sweep across crates/**/*.rs + examples/**/*.rs (16 files) + cargo test --doc gate
- [ ] 86-05-PLAN.md — scripts/*.sh prose carve-outs + D-13 repo-wide grep audit + D-14 cargo doc grep-clean audit

### Phase 87: GitHub Repository Rename
**Goal**: The monorepo lives at `TrustEdge-Labs/sealedge` on GitHub with working redirects from the old URL, local git remotes updated, and in-repo links pointing at the new location.
**Depends on**: Phase 86
**Requirements**: EXT-01
**Success Criteria** (what must be TRUE):
  1. The repo is accessible at `https://github.com/TrustEdge-Labs/sealedge`; requests to `https://github.com/TrustEdge-Labs/trustedge` automatically redirect (GitHub's built-in redirect) and do not 404
  2. The local working clone's `origin` remote URL points to the new `sealedge` repo; `git push` and `git pull` operate against the renamed repo without manual URL fixes
  3. In-repo markdown and Cargo.toml references to the repository URL (now updated in Phase 85–86) resolve correctly against the renamed repo
**Plans:** 2/2 plans complete

Plans:
- [x] 87-01-PLAN.md — Straggler URL cleanup commit — 3 tracked files (.github/ISSUE_TEMPLATE/config.yml, .github/workflows/cla.yml, deploy/digitalocean/app.yaml) updated and pushed before the rename
- [x] 87-02-PLAN.md — User-gated `gh repo rename` + local remote URL update + D-13 4-check verification gate (curl 301 redirect, git fetch/remote, CI green post-rename, DO App Platform auto-deploy) + 87-VERIFICATION.md evidence capture

### Phase 88: External Action & Product Website
**Goal**: Sealedge's external distribution surface — the GitHub Action and the product references on trustedgelabs.com — matches the new brand, with the old action clearly deprecated and redirected so existing users can migrate without breakage.
**Depends on**: Phase 87
**Requirements**: EXT-02, EXT-03, EXT-04
**Success Criteria** (what must be TRUE):
  1. A new GitHub Action repo exists under sealedge naming and is published to the GitHub Marketplace with equivalent functionality to the old `attest-sbom-action`, including SHA256 checksum verification of the downloaded binary (separate repo work — cross-repo deliverable)
  2. The old `TrustEdge-Labs/attest-sbom-action` marketplace listing is marked deprecated and its README redirects readers to the new listing; existing consumers of `@v1` are not silently broken but are clearly told to migrate
  3. Product-page content on `trustedgelabs.com` (served from the `trustedgelabs-website` repo, which itself is not renamed) advertises the product as "Sealedge" — any in-repo website-content files referencing the product name are updated
**Plans:** 4/4 plans complete

Plans:
- [x] 88-01-PLAN.md — Action source + monorepo folder rename + REQUIREMENTS amendment (wave 1, autonomous, EXT-02/EXT-03)
- [x] 88-02-PLAN.md — ci.yml release-job extension (seal + seal.sha256 upload) + dogfood conversion to @v2 (wave 2, autonomous, EXT-02)
- [x] 88-03-PLAN.md — External gh repo rename + @v2 tag cut + Marketplace check + 88-VERIFICATION.md (wave 3, checkpoints, EXT-02/EXT-03)
- [x] 88-04-PLAN.md — Cross-repo trustedgelabs-website product-name sweep + SealVerifier rename + live-preview check (wave 1, checkpoints, EXT-04)

### Phase 89: Final Validation
**Goal**: End-to-end proof that nothing functional regressed during the rebrand — every test, every CI workflow, and every runtime deployment target works under the new names.
**Depends on**: Phase 88
**Requirements**: VALID-01, VALID-02, VALID-03
**Success Criteria** (what must be TRUE):
  1. `cargo test --workspace` passes with all 471 tests green under the new crate/binary/constant names
  2. Feature-matrix tests pass for `yubikey`, `http`, `postgres`, `ca`, and `openapi` combinations (per the existing CI matrix)
  3. All GitHub Actions workflows (ci.yml, semver.yml, wasm-tests.yml, release workflow, self-attestation job) run green on a push to the renamed repo
  4. The WASM build succeeds, `web/dashboard/` builds and type-generates cleanly, and the Docker Compose stack (platform + postgres + dashboard) starts and runs the demo script end-to-end under the new names
**Plans:** 4 plans

Plans:
- [x] 89-01-PLAN.md — scripts/validate-v6.sh + D-10 straggler grep audit + MIGRATION.md hybrid-gate action-rename row (wave 1, autonomous, VALID-01/VALID-03)
- [x] 89-02-PLAN.md — Run validate-v6.sh + WASM/dashboard/docker evidence capture + wasm-tests.yml/semver.yml workflow_dispatch + 89-VERIFICATION.md draft (wave 2, checkpoint for dashboard smoke, VALID-01/VALID-03)
- [x] 89-03-PLAN.md — RELEASE-NOTES-v6.0.0.md + user-gated v6.0.0 tag cut + tag-push CI verification + 89-VERIFICATION.md §2.3 close (wave 3, checkpoint for tag cut, VALID-02)
- [x] 89-04-PLAN.md — ROADMAP/PROJECT milestone-close flip + archive Phases 83-89 to .planning/milestones/v6.0-phases/ (wave 4, autonomous, VALID-01/VALID-02/VALID-03)

## Progress

| Phase | Milestone | Plans Complete | Status | Completed |
|-------|-----------|----------------|--------|-----------|
| 75. Core Attestation Library | v4.0 | 1/1 | Complete | 2026-04-02 |
| 76. CLI + Platform Endpoint | v4.0 | 2/2 | Complete | 2026-04-02 |
| 77. Verify Page + Deployment + Demo | v4.0 | 3/3 | Complete | 2026-04-03 |
| 78. Distribution | v4.0 | 2/2 | Complete | 2026-04-03 |
| 79. Self-Attestation CI | v5.0 | 1/1 | Complete | 2026-04-05 |
| 80. GitHub Action Marketplace | v5.0 | 1/1 | Complete | 2026-04-05 |
| 81. Demo GIF | v5.0 | 0/? | Punted (post-rename) | - |
| 82. Product Landing Page | v5.0 | 0/? | Punted (post-rename) | - |
| 83. Crate & Binary Rename | v6.0 | 7/7 | Complete | 2026-04-18 |
| 84. Crypto Constants & File Extension | v6.0 | 3/3 | Complete | 2026-04-18 |
| 85. Code Sweep — Headers, Text, Metadata | v6.0 | 6/6 | Complete | 2026-04-19 |
| 86. Documentation Sweep | v6.0 | 5/5 | Complete | 2026-04-20 |
| 87. GitHub Repository Rename | v6.0 | 2/2 | Complete | 2026-04-21 |
| 88. External Action & Product Website | v6.0 | 4/4 | Complete | 2026-04-21 |
| 89. Final Validation | v6.0 | 4/4 | Complete | 2026-04-22 |
