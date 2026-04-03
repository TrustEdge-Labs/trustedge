<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


# Phase 75: Core Attestation Library - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.

## Session: 2026-04-01

### Areas Selected
All 3 gray areas discussed: Naming & module placement, Document schema design, Canonical serialization approach.

### Q&A

**Area: Naming & module placement**
- Q: New type is `PointAttestation` in new module, or `SbomAttestation` alongside existing `Attestation`, or Claude decides?
- A: **PointAttestation in new module** — `crates/core/src/point_attestation.rs`. Clear separation from existing `applications/attestation/` which is a different concept.

**Area: Document schema design**
- Q: Generic artifact binding (subject + evidence) or SBOM-specific (sbom_hash + binary_hash)?
- A: **Generic artifact binding** — subject + evidence with freeform labels. Works for any artifact pair (firmware+config, model+weights) without format changes.

**Area: Canonical serialization approach**
- Q: Clone + set signature to None, or #[serde(skip_serializing_if)] on signature, or Claude decides?
- A: **Clone + set signature to None** — same pattern as TrstManifest.to_canonical_bytes(). Proven pattern in this codebase.

### Prior Decisions Applied
- Point attestation format (from CEO review outside voice)
- .te-attestation.json extension (from eng review)
- 16-byte random nonce (from eng review)
- Ed25519 signing + BLAKE3 hashing (from design doc)
