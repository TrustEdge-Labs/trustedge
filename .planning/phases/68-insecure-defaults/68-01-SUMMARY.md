---
phase: 68-insecure-defaults
plan: "01"
subsystem: security
tags: [security, insecure-defaults, config, p2-remediation]
dependency_graph:
  requires: []
  provides: [DFLT-01, DFLT-02]
  affects: [trustedge-platform/ca, trustedge-core/backends]
tech_stack:
  added: []
  patterns: [explicit-config-construction, cfg-test-helpers]
key_files:
  created: []
  modified:
    - crates/platform/src/ca/mod.rs
    - crates/core/src/backends/software_hsm.rs
    - crates/core/src/backends/universal_registry.rs
decisions:
  - Removed impl Default for CAConfig to prevent placeholder JWT secret 'your-secret-key' from silently reaching production
  - Removed impl Default for SoftwareHsmConfig to prevent placeholder passphrase 'changeme123!' from reaching production
  - Removed SoftwareHsmBackend::new() which called the now-deleted default
  - CAConfigBuilder::default() retains inline values (not delegating to CAConfig::default()) — its build() guard already panics on placeholder outside tests
  - universal_registry.rs uses explicit passphrase 'registry-auto-discovery' for auto-discovery context, not a production secret
  - Added #[cfg(test)] test_default() helpers on both configs for test ergonomics without exposing insecure defaults
metrics:
  duration_minutes: 15
  tasks_completed: 2
  files_modified: 3
  completed_date: "2026-03-26"
---

# Phase 68 Plan 01: Insecure Defaults Removal Summary

**One-liner:** Removed `impl Default` from `CAConfig` and `SoftwareHsmConfig`, eliminating placeholder credentials (JWT secret "your-secret-key", passphrase "changeme123!") from production code paths.

## What Was Built

Closed P2 security finding: insecure default configurations that could silently reach production code.

### Changes Made

**crates/platform/src/ca/mod.rs (DFLT-01):**
- Removed `impl Default for CAConfig` entirely — calling `CAConfig::default()` is now a compile error
- Rewrote `impl Default for CAConfigBuilder` to inline values directly (no delegation to the deleted `CAConfig::default()`)
- Added `#[cfg(test)] impl CAConfig { pub fn test_default() -> Self }` for test ergonomics
- Added `test_caconfig_test_default` test verifying the helper works correctly

**crates/core/src/backends/software_hsm.rs (DFLT-02):**
- Removed `impl Default for SoftwareHsmConfig` entirely — calling `SoftwareHsmConfig::default()` is now a compile error
- Removed `SoftwareHsmBackend::new()` which depended on the deleted default
- Added `#[cfg(test)] impl SoftwareHsmConfig { pub fn test_default() -> Self }` for test ergonomics
- Replaced `test_backend_creation_with_new` test with `test_backend_creation_with_test_default`

**crates/core/src/backends/universal_registry.rs:**
- Replaced `SoftwareHsmBackend::new()` call with explicit `SoftwareHsmConfig::builder().default_passphrase("registry-auto-discovery").build()` construction

## Test Results

- `cargo test -p trustedge-platform --lib --features ca -- ca::tests`: 6/6 passed
- `cargo test -p trustedge-core --lib`: 184/184 passed
- `cargo test --workspace --lib -- --skip test_many_keys`: 252 passed, 0 failed
- `cargo clippy --workspace -- -D warnings`: clean

## Commits

| Task | Commit | Description |
|------|--------|-------------|
| 1 | 826e307 | fix(68-01): remove CAConfig Default impl, inline CAConfigBuilder defaults |
| 2 | fdb9a71 | fix(68-01): remove SoftwareHsmConfig Default impl and SoftwareHsmBackend::new() |

## Deviations from Plan

None — plan executed exactly as written.

## Known Stubs

None.

## Self-Check: PASSED

- `/home/john/vault/projects/github.com/trustedge/.claude/worktrees/agent-a971017d/crates/platform/src/ca/mod.rs` — exists, does not contain `impl Default for CAConfig {`
- `/home/john/vault/projects/github.com/trustedge/.claude/worktrees/agent-a971017d/crates/core/src/backends/software_hsm.rs` — exists, does not contain `impl Default for SoftwareHsmConfig {`, does not contain `pub fn new() -> Result<Self>`, does not contain `changeme123!`
- `/home/john/vault/projects/github.com/trustedge/.claude/worktrees/agent-a971017d/crates/core/src/backends/universal_registry.rs` — exists, does not contain `SoftwareHsmBackend::new()`, contains `registry-auto-discovery`
- Commits 826e307 and fdb9a71 verified via `git log`
