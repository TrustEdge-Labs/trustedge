<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


---
phase: 62-config-credential-hygiene
plan: 01
subsystem: infra
tags: [security, config, postgres, docker-compose, jwt, ca]

# Dependency graph
requires: []
provides:
  - DATABASE_URL enforcement gated on debug_assertions in platform config
  - PostgreSQL not exposed to host network in docker-compose
  - CAConfigBuilder::build() panics on placeholder JWT secret outside tests
affects: [platform-server, deploy, ca]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "cfg!(debug_assertions) gate for release-only enforcement without breaking dev defaults"
    - "cfg!(test) gate in build() to allow placeholder secrets in test mode only"

key-files:
  created: []
  modified:
    - crates/platform/src/http/config.rs
    - deploy/docker-compose.yml
    - crates/platform/src/ca/mod.rs

key-decisions:
  - "Use cfg!(debug_assertions) not cfg!(not(debug_assertions)) to preserve dev defaults while enforcing release-mode requirements"
  - "Use cfg!(test) in CAConfigBuilder::build() to allow placeholder secrets in test builds without runtime env tricks"
  - "Remove 5432:5432 host port binding entirely; add docker exec comment for dev access pattern"

patterns-established:
  - "cfg!(debug_assertions) for release-only enforcement: dev builds keep convenient defaults, release builds error explicitly"
  - "cfg!(test) guard in builder methods: allows placeholder test values to pass through without test env setup"

requirements-completed: [CONF-01, CONF-02, CONF-03]

# Metrics
duration: 2min
completed: 2026-03-25
---

# Phase 62 Plan 01: Config Credential Hygiene Summary

**DATABASE_URL release enforcement via cfg!(debug_assertions), postgres host port removed from docker-compose, and CAConfigBuilder::build() panics on placeholder JWT secret outside test builds**

## Performance

- **Duration:** ~2 min
- **Started:** 2026-03-25T12:49:00Z
- **Completed:** 2026-03-25T12:51:56Z
- **Tasks:** 3
- **Files modified:** 3

## Accomplishments

- Release builds of platform-server now error with a clear message when DATABASE_URL is unset (CONF-01)
- PostgreSQL is no longer reachable from the host network — internal Docker DNS only (CONF-02)
- CAConfigBuilder::build() panics when called with the placeholder "your-secret-key" JWT secret outside test mode (CONF-03)

## Task Commits

Each task was committed atomically:

1. **Task 1: Gate DATABASE_URL fallback on debug_assertions** - `f7fa456` (fix)
2. **Task 2: Remove PostgreSQL host port exposure** - `9bda96c` (fix)
3. **Task 3: Reject placeholder JWT secret in CAConfigBuilder::build()** - `bad95fc` (fix)

## Files Created/Modified

- `crates/platform/src/http/config.rs` - DATABASE_URL fallback gated on cfg!(debug_assertions); test asserting error message presence added
- `deploy/docker-compose.yml` - Removed `ports: ["5432:5432"]` from postgres service; added dev-access comment
- `crates/platform/src/ca/mod.rs` - build() panics on placeholder JWT secret when !cfg!(test); test_caconfig_debug_redacts_jwt_secret updated to explicit test secret; test_placeholder_jwt_secret_guard_exists added

## Decisions Made

- `cfg!(debug_assertions)` correctly gates the fallback: debug builds get the convenient default, release builds must provide DATABASE_URL explicitly. No need for a separate feature flag or env var.
- `cfg!(test)` in `CAConfigBuilder::build()` allows test fixtures that rely on `CAConfig::builder().build()` defaults to pass without supplying a secret, while non-test code that forgets to set a secret panics immediately at startup.
- Host port exposure in docker-compose is removed entirely (not just commented out) to prevent accidental re-enablement.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Plan verification command omitted `http` feature flag**

- **Found during:** Task 1 (DATABASE_URL enforcement)
- **Issue:** Plan specified `cargo test -p trustedge-platform --lib --features postgres` but config.rs is under the `http` feature gate. Test only appears in output with `--features "postgres,http"`.
- **Fix:** Verified with both feature flags; tests pass with `--features "postgres,http"` (19 tests) and the postgres-only run still passes (18 tests, skips http module).
- **Files modified:** None — verification command adjusted in execution, plan text not modified.
- **Committed in:** f7fa456 (Task 1 commit)

---

**Total deviations:** 1 auto-noted (verification command required additional feature flag)
**Impact on plan:** No scope change. All acceptance criteria met.

## Issues Encountered

None — all three tasks executed cleanly. Clippy passes with no new warnings.

## Next Phase Readiness

- P0 credential hygiene findings (4, 5, 7) closed
- Platform server will fail fast in release builds if DATABASE_URL is missing
- Ready for Phase 63 (or next milestone planning)

---
*Phase: 62-config-credential-hygiene*
*Completed: 2026-03-25*
