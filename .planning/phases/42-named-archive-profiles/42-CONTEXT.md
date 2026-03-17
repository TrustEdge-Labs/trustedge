# Phase 42: Named Archive Profiles - Context

**Gathered:** 2026-03-16
**Status:** Ready for planning

<domain>
## Phase Boundary

Add three named archive profiles (sensor, audio, log) with typed metadata structs to trst-protocols, extend validation and canonical serialization, and add profile-conditional CLI flags. Existing generic and cam.video profiles must continue working unchanged.

</domain>

<decisions>
## Implementation Decisions

### Metadata Field Design

**Sensor profile:** `SensorMetadata`
- `started_at` (String, required)
- `ended_at` (String, required)
- `sample_rate_hz` (f64, required)
- `unit` (String, required — e.g., "celsius", "psi", "rpm")
- `sensor_model` (String, required — e.g., "DHT22", "BMP280")
- `latitude` (Option<f64>)
- `longitude` (Option<f64>)
- `altitude` (Option<f64>)
- `labels` (BTreeMap<String, String>, optional — same pattern as generic)

**Audio profile:** `AudioMetadata`
- `started_at` (String, required)
- `ended_at` (String, required)
- `sample_rate_hz` (u32, required)
- `bit_depth` (u16, required)
- `channels` (u8, required)
- `codec` (String, required — e.g., "pcm", "opus", "aac")

**Log profile:** `LogMetadata`
- `started_at` (String, required)
- `ended_at` (String, required)
- `application` (String, required — e.g., "nginx", "syslog")
- `host` (String, required)
- `log_level` (String, required — e.g., "info", "error", "debug")
- `log_format` (String, required — e.g., "json", "syslog", "plaintext")

### Profile Naming & CLI Flags
- Profile strings: `"sensor"`, `"audio"`, `"log"` (lowercase, no dots)
- Profile-conditional flags (same pattern as cam.video):
  - `--profile sensor`: `--sample-rate`, `--unit`, `--sensor-model`, `--latitude`, `--longitude`, `--altitude`
  - `--profile audio`: `--sample-rate`, `--bit-depth`, `--channels`, `--codec`
  - `--profile log`: `--application`, `--host`, `--log-level`, `--log-format`
- Error if flag doesn't match the selected profile
- `--sample-rate` is shared between sensor and audio profiles (both use it)

### Serde Discrimination Strategy
- Keep `#[serde(untagged)]` — no breaking change to existing archives
- Enum variant order for untagged deserialization:
  1. CamVideo (unique: `fps`, `resolution`, `codec`)
  2. Sensor (unique: `unit`, `sensor_model`)
  3. Audio (unique: `bit_depth`, `channels`)
  4. Log (unique: `application`, `host`)
  5. Generic (fallback — all fields optional)
- Each profile has at least one unique required field that no other has

### Claude's Discretion
- Exact canonical serialization key ordering for new profiles
- Test structure and coverage approach
- Whether to add helper constructors for new metadata types
- How to handle the `--sample-rate` flag shared between sensor and audio profiles in clap

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Manifest types
- `crates/trst-protocols/src/archive/manifest.rs` — ProfileMetadata enum, validate(), serialize_canonical(), GenericMetadata, CamVideoMetadata
- `crates/trst-protocols/src/lib.rs` — Re-exports for manifest types

### CLI
- `crates/trst-cli/src/main.rs` — Profile-conditional flag handling pattern (cam.video flags)

### Research
- `.planning/research/FEATURES.md` — Named profile feature analysis, MVP definition
- `.planning/research/PITFALLS.md` — Profile validation hard-rejection pitfall, canonical serializer pitfall

### Requirements
- `.planning/REQUIREMENTS.md` — PROF-05 through PROF-08

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `GenericMetadata` struct: pattern for optional fields + BTreeMap labels
- `CamVideoMetadata` struct: pattern for typed required fields
- `serialize_canonical()`: manual key-ordered serialization — needs new match arms
- `validate()`: profile allowlist at line 367 — needs "sensor", "audio", "log" added
- Profile-conditional CLI flag pattern in `handle_wrap()` — cam.video flags gated on profile match

### Established Patterns
- `#[serde(untagged)]` on ProfileMetadata enum with CamVideo-first ordering
- `#[serde(skip_serializing_if = "Option::is_none")]` for optional fields
- BTreeMap for sorted key/value pairs in canonical output
- `to_canonical_bytes()` excludes signature, dispatches per variant

### Integration Points
- `crates/trst-protocols/src/archive/manifest.rs` — Add 3 metadata structs + 3 enum variants
- `crates/trst-protocols/src/lib.rs` — Re-export new types
- `crates/trst-cli/src/main.rs` — Add profile-conditional CLI flags
- `crates/trst-cli/tests/acceptance.rs` — Add acceptance tests for new profiles
- `crates/core/src/archive.rs` — Should work unchanged (profile-agnostic)

</code_context>

<specifics>
## Specific Ideas

- The preview from the discussion shows the exact CLI invocation style users expect
- Sensor profile includes geo fields (latitude/longitude/altitude) for agriculture and drone use cases
- Log profile includes log_format for downstream parsers
- `--sample-rate` shared between sensor and audio is a CLI design detail for Claude to handle

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope.

</deferred>

---

*Phase: 42-named-archive-profiles*
*Context gathered: 2026-03-16*
