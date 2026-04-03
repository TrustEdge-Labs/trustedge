<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


# Phase 40: Demo Script - Context

**Gathered:** 2026-03-15
**Status:** Ready for planning

<domain>
## Phase Boundary

A single `./scripts/demo.sh` script that shows the complete TrustEdge lifecycle: key generation, data wrapping into a .trst archive, submission to the verification server, and receipt. Works against both the docker-compose stack and local cargo builds. This phase does NOT add new CLI features beyond `trst keygen`.

</domain>

<decisions>
## Implementation Decisions

### Demo Lifecycle Steps
1. Generate Ed25519 device key pair (`trst keygen`)
2. Create sample data (random bytes via `dd`, ~10KB)
3. Wrap data into .trst archive (`trst wrap --profile generic`)
4. Submit to platform verification server (`trst emit-request --post http://localhost:3001/v1/verify`)
5. Run local verification (`trst verify`)
6. Display final PASS/FAIL result

### Key Generation
- Add `trst keygen` subcommand to trst-cli: generates Ed25519 key pair to `--out-key` and `--out-pub` files
- Demo script calls `trst keygen` as its first step
- No pre-bundled test keys — every demo run generates fresh keys

### Sample Data
- Script generates random bytes: `dd if=/dev/urandom bs=1K count=10` (or similar)
- No pre-bundled sample files needed
- No user input required (DEMO-04: generates its own sample data)

### Verification Strategy
- Both server and local verification in docker mode
- Server verify: `trst emit-request --post http://localhost:3001/v1/verify` — show receipt
- Local verify: `trst verify` — show PASS/FAIL
- In local-only mode: skip server verification, print note that it requires docker-compose stack

### Output & Presentation
- Colored step banners: `[Step N/M] Description...` with ANSI colors
- Per-step checkmarks: `✔ Created device.key and device.pub`
- Final summary banner: `DEMO COMPLETE - ALL PASSED` or `DEMO FAILED` with which step failed
- Script exits with 0 on all pass, 1 on any failure (DEMO-03: clear PASS/FAIL)

### Artifact Handling
- All demo artifacts go into a `demo-output/` directory (relative to CWD)
- Artifacts kept after demo for user inspection: keys, archive, receipt
- Final output prints the directory path

### Docker vs Local Mode
- Auto-detect: script checks if platform server is reachable at localhost:3001
- If reachable: full demo with server verification
- If not reachable: local-only demo (skip server verify step, print note)
- Override flags: `--local` (force local), `--docker` (force docker, error if stack not running)
- In local mode, uses `cargo run -p trustedge-trst-cli --` to invoke trst
- In docker mode, uses pre-built `trst` binary or `cargo run`

### Claude's Discretion
- Exact ANSI color codes and formatting
- How to detect platform server availability (curl vs wget vs /dev/tcp)
- Whether to use `set -e` or per-command error handling
- Temp directory strategy within demo-output/
- How `trst keygen` outputs the public key (file format: raw bytes vs ed25519:base64 prefix)

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### CLI tools
- `crates/trst-cli/src/main.rs` — trst CLI entry point (wrap, verify, emit-request subcommands)
- `crates/core/src/crypto.rs` — Ed25519 key generation functions
- `crates/trst-cli/tests/acceptance.rs` — Acceptance tests showing CLI usage patterns

### Deployment
- `deploy/docker-compose.yml` — Three-service stack (postgres, platform-server, dashboard)
- `deploy/.env.example` — Environment variable defaults

### Archive format
- `crates/trst-protocols/src/archive/manifest.rs` — TrstManifest with generic/cam.video profiles

### Requirements
- `.planning/REQUIREMENTS.md` — DEMO-01 through DEMO-04 define success criteria

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `trst wrap`: Already supports `--device-key` and `--device-pub` for signing
- `trst verify`: Accepts `--device-pub` with `ed25519:<base64>` format and `--json` output
- `trst emit-request`: Has `--post <URL>` for HTTP submission to platform server
- `trustedge_core::crypto`: Has Ed25519 key generation (`ed25519_dalek::SigningKey`)
- `scripts/ci-check.sh`: Example of structured bash script with step banners

### Established Patterns
- CLI uses clap for argument parsing
- Key format on CLI: `ed25519:<base64>` for public keys
- Device key files: raw Ed25519 private key bytes
- Archive output: directory with manifest.json, signatures/, chunks/

### Integration Points
- `crates/trst-cli/src/main.rs` — Add `keygen` subcommand
- `scripts/demo.sh` — New file
- Platform server `/v1/verify` endpoint — POST target for server verification

</code_context>

<specifics>
## Specific Ideas

- The preview output style shown during discussion is the target: `[Step N/M]` with checkmarks
- `trst emit-request --post` already exists — the demo just needs to use it
- In local mode, the demo should still show all steps except server verification, with a clear message about what's skipped

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope.

</deferred>

---

*Phase: 40-demo-script*
*Context gathered: 2026-03-15*
