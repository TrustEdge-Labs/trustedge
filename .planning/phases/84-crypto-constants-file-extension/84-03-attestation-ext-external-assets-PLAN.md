---
phase: 84-crypto-constants-file-extension
plan: 03
type: execute
wave: 2
depends_on: ["84-01"]
files_modified:
  - web/verify/index.html
  - actions/attest-sbom-action/action.yml
  - actions/attest-sbom-action/README.md
  - deploy/digitalocean/README-deploy.md
autonomous: true
requirements: [REBRAND-04b]

must_haves:
  truths:
    - "web/verify/index.html UI labels read `.se-attestation.json` (no `.te-attestation.json` literal remains in the HTML)"
    - "The verify HTML file-input `accept` attribute (if present in the DOM) filters only on sealedge-consistent values — it must NOT accept legacy `.te-attestation.json` (per CONTEXT D-04 no dual-accept)"
    - "actions/attest-sbom-action/action.yml input description (line 30) and OUT_PATH bash template (line 89) both reference `.se-attestation.json`"
    - "actions/attest-sbom-action/README.md references `.se-attestation.json` throughout; the brand-word `TrustEdge` in surrounding prose is PRESERVED for Phase 86"
    - "deploy/digitalocean/README-deploy.md references `.se-attestation.json`; brand-word `TrustEdge` in surrounding prose is PRESERVED for Phase 86"
    - "cargo check --workspace --locked green at the commit boundary (the HTML is compiled into the platform-server binary via include_str!, so any malformed HTML would surface here)"
    - "The platform-server binary builds cleanly — the verify HTML is bundled via `include_str!` at compile time, so an HTML change triggers a rebuild"
  artifacts:
    - path: "web/verify/index.html"
      provides: "Browser-side attestation verifier UI labeled for .se-attestation.json"
      contains: '.se-attestation.json'
    - path: "actions/attest-sbom-action/action.yml"
      provides: "GitHub Action emits and documents .se-attestation.json outputs"
      contains: '.se-attestation.json'
    - path: "actions/attest-sbom-action/README.md"
      provides: "Action README documents .se-attestation.json outputs"
      contains: '.se-attestation.json'
    - path: "deploy/digitalocean/README-deploy.md"
      provides: "Deployment doc references .se-attestation.json"
      contains: '.se-attestation.json'
  key_links:
    - from: "action.yml OUT_PATH template"
      to: "the on-disk attestation file produced by the GitHub Action"
      via: '${{ runner.temp }}/${BINARY_NAME}.se-attestation.json'
      pattern: '\.se-attestation\.json'
    - from: "web/verify/index.html label + file-input accept (if present)"
      to: "user-selected attestation file for browser verification"
      via: "DOM label text + HTML file input"
      pattern: '\.se-attestation\.json'
---

<objective>
Sweep the `.te-attestation.json` attestation-file extension to `.se-attestation.json` across the in-repo external-facing assets: the deployed HTML verifier page, the GitHub Action source-of-truth (`actions/attest-sbom-action/action.yml` + `README.md`), and the deployment README for DigitalOcean. Clean rename — no dual-accept (per CONTEXT D-04).

Purpose: Completes the remainder of REBRAND-04b alongside Plan 84-02 (which handled Rust sources + the demo script). This plan's file set is fully disjoint from Plan 84-02's — they run in parallel in Wave 2.

Scope boundary (per CONTEXT `<domain>` and `<decisions>` D-03): Phase 84 updates the monorepo source-of-truth files ONLY. Phase 88 handles re-publishing `actions/attest-sbom-action/` to the GitHub Marketplace under a new sealedge listing and deprecating the old one; that is NOT this plan's concern. The brand word "TrustEdge" in prose surrounding the extension literal STAYS — it is Phase 86 scope.

Output: 4 files edited. Single atomic commit. Workspace build remains green (the HTML is `include_str!`-bundled into the platform-server binary, so `cargo check --workspace --locked` validates the HTML is present and parseable as a string).
</objective>

<execution_context>
@$HOME/.claude/get-shit-done/workflows/execute-plan.md
@$HOME/.claude/get-shit-done/templates/summary.md
</execution_context>

<context>
@.planning/PROJECT.md
@.planning/ROADMAP.md
@.planning/STATE.md
@.planning/REQUIREMENTS.md
@.planning/phases/84-crypto-constants-file-extension/84-CONTEXT.md
@.planning/phases/84-crypto-constants-file-extension/84-01-SUMMARY.md
@CLAUDE.md
</context>

<interfaces>
<!-- Concrete edit sites. Extracted from codebase 2026-04-18. -->

web/verify/index.html (2 enumerated sites + any file-input accept attribute):
```
line 123:  <p class="subtitle">Verify the authenticity of a <code>.te-attestation.json</code> file.</p>
             → <code>.se-attestation.json</code>

line 126:  <label for="file-input">Select .te-attestation.json</label>
             → <label for="file-input">Select .se-attestation.json</label>

line 127:  <input type="file" id="file-input" accept=".json,application/json" />
             → UNCHANGED — the accept attribute filters on MIME type / generic .json,
               not on .te-attestation.json. Verify this is still the case after editing;
               if the accept attribute is changed in a later revision to include
               ".te-attestation.json", it MUST be migrated to ".se-attestation.json"
               (no dual-accept per D-04). Current line 127 does NOT reference the
               legacy extension, so no edit is required here — but the executor MUST
               grep the full file to catch any additional occurrences beyond lines
               123 and 126.
```

**Header context on web/verify/index.html line 122:** `<h1>TrustEdge Attestation Verifier</h1>` — this `TrustEdge` brand word STAYS (Phase 86 prose sweep).

actions/attest-sbom-action/action.yml (2 sites):
```
line 30:  description: 'Path to generated .te-attestation.json file'
            → description: 'Path to generated .se-attestation.json file'

line 89:  OUT_PATH="${{ runner.temp }}/${BINARY_NAME}.te-attestation.json"
            → OUT_PATH="${{ runner.temp }}/${BINARY_NAME}.se-attestation.json"
```

actions/attest-sbom-action/README.md (5 sites — enumerated from grep):
```
line 71:  The action writes a `.te-attestation.json` file to `$RUNNER_TEMP` and exposes its path
            → `.se-attestation.json`

line 89:  | `attestation-path` | Absolute path to the generated `.te-attestation.json` file |
            → `.se-attestation.json`

line 99:  4. Writes the attestation to `$RUNNER_TEMP/<binary-name>.te-attestation.json`.
            → `.se-attestation.json`

line 106: trst verify-attestation my-app.te-attestation.json \
            → my-app.se-attestation.json

line 117: -d @my-app.te-attestation.json
            → @my-app.se-attestation.json
```

NOTE: the README example on line 106 uses the `trst` binary name — that binary has already been renamed to `seal` in Phase 83 per STATE.md. Check whether the README still says `trst verify-attestation` (Phase 83 scope was `seal` binaries; if this README was not swept in Phase 83 the executor should note this as a follow-up for Phase 85/86 brand-word prose sweep — do NOT rename `trst` to `seal` in this plan; that is explicitly out of Phase 84 scope per CONTEXT `<domain>` "Out of scope" bullet 1 and 2). The ONLY change here is `.te-` → `.se-` in the extension token. If the binary name is still `trst` in this README, leave it — Phase 85/86 handles binary-name prose in docs.

deploy/digitalocean/README-deploy.md (1 site):
```
line 30:  -d @path/to/attestation.te-attestation.json
            → -d @path/to/attestation.se-attestation.json
```

Brand-word "TrustEdge" prose elsewhere in this README STAYS — Phase 86 scope.
</interfaces>

<tasks>

<task type="auto">
  <name>Task 1: Rename `.te-attestation.json` → `.se-attestation.json` across the 4 external-facing asset files; commit atomically</name>
  <read_first>
    - .planning/phases/84-crypto-constants-file-extension/84-CONTEXT.md (§Decisions D-03 in-repo external assets, D-04 clean rename; §Domain out-of-scope list — especially Phase 85/86 brand-word carve-outs)
    - .planning/phases/84-crypto-constants-file-extension/84-01-SUMMARY.md (confirm Plan 01 landed — this plan does not depend on its code outputs semantically, but the wave ordering ensures the workspace is coherent)
    - web/verify/index.html (entire file — small HTML, need to confirm only lines 123, 126 contain the legacy extension and no file-input `accept` attribute references it)
    - actions/attest-sbom-action/action.yml (entire file, or at minimum lines 28-92 covering inputs section and the Create SBOM attestation step)
    - actions/attest-sbom-action/README.md (entire file — grep shows 5 legacy-extension hits at lines 71, 89, 99, 106, 117; surrounding prose may mention `TrustEdge` and `trst`, both of which MUST be preserved)
    - deploy/digitalocean/README-deploy.md (at minimum lines 20-40 covering the curl example on line 30)
    - CLAUDE.md § "Code Standards" (no emoji — applies to Markdown/HTML prose too)
  </read_first>
  <files>
    web/verify/index.html,
    actions/attest-sbom-action/action.yml,
    actions/attest-sbom-action/README.md,
    deploy/digitalocean/README-deploy.md
  </files>
  <action>
**Step 1 — web/verify/index.html.**

Enumerate legacy hits before editing:
```bash
grep -n 'te-attestation' web/verify/index.html
```
Expected output: two lines (123 and 126 per `<interfaces>`). If the count differs, re-read the file and extend the edit set accordingly — the rule is zero `.te-attestation.json` literals remain in the final HTML.

Apply the 2 edits (lines 123, 126) per `<interfaces>`. Leave line 122 (`<h1>TrustEdge Attestation Verifier</h1>`) untouched — brand-word prose is Phase 86. Leave line 127 (`<input type="file" id="file-input" accept=".json,application/json" />`) untouched — the `accept` attribute does not reference the legacy extension.

**Step 2 — actions/attest-sbom-action/action.yml.**

Enumerate legacy hits before editing:
```bash
grep -n 'te-attestation' actions/attest-sbom-action/action.yml
```
Expected output: two lines (30 and 89 per `<interfaces>`). Apply the 2 edits.

The OUT_PATH template on line 89 is bash inside a `shell: bash` block — `${BINARY_NAME}.se-attestation.json` is a valid bash string suffix (no special chars). No quoting changes needed.

**Step 3 — actions/attest-sbom-action/README.md.**

Enumerate legacy hits before editing:
```bash
grep -n 'te-attestation' actions/attest-sbom-action/README.md
```
Expected output: 5 lines (71, 89, 99, 106, 117 per `<interfaces>`). Apply all 5 edits.

Leave ALL other content untouched — specifically:
- Brand-word "TrustEdge" in prose STAYS (Phase 86).
- Binary name `trst` (if still present in the README) STAYS (Phase 85/86 brand-word sweep; Phase 83 may or may not have swept this doc — not in Phase 84 scope either way).

**Step 4 — deploy/digitalocean/README-deploy.md.**

Enumerate legacy hits before editing:
```bash
grep -n 'te-attestation' deploy/digitalocean/README-deploy.md
```
Expected output: 1 line (line 30). Apply the edit.

**Step 5 — Validate.**

```bash
# Core validation: the HTML is bundled into the platform-server binary via include_str!,
# so a cargo check catches any HTML-file-missing / malformed scenario.
cargo check --workspace --locked
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --locked
```

Action YAML / Markdown files are not parsed by `cargo`, so their syntactic validity relies on:
- `action.yml`: optionally, `actionlint actions/attest-sbom-action/action.yml` if the tool is available in the environment (non-blocking — if the tool is missing, skip).
- Markdown: no linter required; grep confirmation is sufficient.

**Step 6 — Commit atomically.**

```bash
git add web/verify/index.html \
        actions/attest-sbom-action/action.yml \
        actions/attest-sbom-action/README.md \
        deploy/digitalocean/README-deploy.md

git commit -m "$(cat <<'EOF'
refactor(84-03): rename .te-attestation.json → .se-attestation.json in external assets

Sweeps the attestation-file extension across the monorepo external-facing
assets — the deployed HTML verifier UI, the GitHub Action source-of-truth,
and the DigitalOcean deployment README. Clean rename — no dual-accept
(per CONTEXT.md D-04).

  - web/verify/index.html (2 sites, lines 123, 126):
      UI subtitle + file-input label prompt. The file-input `accept`
      attribute (line 127) does not reference the legacy extension; left
      unchanged. The `<h1>TrustEdge Attestation Verifier</h1>` brand-word
      prose on line 122 is Phase 86 scope and left unchanged.

  - actions/attest-sbom-action/action.yml (2 sites, lines 30, 89):
      output description + OUT_PATH bash template that produces the
      attestation file on the Action runner.

  - actions/attest-sbom-action/README.md (5 sites, lines 71, 89, 99, 106, 117):
      action documentation prose. Brand-word "TrustEdge" and binary-name
      mentions (e.g. "trst") left unchanged — those belong to the Phase
      85/86 prose sweeps.

  - deploy/digitalocean/README-deploy.md (1 site, line 30):
      curl example path. Surrounding prose left unchanged.

Scope boundaries (per CONTEXT.md §Decisions D-03 and §Domain):
  - Phase 84 updates the monorepo source-of-truth files; Phase 88 handles
    the external GitHub Marketplace re-publication of the renamed action
    and the trustedgelabs.com product-page updates.
  - Brand-word prose ("TrustEdge" in UI titles, README narrative, deployment
    docs) deliberately preserved — Phase 85/86 prose sweeps.
  - The web/verify/index.html file is bundled into the platform-server
    binary via include_str! at compile time; rebuilding the platform server
    picks up the new bytes. Deployment (pushing the rebuilt binary) is not
    this plan's concern.

Runs in parallel with Plan 84-02 (Rust + demo-script sweep) — disjoint
files_modified ensure no merge conflicts.

Validation:
  - cargo check --workspace --locked green (include_str! HTML bundling
    verified at compile time).
  - cargo fmt --check green.
  - cargo clippy --workspace --all-targets -- -D warnings green.
  - cargo test --workspace --locked green.
  - Zero .te-attestation.json literals remain in the 4 edited files.

Requirements: REBRAND-04b (external-assets portion; Rust + demo script
portion is Plan 84-02).

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```
  </action>
  <acceptance_criteria>
    - `grep -c '\.te-attestation\.json' web/verify/index.html` returns `0`.
    - `grep -c '\.se-attestation\.json' web/verify/index.html` returns `2` (lines 123, 126 per `<interfaces>`).
    - `grep -c '>TrustEdge Attestation Verifier<' web/verify/index.html` returns `≥ 1` (the `<h1>` brand-word prose on line 122 is preserved per the Phase 86 carve-out).
    - `grep -c '\.te-attestation\.json' actions/attest-sbom-action/action.yml` returns `0`.
    - `grep -c '\.se-attestation\.json' actions/attest-sbom-action/action.yml` returns `2` (lines 30, 89).
    - `grep -n 'OUT_PATH=.*\.se-attestation\.json' actions/attest-sbom-action/action.yml` returns 1 matching line on or near line 89.
    - `grep -c '\.te-attestation\.json' actions/attest-sbom-action/README.md` returns `0`.
    - `grep -c '\.se-attestation\.json' actions/attest-sbom-action/README.md` returns `5` (lines 71, 89, 99, 106, 117).
    - `grep -c 'TrustEdge' actions/attest-sbom-action/README.md` returns `≥ 1` (brand-word prose preserved).
    - `grep -c '\.te-attestation\.json' deploy/digitalocean/README-deploy.md` returns `0`.
    - `grep -c '\.se-attestation\.json' deploy/digitalocean/README-deploy.md` returns `1` (line 30).
    - `cargo check --workspace --locked` exits `0` (confirms the `include_str!`-bundled HTML still compiles).
    - `cargo fmt --check` exits `0`.
    - `cargo clippy --workspace --all-targets -- -D warnings` exits `0`.
    - `cargo test --workspace --locked` exits `0`.
    - `git log -1 --pretty=%s` returns a string starting with `refactor(84-03):`.
    - `git status --porcelain` is empty after the commit.
  </acceptance_criteria>
  <verify>
    <automated>cargo check --workspace --locked && cargo fmt --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --locked</automated>
  </verify>
  <done>
    - All 4 external-asset files edited; the 10 total `.te-` → `.se-` replacements landed in one atomic commit.
    - Brand-word "TrustEdge" prose preserved in the HTML h1, action README, and deployment README (explicit Phase 85/86 carve-outs).
    - web/verify/index.html `include_str!` integration confirmed via `cargo check --workspace --locked`.
    - Workspace builds + tests green.
    - REBRAND-04b fully closed when combined with Plan 84-02's Rust+script sweep.
  </done>
</task>

</tasks>

<threat_model>
## Trust Boundaries

| Boundary | Description |
|----------|-------------|
| HTML source → platform-server binary (via include_str!) | `web/verify/index.html` is compile-time-embedded into the platform-server binary. Malformed HTML survives `cargo check` (it's a string, not parsed), but a user-visible regression would appear only at runtime in a browser. Scope here is conservative — only two text edits, zero structural HTML changes. |
| action.yml source-of-truth → external GitHub Marketplace listing | This plan edits the monorepo source only. Phase 88 handles the external republish. A drift between the monorepo source-of-truth and the deployed marketplace action is an expected cross-phase state until Phase 88 lands. |
| README prose brand words (phase separation) | Must NOT premature-rename `TrustEdge` brand words into `Sealedge` — that is Phase 85/86 scope. A greedy sed-replace would violate phase boundaries. |

## STRIDE Threat Register

| Threat ID | Category | Component | Disposition | Mitigation Plan |
|-----------|----------|-----------|-------------|-----------------|
| T-84-10 | T (Tampering) | A greedy text-replace could rewrite `TrustEdge` brand words into sealedge equivalents, prematurely landing Phase 85/86 changes inside Phase 84 and corrupting the phased rebrand sequence. | mitigate | Enumerated line-by-line edits in `<interfaces>` reference the exact substring `.te-attestation.json` (which does NOT contain the brand word). Acceptance criteria explicitly assert `grep -c 'TrustEdge' ... >= 1` on the HTML and action README to prove brand-word prose survived. |
| T-84-11 | I (Information disclosure) | None — file-extension labels are public metadata; no cryptographic primitives or secrets involved. | accept | LOW severity; no mitigation needed. |
| T-84-12 | D (Denial of Service) | `web/verify/index.html` is `include_str!`-bundled into the platform-server binary. A file-system-level issue (missing file, wrong path) during edit would surface as a `cargo check` failure. | mitigate | Acceptance criterion `cargo check --workspace --locked` exits `0` enforces this. The plan edits the file in place — it does not move or rename the file. |
| T-84-13 | E (Elevation of privilege) | action.yml changes could, in principle, introduce command injection if an input was interpolated unsafely. Reviewed — the two edits (line 30 description text + line 89 bash string concatenation) do NOT alter the variable-interpolation structure; the OUT_PATH template remains anchored in `${{ runner.temp }}` (trusted) and `${BINARY_NAME}` (derived from the `inputs.binary` basename). | accept | No new attack surface introduced; the rename is a pure string-label change in an already-reviewed action flow. |
| T-84-14 | S (Spoofing) | UI label mismatch: if line 123 says `.se-attestation.json` but line 126 still says `.te-attestation.json` (or vice versa), a user could upload a file expecting one behavior and get another — low-impact since the verifier uses Ed25519 signatures, not filenames, for authenticity. | mitigate | Both edits land in the same commit. Acceptance criterion `grep -c '.se-attestation.json' web/verify/index.html == 2` enforces both sites got the new extension. |
</threat_model>

<verification>
- `cargo check --workspace --locked` green (validates `include_str!`-bundled HTML).
- `cargo test --workspace --locked` green.
- `cargo fmt --check` + `cargo clippy --workspace --all-targets -- -D warnings` green.
- Grep verification: zero `.te-attestation.json` literals remain in the 4 edited files; brand-word "TrustEdge" prose preserved per Phase 85/86 carve-outs.
</verification>

<success_criteria>
- Together with Plan 84-02, fully closes REBRAND-04b. The in-repo source-of-truth for `.se-attestation.json` is consistent across Rust sources, shell scripts, HTML, GitHub Action YAML + README, and deployment docs.
- ROADMAP Phase 84 Success Criterion #3 (attestation files written/read with sealedge extension across the GitHub Action and the verify HTML page) is satisfied.
- Commit atomic; workspace green at the boundary.
</success_criteria>

<output>
After completion, create `.planning/phases/84-crypto-constants-file-extension/84-03-SUMMARY.md` containing:
- Per-file replacement count (verify/index.html: 2; action.yml: 2; action README.md: 5; deploy README-deploy.md: 1 = 10 total).
- Confirmation greps showing zero `.te-attestation.json` remain in the 4 edited files.
- Confirmation `grep -c TrustEdge web/verify/index.html` returned `>= 1` and `grep -c TrustEdge actions/attest-sbom-action/README.md` returned `>= 1` (brand-word prose carve-outs preserved).
- Commit SHA of the atomic commit.
- Validation evidence: cargo check / fmt / clippy / test exit codes.
- Note cross-referencing Plan 84-02's SUMMARY — confirm the combined set fully closes REBRAND-04b.
</output>
