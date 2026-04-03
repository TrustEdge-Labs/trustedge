---
phase: 76-cli-platform-endpoint
plan: "01"
subsystem: trst-cli
tags: [cli, attestation, sbom, point-attestation]
dependency_graph:
  requires: [phase-75-core-attestation-library]
  provides: [trst-attest-sbom-cmd, trst-verify-attestation-cmd]
  affects: [crates/trst-cli]
tech_stack:
  added: []
  patterns: [CliExitError-exit-codes, handle_*-subcommand-pattern, unencrypted-key-flag]
key_files:
  created: []
  modified:
    - crates/trst-cli/src/main.rs
    - crates/trst-cli/tests/acceptance.rs
decisions:
  - "exit code 10 for both bad signature and hash mismatch (unified per D-09/D-16)"
  - "device_pub arg accepts inline ed25519:... string or file path (resolved at runtime)"
  - "attest-sbom loads keypair using unencrypted-first logic (skips encryption check when --unencrypted set)"
metrics:
  duration_minutes: 10
  completed_date: "2026-04-02"
  tasks_completed: 2
  files_modified: 2
---

# Phase 76 Plan 01: CLI attest-sbom and verify-attestation Summary

**One-liner:** Added `trst attest-sbom` and `trst verify-attestation` CLI subcommands that create and locally verify SBOM attestation documents using PointAttestation from Phase 75.

## Tasks Completed

| # | Task | Commit | Files |
|---|------|--------|-------|
| 1 | Add AttestSbom and VerifyAttestation subcommands | 1a4fd78 | crates/trst-cli/src/main.rs |
| 2 | Add acceptance tests for attest-sbom and verify-attestation | b5254a5 | crates/trst-cli/tests/acceptance.rs |

## What Was Built

### `trst attest-sbom`

New subcommand that:
- Validates binary file (rejects 0-byte, rejects >256 MB)
- Validates SBOM is valid JSON (rejects non-JSON with clear error)
- Loads device keypair (supports `--unencrypted` flag, encrypted key with passphrase prompt)
- Calls `PointAttestation::create()` with binary as subject and SBOM as evidence
- Writes `.te-attestation.json` to `--out` path (default: `attestation.te-attestation.json`)
- Sets output file permissions to 0644 on Unix
- Prints public key, subject hash, evidence hash to stderr

### `trst verify-attestation`

New subcommand that:
- Reads `.te-attestation.json` and parses it via `PointAttestation::from_json()`
- Resolves `--device-pub`: accepts inline `ed25519:...` string or path to `.pub` file
- Calls `PointAttestation::verify_signature()`
- Optionally verifies BLAKE3 file hashes with `--binary` and `--sbom` flags
- Prints human-readable output (format, key, timestamp, subject, evidence, VERIFIED/FAILED)
- Exit codes: 0=success, 1=IO/JSON error, 10=signature failed or hash mismatch

## Acceptance Tests (8 tests)

All 8 tests pass via `cargo test -p trustedge-trst-cli --test acceptance -- attest_sbom verify_attestation`:

1. `test_attest_sbom_creates_attestation_file` — creates file with v1 format and ed25519 key
2. `test_attest_sbom_default_output_name` — default output is `attestation.te-attestation.json`
3. `test_attest_sbom_rejects_zero_byte_binary` — exit 1, stderr contains "0 bytes" or "empty"
4. `test_attest_sbom_rejects_non_json_sbom` — exit 1, stderr contains "not valid JSON"
5. `test_attest_sbom_valid_inputs_succeed` — happy path with small valid binary (confirms 256 MB check does not fire)
6. `test_verify_attestation_success` — exit 0, stdout contains "VERIFIED"
7. `test_verify_attestation_wrong_key_fails` — exit 10, stdout contains "FAILED"
8. `test_verify_attestation_with_file_hashes` — pass with matching files, exit 10 after binary tamper

## Deviations from Plan

None — plan executed exactly as written.

## Self-Check: PASSED

- `/home/john/vault/projects/github.com/trustedge/crates/trst-cli/src/main.rs` — modified, committed 1a4fd78
- `/home/john/vault/projects/github.com/trustedge/crates/trst-cli/tests/acceptance.rs` — modified, committed b5254a5
- Both commits verified in git log
- `cargo build -p trustedge-trst-cli` exits 0
- `cargo clippy -p trustedge-trst-cli -- -D warnings` exits 0
- `cargo fmt --check -p trustedge-trst-cli` exits 0
- All 8 acceptance tests pass
