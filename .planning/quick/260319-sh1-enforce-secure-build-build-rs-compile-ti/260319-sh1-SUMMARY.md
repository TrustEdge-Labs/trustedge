---
phase: 260319-sh1
plan: 01
subsystem: infra
tags: [build.rs, cargo, insecure-tls, security, compile-time]

requires: []
provides:
  - "Compile-time guard blocking release+insecure-tls build combination"
  - "ALLOW_INSECURE_TLS=1 escape hatch for CI/test use"
affects: [trustedge-core, ci]

tech-stack:
  added: []
  patterns: ["build.rs PROFILE+feature guard with env-var escape hatch"]

key-files:
  created:
    - crates/core/build.rs
  modified: []

key-decisions:
  - "Cargo auto-discovers build.rs — no build key added to Cargo.toml"
  - "Debug profile always allowed (CI uses debug for insecure-tls feature validation)"
  - "ALLOW_INSECURE_TLS=1 is the canonical override for release+insecure-tls testing"

patterns-established:
  - "Profile guard pattern: check PROFILE=='release' + feature env var + override escape hatch"

requirements-completed: [SH1]

duration: 10min
completed: 2026-03-20
---

# Quick Task 260319-sh1: Enforce Secure Build — build.rs Compile-Time Guard Summary

**build.rs guard in trustedge-core blocks `cargo build --release --features insecure-tls` with a SECURITY ERROR unless `ALLOW_INSECURE_TLS=1` is explicitly set**

## Performance

- **Duration:** ~10 min
- **Started:** 2026-03-20T00:30:00Z
- **Completed:** 2026-03-20T00:40:00Z
- **Tasks:** 2 (1 code, 1 verification)
- **Files modified:** 1

## Accomplishments

- Created `crates/core/build.rs` with MPL-2.0 header and profile+feature guard
- Verified all four build scenarios behave correctly (blocked, passes, override, no-feature)
- Confirmed CI script uses debug profile for insecure-tls — no CI changes needed
- 172 core lib tests, 12 types tests, 30 trst-protocols tests, 18 platform tests all pass

## Task Commits

1. **Task 1: Create build.rs with release+insecure-tls guard** - `d2ba04f` (feat)

## Files Created/Modified

- `/home/john/vault/projects/github.com/trustedge/crates/core/build.rs` - Compile-time guard: panics when PROFILE=release + CARGO_FEATURE_INSECURE_TLS set + ALLOW_INSECURE_TLS not "1"

## Decisions Made

- `build` key not added to Cargo.toml — Cargo auto-discovers `build.rs` at package root
- Debug profile builds always allowed; guard only fires on `release`
- `cargo:rerun-if-env-changed=ALLOW_INSECURE_TLS` ensures correct cache invalidation when env var changes without source changes

## Deviations from Plan

None — plan executed exactly as written.

## Issues Encountered

None.

## User Setup Required

None — no external service configuration required.

## Next Phase Readiness

- Guard is active immediately; no deployment steps needed
- Future insecure-tls release builds must use `ALLOW_INSECURE_TLS=1` prefix

---
*Quick task: 260319-sh1*
*Completed: 2026-03-20*
