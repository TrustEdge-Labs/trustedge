<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


# Phase 3: trst-core Integration - Context

**Gathered:** 2026-02-10
**Status:** Ready for planning

<domain>
## Phase Boundary

Rename `trst-core` to `trst-protocols` as a standalone WASM-safe crate. Move all type definitions, format constants, and validation logic from core's manifest.rs into trst-protocols. Make trustedge-core depend on trst-protocols for these types. Delete the old trst-core crate. Update all consumers (trst-cli through core, trst-wasm directly).

</domain>

<decisions>
## Implementation Decisions

### Module placement
- Archive/manifest types live under `protocols/` layer — they are protocol/format definitions, not application features
- Full archive format moves together: manifest types, chunk structure, signature format — it's one format spec
- Nested submodules: `protocols::archive::manifest`, `protocols::archive::chunks`, `protocols::archive::signatures`
- Core's existing `manifest.rs` (cam.video profile serialization) stays SEPARATE from trst-protocols archive manifest types — different concerns

### WASM boundary
- Feature-gated WASM: a `wasm` feature flag is NOT needed on trst-protocols — it's WASM-safe by design (no non-WASM deps)
- Subtractive approach for trustedge-core: non-WASM things (transport, YubiKey) use `cfg(not(target_arch = "wasm32"))` — but that's Phase 6 scope
- This phase only ensures archive code compiles to WASM — broader dependency cfg-gating is Phase 6
- WASM verification: `cargo check -p trst-protocols --target wasm32-unknown-unknown` (not whole workspace)

### Migration strategy
- Rename `trst-core` → `trst-protocols`, directory `crates/trst-core/` → `crates/trst-protocols/`
- trustedge-core DEPENDS ON trst-protocols for manifest/archive types (single source of truth, core re-exports)
- trst-cli depends on trustedge-core only (gets trst-protocols types via core re-exports)
- trst-wasm depends on trst-protocols directly (stays lightweight, no unnecessary core dependency)
- Delete old trst-core crate immediately — no deprecation facade

### Deduplication scope — manifest.rs triage
- Three-bucket triage for every item in core's manifest.rs:
  1. **Type definitions** (structs, enums, format constants, serde derives, pure validation) → move to trst-protocols (`archive::manifest` or `capture::profile` depending on domain)
  2. **Operational logic** (I/O, crypto, orchestration) → keep in trustedge-core, rewrite imports to point at trst-protocols types
  3. **Duplicated code** identical to trst-core → delete (this is the duplication Phase 1 audit flagged)
- After triage, delete core's manifest.rs entirely — replace with targeted imports from trst-protocols

### trst-protocols crate structure
- Two top-level domains: `archive` + `capture` — extensible so additional domains slot in without reorganizing
- `archive::manifest` — manifest types, format constants, validation
- `archive::chunks` — chunk header/structure
- `archive::signatures` — signature envelope types
- `capture::profile` — cam.video capture profile types

### Error ownership — split by operation location
- trst-protocols defines scoped errors per sub-module (no top-level ArchiveError):
  - `archive::manifest::ManifestFormatError` — manifest parsing/validation only
  - `archive::chunks::ChunkFormatError` — chunk header/sequence validation only
  - `archive::signatures::SignatureFormatError` — signature envelope parsing only
  - `capture::profile::ProfileFormatError` — capture profile validation only
- Core's `TrustEdgeError` wraps these via `From` impls
- Errors live where the operations that produce them live

### Claude's Discretion
- Exact re-export surface from trustedge-core (which trst-protocols types to re-export)
- Internal module organization within each sub-module
- Test file placement (unit tests in trst-protocols vs integration tests in consumers)

</decisions>

<specifics>
## Specific Ideas

- Core's manifest.rs imports should look like: `use trst_protocols::archive::manifest::ManifestV1;` and `use trst_protocols::capture::profile::CaptureProfile;`
- The crate is designed to grow — structure it so new protocol domains (e.g., receipts, attestation) can be added in later phases without restructuring

</specifics>

<deferred>
## Deferred Ideas

- Full WASM cfg-gating of trustedge-core dependencies — Phase 6 (Feature Flags)
- cargo-machete unused dependency cleanup — Phase 8 (Validation)

</deferred>

---

*Phase: 03-trst-core-integration*
*Context gathered: 2026-02-10*
