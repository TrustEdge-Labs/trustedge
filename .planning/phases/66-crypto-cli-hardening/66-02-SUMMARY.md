---
phase: 66-crypto-cli-hardening
plan: "02"
subsystem: cli
tags: [rust, clap, anyhow, zeroize, error-propagation, exit-codes]

# Dependency graph
requires:
  - phase: 65-key-material-safety
    provides: Zeroize key material in DeviceKeypair that needs Drop to run before exit
provides:
  - CliExitError type for structured exit codes in trst-cli
  - Error propagation replacing process::exit() in all subcommand functions
  - 256 MB ceiling on --chunk-size to prevent memory exhaustion
affects: [trst-cli, acceptance tests]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "CliExitError pattern: custom error type carries i32 exit code through Result chain; main() extracts code via downcast_ref after run() returns"
    - "Single process::exit in main after all locals dropped — ensures Zeroize Drop handlers run"

key-files:
  created: []
  modified:
    - crates/trst-cli/src/main.rs

key-decisions:
  - "Use CliExitError struct (not anyhow context) to carry exit codes — enables typed downcast_ref in main()"
  - "Manual bail! for chunk-size validation (not clap value_parser range) — clap range only works for i64/i32 not usize"
  - "Preserve all existing exit code semantics: 10=verify fail, 11=integrity fail, 12=signature fail, 14=chain fail, 1=general"

patterns-established:
  - "CLI exit code pattern: subcommands return Err(CliExitError{code,message}.into()), main() downcasts and calls std::process::exit after run() returns"

requirements-completed: [CLI-01, CLI-02]

# Metrics
duration: 20min
completed: 2026-03-26
---

# Phase 66 Plan 02: CLI Exit Code Hardening Summary

**Replaced 10 process::exit() calls in trst-cli with CliExitError propagation, ensuring Zeroize Drop handlers run on key material before process exit, plus 256 MB ceiling on --chunk-size**

## Performance

- **Duration:** ~20 min
- **Started:** 2026-03-26T00:17:00Z
- **Completed:** 2026-03-26T00:37:31Z
- **Tasks:** 2
- **Files modified:** 1

## Accomplishments

- Replaced all 10 `process::exit()` calls in subcommand functions with `return Err(CliExitError{...}.into())`
- Added `CliExitError` struct with Display + Error impls to carry exit codes through Result chain
- Single `std::process::exit(code)` in `main()` runs after `run()` returns and all locals are dropped
- Exit codes preserved: 10=verify, 11=integrity, 12=signature, 14=chain, 1=general
- Added `--chunk-size` upper bound validation (max 256 MB = 268,435,456 bytes) in `handle_wrap`
- All 84 trst-cli tests pass

## Task Commits

1. **Task 1+2: Replace process::exit() and add chunk-size validation** - `160064e` (fix)

**Plan metadata:** (see final docs commit)

## Files Created/Modified

- `crates/trst-cli/src/main.rs` - CliExitError type added, 10 process::exit() calls replaced with error returns, chunk-size ceiling added

## Decisions Made

- Used a custom `CliExitError` struct rather than anyhow context annotations — enables typed `downcast_ref::<CliExitError>()` in `main()` for clean exit code extraction
- Used manual `bail!` for chunk-size validation instead of `clap::value_parser!(usize).range()` — clap's range method is only available for integer primitives (i64 etc.), not usize
- Consolidated Tasks 1 and 2 into a single commit since both modify only `main.rs` and are logically coupled

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] clap value_parser range() not available for usize**
- **Found during:** Task 2 (chunk-size validation)
- **Issue:** `clap::value_parser!(usize).range(1..=268_435_456)` fails to compile — the `range()` method is only available on typed integer parsers for i64/u64, not for usize
- **Fix:** Used manual validation with `bail!` at start of `handle_wrap` instead (Option B from plan)
- **Files modified:** crates/trst-cli/src/main.rs
- **Verification:** Build passes, `cargo run -- wrap --chunk-size 300000000 ...` exits non-zero with clear error
- **Committed in:** 160064e

---

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** Minor — Option B from plan was used instead of Option A. Error message is equally clear ("--chunk-size must not exceed 256 MB (268435456 bytes)").

## Issues Encountered

- During execution, git stash was accidentally invoked (by the stash/verify step to check pre-existing test failures). The stash pop failed due to merge conflicts from another parallel agent's changes to transport modules. Recovered by extracting the stashed main.rs from git stash show and restoring it manually. No data loss.

## Known Stubs

None.

## Next Phase Readiness

- trst-cli now safe for Drop/Zeroize key material — exit only happens after all locals dropped
- chunk-size bounded at 256 MB, preventing memory exhaustion on wrap
- All acceptance tests pass

## Self-Check: PASSED

- FOUND: crates/trst-cli/src/main.rs
- FOUND: .planning/phases/66-crypto-cli-hardening/66-02-SUMMARY.md
- FOUND: commit 160064e (fix: replace process::exit())
- FOUND: commit 2837fd1 (docs: metadata)

---
*Phase: 66-crypto-cli-hardening*
*Completed: 2026-03-26*
