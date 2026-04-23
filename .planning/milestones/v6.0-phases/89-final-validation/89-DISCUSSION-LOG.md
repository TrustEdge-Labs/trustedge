# Phase 89: Final Validation - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-04-21
**Phase:** 89-final-validation
**Areas discussed:** Test matrix & execution, v6.0.0 release tag cut, Docker + dashboard + demo depth, Failure policy & evidence

---

## Test matrix & execution

### Q1: What's the scope of the local test-run gate in Phase 89?

| Option | Description | Selected |
|--------|-------------|----------|
| Full matrix, CI-parity | Run the exact feature combos ci.yml runs (default, core+audio/git/keyring/tls, yubikey lib, platform lib+verify_integration+http). No postgres live DB locally, no yubikey hardware. Captures proof that local == CI parity. | ✓ |
| Minimal: workspace test + let CI prove rest | Locally run only `cargo test --workspace --no-default-features` green; treat the first CI run on the v6.0.0 tag as the authoritative proof for VALID-01. | |
| Full matrix + optional hardware/DB | CI-parity locally PLUS yubikey integration test if YubiKey is plugged in AND postgres live-DB test if docker stack is up. Best-effort extras documented as 'partial local coverage'. | |

**User's choice:** Full matrix, CI-parity (Recommended)

### Q2: For VALID-02 (all GitHub Actions workflows green), what's the proof standard?

| Option | Description | Selected |
|--------|-------------|----------|
| One green push to main + one green tag run | A post-rename green CI on main (already happened per Phase 87) plus one green CI run on the v6.0.0 tag. wasm-tests.yml + semver.yml confirmed via workflow_dispatch. Capture run URLs in 89-VERIFICATION.md. | ✓ |
| Recent green runs across all 4 workflows are enough | Accept the most recent green run for each. No need to force a fresh tag-push for the release workflow. | |
| Tag cut IS the gate | Cut v6.0.0 first; VALID-02 is satisfied only if the tag push's ci.yml is green end-to-end. | |

**User's choice:** One green push to main + one green tag run (Recommended)

### Q3: How strict is the 471-test count target under the new names?

| Option | Description | Selected |
|--------|-------------|----------|
| Count must match or exceed 471 | Phase 89 fails if post-rename test count drops below 471 without explicit justification in 89-VERIFICATION.md. Acceptable if count INCREASED. | ✓ |
| Count is informational only | Record actual count in verification; no hard floor. 'All tests green' is the only gate. | |

**User's choice:** Count must match or exceed 471 (Recommended)

### Q4: Do you want a repeatable validation script under scripts/, or run the commands ad-hoc?

| Option | Description | Selected |
|--------|-------------|----------|
| New `scripts/validate-v6.sh` | Commit a shell script that runs the exact matrix Phase 89 will gate against. Reusable for future milestone-close phases. Output log piped to 89-VERIFICATION.md. | ✓ |
| Ad-hoc commands in 89-VERIFICATION.md | Document exact commands + exit codes + test counts in 89-VERIFICATION.md. No new script. | |
| Extend scripts/ci-check.sh | Add a `--full-matrix` flag to existing script. Reuses infrastructure. | |

**User's choice:** New `scripts/validate-v6.sh` (Recommended)

---

## v6.0.0 release tag cut

### Q1: When does the v6.0.0 tag get cut relative to the Phase 89 validation gates?

| Option | Description | Selected |
|--------|-------------|----------|
| Cut tag AFTER local gates pass, before phase close | Sequence: (1) local full-matrix green, (2) CI on main green, (3) cut v6.0.0, (4) tag's CI run is the final VALID-02 gate, (5) post-tag evidence into 89-VERIFICATION.md. | ✓ |
| Cut tag FIRST, let CI be the only gate | Cut v6.0.0 immediately after Phase 88 closes; treat the tag's CI run as authoritative proof. | |
| Defer tag to a separate post-phase step | Phase 89 validates only; v6.0.0 tag cut is a separate manual operation after milestone close. | |

**User's choice:** Cut tag AFTER local gates pass, before phase close (Recommended)

### Q2: What's the tag label format?

| Option | Description | Selected |
|--------|-------------|----------|
| v6.0.0 | Matches prior milestone tag convention (v2.4, v3.0, v4.0). Floating major/minor tags not cut. | ✓ |
| v6.0.0 + floating v6 + v6.0 | Cut all three for install-convenience. Adds floating-tag maintenance burden. | |
| sealedge-v6.0.0 (rebrand-prefixed) | New product-prefixed tag scheme. Breaks continuity with v1.0-v5.0 series. | |

**User's choice:** v6.0.0 (Recommended)

### Q3: Who executes the `git tag` + `git push --tags`?

| Option | Description | Selected |
|--------|-------------|----------|
| User runs it | Matches Phase 87/88 pattern for live external operations. Claude provides exact commands + pre-tag checklist; user executes from their shell. | ✓ |
| Claude runs it via Bash tool | Faster, but tag-push-to-origin is a shared-surface write. | |

**User's choice:** User runs it (Recommended)

### Q4: What's the scope of the v6.0.0 GitHub release notes body?

| Option | Description | Selected |
|--------|-------------|----------|
| Short rebrand announcement + link to MIGRATION.md | ~10-line release notes: 'v6.0 Sealedge Rebrand' + what changed + link to existing MIGRATION.md. | ✓ |
| Full migration guide inline in release notes | Embed the full breaking-change table directly in release notes. | |
| Auto-generated from commits only | Use `gh release create --generate-notes`. Low effort; no rebrand context. | |

**User's choice:** Short rebrand announcement + link to MIGRATION.md (Recommended)

### Q5: Final straggler sweep — known `seal.te-attestation.json` in ci.yml. Is Phase 89 the final sweep?

| Option | Description | Selected |
|--------|-------------|----------|
| Yes — do one final repo-wide grep-audit + fix | Run the established grep with Phase 85/86 allowlist. Fix stragglers in single atomic commit BEFORE cutting v6.0.0. | ✓ |
| Strict verify-only — fail if stragglers exist | Phase 89 fails if grep-audit finds live stragglers. Forces a hotfix phase. | |
| Ignore stragglers — they don't block the rebrand goal | Pre-existing Phase 84 gap; not blocking. Document in deferred. | |

**User's choice:** Yes — do one final repo-wide grep-audit + fix (Recommended)

---

## Docker + dashboard + demo depth

### Q1: For VALID-03 Docker stack gate, what's the depth?

| Option | Description | Selected |
|--------|-------------|----------|
| Full e2e: stack up + demo script + verify receipt | `docker compose up --build` + `./scripts/demo.sh` (auto-detect docker mode). Capture receipt response. | ✓ |
| Stack-up smoke only | `docker compose up` confirm containers healthy. No demo run. | |
| Local demo + separate docker smoke | Two independent gates: `./scripts/demo.sh --local` + `docker compose up` smoke. | |

**User's choice:** Full e2e: stack up + demo script + verify receipt (Recommended)

### Q2: How deep does the dashboard validation go?

| Option | Description | Selected |
|--------|-------------|----------|
| Build + type-gen clean, then manual browser smoke | npm build + npm check + browser smoke for title/headings rendering "Sealedge" + one device-list fetch. Screenshot. | ✓ |
| Build + type-gen only (no browser) | `npm run build` + `npm run check` green. No live browser check. | |
| Playwright/smoke-test script | Headless browser smoke test. Highest coverage but biggest scope creep. | |

**User's choice:** Build + type-gen clean, then manual browser smoke (Recommended)

### Q3: For VALID-03 WASM gate?

| Option | Description | Selected |
|--------|-------------|----------|
| Both crates: cargo check + wasm-pack build | `cargo check` + `wasm-pack build --target web --release` for both crates, capture .wasm sizes. | ✓ |
| cargo check only | Rely on wasm-tests.yml for full wasm-pack build. Less proof. | |

**User's choice:** Both crates: cargo check + wasm-pack build (Recommended)

---

## Failure policy & evidence

### Q1: If a gate fails during Phase 89 validation, what's the policy?

| Option | Description | Selected |
|--------|-------------|----------|
| Fix inline | Hybrid gate — trivial fixes land as atomic commits in Phase 89 itself. Mirrors Phase 85/86 hybrid-treatment pattern. | ✓ |
| Fail the phase, open 89.1 hotfix sub-phase | Strict gate: any failure spawns a new decimal sub-phase. | |
| Fix inline but require 2nd human review gate | Fix inline, but each inline fix confirmed by user before next sub-step proceeds. | |

**User's choice:** Fix inline (Recommended)

### Q2: What goes in 89-VERIFICATION.md?

| Option | Description | Selected |
|--------|-------------|----------|
| Structured evidence table per VALID-01/02/03 | Section per requirement: command, exit code, test count, CI URL, screenshots, release asset list, verify-attestation roundtrip, curl receipt. | ✓ |
| Freeform narrative | Prose-style verification. Faster but harder to audit. | |
| Minimal — exit codes only | Table of commands + green/red status only. | |

**User's choice:** Structured evidence table per VALID-01/02/03 (Recommended)

### Q3 (multi-select): What milestone-close artifacts does Phase 89 produce?

| Option | Description | Selected |
|--------|-------------|----------|
| 89-VERIFICATION.md | Evidence per VALID-01/02/03 — committed at phase close. | ✓ |
| Update ROADMAP.md with completion date + final phase count | Mark Phase 89 [x]; update progress table; mark v6.0 milestone ✅ Shipped. | ✓ |
| Update PROJECT.md (current milestone, v6.0 done) | Flip current milestone to "Shipped v6.0" + prep next milestone slot. | ✓ |
| Archive v6.0 to .planning/milestones/ | Run `/gsd-complete-milestone` flow — archive roadmap, requirements, phase artifacts. | ✓ |

**User's choice:** All four selected (full milestone-close set)

---

## Claude's Discretion

- Plan granularity (planner picks 3-4 plans across script/audit, validation/evidence, tag-cut, milestone-close)
- Validation script structure (monolithic vs split-with-orchestrator)
- Tag-failure recovery (force-update v6.0.0 vs cut v6.0.1)
- Milestone-close ordering (ROADMAP/PROJECT vs archival sequence)
- Release-notes file location (phase-local vs repo-root)

## Deferred Ideas

- Live postgres standalone tests (not gated)
- YubiKey hardware integration test (not gated)
- Floating v6/v6.0 tags
- Playwright dashboard automation
- Phase 81 + Phase 82 (resume after Phase 89 closes)
- crates.io publishing
- Permanent CI guard for trustedge drift
- `validate-v6.sh` → `validate-milestone.sh` generalization
