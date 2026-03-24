---
phase: 59-cli-deploy-hardening
plan: 01
subsystem: cli
tags: [rust, clap, aes-256-gcm, key-management, security]

# Dependency graph
requires: []
provides:
  - trustedge CLI --show-key flag for optional key display to stderr
  - trustedge CLI key loss prevention (bail! when no --key-out or --show-key)
  - Removed unconditional AES-256 key leak to stderr
affects: [trustedge-cli, 59-02]

# Tech tracking
tech-stack:
  added: []
  patterns: [three-way key output gate: file > stderr > error]

key-files:
  created: []
  modified:
    - crates/trustedge-cli/src/main.rs

key-decisions:
  - "Three-way gate for key output: --key-out writes silently, --show-key prints to stderr, neither => actionable bail! error"
  - "show_key field added after verbose in Args struct per D-01/D-02/D-03/D-04 design decisions"

patterns-established:
  - "Key material never printed implicitly — explicit opt-in via --show-key or file via --key-out"

requirements-completed: [CLI-01]

# Metrics
duration: 5min
completed: 2026-03-24
---

# Phase 59 Plan 01: CLI Key Leak Fix Summary

**Removed unconditional AES-256 key stderr leak; replaced with three-way gate: --key-out writes silently, --show-key prints to stderr, neither causes actionable error**

## Performance

- **Duration:** ~5 min
- **Started:** 2026-03-24T13:52:00Z
- **Completed:** 2026-03-24T13:57:28Z
- **Tasks:** 1
- **Files modified:** 1

## Accomplishments
- Added `show_key: bool` field to `Args` struct with `--show-key` long arg
- Replaced `eprintln!("NOTE (demo): AES-256 key (hex) = ...")` with three-way gate
- Generating a key without `--key-out` or `--show-key` now returns a clear error referencing both flags
- `--show-key` prints the hex key to stderr when explicitly requested
- `--key-out <file>` writes key silently, nothing on stderr
- All CI checks pass: build, clippy -D warnings, tests

## Task Commits

Each task was committed atomically:

1. **Task 1: Add --show-key flag and enforce key capture in CLI** - `6dc2436` (feat)

**Plan metadata:** (docs commit follows)

## Files Created/Modified
- `crates/trustedge-cli/src/main.rs` - Added show_key field to Args, updated Mode::Encrypt arm in select_aes_key_with_backend()

## Decisions Made
- Three-way key output gate: --key-out writes silently, --show-key prints to stderr, neither => bail! with message referencing both options — ensures key is never silently lost or silently leaked

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
None.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- CLI-01 requirement satisfied: AES key no longer leaks to stderr in normal operation
- Ready for Phase 59 Plan 02 (next plan in phase)

---
*Phase: 59-cli-deploy-hardening*
*Completed: 2026-03-24*
