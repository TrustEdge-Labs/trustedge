# Phase 89: Final Validation - Pattern Map

**Mapped:** 2026-04-21
**Files analyzed:** 6 (3 new + 3 modified) — plus 1 conditional (`MIGRATION.md` link-target verification)
**Analogs found:** 6 / 6 (every file in scope has a clear in-repo analog — mostly from Phases 85/86/87/88 + `scripts/ci-check.sh`)
**Phase shape:** Validation-gate + evidence-capture phase (Phase 87/88 carry-forward) + one net-new reusable shell script (`scripts/validate-v6.sh`, modeled on the existing `scripts/ci-check.sh`) + one net-new release-notes body file (first such file in this repo — analog is the MIGRATION.md v6.0 header block and CHANGELOG v5.0/v4.0 entries). No net-new product code.

**No RESEARCH.md for this phase** (validation work reuses existing ci.yml command matrix and prior-phase patterns verbatim). All patterns are extracted from existing sealedge artifacts.

---

## File Classification

All files in scope are grouped into the 4 plan-shaped clusters implied by CONTEXT.md §"Claude's Discretion" — plan (a) pre-tag sweep + script, plan (b) evidence capture, plan (c) tag cut + release notes, plan (d) milestone-close artifacts.

| File | New/Modified | Role | Data Flow | Closest Analog | Match Quality |
|------|--------------|------|-----------|----------------|---------------|
| `scripts/validate-v6.sh` | **new** | shell script (CI-parity matrix runner) | batch / sequential-gate | `scripts/ci-check.sh` | **exact** (same pass/fail/skip/warn structure, same `step()` helper, same final-summary block, same non-zero-exit-on-FAIL discipline) |
| `.planning/phases/89-final-validation/89-VERIFICATION.md` | **new** | phase-close evidence document | prose / structured-table | `88-VERIFICATION.md` + `87-VERIFICATION.md` | **exact** (section-per-requirement layout, command/output fenced blocks, rollback-status section, ROADMAP-success-criteria-table at bottom) |
| `.planning/phases/89-final-validation/RELEASE-NOTES-v6.0.0.md` | **new** | GitHub release body for `gh release create --notes-file` | prose (Markdown body, no front-matter) | `MIGRATION.md` §"v6.0: trustedge → sealedge rebrand — clean break" top block + `CHANGELOG.md` v5.0/v4.0 entries | role-match (same "what changed / breaking changes / link to MIGRATION" shape; planner decides whether the file lands at phase-local `.planning/phases/89-final-validation/` or repo root per CONTEXT Claude's-Discretion bullet) |
| `.github/workflows/ci.yml` lines 212/220 | **modified** | YAML workflow (stale straggler fix) | in-place string substitution | Phase 87 Plan 01 Task 1 `cla.yml` targeted Edit pattern + Phase 88 Plan 02 ci.yml straggler-fix pattern | **exact** (identical straggler-fix shape: `seal.te-attestation.json` → `seal.se-attestation.json` is a two-line Phase-84-carryforward sweep; same allowlisted-grep driver as Phase 85/86) |
| `.planning/ROADMAP.md` | **modified** | planning document (mark Phase 89 done + v6.0 shipped + final phase count) | prose (append-and-edit) | v4.0 milestone-close commit `dc0652e` + Phase 88 ROADMAP update pattern (Progress table row + top-line milestone-shipped marker) | **exact** (identical two-touchpoint edit: flip Phase 89 checkbox + completion date in Progress table AND flip v6.0 Milestones header line from `📋 Planned` to `✅ Shipped`) |
| `.planning/PROJECT.md` | **modified** | planning document (flip Current Milestone → Shipped) | prose (append-and-edit) | v4.0 milestone-close commit `dc0652e` PROJECT.md diff (Current-State paragraph + Active-section flip + Completed-Milestones append) | **exact** (three-touchpoint edit: Current-State paragraph rewrite + Active-section → "No active requirements" placeholder + Completed-Milestones v6.0 bullet append + `*Last updated:*` timestamp refresh) |
| `MIGRATION.md` | **conditional-verify** | link target check (the release notes link to this file; planner verifies breaking-change summary still matches) | read-only audit | `MIGRATION.md` v6.0 section (existing) | self-analog (this is a **read-only verification** step, not an edit; confirm existing table covers all items enumerated in RELEASE-NOTES-v6.0.0.md §Breaking changes) |

---

## Pattern Assignments

Each file (or file group) below has the concrete excerpt the planner can reference in PLAN.md `<action>` blocks and `acceptance_criteria` lists.

---

### File 1 — `scripts/validate-v6.sh` (new; D-03)

**Analog:** `scripts/ci-check.sh` — the existing local CI-parity script. CONTEXT.md §"Existing Code Insights" calls it out explicitly as a candidate compose target.

**Key design decision (D-03 / CONTEXT.md §Claude's-Discretion):** Planner chooses between (a) monolithic `validate-v6.sh` OR (b) directory of focused scripts with orchestrator. **Recommendation: monolithic**, mirroring `ci-check.sh`'s single-file 12-step structure — reusability for v7+ milestones is the constraint, and `ci-check.sh` has proven that single-file works for 12 steps across 329 lines.

**Imports / boilerplate pattern** (from `scripts/ci-check.sh:1-16`):

```bash
#!/bin/bash
# Copyright (c) 2025 TRUSTEDGE LABS LLC
# MPL-2.0: https://mozilla.org/MPL/2.0/
# Project: sealedge — Privacy and trust at the edge.
#
# Local CI check — mirrors .github/workflows/ci.yml
# Run before pushing to catch issues without burning GH Actions minutes.
#
# Usage:
#   ./scripts/ci-check.sh          # Fast incremental (default)
#   ./scripts/ci-check.sh --clean  # Full clean build (matches CI exactly)

set -e

cd "$(dirname "$0")/.."
```

**Copy directly for validate-v6.sh** with subject-line adjusted: `# Full v6.0 validation gate — mirrors ci.yml build-and-test feature matrix + WASM + dashboard + docker e2e`. Keep `set -e` (or switch to `set -uo pipefail` as in `demo.sh:13` — planner picks; `ci-check.sh` uses `set -e` and that's sufficient for fail-fast).

**Pass/fail/skip/warn counter pattern** (from `scripts/ci-check.sh:22-50`):

```bash
PASS=0
FAIL=0
SKIP=0
WARN=0

step() {
    echo
    echo "■ $1"
}

pass() {
    echo "  ✔ $1"
    PASS=$((PASS + 1))
}

fail() {
    echo "  ✖ $1"
    FAIL=$((FAIL + 1))
}

skip() {
    echo "  ⚠ $1 (skipped)"
    SKIP=$((SKIP + 1))
}

warn() {
    echo "  ⚠ $1 (warning)"
    WARN=$((WARN + 1))
}
```

**Copy verbatim** — matches project CLAUDE.md "No emoji in code: use UTF-8 symbols for terminal output: ✔ ✖ ⚠ ● ♪ ■".

**Gate step pattern** (from `scripts/ci-check.sh:175-215` — the Step 6 "Build and test" block the D-01 matrix maps onto):

```bash
# ── Step 6: Build + test ───────────────────────────────────────────
step "Step 6: Build and test"

# Build workspace
cargo build --workspace --bins --no-default-features

# Workspace tests (no default features)
if cargo test --workspace --no-default-features --locked; then
    pass "workspace tests (no default features)"
else
    fail "workspace tests"
fi

# Core tests with all non-yubikey features
CORE_FEATURES="git-attestation,keyring,insecure-tls"
$HAS_ALSA && CORE_FEATURES="audio,$CORE_FEATURES"
if cargo test -p sealedge-core --features "$CORE_FEATURES" --locked; then
    pass "sealedge-core tests ($CORE_FEATURES)"
else
    fail "sealedge-core tests ($CORE_FEATURES)"
fi

# YubiKey simulation tests (unit tests only, no hardware)
if $HAS_PCSC; then
    if cargo test -p sealedge-core --features yubikey --lib --locked; then
        pass "yubikey simulation tests"
    else
        fail "yubikey simulation tests"
    fi
else
    skip "PCSC not available"
fi

# Platform feature tests
if cargo test -p sealedge-platform --lib --locked && \
   cargo test -p sealedge-platform --test verify_integration --locked && \
   cargo test -p sealedge-platform --test verify_integration --features http --locked; then
    pass "sealedge-platform tests"
else
    fail "sealedge-platform tests"
fi
```

**Action:** `validate-v6.sh` can either (a) shell-out to `./scripts/ci-check.sh --clean` (single delegated call — cheapest), or (b) copy the test-matrix block above verbatim and add post-gates (WASM, dashboard, docker). **Recommendation: hybrid** — have `validate-v6.sh` shell out to `ci-check.sh` for Steps 1-12, then append four new gate-blocks (WASM, dashboard, docker-compose + `/healthz`, docker demo roundtrip). This keeps single-source-of-truth with the CI-parity script and avoids drift.

**WASM gate pattern** (mirrors `ci-check.sh:217-228` + CONTEXT.md §Specifics `wasm-tests.yml` parity):

```bash
# ── Step N: WASM build + size check ────────────────────────────────
step "Step N: WASM build + size check"
if rustup target list --installed | grep -q wasm32-unknown-unknown; then
    if cargo check -p sealedge-wasm --target wasm32-unknown-unknown && \
       cargo check -p sealedge-seal-wasm --target wasm32-unknown-unknown; then
        pass "WASM cargo check"
    else
        fail "WASM cargo check"
    fi
    if command -v wasm-pack &> /dev/null; then
        (cd crates/wasm && wasm-pack build --target web --release) && \
        (cd crates/seal-wasm && wasm-pack build --target web --release)
        if [ $? -eq 0 ]; then
            SEALEDGE_WASM_SIZE=$(wc -c < crates/wasm/pkg/sealedge_wasm_bg.wasm 2>/dev/null || echo 0)
            SEAL_WASM_SIZE=$(wc -c < crates/seal-wasm/pkg/sealedge_seal_wasm_bg.wasm 2>/dev/null || echo 0)
            echo "  sealedge-wasm: $SEALEDGE_WASM_SIZE bytes"
            echo "  sealedge-seal-wasm: $SEAL_WASM_SIZE bytes"
            if [ "$SEALEDGE_WASM_SIZE" -gt 2097152 ]; then
                fail "sealedge-wasm exceeds 2 MB limit (wasm-tests.yml parity)"
            else
                pass "WASM size check"
            fi
        else
            fail "wasm-pack build"
        fi
    else
        skip "wasm-pack not installed"
    fi
else
    skip "wasm32-unknown-unknown target not installed"
fi
```

(CONTEXT.md §Specifics gives the size threshold: `sealedge-wasm < 2MB`; `sealedge-seal-wasm` record-only.)

**Docker stack gate pattern** (CONTEXT.md §Specifics verbatim; mirrors `scripts/demo.sh:49` auto-detect pattern):

```bash
# ── Step N+1: Docker stack + demo roundtrip ────────────────────────
step "Step N+1: Docker compose + demo end-to-end"
if command -v docker &> /dev/null && docker compose version &> /dev/null; then
    docker compose -f deploy/docker-compose.yml up --build -d

    # Wait for platform health (from CONTEXT.md §Specifics)
    RETRIES=30
    until curl -sf http://localhost:3001/healthz > /dev/null; do
        sleep 2
        RETRIES=$((RETRIES - 1))
        if [ $RETRIES -le 0 ]; then
            fail "platform /healthz not responding after 60s"
            docker compose -f deploy/docker-compose.yml down
            exit 1
        fi
    done
    pass "docker stack healthy"

    # Run demo (auto-detects docker mode since localhost:3001 is up — demo.sh:49)
    if ./scripts/demo.sh; then
        pass "demo roundtrip"
    else
        fail "demo roundtrip"
    fi

    docker compose -f deploy/docker-compose.yml down
else
    skip "docker not available"
fi
```

**Summary block pattern** (from `scripts/ci-check.sh:317-328`):

```bash
# ── Summary ─────────────────────────────────────────────────────────
echo
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  Results: $PASS passed, $FAIL failed, $WARN warnings, $SKIP skipped"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

if [ $FAIL -gt 0 ]; then
    echo "  ✖ Fix failures before tagging v6.0.0."
    exit 1
else
    echo "  ✔ All v6.0 validation gates passed. Safe to cut v6.0.0 tag."
fi
```

**D-02 test-count floor enforcement (≥471):** Add a final test-count-extraction pass. Parse `cargo test` output across all six D-01 commands, sum the `test result: ok. N passed;` lines, and fail if total < 471. Suggestion (after summary block):

```bash
# D-02 floor enforcement — validate-v6.log captures full cargo test output
TOTAL_TESTS=$(grep -oE 'test result: ok\. [0-9]+ passed' validate-v6.log 2>/dev/null | awk '{s+=$4} END {print s+0}')
echo "  Total green tests: $TOTAL_TESTS (floor: 471)"
if [ "$TOTAL_TESTS" -lt 471 ]; then
    echo "  ✖ Test count below v6.0 floor — record justification in 89-VERIFICATION.md before proceeding."
    exit 1
fi
```

**Log-capture pattern** (CONTEXT.md §D-03 "Output log saved as `validate-v6.log` and excerpted into 89-VERIFICATION.md"):

Planner decides whether tee-ing happens inside the script or at call-site. Recommendation: script writes to `$PWD/validate-v6.log` via `exec > >(tee validate-v6.log) 2>&1` at the top of the script, so every subsequent echo/command output lands in the log automatically. Matches the pattern used in `scripts/pre-commit.sh` in spirit (single-source capture).

**Copyright header requirement (CLAUDE.md):** All `.yml`/`.rs`/`.sh` source files must carry the MPL-2.0 header. `scripts/ci-check.sh:2-4` shows the form; `validate-v6.sh` copies it verbatim.

**Acceptance-criteria shape:**

```bash
test -x scripts/validate-v6.sh  # executable bit set
./scripts/ci-check.sh --clean  # Step 1 copyright-headers check — validate-v6.sh included in the check
bash -n scripts/validate-v6.sh  # syntax-valid
./scripts/validate-v6.sh        # returns 0 on a green tree; writes validate-v6.log
grep -c 'test result: ok' validate-v6.log  # records N matrix-test-block ok lines
```

---

### File 2 — `.planning/phases/89-final-validation/89-VERIFICATION.md` (new; D-15)

**Analog:** `88-VERIFICATION.md` (most recent, same-milestone) + `87-VERIFICATION.md` (one earlier, same-milestone). Both demonstrate the "section per requirement" + "command / output / verdict" evidence-row pattern referenced in CONTEXT.md §D-15.

**Top-of-file header pattern** (from `88-VERIFICATION.md:1-12`):

```markdown
<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: sealedge — Privacy and trust at the edge.
-->

# Phase 89 Verification — Final Validation

**Phase:** 89-final-validation
**Requirements:** VALID-01, VALID-02, VALID-03
**Date:** 2026-04-21
**Status:** PASS — <one-line summary once gates are green; fail-fast otherwise>

---
```

**Copy verbatim** with requirement IDs adjusted.

**Section-per-requirement pattern** (from `88-VERIFICATION.md:15-30`, the "Pre-Rename Gate" evidence-table shape):

```markdown
## 1. VALID-01 — Local CI-parity Matrix

All D-01 matrix commands verified green via `scripts/validate-v6.sh` pre-tag.

| # | Command | Exit code | Test count | Evidence |
|---|---------|-----------|------------|----------|
| 1 | `cargo test --workspace --no-default-features --locked` | 0 | N | validate-v6.log excerpt |
| 2 | `cargo test -p sealedge-core --features "audio,git-attestation,keyring,insecure-tls" --locked` | 0 | N | validate-v6.log excerpt |
| 3 | `cargo test -p sealedge-core --features yubikey --lib --locked` | 0 | N | validate-v6.log excerpt |
| 4 | `cargo test -p sealedge-platform --lib --locked` | 0 | N | validate-v6.log excerpt |
| 5 | `cargo test -p sealedge-platform --test verify_integration --locked` | 0 | N | validate-v6.log excerpt |
| 6 | `cargo test -p sealedge-platform --test verify_integration --features http --locked` | 0 | N | validate-v6.log excerpt |

**Total green:** N (D-02 floor: ≥471; justify if lower — D-02)

**Log excerpt:** see `validate-v6.log` (committed alongside this VERIFICATION.md OR referenced by commit SHA).
```

**Command-and-output evidence-row pattern** (from `88-VERIFICATION.md:54-68` — the `curl -I` redirect-check block):

```markdown
## 2. VALID-02 — GitHub Actions green

### 2.1 Post-rename main CI run (reused from 87-VERIFICATION.md)

See `87-VERIFICATION.md` §D-13 §3 — the `ci.yml` run on `d0968b8` post-rename was green. Not re-triggered (D-04).

**Run URL:** https://github.com/TrustEdge-Labs/sealedge/actions/runs/24724694867

### 2.2 `workflow_dispatch` runs on `wasm-tests.yml` + `semver.yml` (D-05)

**Command:**

\`\`\`
gh workflow run wasm-tests.yml --ref main
gh workflow run semver.yml --ref main
gh run list --workflow=wasm-tests.yml --limit=1
gh run list --workflow=semver.yml --limit=1
\`\`\`

**wasm-tests.yml URL:** <captured>
**wasm-tests.yml conclusion:** success
**semver.yml URL:** <captured>
**semver.yml conclusion:** success

### 2.3 Tag-push `v6.0.0` CI run (D-04 authoritative gate)

**Command (post-tag-push):**

\`\`\`
gh run watch $(gh run list --workflow=ci.yml --event=push --limit=1 --json databaseId -q '.[0].databaseId')
\`\`\`

**Run URL:** <captured>
**Conclusion:** success
**Self-attestation job conclusion:** <captured — includes dogfood of sealedge-attest-sbom-action@v2>
**Release assets uploaded:** `seal`, `seal.sha256`, `seal-sbom.cdx.json`, `build.pub`, `seal.se-attestation.json`
```

**ROADMAP success-criteria closing table pattern** (from `88-VERIFICATION.md:287-296`):

```markdown
## N. ROADMAP §Phase 89 Success Criteria

| # | Criterion | Status |
|---|-----------|--------|
| 1 | `cargo test --workspace` passes with 471+ tests under new crate/binary/constant names | ✔ PASS — §1 |
| 2 | Feature-matrix tests pass for yubikey, http, postgres, ca, openapi combinations | ✔ PASS — §1 (postgres via docker compose per D-13 out-of-scope carve-out; openapi exercised via build) |
| 3 | All GitHub Actions workflows run green on push to renamed repo | ✔ PASS — §2 (main CI + tag-push CI + wasm-tests + semver) |
| 4 | WASM builds, web/dashboard builds + typegen, docker compose stack + demo script all green under new names | ✔ PASS — §3 (WASM), §4 (dashboard), §5 (docker) |
```

**Rollback-status pattern** (from `87-VERIFICATION.md:181-190`):

For Phase 89 the "rollback" concept is the D-06 tag-failure recovery (fix-and-force-update vs cut `v6.0.1`). Include a section:

```markdown
## N+1. Tag-Failure Recovery Status

**Not executed.** All D-06 gates (1) local validate green, (2) main CI green, (3) workflow_dispatch runs green, (4) tag cut, (5) tag-push CI green — all passed. No force-update, no `v6.0.1` needed.

**Recovery command (documented verbatim per D-06 / Phase 87 D-15 pattern):**

\`\`\`
# If tag-push run fails: fix inline, then either:
# Option A — force-update (if no consumers downloaded yet):
git tag -f v6.0.0 <new-sha>
git push origin v6.0.0 --force

# Option B — cut v6.0.1 (audit-cleaner):
git tag -a v6.0.1 -m "v6.0 Sealedge Rebrand — fix applied"
git push origin v6.0.1
\`\`\`

Solo-dev context, no production consumers in the bootstrap window — force-update is acceptable per CONTEXT.md §Claude's-Discretion.
```

**Verifier-appended section pattern** (from `88-VERIFICATION.md:417+`, the `## Phase Goal Verification (verifier agent)` block):

Phase 89's verifier appends its independent-scoring section at the bottom of the same `89-VERIFICATION.md` file — the executor does NOT write it. Planner does NOT include a verifier section in their authored-during-execution draft; the verifier agent appends `_Verifier-section appended: <date>_` trailer during phase-close.

**Acceptance-criteria shape:**

```bash
test -f .planning/phases/89-final-validation/89-VERIFICATION.md
# Structure-level checks:
grep -c '^## ' .planning/phases/89-final-validation/89-VERIFICATION.md  # >= 5 sections (3 VALID + rollback + ROADMAP table)
grep -c 'VALID-01\|VALID-02\|VALID-03' .planning/phases/89-final-validation/89-VERIFICATION.md  # >= 3
grep -c 'PASS\|PASS —' .planning/phases/89-final-validation/89-VERIFICATION.md  # >= 3 (one per VALID-XX)
# Evidence-row checks:
grep -c 'Run URL:\|Command:\|\*\*Output:\*\*' .planning/phases/89-final-validation/89-VERIFICATION.md  # >= 10 evidence markers
```

---

### File 3 — `RELEASE-NOTES-v6.0.0.md` (new; D-09)

**Location decision (CONTEXT.md §Claude's-Discretion last bullet):** Planner picks between `.planning/phases/89-final-validation/RELEASE-NOTES-v6.0.0.md` (phase-local; cleaner planning history — Claude's phrasing prefers this) OR repo-root `RELEASE-NOTES-v6.0.0.md` (ships with the tag). **Recommendation: phase-local**, because (a) it's a one-shot artifact consumed by `gh release create --notes-file`, (b) repo-root is cluttered (CHANGELOG.md already exists there — see "Shared Patterns" below for the relationship), and (c) CONTEXT.md §Specifics `--notes-file` command already references phase-local path.

**Analog:** `MIGRATION.md` v6.0 header block (`MIGRATION.md:12-46`) + `CHANGELOG.md` v5.0/v4.0 entries (`CHANGELOG.md:17-60`). The release notes are a short announcement + link-out, not a full migration table.

**Content shape (CONTEXT.md §Specifics verbatim sketch — planner refines; ~10 lines body):**

```markdown
# v6.0.0 — Sealedge Rebrand

Trademark-driven rename from "trustedge" to "sealedge" — clean break,
no backward-compat decrypt path.

## Breaking changes
- Repo: TrustEdge-Labs/trustedge → TrustEdge-Labs/sealedge (GitHub 301 redirect in place)
- Crates: trustedge-* → sealedge-* (workspace + 2 experimental crates)
- Binaries: trst → seal, trustedge → sealedge, trustedge-server → sealedge-server, etc.
- Crypto constants: TRUSTEDGE-KEY-V1 → SEALEDGE-KEY-V1, TRUSTEDGE_ENVELOPE_V1 → SEALEDGE_ENVELOPE_V1
- File extensions: .trst → .seal, .te-attestation.json → .se-attestation.json
- Env vars: TRUSTEDGE_* → SEALEDGE_*
- GitHub Action: TrustEdge-Labs/attest-sbom-action@v1 → TrustEdge-Labs/sealedge-attest-sbom-action@v2

TrustEdge Labs (the company brand) is unchanged. trustedgelabs.com domain unchanged.

See [MIGRATION.md](MIGRATION.md) for full upgrade guidance.
```

**Key alignments planner must verify against `MIGRATION.md`:**

The breaking-change bullets above MUST match the `MIGRATION.md:17-40` "What renamed" table. Specifically:

| Release-notes bullet | MIGRATION.md table row(s) | Status |
|---|---|---|
| Repo rename | row 20 (`TrustEdge-Labs/trustedge → TrustEdge-Labs/sealedge`) | Present |
| Crate prefix | row 1 (`trustedge-*` → `sealedge-*`) | Present |
| Binaries | rows 2-8 (main CLI, archive CLI, inspector, server, client, platform-server) | Present — planner must pick whether to enumerate every binary in the release notes or use "etc." (CONTEXT.md §Specifics uses "etc." — keeps body short per D-09 10-line goal) |
| Crypto constants | rows 10-14 (header, envelope, chunk-key, session-key, genesis, manifest) | Present |
| File extensions | rows 9, 17 (.trst → .seal, .te-attestation.json → .se-attestation.json) | Present |
| Env vars | row 9 (`TRUSTEDGE_*` → `SEALEDGE_*`) | Present |
| GitHub Action | **not in MIGRATION.md table** — this is Phase 88 cross-repo scope | Planner's release notes should include it (external-visible change), but MIGRATION.md's body doesn't currently enumerate it — may need a trivial addition to MIGRATION.md's table OR a section under "§3.2 Action references" (planner decides whether this counts as a hybrid-gate D-14 inline fix) |

**Copyright header (CLAUDE.md):** Markdown files in this repo use the HTML-comment form (`<!-- Copyright ... -->`); see `MIGRATION.md:1-6` and `CHANGELOG.md:1-6`. Apply to the release-notes file.

**Acceptance-criteria shape:**

```bash
test -f .planning/phases/89-final-validation/RELEASE-NOTES-v6.0.0.md
wc -l .planning/phases/89-final-validation/RELEASE-NOTES-v6.0.0.md  # ~15-25 lines total (including header + blank lines)
grep -c 'MIGRATION.md' .planning/phases/89-final-validation/RELEASE-NOTES-v6.0.0.md  # >= 1 (link present)
grep -c 'Breaking changes\|## Breaking' .planning/phases/89-final-validation/RELEASE-NOTES-v6.0.0.md  # >= 1
# D-09 content parity with MIGRATION.md:
grep -c 'sealedge-attest-sbom-action' .planning/phases/89-final-validation/RELEASE-NOTES-v6.0.0.md  # >= 1 (Phase 88 external action)
# File can be fed to gh release create --notes-file:
gh release create v6.0.0 --notes-file .planning/phases/89-final-validation/RELEASE-NOTES-v6.0.0.md --title "v6.0.0 — Sealedge Rebrand" --draft
# (draft flag prevents accidental publish — user removes --draft when cutting)
```

---

### File 4 — `.github/workflows/ci.yml` lines 212/220 (modified; D-10 straggler fix)

**Analog:** Phase 87 Plan 01 Task 1 `cla.yml` line-exact substring swap pattern + Phase 88 Plan 02 ci.yml edit pattern.

**Current state excerpt** (`.github/workflows/ci.yml:211-224`):

```yaml
      - name: Attest SBOM (dogfood sealedge-attest-sbom-action)
        id: attest
        uses: TrustEdge-Labs/sealedge-attest-sbom-action@v2  # TODO: pin to commit SHA after first @v2.0.0 tag (Plan 03)
        with:
          binary: ./target/release/seal
          sbom: seal-sbom.cdx.json

      - name: Upload attestation to release
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        run: |
          gh release upload "${{ github.ref_name }}" \
            "${{ steps.attest.outputs.attestation-path }}" \
            --clobber
```

**Important clarification on CONTEXT.md §D-10 wording:** CONTEXT.md references `seal.te-attestation.json` on "lines 212/220". Re-reading the current `ci.yml:211-224`, the `.te-attestation.json` straggler string is **not in the current self-attestation job body** — the current body uses `${{ steps.attest.outputs.attestation-path }}` (the action's output), which the `sealedge-attest-sbom-action@v2`'s `action.yml:30` declares as `.se-attestation.json` per Phase 84. Planner should **re-grep** before editing:

```bash
grep -n 'te-attestation' .github/workflows/ci.yml
grep -n 'te-attestation' .github/workflows/*.yml
```

If the grep returns zero, the straggler was already fixed in Phase 88 (Plan 02 refactor inlined the action `uses:` call, which eliminates the hardcoded `.te-attestation.json` filename) and CONTEXT.md D-10 is now a no-op for ci.yml specifically. In that case, the D-10 sweep's real target is the **repo-wide grep-audit** (Shared.3 below) and `ci.yml` lines 212/220 is a stale CONTEXT reference that should be noted in the plan as "verified clean, not modified" (hybrid-gate discovery — Phase 85/86 D-14 pattern).

If the grep returns hits, the edit is a line-local substring swap:

| From | To |
|---|---|
| `seal.te-attestation.json` | `seal.se-attestation.json` |

**Edit discipline (Phase 87 Plan 01 Task 1 verbatim carry-forward):**

> "Use the Edit tool for each file (preferred over `sed` for deterministic outcomes)... Keep line numbering intact — the edits are pure substring swaps, no line insertions or deletions. Do NOT reflow YAML whitespace."

**Commit message convention (Phase 88 Plan 02 carry-forward):** `fix(89): ci.yml — fix stale seal.te-attestation.json → seal.se-attestation.json straggler` with Co-Authored-By trailer.

**Acceptance-criteria shape:**

```bash
# Before fix:
grep -cE 'te-attestation|\.te-' .github/workflows/ci.yml  # returns 2 (lines 212, 220) if CONTEXT is correct; 0 if already fixed
# After fix:
grep -cE 'te-attestation|\.te-' .github/workflows/ci.yml  # returns 0
# Line count unchanged:
wc -l .github/workflows/ci.yml  # same as pre-fix
# YAML still parses:
python3 -c "import yaml; yaml.safe_load(open('.github/workflows/ci.yml'))"  # no exception
```

---

### File 5 — `.planning/ROADMAP.md` (modified; D-16)

**Analog:** v4.0 milestone-close commit `dc0652e` (chore: complete v4.0 milestone) + Phase 88 ROADMAP update (commit pattern `docs(88-XX): phase verification appended` + its predecessor that marked Phase 88 `[x]` in the Progress table).

**Three touchpoints to edit** (same structure as prior milestone closes):

**Touchpoint 1 — `Milestones` header list** (`.planning/ROADMAP.md:34`):

From:
```markdown
- 📋 **v6.0 Sealedge Rebrand** - Phases 83-89 (planned)
```

To:
```markdown
- ✅ **v6.0 Sealedge Rebrand** - Phases 83-89 (shipped 2026-04-21)
```

**Touchpoint 2 — `Phases` section** (`.planning/ROADMAP.md:54`):

From:
```markdown
### 📋 v6.0 Sealedge Rebrand (Planned)
```

To:
```markdown
### ✅ v6.0 Sealedge Rebrand (Shipped)
```

And the Phase 89 line (`.planning/ROADMAP.md:64`):

From:
```markdown
- [ ] **Phase 89: Final Validation** - Full workspace test suite, all GitHub Actions workflows, WASM + dashboard + Docker stack all green end-to-end under the new names
```

To:
```markdown
- [x] **Phase 89: Final Validation** - Full workspace test suite, all GitHub Actions workflows, WASM + dashboard + Docker stack all green end-to-end under the new names (completed 2026-04-21 — N plans)
```

(Replace `N plans` with actual plan count after plan authoring completes — planner pulls from `ls .planning/phases/89-final-validation/89-*-PLAN.md | wc -l`.)

**Touchpoint 3 — Progress table row** (`.planning/ROADMAP.md:267`):

From:
```markdown
| 89. Final Validation | v6.0 | 0/? | Not started | - |
```

To:
```markdown
| 89. Final Validation | v6.0 | N/N | Complete    | 2026-04-21 |
```

(Matches the exact column alignment of Phase 87/88 rows in the same table.)

**Commit pattern** (Phase 87/88 carry-forward + v4.0 `dc0652e` style):

```
git add .planning/ROADMAP.md
git commit -m "$(cat <<'EOF'
docs(89): mark Phase 89 complete + v6.0 milestone shipped

All VALID-01/02/03 gates verified green per 89-VERIFICATION.md.
v6.0 Sealedge Rebrand shipped 2026-04-21 — 7 phases, N plans.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

**Acceptance-criteria shape:**

```bash
grep -c '✅ \*\*v6.0 Sealedge Rebrand' .planning/ROADMAP.md  # >= 2 (Milestones list + Phases section)
grep -c '📋 \*\*v6.0' .planning/ROADMAP.md  # 0 (old planned marker gone)
grep -c '- \[x\] \*\*Phase 89' .planning/ROADMAP.md  # >= 1
grep -cE '89\. Final Validation.*v6\.0.*Complete' .planning/ROADMAP.md  # >= 1 (Progress table row)
```

---

### File 6 — `.planning/PROJECT.md` (modified; D-16)

**Analog:** v4.0 milestone-close commit `dc0652e` PROJECT.md diff (verbatim pattern — see git history).

**Three touchpoints to edit** (same structure as v4.0 close):

**Touchpoint 1 — `Current State` paragraph** (`.planning/PROJECT.md:21`):

From:
```markdown
Shipped v5.0 Portfolio Polish (partial — 2 of 4 phases). Self-attestation wired into CI, GitHub Action published to Marketplace. Phases 81-82 (demo GIF, landing page) punted to post-rename roadmap. Starting v6.0 Sealedge Rebrand — trademark-driven rename from "trustedge" to "sealedge" end-to-end. 471 tests across 9 workspace crates.
```

To (planner drafts the replacement; shape matches v4.0 close — past-tense "Shipped"):
```markdown
Shipped v6.0 Sealedge Rebrand — trademark-driven rename from "trustedge" to "sealedge" end-to-end. Repo, crates, binaries, crypto constants, file extensions, env vars, GitHub Action all renamed; TrustEdge-Labs org/brand unchanged. Clean break with no backward-compat decrypt path. N+ tests across 9 workspace crates (post-rename floor ≥471 per D-02). Phases 81-82 (demo GIF, landing page) remain punted.
```

**Touchpoint 2 — `Current Milestone: v6.0` section header** (`.planning/PROJECT.md:23`):

From:
```markdown
## Current Milestone: v6.0 Sealedge Rebrand
```

To (matches v4.0 close — delete the "Current Milestone" section entirely and replace `Active` with "No active requirements"):

Per v4.0 `dc0652e` pattern, the `## Current Milestone: v6.0 Sealedge Rebrand` header and its entire goal/target-features block move to the "Completed Milestones" summary list at bottom. The spot is replaced with a compact next-slot placeholder.

**Touchpoint 3 — `Active` section** (`.planning/PROJECT.md:238-250`):

From (verbatim current):
```markdown
### Active

**Goal:** Rename the product from "trustedge" to "sealedge" end-to-end — clean break with no legacy compatibility path. TrustEdge-Labs org/brand retains its identity.

**Target features:**
- Rename GitHub monorepo (`trustedge` → `sealedge`) and all workspace crates (`trustedge-*` → `sealedge-*`)
- Rename all CLI binaries including `trst` (pick new short name)
- Replace crypto constants (`TRUSTEDGE-KEY-V1`, `TRUSTEDGE_ENVELOPE_V1`) with sealedge equivalents — clean break
- Sweep code: copyright headers, error messages, log output, CLI help, env vars, comments
- Sweep docs: README, CLAUDE.md, docs/, scripts, examples
- Republish GitHub Action under sealedge name; deprecate old marketplace listing
- Update product references on trustedgelabs.com to advertise Sealedge
- Validate tests, CI, WASM, Docker all green under new names
```

To (v4.0 close pattern — `dc0652e` diff shows this exact shape):
```markdown
### Active

(No active requirements — v6.0 milestone complete, next milestone will define new requirements via `/gsd:new-milestone`)
```

**Touchpoint 4 — `Completed Milestones` append** (`.planning/PROJECT.md:292-293`):

After the existing v5.0 bullet, append:
```markdown
- **v6.0 Sealedge Rebrand** — Trademark-driven rename from "trustedge" to "sealedge" end-to-end. 7 phases (83-89), N plans. Repo/crates/binaries/constants/extensions/env-vars all renamed; TrustEdge-Labs org/brand unchanged; GitHub Action renamed and re-tagged @v2; `sealedge-attest-sbom-action@v2` dogfooded in CI self-attestation; tests floor ≥471 maintained
```

**Touchpoint 5 — Footer timestamp** (`.planning/PROJECT.md:443`):

From:
```markdown
*Last updated: 2026-04-18 — v5.0 archived (partial), v6.0 Sealedge Rebrand started*
```

To:
```markdown
*Last updated: 2026-04-21 — v6.0 Sealedge Rebrand shipped*
```

**Context block updates (optional per D-16):** The `## Context` block (`.planning/PROJECT.md:295-323`) references `trst` binary commands (line 300), `TRUSTEDGE-KEY-V1` (line 301), `.te-attestation.json` contexts, `TrustEdge-Labs/trustedge` URLs (line 5), etc. **The v6.0 rebrand phases 83-88 already swept these in the body of PROJECT.md's Validated list** — but the Context prose block at the bottom may retain a few trustedge/trst references that were part of the "current working state" language. Planner should run a targeted grep:

```bash
grep -nE 'trustedge|trst\b|TRUSTEDGE|\.trst\b|\.te-attestation' .planning/PROJECT.md
```

and sweep any remaining hits in this same milestone-close commit (hybrid-gate D-14 inline), **except** for historical references that preserve audit trail (e.g., "— v2.2", "— v4.0 Phase 78" footnotes that describe what shipped in those versions — those should stay, mirrors Phase 86 D-03 MIGRATION.md pattern).

**Commit pattern** (v4.0 `dc0652e` carry-forward):

```
git add .planning/PROJECT.md
git commit -m "$(cat <<'EOF'
chore(89): flip PROJECT.md to Shipped v6.0 Sealedge Rebrand

Current State paragraph, Active section ("no active requirements"),
Completed Milestones v6.0 bullet, and footer timestamp all updated.
Matches v4.0 milestone-close pattern (dc0652e).

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

**Acceptance-criteria shape:**

```bash
grep -c 'Shipped v6.0 Sealedge Rebrand' .planning/PROJECT.md  # >= 1 (Current State)
grep -c 'Current Milestone: v6.0' .planning/PROJECT.md  # 0 (header moved/deleted)
grep -c '(No active requirements' .planning/PROJECT.md  # >= 1
grep -c '- \*\*v6.0 Sealedge Rebrand\*\*' .planning/PROJECT.md  # >= 1 (Completed Milestones bullet)
grep -c '2026-04-21 — v6.0 Sealedge Rebrand shipped' .planning/PROJECT.md  # >= 1 (footer)
```

---

### File 7 (conditional) — `MIGRATION.md` link-target verification (D-09 read-only check)

**Status:** read-only verification, not an edit (unless the D-14 hybrid gate triggers).

**Rationale:** The release notes (File 3) link to `MIGRATION.md` as the authoritative upgrade guide. Phase 89 must confirm that every bullet in RELEASE-NOTES-v6.0.0.md §"Breaking changes" is covered by a row in `MIGRATION.md`'s "What renamed" table OR has a corresponding prose section in MIGRATION.md.

**Verification command:**

```bash
# Every release-notes breaking-change keyword must appear in MIGRATION.md:
for key in 'TrustEdge-Labs/sealedge' 'sealedge-\*' 'trst → seal' \
           'SEALEDGE-KEY-V1' 'SEALEDGE_ENVELOPE_V1' '.seal' '.se-attestation.json' \
           'SEALEDGE_\*' 'sealedge-attest-sbom-action@v2'; do
    grep -q "$key" MIGRATION.md || echo "MISSING in MIGRATION.md: $key"
done
```

**Expected:** zero "MISSING" lines. If any surface, hybrid-gate D-14 kicks in — planner adds the missing row/section to MIGRATION.md in the same commit as the release-notes file creation (atomic commit discipline, Phase 85/86 carry-forward).

**Known gap from analog analysis (above, File 3):** `sealedge-attest-sbom-action@v2` is **not currently in MIGRATION.md's "What renamed" table**. Planner should pre-emptively add it as a row (or add a brief §"Action references" section) during Phase 89 execution. This is a trivial rebrand-side-effect edit that fits the D-14 hybrid gate.

---

## Shared Patterns

These cross-cutting patterns apply across all Phase 89 plans and match the established discipline from Phases 83-88.

### Shared.1 — User-driven external `git tag` / `git push --tags` (Phase 87 D-05 + Phase 88 Plan 03 carry-forward)

**Source:** `.planning/phases/87-github-repository-rename/87-02-PLAN.md` Task 2-3 + `.planning/phases/88-external-action-product-website/88-03-PLAN.md` user-runs-it pattern.

**Apply to:** Plan (c) — the v6.0.0 tag cut and release creation (D-06, D-08).

**Exact commands (CONTEXT.md §Specifics verbatim — user runs from their shell):**

```bash
# Pre-tag gate (D-06 — Claude confirms all 6 items green before user runs tag commands):
git status                                                           # 1. clean on main
git log origin/main..HEAD                                            # 1. empty (main pushed)
./scripts/validate-v6.sh                                             # 2. exit code 0
gh run list --workflow=ci.yml --branch main --limit 1                # 3. latest run conclusion=success
gh run list --workflow=wasm-tests.yml --limit 1                      # 4. workflow_dispatch conclusion=success
gh run list --workflow=semver.yml --limit 1                          # 4. workflow_dispatch conclusion=success
git ls-files | xargs grep -nE 'trustedge|TRUSTEDGE|\.trst|\.te-attestation' 2>/dev/null | \
    grep -vE 'TrustEdge-Labs|TrustEdge Labs|MIGRATION\.md|CHANGELOG\.md|^\.planning/(milestones|phases)/|RFC_K256_SUPPORT\.md|improvement-plan\.md|security-review-platform\.md'
                                                                     # 5. zero hits (D-10)
gh pr list --state open                                              # 6. empty

# The tag cut (user runs from their shell):
git tag -a v6.0.0 -m "v6.0 Sealedge Rebrand"
git push origin v6.0.0

# Release creation (user runs):
gh release create v6.0.0 \
  --title "v6.0.0 — Sealedge Rebrand" \
  --notes-file .planning/phases/89-final-validation/RELEASE-NOTES-v6.0.0.md

# Post-tag-push CI watch (user or Claude captures URL):
gh run watch $(gh run list --workflow=ci.yml --event=push --limit=1 --json databaseId -q '.[0].databaseId')
```

**Discipline (Phase 87/88 carry-forward):**
- These are `checkpoint:human-action gate="blocking"` tasks in the plan, not shell commands Claude invokes.
- Plan provides exact command strings + resume-signal protocol ("reply with `tag-cut` after tag is pushed").
- Evidence captured in `89-VERIFICATION.md` §2.3 via paste-the-output workflow.
- Rollback discipline (D-06 / CONTEXT.md §Claude's-Discretion): if tag-push CI fails, user decides force-update vs `v6.0.1` cut; Claude provides both command sequences.

### Shared.2 — Atomic commit discipline (Phase 83-88 carry-forward)

**Source:** Phase 87 Plan 01 Task 3 commit-message HEREDOC + Phase 88 Shared.2 pattern.

**Apply to:** Every Phase 89 commit (expected 4-6 commits per the plan-shape in CONTEXT.md §Claude's-Discretion):
1. `fix(89): ci.yml straggler — seal.te-attestation.json → seal.se-attestation.json` (File 4)
2. `feat(89): add scripts/validate-v6.sh CI-parity matrix runner` (File 1)
3. `docs(89): draft 89-VERIFICATION.md VALID-01/02/03 evidence` (File 2)
4. `docs(89): add RELEASE-NOTES-v6.0.0.md body` (File 3)
5. `docs(89): mark Phase 89 complete + v6.0 milestone shipped in ROADMAP.md` (File 5)
6. `chore(89): flip PROJECT.md to Shipped v6.0 Sealedge Rebrand` (File 6)

(Plans (a) / (b) / (c) / (d) may bundle commits 1-2 / 3 / tag-cut+4 / 5-6 — planner decides atomic boundaries.)

**Pattern:** same as Phase 88 Shared.2 — HEREDOC commit message with `type(89): <subject>` convention and `Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>` trailer.

**Commit-message type prefixes** (Phase 83/85/86/87/88 carry-forward):
- `fix(89)`: straggler fixes (ci.yml .te-attestation → .se-attestation)
- `feat(89)`: net-new capabilities (validate-v6.sh script)
- `docs(89)`: documentation / requirement-text / phase-artifact additions (VERIFICATION.md, release notes, ROADMAP)
- `chore(89)`: housekeeping (PROJECT.md milestone-close flip, milestone archival move to `.planning/milestones/v6.0/`)

### Shared.3 — Repo-wide grep-audit with allowlist (Phase 85/86/87/88 carry-forward)

**Source:** Phase 85 D-11/D-12 carry-forward + Phase 86 D-01a allowlist + Phase 87/88 cross-repo extensions.

**Apply to:** D-10 pre-tag sweep (Plan a).

**Exact command (CONTEXT.md §Specifics verbatim):**

```bash
git ls-files \
  | xargs grep -nE "trustedge|TRUSTEDGE|\.trst\b|\.te-attestation" 2>/dev/null \
  | grep -vE "TrustEdge-Labs|TrustEdge Labs|trustedgelabs\.com|MIGRATION\.md|CHANGELOG\.md|^\.planning/(milestones|phases)/|RFC_K256_SUPPORT\.md|improvement-plan\.md|security-review-platform\.md"
```

**Expected:** zero results outside the known `ci.yml` lines 212/220 stragglers (File 4 above fixes those in the same atomic commit). If the D-14 hybrid gate finds additional side-effect stragglers during the matrix run or docker-compose gate, fix inline with `fix(89):` commits (Phase 85/86 pattern).

**Allowlist rationale (preserved — do NOT sweep):**
- `TrustEdge-Labs` (org/brand — Phase 85 D-02)
- `TrustEdge Labs` (company legal entity — Phase 85 D-03)
- `trustedgelabs.com` (company domain — Phase 86 D-09)
- `MIGRATION.md`, `CHANGELOG.md` (historical-artifact hybrid treatment — Phase 86 D-03)
- `.planning/milestones/` + `.planning/phases/` (planning history — Phase 86 D-01a)
- `RFC_K256_SUPPORT.md`, `improvement-plan.md`, `security-review-platform.md` (Phase 86 D-01a historical-artifact carve-out)

### Shared.4 — Verification-file evidence capture (Phase 87/88 carry-forward)

**Source:** `87-VERIFICATION.md` + `88-VERIFICATION.md` — Phase 89's template.

**Apply to:** `89-VERIFICATION.md` (File 2 above).

**Sections to include (Phase 87/88 VERIFICATION.md shape — matches D-15):**

1. **VALID-01 evidence** — Local matrix table (6 rows, one per D-01 command + exit code + test count)
2. **VALID-02 evidence** — Three sub-sections: 2.1 post-rename main CI (reused from 87-VERIFICATION), 2.2 workflow_dispatch runs on wasm-tests+semver, 2.3 tag-push v6.0.0 CI run (authoritative gate including self-attestation job + release assets)
3. **VALID-03 evidence** — WASM sizes, dashboard build+typecheck output, dashboard browser-smoke screenshot, docker stack healthz, demo roundtrip receipt JSON
4. **Tag-failure recovery status** — not executed / executed with which recovery option
5. **ROADMAP §Phase 89 Success Criteria table** — 4-row table (from ROADMAP.md:242-246) with ✔ PASS markers pointing to evidence sections
6. **Deferred Operational Findings** — any hybrid-gate discoveries that didn't block the close (same tone as 87-VERIFICATION.md §"Deferred Operational Findings" and 88-VERIFICATION.md's parallel section)

### Shared.5 — Hybrid gate for rebrand-side-effect fixes (Phase 85/86 D-14 carry-forward)

**Source:** `.planning/phases/85-code-sweep-headers-text-metadata/85-CONTEXT.md` D-14 + Phase 86 D-14 reinforcement + CONTEXT.md D-14 verbatim.

**Apply to:** Any rebrand-side-effect breakage discovered during matrix/WASM/dashboard/docker validation.

**Discipline:**
- Trivial fixes (stale docstring, broken test assert that's clearly a Phase 83-86 carry-forward leftover, copy-paste stragglers the D-10 grep missed): land as atomic `fix(89):` commits in Phase 89 itself.
- Scope-expansion-shaped issues (new test coverage, new feature, refactor): defer to hotfix sub-phase or future milestone.
- Decision boundary: "could a disciplined reviewer reasonably call this a rebrand side-effect?" — if yes, fix; if ambiguous, defer.
- Log in 89-VERIFICATION.md §"Deferred Operational Findings" + reference the deferral rationale.

### Shared.6 — Milestone-close archival (D-16 / `/gsd-complete-milestone` flow)

**Source:** Prior milestone-close commits (`dc0652e` v4.0, `206bffb` v3.0, pattern repeated through v5.0) + existing `.planning/milestones/v1.0-ROADMAP.md` through `v5.0-*` structure.

**Apply to:** Plan (d) — final milestone-close artifacts.

**Archive operation (mirror v4.0 `dc0652e`):**

```bash
# Create v6.0 milestone directory:
mkdir -p .planning/milestones/v6.0-phases

# Move all Phase 83-89 directories:
git mv .planning/phases/83-crate-and-binary-rename .planning/milestones/v6.0-phases/
git mv .planning/phases/84-crypto-constants-file-extension .planning/milestones/v6.0-phases/
git mv .planning/phases/85-code-sweep-headers-text-metadata .planning/milestones/v6.0-phases/
git mv .planning/phases/86-documentation-sweep .planning/milestones/v6.0-phases/
git mv .planning/phases/87-github-repository-rename .planning/milestones/v6.0-phases/
git mv .planning/phases/88-external-action-product-website .planning/milestones/v6.0-phases/
git mv .planning/phases/89-final-validation .planning/milestones/v6.0-phases/

# Move REQUIREMENTS.md + ROADMAP.md to milestone-pinned copies:
git mv .planning/REQUIREMENTS.md .planning/milestones/v6.0-REQUIREMENTS.md
# (ROADMAP.md stays — it's the live roadmap; v6.0 entries remain in it as historical context)

# Commit:
git commit -m "$(cat <<'EOF'
chore: complete v6.0 milestone

Archive v6.0 Sealedge Rebrand: 7 phases (83-89), N plans, N+ tests.
Phase directories moved to .planning/milestones/v6.0-phases/.
REQUIREMENTS.md archived to .planning/milestones/v6.0-REQUIREMENTS.md.
PROJECT.md and ROADMAP.md updated in separate prior commits.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

**Ordering per CONTEXT.md §Claude's-Discretion:** planner decides whether ROADMAP/PROJECT updates land before or after the archival move. Recommendation: **ROADMAP/PROJECT first, archival move last** — matches v4.0 `dc0652e` diff order (PROJECT.md evolved first, then phase-dirs moved).

**Alternative: run `/gsd-complete-milestone` directly** (CONTEXT.md §Claude's-Discretion). Planner chooses at execution time.

---

## No Analog Found

None. **Every file in Phase 89's scope has a clear analog from prior v6.0 phases (85, 86, 87, 88) or the existing `scripts/ci-check.sh` infrastructure.** The phase is a validation + evidence-capture + milestone-close phase — no new construction patterns introduced.

The one nominally-new artifact — `RELEASE-NOTES-v6.0.0.md` — is a first-of-its-kind file in this repo (no prior `RELEASE-NOTES-*.md` exists), but its shape is directly derived from `MIGRATION.md` v6.0 header + `CHANGELOG.md` v5.0/v4.0 entries, both of which establish the "what changed + link-out" rhetorical pattern Phase 89 copies.

---

## Metadata

**Analog search scope:**
- `scripts/ci-check.sh` — **primary analog for `validate-v6.sh`** (12-step structure, PASS/FAIL/SKIP/WARN counters, step() helpers, D-01 matrix already embedded at Step 6)
- `scripts/demo.sh` — secondary analog for the docker-stack auto-detect pattern (lines 44-56 `SERVER_AVAILABLE` gate)
- `.planning/phases/87-github-repository-rename/87-VERIFICATION.md` — VERIFICATION.md template (section-per-requirement, evidence tables, ROADMAP SC closing table, deferred findings)
- `.planning/phases/88-external-action-product-website/88-VERIFICATION.md` — VERIFICATION.md template + evidence-row shape (command/output fenced blocks, verifier-appended trailer)
- `.planning/phases/88-external-action-product-website/88-PATTERNS.md` — prior pattern-map structure (this file's structural parent)
- `.github/workflows/ci.yml` lines 171-224 — self-analog for File 4 straggler fix + the D-04 tag-push CI job the release validates
- `.planning/ROADMAP.md` lines 260-268 — Progress table row pattern for File 5
- `.planning/PROJECT.md` lines 19-25, 238-250, 271-293, 443 — Current-State + Active + Completed-Milestones + footer pattern for File 6
- `MIGRATION.md` §v6.0 (lines 12-46) — link-target + breaking-change table for File 3 + File 7 verification
- `CHANGELOG.md` lines 17-60 — v5.0/v4.0 entry shape as secondary reference for File 3
- git history: `dc0652e` (v4.0 close), `206bffb` (v3.0 close), `c4edc9a` (v2.9 close) — milestone-close commit pattern for File 5/6 + Shared.6

**Files scanned:** ~14 (6 direct analogs + 4 planning-artifact analogs + ~4 via git log history commits)

**Pattern extraction date:** 2026-04-21

**Key insight for planner:** Phase 89 is **not a build phase** — it is a **validation + artifact-capture + milestone-close phase**. Every technique required has been demonstrated successfully in prior v6.0 phases or in the existing `scripts/ci-check.sh`. Plan authors should cite prior-phase decision IDs (D-01-D-15 on CONTEXT.md; D-14 Phase 85/86 hybrid-gate; D-05 Phase 87 external-op) when authoring `<action>` blocks — matches the "concrete, not abstract" quality bar Phase 88 established. The only novel technique is the `validate-v6.sh` script itself, and it's a copy-and-extend of `ci-check.sh` rather than a ground-up new pattern.
