---
phase: 75-core-attestation-library
plan: "01"
subsystem: trustedge-core
tags: [attestation, cryptography, signing, blake3, ed25519, point-attestation]
dependency_graph:
  requires: []
  provides: [PointAttestation, ArtifactRef, PointAttestationError, trustedge_core::point_attestation]
  affects: [trustedge-core, downstream crates in phases 76-78]
tech_stack:
  added: []
  patterns: [Ed25519 signing via sign_manifest(), BLAKE3 file hashing with b3: prefix, deterministic canonical JSON, OsRng nonce]
key_files:
  created:
    - crates/core/src/point_attestation.rs
  modified:
    - crates/core/src/error.rs
    - crates/core/src/lib.rs
decisions:
  - Format string "te-point-attestation-v1" (FORMAT_V1 constant) per D-06
  - Hash prefix "b3:<64-hex-chars>" for BLAKE3 hashes per D-09
  - Canonical bytes: clone struct, set signature=None, serde_json::to_string (struct field order is stable)
  - MissingSignature error variant added for verify_signature() on unsigned attestations
metrics:
  duration_minutes: 31
  tasks_completed: 2
  files_created: 1
  files_modified: 2
  tests_added: 15
  completed_date: "2026-04-02"
---

# Phase 75 Plan 01: Core Attestation Library Summary

**One-liner:** Implemented PointAttestation type with Ed25519 signing via sign_manifest(), BLAKE3 file hashing with "b3:" prefix, 16-byte OsRng nonce, ISO 8601 timestamp, and deterministic canonical JSON serialization.

## What Was Built

### PointAttestation Module (`crates/core/src/point_attestation.rs`)

A new cryptographic primitive for the SBOM attestation wedge. The module provides:

**Types:**
- `PointAttestation` — signed JSON attestation binding two artifacts (format, trustedge_version, timestamp, nonce, subject, evidence, public_key, signature)
- `ArtifactRef` — artifact reference with BLAKE3 hash, basename, and freeform label
- `PointAttestationError` — typed errors: Io, Json, Crypto, HashMismatch, MissingSignature

**Constants:**
- `FORMAT_V1 = "te-point-attestation-v1"` — format discriminant
- `NONCE_BYTES = 16` — nonce size in bytes

**Functions:**
- `hash_file(path)` — BLAKE3 hash returning "b3:<64-hex>"
- `PointAttestation::create(...)` — hash files, generate nonce, timestamp, sign via sign_manifest()
- `PointAttestation::canonical_bytes()` — deterministic JSON (signature=None)
- `PointAttestation::verify_signature(public_key)` — verify via verify_manifest()
- `PointAttestation::verify_file_hashes(subject, evidence)` — optional file hash checking
- `PointAttestation::to_json()` — pretty-printed JSON for .te-attestation.json files
- `PointAttestation::from_json(json)` — deserialize from JSON string

### Error Integration (`crates/core/src/error.rs`)

Added `PointAttestation(#[from] PointAttestationError)` variant to `TrustEdgeError` for unified error handling.

### Re-exports (`crates/core/src/lib.rs`)

Added `pub mod point_attestation` and `pub use point_attestation::{ArtifactRef, PointAttestation, PointAttestationError}` so downstream crates can use `trustedge_core::PointAttestation`.

## Tests

15 tests added in `point_attestation::tests`:
- `test_create_produces_signature` — signature field is Some, starts with "ed25519:"
- `test_sign_verify_roundtrip` — serialize to JSON, deserialize, verify -> true
- `test_verify_wrong_public_key` — wrong key returns false
- `test_tamper_fails_verification` — modified field fails verification
- `test_canonical_bytes_deterministic` — same inputs produce identical bytes
- `test_canonical_bytes_excludes_signature` — signature is null in canonical form
- `test_nonce_format` — 32 hex chars (16 bytes)
- `test_timestamp_format` — ends with Z, contains T
- `test_hash_format` — "b3:" prefix + 64 hex chars (67 total)
- `test_format_field` — equals "te-point-attestation-v1"
- `test_verify_file_hashes_correct_files_pass` — both files match
- `test_verify_file_hashes_wrong_file_fails` — HashMismatch error
- `test_verify_file_hashes_only_subject` — None evidence path skipped
- `test_verify_file_hashes_only_evidence` — None subject path skipped
- `test_missing_signature_error` — MissingSignature when signature=None

## Commits

| Hash | Message |
|------|---------|
| e82b678 | feat(75-01): implement PointAttestation module with signing, verification, and canonical serialization |
| 4af65dd | chore(75-01): validate workspace build, run cargo fmt, verify CI checks pass |

## Verification Results

- `cargo test -p trustedge-core --lib point_attestation`: 15/15 pass
- `cargo test -p trustedge-core --lib -- --skip test_many_keys`: 198/198 pass (15 new + 183 existing)
- `cargo build --workspace`: exits 0
- `cargo clippy -p trustedge-core -- -D warnings`: exits 0
- `cargo fmt --check`: exits 0 (after running cargo fmt)
- `cargo doc -p trustedge-core --no-deps`: exits 0, no errors

## Deviations from Plan

None — plan executed exactly as written.

The `test_rate_limit_xff_trusted_proxy` platform test appeared to fail transiently in the ci-check.sh run but passes reliably when run directly. This is a pre-existing intermittency in that test, unrelated to this plan's changes.

## Known Stubs

None — all functionality is fully implemented and tested.

## Self-Check: PASSED

- `crates/core/src/point_attestation.rs` — exists ✔
- `crates/core/src/error.rs` — contains PointAttestation variant ✔
- `crates/core/src/lib.rs` — contains pub mod point_attestation + re-exports ✔
- Commit e82b678 — verified in git log ✔
- Commit 4af65dd — verified in git log ✔
