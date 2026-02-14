---
phase: 20-dead-code-removal
plan: 01
subsystem: core
tags: [refactoring, dead-code, technical-debt]
dependency-graph:
  requires: [phase-19]
  provides: [clean-codebase]
  affects: [trustedge-server, universal-keyring, tcp-transport, software-hsm]
tech-stack:
  added: []
  patterns: [fail-fast-compilation]
key-files:
  created: []
  modified:
    - crates/core/src/bin/trustedge-server.rs
    - crates/core/src/backends/universal_keyring.rs
    - crates/core/src/transport/tcp.rs
    - crates/core/src/backends/software_hsm.rs
    - crates/core/tests/software_hsm_integration.rs
    - crates/core/src/vectors.rs
    - crates/core/src/applications/attestation/mod.rs
decisions:
  - "Remove all dead code without justification rather than annotating with #[allow(dead_code)]"
  - "Delete legacy server functions (handle_connection, process_and_decrypt_chunk, save_chunk_to_disk) that duplicated hardened handler"
  - "Remove reserved encrypt_aes_gcm/decrypt_aes_gcm from keyring backend (never implemented, unsupported operation)"
  - "Keep single #[allow(dead_code)] in attestation with documented justification (bincode deserialization layout)"
metrics:
  duration: 9m 31s
  completed: 2026-02-14
---

# Phase 20 Plan 01: Dead Code Removal Summary

Complete elimination of dead code from trustedge workspace: legacy server functions, reserved keyring methods, unused struct fields, and unjustified #[allow(dead_code)] annotations.

## What Was Built

**trustedge-server.rs (DEAD-01, DEAD-03):**
- Deleted legacy `handle_connection` function (~160 LOC)
- Deleted `process_and_decrypt_chunk` function (~125 LOC)
- Deleted `save_chunk_to_disk` function (~30 LOC)
- Removed `parse_key_hex` helper (no longer needed)
- Cleaned ProcessingSession: removed `chunks`, `cipher`, `stream_header_hash`, `connection_start` fields
- Removed unused imports: HashMap, SignedManifest, Manifest, NONCE_LEN, build_aad, Aes256Gcm, Payload, VerifyingKey, Signature, AsyncReadExt, AsyncWriteExt, serde_json, chrono

**universal_keyring.rs (DEAD-02):**
- Deleted `encrypt_aes_gcm` method (reserved, never implemented)
- Deleted `decrypt_aes_gcm` method (reserved, never implemented)
- Removed aes_gcm imports (`Aead`, `KeyInit`, `Aes256Gcm`)
- Removed rand_core imports (`OsRng`, `RngCore`)
- Updated module doc: "supporting key derivation and hash operations" (removed AES encryption/decryption)

**tcp.rs (DEAD-04):**
- Removed `connection_start: Instant` field (never read)
- Updated test assertions

**software_hsm.rs (DEAD-04):**
- Deleted `create_test_backend_with_config` helper (never called)

**software_hsm_integration.rs (DEAD-04):**
- Deleted `generate_key_pair_via_universal_backend` helper (never called)

**vectors.rs (DEAD-04):**
- Removed file-level `#![allow(dead_code)]` (functions used within test module)

**attestation/mod.rs (DEAD-04):**
- Added comment explaining justified `#[allow(dead_code)]` on `verification_key` field: "Field required for correct bincode deserialization layout"

## Deviations from Plan

None - plan executed exactly as written.

## Key Decisions Made

1. **Deleted legacy server code rather than maintaining it.** The hardened connection handler (`handle_hardened_connection`) fully replaces the legacy code, which was kept only for compatibility. Removing ~315 LOC of dead code eliminates maintenance burden.

2. **Removed reserved keyring encryption methods.** The `encrypt_aes_gcm` and `decrypt_aes_gcm` methods were never implemented - `perform_operation` returns `UnsupportedOperation` errors for encrypt/decrypt operations. Deleting these methods accurately reflects the backend's capabilities.

3. **Single justified #[allow(dead_code)] remains.** The `verification_key` field in `AttestationFile` must exist for bincode to deserialize the binary format correctly, even though the field is not explicitly read in code. Added a comment explaining this.

## Validation Results

**Build:**
```
cargo build --workspace --release
```
Zero warnings. Clean build.

**Tests:**
```
cargo test --workspace
```
All tests pass.

**Dead code audit:**
```
grep -rn "#\[allow(dead_code)\]" crates/
```
Result: Only 1 instance in `crates/core/src/applications/attestation/mod.rs` with documented justification.

**Deleted functions verified:**
- `handle_connection`: No matches
- `process_and_decrypt_chunk`: No matches
- `save_chunk_to_disk`: No matches
- `encrypt_aes_gcm`: No matches
- `decrypt_aes_gcm`: No matches
- `create_test_backend_with_config`: No matches
- `generate_key_pair_via_universal_backend`: No matches

**Code Impact:**
- ~480 lines of dead code removed
- 7 files modified
- 0 API breaking changes
- 0 test failures

## Self-Check

Verification of SUMMARY.md claims:

**Created files:**
None.

**Modified files:**
```
[✔] crates/core/src/bin/trustedge-server.rs exists
[✔] crates/core/src/backends/universal_keyring.rs exists
[✔] crates/core/src/transport/tcp.rs exists
[✔] crates/core/src/backends/software_hsm.rs exists
[✔] crates/core/tests/software_hsm_integration.rs exists
[✔] crates/core/src/vectors.rs exists
[✔] crates/core/src/applications/attestation/mod.rs exists
```

**Commits:**
```
[✔] 4803a7d: refactor(20-01): delete legacy server functions and clean ProcessingSession
[✔] 3ac7db2: refactor(20-01): remove reserved keyring functions and unused test helpers
```

## Self-Check: PASSED

All file references verified, all commits exist, all claims validated.
