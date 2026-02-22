---
phase: 32-workspace-cleanup
verified: 2026-02-22T19:00:00Z
status: passed
score: 10/10 must-haves verified
re_verification: false
gaps: []
human_verification: []
---

# Phase 32: Workspace Cleanup Verification Report

**Phase Goal:** Deprecated facade crates are gone from the workspace, and Tier 2 experimental crates are isolated so their dependency graph does not contaminate the shared Cargo.lock
**Verified:** 2026-02-22T19:00:00Z
**Status:** passed
**Re-verification:** No — initial verification

## Goal Achievement

### Observable Truths

| #  | Truth | Status | Evidence |
|----|-------|--------|----------|
| 1  | trustedge-receipts and trustedge-attestation directories do not exist on disk | VERIFIED | `ls crates/receipts/ crates/attestation/` returns "No such file or directory" for both |
| 2  | cargo build --workspace succeeds without the facade crates | VERIFIED | Cargo.lock regenerated cleanly; commit `c4b201b` confirms build success post-deletion |
| 3  | Cargo.lock no longer references trustedge-receipts or trustedge-attestation | VERIFIED | `grep -c 'trustedge-receipts\|trustedge-attestation' Cargo.lock` returns 0 |
| 4  | Pubky crates live under crates/experimental/ with their own workspace Cargo.toml | VERIFIED | `crates/experimental/Cargo.toml` exists with `[workspace]` listing pubky and pubky-advanced as members |
| 5  | Root workspace members list does not include pubky or pubky-advanced | VERIFIED | Workspace members list contains only 9 crates under crates/ + examples/cam.video; no pubky entries |
| 6  | Experimental workspace Cargo.lock is gitignored | VERIFIED | `crates/experimental/.gitignore` contains `/target/` and `Cargo.lock`; git confirms file is not tracked |
| 7  | Root Cargo.lock no longer references trustedge-pubky or trustedge-pubky-advanced | VERIFIED | `grep -c 'name = "trustedge-pubky"' Cargo.lock` returns 0; package count is 636 (down from 697) |
| 8  | Root workspace.dependencies no longer includes pubky, x25519-dalek, hkdf as direct deps | VERIFIED | grep on Cargo.toml confirms none of these appear in `[workspace.dependencies]`; note: hkdf appears in Cargo.lock as a transitive dep of p256/sqlx — this is correct and expected |
| 9  | ci-check.sh and ci.yml use single-tier workspace passes with no facade or pubky crate references | VERIFIED | ci-check.sh Step 4 uses `--workspace` flag; no references to trustedge-receipts/attestation/pubky in either file; tiered logic removed |
| 10 | CLAUDE.md, README.md, DEPENDENCIES.md, FEATURES.md reflect the actual 9-crate workspace | VERIFIED | CLAUDE.md says "9 crates under crates/"; all docs note experimental crates in crates/experimental/; no facade crate sections remain |

**Score:** 10/10 truths verified

### Required Artifacts

| Artifact | Expected | Status | Details |
|----------|----------|--------|---------|
| `Cargo.toml` | Workspace members without facade crates or pubky; contains `members = [` | VERIFIED | Members list: core, types, platform, platform-server, trustedge-cli, wasm, trst-protocols, trst-cli, trst-wasm, examples/cam.video — no receipts, attestation, pubky |
| `crates/experimental/Cargo.toml` | Experimental workspace definition with pubky crates as members; contains `[workspace]` | VERIFIED | File exists; contains `[workspace]` with `"pubky"` and `"pubky-advanced"` members; no `[workspace.dependencies]` (correct by design) |
| `crates/experimental/.gitignore` | Gitignore excluding Cargo.lock; contains `Cargo.lock` | VERIFIED | File exists; contains `/target/` and `Cargo.lock` |
| `scripts/ci-check.sh` | Simplified CI without tiered logic or removed crate references | VERIFIED | Step 4 uses `cargo clippy --workspace --all-targets --no-default-features`; Step 11 uses `cargo build --workspace` and `cargo test --workspace`; baseline=70 |
| `.github/workflows/ci.yml` | GitHub Actions CI without removed crate references | VERIFIED | Uses `--workspace` for clippy and tests; no facade/pubky crate references; baseline=70; single remaining `continue-on-error: true` is for semver-checks (acceptable) |
| `CLAUDE.md` | Architecture Overview matching actual workspace | VERIFIED | Reads "9 crates under crates/"; Deprecated pubky/receipts/attestation sections removed; experimental note present |
| `README.md` | Crate table matching actual workspace | VERIFIED | No trustedge-receipts or trustedge-attestation references; experimental workspace note on line 153 |
| `DEPENDENCIES.md` | Dependency audit for current crates only | VERIFIED | Sections for trustedge-receipts, trustedge-attestation, trustedge-pubky, trustedge-pubky-advanced removed; "Experimental Tier" section removed; note about crates/experimental/ added |
| `FEATURES.md` | No trustedge-attestation symbols feature section | VERIFIED | grep for trustedge-attestation returns no matches |

### Key Link Verification

| From | To | Via | Status | Details |
|------|----|-----|--------|---------|
| `crates/experimental/Cargo.toml` | `crates/experimental/pubky/Cargo.toml` | workspace members listing "pubky" | WIRED | Pattern `members.*pubky` confirmed in experimental Cargo.toml |
| `crates/experimental/pubky/Cargo.toml` | `crates/core` | `path = "../../core"` | WIRED | Path resolves: `crates/experimental/pubky/../../core/` = `crates/core/`; confirmed in file |
| `crates/experimental/pubky-advanced/Cargo.toml` | `crates/core` | `path = "../../core"` | WIRED | Same path pattern; confirmed in file |
| `scripts/ci-check.sh` | `Cargo.toml` | `--workspace` flag (not explicit `-p` names) | WIRED | ci-check.sh Steps 4 and 11 use `--workspace`; no stale `-p trustedge-pubky` or facade references |
| `.github/workflows/ci.yml` | `Cargo.toml` | `--workspace` flag | WIRED | ci.yml "clippy (workspace)" and "tests (workspace)" steps use `--workspace`; all current members covered |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
|-------------|------------|-------------|--------|----------|
| WRK-01 | 32-01 | Deprecated facade crates (trustedge-receipts, trustedge-attestation) deleted from workspace | SATISFIED | crates/receipts/ and crates/attestation/ do not exist; Cargo.lock has 0 references; commit `c4b201b` |
| WRK-02 | 32-03 | CI scripts and documentation updated to remove facade references | SATISFIED | ci-check.sh and ci.yml have 0 references to removed crates; CLAUDE.md, README.md, FEATURES.md, DEPENDENCIES.md verified clean |
| WRK-03 | 32-02 | Tier 2 experimental crates separated from Tier 1 dependency graph (pubky dep tree no longer in shared Cargo.lock) | SATISFIED | crates/experimental/ standalone workspace created; `trustedge-pubky` absent from root Cargo.lock; root package count is 636 (down from 697, -61 pubky transitive deps) |
| WRK-04 | 32-02 | Workspace Cargo.toml cleaned of unused workspace dependencies after separation | SATISFIED | pubky, x25519-dalek, hkdf removed from root `[workspace.dependencies]`; confirmed absent from Cargo.toml; hkdf remains in Cargo.lock only as legitimate transitive dep of p256/sqlx |

**Orphaned Requirements:** None — all four WRK requirements are claimed by plans and verified satisfied.

### Anti-Patterns Found

None. Scanning ci-check.sh, ci.yml, CLAUDE.md, DEPENDENCIES.md produced no blocker or warning anti-patterns. The `TODO` and `FIXME` occurrences in ci-check.sh are part of the TODO hygiene step's grep pattern strings — they are scanning for those markers, not containing them as unimplemented code.

### Human Verification Required

None. All phase 32 goals are structurally verifiable through file existence, content checks, and dependency graph analysis. No visual, real-time, or external service behaviors require human testing.

### Gaps Summary

No gaps found. All must-haves verified against the actual codebase:

- Facade crate directories confirmed absent on disk
- Root Cargo.lock confirmed free of facade and pubky package entries (636 packages, down from 697)
- Experimental workspace exists at crates/experimental/ with correct structure, gitignore, and path dependencies
- Root workspace.dependencies cleaned of pubky-only entries (pubky, x25519-dalek, hkdf)
- CI scripts simplified to single-tier --workspace passes with baseline=70 in both ci-check.sh and ci.yml
- All documentation files (CLAUDE.md, README.md, DEPENDENCIES.md, FEATURES.md) updated to reflect 9-crate workspace
- MIGRATION.md retains historical crate names as educational references only (documented decision in 32-03)
- All 5 task commits confirmed in git history: c4b201b, 3ba8688, 8851916, e63afcf, f6e52e7

One note on hkdf: the plan truth stated "Root Cargo.lock no longer contains pubky-specific heavy deps (x25519-dalek, hkdf as direct)". `hkdf` remains in the root Cargo.lock as a transitive dependency of `p256` (via `elliptic-curve`) and `sqlx-postgres` — not from pubky crates. This is correct behavior. The plan decision log explicitly states "x25519-dalek and hkdf removed from root workspace.dependencies" (not from the Cargo.lock entirely), and this is verified: neither appears in `[workspace.dependencies]`.

---

_Verified: 2026-02-22T19:00:00Z_
_Verifier: Claude (gsd-verifier)_
