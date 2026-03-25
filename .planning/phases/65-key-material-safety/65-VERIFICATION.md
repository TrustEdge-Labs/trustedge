---
phase: 65-key-material-safety
verified: 2026-03-25T23:30:00Z
status: passed
score: 3/3 must-haves verified
re_verification: false
---

# Phase 65: Key Material Safety Verification Report

**Phase Goal:** Key files get restrictive permissions; PrivateKey cannot be accidentally serialized
**Verified:** 2026-03-25T23:30:00Z
**Status:** passed
**Re-verification:** No — initial verification

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | Auto-generated key files from trst wrap have 0600 Unix permissions | VERIFIED | `load_or_generate_keypair()` in main.rs lines 1132-1146 sets `set_permissions(&secret_path, perms)` with `0o600` after both encrypted and unencrypted write paths |
| 2 | PrivateKey cannot be serialized to JSON or bincode by external consumers | VERIFIED | `PrivateKey` struct at line 30 derives only `#[derive(Clone, Zeroize)]`; no `Serialize`/`Deserialize`; `to_bytes()`/`from_bytes()` methods removed from PrivateKey impl |
| 3 | PrivateKey key_bytes field is not directly accessible outside trustedge-core | VERIFIED | Line 36: `pub(crate) key_bytes: Vec<u8>`; fields `algorithm` and `key_id` also `pub(crate)`; `as_bytes()` accessor remains `pub` for read-only external access |

**Score:** 3/3 truths verified

### Required Artifacts

| Artifact | Expected | Status | Details |
|----------|----------|--------|---------|
| `crates/trst-cli/src/main.rs` | 0600 permission setting in `load_or_generate_keypair` | VERIFIED | Lines 1132-1146: `#[cfg(unix)]` block with `set_permissions(&secret_path, perms)` using `0o600`; matching block in `keygen` at lines 364-377 |
| `crates/core/src/asymmetric.rs` | PrivateKey without Serialize/Deserialize derives, pub(crate) key_bytes | VERIFIED | Line 30: `#[derive(Clone, Zeroize)]` only; line 36: `pub(crate) key_bytes`; lines 34, 39: `pub(crate) algorithm`, `pub(crate) key_id`; no `to_bytes`/`from_bytes` on PrivateKey |
| `crates/trst-cli/tests/security_key_file_protection.rs` | Permission test `test_wrap_autogen_key_permissions_0600` | VERIFIED | Test at line 41; runs `trst wrap --unencrypted` without `--device-key`, asserts `device.key` has mode `0o600`; gated with `#[cfg(unix)]` |

### Key Link Verification

| From | To | Via | Status | Details |
|------|----|-----|--------|---------|
| `crates/trst-cli/src/main.rs` | `load_or_generate_keypair` | `set_permissions` after `fs::write` | VERIFIED | Line 1136: `std::fs::set_permissions(&secret_path, perms)` present; covers both unencrypted (line 1115) and encrypted (line 1128) write paths |
| `crates/core/src/asymmetric.rs` | `PrivateKey` struct | derive macro removal and field visibility | VERIFIED | Line 30: `#[derive(Clone, Zeroize)]`; grep confirms `Serialize`/`Deserialize` appear only on `PublicKey` (line 19) |

### Data-Flow Trace (Level 4)

Not applicable — this phase modifies a CLI handler and a struct definition, not a component rendering dynamic data.

### Behavioral Spot-Checks

| Behavior | Check | Result | Status |
|----------|-------|--------|--------|
| No external crates access `PrivateKey.key_bytes` directly | `grep -rn "\.key_bytes" crates/ --include="*.rs"` filtered to non-asymmetric.rs | No results outside `asymmetric.rs` | PASS |
| No serde serialization of PrivateKey possible | `grep -rn "PrivateKey" crates/ --include="*.rs"` filtered for `serialize`/`serde_json`/`bincode` | No results | PASS |
| `to_bytes`/`from_bytes` removed from PrivateKey | `grep -n "to_bytes\|from_bytes" asymmetric.rs` on PrivateKey impl block (lines 103-137) | Neither method present in PrivateKey impl; they exist only on PublicKey | PASS |
| Commits documented in SUMMARY.md exist in git history | `git log --oneline c6a54dc d76b02c cbdb229` | All three commits found | PASS |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
|-------------|------------|-------------|--------|----------|
| KEY-01 | 65-01-PLAN.md | Auto-generated key files in `trst wrap` get 0600 Unix permissions (matching `keygen` behavior) | SATISFIED | `load_or_generate_keypair` sets `0o600` on `secret_path`; test `test_wrap_autogen_key_permissions_0600` verifies behavior |
| KEY-02 | 65-01-PLAN.md | PrivateKey serde derives removed or key_bytes field made private to prevent accidental serialization | SATISFIED | `PrivateKey` derives only `Clone` and `Zeroize`; all three fields `pub(crate)`; `to_bytes`/`from_bytes` methods removed |

### Anti-Patterns Found

None detected. Searched modified files for TODOs, placeholder returns, empty handlers, and hardcoded stubs — none present.

| File | Line | Pattern | Severity | Impact |
|------|------|---------|----------|--------|
| — | — | — | — | — |

### Human Verification Required

None. Both changes are mechanically verifiable:

- File permission setting is a direct syscall call confirmed in source
- Derive macro removal is visible in source and enforced at compile time
- Field visibility is enforced by the Rust compiler

### Gaps Summary

No gaps. All three observable truths verified, both key links wired, both requirements satisfied. The SUMMARY.md claims match what exists in the codebase.

---

_Verified: 2026-03-25T23:30:00Z_
_Verifier: Claude (gsd-verifier)_
