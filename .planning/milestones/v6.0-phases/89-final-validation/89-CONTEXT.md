# Phase 89: Final Validation - Context

**Gathered:** 2026-04-21
**Status:** Ready for planning

<domain>
## Phase Boundary

End-to-end proof that nothing functional regressed during the v6.0 Sealedge rebrand. Phase 89 runs the validation gates (full local feature-matrix tests, all GitHub Actions workflows green, WASM/dashboard/Docker stack + demo end-to-end), cuts the `v6.0.0` release tag once gates pass, captures structured evidence in `89-VERIFICATION.md`, and closes the v6.0 milestone (ROADMAP/PROJECT update + archival to `.planning/milestones/v6.0/`).

**In scope:**
- Local CI-parity full feature matrix run (default, core+audio/git/keyring/tls, yubikey lib, platform lib + verify_integration + http) — must match or exceed 471 tests green
- New `scripts/validate-v6.sh` script that runs the matrix and pipes log output for `89-VERIFICATION.md`
- Final repo-wide grep-audit for stale `trustedge|TRUSTEDGE|.trst|.te-attestation` references using the Phase 85/86 allowlist; fix any live stragglers (including known `seal.te-attestation.json` in `ci.yml` lines 212/220) in a single atomic commit before tagging
- WASM gate: `cargo check` against wasm32 + `wasm-pack build --target web --release` for both `sealedge-wasm` and `sealedge-seal-wasm`; record .wasm file size
- Dashboard gate: `npm ci && npm run build && npm run check` clean, then a manual `npm run dev` browser smoke (title + headings render "Sealedge", one device-list fetch against the platform server). Screenshot in 89-VERIFICATION.md
- Docker stack gate: `docker compose -f deploy/docker-compose.yml up --build` — confirm platform + postgres + dashboard containers healthy; then `./scripts/demo.sh` (auto-detect docker mode), capture verify-receipt response in 89-VERIFICATION.md
- VALID-02 proof: one green push-to-main CI run on `main` (post-rename) AND one green tag-push CI run on `v6.0.0` (exercises release/self-attestation job, dogfoods `sealedge-attest-sbom-action@v2`); `wasm-tests.yml` + `semver.yml` re-confirmed via `workflow_dispatch`. Capture all run URLs
- Cut `v6.0.0` tag (single tag, no floating major/minor) AFTER local matrix is green AND main CI is green; user runs `git tag` + `git push --tags` from their shell (Phase 87/88 user-driven external-op pattern)
- v6.0.0 GitHub release notes: ~10-line short rebrand announcement (what changed: repo name, crate names, binary names, constants, file ext) + link to existing `MIGRATION.md` for details
- 89-VERIFICATION.md as structured evidence table per VALID-01/02/03: command, exit code, test count, CI run URL, release asset list (`seal`, `seal.sha256`, `seal-sbom.cdx.json`, `build.pub`, `.se-attestation.json`), verify-attestation roundtrip output, curl receipt from docker platform, dashboard screenshot
- Milestone-close artifacts: 89-VERIFICATION.md committed, ROADMAP.md updated (Phase 89 [x] with completion date + v6.0 milestone marked ✅ Shipped + final phase count), PROJECT.md flipped to "Shipped v6.0", archive to `.planning/milestones/v6.0/`

**Out of scope (explicit carve-outs):**
- Live `postgres` integration tests beyond the docker compose stack health-check (no separate `cargo test --features postgres` against a live DB) — `postgres` feature gates are exercised via `cargo build`/`cargo check` and the docker stack roundtrip, not via standalone test invocations
- YubiKey hardware integration tests (`yubikey_integration` test) — yubikey feature is exercised via the `--lib` simulation tests in CI; hardware test stays out of scope (matches CI behavior)
- Playwright/headless-browser dashboard test — manual browser smoke only; automated browser tests are post-v6.0 polish
- Floating `v6` and `v6.0` tags — only `v6.0.0` cut, matching prior milestone tag pattern (v2.4, v3.0, v4.0)
- Re-running Phase 87/88 verifications — those are already PASS in their respective `87/88-VERIFICATION.md`; Phase 89 cites them, doesn't re-run
- New features, refactors, or scope expansion — strict rebrand-validation only; trivial inline fixes ARE allowed (hybrid gate per Phase 85/86 pattern), but anything beyond a clear rebrand-side-effect deferred
- Phases 81 (demo GIF) and 82 (landing page) — explicitly punted-then-resumed AFTER Phase 89 closes (per v5.0 partial-shipped state)
- Publishing crates to crates.io — out-of-scope per v6.0 REQUIREMENTS.md

</domain>

<decisions>
## Implementation Decisions

### Test matrix scope and execution (VALID-01)

- **D-01:** **Local full CI-parity matrix.** Run the exact feature combos `ci.yml` runs: `cargo test --workspace --no-default-features --locked` + `cargo test -p sealedge-core --features "audio,git-attestation,keyring,insecure-tls" --locked` + `cargo test -p sealedge-core --features yubikey --lib --locked` + `cargo test -p sealedge-platform --lib --locked` + `cargo test -p sealedge-platform --test verify_integration --locked` + `cargo test -p sealedge-platform --test verify_integration --features http --locked`. No live postgres DB, no YubiKey hardware. Local-CI parity is the proof.
- **D-02:** **Test-count floor: ≥471.** Phase 89 fails if the post-rename total green test count drops below 471 without explicit justification in 89-VERIFICATION.md. Acceptable if count INCREASED (e.g., Phase 84 D-02 clean-break rejection tests added rows). Captured in the validation script's final summary.
- **D-03:** **`scripts/validate-v6.sh` script.** Commit a reusable shell script that runs the D-01 matrix end-to-end with structured output. Each gate produces a green/red row + test count. Final exit code is 0 only if every gate passed. Output log saved as `validate-v6.log` and excerpted into 89-VERIFICATION.md. Reusable for future milestone-close phases.

### GitHub Actions workflow gate (VALID-02)

- **D-04:** **Two-run proof: post-rename green main CI + green tag-push CI on `v6.0.0`.** The post-rename main run is already captured in 87-VERIFICATION.md and is reused (not re-triggered). The tag-push run on `v6.0.0` is new and is the authoritative gate for the release / self-attestation job. Both run URLs captured in 89-VERIFICATION.md.
- **D-05:** **`wasm-tests.yml` + `semver.yml` confirmed via `workflow_dispatch`.** These are not exercised by every push (path-filtered + scheduled). Trigger one manual run of each post-rename, capture URL + green status in 89-VERIFICATION.md.

### v6.0.0 release tag cut

- **D-06:** **Tag cut sequence.** (1) Local validation script green; (2) main CI green; (3) `wasm-tests.yml` + `semver.yml` workflow_dispatch runs green; (4) cut `v6.0.0`; (5) tag-push CI run (including release/self-attestation job dogfooding `sealedge-attest-sbom-action@v2`) green; (6) capture all evidence. Tag is cut BEFORE phase close (not deferred), AFTER local + main-CI gates pass. Fail-fast: if the tag-push run is red, fix inline and force-update the tag (or cut `v6.0.1` if force-update is destructive — planner decides at execution).
- **D-07:** **Tag label: `v6.0.0` only.** Single annotated tag matching prior milestone tag convention (`v2.4`, `v3.0`, `v4.0`). No floating `v6` or `v6.0` tags. Solo-dev, no install-tooling that depends on floating tags.
- **D-08:** **User runs `git tag` + `git push --tags`.** Matches Phase 87 (D-05) and Phase 88 (Plan 03) external-operation pattern. Claude provides exact commands + pre-tag checklist; user executes from their shell. Audit trail in conversation + 89-VERIFICATION.md.
- **D-09:** **Release notes: short rebrand announcement + link to MIGRATION.md.** ~10-line body: "v6.0 Sealedge Rebrand — trademark-driven rename from trustedge to sealedge" + breaking-change summary (repo name, crate names, binary names, constants, file ext) + link to existing `MIGRATION.md`. No inline duplication of the full migration table. Created via `gh release create v6.0.0` + `--notes-file` (planner provides the file in `.planning/phases/89-final-validation/`).

### Final straggler sweep

- **D-10:** **One-shot repo-wide grep-audit + atomic-commit fix BEFORE tagging.** Grep pattern: `trustedge|TRUSTEDGE|\.trst|\.te-attestation`. Allowlist mirrors Phase 85/86: `TrustEdge-Labs` (org/brand), `TrustEdge Labs` (company-brand prose), `MIGRATION.md`, `CHANGELOG.md`, `.planning/milestones/`, `.planning/phases/` (historical project-management artifacts), `RFC_K256_SUPPORT.md`, `improvement-plan.md`, `security-review-platform.md` (Phase 86 D-01a allowlist), comments referencing "the old trustedge name" intentionally for migration docs. Fix any live stragglers in a single atomic commit before cutting the v6.0.0 tag. Known stragglers: `ci.yml` lines 212/220 (`seal.te-attestation.json` → `seal.se-attestation.json`).

### Docker + dashboard + demo depth (VALID-03)

- **D-11:** **WASM gate: cargo check + wasm-pack build for both crates.** `cargo check -p sealedge-wasm --target wasm32-unknown-unknown` + `cargo check -p sealedge-seal-wasm --target wasm32-unknown-unknown` + `wasm-pack build --target web --release` for each. Record .wasm file size in 89-VERIFICATION.md (parity with `wasm-tests.yml` size-check job: `sealedge-wasm` < 2MB, `sealedge-seal-wasm` size noted).
- **D-12:** **Dashboard gate: build + type-gen + manual browser smoke.** `cd web/dashboard && npm ci && npm run build && npm run check` green. Then `npm run dev`, open in browser, confirm: (a) page title and headings render "Sealedge" (no "TrustEdge" product references), (b) one successful device-list fetch against the running platform server. Screenshot home page + device list in 89-VERIFICATION.md.
- **D-13:** **Docker stack gate: full e2e (stack up + demo + receipt).** `docker compose -f deploy/docker-compose.yml up --build` — confirm `platform`, `postgres`, `dashboard` containers reach healthy/running state. Then `./scripts/demo.sh` (auto-detect mode picks docker since `localhost:3001/healthz` is up). Capture verify-receipt response (curl POST to `/v1/verify`) into 89-VERIFICATION.md. Proves archive → verify roundtrip works end-to-end under the new names.

### Failure policy and evidence capture

- **D-14:** **Hybrid gate: fix inline.** Trivial fixes (copy-paste stragglers, stale docstring, a broken test assert that's a clear rebrand-side-effect) land as atomic commits IN Phase 89 itself. Mirrors Phase 85/86 hybrid-treatment pattern. Anything that smells like scope-expansion (new test, new feature, refactor) defers to a hotfix sub-phase or a future milestone.
- **D-15:** **89-VERIFICATION.md = structured evidence table per VALID-01/02/03.** Section per requirement. Each evidence row records: command run, exit code, test count (where applicable), CI run URL, screenshots (dashboard), release asset list (`seal`, `seal.sha256`, `seal-sbom.cdx.json`, `build.pub`, `.se-attestation.json`), verify-attestation roundtrip output, curl receipt from docker platform. Mirrors `87-VERIFICATION.md` / `88-VERIFICATION.md` style.

### Milestone-close artifacts

- **D-16:** **Phase 89 produces all four milestone-close artifacts.** (a) `89-VERIFICATION.md`, (b) ROADMAP.md updated with Phase 89 [x] completion date + v6.0 milestone marked ✅ Shipped + final phase count, (c) PROJECT.md flipped from "Current Milestone: v6.0" to "Shipped v6.0 Sealedge Rebrand" with next-milestone slot prepared, (d) archive v6.0 to `.planning/milestones/v6.0/` (run the `/gsd-complete-milestone` flow OR planner sequences the archive moves manually). Each artifact lands as its own atomic commit at phase close.

### Claude's Discretion

- **Plan granularity.** Likely 3-4 plans:
  - (a) `validate-v6.sh` script + final straggler grep-audit + inline fixes (single atomic-commit set, pre-tag)
  - (b) Local matrix run + WASM/dashboard/docker e2e + 89-VERIFICATION.md draft (evidence capture)
  - (c) `v6.0.0` tag cut + release notes + tag-push CI verification (user-gated)
  - (d) Milestone-close artifacts (ROADMAP/PROJECT/archive)
  - Planner may split or merge as coherent atomic commits allow.
- **Validation script structure.** Planner decides whether `validate-v6.sh` is a single monolithic script or a directory of focused scripts (`validate-tests.sh`, `validate-wasm.sh`, `validate-docker.sh`) with a top-level orchestrator. Either is fine; reusability for future milestones is the constraint.
- **Tag-failure recovery.** If the tag-push CI run fails AFTER tag cut: planner decides between (1) fix-and-force-update `v6.0.0` (clean if no consumers downloaded yet) or (2) leave `v6.0.0` and cut `v6.0.1` (audit-cleaner). User confirms before either.
- **Milestone-close ordering.** Planner sequences whether ROADMAP/PROJECT updates land before or after the archival move. The `/gsd-complete-milestone` flow may dictate this; if planner runs it directly, follow its output.
- **Release-notes file location.** Planner decides whether the `--notes-file` body lives at `.planning/phases/89-final-validation/RELEASE-NOTES-v6.0.0.md` (kept with phase artifacts) or at repo root `RELEASE-NOTES-v6.0.0.md` (released with the tag). Either works; phase-local keeps the planning history clean.

### Folded Todos

None — no backlog todos match this phase (gsd-sdk query returned 0 matches).

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Milestone-level (v6.0 scope)
- `.planning/REQUIREMENTS.md` §Valid — VALID-01 / VALID-02 / VALID-03 (all three require Phase 89 close)
- `.planning/REQUIREMENTS.md` §"Out of Scope" — v6.0 release tag cut under new repo name (informs D-06/D-07)
- `.planning/PROJECT.md` §Current Milestone — v6.0 target, clean-break preference, 471-test count baseline (informs D-02)
- `.planning/ROADMAP.md` §"Phase 89: Final Validation" (line 238-247) — goal + 4 success criteria

### Prior v6.0 phase decisions that carry forward
- `.planning/phases/83-crate-and-binary-rename/83-CONTEXT.md` — `trst` → `seal` binary, `.trst` → `.seal` extension (informs D-10 grep targets)
- `.planning/phases/84-crypto-constants-file-extension/84-CONTEXT.md` — `TRUSTEDGE-KEY-V1` → `SEALEDGE-KEY-V1`, `.te-attestation.json` → `.se-attestation.json` (informs D-10 grep targets + the known `ci.yml` straggler)
- `.planning/phases/85-code-sweep-headers-text-metadata/85-CONTEXT.md` — Phase 85 sweep allowlist pattern (informs D-10 allowlist; "TrustEdge Labs" company-brand stays)
- `.planning/phases/86-documentation-sweep/86-CONTEXT.md` §D-01a — historical-artifact allowlist (RFC_K256_SUPPORT.md, improvement-plan.md, security-review-platform.md, MIGRATION.md, CHANGELOG.md hybrid treatment) — directly drives D-10 allowlist
- `.planning/phases/87-github-repository-rename/87-CONTEXT.md` — user-driven `gh` external-op pattern, post-rename green-CI proof (D-04 reuses 87's main-run as evidence; D-08 reuses 87's user-runs-it pattern)
- `.planning/phases/88-external-action-product-website/88-CONTEXT.md` §D-11/D-12/D-14 — `seal` binary upload + dogfood `sealedge-attest-sbom-action@v2` in self-attest job; first v6.0.0 release run is the implicit smoke-test of `@v2` (D-06 closes this loop)
- `.planning/phases/88-external-action-product-website/88-CONTEXT.md` §Deferred — flagged stale `seal.te-attestation.json` in `ci.yml` as Phase 89 sweep candidate (D-10 picks this up explicitly)

### Memory / user context
- `~/.claude/projects/-home-john-vault-projects-github-com-trustedge/memory/project_v6_rebrand.md` — clean-break philosophy, solo dev, no production users, trademark-driven rename context
- `~/.claude/projects/-home-john-vault-projects-github-com-trustedge/memory/feedback_clean_break_compat.md` — no backward-compat shims; informs validation strictness (no "must also pass under old names" hedging)
- `~/.claude/projects/-home-john-vault-projects-github-com-trustedge/memory/MEMORY.md` — milestone history, pending todos (Phase 81 demo GIF + Phase 82 landing page resume after Phase 89 closes)

### Source files / artifacts touched by Phase 89
- `scripts/validate-v6.sh` — new file (D-03); reusable matrix-runner script
- `scripts/ci-check.sh` — existing CI parity script; `validate-v6.sh` may compose against this
- `.github/workflows/ci.yml` lines 212/220 — known stale `seal.te-attestation.json` straggler (D-10)
- `.github/workflows/ci.yml` (release/self-attest job lines 171-224) — first v6.0.0 tag-push run validates dogfood of `sealedge-attest-sbom-action@v2` (D-04/D-06)
- `.github/workflows/wasm-tests.yml` — re-confirmed via `workflow_dispatch` (D-05)
- `.github/workflows/semver.yml` — re-confirmed via `workflow_dispatch` (D-05)
- `deploy/docker-compose.yml` — Docker stack composition (D-13)
- `web/dashboard/` — npm build + type-gen + manual browser smoke (D-12)
- `crates/wasm/`, `crates/seal-wasm/` — WASM gate targets (D-11)
- `scripts/demo.sh` — full e2e roundtrip (D-13)

### Cross-repo references (read-only this phase)
- `TrustEdge-Labs/sealedge-attest-sbom-action@v2` — the dogfood target; the v6.0.0 tag-push CI run consumes this. Phase 88 already cut + verified `@v2`; Phase 89 doesn't touch the action repo.

### External service docs
- GitHub release notes via `gh release create --notes-file` — preferred over `--notes` for multi-line bodies (D-09)
- `wasm-pack build --target web --release` — produces `pkg/<crate>_bg.wasm`; size measured with `wc -c` (D-11, parity with `wasm-tests.yml`)

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- **Phase 87/88 user-driven external-op pattern** — `gh repo rename` (87), `gh repo rename` for the action (88) — same pattern for `git push --tags v6.0.0` (D-08). Pre-op gate checklist + user-runs-it + Claude captures audit-trail.
- **`scripts/ci-check.sh`** — existing local CI-parity script. `validate-v6.sh` may compose against this OR reproduce its commands; planner decides at execution. Either way, the matrix in D-01 is already proven by `ci.yml`.
- **`scripts/demo.sh` auto-detect mode** — when `localhost:3001/healthz` is up, the script switches to docker mode automatically. D-13 leverages this: `docker compose up` first, then `./scripts/demo.sh` runs in docker mode without explicit flag.
- **`87-VERIFICATION.md` and `88-VERIFICATION.md` structure** — section per requirement, evidence table, command + output captured. D-15 mirrors this directly.

### Established Patterns
- **Single atomic commit per logical step** (Phase 83-88 carry-forward) — straggler-fix commit, validation-script commit, evidence-doc commits, tag-cut, milestone-close commits each land as their own commit.
- **Hybrid gate (fix inline if trivial)** (Phase 85/86 carry-forward) — D-14 reuses this for any rebrand-side-effect breakage discovered during validation. Strict gate would cause excess phase overhead for a solo-dev close-out.
- **Repo-wide grep with allowlist** (Phase 85/86/87/88 carry-forward) — D-10 reuses the established `trustedge|TRUSTEDGE` grep with the same allowlist (TrustEdge-Labs brand, MIGRATION.md historical, CHANGELOG.md, .planning/ archives, Phase 86 D-01a artifacts).
- **Tag-format convention** — `v6.0.0` matches `v2.4`, `v3.0`, `v4.0` history (per recent commits + memory milestone history). No floating major/minor tags.

### Integration Points
- **v6.0.0 tag-push ↔ ci.yml release/self-attest job ↔ sealedge-attest-sbom-action@v2** — three-way integration. The first green tag-push run proves: (a) full workspace builds release-mode green under new names, (b) `seal` binary uploads cleanly, (c) `sealedge-attest-sbom-action@v2` (renamed in Phase 88) downloads `seal` + verifies SHA256 + attests SBOM correctly, (d) attestation file uploads with the correct `.se-attestation.json` extension. Single run validates VALID-02 + closes the Phase 88 deferred dogfood loop.
- **docker compose stack ↔ scripts/demo.sh ↔ verify-receipt** — D-13 flow: `docker compose up` → platform server healthy at :3001 → `./scripts/demo.sh` auto-detects docker → wraps a sample, posts to `/v1/verify` → receipt comes back signed. Proves the entire archive/verify roundtrip end-to-end.
- **dashboard ↔ platform server** — D-12 manual browser smoke proves the SvelteKit dashboard talks to the platform server (device-list fetch). Confirms env var swap (`TRUSTEDGE_*` → `SEALEDGE_*` in `.env.local` if applicable) didn't break wiring.
- **Phase 87/88 evidence reuse** — Phase 89 cites prior verification files for already-proven gates (post-rename main CI green, repo redirect working, Action repo renamed); does NOT re-run them.

</code_context>

<specifics>
## Specific Ideas

- **Local matrix command set** (D-01 verbatim, mirrors `ci.yml` build-and-test job):
  ```
  cargo test --workspace --no-default-features --locked
  cargo test -p sealedge-core --features "audio,git-attestation,keyring,insecure-tls" --locked
  cargo test -p sealedge-core --features yubikey --lib --locked
  cargo test -p sealedge-platform --lib --locked
  cargo test -p sealedge-platform --test verify_integration --locked
  cargo test -p sealedge-platform --test verify_integration --features http --locked
  ```

- **Final straggler grep audit** (D-10 — to be run BEFORE the v6.0.0 tag cut):
  ```
  git ls-files \
    | xargs grep -nE "trustedge|TRUSTEDGE|\.trst\b|\.te-attestation" 2>/dev/null \
    | grep -vE "TrustEdge-Labs|TrustEdge Labs|trustedgelabs\.com|MIGRATION\.md|CHANGELOG\.md|^\.planning/(milestones|phases)/|RFC_K256_SUPPORT\.md|improvement-plan\.md|security-review-platform\.md"
  ```
  Expected: zero results outside the known `ci.yml` lines 212/220 stragglers (which Phase 89 fixes in D-10).

- **WASM size check** (D-11 — mirrors `wasm-tests.yml`):
  ```
  cd crates/wasm && wasm-pack build --target web --release
  wc -c < pkg/sealedge_wasm_bg.wasm                # must be < 2MB

  cd crates/seal-wasm && wasm-pack build --target web --release
  wc -c < pkg/sealedge_seal_wasm_bg.wasm           # record size
  ```

- **Docker e2e command set** (D-13):
  ```
  docker compose -f deploy/docker-compose.yml up --build -d
  # wait for platform health
  until curl -sf http://localhost:3001/healthz > /dev/null; do sleep 2; done
  # run demo (auto-detects docker mode)
  ./scripts/demo.sh
  # capture receipt for evidence
  ```

- **Tag cut sequence** (D-06/D-08 — exact commands for user):
  ```
  # User runs these from their shell after Claude confirms gates green:
  git tag -a v6.0.0 -m "v6.0 Sealedge Rebrand"
  git push origin v6.0.0
  # Then create release with notes file:
  gh release create v6.0.0 \
    --title "v6.0.0 — Sealedge Rebrand" \
    --notes-file .planning/phases/89-final-validation/RELEASE-NOTES-v6.0.0.md
  ```

- **Tag-push CI watch** (D-04/D-06):
  ```
  # After tag push, watch the run that the tag triggered:
  gh run watch $(gh run list --workflow=ci.yml --event=push --limit=1 --json databaseId -q '.[0].databaseId')
  # Capture URL for 89-VERIFICATION.md
  ```

- **Pre-tag gate checklist** (D-06, mirrors Phase 87 D-06 pattern):
  1. `git status` clean on `main`; `git log origin/main..HEAD` empty (main pushed)
  2. `validate-v6.sh` exit code 0 (full local matrix green)
  3. Latest CI run on main green (`gh run list --workflow=ci.yml --branch main --limit 1`)
  4. wasm-tests.yml + semver.yml workflow_dispatch runs green (URLs captured)
  5. Straggler grep audit zero-result (D-10 commit landed)
  6. No open PRs (`gh pr list --state open`)

- **Release notes body sketch** (D-09 — planner refines):
  ```
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

</specifics>

<deferred>
## Deferred Ideas

- **Live `cargo test --features postgres` against a running PostgreSQL** — not gated by Phase 89; postgres feature is exercised via `cargo build`/`cargo check` and the docker stack roundtrip. Standalone live-DB postgres tests are post-v6.0 polish.
- **YubiKey hardware integration test (`yubikey_integration`)** — only the `--lib` simulation tests are gated. Hardware integration test stays out of scope (matches CI behavior; requires plugged-in YubiKey).
- **Floating `v6` and `v6.0` tags** — single `v6.0.0` only. Floating tags are post-v6.0 if/when install tooling needs them.
- **Playwright/headless-browser dashboard test** — manual smoke only this phase. Automated browser test is a future polish item.
- **Phase 81 (demo GIF) and Phase 82 (landing page) execution** — punted from v5.0; resume AFTER Phase 89 closes (per memory + v5.0 roadmap state). Not part of v6.0 milestone close.
- **Publishing crates to crates.io under sealedge-* names** — out-of-scope per v6.0 REQUIREMENTS.md.
- **Permanent CI guard against `trustedge` product-name drift** — one-time grep (D-10) is sufficient. Add as backlog if drift recurs.
- **`scripts/validate-v6.sh` generalized to `scripts/validate-milestone.sh`** — start as v6-specific; if v7+ wants the same gate, generalize then. YAGNI for now.

### Reviewed Todos (not folded)
None — no todos were considered for this phase (gsd-sdk query returned 0 matches).

</deferred>

---

*Phase: 89-final-validation*
*Context gathered: 2026-04-21*
