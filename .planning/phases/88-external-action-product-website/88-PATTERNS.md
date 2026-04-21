# Phase 88: External Action & Product Website - Pattern Map

**Mapped:** 2026-04-21
**Files analyzed:** ~28 files across 2 repos (sealedge monorepo + trustedgelabs-website)
**Analogs found:** 10 / 10 scope groups (every file in scope has a clear in-repo analog — all in prior v6.0 phases 83, 85, 86, 87)
**Phase shape:** Mechanical rename-in-place sweep (Phases 83/85/86/87 carry-forward) + one external `gh repo rename` operation (Phase 87 carry-forward) + cross-repo parallel sweep (new surface, but disciplined by the Phase 85/86 allowlisted-grep pattern). No net-new construction.

**No RESEARCH.md for this phase.** All patterns are extracted from existing sealedge artifacts (prior phases' plans, SUMMARYs, and source files Phase 88 edits).

---

## File Classification

All files in scope are grouped into the 6 plan-shaped clusters implied by CONTEXT.md §"Claude's Discretion" and D-17 (cross-repo plan split at planner's discretion).

### Group A — Action source in-place rewrite (D-01, D-06, D-13 partial)

| File | Role | Data Flow | Closest Analog | Match Quality |
|------|------|-----------|----------------|---------------|
| `actions/attest-sbom-action/action.yml` | composite-action YAML (metadata + inline shell) | request-response (CI-time) | `.github/workflows/cla.yml` (Phase 87 Plan 01) + `.github/workflows/ci.yml` self-attest job | role-match (YAML config with embedded release-URL refs + product-name metadata); planner treats it as a **YAML-native targeted sed** the same way Phase 87 Plan 01 did for `cla.yml` |
| `actions/attest-sbom-action/README.md` | public-facing README (Marketplace + GitHub) | prose | `crates/seal-cli/README.md` + Phase 86 D-02/D-03 hybrid treatment for CHANGELOG/MIGRATION | role-match with Phase 86 prose sweep; **add a short "Renamed from attest-sbom-action" notice** mirrors MIGRATION.md D-03's "v6.0: trustedge → sealedge rebrand — clean break" top section pattern |

### Group B — Monorepo folder rename (D-06)

| File | Role | Data Flow | Closest Analog | Match Quality |
|------|------|-----------|----------------|---------------|
| `actions/attest-sbom-action/` → `actions/sealedge-attest-sbom-action/` | directory rename | mechanical move | Phase 83 Plan 01 crate-directory normalization (`crates/trustedge-cli/` → `crates/cli/`, 4 dirs total) | **exact** (same `git mv <old-dir> <new-dir>` mechanic, workspace/tree green at commit boundary, no content diff in the move commit) |

### Group C — ci.yml release job extension + dogfood conversion (D-12, D-14)

| File | Role | Data Flow | Closest Analog | Match Quality |
|------|------|-----------|----------------|---------------|
| `.github/workflows/ci.yml` (self-attest job, lines 171-222) | workflow YAML (existing job extension + `uses:` swap) | batch / release-event-driven | **itself** — the existing inline self-attest job IS the pattern; extension mirrors the existing `Upload attestation assets` step shape (lines 215-222) | **exact** (self-analog: the new `seal` + `seal.sha256` upload step sits beside the existing `gh release upload` call with the same `--clobber` flag and `${{ github.ref_name }}` ref); the `uses:` swap mirrors Phase 86's `@v0.24.0` SHA-pinned reference shape |

### Group D — REQUIREMENTS.md wording amendment (D-02)

| File | Role | Data Flow | Closest Analog | Match Quality |
|------|------|-----------|----------------|---------------|
| `.planning/REQUIREMENTS.md` §Ext (EXT-02, EXT-03) | requirements document | prose | Phase 85/86 requirement-update commits (commit style `docs(N): ...`) + Phase 87 Plan 01 commit pattern | role-match (amendment to prior requirement wording; atomic commit; separate from execution commits) |

### Group E — Cross-repo website: TrstVerifier rename + App import (D-15)

| File | Role | Data Flow | Closest Analog | Match Quality |
|------|------|-----------|----------------|---------------|
| `trustedgelabs-website/src/components/TrstVerifier.tsx` → `SealVerifier.tsx` | React component (file rename + identifier rename) | request-response (browser-side WASM verify) | Phase 83 Plan 01 binary-source-file rename (`trustedge-server.rs` → `sealedge-server.rs` via `git mv` + inline identifier sweep) | **exact** (same pattern: `git mv` the file, rename the exported symbol, update all importers; here the importer is `Solution.tsx` line 12, not multi-crate consumers, so blast radius is 1 file) |

### Group F — Cross-repo website: text/branding sweep across ~19 TSX + 3 metadata files (D-15, D-18)

| File | Role | Data Flow | Closest Analog | Match Quality |
|------|------|-----------|----------------|---------------|
| `trustedgelabs-website/src/components/{Hero,Features,Footer,Problem,Solution,UseCases,EnterpriseSolutions,IntegrationGuide,Security,CodeExamples,ArchiveSystem,PerformanceBenchmarks,TechnicalCapabilities,Contact,GetStarted,Thanks,PrivacyPolicy,TermsOfService,WasmDemo}.tsx` | React components (prose + JSX text + SVG labels) | prose | Phase 85 `D-11–D-14` casing rules + Phase 86 `D-13` allowlisted-grep discipline | **exact** for methodology (brand vs product distinction: `TrustEdge Labs` company stays, `TrustEdge`/`trustedge` product swaps to `Sealedge`/`sealedge`); grep-allowlist driven; one-time sweep |
| `trustedgelabs-website/index.html`, `README.md`, `package.json` | HTML entry + repo README + npm metadata | metadata | Phase 85 Cargo.toml metadata sweep (D-07 / D-08) + Phase 86 `.github/README.md` sweep | role-match (check `<title>`, meta description, package description/keywords) |

---

## Pattern Assignments

Each file (or file group) below has the concrete excerpt the planner can reference in PLAN.md `<action>` blocks and `acceptance_criteria` lists.

---

### Group A.1 — `actions/attest-sbom-action/action.yml` (YAML, composite action)

**Analog:** `.github/workflows/cla.yml` targeted edits (Phase 87 Plan 01 Task 1) — **line-exact substring swaps preserving YAML structure, no reflowing**.

**Current state excerpt** (`actions/attest-sbom-action/action.yml:1-44`, the blocks Phase 88 edits):

```yaml
# Copyright (c) 2025 TRUSTEDGE LABS LLC
# MPL-2.0: https://mozilla.org/MPL/2.0/
# Project: trustedge — Privacy and trust at the edge.

name: 'TrustEdge SBOM Attestation'
description: 'Attest a binary artifact with its CycloneDX SBOM using TrustEdge'

branding:
  icon: 'shield'
  color: 'blue'

inputs:
  binary:
    description: 'Path to the binary artifact to attest'
    required: true
  sbom:
    description: 'Path to CycloneDX JSON SBOM file'
    required: true
  key:
    description: 'Path to Ed25519 device key (generates ephemeral if not provided)'
    required: false
    default: ''
  trst-version:
    description: 'TrustEdge release version to download (e.g., v4.0.0)'
    required: false
    default: 'latest'

outputs:
  attestation-path:
    description: 'Path to generated .se-attestation.json file'
    value: ${{ steps.attest.outputs.attestation-path }}

runs:
  using: composite
  steps:
    - name: Download trst binary
      shell: bash
      run: |
        VERSION="${{ inputs.trst-version }}"
        if [ "$VERSION" = "latest" ]; then
          BASE_URL="https://github.com/TrustEdge-Labs/trustedge/releases/latest/download"
        else
          BASE_URL="https://github.com/TrustEdge-Labs/trustedge/releases/download/${VERSION}"
        fi

        # Download binary
        curl -fsSL "${BASE_URL}/trst" -o "${{ runner.temp }}/trst"
```

**Expected transformation** (exhaustive — all substitutions required by D-03, D-13, D-06 metadata updates; matches Phase 87 Plan 01 style of line-by-line substitution):

| Line(s) | From | To | Category |
|---------|------|-----|----------|
| 3 | `# Project: trustedge — Privacy and trust at the edge.` | `# Project: sealedge — Privacy and trust at the edge.` | copyright header (Phase 85 D-04 carry-forward) |
| 5 | `name: 'TrustEdge SBOM Attestation'` | `name: 'Sealedge SBOM Attestation'` | Marketplace listing title (D-10) |
| 6 | `description: 'Attest a binary artifact with its CycloneDX SBOM using TrustEdge'` | `description: 'Attest a binary artifact with its CycloneDX SBOM using Sealedge'` | Marketplace description (D-10) |
| 23 | `  trst-version:` | `  seal-version:` | input name rename (D-13) |
| 24 | `    description: 'TrustEdge release version to download (e.g., v4.0.0)'` | `    description: 'Sealedge release version to download (e.g., v6.0.0)'` | input description |
| 36 | `    - name: Download trst binary` | `    - name: Download seal binary` | step name |
| 39 | `        VERSION="${{ inputs.trst-version }}"` | `        VERSION="${{ inputs.seal-version }}"` | input read callsite |
| 41, 43 | `https://github.com/TrustEdge-Labs/trustedge/releases/...` | `https://github.com/TrustEdge-Labs/sealedge/releases/...` | release-URL base (D-01 / relies on Phase 87 rename + 301 redirect) |
| 47 | `curl -fsSL "${BASE_URL}/trst" -o "${{ runner.temp }}/trst"` | `curl -fsSL "${BASE_URL}/seal" -o "${{ runner.temp }}/seal"` | binary name (D-13, Phase 83 D-01 carry-forward) |
| 48 | `chmod +x "${{ runner.temp }}/trst"` | `chmod +x "${{ runner.temp }}/seal"` | binary path |
| 51-53 | `trst.sha256` references (3 sites) | `seal.sha256` references | SHA256 filename (D-05 parity pattern) |
| 68-73 | `"${{ runner.temp }}/trst" keygen ...`, env vars `TRST_KEY`/`TRST_PUB`, `ephemeral.key`/`ephemeral.pub` | `"${{ runner.temp }}/seal" keygen ...`, env vars `SEAL_KEY`/`SEAL_PUB` (or keep `DEVICE_KEY`/`DEVICE_PUB` — planner picks; prefer `SEAL_*` for consistency with Phase 85 D-13 env-var casing rule) | binary + env var sweep |
| 79, 82 | `TRST_KEY`, `TRST_PUB` env var writes | `SEAL_KEY`, `SEAL_PUB` | env vars (D-13 carry-forward) |
| 90 | `"${{ runner.temp }}/trst" attest-sbom \` | `"${{ runner.temp }}/seal" attest-sbom \` | binary invocation |
| 93-94 | `--device-key "$TRST_KEY"` / `--device-pub "$TRST_PUB"` | `--device-key "$SEAL_KEY"` / `--device-pub "$SEAL_PUB"` | env var reads |

**Pattern to copy from Phase 87 Plan 01 Task 1** (verbatim discipline notes):

> "Use the Edit tool for each file (preferred over `sed` for deterministic outcomes and because these are 3 tracked config files)." ... "Keep line numbering intact — the edits are pure substring swaps, no line insertions or deletions. Do NOT reflow YAML whitespace."

Applies directly to `action.yml`: the action.yml is exactly 99 lines; every edit is a line-local substring swap; no step reordering.

**Acceptance-criteria shape** (copy Phase 87 Plan 01 Task 1 acceptance criteria shape verbatim):

```
grep -c 'trustedge' actions/attest-sbom-action/action.yml  # returns 0
grep -c 'sealedge' actions/attest-sbom-action/action.yml  # returns N (enumerate in plan)
grep -c 'trst'      actions/attest-sbom-action/action.yml  # returns 0 for binary refs; caveat: 'trst-version' input-name appears in 1 remaining line if we keep backward-compat (we do NOT per D-13 clean break)
grep -c 'seal-version' actions/attest-sbom-action/action.yml  # returns 3 (input key, description reference, shell read)
git diff --stat actions/attest-sbom-action/action.yml  # expected ~15-18 insertions / ~15-18 deletions; no line-count delta
```

---

### Group A.2 — `actions/attest-sbom-action/README.md` (prose + YAML code examples)

**Analog:** `MIGRATION.md` top-section-notice pattern (Phase 86 D-03) + `crates/seal-cli/README.md` product-name-swap pattern (Phase 86 D-06).

**Current state excerpt** (`actions/attest-sbom-action/README.md:1-6, 22, 46, 62-65, 93, 112-125`):

```markdown
# TrustEdge SBOM Attestation Action

> Attest a binary artifact with its CycloneDX SBOM using TrustEdge — one YAML line, cryptographic proof.

...

  uses: TrustEdge-Labs/attest-sbom-action@v1

...

    trst-version: 'v4.0.0'

...

trst keygen --out-key build.key --out-pub build.pub --unencrypted
base64 -w0 build.key   # paste this as TRUSTEDGE_KEY secret

...

1. Downloads the `trst` binary from [TrustEdge-Labs/trustedge releases](https://github.com/TrustEdge-Labs/trustedge/releases) and verifies its SHA256 checksum ...

...

curl -X POST https://verify.trustedge.dev/v1/verify-attestation \
  -H "Content-Type: application/json" \
  -d @my-app.se-attestation.json

## Links

- [TrustEdge repository](https://github.com/TrustEdge-Labs/trustedge)
- [TrustEdge public verifier](https://verify.trustedge.dev)
```

**Expected transformation** (full sweep + top-notice addition):

1. **Add a short "Renamed from" notice at the top** — mirrors Phase 86 D-03 MIGRATION.md hybrid-treatment pattern. Sample (planner's wording; ~3 lines per D-08):

```markdown
# Sealedge SBOM Attestation Action

> **Renamed from `TrustEdge-Labs/attest-sbom-action`.** This repo was renamed in v6.0 to match the Sealedge product name. `@v1` stays frozen as the pre-rebrand behavior; `@v2+` uses Sealedge binary/URL naming. GitHub's built-in 301 redirect covers existing `uses: TrustEdge-Labs/attest-sbom-action@v1` references.

> Attest a binary artifact with its CycloneDX SBOM using Sealedge — one YAML line, cryptographic proof.
```

2. **Sweep product-name references** per Phase 85 D-11–D-14 casing rules (Phase 86 D-12 carry-forward):
   - `TrustEdge SBOM Attestation Action` → `Sealedge SBOM Attestation Action`
   - `using TrustEdge` (prose) → `using Sealedge`
   - `TrustEdge public verifier` (prose) → `Sealedge public verifier`
   - `trst` binary + CLI examples → `seal`
   - `trst-version:` YAML example → `seal-version:`
   - `TRUSTEDGE_KEY` secret-name example → `SEALEDGE_KEY` (Phase 85 D-10 carry-forward)
   - `TrustEdge-Labs/attest-sbom-action@v1` (examples) → `TrustEdge-Labs/sealedge-attest-sbom-action@v2`
   - `https://github.com/TrustEdge-Labs/trustedge` → `https://github.com/TrustEdge-Labs/sealedge` (links; Phase 87 rename + redirect)
   - `https://verify.trustedge.dev` → `https://verify.sealedge.dev` (Phase 86 D-09 `scripts/demo-attestation.sh` aspirational-endpoint precedent)
   - `.te-attestation.json` references — **leave alone**; Phase 84 renamed to `.se-attestation.json` and the action source already uses that; README should match the action source

**Verbatim commit message** (follows Phase 87 Plan 01 commit convention: `fix(N): <scope>` for renames, `docs(N): <scope>` for docs; planner picks `docs(88)` since the README edit is a prose sweep).

---

### Group B — Monorepo folder rename `actions/attest-sbom-action/` → `actions/sealedge-attest-sbom-action/`

**Analog (exact match):** Phase 83 Plan 01 `git mv crates/trustedge-cli crates/cli` (+ 3 other crate dirs in the same commit).

**Excerpt from Phase 83 Plan 01 (`83-01-PLAN.md:154-178`, the action block):**

```markdown
Execute the following substitutions exactly.

**1. Rename crate directory:**

    git mv crates/trustedge-cli crates/cli

**2. Update workspace root `Cargo.toml`:**
- Replace the `members` list entry `"crates/trustedge-cli"` with `"crates/cli"`.
...

**4. Rename binary source files:**

    git mv crates/core/src/bin/trustedge-server.rs crates/core/src/bin/sealedge-server.rs
    git mv crates/core/src/bin/trustedge-client.rs crates/core/src/bin/sealedge-client.rs
```

**Pattern to copy:**
- Use `git mv <old> <new>` (preserves git rename tracking — `git log --follow` works post-rename).
- Do the rename in the **same commit** that updates inbound references. In Phase 88 this means the `ci.yml` `uses:` swap (Group C) either:
  - (a) lands in the same commit as the folder rename (single atomic commit; matches Phase 83 Plan 01 style), OR
  - (b) lands in a follow-on commit (matches Phase 87 Plan 01's separation-of-concerns between "content edit" and "external operation") — planner picks.
- **Workspace/tree green at commit boundary** — for Phase 83 this was `cargo build --workspace --locked`; for Phase 88 this is lighter-weight: `grep -c "actions/attest-sbom-action" .github/workflows/*.yml` must return 0 if the rename commit claims to be self-contained, OR `.github/workflows/ci.yml` must be updated in the same commit. Planner decides atomic boundary.

**Acceptance-criteria shape** (Phase 83 Plan 01 carry-forward, adapted):

```
test -d actions/sealedge-attest-sbom-action  # exists
test ! -d actions/attest-sbom-action          # does not exist
git log --follow --oneline actions/sealedge-attest-sbom-action/action.yml | wc -l  # >= 2 (shows pre-rename history via --follow)
grep -rn 'actions/attest-sbom-action' .github/ 2>/dev/null  # returns 0 OR the rename commit is grouped with the ci.yml uses-swap commit
```

---

### Group C — `.github/workflows/ci.yml` self-attest job (D-12 extension + D-14 dogfood conversion)

**Analog:** **The existing job IS the pattern.** Self-analog extension — the new `seal` + `seal.sha256` upload steps sit alongside the existing `Upload attestation assets` step, and the `uses:` swap replaces the inline `Build seal binary` → `Generate CycloneDX SBOM` → `Generate ephemeral Ed25519 keypair` → `Create SBOM attestation` step chain with a single `uses: TrustEdge-Labs/sealedge-attest-sbom-action@v2` step.

**Current state excerpt** (`.github/workflows/ci.yml:171-222`, the complete self-attest job):

```yaml
  # ── Self-attestation (release tags only) ─────────────────────
  self-attestation:
    runs-on: ubuntu-latest
    if: startsWith(github.ref, 'refs/tags/v')
    needs: build-and-test
    continue-on-error: true
    permissions:
      contents: write
    steps:
      - uses: actions/checkout@34e114876b0b11c390a56381ad16ebd13914f8d5 # v4
      - uses: dtolnay/rust-toolchain@631a55b12751854ce901bb631d5902ceb48146f7 # stable
      - uses: Swatinem/rust-cache@e18b497796c12c097a38f9edb9d0641fb99eee32 # v2

      - name: Install system dependencies
        run: sudo apt-get update && sudo apt-get install -y libpcsclite-dev pkg-config pkgconf

      - name: Build seal binary
        run: cargo build -p sealedge-seal-cli --release --locked

      - name: Generate CycloneDX SBOM
        uses: anchore/sbom-action@e22c389904149dbc22b58101806040fa8d37a610 # v0.24.0
        with:
          path: ./target/release/seal
          format: cyclonedx-json
          output-file: seal-sbom.cdx.json
          upload-artifact: false

      - name: Generate ephemeral Ed25519 keypair
        run: |
          ./target/release/seal keygen \
            --out-key ephemeral.key \
            --out-pub build.pub \
            --unencrypted

      - name: Create SBOM attestation
        run: |
          ./target/release/seal attest-sbom \
            --binary ./target/release/seal \
            --sbom seal-sbom.cdx.json \
            --device-key ephemeral.key \
            --device-pub build.pub \
            --out seal.te-attestation.json \
            --unencrypted

      - name: Upload attestation assets
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        run: |
          gh release upload "${{ github.ref_name }}" \
            seal.te-attestation.json \
            build.pub \
            --clobber
```

**D-12 extension pattern (add seal + seal.sha256 upload) — copy the existing `Upload attestation assets` step shape:**

Per CONTEXT.md §Specifics (the exact append the planner pastes or adapts):

```yaml
      - name: Compute seal binary SHA256 checksum
        run: |
          sha256sum ./target/release/seal | awk '{print $1 "  seal"}' > seal.sha256

      - name: Upload seal binary + checksum to release
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        run: |
          gh release upload "${{ github.ref_name }}" \
            ./target/release/seal \
            seal.sha256 \
            --clobber
```

Placement: before or after the existing `Upload attestation assets` step (order doesn't matter; both use `--clobber`). Planner decides. Recommend **before** so the binary exists in the release by the time the attestation-upload step runs (attestation references the binary by hash; no ordering dep in CI, but human-intuitive to have the binary uploaded first).

**D-14 dogfood-conversion pattern — replace 4 inline steps with 1 `uses:` step.** Post-conversion shape (copy Phase 86 `anchore/sbom-action@e22c389...` SHA-pinned + major-tag-commented reference style):

```yaml
      - name: Generate CycloneDX SBOM
        uses: anchore/sbom-action@e22c389904149dbc22b58101806040fa8d37a610 # v0.24.0
        with:
          path: ./target/release/seal
          format: cyclonedx-json
          output-file: seal-sbom.cdx.json
          upload-artifact: false

      - name: Attest SBOM (dogfood sealedge-attest-sbom-action)
        uses: TrustEdge-Labs/sealedge-attest-sbom-action@v2  # TODO: pin to commit SHA after first @v2.0.0 tag
        with:
          binary: ./target/release/seal
          sbom: seal-sbom.cdx.json
          seal-version: 'latest'  # OR pin to ${{ github.ref_name }} to self-host
```

**Subtle dependency caveat (D-11 timing):** The dogfood step cannot fire green on the first release tag until `@v2` is cut on the renamed action repo AND `seal` + `seal.sha256` exist in the release. Per CONTEXT.md §D-11 this is a chicken-and-egg Phase 88 → Phase 89 handoff: Phase 88 ships the code + cuts `@v2`; Phase 89's v6.0.0 release-tag push is the first run that proves the end-to-end chain. Plan `continue-on-error: true` stays (it already is — line 176) — this covers the bootstrap window.

**Stale `.te-attestation.json` extension** (CONTEXT.md §"Out of scope" + §Deferred last bullet): `seal.te-attestation.json` on lines 212 and 220 is a pre-existing Phase 84 cleanup gap. Two clean options:
- Option A: Fix to `seal.se-attestation.json` incidentally during the D-14 conversion, since the `Create SBOM attestation` inline step is being deleted entirely — the sustained path is whatever `sealedge-attest-sbom-action@v2`'s `attestation-path` output writes, which uses `.se-attestation.json` per action.yml line 30.
- Option B: Leave for Phase 89's VALID-03 sweep.

Planner picks. Recommendation: **Option A** (the step disappears in the D-14 refactor, so the stale extension disappears by virtue of the refactor — no incidental expansion of scope).

**Acceptance-criteria shape:**

```
yq '.jobs.self-attestation.steps | length' .github/workflows/ci.yml
# post-change: fewer steps than pre-change (4 inline steps collapsed into 1 `uses:` step), +1-2 new upload steps

grep -c 'sealedge-attest-sbom-action@v2' .github/workflows/ci.yml  # returns >= 1
grep -c 'seal.te-attestation.json' .github/workflows/ci.yml        # returns 0 (Option A)
grep -c 'seal.sha256' .github/workflows/ci.yml                      # returns >= 2 (compute + upload)

# Workflow YAML parses cleanly (Phase 87 Plan 01 parallel: edit with targeted Edit, don't reflow YAML):
yq '.jobs.self-attestation' .github/workflows/ci.yml > /dev/null
```

---

### Group D — `.planning/REQUIREMENTS.md` §Ext (EXT-02 / EXT-03 amendment)

**Analog:** Phase 87 Plan 01 atomic commit pattern + Phase 86 hybrid-treatment rewording pattern (D-03 MIGRATION.md v6.0 section).

**Current state excerpt** (`.planning/REQUIREMENTS.md:39-41`):

```markdown
- [ ] **EXT-02**: New GitHub Action repo created under sealedge naming and published to GitHub Marketplace (equivalent functionality to current `attest-sbom-action`); SHA256 checksum verification preserved
- [ ] **EXT-03**: Old `TrustEdge-Labs/attest-sbom-action` marketplace listing marked deprecated with README redirect to the new listing
```

**Expected transformation** (CONTEXT.md §Specifics guidance — verbatim planner-drafted wording):

```markdown
- [ ] **EXT-02**: The `TrustEdge-Labs/attest-sbom-action` repo is renamed to `TrustEdge-Labs/sealedge-attest-sbom-action` via `gh repo rename`; action source references sealedge/seal; a new `@v2` tag ships the rebranded action; SHA256 checksum verification of the downloaded binary is preserved
- [ ] **EXT-03**: GitHub's built-in 301 redirect covers existing `uses: TrustEdge-Labs/attest-sbom-action@v1` references; the pre-rebrand `@v1` tag stays frozen; the post-rename README carries a short notice pointing readers to `@v2` and the renamed repo
```

**Commit pattern** (Phase 87 Plan 01 carry-forward — atomic, scope-disciplined, HEREDOC message):

```
git add .planning/REQUIREMENTS.md
git commit -m "$(cat <<'EOF'
docs(88): amend EXT-02/EXT-03 to match rename-in-place approach

Per 88-CONTEXT.md D-02: original wording assumed a two-repo migration
(new repo + old listing marked deprecated). The actual execution plan is
a rename-in-place backed by GitHub's 301 redirect, matching Phase 87's
pattern. Rewording aligns the requirement with the delivery shape so
auditors don't find a mismatch.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

**Acceptance-criteria shape:**

```
grep -c 'sealedge-attest-sbom-action' .planning/REQUIREMENTS.md  # returns >= 2 (EXT-02 + EXT-03)
grep -c 'marketplace listing marked deprecated' .planning/REQUIREMENTS.md  # returns 0 (old wording removed)
git diff --stat .planning/REQUIREMENTS.md  # expected 1 file changed, ~2 insertions / ~2 deletions
```

---

### Group E — Cross-repo website: `TrstVerifier.tsx` → `SealVerifier.tsx` rename + import update

**Analog (exact match):** Phase 83 Plan 01 binary-source-file `git mv` + inline identifier sweep pattern. In Phase 83 the file was `crates/core/src/bin/trustedge-server.rs` → `sealedge-server.rs`, with `Cargo.toml` `[[bin]]` `name` + `path` fields updated in the same commit. In Phase 88 the file is `.tsx` and the importer is another `.tsx`, but the shape is identical.

**Current state excerpt** (`trustedgelabs-website/src/components/TrstVerifier.tsx:11, 271`):

```tsx
const TrstVerifier: React.FC = () => {
...
export default TrstVerifier;
```

**Importer** (`trustedgelabs-website/src/components/Solution.tsx:11-12` and `:92-93`):

```tsx
import WasmDemo from './WasmDemo';
import TrstVerifier from './TrstVerifier';
...
          <WasmDemo />
          <TrstVerifier />
```

**Prose reference** (`trustedgelabs-website/src/components/IntegrationGuide.tsx:199`):

```tsx
<span>Explore .trst archive verification with the TrstVerifier component</span>
```

**Expected transformation** (single atomic commit):

1. **`git mv` the file:**
   ```
   cd /home/john/vault/projects/github.com/trustedgelabs-website
   git mv src/components/TrstVerifier.tsx src/components/SealVerifier.tsx
   ```

2. **Rename the symbol inside the moved file:**
   - Line 11: `const TrstVerifier: React.FC = () => {` → `const SealVerifier: React.FC = () => {`
   - Line 271: `export default TrstVerifier;` → `export default SealVerifier;`

3. **Update inbound importers** (Phase 83 parallel — also update all `use trustedge_*` consumers in the same commit):
   - `Solution.tsx:12` import rename: `import TrstVerifier from './TrstVerifier';` → `import SealVerifier from './SealVerifier';`
   - `Solution.tsx:93` JSX rename: `<TrstVerifier />` → `<SealVerifier />`
   - `IntegrationGuide.tsx:199` prose rename: `the TrstVerifier component` → `the SealVerifier component` (and `.trst archive verification` → `.seal archive verification` per D-15 / Phase 83 D-02 carry-forward)

**Note on App.tsx:** `App.tsx` does NOT directly import `TrstVerifier`; `Solution.tsx` does. CONTEXT.md §D-15 calls out "import updates in `src/App.tsx`", but grep confirms the actual importer is `Solution.tsx`. Planner should update `Solution.tsx`, not `App.tsx`. Flag in plan as a CONTEXT.md correction.

**Acceptance-criteria shape:**

```
# In the website repo:
test -f src/components/SealVerifier.tsx
test ! -f src/components/TrstVerifier.tsx
grep -c 'TrstVerifier' src/  # returns 0 (after sweep)
grep -c 'SealVerifier' src/  # returns >= 3 (component decl, export, import, JSX usage)
git log --follow --oneline src/components/SealVerifier.tsx | wc -l  # >= 2 (follows pre-rename history)
```

---

### Group F — Cross-repo website: text/branding sweep (~19 TSX + 3 metadata)

**Analog:** Phase 85 D-11–D-14 casing rules + Phase 86 D-13 allowlisted-grep discipline.

**Cross-repo grep audit command** (CONTEXT.md §Specifics verbatim — reuse without modification):

```
git -C /home/john/vault/projects/github.com/trustedgelabs-website ls-files \
  | xargs grep -n "TrustEdge\|trustedge\|TRUSTEDGE" 2>/dev/null \
  | grep -vE "TrustEdge Labs|TrustEdge-Labs|trustedgelabs\.com|node_modules|package-lock"
```

**Expected:** zero results after sweep.

**Casing rules (Phase 85 D-11–D-14 verbatim carry-forward):**

| Surface | Rule | Phase-85 source |
|---------|------|-----------------|
| JSX prose / button labels / marketing copy | `Sealedge` (Title case) when referring to the product | D-11 Dashboard UI rule |
| React component exported symbols | PascalCase following actual rename (`SealVerifier`, not `TrstVerifier`) | Phase 83 D-01 identifier rename carry-forward |
| TypeScript types & interfaces | `SealWasm`, `SealVerifyResult` (mirror Rust-side renames) | Phase 83 D-01 |
| JSX text `Hello, TrustEdge!` (WasmDemo.tsx line 15) | `Hello, Sealedge!` (sentence-start brand → Title case, D-14 edge case) | Phase 85 D-14 |
| Copyright lines `Copyright 2025 TrustEdge Labs LLC` | **stays** (Phase 85 D-02 / D-03 legal-entity carry-forward) | Phase 85 D-03 |
| `TrustEdge-Labs` org (github URLs) | **stays** (Phase 85 D-02 / Phase 86 carve-out) | Phase 85 D-02 |
| `.trst` archive extension (prose mentions) | `.seal` (Phase 83 D-02 carry-forward) | Phase 83 D-02 |

**Excerpt — `WasmDemo.tsx` (line 9-15)** shows the typical pattern the sweep handles:

```tsx
import { useWasm } from '../wasm/useWasm';
import { loadTrustedgeWasm } from '../wasm/loader';
import type { TrustEdgeWasm, EncryptedData } from '../wasm/types';

const WasmDemo: React.FC = () => {
  const { module: wasmModule, loading: isLoading, error: wasmError } = useWasm(loadTrustedgeWasm);

  const [inputText, setInputText] = useState('Hello, TrustEdge! This is a secret message.');
```

**D-16 explicit carve-out reminder:** `loadTrustedgeWasm`, `TrustEdgeWasm`, `../wasm/loader`, `../wasm/types`, `../wasm/useWasm` come from `src/wasm/trustedge-wasm/` + `src/wasm/trustedge-trst-wasm/` directories. Per CONTEXT.md D-16, the **WASM package-import swap is deferred** — planner does **NOT** rename `loadTrustedgeWasm` → `loadSealedgeWasm` or the underlying `src/wasm/trustedge-wasm/` directory or `src/wasm/trustedge-trst-wasm/` directory in Phase 88. Only the **visible text** `'Hello, TrustEdge!'` → `'Hello, Sealedge!'` updates. Separate the concerns cleanly.

The grep audit therefore needs an **additional allowlist** for `src/wasm/` to avoid hitting the deferred directory:

```
git -C /home/john/vault/projects/github.com/trustedgelabs-website ls-files \
  | xargs grep -n "TrustEdge\|trustedge\|TRUSTEDGE" 2>/dev/null \
  | grep -vE "TrustEdge Labs|TrustEdge-Labs|trustedgelabs\.com|node_modules|package-lock|src/wasm/trustedge-|useWasm|loader\.ts|types\.ts"
```

(Add the deferred-WASM patterns to the allowlist. Planner tunes final regex during plan-02/03 authoring.)

**Excerpt — `Hero.tsx` (line 2-7)** shows the copyright-header anomaly on this repo (duplicate block):

```tsx
/*
 * Copyright 2025 John Turner
 */

/*
 * Copyright 2025 TrustEdge Labs LLC
 */
```

Two copyright blocks coexist — the `TrustEdge Labs LLC` legal-entity block stays per D-15 (company brand); the `John Turner` block also stays (personal attribution). No action required; allowlist hits it as `TrustEdge Labs`.

**Metadata sweep — `index.html`, `README.md`, `package.json`:**

Follow Phase 86 D-06 (other markdown surfaces) + Phase 85 D-07/D-08 (Cargo.toml description + URLs) — on the website this translates to:
- `index.html` `<title>` tag, `<meta name="description">`, any hardcoded product name
- `README.md` — README prose sweep (same shape as Phase 86 crate READMEs)
- `package.json` — `description`, `keywords`, `name` (likely `trustedgelabs-website` stays because the repo name stays per CONTEXT.md §"Out of scope"; only internal product references change)

**Acceptance-criteria shape (cross-repo grep audit — CONTEXT.md §Specifics verbatim):**

```
# Run in the website repo (cross-repo scope):
git -C /home/john/vault/projects/github.com/trustedgelabs-website ls-files \
  | xargs grep -n "TrustEdge\|trustedge\|TRUSTEDGE" 2>/dev/null \
  | grep -vE "TrustEdge Labs|TrustEdge-Labs|trustedgelabs\.com|node_modules|package-lock|src/wasm/trustedge-|useWasm\.ts|loader\.ts|types\.ts|dist/"
# Expected: zero lines (D-18 / CONTEXT.md §Specifics).

# Plus a live-preview screenshot OR `npm run dev` + curl home page for D-18 verification.
```

---

## Shared Patterns

These cross-cutting patterns apply across all Phase 88 plans and match the established discipline from Phases 83–87.

### Shared.1 — Rename-in-place with GitHub 301 redirect (Phase 87 carry-forward)

**Source:** `.planning/phases/87-github-repository-rename/87-02-PLAN.md` Tasks 2–4 + `.planning/phases/87-github-repository-rename/87-VERIFICATION.md` §D-13 §1.

**Apply to:** Plans executing `gh repo rename attest-sbom-action` (the external operation per D-01).

**Exact commands (verbatim copy-paste — same shape as Phase 87):**

```bash
# Pre-rename gate (D-06 carry-forward adapted for the action repo):
gh pr list -R TrustEdge-Labs/attest-sbom-action --state open  # expect: empty
gh auth status  # expect: repo scope active

# The rename (user runs from their shell — not Claude's Bash tool, per Phase 87 D-05 pattern):
gh repo rename sealedge-attest-sbom-action -R TrustEdge-Labs/attest-sbom-action

# Expected output:
# ✓ Renamed repository TrustEdge-Labs/sealedge-attest-sbom-action

# Verification (D-09 / Phase 87 D-13 §1 parallel):
curl -I -s -o /dev/null -w '%{http_code} -> %{redirect_url}\n' \
  https://github.com/TrustEdge-Labs/attest-sbom-action
# Expected: 301 -> https://github.com/TrustEdge-Labs/sealedge-attest-sbom-action

# Rollback (D-15 verbatim carry-forward — keep in PLAN.md for fast copy-paste):
gh repo rename attest-sbom-action -R TrustEdge-Labs/sealedge-attest-sbom-action
```

### Shared.2 — Atomic commit discipline (Phase 83–87 carry-forward)

**Source:** Phase 87 Plan 01 Task 3 commit-message HEREDOC + Phase 83 Plan 01 single-atomic-commit-per-logical-step pattern.

**Apply to:** Every Phase 88 commit (in both repos).

**Pattern:**

```bash
# Stage specifically — never `git add -A` (Phase 87 D-03 discipline):
git add <specific-files>

# Confirm the diff is exactly what you expect:
git diff --cached --stat

# Commit with HEREDOC message + Co-Authored-By trailer:
git commit -m "$(cat <<'EOF'
<type>(88): <one-line subject under 72 chars>

<body paragraph explaining what + why; 2-4 sentences max>

Per 88-CONTEXT.md D-XX / EXT-YY.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

**Commit-message type prefixes** (Phase 83/85/86/87 carry-forward):
- `fix(88)`: bug-style fixes to existing content (action.yml binary-name swaps fit here — fixes stale strings)
- `docs(88)`: documentation or requirement-text changes (REQUIREMENTS.md amendment, README.md updates)
- `chore(88)`: mechanical moves (folder rename, scope changes)
- `feat(88)`: net-new capabilities (none expected in Phase 88)

### Shared.3 — Grep-allowlist audit (Phase 85 / 86 / 87 carry-forward)

**Source:** Phase 86 D-13 `.planning/phases/86-documentation-sweep/86-CONTEXT.md` + Phase 87 D-13 §3 CONTEXT.md `Specifics` block.

**Apply to:** Both the monorepo Phase 88 scope (Group A+B+C+D) AND the cross-repo trustedgelabs-website scope (Group E+F).

**Monorepo audit** (post-Phase 88 surface — new allowlist entries):

```bash
# Replaces Phase 87's allowlist with Phase 88 extensions:
git ls-files | xargs grep -n "TrustEdge-Labs/trustedge" 2>/dev/null \
  | grep -vE '^\.planning/|^\.factory/|improvement-plan\.md|RFC_K256_SUPPORT\.md|security-review-platform\.md|CHANGELOG\.md|MIGRATION\.md'
# Note: actions/attest-sbom-action/ is NO LONGER in the allowlist (Phase 88 swept it).
# Expected: zero hits.
```

**Cross-repo website audit** (CONTEXT.md §Specifics verbatim):

```bash
git -C /home/john/vault/projects/github.com/trustedgelabs-website ls-files \
  | xargs grep -n "TrustEdge\|trustedge\|TRUSTEDGE" 2>/dev/null \
  | grep -vE "TrustEdge Labs|TrustEdge-Labs|trustedgelabs\.com|node_modules|package-lock"
# Expected: zero hits (D-18).
# Planner tunes the allowlist to include src/wasm/trustedge-* deferred per D-16.
```

### Shared.4 — Verification-file evidence capture (Phase 87 D-16 pattern)

**Source:** `.planning/phases/87-github-repository-rename/87-VERIFICATION.md` — the template Phase 88 copies.

**Apply to:** `.planning/phases/88-external-action-product-website/88-VERIFICATION.md` (the phase-close artifact — CONTEXT.md §"In scope" last bullet).

**Sections to include (Phase 87 VERIFICATION.md lines 1-218 shape):**

1. **Pre-rename gate evidence** (adapted D-06 for the action repo): open-PRs count, `gh auth status` output
2. **D-05-parallel rename operation record:** `gh repo rename sealedge-attest-sbom-action -R TrustEdge-Labs/attest-sbom-action` output + timestamp
3. **D-09 curl redirect check** (verbatim from CONTEXT.md §Specifics):
   ```
   curl -I -s -o /dev/null -w '%{http_code} -> %{redirect_url}\n' https://github.com/TrustEdge-Labs/attest-sbom-action
   ```
   Expected: `301 -> https://github.com/TrustEdge-Labs/sealedge-attest-sbom-action`
4. **D-10 Marketplace listing check:** screenshot of listing title/description after rename (user-driven check per CONTEXT.md, since Marketplace UI is browser-only)
5. **D-18 grep audit output** (both monorepo + cross-repo commands, both returning zero lines)
6. **D-18 live preview screenshot** of trustedgelabs-website post-update (home page + key components)
7. **Rollback status** — verbatim rollback command documented + whether it was executed
8. **ROADMAP success-criteria table** — EXT-02/EXT-03/EXT-04 status

### Shared.5 — "External GitHub operation" vs "in-repo commit" separation (Phase 87 carry-forward)

**Source:** Phase 87 CONTEXT.md §D-05 ("User runs the rename command from their shell") + §D-07 ordering.

**Apply to:**
- The `gh repo rename` call on the action repo (Phase 88 D-01)
- The `@v2.0.0` + floating `@v2` tag cut on the renamed action repo (Phase 88 D-04, D-11)
- Marketplace listing re-publish (Phase 88 D-10) if auto-refresh didn't fire

**Discipline:**
- These are **not commits** — they are out-of-band operations on GitHub's surface.
- PLAN.md task type is `checkpoint:human-action gate="blocking"` (Phase 87 Plan 02 Task 2 shape).
- User runs from their shell; Claude provides the verbatim command string and the resume-signal protocol.
- Evidence captured in `88-VERIFICATION.md` (Shared.4 above) via paste-the-output workflow.

---

## No Analog Found

None. **Every file in Phase 88's scope has a clear analog from Phases 83, 85, 86, or 87.** The phase is entirely a carry-forward/mechanical/external-op phase; no new construction patterns are introduced.

The one surface that is **slightly novel** — cross-repo sweeps under `/home/john/vault/projects/github.com/trustedgelabs-website/` — is not pattern-novel; it just executes the Phase 85/86 grep-allowlist sweep discipline on a filesystem tree that sits outside the sealedge monorepo. The commit mechanics (`git add` specific files, atomic commit per logical step, HEREDOC messages) are identical; only the working directory changes.

---

## Metadata

**Analog search scope:**
- `.planning/phases/83-crate-and-binary-rename/` (folder-rename + identifier-sweep pattern)
- `.planning/phases/85-code-sweep-headers-text-metadata/` (casing rules, allowlisted grep)
- `.planning/phases/86-documentation-sweep/` (README hybrid-treatment, D-13 grep audit, `actions/attest-sbom-action/**` carve-out that Phase 88 now owns)
- `.planning/phases/87-github-repository-rename/` (gh repo rename mechanics, D-13 curl redirect, D-15 rollback, VERIFICATION.md evidence format)
- `actions/attest-sbom-action/` (the files Phase 88 directly edits — self-analog for Group A)
- `.github/workflows/ci.yml` lines 171-222 (self-analog for Group C)
- `/home/john/vault/projects/github.com/trustedgelabs-website/` (cross-repo targets — self-analog for Groups E+F)

**Files scanned:** ~20 planning + ~8 source files across 2 repos

**Pattern extraction date:** 2026-04-21

**Key insight for planner:** Phase 88 is **not a build phase** — it is a **sweep + rename + external-op phase**, identical in shape to Phase 87 but with a cross-repo dimension added. Every single technique the executor needs has already been demonstrated successfully in an earlier v6.0 phase. Plan authors should explicitly cite the prior phase's decision IDs (D-01 through D-16 on the upstream phases) when authoring `<action>` blocks — it compresses the planner-to-executor handoff and matches the "concrete, not abstract" quality bar.
