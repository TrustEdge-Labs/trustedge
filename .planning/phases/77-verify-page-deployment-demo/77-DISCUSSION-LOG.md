# Phase 77: Verify Page + Deployment + Demo - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.

## Session: 2026-04-02

### Areas Selected
All 3 gray areas discussed: Verify page design, Deployment target, Demo script design.

### Q&A

**Area: Verify page design**
- Q: Minimal functional, full third-party flow, or Claude decides?
- A: **Minimal functional** — single file picker, verify button, receipt display. ~100 lines HTML/CSS/JS. Third-party flow deferred.

**Area: Deployment target**
- Q: Fly.io, Railway, or Claude decides?
- A: User corrected: **DigitalOcean App Platform** (container). User's existing infrastructure is on DO.

**Area: Demo script design**
- Q: Separate script or extend demo.sh?
- A: **Separate: scripts/demo-attestation.sh** — focused on SBOM attestation, doesn't touch existing demo.sh.

### Prior Decisions Applied
- POST body is attestation JSON directly (Phase 76 D-11)
- Platform returns 200 with status + JWS receipt (Phase 76 D-13)
- In-memory backend, ephemeral receipts (CEO review)
- Verify page handles errors and timeouts (eng review)
