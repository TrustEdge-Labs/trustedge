---
phase: 38-archive-profiles
plan: 02
subsystem: archive
tags: [rust, trst-cli, trst-wasm, manifest, generic-profile, acceptance-tests]

requires:
  - "38-01: TrstManifest with ProfileMetadata enum"

provides:
  - "CLI default profile changed from cam.video to generic"
  - "Profile-conditional flags: --fps and --chunk-seconds optional (cam.video only)"
  - "Generic metadata CLI flags: --data-type, --source, --description, --mime-type"
  - "WASM verifier updated to TrstManifest (profile-agnostic)"
  - "Examples updated to use ProfileMetadata::CamVideo(CamVideoMetadata { ... })"
  - "4 new acceptance tests for generic profile (default, explicit, metadata, regression)"

affects:
  - "38-03 (wrap command end-to-end demo)"
  - "38-04 (generic profile pipeline)"

tech-stack:
  added: []
  patterns:
    - "Profile dispatch in CLI: match args.profile.as_str() { cam.video => CamVideo, _ => Generic }"
    - "Index-based start_time for generic (segment-N), time-based for cam.video"
    - "Optional flags pattern: chunk_seconds: Option<f64>, fps: Option<u32> with .unwrap_or(default)"

key-files:
  created: []
  modified:
    - "crates/trst-cli/src/main.rs"
    - "crates/trst-wasm/src/lib.rs"
    - "crates/trst-cli/tests/acceptance.rs"
    - "examples/cam.video/record_and_wrap.rs"
    - "examples/cam.video/verify_cli.rs"

key-decisions:
  - "Generic profile uses index-based segment start_time (segment-N) rather than time-based; generic data is not inherently time-indexed"
  - "Unknown profiles fall through to generic path in match (future-proofing); cam.video is the only explicitly named branch"
  - "chunk_seconds defaults to 0.0 for generic (not time-based), 2.0 for cam.video"

patterns-established:
  - "Profile dispatch: match args.profile.as_str() { \"cam.video\" => ..., _ => generic }"
  - "Acceptance test structure: wrap_generic_archive() helper mirrors wrap_archive() without --profile flag"

requirements-completed: [PROF-03]

duration: 22min
completed: 2026-03-15
---

# Phase 38 Plan 02: Archive Profiles - Consumer Updates Summary

**CLI default changed to generic profile, WASM updated to TrstManifest, 4 new generic acceptance tests added; all 11 acceptance tests pass and workspace builds cleanly**

## Performance

- **Duration:** 22 min
- **Started:** 2026-03-15T19:48:51Z
- **Completed:** 2026-03-15T20:10:00Z
- **Tasks:** 3
- **Files modified:** 5

## Accomplishments

- Changed `--profile` default from `"cam.video"` to `"generic"` in trst-cli WrapCmd
- `--fps` and `--chunk-seconds` are now `Option<u32>` and `Option<f64>` (cam.video profile only, with sensible defaults)
- Added `--data-type`, `--source`, `--description`, `--mime-type` flags for generic profile metadata
- CLI builds `ProfileMetadata::CamVideo(CamVideoMetadata { ... })` or `ProfileMetadata::Generic(GenericMetadata { ... })` based on `--profile`
- Generic segments use index-based start_time (`"segment-0"`, `"segment-1"`, ...) since generic data is not time-indexed
- WASM verifier updated: all 3 occurrences of `CamVideoManifest` replaced with `TrstManifest`
- `verify_archive_continuity()` signature updated to `&TrstManifest` (profile-agnostic, no logic change)
- `record_and_wrap.rs` example: `capture: CaptureInfo { ... }` replaced with `metadata: ProfileMetadata::CamVideo(CamVideoMetadata { ... })`
- `verify_cli.rs` example: profile dispatch on `ProfileMetadata` variant for output (CamVideo shows resolution/fps)
- 4 new acceptance tests: default profile (no flag), explicit `--profile generic`, metadata flags, cam.video regression
- All 11 acceptance tests pass (7 original + 4 new)

## Task Commits

1. **Task 1: Update CLI to support generic profile as default** - `86d3b1b`
2. **Task 2: Update WASM verifier and examples for TrstManifest** - `1c83b6b`
3. **Task 3: Update acceptance tests and add generic profile coverage** - `5ef4309`

## Files Created/Modified

- `crates/trst-cli/src/main.rs` - Default profile generic, optional cam.video flags, generic metadata flags, ProfileMetadata dispatch
- `crates/trst-wasm/src/lib.rs` - TrstManifest replaces CamVideoManifest (3 occurrences + fn signature)
- `crates/trst-cli/tests/acceptance.rs` - TrstManifest import, 4 new acceptance tests, wrap_generic_archive() helper
- `examples/cam.video/record_and_wrap.rs` - metadata: ProfileMetadata::CamVideo(CamVideoMetadata { ... })
- `examples/cam.video/verify_cli.rs` - ProfileMetadata dispatch for output fields

## Decisions Made

- Generic profile uses index-based segment `start_time` (`"segment-0"`, `"segment-1"`) not time-based, since generic data doesn't have an inherent temporal axis
- Unknown profiles fall through to `_` branch in the match (same as generic), providing forward compatibility
- `chunk_seconds` defaults: `0.0` for generic (no temporal meaning), `2.0` for cam.video (standard P0 default)

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] `capture` field removed from TrstManifest in Plan 01**
- **Found during:** Task 1 (build check)
- **Issue:** `examples/cam.video/record_and_wrap.rs` used old struct literal syntax `capture: CaptureInfo { ... }` which is incompatible with new `metadata: ProfileMetadata::CamVideo(...)` field
- **Fix:** Updated both examples (`record_and_wrap.rs`, `verify_cli.rs`) in Task 2
- **Files modified:** `examples/cam.video/record_and_wrap.rs`, `examples/cam.video/verify_cli.rs`
- **Commit:** `1c83b6b`

## Test Results

- Acceptance tests: 11/11 pass (7 original cam.video + 4 new generic)
- trst-protocols unit tests: 18/18 pass
- trustedge-core unit tests: 161/162 pass (1 skipped: test_many_keys, known long-runner >60s)
- Workspace build: clean (all 9 root workspace crates)
- cam.video examples: compile clean in separate workspace

## Self-Check: PASSED

All files found and commits verified:
- `crates/trst-cli/src/main.rs` - FOUND
- `crates/trst-wasm/src/lib.rs` - FOUND
- `crates/trst-cli/tests/acceptance.rs` - FOUND
- `examples/cam.video/record_and_wrap.rs` - FOUND
- `examples/cam.video/verify_cli.rs` - FOUND
- Commit `86d3b1b` - FOUND
- Commit `1c83b6b` - FOUND
- Commit `5ef4309` - FOUND

---
*Phase: 38-archive-profiles*
*Completed: 2026-03-15*
