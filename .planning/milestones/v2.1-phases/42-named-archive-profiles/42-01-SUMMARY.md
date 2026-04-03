<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


---
phase: 42-named-archive-profiles
plan: 01
subsystem: archive
tags: [rust, serde, trst-protocols, manifest, named-profiles]

# Dependency graph
requires: []
provides:
  - SensorMetadata struct with required fields (started_at, ended_at, sample_rate_hz, unit, sensor_model) and optional geo fields + labels
  - AudioMetadata struct with required fields (started_at, ended_at, sample_rate_hz, bit_depth, channels, codec)
  - LogMetadata struct with required fields (started_at, ended_at, application, host, log_level, log_format)
  - ProfileMetadata enum extended with Sensor, Audio, Log variants in serde-safe order
  - validate() accepts sensor, audio, log profiles with field-level validation
  - serialize_canonical() produces deterministic JSON for all 3 new profiles
  - new_sensor(), new_audio(), new_log() constructor methods on TrstManifest
  - lib.rs re-exports SensorMetadata, AudioMetadata, LogMetadata
affects: [42-02, 43, 44, trustedge-trst-cli, trustedge-trst-wasm]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Untagged serde enum variant order: each typed variant must have at least one required field absent from Generic to ensure deterministic deserialization without tags"
    - "serialize_canonical() extended with explicit match arms per variant — key order hard-coded for determinism"
    - "Constructor pattern: new_sensor()/new_audio()/new_log() parallel to new_cam_video()"

key-files:
  created: []
  modified:
    - crates/trst-protocols/src/archive/manifest.rs
    - crates/trst-protocols/src/lib.rs
    - examples/cam.video/verify_cli.rs

key-decisions:
  - "Variant order in ProfileMetadata: CamVideo, Sensor, Audio, Log, Generic — each typed variant has distinguishing required fields to prevent serde ambiguity with untagged deserialization"
  - "SensorMetadata.labels uses BTreeMap<String,String> (same as GenericMetadata) for sorted canonical output"
  - "AudioMetadata.sample_rate_hz is u32 (integer Hz) vs SensorMetadata.sample_rate_hz which is f64 (fractional Hz for precision sensors)"

patterns-established:
  - "Untagged serde discrimination: typed variants before Generic, each with unique required field(s)"
  - "Canonical serialization: explicit key-order match arms ensure deterministic JSON for signing"

requirements-completed: [PROF-05, PROF-06, PROF-07]

# Metrics
duration: 3min
completed: 2026-03-17
---

# Phase 42 Plan 01: Named Archive Profiles - Type Foundation Summary

**SensorMetadata, AudioMetadata, LogMetadata structs added to TrstManifest type system with untagged serde variants, validate() field checks, deterministic canonical serialization, and 18 new unit tests**

## Performance

- **Duration:** 3 min
- **Started:** 2026-03-17T03:13:45Z
- **Completed:** 2026-03-17T03:17:29Z
- **Tasks:** 1
- **Files modified:** 3

## Accomplishments

- 3 new metadata structs (SensorMetadata, AudioMetadata, LogMetadata) with all plan-specified fields and derive macros
- ProfileMetadata enum extended to 5 variants (CamVideo, Sensor, Audio, Log, Generic) with correct serde discrimination order
- validate() updated to accept 5 profiles and enforce required-field invariants per variant
- serialize_canonical() extended with explicit key-ordered match arms for all 3 new variants
- 3 new constructor methods: new_sensor(), new_audio(), new_log() on TrstManifest
- 18 new unit tests: round-trip, canonical key ordering, validation acceptance, and untagged serde discrimination
- All 30 tests pass (12 existing + 18 new), 0 clippy warnings

## Task Commits

Each task was committed atomically:

1. **Task 1: Add metadata structs and enum variants** - `2103c3b` (feat)

## Files Created/Modified

- `crates/trst-protocols/src/archive/manifest.rs` - Added 3 metadata structs, 3 enum variants, updated validate() and serialize_canonical(), added 3 constructors and 18 tests
- `crates/trst-protocols/src/lib.rs` - Added SensorMetadata, AudioMetadata, LogMetadata to re-exports
- `examples/cam.video/verify_cli.rs` - Added Sensor, Audio, Log match arms (Rule 3 fix)

## Decisions Made

- Variant order in ProfileMetadata is CamVideo, Sensor, Audio, Log, Generic. Each typed variant has unique required fields (unit+sensor_model, bit_depth+channels, application+host) absent from Generic, ensuring untagged deserialization picks the correct variant.
- SensorMetadata.labels uses BTreeMap matching GenericMetadata.labels pattern for consistent canonical output.
- AudioMetadata.sample_rate_hz is u32 (integer Hz values like 44100, 48000) while SensorMetadata.sample_rate_hz is f64 (fractional rates possible in precision sensors).

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Updated examples/cam.video/verify_cli.rs match arms**
- **Found during:** Task 1 (workspace build verification)
- **Issue:** `verify_cli.rs` had a non-exhaustive match on `ProfileMetadata` - adding 3 new variants caused a compile error `E0004`
- **Fix:** Added Sensor, Audio, Log match arms with appropriate println! output for each variant's key fields
- **Files modified:** examples/cam.video/verify_cli.rs
- **Verification:** `cargo build --workspace` passes with no errors
- **Committed in:** 2103c3b (Task 1 commit)

---

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** Required fix - workspace would not build without it. No scope creep.

## Issues Encountered

None beyond the blocking match arms fix.

## Next Phase Readiness

- Type foundation complete; plan 02 (CLI wrap commands for sensor/audio/log) can proceed
- All new types are exported from crate root via lib.rs
- SensorMetadata, AudioMetadata, LogMetadata available to trustedge-trst-cli and trustedge-trst-wasm

---
*Phase: 42-named-archive-profiles*
*Completed: 2026-03-17*
