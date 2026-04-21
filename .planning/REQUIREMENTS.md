<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->

# Requirements: v6.0 Sealedge Rebrand

**Defined:** 2026-04-18
**Core Value:** Rename the product from "trustedge" to "sealedge" end-to-end so it can move past its trademark constraint. Clean break, no legacy compatibility path. TrustEdge-Labs org/brand retains its identity; only the product renames.

## v6.0 Requirements

Requirements for this milestone. Each maps to roadmap phases.

### Rebrand (core rename)

- [x] **REBRAND-01**: All workspace crates renamed `trustedge-*` → `sealedge-*` across Cargo.toml manifests, workspace members, and inter-crate deps ✓ Phase 83
- [x] **REBRAND-02**: All CLI binaries renamed (`trustedge` → `sealedge`, `trustedge-server` → `sealedge-server`, `trustedge-client` → `sealedge-client`, `trustedge-platform-server` → `sealedge-platform-server`, `trst` → `seal`, `inspect-trst` → `inspect-seal`) — no binary retains a trustedge-derived name ✓ Phase 83
- [x] **REBRAND-03**: Crypto wire-format constants replaced (`TRUSTEDGE-KEY-V1` → `SEALEDGE-KEY-V1`, `TRUSTEDGE_ENVELOPE_V1` HKDF domain-separation → `SEALEDGE_ENVELOPE_V1`) — clean break, no backward-compat decrypt of existing data ✓ Phase 84
- **REBRAND-04** (split into 04a and 04b, see traceability below):
  - [x] **REBRAND-04a**: `.trst` archive extension → `.seal` across CLI, core library, examples, tests ✓ Phase 83
  - [x] **REBRAND-04b**: `.te-attestation.json` → `.se-attestation.json` across CLI, platform endpoint, GitHub Action, verify page ✓ Phase 84
- [ ] **REBRAND-05**: Copyright/license headers in every `.rs` file say `Project: sealedge` (no `trustedge` strings remain)
- [ ] **REBRAND-06**: All user-facing text updated — error messages, log output, CLI help text, env-var prefixes (`TRUSTEDGE_*` → `SEALEDGE_*`), dashboard UI labels and titles
- [ ] **REBRAND-07**: Cargo.toml metadata updated (`description`, `repository`, `homepage`, `documentation` URLs) for every workspace crate

### Docs (documentation sweep)

- [x] **DOCS-01**: Root project docs updated — `README.md`, `CLAUDE.md`, `DEPENDENCIES.md`, `SECURITY.md` (if present) reflect sealedge throughout
- [x] **DOCS-02**: Developer docs directory swept — `docs/architecture.md`, `docs/cli.md`, `docs/development.md`, `docs/testing.md`, `docs/user/*` updated for new names
- [x] **DOCS-03**: Code doc comments (`///`, `//!`) and module docstrings updated — `cargo doc` renders sealedge everywhere
- [x] **DOCS-04**: Scripts and examples updated — `scripts/*.sh`, `examples/cam.video/*`, demo scripts invoke new binary names and reference new file extension

### Ext (external surfaces)

- [ ] **EXT-01**: GitHub monorepo renamed `TrustEdge-Labs/trustedge` → `TrustEdge-Labs/sealedge` with GitHub's automatic redirect enabled; local git remotes updated
- [ ] **EXT-02**: The `TrustEdge-Labs/attest-sbom-action` repo is renamed to `TrustEdge-Labs/sealedge-attest-sbom-action` via `gh repo rename`; action source references sealedge/seal; a new `@v2` tag ships the rebranded action; SHA256 checksum verification of the downloaded binary is preserved
- [ ] **EXT-03**: GitHub's built-in 301 redirect covers existing `uses: TrustEdge-Labs/attest-sbom-action@v1` references; the pre-rebrand `@v1` tag stays frozen; the post-rename README carries a short notice pointing readers to `@v2` and the renamed repo
- [ ] **EXT-04**: Product references on `trustedgelabs.com` updated — the labs brand stays, but the advertised product is now "Sealedge"; any in-repo website content (trustedgelabs-website references) updated

### Valid (verify nothing broke)

- [ ] **VALID-01**: Full workspace test suite passes under new names — all 471 tests green across default, yubikey, http, postgres, ca, and openapi feature combinations
- [ ] **VALID-02**: All GitHub Actions workflows green (ci.yml, semver.yml, wasm-tests.yml, release workflow, self-attestation job)
- [ ] **VALID-03**: WASM builds, dashboard build + type generation, and Docker compose stack (platform + postgres + dashboard) all start clean and the demo script runs end-to-end under the new names

## Future Requirements

Deferred to later milestones:

- Record demo GIF and embed in README (post-rename, was v5.0 Phase 81)
- Ship product landing page on trustedgelabs.com (post-rename, was v5.0 Phase 82)
- C2PA compatibility for media profiles (cam.video, audio) — first amendment auditor use case
- Verification badge endpoint for README embedding
- SBOM diff/drift detection between attested versions
- te-prove: FOSS supply chain trust policy engine (parked in `.planning/ideas/`)

## Out of Scope

Explicit exclusions for this milestone:

- **Backward-compatible decryption of existing `TRUSTEDGE-KEY-V1` / `TRUSTEDGE_ENVELOPE_V1` data** — solo dev, no production users, clean break is acceptable and avoids legacy code paths forever
- **Rebranding the `TrustEdge-Labs` GitHub org or `trustedgelabs.com` domain** — only the product renames; the labs identity is retained
- **Rebranding the `trustedgelabs-website` repo name** — the labs-owned website repo keeps its name; only its product-page content changes
- **Publishing crates to `crates.io` under new names** — workspace is not currently published; publishing is a separate future initiative
- **Adding new features or behaviors during the rebrand** — rename-only; functional changes come in subsequent milestones
- **Tagging an official `v6.0` semver release on the old repo name before the GitHub repo rename** — the release tag is cut under the new repo name to avoid fragmented release history

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| REBRAND-01  | Phase 83 — Crate & Binary Rename | ✓ Complete (2026-04-18) |
| REBRAND-02  | Phase 83 — Crate & Binary Rename | ✓ Complete (2026-04-18) |
| REBRAND-03  | Phase 84 — Crypto Constants & File Extension | ✓ Complete (2026-04-18) |
| REBRAND-04a | Phase 83 — Crate & Binary Rename (`.trst` → `.seal`) | ✓ Complete (2026-04-18) |
| REBRAND-04b | Phase 84 — Crypto Constants & File Extension (`.te-attestation.json` → `.se-attestation.json`) | ✓ Complete (2026-04-18) |
| REBRAND-05  | Phase 85 — Code Sweep (Headers, Text, Metadata) | Pending |
| REBRAND-06  | Phase 85 — Code Sweep (Headers, Text, Metadata) | Pending |
| REBRAND-07  | Phase 85 — Code Sweep (Headers, Text, Metadata) | Pending |
| DOCS-01     | Phase 86 — Documentation Sweep | Complete |
| DOCS-02     | Phase 86 — Documentation Sweep | Complete |
| DOCS-03     | Phase 86 — Documentation Sweep | Complete |
| DOCS-04     | Phase 86 — Documentation Sweep | Complete |
| EXT-01      | Phase 87 — GitHub Repository Rename | Pending |
| EXT-02      | Phase 88 — External Action & Product Website | Pending |
| EXT-03      | Phase 88 — External Action & Product Website | Pending |
| EXT-04      | Phase 88 — External Action & Product Website | Pending |
| VALID-01    | Phase 89 — Final Validation | Pending |
| VALID-02    | Phase 89 — Final Validation | Pending |
| VALID-03    | Phase 89 — Final Validation | Pending |

**Coverage:** 18/18 requirements mapped (100%) — no orphans, no duplicates.

---

<!-- Last updated: 2026-04-18 -->
