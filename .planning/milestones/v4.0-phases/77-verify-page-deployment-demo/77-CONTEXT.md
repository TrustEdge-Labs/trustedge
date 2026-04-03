# Phase 77: Verify Page + Deployment + Demo - Context

**Gathered:** 2026-04-02
**Status:** Ready for planning

<domain>
## Phase Boundary

Deliver three things: (1) a static HTML verification page that lets anyone upload a `.te-attestation.json` and see the verification result, (2) a public deployment of the platform server on DigitalOcean App Platform, and (3) a demo script that runs the full attestation flow in under 60 seconds.

This phase makes the SBOM attestation wedge publicly accessible. CLI and platform endpoint are already built (Phase 76).

</domain>

<decisions>
## Implementation Decisions

### Verify Page
- **D-01:** Minimal functional page. Single file input for `.te-attestation.json`. "Verify" button. Shows receipt details on success, error message on failure. Clean layout, no heavy branding. ~100 lines of HTML/CSS/JS.
- **D-02:** Static HTML file, NOT a SvelteKit component. No build step. Self-contained (inline CSS/JS). Lives at `web/verify/index.html` or similar static location.
- **D-03:** Uploads attestation JSON to `POST /v1/verify-attestation` via fetch(). Displays parsed receipt on success (subject hash, evidence hash, signing key, timestamp, format version).
- **D-04:** Error handling: network errors show "Server unavailable, try again later". Timeout after 30 seconds shows "Request timed out". API errors (400, 429) show appropriate messages. No silent failures.
- **D-05:** Third-party verification flow (attestation + binary + public key upload) deferred. This page does signature verification only via the platform endpoint. File hash comparison is a future enhancement.

### Deployment
- **D-06:** DigitalOcean App Platform with container deployment. Push Docker image of platform-server.
- **D-07:** Platform runs WITHOUT postgres feature (in-memory backend only). No auth, no device registration, no receipt storage. Just /v1/verify, /v1/verify-attestation, /healthz, and JWKS endpoints.
- **D-08:** Ephemeral receipts (lost on restart). Acceptable for demo stage.
- **D-09:** Existing rate limiting (governor, 10 req/sec default) and 2MB body limit apply.
- **D-10:** Create a `deploy/digitalocean/` directory with App Platform spec (app.yaml or doctl config) and deployment instructions.
- **D-11:** The verify HTML page is served as a static asset by the platform server (or co-deployed as a static site). Decision on exactly how (embedded in binary, nginx sidecar, or DO static site) is Claude's discretion.

### Demo Script
- **D-12:** Separate script: `scripts/demo-attestation.sh`. Does NOT modify existing `scripts/demo.sh`.
- **D-13:** Flow: check prerequisites (trst binary, syft) → keygen → generate SBOM with syft on the trst binary itself → attest-sbom → verify-attestation locally → optionally verify against public endpoint.
- **D-14:** Must complete in under 60 seconds total (wall clock). Most time will be syft SBOM generation (~10-20s).
- **D-15:** Script auto-detects whether the public endpoint is reachable. If not, skips remote verification and notes it.
- **D-16:** Clean output with step numbers and timing. Uses existing TrustEdge terminal symbols (✔ ✖ ⚠).

### Claude's Discretion
- Exact HTML/CSS styling of the verify page (keep it clean and professional)
- Whether to serve the verify page from the platform binary (e.g., static file handler) or deploy separately
- DigitalOcean App Platform spec details (instance size, region, health check config)
- Whether demo script downloads syft if not installed, or just errors with install instructions

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase 76 Implementation (endpoints this phase exposes)
- `crates/platform/src/http/handlers.rs` — verify_attestation_handler() for the endpoint the verify page calls
- `crates/platform/src/http/router.rs` — route registration pattern, verify_router with rate limiting

### Existing Deployment
- `deploy/Dockerfile` — Multi-stage Rust build for platform-server (debian-slim runtime)
- `deploy/docker-compose.yml` — Full stack (platform + postgres + dashboard)
- `deploy/.env.example` — Environment variable documentation

### Existing Demo Script
- `scripts/demo.sh` — Pattern for demo scripts (auto-detection, step output, timing)

### Design Documents
- `docs/designs/sbom-attestation-wedge.md` — CEO plan with deployment constraints
- `~/.gstack/projects/TrustEdge-Labs-trustedge/john-main-design-20260401-172433.md` — Design doc with verify page spec

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `deploy/Dockerfile` — existing multi-stage build, can be used as-is for DO App Platform
- `scripts/demo.sh` — patterns for auto-detection, colored output, step timing
- Platform server already serves /healthz for health checks

### Established Patterns
- Docker deployment: multi-stage build with debian-slim runtime, non-root user
- Demo scripts: auto-detect docker/local mode, colorized output with ✔/✖ symbols
- Platform: Axum with tower middleware, graceful shutdown

### Integration Points
- Verify page calls POST /v1/verify-attestation on the deployed platform
- Demo script calls `trst attest-sbom` and `trst verify-attestation` (from Phase 76)
- DO App Platform needs a Dockerfile and app spec

</code_context>

<specifics>
## Specific Ideas

- The verify page should feel like a professional tool page, not a toy demo. Think: clean white background, monospace for hashes, green/red for pass/fail, minimal text.
- The demo script should be copy-pasteable from the README. Someone reading the quick start should be able to run it immediately.
- DigitalOcean App Platform was chosen because the user's existing infrastructure is on DO.

</specifics>

<deferred>
## Deferred Ideas

- Third-party verification flow on verify page (attestation + binary + public key)
- Persistent receipt storage (SQLite or postgres)
- Custom domain for public verifier

</deferred>

---

*Phase: 77-verify-page-deployment-demo*
*Context gathered: 2026-04-02*
