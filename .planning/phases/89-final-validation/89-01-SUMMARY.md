<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: sealedge — Privacy and trust at the edge.
-->

---
phase: 89-final-validation
plan: 01
subsystem: validation-scripts, milestone-close-docs
tags: [validation, milestone-close, sealedge, rebrand, v6.0]
requires:
  - .planning/phases/89-final-validation/89-CONTEXT.md (D-01, D-02, D-03, D-10, D-11, D-12, D-13, D-14)
  - .planning/phases/89-final-validation/89-PATTERNS.md (File 1, File 4, File 7, Shared.3)
  - scripts/ci-check.sh (primary analog for validate-v6.sh)
provides:
  - scripts/validate-v6.sh (reusable v6.0 validation gate: ci-parity matrix + WASM + dashboard + docker e2e + D-02 floor)
  - MIGRATION.md "What renamed" table row for the GitHub Action rename (TrustEdge-Labs/attest-sbom-action@v1 → TrustEdge-Labs/sealedge-attest-sbom-action@v2)
  - Audit record: ci.yml D-10 disposition ("verified clean, not modified")
  - Audit record: D-10 repo-wide grep raw output classification (allowlist gaps vs true-straggler scope)
affects:
  - Plan 89-02 (will invoke scripts/validate-v6.sh for VALID-01/03 evidence capture)
  - Plan 89-03 (release-notes body links to MIGRATION.md; new row is visible to release consumers)
  - v6.0 milestone close (validate-v6.sh is the single-command reproducible gate for future milestone closes — YAGNI-guarded v6-specific today)
tech-stack:
  added: []
  patterns:
    - shell hybrid-gate delegation (validate-v6.sh Step 1 shells out to ci-check.sh for the D-01 matrix)
    - tee-to-log evidence capture (exec > >(tee validate-v6.log) 2>&1)
    - trap-based docker teardown (cleanup_docker EXIT)
    - D-02 escape hatch with mandatory justification string + log line (--allow-regression "<text>" → "D-02 JUSTIFICATION: <text>")
key-files:
  created:
    - scripts/validate-v6.sh
  modified:
    - MIGRATION.md
decisions:
  - Monolithic validate-v6.sh (not a directory of focused scripts) — matches ci-check.sh proven shape; reuses PASS/FAIL/SKIP/WARN helpers verbatim
  - Hybrid delegation to ci-check.sh --clean for Step 1 — single source of truth for the CI-parity feature matrix; avoids drift
  - D-02 floor escape hatch uses positional-arg justification (not env var) — keeps the audit trail in the shell history
  - ci.yml D-10 reference is "verified clean, not modified" — post-Phase-88 refactor to `${{ steps.attest.outputs.attestation-path }}` already eliminated the hardcoded `.te-attestation.json` filename
metrics:
  duration: ~35 minutes (read files, author validate-v6.sh, audit, insert MIGRATION row, commit, write SUMMARY)
  tasks_completed: 2 / 2
  files_touched: 2 (1 new, 1 modified)
  commits: 1 (atomic — both tasks in one commit per plan's discipline)
  completed_date: 2026-04-22
---

# Phase 89 Plan 01: Validate-v6 Script + MIGRATION Action-Rename Row Summary

## One-line outcome

Shipped the reusable `scripts/validate-v6.sh` v6.0 validation-gate runner (ci-check.sh delegation + WASM + dashboard + docker e2e + D-02 floor with `--allow-regression` escape hatch) and filled the MIGRATION.md "What renamed" table D-14 hybrid gap with the GitHub Action rename row — ci.yml required no edit (verified clean, not modified).

## Deliverables

1. **`scripts/validate-v6.sh`** (new, 237 lines, executable, MPL-2.0 header with `Project: sealedge`):
   - Step 1: delegates full D-01 CI-parity feature matrix to `./scripts/ci-check.sh --clean`
   - Step 2: WASM cargo check on `wasm32-unknown-unknown` for `sealedge-wasm` + `sealedge-seal-wasm`
   - Step 3: WASM wasm-pack build + 2 MB size floor on `sealedge-wasm` (wasm-tests.yml parity per D-11)
   - Step 4: Dashboard `npm ci` + `npm run build` + `npm run check` (D-12 build-and-typecheck subset)
   - Step 5: Docker compose up + healthz wait (30×2s = 60s max) + `./scripts/demo.sh` + trap-based teardown (D-13)
   - Summary block: PASS/FAIL/WARN/SKIP totals with non-zero-FAIL exit 1
   - D-02 test-count floor (≥ 471) enforced by parsing `validate-v6.log`; `--allow-regression "<justification>"` flag writes `D-02 JUSTIFICATION: <text>` to the log for 89-VERIFICATION.md cross-reference
   - Two flags: `--skip-docker` + `--allow-regression "<justification>"`
   - Log capture via `exec > >(tee validate-v6.log) 2>&1` immediately after flag parsing

2. **`MIGRATION.md` "What renamed" table**: one new row inserted between `GitHub repo URL` and the `Preserved unchanged:` block:

   ```
   | GitHub Action | `TrustEdge-Labs/attest-sbom-action@v1` | `TrustEdge-Labs/sealedge-attest-sbom-action@v2` |
   ```

   Column count confirmed = 3 (4 pipes per row); all rows in the table share the same pipe count (awk check: 1 unique pipe-count value).

## D-10 audit output (literal — raw grep result summary)

**Command run (CONTEXT.md §Specifics verbatim):**

```bash
git ls-files \
  | xargs grep -nE "trustedge|TRUSTEDGE|\.trst\b|\.te-attestation" 2>/dev/null \
  | grep -vE "TrustEdge-Labs|TrustEdge Labs|trustedgelabs\.com|MIGRATION\.md|CHANGELOG\.md|^\.planning/(milestones|phases)/|RFC_K256_SUPPORT\.md|improvement-plan\.md|security-review-platform\.md"
```

**Raw hit count post-commit:** 875 lines across ~180 files. Classification:

| Category | Hit count | Disposition |
|---|---|---|
| Planning-management artifacts NOT yet in allowlist (`.planning/research/`, `.planning/codebase/`, `.planning/quick/`, `.planning/ideas/`, `.planning/RETROSPECTIVE.md`, `.planning/PROJECT.md`, `.planning/ROADMAP.md`, `.planning/STATE.md`, `.planning/REQUIREMENTS.md`, `.planning/MILESTONES.md`) | 444 | **Allowlist gap** — same audit-trail rationale as `^\.planning/(milestones\|phases)/`. Recommend extending the allowlist regex in Plan 02's validate-v6 gate invocation and future milestone-close sweeps to `^\.planning/` generally (the whole planning directory is historical prose, not product surface). |
| `.factory/security-review-cli-archive.md` | 30 | **Allowlist gap** — directly analogous to the already-allowlisted `security-review-platform.md`, `improvement-plan.md`, `RFC_K256_SUPPORT.md` (Phase 86 D-01a historical-security-review carve-out). Recommend adding `.factory/security-review-cli-archive.md` to the allowlist. |
| `.claude/settings.local.json`, `trustedge-yubikey-demo.cast` | ~15 | **Allowlist gap** — dev-local tooling state + pre-rename asciinema cast. Both are historical/user-local artifacts. Recommend allowlisting. |
| `Copyright (c) 2025 TRUSTEDGE LABS LLC` in source headers (the legal-entity string, not the brand) | 259 | **Already mentally allowlisted** — Phase 85 D-03 preserves `TRUSTEDGE LABS LLC` as the legal entity. The current allowlist regex (`TrustEdge Labs` + `TrustEdge-Labs`) does NOT catch the all-caps `TRUSTEDGE LABS LLC` form. Recommend adding `TRUSTEDGE LABS LLC` to the allowlist regex. |
| Residual genuine candidate stragglers after extending allowlist (e.g., `.github/workflows/*.yml` headers reading `Project: trustedge`, `.gitignore` `*.trst` + `trustedge-core/out.trst`, `crates/core/src/io/mod.rs`, `crates/core/src/crypto.rs` comments, `crates/wasm/examples/node-example.js`, `crates/seal-wasm/examples/node-example.js`, `deploy/digitalocean/Dockerfile`, `web/demo/index.html`, `Makefile`, etc.) | ~172 | **Deferred — scope-expansion shaped.** These are legitimate Phase 85/86 sweep gaps but span ~40+ files with hundreds of substitutions. Out-of-scope for Plan 01's atomic-commit boundary (D-14 "trivial rebrand-side-effect" threshold); log here for a hotfix sub-phase or Phase 89 Plan 02 validation findings. A targeted ripgrep pass with `rg -l 'Project: trustedge\|\\btrustedge_\\|TRUSTEDGE_\\|\\.trst\\b\\|\\.te-attestation'` shows the scope. |

**Plan 01 disposition:** D-10 grep is not "zero hits" today because the Phase 86 allowlist did not enumerate every planning-adjacent artifact directory and the legal-entity string pattern was incomplete. Plan 01 does NOT sweep those in this atomic commit — the plan's Step A instruction for allowlist gaps is "extend the allowlist pattern in the SUMMARY's audit-output section (do NOT modify the file in that case; it's an allowlist gap, not a straggler)". The recommended extended allowlist pattern (for Plan 02 and beyond) is:

```bash
grep -vE "TrustEdge-Labs|TrustEdge Labs|TRUSTEDGE LABS LLC|trustedgelabs\.com|MIGRATION\.md|CHANGELOG\.md|^\.planning/|RFC_K256_SUPPORT\.md|improvement-plan\.md|security-review-platform\.md|\.factory/security-review-cli-archive\.md|\.claude/settings\.local\.json|trustedge-yubikey-demo\.cast"
```

After extending the allowlist, residual ~172 candidate-straggler hits remain — those are flagged for a hotfix sub-phase outside this plan's scope.

## ci.yml D-10 disposition

**Result: verified clean, not modified.**

**Commands run (Step B pre-grep):**

```bash
grep -nE 'te-attestation|\.te-' .github/workflows/ci.yml
# exit 1 (no matches)

grep -nE 'te-attestation|\.te-' .github/workflows/*.yml
# exit 1 (no matches)
```

Phase 88 Plan 02's refactor to `${{ steps.attest.outputs.attestation-path }}` superseded the CONTEXT.md D-10 reference on lines 212/220 — the hardcoded `.te-attestation.json` filename was replaced by the action's output variable (sealedge-attest-sbom-action@v2 emits `.se-attestation.json` per Phase 84). No file edit performed; commit subject does NOT include `+ ci.yml straggler fix`.

## MIGRATION.md diff excerpt

```diff
 | V2 session key (experimental) | `TRUSTEDGE_V2_SESSION_KEY` | `SEALEDGE_V2_SESSION_KEY` |
 | V2 audio domain (experimental) | `TRUSTEDGE_AUDIO_V2` | `SEALEDGE_AUDIO_V2` |
 | GitHub repo URL | `TrustEdge-Labs/trustedge` | `TrustEdge-Labs/sealedge` |
+| GitHub Action | `TrustEdge-Labs/attest-sbom-action@v1` | `TrustEdge-Labs/sealedge-attest-sbom-action@v2` |

 Preserved unchanged:
 - `TrustEdge-Labs` (GitHub organization name)
```

**Column-consistency W-5 check:**

```bash
awk '/^### What renamed/{flag=1; next} /^### /{flag=0} flag && /^\|/ {n=gsub(/\|/,"|"); print n}' MIGRATION.md | sort -u | wc -l
# → 1 (all rows have the same pipe count — no structural break)
```

All rows in the table have 4 pipes = 3 columns. No table break. `git diff MIGRATION.md`: exactly +1 line, 0 removed.

## validate-v6.sh structural outline

| Step | Purpose | Pass/Fail/Skip rows emitted | D-ref |
|---|---|---|---|
| Flag parsing | `--skip-docker`, `--allow-regression "<text>"` (non-empty validation) | none (silent parse) | D-02, D-13 |
| `exec > >(tee validate-v6.log)` | capture all stdout/stderr to log for 89-VERIFICATION.md evidence | none | D-03 |
| Banner | "● Sealedge v6.0 validation gate" + mode flags echo | none | — |
| Step 1 | Delegate full D-01 matrix to `./scripts/ci-check.sh --clean` | 1 (pass/fail) | D-01 |
| Step 2 | WASM cargo check on wasm32 for both crates | 1 (pass/fail/skip on target absent) | D-11 |
| Step 3 | wasm-pack build both crates + 2 MB floor on sealedge-wasm | 1 (pass/fail/skip on wasm-pack absent) | D-11 |
| Step 4 | Dashboard npm ci + build + check | 3 (one per npm cmd) | D-12 |
| Step 5 | Docker compose up + healthz (30×2s) + demo.sh + trap-teardown | 2 (healthz + demo) or skip | D-13 |
| Summary | "━" rule + "Results: $PASS passed, $FAIL failed, $WARN warnings, $SKIP skipped" + exit 1 on fail | — | — |
| D-02 floor | grep validate-v6.log for `test result: ok. N passed` → sum → < 471 exits 1 unless `--allow-regression` writes `D-02 JUSTIFICATION: <text>` | — | D-02 |

Total green-path rows on a healthy dev box: 8 pass rows (1 CI-parity + 1 WASM check + 1 WASM size + 3 dashboard + 2 docker).

## Self-Check

**Files created:**

- `scripts/validate-v6.sh`: `[ -f scripts/validate-v6.sh ] && echo "FOUND"` → FOUND
- `.planning/phases/89-final-validation/89-01-SUMMARY.md` (this file): FOUND (self)

**Files modified:**

- `MIGRATION.md`: `grep -c 'sealedge-attest-sbom-action@v2' MIGRATION.md` → 1 (FOUND)

**Commit verification:**

- `05862ae`: `git log --oneline --all | grep -q 05862ae && echo FOUND` → FOUND
- Commit subject: `feat(89-01): add scripts/validate-v6.sh validation gate + MIGRATION.md action-rename row (D-14 hybrid fix)` ✓
- `git show --stat 05862ae`: 2 files changed, 238 insertions(+), 0 deletions ✓
- No accidental file deletions ✓
- No untracked leftovers ✓

**Acceptance criteria cross-check (Plan 01 Task 1 + Task 2):**

| Criterion | Command | Result |
|---|---|---|
| scripts/validate-v6.sh executable | `test -x scripts/validate-v6.sh` | ✓ exit 0 |
| scripts/validate-v6.sh syntax-valid | `bash -n scripts/validate-v6.sh` | ✓ exit 0 |
| MPL-2.0 header in first 5 lines | `head -5 scripts/validate-v6.sh \| grep -c 'MPL-2.0'` | 1 ≥ 1 ✓ |
| `Project: sealedge` in first 5 lines | `head -5 scripts/validate-v6.sh \| grep -c 'Project: sealedge'` | 1 ≥ 1 ✓ |
| D-01 matrix or ci-check.sh delegation present | `grep -c 'cargo test --workspace --no-default-features --locked\|./scripts/ci-check.sh'` | 2 ≥ 1 ✓ |
| `wasm-pack build --target web --release` count ≥ 2 | `grep -c 'wasm-pack build --target web --release'` | 2 ≥ 2 ✓ |
| `docker compose -f deploy/docker-compose.yml` count ≥ 2 | `grep -c 'docker compose -f deploy/docker-compose.yml'` | 3 ≥ 2 ✓ |
| `2097152` present | `grep -c '2097152'` | 3 ≥ 1 ✓ |
| `\b471\b` present | `grep -cE '\b471\b'` | 5 ≥ 1 ✓ |
| `npm run build` present | `grep -c 'npm run build'` | 3 ≥ 1 ✓ |
| `npm run check` present | `grep -c 'npm run check'` | 3 ≥ 1 ✓ |
| `allow-regression` present | `grep -c 'allow-regression'` | 7 ≥ 1 ✓ |
| `D-02 JUSTIFICATION:` present | `grep -c 'D-02 JUSTIFICATION:'` | 1 ≥ 1 ✓ |
| Line count 120–240 | `wc -l < scripts/validate-v6.sh` | 237 (in range) ✓ |
| No emoji in file | `grep -cP '[\x{1F300}-\x{1F9FF}]' scripts/validate-v6.sh` | 0 ✓ |
| Step 1 (copyright-headers) would pass for new file | mirrored ci-check.sh Step 1 loop over `.rs/.yml/.yaml/.toml` | 0 missing ✓ (`.sh` not in scope) |
| MIGRATION.md `sealedge-attest-sbom-action@v2` present | `grep -c 'sealedge-attest-sbom-action@v2'` | 1 ≥ 1 ✓ |
| MIGRATION.md `TrustEdge-Labs/attest-sbom-action@v1` present in Before column | `grep -c 'TrustEdge-Labs/attest-sbom-action@v1'` | 1 ≥ 1 ✓ |
| MIGRATION.md exactly 1 new `\| GitHub Action ` row | `grep -c '^\| GitHub Action '` | 1 ✓ |
| ci.yml zero te-attestation hits post-task | `grep -cE 'te-attestation\|\.te-' .github/workflows/ci.yml` | 0 ✓ |
| MIGRATION.md table column-consistency (W-5) | awk pipe-count check | 1 unique value ✓ |
| MIGRATION.md copyright header preserved | `head -6 MIGRATION.md \| grep -c 'Project: sealedge'` | 1 ≥ 1 ✓ |
| MIGRATION.md lines added ≤ 3 | `git diff MIGRATION.md \| grep -c '^+' (excluding +++)` | 1 ≤ 3 ✓ |
| MIGRATION.md lines removed = 0 | `git diff MIGRATION.md \| grep -c '^-' (excluding ---)` | 0 ✓ |

**All acceptance criteria met.** Self-Check: PASSED.

## Deviations from Plan

**None for the prescribed tasks.**

### Auto-noted during execution (not actionable in this plan)

**1. [Rule 2-borderline — Allowlist scope finding] D-10 audit uncovered larger allowlist gaps and residual stragglers than PLAN/CONTEXT assumed.**
- **Found during:** Task 2 Step A audit
- **Observation:** CONTEXT.md §Specifics predicted "zero results outside the known `ci.yml` lines 212/220 stragglers"; actual result is 875 raw hits after the plan's allowlist. Of those: (a) ~444 are legitimate planning-management historical artifacts missed by the `^\.planning/(milestones\|phases)/` pattern (e.g., `.planning/research/`, `.planning/codebase/`, `.planning/quick/`, `.planning/ideas/`, `.planning/PROJECT.md`, `.planning/ROADMAP.md`, etc.), (b) 259 are `TRUSTEDGE LABS LLC` legal-entity strings (Phase 85 D-03 carve-out but not in the allowlist regex), (c) ~30 are historical `.factory/security-review-cli-archive.md` (parallel to the existing security-review-platform.md allowlist), (d) residual ~172 are candidate true stragglers (`Project: trustedge` in `.yml` workflow headers, `.trst` in `.gitignore` + examples, old comments in `crates/core/src/{io,crypto}.rs`, etc.).
- **Action taken:** Per the plan's explicit instruction ("If hits appear, classify each... allowlist gap → do NOT modify the file; extend allowlist pattern in SUMMARY's audit-output section"), documented the classification in §"D-10 audit output" above with a recommended extended allowlist regex for Plan 02 and future milestone-close sweeps. Did NOT modify files beyond the two prescribed by the plan (validate-v6.sh + MIGRATION.md row).
- **Rule 4 scope-judgment call:** The ~172 residual candidate stragglers span ~40+ files with hundreds of substitutions. That is clearly scope-expansion territory (Phase 85/86 sweep gaps), not a trivial rebrand-side-effect per D-14. Flagged for a hotfix sub-phase outside Plan 01's atomic-commit boundary. **Not fixed in this commit** — consistent with the plan's Phase 85/86 D-14 decision boundary: "could a disciplined reviewer reasonably call this a rebrand side-effect?" — yes each individual hit could, but the aggregate scope is not trivial inline-fix shape.

## Hybrid-gate findings

None fixed in this plan beyond the prescribed MIGRATION.md row. The residual ~172 candidate stragglers are recorded above for a hotfix sub-phase.

## Authentication gates

None.

## Commit SHA

- `05862ae` — `feat(89-01): add scripts/validate-v6.sh validation gate + MIGRATION.md action-rename row (D-14 hybrid fix)` (HEAD)

## Files and line-level evidence

| File | Change | Line count |
|---|---|---|
| `scripts/validate-v6.sh` | new (+237 lines) | 237 |
| `MIGRATION.md` | modified (+1 line) | new row inserted between line 40 (GitHub repo URL) and line 42 (Preserved unchanged) |
| `.planning/phases/89-final-validation/89-01-SUMMARY.md` | new (this file, plan close artifact) | — |

## Notes for Plan 02 and downstream consumers

- `scripts/validate-v6.sh` is the single-command reproducible gate for v6.0 milestone close. Plan 02 will invoke it and capture `validate-v6.log` excerpts into 89-VERIFICATION.md §VALID-01 / §VALID-03.
- The `--allow-regression "<justification>"` flag MUST be accompanied by a matching `D-02 justification: <text>` note in 89-VERIFICATION.md §1 — this is both CONTEXT.md D-02 discipline and an explicit cross-check the script writes to validate-v6.log so the verifier agent can confirm both sides match.
- The D-10 allowlist gap discovered in Task 2 should be applied to Plan 02's pre-tag re-grep — use the extended allowlist regex documented in §"D-10 audit output" above, not the CONTEXT.md one verbatim, so the zero-hit acceptance criterion holds without needing to re-classify 444+ hits each time.
- The residual ~172 candidate stragglers (Phase 85/86 sweep gaps, top culprits `crates/wasm/examples/node-example.js` @21 hits, `crates/seal-wasm/examples/node-example.js` @21, `deploy/digitalocean/Dockerfile` @11, `web/demo/index.html` @10, `deploy/Dockerfile` @9, `.gitignore` @8, `deploy/docker-compose.yml` @7) should be handled in a post-tag hotfix sub-phase — they are legitimate rename targets but not trivial inline-fix scope for Phase 89's pre-tag close-out.
