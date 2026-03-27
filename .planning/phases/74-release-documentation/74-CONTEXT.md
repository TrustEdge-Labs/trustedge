# Phase 74: Release Documentation - Context

**Gathered:** 2026-03-27
**Status:** Ready for planning

<domain>
## Phase Boundary

Comprehensive documentation sweep to ensure all user-facing docs accurately reflect the v3.0 codebase. Covers README, CLAUDE.md, docs/ subdirectories, and demo instructions. This is the final phase before the official v3.0 signed release.

</domain>

<decisions>
## Implementation Decisions

### Documentation Scope (DOCS-01 + DOCS-02)
- **D-01:** Audit and update ALL user-facing documentation:
  - `README.md` — quick-start commands, feature list, architecture summary, version references
  - `CLAUDE.md` — CLI binary tables, feature flag tables, test commands, build commands, architecture overview
  - `docs/` — architecture.md, yubikey-guide.md, subdirectories (developer, hardware, legal, technical, user)
  - `scripts/demo.sh` — ensure demo instructions match current CLI flags and behavior
  - `deploy/.env.example` — usage instructions (created in Phase 73)

- **D-02:** Focus on accuracy, not rewriting. If a section is accurate, leave it. If a section references old behavior (pre-v2.4 patterns, removed features, changed flags), update it. Key changes since last README rewrite (v2.0):
  - Encrypted key files (TRUSTEDGE-KEY-V1) with --unencrypted escape hatch (v2.2)
  - Named profiles: sensor, audio, log (v2.1)
  - trst unwrap command (v2.1)
  - Platform: configurable RECEIPT_TTL_SECS, no version in /healthz, strict PORT validation (v3.0)
  - Docker Compose now requires `cp deploy/.env.example deploy/.env` before `docker compose up` (v3.0)
  - Envelope::hash() returns Result (v3.0)

### Claude's Discretion
- Which docs/ files actually need updates vs are already current
- Whether to consolidate or reorganize docs/ structure
- Level of detail for new features in README vs docs/

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Root docs
- `README.md` — Main project README (160 lines, last rewritten v2.0)
- `CLAUDE.md` — Developer/AI guide with build commands, architecture, patterns

### docs/ directory
- `docs/architecture.md` — Architecture documentation
- `docs/yubikey-guide.md` — YubiKey hardware guide
- `docs/developer/` — Developer documentation
- `docs/hardware/` — Hardware-related docs
- `docs/legal/` — Legal docs (likely MPL-2.0 related)
- `docs/technical/` — Technical deep-dives
- `docs/user/` — User-facing guides
- `docs/README.md` — Docs index

### Scripts
- `scripts/demo.sh` — Demo script with keygen/wrap/verify lifecycle

### Deploy
- `deploy/.env.example` — New env file template (created Phase 73)
- `deploy/docker-compose.yml` — Updated to use env_file (Phase 73)

</canonical_refs>

<code_context>
## Existing Code Insights

### Key Changes Since Last Docs Update (v2.0)
- v2.1: trst unwrap, named profiles (sensor/audio/log), YubiKey CLI
- v2.2: Encrypted keys at rest, --unencrypted flag, RSA OAEP
- v2.3-v2.4: Security tests, error path tests, base64 replacement
- v2.5-v2.9: Rate limiting, CORS, zeroize, CI hardening, nginx security
- v3.0: Receipt TTL configurable, healthz clean, PORT strict, crypto hygiene, deployment hardening

### Established Patterns
- README focuses on quick-start and use cases (128 lines, 4 use cases, 3-command start)
- CLAUDE.md is the comprehensive developer reference
- docs/ has subdirectories by audience (developer, user, technical)

### Integration Points
- Demo script uses `trst keygen`, `trst wrap`, `trst verify` — these now require passphrase or --unencrypted
- Docker quick-start now requires `cp .env.example .env` step

</code_context>

<specifics>
## Specific Ideas

No specific requirements — comprehensive audit with accuracy focus, not rewriting for style.

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope

</deferred>

---

*Phase: 74-release-documentation*
*Context gathered: 2026-03-27*
