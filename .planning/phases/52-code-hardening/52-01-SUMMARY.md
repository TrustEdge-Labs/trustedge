---
phase: 52-code-hardening
plan: "01"
subsystem: trustedge-core
tags: [crypto, security, auth, base64, key-format, timestamp]
dependency_graph:
  requires: []
  provides: [CRYP-01, CRYP-02, AUTH-01]
  affects: [crates/core/src/crypto.rs, crates/core/src/auth.rs, crates/core/Cargo.toml]
tech_stack:
  added: [base64 = "0.22"]
  patterns: [standard base64 crate, PBKDF2_MIN_ITERATIONS constant, unidirectional timestamp validation]
key_files:
  created: []
  modified:
    - crates/core/Cargo.toml
    - crates/core/src/crypto.rs
    - crates/core/src/auth.rs
decisions:
  - "Use BASE64.encode/decode from base64 crate (STANDARD engine) — standard library handles RFC 4648 compliance, padding, URL-safety variants correctly"
  - "PBKDF2_MIN_ITERATIONS named constant instead of magic 600_000 — self-documenting, citeable, single point of change"
  - "version field added to export but not required on import — backward-compatible: old files without version still decode correctly"
  - "FUTURE_TOLERANCE_SECS = 5, PAST_TOLERANCE_SECS = 300 — tight future window (clock skew), generous past window (replay protection)"
metrics:
  duration: 12m
  completed_date: "2026-03-22"
  tasks_completed: 2
  tasks_total: 2
  files_changed: 3
---

# Phase 52 Plan 01: Replace Custom Base64, Version Key Format, Fix Timestamp Check Summary

**One-liner:** Replace 60-line custom base64 implementation with standard base64 crate, add version field to encrypted key file format, and split bidirectional timestamp tolerance into asymmetric 5s/300s bounds.

## Tasks Completed

| Task | Name | Commit | Files |
|------|------|--------|-------|
| 1 | Replace custom base64, version key file format (CRYP-01, CRYP-02) | 33bf22d | crates/core/Cargo.toml, crates/core/src/crypto.rs |
| 2 | Make auth timestamp check unidirectional (AUTH-01) | b909432 | crates/core/src/auth.rs |

## What Was Done

### Task 1: Custom Base64 Replacement and Key File Versioning

Added `base64 = "0.22"` to `crates/core/Cargo.toml` and replaced the custom 60-line `base64_encode`/`base64_decode` implementation across all 23 call sites in `crypto.rs` with `BASE64.encode()` / `BASE64.decode()` from the standard base64 crate (`STANDARD` engine = RFC 4648 with padding).

Added `PBKDF2_MIN_ITERATIONS: u32 = 600_000` constant with OWASP 2023 citation, replacing the hardcoded magic number in `export_secret_encrypted`.

Added `"version": 1` to the JSON metadata block emitted by `export_secret_encrypted`. The import path (`import_secret_encrypted`) does not require the field, preserving backward compatibility with existing key files generated before this change.

### Task 2: Unidirectional Timestamp Validation

In `authenticate_client()` in `auth.rs`, replaced:
```rust
if response.timestamp.abs_diff(now) > 300 { ... }
```

With a two-part check using named constants:
- `FUTURE_TOLERANCE_SECS = 5`: rejects timestamps more than 5 seconds ahead of server time — prevents replay using a future-dated response
- `PAST_TOLERANCE_SECS = 300`: rejects timestamps more than 300 seconds in the past — replay window unchanged from previous behavior
- Uses `now.saturating_sub(response.timestamp)` for overflow safety on the past check
- Distinct error messages for each rejection path (future vs. stale) for debuggability

## Deviations from Plan

None — plan executed exactly as written. Clippy flagged 3 needless borrow warnings (`&salt` → `salt`, `&nonce_bytes` → `nonce_bytes`, `&self.secret` → `self.secret`) which were fixed inline per standard deviation rules.

## Verification Results

- `grep -c "fn base64_encode\|fn base64_decode" crates/core/src/crypto.rs` → 0 (deleted)
- `grep '"version"' crates/core/src/crypto.rs` → `"version": 1,`
- `grep -c 'abs_diff' crates/core/src/auth.rs` → 0
- `grep 'FUTURE_TOLERANCE' crates/core/src/auth.rs` → `FUTURE_TOLERANCE_SECS: u64 = 5`
- `cargo test -p trustedge-core --lib -- test_base64 test_device_keypair test_encrypted_key ...` → 19 passed, 0 failed
- `cargo clippy -p trustedge-core -- -D warnings` → clean

## Known Stubs

None.

## Self-Check: PASSED

- `crates/core/src/crypto.rs` — exists and modified
- `crates/core/src/auth.rs` — exists and modified
- `crates/core/Cargo.toml` — exists and modified
- Commit `33bf22d` — exists in git log
- Commit `b909432` — exists in git log
