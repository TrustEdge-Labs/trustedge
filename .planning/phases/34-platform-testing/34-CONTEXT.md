# Phase 34: Platform Testing - Context

**Gathered:** 2026-02-22
**Status:** Ready for planning

<domain>
## Phase Boundary

Add integration tests for the platform-server binary wiring and a full HTTP verify round-trip test. Tests validate that AppState constructs correctly, env config errors are caught, middleware parity is enforced between create_test_app and create_router, and the complete sign-then-verify pipeline works end-to-end over HTTP. No new features or API changes.

</domain>

<decisions>
## Implementation Decisions

### Test placement
- Split by concern: wiring tests in `crates/platform-server/tests/`, HTTP round-trip tests in `crates/platform/tests/verify_integration.rs`
- Round-trip test goes in the existing `verify_integration.rs` file (already has HTTP test infra and helpers)
- Wiring tests cover only non-postgres (verify-only) mode — postgres wiring requires a real DB which is out of scope
- Wiring tests should verify Config::from_env() returns meaningful errors when required env vars are missing

### Round-trip test design
- Full cryptographic receipt verification: decode JWS, verify Ed25519 signature against JWKS endpoint, confirm claims match submitted payload
- Comprehensive coverage: happy path, wrong key, empty segments, missing manifest, invalid hash format — full validation matrix
- Random Ed25519 key generation per test run (consistent with existing tests in the codebase)
- Separate test functions: `test_verify_round_trip` (happy path) and `test_verify_receipt_matches_jwks` (receipt signature validation against JWKS)

### create_test_app fidelity
- Non-postgres tests reuse `create_router()` directly (no separate test app builder needed)
- Extract a shared builder function that both `create_router` and `create_test_app` call — single source of truth for middleware stack
- Add a CORS parity test: send OPTIONS preflight to both create_test_app and create_router outputs, assert identical Access-Control-Allow-Origin, Allow-Methods, and Allow-Headers response headers

### Claude's Discretion
- Exact shared builder function signature and placement
- Test helper structure (shared fixtures, builder patterns)
- Which env vars to test for missing-config errors (inspect Config::from_env implementation)

</decisions>

<specifics>
## Specific Ideas

No specific requirements — open to standard approaches

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope

</deferred>

---

*Phase: 34-platform-testing*
*Context gathered: 2026-02-22*
