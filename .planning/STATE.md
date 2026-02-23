<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-02-22)

**Core value:** A single, reliable trustedge-core library that owns all cryptographic operations — thin CLIs and WASM bindings are just frontends.
**Current focus:** v1.7 Security & Quality Hardening — Phase 34 in progress

## Current Position

Phase: 34 of 34 (Platform Testing) — IN PROGRESS
Plan: 2 of ? in phase 34
Status: Plan 34-02 complete (router parity refactor: build_base_router extracted, create_test_app delegates to create_router; 4 new HTTP integration tests: CORS parity, verify round-trip, JWKS receipt verification, wrong-key failure)
Last activity: 2026-02-22 — executed 34-02 (router parity and verify round-trip tests)

Progress: [█████████░] 90%

## Performance Metrics

**Cumulative (all milestones):**
- v1.0: 8 phases, 17 plans, 31 tasks
- v1.1: 4 phases, 6 plans, 8 tasks
- v1.2: 2 phases, 4 plans, 9 tasks
- v1.3: 4 phases, 5 plans, 7 tasks
- v1.4: 5 phases, 5 plans, 10 tasks
- v1.5: 4 phases, 8 plans, 16 tasks
- v1.6: 3 phases, 6 plans, 11 tasks
- **v1.7 so far: 5 phases, 11 plans, 18 tasks**
- **Total: 35 phases, 62 plans, 110 tasks**

## Accumulated Context

### Decisions

- **34-02:** build_base_router returns Router<AppState> (unfinalized) so create_router can add postgres routes before with_state
- **34-02:** create_test_app delegates entirely to create_router — no duplicated route definitions, middleware stack identical to production
- **34-02:** CORS parity test uses two independent router instances from cloned AppState, not test/prod path comparison
- **34-02:** JWS JWKS 'x' field uses standard base64 (not url-safe) — matches jwks.rs BASE64.encode convention
- **34-02:** test_verify_wrong_key expects HTTP 200 with passed=false (server returns result, not error status)
- **34-01:** Axum returns 422 Unprocessable Entity (not 400) for JSON extraction failures — test asserts UNPROCESSABLE_ENTITY
- **34-01:** OnceLock<Mutex<()>> pattern serializes env-var tests to prevent parallel-thread races on PORT variable
- **34-01:** serde_json added as explicit dev-dep in platform-server — integration test binaries don't inherit transitive deps
- **34-01:** Tests run with --no-default-features to exercise verify-only code paths (no db_pool in AppState)
- **33-02:** CorsLayer::new() used for verify-only build — tower-http default denies all cross-origin, no explicit deny config needed
- **33-02:** CA api.rs validate functions return CAError instead of String — cleaner for library callers, consistent with module error type
- **33-02:** Removed #[cfg(feature = "http")] gate on pub mod api — api.rs no longer imports axum, compiles with just ca feature
- **33-01:** validate_verify_request_full: 4 checks ordered (segments, device_pub, manifest, hash format) — first-error-wins, matches prior inline handler behavior
- **33-01:** build_receipt_if_requested: manifest_digest_fn parameter avoids feature-flag coupling in validation.rs
- **33-01:** postgres handler retains inline receipt construction — DB storage interleaving makes extraction non-trivial
- **33-01:** Feature-gated imports in handlers.rs to prevent unused-import warnings under each feature combination
- **31-01:** Implemented Secret<T> in-house rather than adding secrecy crate — zeroize already a workspace dep, API surface is small
- **31-01:** Used derive(Zeroize, ZeroizeOnDrop) rather than manual impl — cleaner and less error-prone
- **31-01:** No Display/Deref/Serialize on Secret<T> — using {} or serde is a compile error by design
- **31-02:** Builder pattern chosen over public struct fields — prevents accidental bypass of Secret<T> wrapping
- **31-02:** pin() and default_passphrase() getters return &str via expose_secret() — minimal exposure surface
- **31-02:** Stale pkcs11_module_path/slot fields in platform CA service removed (auto-fix Rule 1)
- **31-03:** LoginRequest uses custom Deserialize via private LoginRequestRaw — password wrapped in Secret at JSON parsing boundary
- **31-03:** CAConfig builder added alongside Default impl — guides callers to use builder, preventing direct struct literal construction
- **31-03:** CI Step 23 uses grep -B2 on struct declarations — catches Serialize derive and missing [REDACTED] on all 4 secret-holding structs
- **32-01:** Deleted facade crates immediately — they were not published to crates.io so no yanking needed; git history preserves them
- **32-02:** Experimental workspace uses no [workspace.dependencies] — each crate pins explicit versions to avoid coupling
- **32-02:** rsa retained in root workspace.dependencies — trustedge-core/asymmetric.rs uses it directly (not pubky-only)
- **32-02:** Tier 1/Tier 2 classification replaced with flat list + experimental note pointing to crates/experimental/
- **32-03:** MIGRATION.md retains historical crate names in migration guidance (educational references, not active; not scanned by CI)
- **32-03:** ci.yml --workspace flag covers trustedge-types and trustedge-platform (were missing from old explicit -p list)

### Pending Todos

None.

### Blockers/Concerns

- Hardware tests require physical YubiKey 5 series (carried from prior milestones)
- RSA Marvin Attack advisory (RUSTSEC-2023-0071) risk-accepted (carried from v1.3)

## Session Continuity

Last session: 2026-02-22
Stopped at: Completed 34-02-PLAN.md (router parity refactor and verify round-trip tests — build_base_router extracted, 11 integration tests total)
Resume at: Continue phase 34

---
*Last updated: 2026-02-22 after executing 34-02*
