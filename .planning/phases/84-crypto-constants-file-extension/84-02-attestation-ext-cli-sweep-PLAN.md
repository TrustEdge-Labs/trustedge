---
phase: 84-crypto-constants-file-extension
plan: 02
type: execute
wave: 2
depends_on: ["84-01"]
files_modified:
  - crates/core/src/point_attestation.rs
  - crates/seal-cli/src/main.rs
  - crates/seal-cli/tests/acceptance.rs
  - scripts/demo-attestation.sh
autonomous: true
requirements: [REBRAND-04b]

must_haves:
  truths:
    - "seal-cli's attest-sbom default output path is `attestation.se-attestation.json` (not `attestation.te-attestation.json`)"
    - "seal-cli's CLI help text for the attest-sbom --out flag and the verify-attestation positional arg references `.se-attestation.json`"
    - "crates/seal-cli/tests/acceptance.rs test fixtures use `.se-attestation.json` for all attestation-file paths, and the default-output assertion checks for `attestation.se-attestation.json`"
    - "crates/core/src/point_attestation.rs to_json doc comment references `.se-attestation.json`"
    - "scripts/demo-attestation.sh writes to a `.se-attestation.json` output path"
    - "The seal-cli binary runs end-to-end: attest-sbom produces a .se-attestation.json file, verify-attestation reads it"
    - "cargo check --workspace --locked green at the commit boundary"
    - "cargo test --workspace --locked green"
  artifacts:
    - path: "crates/seal-cli/src/main.rs"
      provides: "attest-sbom default output path + help text aligned to .se-attestation.json"
      contains: 'attestation.se-attestation.json'
    - path: "crates/seal-cli/tests/acceptance.rs"
      provides: "Test fixtures asserting .se-attestation.json output"
      contains: '.se-attestation.json'
    - path: "scripts/demo-attestation.sh"
      provides: "Demo writes to .se-attestation.json"
      contains: '.se-attestation.json'
    - path: "crates/core/src/point_attestation.rs"
      provides: "Point attestation doc comment aligned to new extension"
      contains: '.se-attestation.json'
  key_links:
    - from: "seal-cli attest-sbom --out default"
      to: "on-disk attestation filename"
      via: 'PathBuf::from("attestation.se-attestation.json")'
      pattern: 'attestation\.se-attestation\.json'
    - from: "scripts/demo-attestation.sh ATTESTATION_PATH"
      to: "attest-sbom --out argument"
      via: "shell variable"
      pattern: '\.se-attestation\.json'
    - from: "acceptance.rs test assertions"
      to: "CLI-produced output filenames"
      via: "tempdir.path().join(\".se-attestation.json\")"
      pattern: '\.se-attestation\.json'
---

<objective>
Sweep the `.te-attestation.json` attestation-file extension to `.se-attestation.json` across all executable Rust code owned by the workspace (core crate doc comment + seal-cli CLI help and default-output literal + seal-cli acceptance tests) and the demo shell script. Clean rename — no dual-accept (per CONTEXT D-04).

Purpose: Completes the in-repo Rust/shell portion of REBRAND-04b. The external-asset portion (HTML verify page + GitHub Action + deployment README) is handled in parallel by Plan 03.

Output: 4 files edited — `crates/core/src/point_attestation.rs` (1 doc-comment line), `crates/seal-cli/src/main.rs` (3 sites: `--out` help text, `attestation` positional help text, default-path literal), `crates/seal-cli/tests/acceptance.rs` (5 fixture paths + 1 default-output assertion), `scripts/demo-attestation.sh` (1 path literal). Single atomic commit. Workspace builds + tests green.

Rationale (Plan 02 shape): Depends on Plan 01 because the end-to-end cargo test run in this plan exercises `keygen` / `attest-sbom` binaries that now produce `SEALEDGE-KEY-V1`-headed key files; the seal-cli acceptance tests at `crates/seal-cli/tests/acceptance.rs` traverse the full keygen→attest-sbom→verify-attestation pipeline and must see the new key-file header. Running this plan before Plan 01 lands would either fail cargo test (tests use the new header via the binary) or force non-atomic commits — both violate the commit-granularity rule.
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

crates/core/src/point_attestation.rs:
```
line 234:  /// Serialize to pretty-printed JSON for writing to a `.te-attestation.json` file.
             → /// Serialize to pretty-printed JSON for writing to a `.se-attestation.json` file.
```

crates/seal-cli/src/main.rs (3 sites):
```
line 322:  help = "Output path [default: attestation.te-attestation.json]"
             → help = "Output path [default: attestation.se-attestation.json]"

line 331:  #[arg(value_name = "ATTESTATION", help = "Path to .te-attestation.json file")]
             → #[arg(value_name = "ATTESTATION", help = "Path to .se-attestation.json file")]

line 1461: .unwrap_or_else(|| PathBuf::from("attestation.te-attestation.json"));
             → .unwrap_or_else(|| PathBuf::from("attestation.se-attestation.json"));
```

The word "attestation" in `attestation.te-attestation.json` is a generic English noun prefix (per CONTEXT D-04 Claude's Discretion) — it stays; only the `te-` inside the extension renames.

crates/seal-cli/tests/acceptance.rs (6 sites — all test fixtures):
```
line 1149:  let out_path = tempdir.path().join("output.te-attestation.json");
             → .join("output.se-attestation.json")

line 1207:  let default_out = tempdir.path().join("attestation.te-attestation.json");
             → .join("attestation.se-attestation.json")

line 1210:  "default output file attestation.te-attestation.json should exist"
             → "default output file attestation.se-attestation.json should exist"

line 1290:  let out_path = tempdir.path().join("result.te-attestation.json");
             → .join("result.se-attestation.json")

line 1320:  let out_path = tempdir.path().join("attest.te-attestation.json");
             → .join("attest.se-attestation.json")

line 1363:  let out_path = tempdir.path().join("attest.te-attestation.json");
             → .join("attest.se-attestation.json")

line 1424:  let out_path = tempdir.path().join("attest.te-attestation.json");
             → .join("attest.se-attestation.json")
```

scripts/demo-attestation.sh:
```
line 167:  ATTESTATION_PATH="$DEMO_DIR/attestation.te-attestation.json"
             → ATTESTATION_PATH="$DEMO_DIR/attestation.se-attestation.json"
```

Note: `demo-attestation.sh` contains `echo` statements with the word "TrustEdge" in surrounding prose. Per CONTEXT `<domain>` and `<deferred>`: those brand-word prose references STAY — Phase 85/86 scope. Only the `.te-attestation.json` → `.se-attestation.json` output path literal changes here.
</interfaces>

<tasks>

<task type="auto">
  <name>Task 1: Rename `.te-attestation.json` → `.se-attestation.json` across core doc comment, seal-cli CLI+tests, and the demo script; commit atomically</name>
  <read_first>
    - .planning/phases/84-crypto-constants-file-extension/84-CONTEXT.md (§Decisions D-04 clean rename no dual-accept; §Claude's Discretion for the "attestation." prefix; §Deferred for demo-script brand-word prose carve-out)
    - .planning/phases/84-crypto-constants-file-extension/84-01-SUMMARY.md (confirm Plan 01 landed — the new SEALEDGE-KEY-V1 header is live, so CLI keygen/attest-sbom paths in the acceptance tests will produce the new header)
    - crates/core/src/point_attestation.rs (lines 225-243 — the `to_json` fn and its doc comment)
    - crates/seal-cli/src/main.rs lines 310-340 (the attest-sbom Args struct + verify-attestation struct with the help-text sites at lines 322 and 331)
    - crates/seal-cli/src/main.rs lines 1455-1475 (the attest-sbom handler with the default-path `unwrap_or_else` at line 1461)
    - crates/seal-cli/tests/acceptance.rs lines 1140-1430 (the 7 test sites listed in <interfaces>; scan surrounding ±20 lines to catch any adjacent string-match assertions that reference the extension)
    - scripts/demo-attestation.sh (entire file — small, needed to confirm the brand-word `echo` lines are carved out and only the `ATTESTATION_PATH` literal is rewritten)
    - CLAUDE.md § "Build & Test Commands" and § "Code Standards"
  </read_first>
  <files>
    crates/core/src/point_attestation.rs,
    crates/seal-cli/src/main.rs,
    crates/seal-cli/tests/acceptance.rs,
    scripts/demo-attestation.sh
  </files>
  <action>
**Step 1 — crates/core/src/point_attestation.rs line 234.**

Single-line doc-comment edit:
```
/// Serialize to pretty-printed JSON for writing to a `.te-attestation.json` file.
```
→
```
/// Serialize to pretty-printed JSON for writing to a `.se-attestation.json` file.
```

No other line in this file references the extension literal (confirmed via `grep -n 'te-attestation' crates/core/src/point_attestation.rs` returning 1 hit).

**Step 2 — crates/seal-cli/src/main.rs (3 sites, per `<interfaces>` above).**

Apply each exact replacement at lines 322, 331, 1461. Verify file still compiles after each via `cargo check -p trustedge-seal-cli --locked` (if the crate name is `trustedge-seal-cli`; else check with `cargo check -p seal-cli --locked` — use the name listed in `crates/seal-cli/Cargo.toml` `[package].name`).

**Step 3 — crates/seal-cli/tests/acceptance.rs (7 sites, per `<interfaces>` above).**

Apply each exact replacement. The 7 sites are enumerated — if `grep -c 'te-attestation' crates/seal-cli/tests/acceptance.rs` returns anything other than `7` before editing, stop and re-read the file; an extra occurrence means the `<interfaces>` enumeration is stale.

**Step 4 — scripts/demo-attestation.sh line 167.**

Single-line edit of the `ATTESTATION_PATH` assignment. Do NOT edit any `echo` or `printf` line that contains "TrustEdge" as surrounding prose — those stay for Phase 85/86 (confirmed by CONTEXT `<deferred>` item 2).

**Step 5 — Validate the full workspace.**

```bash
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo check --workspace --locked
cargo test --workspace --locked
```

If acceptance.rs tests fail with an assertion like "expected file attest.se-attestation.json to exist, not found" — that means the CLI produced a file with a different name than the test expected; double-check that the default-path literal at main.rs line 1461 was renamed correctly AND that each `--out <path>.se-attestation.json` flag in the test invocations matches the test's `out_path` variable.

**Step 6 — Sanity-check the demo script end-to-end (non-blocking, optional):**

```bash
./scripts/demo-attestation.sh  # Expected: produces a .se-attestation.json in the demo dir and verifies it.
```

If the demo script fails for any reason unrelated to the extension rename (e.g. missing docker, build cache issues) — note it in the SUMMARY but do NOT block the commit. The commit is gated on cargo fmt/clippy/check/test, not on the optional demo run.

**Step 7 — Commit atomically (commit-granularity rule: all 4 files together, one commit).**

```bash
git add crates/core/src/point_attestation.rs \
        crates/seal-cli/src/main.rs \
        crates/seal-cli/tests/acceptance.rs \
        scripts/demo-attestation.sh

git commit -m "$(cat <<'EOF'
refactor(84-02): rename .te-attestation.json → .se-attestation.json in Rust + demo script

Sweeps the attestation-file extension to its sealedge form across the
workspace Rust sources and the demo shell script. Clean rename — no
dual-accept (per CONTEXT.md D-04).

  - crates/core/src/point_attestation.rs:234 — doc comment on to_json fn

  - crates/seal-cli/src/main.rs (3 sites):
      line 322: attest-sbom --out help "[default: attestation.te-attestation.json]"
                → "[default: attestation.se-attestation.json]"
      line 331: verify-attestation positional help "Path to .te-attestation.json file"
                → "Path to .se-attestation.json file"
      line 1461: default output path literal
                "attestation.te-attestation.json" → "attestation.se-attestation.json"
      (The word "attestation." is the generic English prefix — unchanged.)

  - crates/seal-cli/tests/acceptance.rs (7 sites): all .te-attestation.json
    fixture paths + the default-output assertion message updated to
    .se-attestation.json.

  - scripts/demo-attestation.sh:167 — ATTESTATION_PATH variable.
    Brand-word "TrustEdge" echo statements deliberately LEFT for Phase 85/86.

External-asset portion of REBRAND-04b (web/verify/index.html,
actions/attest-sbom-action/*, deploy/digitalocean/README-deploy.md)
is handled by Plan 84-03 in parallel (no file overlap).

Validation:
  - cargo fmt --check green
  - cargo clippy --workspace --all-targets -- -D warnings green
  - cargo check --workspace --locked green
  - cargo test --workspace --locked green — acceptance tests exercise
    keygen → attest-sbom → verify-attestation end-to-end under the new
    extension and against the SEALEDGE-KEY-V1 header from Plan 84-01.

Requirements: REBRAND-04b (partial — external assets are Plan 84-03).

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```
  </action>
  <acceptance_criteria>
    - `grep -c '\.te-attestation\.json' crates/core/src/point_attestation.rs` returns `0`.
    - `grep -c '\.se-attestation\.json' crates/core/src/point_attestation.rs` returns `1`.
    - `grep -c '\.te-attestation\.json' crates/seal-cli/src/main.rs` returns `0`.
    - `grep -c '\.se-attestation\.json' crates/seal-cli/src/main.rs` returns `3` (lines 322, 331, 1461).
    - `grep -n 'attestation.se-attestation.json' crates/seal-cli/src/main.rs` returns `2` lines (line 322 help text + line 1461 default path — the literal `attestation.se-attestation.json` appears in both; line 331's `"Path to .se-attestation.json file"` does NOT include the `attestation.` prefix).
    - `grep -c '\.te-attestation\.json' crates/seal-cli/tests/acceptance.rs` returns `0`.
    - `grep -c '\.se-attestation\.json' crates/seal-cli/tests/acceptance.rs` returns `7` (per the `<interfaces>` enumeration: lines 1149, 1207, 1210, 1290, 1320, 1363, 1424).
    - `grep -c '\.te-attestation\.json' scripts/demo-attestation.sh` returns `0`.
    - `grep -c '\.se-attestation\.json' scripts/demo-attestation.sh` returns `1`.
    - `grep -c 'TrustEdge' scripts/demo-attestation.sh` returns `≥ 1` (the brand-word echo prose is preserved per the CONTEXT carve-out).
    - `cargo fmt --check` exits `0`.
    - `cargo clippy --workspace --all-targets -- -D warnings` exits `0`.
    - `cargo check --workspace --locked` exits `0` at the commit boundary.
    - `cargo test --workspace --locked` exits `0`, specifically including all seal-cli acceptance tests that exercise the keygen → attest-sbom → verify-attestation pipeline.
    - `cargo test -p trustedge-seal-cli --test acceptance` exits `0` (OR the equivalent package name if `crates/seal-cli/Cargo.toml` uses a different `[package].name` — the executor must resolve the actual package name before running this check).
    - `git log -1 --pretty=%s` returns a string starting with `refactor(84-02):`.
    - `git status --porcelain` is empty after the commit.
  </acceptance_criteria>
  <verify>
    <automated>cargo check --workspace --locked && cargo fmt --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --locked</automated>
  </verify>
  <done>
    - All 4 files edited and the 16 total `.te-attestation.json` → `.se-attestation.json` replacements landed in one atomic commit.
    - No production code paths retain the legacy extension literal.
    - Brand-word prose in scripts/demo-attestation.sh explicitly preserved for Phase 85/86 scope.
    - Workspace builds + tests green; Plan 03 may land in parallel.
  </done>
</task>

</tasks>

<threat_model>
## Trust Boundaries

| Boundary | Description |
|----------|-------------|
| CLI help-text claim vs. actual default output path | If the help text advertises `.se-attestation.json` but the `unwrap_or_else` still constructs `.te-attestation.json` (or vice versa), users get a file whose name does not match the documented default — low-impact but a correctness bug. |
| Test-fixture path literals vs. CLI-produced file names | Test assertions that hard-code `.te-attestation.json` while the CLI produces `.se-attestation.json` (or vice versa) will cause cargo test failures — a canary that catches drift. |
| Demo-script brand-word prose vs. extension literal | A greedy sed-replace of `te` → `se` on `demo-attestation.sh` would corrupt any word containing `te` (e.g. "TrustEdge") — the rename must be anchored to `.te-attestation.json` as a complete token. |

## STRIDE Threat Register

| Threat ID | Category | Component | Disposition | Mitigation Plan |
|-----------|----------|-----------|-------------|-----------------|
| T-84-06 | T (Tampering) | Over-aggressive text replacement could corrupt the "TrustEdge" brand word in `demo-attestation.sh` echo prose, prematurely landing Phase 85/86 changes inside Phase 84. | mitigate | Enumerated exact line-by-line edits in `<interfaces>`; `<acceptance_criteria>` asserts `grep -c 'TrustEdge' scripts/demo-attestation.sh >= 1` AND `grep -c '.te-attestation.json' scripts/demo-attestation.sh == 0`. The sed pattern `.te-attestation.json` is specific enough to avoid collateral damage. |
| T-84-07 | D (Denial of Service) | Test fixtures drift from CLI default (acceptance.rs line 1207 asserts `attestation.se-attestation.json` exists; if main.rs line 1461 remains `te-`, the test fails). | mitigate | Both files land in the SAME commit; `cargo test --workspace --locked` gate before commit catches any drift. |
| T-84-08 | I (Information disclosure) | None — this plan changes file-extension labels only. No cryptographic primitives, no key handling, no authentication surface. Pure cosmetic-per-CONTEXT. | accept | No mitigation needed — the extension rename is a labeling change with zero impact on the cryptographic scheme. LOW severity. |
| T-84-09 | S (Spoofing) | A user could name an arbitrary file `foo.se-attestation.json` and have the CLI / tests treat it as an attestation. | accept | This is intrinsic to any file-extension-based labeling scheme. The attestation format is validated by Ed25519 signature verification (on the JSON content), not by filename. The rename from `.te-` to `.se-` does not change this threat posture. |
</threat_model>

<verification>
- `cargo check --workspace --locked` green (phase-83-carry-forward commit-granularity rule).
- `cargo test --workspace --locked` green — seal-cli acceptance tests exercise the full CLI pipeline under the new extension against the new SEALEDGE-KEY-V1 header from Plan 84-01.
- `cargo fmt --check` + `cargo clippy --workspace --all-targets -- -D warnings` green.
- Grep verification: zero `.te-attestation.json` literals remain in the 4 edited files; the "TrustEdge" brand-word prose in the demo script is preserved.
</verification>

<success_criteria>
- Partially satisfies REBRAND-04b: the Rust/shell portion of the attestation-extension rename is complete (`crates/core`, `crates/seal-cli` src + tests, demo script). The external-asset portion (HTML, action.yml, deployment README) is finalized by Plan 84-03.
- ROADMAP Phase 84 Success Criterion #3 (attestation files written/read with sealedge extension across CLI subcommands) is covered by this plan for the CLI subcommands portion.
- Commit atomic; workspace green at the boundary.
</success_criteria>

<output>
After completion, create `.planning/phases/84-crypto-constants-file-extension/84-02-SUMMARY.md` containing:
- Per-file replacement count (point_attestation.rs: 1; main.rs: 3; acceptance.rs: 7; demo-attestation.sh: 1 = 12 total).
- Confirmation grep commands showing zero `.te-attestation.json` remain in the 4 edited files.
- Confirmation `grep -c TrustEdge scripts/demo-attestation.sh` returned `>= 1` (brand-word prose carve-out preserved).
- Commit SHA of the atomic commit.
- Validation evidence: cargo fmt / clippy / check / test exit codes + which acceptance test names exercise the full CLI pipeline under the new extension.
</output>
