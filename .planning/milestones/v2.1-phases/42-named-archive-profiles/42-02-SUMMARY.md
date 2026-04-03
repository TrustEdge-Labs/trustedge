<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


---
phase: 42-named-archive-profiles
plan: 02
subsystem: archive
tags: [trst, cli, clap, sensor, audio, log, profiles, acceptance-tests]

# Dependency graph
requires:
  - phase: 42-named-archive-profiles
    plan: 01
    provides: SensorMetadata, AudioMetadata, LogMetadata types in trst-protocols manifest

provides:
  - trst wrap --profile sensor/audio/log CLI flags with profile-specific required fields
  - End-to-end sensor, audio, and log archive wrap+verify round-trips working
  - 5 new acceptance tests for sensor/audio/log profiles (geo, missing-flag regression)
  - SensorMetadata, AudioMetadata, LogMetadata re-exported from trustedge-core

affects: [42-named-archive-profiles, 43-chunk-encryption, downstream trst-cli consumers]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - Profile-conditional CLI flags in WrapCmd with ok_or_else anyhow errors for missing required args
    - Negative float CLI values use --flag=VALUE syntax to avoid clap flag ambiguity
    - Acceptance tests verify manifest JSON field presence via serde_json::Value assertions

key-files:
  created: []
  modified:
    - crates/trst-cli/src/main.rs
    - crates/trst-cli/tests/acceptance.rs
    - crates/core/src/lib.rs

key-decisions:
  - "Use --longitude=-122.4194 (equals syntax) for negative float args; positional parsing interprets leading '-' as flags"
  - "SensorMetadata, AudioMetadata, LogMetadata added to trustedge-core re-exports alongside existing CamVideoMetadata/GenericMetadata"

patterns-established:
  - "Profile match arm pattern: extract required fields with ok_or_else anyhow bail, then construct ProfileMetadata variant"

requirements-completed: [PROF-05, PROF-06, PROF-07, PROF-08]

# Metrics
duration: 12min
completed: 2026-03-17
---

# Phase 42 Plan 02: Named Archive Profiles -- CLI Flags & Acceptance Tests Summary

**trst wrap --profile sensor/audio/log with profile-specific required flags, geo support, and 5 new acceptance tests covering full wrap+verify round-trips**

## Performance

- **Duration:** ~12 min
- **Started:** 2026-03-17T03:30:00Z
- **Completed:** 2026-03-17T03:42:00Z
- **Tasks:** 2
- **Files modified:** 3

## Accomplishments

- Added 13 new WrapCmd CLI flags for sensor (sample_rate, unit, sensor_model, latitude, longitude, altitude), audio (bit_depth, channels, codec), and log (application, host, log_level, log_format) profiles
- Added sensor/audio/log match arms in handle_wrap() with clear required-field validation using anyhow errors
- Added SensorMetadata, AudioMetadata, LogMetadata to trustedge-core re-exports so downstream crates get them via trustedge-core
- 5 new acceptance tests all pass: sensor wrap+verify, audio wrap+verify, log wrap+verify, sensor with geo fields, missing required flag regression
- All 19 acceptance tests pass (12 existing + 2 from plan 01 + 5 new)

## Task Commits

Each task was committed atomically:

1. **Task 1: Add sensor/audio/log CLI flags and metadata construction** - `25172b4` (feat)
2. **Task 2: Add acceptance tests for sensor, audio, and log profiles** - `0fcf066` (feat)

**Plan metadata:** (docs commit below)

## Files Created/Modified

- `crates/trst-cli/src/main.rs` - Added 13 WrapCmd fields + sensor/audio/log match arms in handle_wrap()
- `crates/trst-cli/tests/acceptance.rs` - Added 5 acceptance tests for new profiles
- `crates/core/src/lib.rs` - Added AudioMetadata, LogMetadata, SensorMetadata to re-export list

## Decisions Made

- Negative float CLI values require `--flag=VALUE` syntax (not `--flag VALUE`) because clap interprets leading `-` as a flag prefix. Applied to `--longitude=-122.4194` in acceptance test.
- New metadata types re-exported from trustedge-core alongside existing types to keep downstream import paths consistent.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Fixed negative longitude CLI parsing in acceptance test**
- **Found during:** Task 2 (acceptance_sensor_with_geo test)
- **Issue:** Passing `"--longitude", "-122.4194"` as separate args caused clap to treat `-122.4194` as an unknown flag
- **Fix:** Changed to `"--longitude=-122.4194"` (equals-sign syntax) which clap handles correctly
- **Files modified:** crates/trst-cli/tests/acceptance.rs
- **Verification:** acceptance_sensor_with_geo test passes
- **Committed in:** 0fcf066 (Task 2 commit)

---

**Total deviations:** 1 auto-fixed (Rule 1 - bug in test invocation)
**Impact on plan:** Necessary for correct test behavior. No scope creep.

## Issues Encountered

None beyond the clap negative-float parsing issue described above.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Phase 42 complete: all profile types (CamVideo, Sensor, Audio, Log, Generic) have full CLI support and acceptance test coverage
- Phase 43 (chunk encryption) can proceed; trst-protocols and trst-cli are stable
- No blockers

## Self-Check: PASSED

- SUMMARY.md: FOUND
- Task 1 commit 25172b4: FOUND
- Task 2 commit 0fcf066: FOUND

---
*Phase: 42-named-archive-profiles*
*Completed: 2026-03-17*
