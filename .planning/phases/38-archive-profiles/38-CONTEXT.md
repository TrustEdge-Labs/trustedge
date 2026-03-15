# Phase 38: Archive Profiles - Context

**Gathered:** 2026-03-15
**Status:** Ready for planning

<domain>
## Phase Boundary

Users can wrap any data type into a tamper-evident .trst archive using a generic profile. The generic profile becomes the default. cam.video remains available as a specialized profile. Backward compatibility with existing cam.video archives is NOT required — design for the best future architecture.

</domain>

<decisions>
## Implementation Decisions

### Manifest Structure
- Shared base + profile-specific metadata section
- Common fields stay fixed across all profiles: device, segments, claims, signature, trst_version, profile, chunk, prev_archive_hash
- The `capture` field (currently CaptureInfo with fps/resolution/codec) is replaced by a `metadata` section
- For generic profile: semi-structured metadata (typed optional fields + free-form map)
- For cam.video: profile-specific metadata section with fps, resolution, codec
- The manifest type should be renamed from `CamVideoManifest` to something profile-agnostic (e.g., `TrstManifest` or `ArchiveManifest`)

### Profile Selection
- Generic is the default when `--profile` is omitted
- cam.video requires explicit `--profile cam.video`
- CLI `--profile` flag default changes from `"cam.video"` to `"generic"`

### Backward Compatibility
- NOT a requirement — solo builder, greenfield, no existing users
- Design for best future architecture, not backward compat
- Existing cam.video validation that rejects non-cam.video profiles (manifest.rs line 260-263) should be removed/replaced
- `to_canonical_bytes()` should handle multiple profile types cleanly

### Metadata Design
- Semi-structured approach: a few typed optional fields plus a free-form map
- Typed fields: `data_type` (string, e.g., "video", "sensor", "audio", "log", "binary"), `source` (string, e.g., "drone-cam-01"), `description` (optional string), `mime_type` (optional string)
- Free-form: `labels` or `tags` as `HashMap<String, String>` for anything else
- cam.video profile populates its own typed section (fps, resolution, codec) within the metadata section

### Claude's Discretion
- Exact struct hierarchy and enum design for profile-specific metadata
- How to handle canonical serialization for the new metadata section (ordered keys approach)
- Whether to use serde tagged enum or separate structs for profile metadata
- WASM compatibility strategy for the new types (trst-wasm imports manifest types)
- Test structure and coverage approach

</decisions>

<specifics>
## Specific Ideas

- The platform verification path already uses `serde_json::Value` for manifest — already profile-agnostic on the server side
- `trst-wasm` imports `CamVideoManifest` and will need updates
- The `to_canonical_bytes()` method has hardcoded field ordering for cam.video — needs generalization
- cam.video-specific CLI flags (--fps, --chunk-seconds) should only apply when `--profile cam.video` is used

</specifics>

<code_context>
## Existing Code Insights

### Reusable Assets
- `CamVideoManifest::to_canonical_bytes()`: canonical serialization pattern can be generalized
- `DeviceInfo`, `ChunkInfo`, `SegmentInfo`: these are profile-agnostic and stay as-is
- `sign_manifest()`, `verify_manifest()`: work on canonical bytes, profile-agnostic
- `chain_next()`, `genesis()`, `segment_hash()`: BLAKE3 chain is fully profile-agnostic
- `encrypt_segment()`, `generate_aad()`: encryption is profile-agnostic

### Established Patterns
- Manifest types live in `crates/trst-protocols/src/archive/manifest.rs` (WASM-compatible, minimal deps)
- CLI arg parsing via clap with profile-specific flags
- Canonical JSON serialization with explicit key ordering (not serde default)
- Platform uses `serde_json::Value` for manifest in VerifyRequest — already generic

### Integration Points
- `crates/trst-protocols/src/archive/manifest.rs` — manifest type definitions (rename + extend)
- `crates/trst-protocols/src/lib.rs` — re-exports (update for new names)
- `crates/trst-cli/src/main.rs` — CLI wrap/verify commands (handle generic profile)
- `crates/core/src/manifest.rs` — core manifest operations (update imports)
- `crates/trst-wasm/src/lib.rs` — browser verification (update manifest types)
- `crates/core/src/archive.rs` — archive I/O (update for new manifest type)
- `examples/cam.video/` — example code (update for new types)

</code_context>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope.

</deferred>

---

*Phase: 38-archive-profiles*
*Context gathered: 2026-03-15*
