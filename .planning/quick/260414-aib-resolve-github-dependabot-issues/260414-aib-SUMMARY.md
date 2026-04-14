---
phase: 260414-aib
plan: 01
subsystem: dependencies
tags: [security, dependencies, npm, cargo, rand]
one_liner: "Resolved 7 npm security vulnerabilities via audit fix and migrated rand 0.8→0.9 with all breaking API changes fixed"
completed_at: "2026-04-14T11:58:30Z"
duration_minutes: 55
tasks_completed: 3
files_changed: 18
commits:
  - hash: 8f47b59
    message: "chore(260414-aib-01): update npm deps via audit fix"
  - hash: 9f82158
    message: "feat(260414-aib-01): migrate rand 0.8 → 0.9, fix all breaking API changes"
key_decisions:
  - "Keep rand_core = 0.6 in workspace — ed25519-dalek 2.x / rsa 0.9 / p256 0.13 all require rand_core 0.6 CryptoRngCore; upgrading to rand_core 0.9 would break all crypto crate call sites"
  - "Use rand_core::OsRng (0.6) for all crypto crate call sites; use rand::rng() (0.9) only for plain byte generation"
  - "WASM crates: use aead::rand_core::RngCore instead of rand::RngCore to avoid cross-version trait mismatch"
---

# Quick Task 260414-aib: Resolve GitHub Dependabot Issues

## Summary

Resolved all open Dependabot security alerts: 7 npm vulnerabilities fixed via `npm audit fix`, and rand migrated from 0.8 to 0.9 with all breaking API changes addressed. Both stale Dependabot PRs (#29, #30) closed with explanatory comments.

## Tasks Completed

### Task 1: Update npm dependencies
- Ran `npm audit fix` which resolved 7 vulnerabilities (5 high, 2 moderate):
  - flatted: prototype pollution (GHSA-rf6f-7fwh-wjgh)
  - lodash: code injection + prototype pollution
  - picomatch: method injection + ReDoS
  - vite: path traversal + arbitrary file read
  - @sveltejs/kit: DoS redirect + body size bypass
  - brace-expansion: zero-step sequence DoS
  - yaml: stack overflow via deep nesting
- Dashboard `npm run build` and `npm run check` both pass
- `npm audit` shows 0 vulnerabilities

### Task 2: Migrate rand 0.8 → 0.9

Key changes:
- `Cargo.toml`: `rand = "0.9"`, keep `rand_core = "0.6"` (crypto crate compatibility)
- `getrandom`: updated to `0.3` with `wasm_js` feature (renamed from `js`)
- `crates/trst-cli/Cargo.toml`: `rand = "0.9"`, `rand_chacha = "0.9"`, removed `rand_core` `getrandom` feature
- `crates/platform/Cargo.toml`: added `rand_core` workspace dep (needed for `rand_core::OsRng`)

Breaking API changes fixed:
- `rand::thread_rng()` → `rand::rng()` (envelope.rs, trst-cli/main.rs, platform auth)
- `rand::Rng::gen_range` → `random_range` (platform token generation)
- All `rand::rngs::OsRng` → `rand_core::OsRng` in call sites passing to ed25519-dalek/rsa/p256
- WASM crates: `rand::RngCore` → `aead::rand_core::RngCore` (avoids cross-version trait mismatch)

All 406 tests pass. Clippy clean.

### Task 3: Close Dependabot PRs
- PR #29 (npm flatted/lodash/picomatch): closed with comment referencing commit 8f47b59
- PR #30 (Cargo rand 0.8→0.9): closed with comment referencing commit 9f82158

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] rand_core 0.9 incompatible with ed25519-dalek/rsa/p256**
- **Found during:** Task 2
- **Issue:** Plan suggested upgrading `rand_core = "0.6"` to `"0.9"`, but ed25519-dalek 2.x, rsa 0.9, and p256 0.13 all depend on rand_core 0.6's `CryptoRngCore` trait. rand_core 0.9 is a different version — traits are incompatible.
- **Fix:** Kept `rand_core = "0.6"` in workspace; used `rand_core::OsRng` (0.6) for all calls into crypto crates; used `rand::rng()` (0.9) only for plain byte generation.
- **Files modified:** Cargo.toml, all affected .rs files

**2. [Rule 2 - Missing dep] trustedge-platform had no rand_core dependency**
- **Found during:** Task 2
- **Issue:** jwks.rs needed `rand_core::OsRng` but `rand_core` was not in platform's Cargo.toml
- **Fix:** Added `rand_core = { workspace = true }` to crates/platform/Cargo.toml

**3. [Rule 1 - Bug] trst-cli/Cargo.toml had invalid rand_core feature**
- **Found during:** Task 2
- **Issue:** `rand_core = { workspace = true, features = ["getrandom"] }` — in rand_core 0.9, `getrandom` is no longer a feature flag
- **Fix:** Removed the `features = ["getrandom"]` from rand_core dep in trst-cli

**4. [Rule 1 - Bug] npm audit found more vulnerabilities than planned**
- **Found during:** Task 1
- **Issue:** Plan listed flatted/lodash/picomatch/vite; audit also found @sveltejs/kit, brace-expansion, yaml
- **Fix:** `npm audit fix` resolved all 7 (not just 4) vulnerabilities

## Verification Results

1. `cd web/dashboard && npm run build && npm run check` — PASS
2. `npm audit` — 0 vulnerabilities
3. `cargo build --workspace` — PASS (clean, no warnings)
4. `cargo test --workspace` — 406 tests pass (0 failures)
5. `cargo clippy --workspace -- -D warnings` — PASS (no warnings)
6. PRs #29 and #30 — CLOSED with comments

## Self-Check

- Commits exist: 8f47b59 (npm), 9f82158 (rand migration) — VERIFIED
- Dependabot PRs closed — VERIFIED (gh pr list --state closed shows both)
- No stubs introduced — this is pure dependency/build work

## Self-Check: PASSED
