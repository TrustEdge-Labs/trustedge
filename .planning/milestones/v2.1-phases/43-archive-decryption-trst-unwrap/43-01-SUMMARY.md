<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


---
phase: 43-archive-decryption-trst-unwrap
plan: "01"
subsystem: crypto-archive
tags: [hkdf, encryption, trst-cli, key-derivation]
dependency_graph:
  requires: []
  provides: [derive_chunk_key, secret_bytes, nonce-prepended-chunks]
  affects: [trustedge-core, trustedge-trst-cli]
tech_stack:
  added: [HKDF-SHA256 chunk key derivation in crypto.rs]
  patterns: [HKDF-SHA256 with empty salt + domain tag, nonce-prepended ciphertext format]
key_files:
  created: []
  modified:
    - crates/core/src/crypto.rs
    - crates/core/src/lib.rs
    - crates/trst-cli/src/main.rs
decisions:
  - "HKDF-SHA256 with empty salt acceptable because Ed25519 device key is already high-entropy IKM"
  - "Domain tag TRUSTEDGE_TRST_CHUNK_KEY separates chunk key derivation from other uses"
  - "BLAKE3 manifest hashes cover nonce+ciphertext (not ciphertext only) to match what validate_archive reads from disk"
metrics:
  duration_seconds: 2072
  completed_date: "2026-03-17"
  tasks_completed: 2
  files_modified: 3
---

# Phase 43 Plan 01: HKDF Chunk Key Derivation and Nonce-Prepended Archives Summary

HKDF-SHA256 chunk key derivation replaces the hardcoded demo key in trst wrap; chunk files now store [nonce:24][ciphertext:N] for future unwrap decryption.

## Tasks Completed

| Task | Name | Commit | Key Files |
|------|------|--------|-----------|
| 1 | Add derive_chunk_key() and secret_bytes() to crypto.rs | 07d7113 | crates/core/src/crypto.rs, crates/core/src/lib.rs |
| 2 | Update handle_wrap() to use HKDF key and prepend nonces | d6efacf | crates/trst-cli/src/main.rs |

## What Was Built

**Task 1 - derive_chunk_key() + secret_bytes():**
- `pub fn derive_chunk_key(device_secret_bytes: &[u8; 32]) -> chacha20poly1305::Key` added to crypto.rs
- Uses `Hkdf::<Sha256>::new(None, device_secret_bytes)` then `expand(b"TRUSTEDGE_TRST_CHUNK_KEY", &mut okm)`
- `pub fn secret_bytes(&self) -> &[u8; 32]` accessor added to `DeviceKeypair`
- `derive_chunk_key` re-exported from trustedge-core lib.rs
- 3 unit tests: deterministic, different-inputs, secret_bytes round-trip — all pass

**Task 2 - Updated handle_wrap():**
- Replaced `Key::from(*b"0123456789abcdef0123456789abcdef")` with `derive_chunk_key(device_keypair.secret_bytes())`
- Each chunk now stored as `[nonce:24][ciphertext:N]` via `chunk_with_nonce`
- BLAKE3 manifest hashes now cover `nonce+ciphertext` to match what `validate_archive` reads from disk
- Removed unused `use chacha20poly1305::Key` import
- All 19 acceptance tests pass with the new format

## Verification Results

- `cargo test -p trustedge-core --lib -- test_derive_chunk_key test_secret_bytes`: 3/3 pass
- `cargo test -p trustedge-trst-cli --test acceptance`: 19/19 pass
- `cargo clippy -p trustedge-core -p trustedge-trst-cli -- -D warnings`: clean
- Hardcoded key `0123456789abcdef0123456789abcdef` removed from trst-cli/src/main.rs

## Deviations from Plan

None - plan executed exactly as written.

## Self-Check: PASSED

Files exist:
- FOUND: crates/core/src/crypto.rs
- FOUND: crates/core/src/lib.rs
- FOUND: crates/trst-cli/src/main.rs

Commits exist:
- FOUND: 07d7113 (feat(43-01): add derive_chunk_key() and secret_bytes() to crypto.rs)
- FOUND: d6efacf (feat(43-01): update handle_wrap() with HKDF key derivation and nonce-prepended chunks)
