---
phase: 65-key-material-safety
plan: "01"
subsystem: trst-cli, core
tags: [security, key-protection, permissions, serialization]
dependency_graph:
  requires: []
  provides: [KEY-01, KEY-02]
  affects: [crates/trst-cli/src/main.rs, crates/core/src/asymmetric.rs]
tech_stack:
  added: []
  patterns: [unix-permissions-0600, pub(crate)-field-visibility]
key_files:
  created:
    - crates/trst-cli/tests/security_key_file_protection.rs (test added, file pre-existed)
  modified:
    - crates/trst-cli/src/main.rs
    - crates/core/src/asymmetric.rs
decisions:
  - "Restrict all three PrivateKey fields (algorithm, key_bytes, key_id) to pub(crate), not just key_bytes — consistent visibility hardening"
  - "Use as_bytes() accessor in Debug impl and all tests for consistency, even though pub(crate) would allow direct access"
  - "Apply permission block after the entire if/else write block in load_or_generate_keypair to cover both encrypted and unencrypted paths"
metrics:
  duration: "~10 minutes"
  completed: "2026-03-25T23:00:05Z"
  tasks_completed: 2
  files_modified: 3
---

# Phase 65 Plan 01: Key Material Safety Summary

**One-liner:** 0600 Unix permissions on wrap-generated key files and PrivateKey serde-serialization prevention via field visibility restriction.

## Tasks Completed

| Task | Name | Commit | Files |
|------|------|--------|-------|
| 1 | Set 0600 permissions on wrap auto-generated key files (KEY-01) | c6a54dc | main.rs, security_key_file_protection.rs |
| 2 | Remove PrivateKey serde derives and restrict key_bytes field (KEY-02) | d76b02c | asymmetric.rs |
| - | cargo fmt formatting fixes | cbdb229 | asymmetric.rs, main.rs |

## What Was Built

### KEY-01: Wrap key file permissions (0600)

Added a `#[cfg(unix)]` permission block in `load_or_generate_keypair()` in `crates/trst-cli/src/main.rs` that sets the auto-generated `device.key` file to mode 0600 (owner read/write only) after writing. This mirrors the identical pattern already in the `keygen` command handler. Non-Unix platforms receive a warning message.

Added `test_wrap_autogen_key_permissions_0600` to `security_key_file_protection.rs` — runs `trst wrap --unencrypted` without `--device-key`, then asserts the resulting `device.key` has Unix mode `0o600`.

### KEY-02: PrivateKey serialization prevention

Changed `PrivateKey` struct in `crates/core/src/asymmetric.rs`:
- Removed `Serialize` and `Deserialize` from the derive macro (`#[derive(Clone, Zeroize)]` only)
- Changed `key_bytes`, `algorithm`, and `key_id` fields from `pub` to `pub(crate)`
- Removed `to_bytes()` and `from_bytes()` methods that depended on the removed serde derives
- Updated `Debug for KeyPair`, test assertions, and internal use to use `as_bytes()` accessor

The public `as_bytes()` accessor remains available to external consumers for read-only access.

## Verification

- `cargo test -p trustedge-trst-cli --test security_key_file_protection test_wrap_autogen_key_permissions_0600` passes
- `cargo test -p trustedge-core --lib -- asymmetric` — 6/6 tests pass
- `cargo build --workspace` — clean build, no errors
- `cargo clippy --workspace -- -D warnings` — no warnings
- `cargo fmt --check` — clean

## Deviations from Plan

### Auto-fixed Issues

None - plan executed exactly as written, with one discretionary extension.

**Additional field restriction:** The plan noted "Per discretion: Also make `algorithm` and `key_id` fields on PrivateKey `pub(crate)` for consistency" — this was implemented as specified, restricting all three PrivateKey fields to `pub(crate)`. The experimental pubky crate accesses only `PublicKey.key_bytes` (not `PrivateKey`), so no external breakage occurred.

## Known Stubs

None. Both security fixes are fully implemented and verified.

## Self-Check: PASSED

- [x] `crates/trst-cli/src/main.rs` — `set_permissions.*secret_path` present (confirmed grep count: 2 for keygen + wrap)
- [x] `crates/core/src/asymmetric.rs` — `pub(crate) key_bytes` present, no `Serialize`/`Deserialize` on PrivateKey
- [x] `crates/trst-cli/tests/security_key_file_protection.rs` — `test_wrap_autogen_key_permissions_0600` present
- [x] Commits c6a54dc, d76b02c, cbdb229 exist in git log
