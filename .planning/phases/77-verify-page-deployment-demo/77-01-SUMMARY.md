---
phase: "77"
plan: "01"
subsystem: platform-http
tags: [verify-page, static-html, axum, include_str]
dependency_graph:
  requires: [76-02]
  provides: [PLAT-02, PLAT-03]
  affects: [crates/platform, web/verify]
tech_stack:
  added: []
  patterns: [include_str compile-time embedding, axum Html responder]
key_files:
  created:
    - web/verify/index.html
    - crates/platform/src/http/static_files.rs
  modified:
    - crates/platform/src/http/router.rs
    - crates/platform/src/http/mod.rs
decisions:
  - Use include_str! to embed HTML at compile time — no runtime file dependency
  - GET /verify added to build_base_router (public, unthrottled, same-origin)
  - No CORS changes needed — page is served from same origin as API
  - AbortController with 30s timeout for network resilience
  - Client-side attestation JSON parsing for richer evidence display
metrics:
  duration_minutes: 5
  completed_date: "2026-04-03"
  tasks_completed: 2
  tasks_total: 2
  files_changed: 4
---

# Phase 77 Plan 01: Verify Page Summary

**One-liner:** Self-contained HTML attestation verifier served at GET /verify via compile-time include_str! embedding in the platform binary.

## What Was Built

A static HTML verification page (`web/verify/index.html`) that lets anyone upload a `.te-attestation.json` file and see the cryptographic verification result in a browser. The page is embedded into the platform binary at compile time using `include_str!` and served at `GET /verify` as the same origin as the API — no CORS issues.

## Tasks Completed

| Task | Name | Commit | Files |
|------|------|--------|-------|
| 1 | Create static HTML verify page | 925d286 | web/verify/index.html |
| 2 | Serve verify page from platform server | 6ee8266 | crates/platform/src/http/static_files.rs, router.rs, mod.rs |

## Decisions Made

- **include_str! embedding**: The HTML is compiled into the platform binary directly. No runtime file path dependency, no tower-http serve-dir needed. Works identically in Docker or local builds.
- **build_base_router placement**: GET /verify added alongside /healthz and /.well-known/jwks.json — public, unthrottled, no auth. Appropriate since it's a static page.
- **Same-origin CORS**: Because the page is served from the platform binary itself, fetch() calls to /v1/verify-attestation are same-origin. The existing restrictive CorsLayer for non-postgres builds is unchanged.
- **Client-side attestation parsing**: The page parses the uploaded attestation JSON client-side to extract evidence label, hash, and size_bytes for richer display beyond what the API response includes.

## Verification Results

- `cargo build -p trustedge-platform --features http`: PASS
- `cargo clippy -p trustedge-platform --features http -- -D warnings`: PASS (0 warnings)
- `cargo fmt --check`: PASS
- `cargo test -p trustedge-platform --lib`: 18/18 passed (no regressions)
- web/verify/index.html: 292 lines, contains fetch to /v1/verify-attestation, AbortController, all error states

## Deviations from Plan

None — plan executed exactly as written.

## Known Stubs

None. The verify page is fully wired to the /v1/verify-attestation endpoint.

## Self-Check: PASSED

- web/verify/index.html: FOUND
- crates/platform/src/http/static_files.rs: FOUND
- Commit 925d286: FOUND
- Commit 6ee8266: FOUND
