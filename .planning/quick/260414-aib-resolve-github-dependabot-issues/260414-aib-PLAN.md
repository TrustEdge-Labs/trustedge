---
phase: 260414-aib
plan: 01
type: execute
wave: 1
depends_on: []
files_modified:
  - web/dashboard/package-lock.json
  - web/dashboard/package.json
  - Cargo.toml
  - Cargo.lock
  - crates/core/src/envelope.rs
  - crates/core/src/crypto.rs
  - crates/core/src/auth.rs
  - crates/core/src/point_attestation.rs
  - crates/core/src/hybrid.rs
  - crates/core/src/asymmetric.rs
  - crates/core/src/backends/software_hsm.rs
  - crates/core/src/applications/attestation/mod.rs
  - crates/core/src/applications/receipts/mod.rs
  - crates/core/src/bin/trustedge-client.rs
  - crates/core/benches/crypto_benchmarks.rs
  - crates/core/examples/receipts_demo.rs
  - crates/core/tests/domain_separation_test.rs
  - crates/platform/tests/verify_integration.rs
  - crates/trst-cli/src/main.rs
  - crates/trst-cli/Cargo.toml
  - crates/trst-cli/tests/acceptance.rs
  - crates/wasm/src/crypto.rs
  - crates/trst-wasm/src/crypto.rs
  - crates/trustedge-cli/src/main.rs
autonomous: true
requirements: []
must_haves:
  truths:
    - "npm audit shows no known vulnerabilities for flatted, lodash, picomatch, vite"
    - "cargo build --workspace compiles cleanly with rand 0.9"
    - "cargo test --workspace passes all existing tests"
    - "Dependabot PRs #29 and #30 are closed with explanatory comments"
  artifacts:
    - path: "web/dashboard/package-lock.json"
      provides: "Updated npm dependency locks"
    - path: "Cargo.toml"
      provides: "Workspace rand = 0.9 declaration"
    - path: "Cargo.lock"
      provides: "Updated Cargo dependency locks"
  key_links:
    - from: "Cargo.toml (workspace)"
      to: "all crates using rand"
      via: "workspace dependency inheritance"
      pattern: 'rand\s*=\s*"0\.9"'
---

<objective>
Resolve all open Dependabot security alerts by updating npm dependencies (flatted, lodash, picomatch, vite) and migrating Rust rand 0.8 to 0.9 with its breaking API changes.

Purpose: Close 7 security alerts (GitHub alerts 51, 55, 58-61, 64-67) and clean up stale Dependabot PRs.
Output: Updated lockfiles, migrated Rust code, closed PRs.
</objective>

<execution_context>
@$HOME/.claude/get-shit-done/workflows/execute-plan.md
@$HOME/.claude/get-shit-done/templates/summary.md
</execution_context>

<context>
@CLAUDE.md
@Cargo.toml
@web/dashboard/package.json
</context>

<tasks>

<task type="auto">
  <name>Task 1: Update npm dependencies and vite</name>
  <files>web/dashboard/package.json, web/dashboard/package-lock.json</files>
  <action>
In web/dashboard/:
1. Run `npm update flatted lodash picomatch` to pull security patches into the lockfile.
2. Update vite to latest 6.x: run `npm install vite@latest --save-dev` (resolves alerts 60-61 for CVE in vite).
3. Run `npm audit` and verify no remaining high/medium vulnerabilities for these packages.
4. Run `npm run build` to verify the dashboard builds cleanly.
5. Run `npm run check` to verify TypeScript/Svelte type checking passes.

If npm audit still shows issues after updating, check if `npm audit fix` resolves them. Do NOT use `--force` flag.
  </action>
  <verify>
    <automated>cd web/dashboard && npm run build && npm run check && npm audit --audit-level=moderate 2>&1 | grep -E "found 0 vulnerabilities|0 vulnerabilities" || echo "AUDIT_ISSUES_REMAIN"</automated>
  </verify>
  <done>npm build and check pass. npm audit shows no moderate+ vulnerabilities for flatted, lodash, picomatch, or vite.</done>
</task>

<task type="auto">
  <name>Task 2: Migrate rand 0.8 to 0.9 across Rust workspace</name>
  <files>
Cargo.toml, Cargo.lock,
crates/core/src/envelope.rs, crates/core/src/crypto.rs, crates/core/src/auth.rs,
crates/core/src/point_attestation.rs, crates/core/src/hybrid.rs, crates/core/src/asymmetric.rs,
crates/core/src/backends/software_hsm.rs, crates/core/src/applications/attestation/mod.rs,
crates/core/src/applications/receipts/mod.rs, crates/core/src/bin/trustedge-client.rs,
crates/core/benches/crypto_benchmarks.rs, crates/core/examples/receipts_demo.rs,
crates/core/tests/domain_separation_test.rs,
crates/platform/tests/verify_integration.rs,
crates/trst-cli/src/main.rs, crates/trst-cli/Cargo.toml, crates/trst-cli/tests/acceptance.rs,
crates/wasm/src/crypto.rs, crates/trst-wasm/src/crypto.rs,
crates/trustedge-cli/src/main.rs
  </files>
  <action>
**Step 1: Update Cargo.toml dependencies**

In workspace root Cargo.toml:
- Change `rand = "0.8"` to `rand = "0.9"`
- Change `rand_core = "0.6"` to `rand_core = "0.9"` (rand 0.9 uses rand_core 0.9)
- Change `getrandom = { version = "0.2", features = ["js"] }` to `getrandom = { version = "0.3", features = ["wasm_js"] }` (rand 0.9 uses getrandom 0.3; the "js" feature was renamed to "wasm_js")

In crates/trst-cli/Cargo.toml:
- Change `rand = "0.8"` to `rand = "0.9"` (this crate has its own non-workspace rand dep)
- Change `rand_chacha = "0.3"` to `rand_chacha = "0.9"` (compatible with rand 0.9)

Do NOT touch crates/experimental/ — those are a separate workspace and not part of CI.

**Step 2: Fix breaking API changes**

The key breaking changes in rand 0.9:

1. `rand::thread_rng()` replaced by `rand::rng()`:
   - In crates/core/src/envelope.rs line 157: change `rand::thread_rng()` to `rand::rng()`
   - In crates/trst-cli/src/main.rs line 529: change `rand::thread_rng()` to `rand::rng()`

2. `rand::rngs::OsRng` is now a zero-sized unit struct, no longer needs `&mut`:
   - `rand::rngs::OsRng` usage: In rand 0.9, `OsRng` is still available at `rand::rngs::OsRng` but it's now `OsRng` (not `OsRng {}`). The struct instantiation `OsRng {}` should become just `OsRng`. Most usages with `&mut OsRng` should still work since OsRng implements RngCore.
   - Check each usage — if code does `let mut csprng = OsRng {};` change to `let mut csprng = OsRng;`
   - If code does `OsRng {}` in struct literal position, change to just `OsRng`

3. `rand_core::OsRng` path: In rand_core 0.9, OsRng moved. Import from `rand::rngs::OsRng` instead, or use `rand_core::OsRng` if still available. Check compilation — if `rand_core::OsRng` is gone, switch to `rand::rngs::OsRng`.

4. `rand::RngCore` trait: Still available, no change needed for `fill_bytes()` calls.

5. `rand::prelude::*` in trst-cli: Still works in 0.9, includes `RngCore` etc.

6. `rand_core::RngCore` re-export: Should still work, but verify.

7. `OsRng.fill_bytes()`: In rand 0.9, OsRng implements RngCore directly so `OsRng.fill_bytes(&mut buf)` still works. But note OsRng might need to be mutable: `let mut rng = OsRng; rng.fill_bytes(...)`.

8. The `aead::OsRng` import (from aes-gcm crate): This is re-exported from the crypto-common/aead crate, NOT from rand. These imports like `aead::{Aead, AeadCore, KeyInit, OsRng}` should still work unchanged since they come from the `crypto-common` crate. Do NOT change these.

**Step 3: Build and fix iteratively**

Run `cargo build --workspace 2>&1` and fix any remaining compilation errors. The most likely issues will be:
- Import path changes for OsRng
- `thread_rng()` renamed to `rng()`  
- Trait method signature changes
- `OsRng {}` struct literal syntax

Run `cargo test --workspace` to verify all 406+ tests still pass. Fix any test failures.

Run `cargo clippy --workspace -- -D warnings` to ensure no new warnings.
  </action>
  <verify>
    <automated>cd /home/john/vault/projects/github.com/trustedge && cargo build --workspace 2>&1 | tail -5 && cargo test --workspace 2>&1 | tail -10</automated>
  </verify>
  <done>Workspace compiles with rand 0.9. All existing tests pass. No clippy warnings.</done>
</task>

<task type="auto">
  <name>Task 3: Close Dependabot PRs with comments</name>
  <files></files>
  <action>
After the npm and Cargo updates are committed to main:

1. Close PR #29 (npm flatted/lodash/picomatch) with comment:
   "Closing — these dependency updates were applied directly to main along with a vite security update. See commit [ref]."

2. Close PR #30 (Cargo rand 0.8->0.9) with comment:
   "Closing — rand 0.9 migration (with breaking API changes fixed) was applied directly to main. See commit [ref]."

Use `gh pr close 29 --repo trustedge/trustedge --comment "..."` and same for 30.

Note: The repo might be `trustedge-labs/trustedge` or `trustedge/trustedge` — check with `gh repo view --json nameWithOwner` first.
  </action>
  <verify>
    <automated>gh pr list --repo $(gh repo view --json nameWithOwner -q .nameWithOwner) --state closed 2>&1 | grep -E "#29|#30" || echo "PRs not found as closed"</automated>
  </verify>
  <done>Both PRs #29 and #30 are closed with explanatory comments referencing the direct-to-main commits.</done>
</task>

</tasks>

<threat_model>
## Trust Boundaries

| Boundary | Description |
|----------|-------------|
| npm registry | Third-party packages pulled via npm update |
| crates.io | Third-party crates pulled via cargo update |

## STRIDE Threat Register

| Threat ID | Category | Component | Disposition | Mitigation Plan |
|-----------|----------|-----------|-------------|-----------------|
| T-aib-01 | Tampering | npm packages | mitigate | npm lockfile integrity + npm audit verification |
| T-aib-02 | Tampering | Cargo crates | mitigate | Cargo.lock integrity + full test suite verification |
| T-aib-03 | Information Disclosure | rand CSPRNG | mitigate | Verify OsRng still used for all crypto operations (no downgrade to non-CS RNG) |
</threat_model>

<verification>
1. `cd web/dashboard && npm run build && npm run check` passes
2. `npm audit` shows no moderate+ vulnerabilities
3. `cargo build --workspace` compiles cleanly
4. `cargo test --workspace` — all tests pass
5. `cargo clippy --workspace -- -D warnings` — no warnings
6. PRs #29 and #30 are closed on GitHub
</verification>

<success_criteria>
All 7 Dependabot security alerts resolved. Workspace builds and tests pass with updated dependencies. Stale PRs cleaned up.
</success_criteria>

<output>
No SUMMARY.md needed for quick plans.
</output>
