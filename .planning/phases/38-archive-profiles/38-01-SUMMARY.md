---
phase: 38-archive-profiles
plan: 01
subsystem: archive
tags: [rust, trst-protocols, trustedge-core, manifest, serde, canonical-json]

requires: []
provides:
  - "TrstManifest struct with ProfileMetadata enum (CamVideo + Generic variants)"
  - "GenericMetadata with optional typed fields (data_type, source, description, mime_type) and BTreeMap labels"
  - "CamVideoMetadata replacing CaptureInfo struct (type alias kept for backward compat)"
  - "CamVideoManifest type alias for TrstManifest (zero breakage)"
  - "Canonical serialization for both profile types with deterministic key ordering"
  - "validate() accepting both generic and cam.video profiles"
  - "TrstManifest re-exported from trustedge-core alongside all new types"
affects:
  - "38-02 (trst-cli/trst-wasm update)"
  - "38-03 (wrap command generic profile support)"
  - "38-04 (generic profile end-to-end)"

tech-stack:
  added: []
  patterns:
    - "ProfileMetadata enum: untagged serde with CamVideo first (has required fields that disambiguate)"
    - "BTreeMap for labels ensures sorted keys in canonical JSON output"
    - "Type alias pattern: CamVideoManifest = TrstManifest, CaptureInfo = CamVideoMetadata"

key-files:
  created: []
  modified:
    - "crates/trst-protocols/src/archive/manifest.rs"
    - "crates/trst-protocols/src/lib.rs"
    - "crates/core/src/archive.rs"
    - "crates/core/src/lib.rs"
    - "crates/core/src/protocols/mod.rs"

key-decisions:
  - "CamVideoManifest kept as type alias (not deleted) to avoid breakage in trst-cli and trst-wasm before Plan 02"
  - "CaptureInfo kept as type alias for CamVideoMetadata for same backward-compat reason"
  - "metadata field replaces capture field in TrstManifest; old capture key only appears in canonical output for cam.video manifests"
  - "Untagged serde with CamVideo variant listed first: CamVideo has required fields (timezone, fps, resolution, codec) absent from Generic, enabling reliable disambiguation"
  - "new() creates generic profile by default; new_cam_video() creates cam.video profile"

patterns-established:
  - "Profile dispatch: match &self.metadata { ProfileMetadata::CamVideo(m) => ..., ProfileMetadata::Generic(m) => ... }"
  - "Canonical serialization: manual string building with explicit field ordering per profile variant"

requirements-completed: [PROF-01, PROF-02, PROF-04]

duration: 18min
completed: 2026-03-15
---

# Phase 38 Plan 01: Archive Profiles - Contract Types Summary

**TrstManifest with ProfileMetadata enum (CamVideo + Generic) replaces CamVideoManifest as the profile-agnostic contract type, with backward-compat aliases and canonical serialization for both profiles**

## Performance

- **Duration:** 18 min
- **Started:** 2026-03-15T17:10:00Z
- **Completed:** 2026-03-15T17:28:00Z
- **Tasks:** 2
- **Files modified:** 5

## Accomplishments

- Defined `TrstManifest` with `ProfileMetadata` enum supporting `Generic(GenericMetadata)` and `CamVideo(CamVideoMetadata)` variants
- `GenericMetadata` has typed optional fields (data_type, source, description, mime_type) and a `BTreeMap<String, String>` labels field for sorted canonical output
- `CamVideoManifest = TrstManifest` and `CaptureInfo = CamVideoMetadata` type aliases ensure zero breakage for existing consumers
- `to_canonical_bytes()` dispatches on profile variant with deterministic key ordering for both profiles
- `validate()` accepts both `"generic"` and `"cam.video"` profiles (removed old single-profile restriction)
- Core crate re-exports all new types; archive.rs uses `TrstManifest` throughout

## Task Commits

1. **Task 1: Define TrstManifest with ProfileMetadata enum and canonical serialization** - `8fbfe8a` (feat, TDD)
2. **Task 2: Update core crate re-exports and archive module to use TrstManifest** - `cf98637` (feat)

## Files Created/Modified

- `crates/trst-protocols/src/archive/manifest.rs` - Full refactor: TrstManifest, ProfileMetadata enum, GenericMetadata, CamVideoMetadata, type aliases, 18 tests
- `crates/trst-protocols/src/lib.rs` - Updated re-exports to include all new types
- `crates/core/src/archive.rs` - write_archive/read_archive use TrstManifest; test helper updated
- `crates/core/src/lib.rs` - Added TrstManifest, CamVideoMetadata, GenericMetadata, ProfileMetadata re-exports
- `crates/core/src/protocols/mod.rs` - Updated doc comments to reference TrstManifest

## Decisions Made

- Used `#[serde(untagged)]` with `CamVideo` first: it has required fields (`timezone`, `fps`, `resolution`, `codec`) that `Generic` lacks, so serde's try-first approach reliably disambiguates
- `metadata` field replaces `capture` in the manifest struct. The canonical bytes use key `"metadata"` for both profiles. Old `capture` key only appears if reading a legacy manifest (handled by trst-cli in Plan 02)
- Type aliases (`CamVideoManifest`, `CaptureInfo`) kept so trst-cli and trst-wasm compile without changes until Plan 02

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

- cargo fmt pre-commit hook reformatted two formatting choices in manifest.rs (multi-line vs single-line format! calls). Applied `cargo fmt` before retrying commit - no logic changes.

## Next Phase Readiness

- Contract types established; trst-cli and trst-wasm can now be updated (Plan 02)
- `TrstManifest::new_cam_video()` provides cam.video default constructor for wrap command
- `TrstManifest::new()` provides generic default constructor for new profile support

## Self-Check: PASSED

All files found and commits verified:
- `crates/trst-protocols/src/archive/manifest.rs` - FOUND
- `crates/trst-protocols/src/lib.rs` - FOUND
- `crates/core/src/archive.rs` - FOUND
- `crates/core/src/lib.rs` - FOUND
- Commit `8fbfe8a` - FOUND
- Commit `cf98637` - FOUND

---
*Phase: 38-archive-profiles*
*Completed: 2026-03-15*
