# Phase 76: CLI + Platform Endpoint - Context

**Gathered:** 2026-04-02
**Status:** Ready for planning

<domain>
## Phase Boundary

Add two new CLI subcommands (`trst attest-sbom`, `trst verify-attestation`) and one new platform HTTP endpoint (`POST /v1/verify-attestation`) that use the `PointAttestation` library from Phase 75. The CLI creates and verifies attestation documents locally. The platform endpoint verifies attestation documents server-side and issues JWS receipts.

This phase delivers CLI + platform. The static HTML verify page, public deployment, and demo script are Phase 77.

</domain>

<decisions>
## Implementation Decisions

### CLI: attest-sbom Subcommand
- **D-01:** Flag names are SBOM-specific: `--binary <path>`, `--sbom <path>`, `--device-key <path>`, `--device-pub <path>`, `--out <path>`. CLI maps `--binary` to PointAttestation's `subject` (label="binary") and `--sbom` to `evidence` (label="sbom") internally.
- **D-02:** `--out` defaults to `attestation.te-attestation.json` in the current directory if not specified.
- **D-03:** `--unencrypted` flag supported (same pattern as existing wrap/unwrap/keygen). Passphrase prompted if key is encrypted.
- **D-04:** Input validation: reject 0-byte binary (exit 1), non-JSON SBOM (exit 1), binary >256MB (exit 1). Error messages must be clear and actionable.
- **D-05:** SBOM is treated as opaque JSON. No CycloneDX schema validation. `serde_json::from_str::<serde_json::Value>()` check is sufficient.
- **D-06:** Output file gets 0644 permissions (not secret, contains only public data + signature).

### CLI: verify-attestation Subcommand
- **D-07:** Usage: `trst verify-attestation <attestation-path> --device-pub <pub-key-string-or-file>`. Accepts both inline "ed25519:..." string and path to .pub file.
- **D-08:** Optional `--binary <path>` and `--sbom <path>` flags for file hash verification. If provided, verifies BLAKE3 hashes match the attestation document's subject/evidence hashes.
- **D-09:** Exit codes: 0=verified, 1=general error (IO, JSON), 10=verification failed (bad signature or hash mismatch). Uses CliExitError pattern from existing trst-cli code.
- **D-10:** Output: prints human-readable verification result to stdout (key, timestamp, hashes, pass/fail status). No JSON output mode for now.

### Platform: POST /v1/verify-attestation
- **D-11:** Request body IS the attestation JSON document directly (not wrapped). Content-Type: application/json.
- **D-12:** The endpoint parses the attestation, extracts the embedded public key, verifies the signature, and returns a JWS receipt on success. Same JWKS signing as existing /v1/verify endpoint.
- **D-13:** Response shape follows existing /v1/verify pattern: 200 with `{ status: "verified"|"failed", receipt: "..." (if verified), details: {...} }`.
- **D-14:** Uses existing middleware: 2MB body limit, governor rate limiting on the verify router. New route added to `verify_router` alongside `/v1/verify`.
- **D-15:** Validation: check `format` field equals `"te-point-attestation-v1"`, all required fields present. 400 on malformed requests.

### Error Handling
- **D-16:** CLI exit codes: 0=success, 1=general error, 10=verification failed. Matches simplified scheme agreed in eng review.
- **D-17:** Platform returns 200 for both verified and failed attestations (status field distinguishes). 400 for malformed requests. 429 for rate limited. 413 for oversized body. Crypto error messages are generic (no internal details leaked).

### Claude's Discretion
- Internal organization of attest-sbom/verify-attestation handlers in main.rs (inline or split into helper functions)
- Whether to share validation logic between CLI and platform or keep separate
- JWS receipt payload structure for attestation verification (should include subject/evidence hashes for verifier consumption)

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase 75 Implementation (the library this phase consumes)
- `crates/core/src/point_attestation.rs` — PointAttestation struct, ArtifactRef, create(), verify_signature(), verify_file_hashes(), to_json(), from_json(), hash_file()

### Existing CLI Patterns
- `crates/trst-cli/src/main.rs` — Commands enum (line ~107), WrapCmd/VerifyCmd arg structs, handle_wrap() (line ~423), handle_verify() (line ~811) for subcommand pattern, CliExitError (line ~57) for exit codes
- `crates/trst-cli/src/main.rs` — `load_or_generate_keypair()` and `warn_unencrypted()` for key handling pattern

### Existing Platform Patterns
- `crates/platform/src/http/handlers.rs` — verify_handler() (line ~66 or ~120) for request handling, validation, receipt signing pattern
- `crates/platform/src/http/router.rs` — create_router() (line ~53), verify_router setup (line ~69) for route registration with rate limiting
- `crates/platform/src/http/signing.rs` — sign_receipt_jws() for JWS receipt creation

### Validation and Security
- `crates/platform/src/validation.rs` — validate_verify_request_full() for request validation pattern
- `crates/platform/src/http/handlers.rs` — generic error messages pattern (no crypto internals leaked)

### Design Documents
- `docs/designs/sbom-attestation-wedge.md` — CEO plan with CLI interface specification
- `~/.gstack/projects/TrustEdge-Labs-trustedge/john-main-design-20260401-172433.md` — Approved design doc

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `PointAttestation::create()` — call from CLI attest-sbom handler with paths and keypair
- `PointAttestation::verify_signature()` — call from both CLI verify-attestation and platform handler
- `PointAttestation::verify_file_hashes()` — call from CLI verify-attestation when --binary/--sbom provided
- `PointAttestation::from_json()` — parse attestation JSON in both CLI and platform
- `DeviceKeypair` — existing key loading, encrypted key handling, passphrase prompting
- `CliExitError` — existing exit code propagation pattern
- `sign_receipt_jws()` — existing JWS receipt signing for platform

### Established Patterns
- CLI subcommands: `#[derive(Args)]` struct + `fn handle_*()` function + `Commands` enum variant
- Platform handlers: `async fn handler(State(state), Json(body)) -> impl IntoResponse`
- Route registration: `Router::new().route("/v1/...", post(handler))` inside verify_router
- Rate limiting: applied via `route_layer` to verify_router sub-tree
- Encrypted key handling: `is_encrypted_key_file()` check, `rpassword::prompt_password()` for passphrase

### Integration Points
- `crates/trst-cli/src/main.rs` — add `AttestSbom` and `VerifyAttestation` to Commands enum
- `crates/platform/src/http/handlers.rs` — add `verify_attestation_handler()` function
- `crates/platform/src/http/router.rs` — add `.route("/v1/verify-attestation", post(verify_attestation_handler))` to verify_router
- `crates/trst-cli/tests/acceptance.rs` — add acceptance tests for new subcommands

</code_context>

<specifics>
## Specific Ideas

- The `attest-sbom` subcommand should feel like a natural extension of the existing `wrap` command. Same key handling patterns, same `--unencrypted` escape hatch, same output messaging style.
- The platform endpoint should be a lean function: parse JSON, call PointAttestation::from_json(), call verify_signature(), sign receipt, return. No new state management needed.
- Both the CLI verify-attestation and the platform endpoint verify the same thing (signature). The difference is: CLI is local (no receipt), platform issues a JWS receipt.

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope.

</deferred>

---

*Phase: 76-cli-platform-endpoint*
*Context gathered: 2026-04-02*
