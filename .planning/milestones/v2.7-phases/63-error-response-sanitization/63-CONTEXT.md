<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


# Phase 63: Error Response Sanitization - Context

**Gathered:** 2026-03-25
**Status:** Ready for planning

<domain>
## Phase Boundary

Crypto verification errors never leak raw library error messages to API clients. Clients receive a category-level message (what failed), full details stay in server-side logs only. Successful responses are unaffected.

</domain>

<decisions>
## Implementation Decisions

### Client-facing error messages
- **D-01:** Replace `format!("Cryptographic verification failed: {}", e)` with a static string `"Cryptographic verification failed"` — no raw error appended. Applies to both verify handlers (line 86 and line 168 in handlers.rs).
- **D-02:** Replace `format!("Failed to sign receipt: {}", e)` with `"Receipt generation failed"` — same pattern for the receipt signing error (line 273).
- **D-03:** Category-only approach: the client knows WHAT category of operation failed (verification vs receipt), but not WHY internally (no library error strings, key material hints, or algorithm details).

### Server-side logging
- **D-04:** Keep existing `warn!("Verification failed: {}", e)` logging unchanged. It already captures the full error detail server-side. No need to upgrade to `error!` or add structured fields — the current logging is sufficient.
- **D-05:** Verify that all three error paths have a `warn!()` call before the error return. If any is missing, add it.

### Testing
- **D-06:** Add/update integration tests that verify: (a) a failed verification returns a response body that does NOT contain the raw library error string, and (b) the response body contains only the generic category message.

### Claude's Discretion
- Whether to extract the generic messages into constants or keep them inline
- Exact test structure (new test file vs extending existing verify_integration tests)
- Whether the `ValidationError` field name stays "verification_failed" or changes

</decisions>

<canonical_refs>
## Canonical References

No external specs — requirements fully captured in decisions above and in the security review findings table.

### Files to modify
- `crates/platform/src/http/handlers.rs` — Three `format!()` calls at lines 86, 168, 273 that leak raw errors to clients (Finding 6)

### Related test files
- `crates/platform/tests/verify_integration.rs` — Existing integration tests for the verify endpoint

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `ValidationError::new(code, message)` — existing error struct used for all error responses; just change the message parameter
- `warn!()` macro from `tracing` already imported and used in all three error paths

### Established Patterns
- Error response pattern: `Err((StatusCode::BAD_REQUEST, Json(ValidationError::new("code", "message"))))` — keep this pattern, just change the message string
- `verify_handler` (non-postgres) and `verify_handler` (postgres) have identical verification error handling — keep them in sync

### Integration Points
- The verify endpoint returns `ValidationError` JSON to clients — changing the message changes what API consumers see
- The `warn!()` call writes to structured logs (tracing subscriber) — no change needed here

</code_context>

<specifics>
## Specific Ideas

No specific requirements — standard error sanitization patterns.

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope.

</deferred>

---

*Phase: 63-error-response-sanitization*
*Context gathered: 2026-03-25*
