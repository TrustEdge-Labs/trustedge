<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


---
phase: 69-code-quality
plan: 01
subsystem: security
tags: [regex, lazylock, cli, unencrypted, warning, platform, trst-cli]

# Dependency graph
requires: []
provides:
  - "Static LazyLock<Regex> HASH_REGEX in validate_segment_hashes — compiled once at first use"
  - "stderr security warning for --unencrypted flag in all three trst-cli handlers"
affects: [trst-cli, platform-verify]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Use std::sync::LazyLock for module-level static regex compilation"
    - "emit security warnings via eprintln! to stderr before processing --unencrypted operations"

key-files:
  created: []
  modified:
    - crates/platform/src/verify/validation.rs
    - crates/trst-cli/src/main.rs

key-decisions:
  - "HASH_REGEX placed as module-level static above validate_segment_hashes(); Regex::new inside LazyLock::new closure"
  - "warn_unencrypted() extracted as a shared helper called from handle_keygen, handle_wrap, handle_unwrap"

patterns-established:
  - "LazyLock<Regex> for static regexes: eliminates per-request allocation without unsafe code"

requirements-completed: [QUAL-01, QUAL-02]

# Metrics
duration: 12min
completed: 2026-03-26
---

# Phase 69 Plan 01: Code Quality (Regex + Unencrypted Warning) Summary

**Static LazyLock regex eliminates per-request allocation in validate_segment_hashes; all three --unencrypted trst-cli handlers now emit a visible stderr security warning**

## Performance

- **Duration:** 12 min
- **Started:** 2026-03-26T19:36:00Z
- **Completed:** 2026-03-26T19:48:00Z
- **Tasks:** 2
- **Files modified:** 2

## Accomplishments
- Replaced per-call `Regex::new()` in `validate_segment_hashes()` with a module-level `static HASH_REGEX: LazyLock<Regex>` — compiled exactly once at first use
- Added `warn_unencrypted()` helper to trst-cli that prints a prominent stderr warning using the UTF-8 warning symbol (U+26A0) per CLAUDE.md conventions
- Warning wired into `handle_keygen`, `handle_wrap`, and `handle_unwrap` as first statement when `args.unencrypted` is true
- All 18 platform lib tests and 28 acceptance tests pass; workspace clippy clean

## Task Commits

Each task was committed atomically:

1. **Task 1: LazyLock regex in validate_segment_hashes** - `9856c35` (fix)
2. **Task 2: stderr warning for --unencrypted in trst-cli** - `4d20078` (feat)

**Plan metadata:** (final docs commit)

## Files Created/Modified
- `crates/platform/src/verify/validation.rs` - Added `use std::sync::LazyLock`, added `static HASH_REGEX: LazyLock<Regex>`, removed per-call `Regex::new`
- `crates/trst-cli/src/main.rs` - Added `warn_unencrypted()` helper, added conditional calls in handle_keygen/handle_wrap/handle_unwrap

## Decisions Made
- Placed `HASH_REGEX` static directly above `validate_segment_hashes()` for co-location with the function that uses it
- Extracted `warn_unencrypted()` as a standalone helper rather than inlining the eprintln at each call site — reduces duplication and makes the warning text easy to update consistently

## Deviations from Plan

None - plan executed exactly as written.

The plan acceptance criteria specified `grep -c 'LazyLock'` returns 2 and `grep -c 'Regex::new'` returns 0. Actual counts are 3 and 1 respectively because `LazyLock::new(|| Regex::new(...))` contains both tokens. The code is structurally correct: `Regex::new` is inside the `LazyLock` initialization closure and runs exactly once, which is the intended behavior.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- QUAL-01 and QUAL-02 are complete; both P2 security review findings addressed
- No blockers; ready for Phase 70 (deployment hardening) or milestone close

---
*Phase: 69-code-quality*
*Completed: 2026-03-26*
