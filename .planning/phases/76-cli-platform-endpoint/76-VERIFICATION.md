---
phase: 76-cli-platform-endpoint
verified: 2026-04-01T00:00:00Z
status: passed
score: 10/10 must-haves verified
re_verification: false
human_verification:
  - test: "Run trst attest-sbom against a real binary and inspect the .te-attestation.json output"
    expected: "File written with correct format field, valid ed25519 public key, BLAKE3 hashes, and RFC3339 timestamp"
    why_human: "Test suite uses a temp binary with known content; visual inspection of real output confirms UX quality"
  - test: "POST a valid .te-attestation.json to a running platform server and inspect the JWS receipt"
    expected: "200 response with status=verified and a JWT-format receipt (three dot-separated base64url segments)"
    why_human: "Integration tests use tower::oneshot without a live server; real HTTP stack validation requires a running process"
  - test: "Verify REQUIREMENTS.md checkbox status for CLI-01, CLI-02, CLI-03"
    expected: "Boxes should be checked [x] since implementation is complete and tests pass"
    why_human: "REQUIREMENTS.md shows CLI-01/02/03 as unchecked (Pending) even though code and tests exist. A human should update the tracking table."
---

# Phase 76: CLI Platform Endpoint Verification Report

**Phase Goal:** Users can create and verify attestations end-to-end from the command line, and the platform server exposes a network verification endpoint that returns a JWS receipt.
**Verified:** 2026-04-01
**Status:** PASSED
**Re-verification:** No — initial verification

## Goal Achievement

### Observable Truths

| #  | Truth | Status | Evidence |
|----|-------|--------|----------|
| 1  | User can run `trst attest-sbom --binary <path> --sbom <path> --device-key <key>` and get a .te-attestation.json file | ✓ VERIFIED | `handle_attest_sbom` in main.rs lines 1393-1487; `test_attest_sbom_creates_attestation_file` passes |
| 2  | User can run `trst verify-attestation <file> --device-pub <pub>` and see verified/failed result with key, timestamp, hashes | ✓ VERIFIED | `handle_verify_attestation` in main.rs lines 1489-1554; `test_verify_attestation_success` passes |
| 3  | attest-sbom rejects 0-byte binary with clear error and exit code 1 | ✓ VERIFIED | Lines 1402-1408 in main.rs; "Error: binary file is empty (0 bytes)"; `test_attest_sbom_rejects_zero_byte_binary` passes |
| 4  | attest-sbom rejects non-JSON SBOM with clear error and exit code 1 | ✓ VERIFIED | Lines 1421-1430 in main.rs; "Error: SBOM file is not valid JSON"; `test_attest_sbom_rejects_non_json_sbom` passes |
| 5  | attest-sbom rejects binary >256MB with clear error and exit code 1 | ✓ VERIFIED | Lines 1409-1418 in main.rs; "Error: binary file exceeds 256 MB limit ({} bytes)"; logic present, code-inspection verified per plan guidance |
| 6  | verify-attestation exits 10 on bad signature, 0 on success, 1 on IO/JSON error | ✓ VERIFIED | Lines 1527-1531 (code 10); `test_verify_attestation_wrong_key_fails` asserts exit 10; `test_verify_attestation_success` asserts exit 0 |
| 7  | POST /v1/verify-attestation accepts attestation JSON and returns 200 with status verified + JWS receipt | ✓ VERIFIED | handlers.rs lines 258-271; `test_verify_attestation_valid` passes (asserts receipt starts with "ey") |
| 8  | POST /v1/verify-attestation returns 200 with status failed for bad signature | ✓ VERIFIED | handlers.rs lines 194-204; `test_verify_attestation_bad_signature` passes |
| 9  | POST /v1/verify-attestation returns 400 for malformed requests (wrong format, missing fields) | ✓ VERIFIED | Three distinct 400 paths in handlers.rs (invalid_attestation, invalid_format, missing_signature); 3 tests pass |
| 10 | Rate limiting applies to /v1/verify-attestation (same governor as /v1/verify) | ✓ VERIFIED | router.rs lines 70-76: both routes inside same `verify_router` with shared `route_layer(rate_limit_middleware)` |

**Score:** 10/10 truths verified

### Required Artifacts

| Artifact | Expected | Status | Details |
|----------|----------|--------|---------|
| `crates/trst-cli/src/main.rs` | AttestSbom and VerifyAttestation subcommands | ✓ VERIFIED | `AttestSbomCmd`, `VerifyAttestationCmd`, `handle_attest_sbom`, `handle_verify_attestation` all present; `PointAttestation::create` called at line 1450 |
| `crates/trst-cli/tests/acceptance.rs` | Acceptance tests for attest-sbom and verify-attestation | ✓ VERIFIED | 8 tests at lines 1145-1482; all 8 pass in live run |
| `crates/platform/src/http/handlers.rs` | verify_attestation_handler function | ✓ VERIFIED | Function at line 138; fully substantive (not a stub); calls `PointAttestation::from_json`, `verify_signature`, `sign_receipt_jws` |
| `crates/platform/src/http/router.rs` | Route registration for /v1/verify-attestation | ✓ VERIFIED | Line 72: `.route("/v1/verify-attestation", post(verify_attestation_handler))` inside rate-limited `verify_router` |
| `crates/platform/tests/verify_integration.rs` | Integration tests for attestation verification endpoint | ✓ VERIFIED | 5 tests at lines 1184-1330; all 5 pass in live run |

### Key Link Verification

| From | To | Via | Status | Details |
|------|-----|-----|--------|---------|
| `crates/trst-cli/src/main.rs` | `crates/core/src/point_attestation.rs` | `PointAttestation::create()` and `verify_signature()` | ✓ WIRED | Import at line 29; `PointAttestation::create` at line 1450; `verify_signature` at line 1507 |
| `crates/platform/src/http/handlers.rs` | `crates/core/src/point_attestation.rs` | `PointAttestation::from_json()` and `verify_signature()` | ✓ WIRED | `from_json` at line 143; `verify_signature` at line 179 |
| `crates/platform/src/http/handlers.rs` | `crates/platform/src/verify/signing.rs` | `sign_receipt_jws` for JWS receipt creation | ✓ WIRED | Imported at line 38 (non-postgres path) and line 34 (postgres path); called at line 247 |
| `crates/platform/src/http/router.rs` | `crates/platform/src/http/handlers.rs` | `post(verify_attestation_handler)` route registration | ✓ WIRED | Import at line 26; route at line 72 |

### Data-Flow Trace (Level 4)

| Artifact | Data Variable | Source | Produces Real Data | Status |
|----------|---------------|--------|--------------------|--------|
| `verify_attestation_handler` | `attestation` (PointAttestation) | Request body parsed via `PointAttestation::from_json(&body)` | Yes — parsed from live POST body | ✓ FLOWING |
| `verify_attestation_handler` | `jws_token` (String) | `sign_receipt_jws(&receipt_claims, &keys, state.receipt_ttl_secs)` | Yes — signed with real key material from KeyManager | ✓ FLOWING |
| `handle_attest_sbom` | `attestation` (PointAttestation) | `PointAttestation::create(&args.binary, "binary", &args.sbom, "sbom", &device_keypair)` | Yes — reads real files, computes BLAKE3 hashes, signs with device key | ✓ FLOWING |

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
|----------|---------|--------|--------|
| 8 CLI acceptance tests pass | `cargo test -p trustedge-trst-cli --test acceptance -- attest_sbom verify_attestation` | 8 passed, 0 failed | ✓ PASS |
| 5 platform integration tests pass | `cargo test -p trustedge-platform --test verify_integration --features http -- verify_attestation` | 5 passed, 0 failed | ✓ PASS |
| Build succeeds (both crates) | `cargo build -p trustedge-trst-cli -p trustedge-platform --features http` | Finished in 5.41s | ✓ PASS |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
|-------------|------------|-------------|--------|----------|
| CLI-01 | 76-01-PLAN.md | User can run `trst attest-sbom` to bind a CycloneDX JSON SBOM to a binary artifact, producing a `.te-attestation.json` file | ✓ SATISFIED | `handle_attest_sbom` implemented; `test_attest_sbom_creates_attestation_file` verifies output file exists with correct content |
| CLI-02 | 76-01-PLAN.md | User can run `trst verify-attestation` to verify an attestation document locally, with optional binary/SBOM hash checking | ✓ SATISFIED | `handle_verify_attestation` implemented with `--binary`/`--sbom` optional hash verification; `test_verify_attestation_with_file_hashes` covers the optional path |
| CLI-03 | 76-01-PLAN.md | CLI rejects 0-byte binaries, non-JSON SBOMs, and binaries >256MB with clear error messages | ✓ SATISFIED | All three rejection paths implemented (lines 1402-1430); 2 acceptance tests cover 0-byte and non-JSON cases; 256MB check verified by code inspection |
| PLAT-01 | 76-02-PLAN.md | Platform exposes `POST /v1/verify-attestation` endpoint that verifies point attestations and returns a JWS receipt | ✓ SATISFIED | Endpoint live at `/v1/verify-attestation`; returns `{ status, receipt, details }` with JWS on success; 5 integration tests pass |

**Note on REQUIREMENTS.md tracking table:** CLI-01, CLI-02, and CLI-03 show as `[ ] Pending` in REQUIREMENTS.md while PLAT-01 shows as `[x] Complete`. The CLI implementations are complete and all tests pass — this is a documentation-only discrepancy in the status table that should be corrected.

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
|------|------|---------|----------|--------|
| None found | — | — | — | No blockers or warnings |

No TODO, FIXME, placeholder, or stub patterns found in any of the phase-modified files for attestation-related code.

### Human Verification Required

#### 1. End-to-end CLI output inspection

**Test:** Run `trst keygen --unencrypted --out-key device.key --out-pub device.pub` then `trst attest-sbom --binary /usr/bin/ls --sbom sbom.json --device-key device.key --device-pub device.pub --unencrypted` and inspect the written `.te-attestation.json`
**Expected:** JSON file with `"format": "te-point-attestation-v1"`, `"public_key": "ed25519:..."`, two ArtifactRef objects with BLAKE3 hashes, and a valid base64url signature string
**Why human:** Tests verify file existence and key substrings, but visual inspection of a real attestation document against the spec confirms UX quality and field layout

#### 2. Live server JWS receipt validation

**Test:** Start `trustedge-platform-server` and POST a valid `.te-attestation.json` to `POST /v1/verify-attestation`; decode the returned JWS receipt
**Expected:** 200 response, `status: "verified"`, receipt is a valid JWT with subject/evidence hashes in claims
**Why human:** Integration tests use Axum's `tower::oneshot` (no real TCP); a live server test confirms the full HTTP stack, content-type handling, and rate limit governor behavior

#### 3. REQUIREMENTS.md checkbox correction

**Test:** Update REQUIREMENTS.md to mark CLI-01, CLI-02, CLI-03 as `[x]` (complete) and change their table status from "Pending" to "Complete"
**Expected:** All four Phase 76 requirements show as complete
**Why human:** Documentation update requires human judgment on when to mark requirements complete

### Gaps Summary

No gaps found. All 10 observable truths are verified by code inspection and live test runs. All 4 requirement IDs (CLI-01, CLI-02, CLI-03, PLAT-01) are satisfied by the implementation. The REQUIREMENTS.md status discrepancy for CLI-01/02/03 is a tracking update needed, not a code gap.

---

_Verified: 2026-04-01_
_Verifier: Claude (gsd-verifier)_
