---
phase: 85-code-sweep-headers-text-metadata
plan: "02"
subsystem: experimental-crypto
tags: [rebrand, crypto, byte-literals, clean-break, d-02-tests]
dependency_graph:
  requires: []
  provides: [REBRAND-05-experimental-byte-literals]
  affects: [crates/experimental/pubky-advanced]
tech_stack:
  added: []
  patterns: [D-02 clean-break shadow-const test pattern (Phase 84 carry-forward)]
key_files:
  created: []
  modified:
    - crates/experimental/pubky-advanced/src/keys.rs
    - crates/experimental/pubky-advanced/src/envelope.rs
    - crates/experimental/pubky-advanced/examples/hybrid_encryption_demo.rs
decisions:
  - "D-02 shadow consts (OLD_X25519_DERIVATION, OLD_V2_SESSION_KEY) live only in #[cfg(test)] modules — zero production footprint for legacy values"
  - "Audio demo (TRUSTEDGE_AUDIO_V2) received plain rename only per CONTEXT.md D-02 category — no test treatment needed"
metrics:
  duration: "~15 minutes"
  completed: "2026-04-19T11:30:47Z"
  tasks_completed: 2
  tasks_total: 2
  files_modified: 3
---

# Phase 85 Plan 02: Experimental Pubky-Advanced Crypto Byte Literal Rename Summary

Renamed 3 byte-literal constants in `crates/experimental/pubky-advanced/` from the `TRUSTEDGE_*` brand to `SEALEDGE_*`, with full D-02 clean-break test treatment for the 2 crypto-meaningful domain separators and a plain rename for the demo-only audio header.

## Constant Renames (Before → After)

| File | Line | Old Value | New Value | Category |
|------|------|-----------|-----------|----------|
| `crates/experimental/pubky-advanced/src/keys.rs` | 132 | `b"TRUSTEDGE_X25519_DERIVATION"` | `b"SEALEDGE_X25519_DERIVATION"` | Crypto — BLAKE3 X25519-from-Ed25519 domain tag |
| `crates/experimental/pubky-advanced/src/envelope.rs` | 251 | `b"TRUSTEDGE_V2_SESSION_KEY"` | `b"SEALEDGE_V2_SESSION_KEY"` | Crypto — HKDF-SHA256 V2 hybrid-envelope session-key info prefix |
| `crates/experimental/pubky-advanced/examples/hybrid_encryption_demo.rs` | 149 | `b"TRUSTEDGE_AUDIO_V2"` | `b"SEALEDGE_AUDIO_V2"` | Demo only — plain rename, no test treatment |

## New D-02 Clean-Break Tests (4 total)

### `crates/experimental/pubky-advanced/src/keys.rs` — `mod clean_break_x25519_tests`

- `test_old_x25519_derivation_produces_distinct_key` — KAT: BLAKE3 hashes of old vs new tag + fixed 32-byte Ed25519 input are distinct 32-byte values
- `test_old_x25519_derivation_rejected_cleanly` — Rejection: X25519 static secrets derived under old vs new tag differ, so ECDH cannot silently succeed across domain boundary

Shadow const: `const OLD_X25519_DERIVATION: &[u8] = b"TRUSTEDGE_X25519_DERIVATION"` (test-only, zero production footprint)

### `crates/experimental/pubky-advanced/src/envelope.rs` — `mod clean_break_v2_session_key_tests`

- `test_old_v2_session_key_produces_distinct_okm` — KAT: HKDF-SHA256 OKMs for old vs new info prefix are distinct 32-byte AEAD keys
- `test_old_v2_session_key_rejected_cleanly` — Rejection: AEAD session keys derived under old vs new prefix differ, so legacy ciphertexts fail tag verification under new keys

Shadow const: `const OLD_V2_SESSION_KEY: &[u8] = b"TRUSTEDGE_V2_SESSION_KEY"` (test-only, zero production footprint)

## Legacy Byte-Literal Occurrences (Post-Rename)

| Legacy Constant | Hits in Production Code | Hits in Test Modules |
|-----------------|------------------------|----------------------|
| `b"TRUSTEDGE_X25519_DERIVATION"` | 0 | 1 (shadow const in `clean_break_x25519_tests`) |
| `b"TRUSTEDGE_V2_SESSION_KEY"` | 0 | 1 (shadow const in `clean_break_v2_session_key_tests`) |
| `b"TRUSTEDGE_AUDIO_V2"` | 0 | 0 (plain rename, no shadow) |

## Commit

`41c4dc7` — `refactor(85-02): rename experimental pubky-advanced crypto byte literals — TRUSTEDGE* → SEALEDGE*`

3 files changed, 104 insertions(+), 3 deletions(-)

## Validation Evidence

| Check | Result |
|-------|--------|
| `cd crates/experimental && cargo fmt --check` | exit 0 |
| `cd crates/experimental && cargo clippy --workspace --all-targets -- -D warnings` | exit 0 |
| `cd crates/experimental && cargo check --workspace --locked` | exit 0 (via clippy check) |
| `cd crates/experimental && cargo test --workspace --locked` | exit 0 — 14 tests pass including all 4 new D-02 tests |
| `cargo run --example hybrid_encryption_demo` | round-trip seal/unseal succeeds; preview bytes confirm SEALEDGE_AUDIO_V2 in output |
| Root workspace `cargo check --workspace --locked` | Not run separately — experimental workspace is separate; root was not modified |

Test output showing 4 new D-02 tests passing:
```
test keys::clean_break_x25519_tests::test_old_x25519_derivation_rejected_cleanly ... ok
test keys::clean_break_x25519_tests::test_old_x25519_derivation_produces_distinct_key ... ok
test envelope::clean_break_v2_session_key_tests::test_old_v2_session_key_produces_distinct_okm ... ok
test envelope::clean_break_v2_session_key_tests::test_old_v2_session_key_rejected_cleanly ... ok
```

## Deviations from Plan

None — plan executed exactly as written. All 3 renames landed in one atomic commit per plan specification. `hkdf` and `sha2` were already in `[dependencies]` (not just dev-dependencies) so no Cargo.toml edits were needed.

## Threat Flags

None — no new network endpoints, auth paths, file access patterns, or schema changes introduced. This plan only renames domain-separation byte literals and adds test-only shadow consts.

## Self-Check: PASSED

- `crates/experimental/pubky-advanced/src/keys.rs` — FOUND, contains `b"SEALEDGE_X25519_DERIVATION"` at production site, shadow const in test module only
- `crates/experimental/pubky-advanced/src/envelope.rs` — FOUND, contains `b"SEALEDGE_V2_SESSION_KEY"` at production site, shadow const in test module only
- `crates/experimental/pubky-advanced/examples/hybrid_encryption_demo.rs` — FOUND, contains `b"SEALEDGE_AUDIO_V2"`
- Commit `41c4dc7` — FOUND in git log
