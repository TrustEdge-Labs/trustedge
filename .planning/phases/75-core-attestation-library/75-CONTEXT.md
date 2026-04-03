<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


# Phase 75: Core Attestation Library - Context

**Gathered:** 2026-04-01
**Status:** Ready for planning

<domain>
## Phase Boundary

Deliver the `PointAttestation` type in trustedge-core: a lightweight JSON attestation document that cryptographically binds two artifacts (subject + evidence) with Ed25519 signing, BLAKE3 hashing, random nonce, timestamp, and canonical serialization. This is the foundation for the SBOM attestation wedge (Phases 76-78).

This phase delivers the LIBRARY only. CLI commands, platform endpoints, and deployment are subsequent phases.

</domain>

<decisions>
## Implementation Decisions

### Naming & Module Placement
- **D-01:** New type is `PointAttestation` (not `SbomAttestation` or `AttestationDocument`). "Point" distinguishes from stream attestation (.trst archives) and existing `Attestation` struct (git-commit software birth certificates).
- **D-02:** New module at `crates/core/src/point_attestation.rs` (top-level module, not nested under applications/attestation/). The existing `applications/attestation/` module is a different concept (SHA-256, git commit, sealed envelope format).
- **D-03:** Re-export from `crates/core/src/lib.rs` for downstream crate access.

### Document Schema Design
- **D-04:** Generic artifact binding, NOT SBOM-specific. Schema uses `subject` (the thing being attested) and `evidence` (what proves something about the subject) as named artifact slots. Each slot has `hash`, `filename`, and `label` fields.
- **D-05:** Labels are freeform strings populated by the CLI (e.g., "binary", "sbom") but the format itself is artifact-agnostic. Future use cases (firmware + config, model + weights) work without format changes.
- **D-06:** Schema versioned via `"format": "te-point-attestation-v1"` field for forward compatibility.
- **D-07:** Output file extension is `.te-attestation.json`.

### Document Schema Fields
- **D-08:** Required fields: `format`, `trustedge_version`, `timestamp` (ISO 8601), `nonce` (hex-encoded 16 random bytes), `subject` (hash + filename + label), `evidence` (hash + filename + label), `public_key` (prefixed, e.g., "ed25519:base64"), `signature` (prefixed, e.g., "ed25519:base64").
- **D-09:** Hash format follows existing TrustEdge convention: `"b3:<64-hex-chars>"` (BLAKE3 with `b3:` prefix, matching platform's `validate_segment_hashes()` regex pattern).

### Canonical Serialization
- **D-10:** Clone struct, set `signature` to `None`, serialize to deterministic JSON. Same pattern as `TrstManifest::to_canonical_bytes()`.
- **D-11:** Use `serde_json::to_string()` with struct field ordering (serde preserves struct declaration order). For nested `subject`/`evidence` objects, field ordering is deterministic because they're structs, not maps.

### Cryptographic Operations
- **D-12:** Signing reuses existing `sign_manifest()` pattern from `crypto.rs` (Ed25519 via `DeviceKeypair`). The `canonical_bytes` input is the canonical JSON of the attestation document.
- **D-13:** Verification reuses `verify_manifest()` dispatch pattern (prefix-based algorithm selection). Currently supports `ed25519:` prefix.
- **D-14:** 16-byte random nonce from `OsRng` included in signed payload. Prevents replay of same-second attestations of identical files.

### Claude's Discretion
- Internal struct naming (e.g., `ArtifactRef` for the subject/evidence inner struct)
- Whether to implement `Display` or other trait impls
- Error type design (new enum or reuse CryptoError)
- Test file organization (inline mod tests vs separate test file)

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Existing Crypto Patterns
- `crates/core/src/crypto.rs` — `sign_manifest()` (line ~406) and `verify_manifest()` (line ~427) for Ed25519 signing/verification pattern with prefix-based dispatch
- `crates/core/src/crypto.rs` — `DeviceKeypair` struct for key management, `from_public()` for public key parsing

### Existing Canonical Serialization
- `crates/trst-protocols/src/archive/manifest.rs` — `TrstManifest::to_canonical_bytes()` (line ~242) for the clone+set-None+serialize pattern
- `crates/trst-protocols/src/archive/manifest.rs` — `serialize_canonical()` for deterministic JSON with sorted keys

### Existing Attestation (different concept, avoid naming collision)
- `crates/core/src/applications/attestation/mod.rs` — Existing `Attestation` struct (SHA-256, git commit). Different purpose but overlapping domain name.

### Hash Format Convention
- `crates/platform/src/validation.rs` — `validate_segment_hashes()` uses `b3:[0-9a-f]{64}` regex for BLAKE3 hash format. New point attestation hashes MUST match this pattern.

### Design Documents
- `docs/designs/sbom-attestation-wedge.md` — CEO plan with scope decisions and vision
- `~/.gstack/projects/TrustEdge-Labs-trustedge/john-main-design-20260401-172433.md` — Approved design doc with CLI interface, deployment constraints, success criteria

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `DeviceKeypair` — key generation, encrypted storage, signing. Directly reusable for point attestation signing.
- `sign_manifest()` / `verify_manifest()` — Ed25519 signing/verification with prefix-based dispatch. Canonical bytes input matches our needs.
- `blake3` crate — already in workspace dependencies. Used extensively for segment hashing.
- `chrono` — already in workspace for ISO 8601 timestamps.
- `rand::OsRng` — already used for nonce generation in envelope.rs.
- `base64::Engine` — already imported and used across crypto.rs.

### Established Patterns
- Prefix-based algorithm dispatch: `"ed25519:base64"` for signatures, `"ed25519:base64"` for public keys, `"b3:hex"` for hashes. New code MUST use the same prefix convention.
- `#[derive(Serialize, Deserialize, Debug, Clone)]` on all wire types.
- `Option<String>` for signature field (None before signing, Some after).
- `anyhow::Result` for CLI-facing functions, `thiserror` enums for library errors.

### Integration Points
- `crates/core/src/lib.rs` — re-export new types here for downstream access.
- `crates/trst-cli/src/main.rs` — Phase 76 will add `AttestSbom` and `VerifyAttestation` subcommands that use the new library.
- `crates/platform/src/http/handlers.rs` — Phase 76 will add `verify_attestation_handler` that uses the new verification function.

</code_context>

<specifics>
## Specific Ideas

- The `subject` / `evidence` naming was chosen to be generic (not SBOM-specific) so the format works for firmware+config, model+weights, and other artifact pairs without schema changes.
- The `label` field in each artifact slot lets the CLI communicate what kind of artifact it is (e.g., "binary", "sbom") without the format enforcing it.
- The outside voice from the CEO review argued that .trst format is wrong for single-shot attestation. The user agreed. This is a NEW format alongside .trst, not a replacement.

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope.

</deferred>

---

*Phase: 75-core-attestation-library*
*Context gathered: 2026-04-01*
