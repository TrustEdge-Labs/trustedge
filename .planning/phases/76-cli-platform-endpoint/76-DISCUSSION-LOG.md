# Phase 76: CLI + Platform Endpoint - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.

## Session: 2026-04-02

### Areas Selected
All 3 gray areas discussed: CLI argument design, Platform request/response format, Error responses & exit codes.

### Q&A

**Area: CLI argument design**
- Q: --binary + --sbom (SBOM-specific) or --subject + --evidence (generic) or Claude decides?
- A: **--binary + --sbom** — SBOM-specific flags. CLI maps binary->subject, sbom->evidence internally. Consistent with design doc.

**Area: Platform request/response format**
- Q: Direct attestation JSON as POST body, or wrapper with explicit device_pub, or Claude decides?
- A: **Direct attestation JSON** — POST body IS the attestation document. Public key comes from the embedded field. Simpler API.

**Area: Error responses & exit codes**
- Q: Simple (0/1/10) or detailed (match existing verify exit codes) or Claude decides?
- A: **Simple: 0/1/10** — 0=success, 1=general error, 10=verification failed. Platform returns 200 with status field.

### Prior Decisions Applied
- PointAttestation type from Phase 75 (create, verify_signature, verify_file_hashes, to_json, from_json)
- .te-attestation.json extension (Phase 75 D-07)
- Reject 0-byte binary, non-JSON SBOM, >256MB binary (CEO review)
- CycloneDX JSON only, SBOM treated as opaque JSON (CEO review)
- New /v1/verify-attestation endpoint separate from /v1/verify (eng review)
