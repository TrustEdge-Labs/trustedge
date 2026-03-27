<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->

# Roadmap: TrustEdge

## Milestones

- ✅ **v1.0 Consolidation** - Phases 1-8 (shipped 2026-02-11)
- ✅ **v1.1 YubiKey Overhaul** - Phases 9-12 (shipped 2026-02-11)
- ✅ **v1.2 Scope Reduction** - Phases 13-14 (shipped 2026-02-12)
- ✅ **v1.3 Dependency Audit** - Phases 15-18 (shipped 2026-02-13)
- ✅ **v1.4 Placeholder Elimination** - Phases 19-23 (shipped 2026-02-13)
- ✅ **v1.5 Platform Consolidation** - Phases 24-27 (shipped 2026-02-22)
- ✅ **v1.6 Final Consolidation** - Phases 28-30 (shipped 2026-02-22)
- ✅ **v1.7 Security Hardening** - Phases 31-34 (shipped 2026-02-23)
- ✅ **v1.8 KDF Fix** - Phases 35-37 (shipped 2026-02-24)
- ✅ **v2.0 End-to-End Demo** - Phases 38-41 (shipped 2026-03-16)
- ✅ **v2.1 Data Lifecycle** - Phases 42-44 (shipped 2026-03-18)
- ✅ **v2.2 Security Remediation** - Phases 45-47 (shipped 2026-03-19)
- ✅ **v2.3 Security Testing** - Phases 48-51 (shipped 2026-03-21)
- ✅ **v2.4 Security Review Remediation** - Phases 52-53 (shipped 2026-03-22)
- ✅ **v2.5 Critical Security Fixes** - Phases 54-56 (shipped 2026-03-23)
- ✅ **v2.6 Security Hardening** - Phases 57-60 (shipped 2026-03-24)
- ✅ **v2.7 CI & Config Security** - Phases 61-63 (shipped 2026-03-25)
- ✅ **v2.8 High Priority Hardening** - Phases 64-67 (shipped 2026-03-26)
- ✅ **v2.9 Security Review P2 Remediation** - Phases 68-70 (shipped 2026-03-26)
- 🚧 **v3.0 Release Polish** - Phases 71-74 (in progress)

## Phases

### v3.0 Release Polish (In Progress)

**Milestone Goal:** Resolve remaining P2 security findings and prepare for official v3.0 signed release — configurable TTL, version fingerprint removal, crypto path hardening, deployment secrets, nginx header coverage, and documentation sweep.

- [x] **Phase 71: Platform Code Quality** - Make receipt TTL configurable, remove version fingerprint from healthz, and fail fast on invalid PORT (completed 2026-03-26)
- [ ] **Phase 72: Core Crypto Hygiene** - Replace unwrap paths in production crypto code with explicit error handling
- [ ] **Phase 73: Deployment Hardening** - Add missing nginx security headers and move Docker Compose secrets out of plaintext
- [ ] **Phase 74: Release Documentation** - Update README and user-facing docs to reflect v3.0 state

## Phase Details

### Phase 71: Platform Code Quality
**Goal**: Platform configuration is explicit, observable, and fails loudly on misconfiguration
**Depends on**: Nothing (first phase of v3.0)
**Requirements**: PLAT-01, PLAT-02, PLAT-03
**Success Criteria** (what must be TRUE):
  1. Setting `RECEIPT_TTL_SECS=7200` causes issued JWS receipts to expire 2 hours after issuance
  2. A `GET /healthz` request without authentication returns a response that omits the exact crate version string
  3. Starting the server with `PORT=notanumber` prints a clear error message and exits non-zero instead of silently binding to 3001
**Plans**: 1 plan
Plans:
- [x] 71-01-PLAN.md — Configurable receipt TTL, clean healthz, strict PORT parsing

### Phase 72: Core Crypto Hygiene
**Goal**: Production crypto paths surface failures explicitly rather than silently swallowing errors
**Depends on**: Phase 71
**Requirements**: CORE-01, CORE-02
**Success Criteria** (what must be TRUE):
  1. `generate_aad()` uses `.expect("AAD serialization is infallible")` — the intent is documented, not hidden by a bare `.unwrap()`
  2. `Envelope::hash()` returns `Result` so callers receive an error instead of an empty-input hash when serialization fails
  3. `cargo clippy -- -D warnings` passes with no new suppressions added
**Plans**: TBD

### Phase 73: Deployment Hardening
**Goal**: nginx configs apply security headers consistently and Docker Compose does not store database credentials in plaintext
**Depends on**: Phase 71
**Requirements**: DEPL-01, DEPL-02, DEPL-03
**Success Criteria** (what must be TRUE):
  1. A response from any nginx-proxied location block includes X-Content-Type-Options, X-Frame-Options, Referrer-Policy, and Content-Security-Policy headers in both `nginx.conf` and `nginx-ssl.conf.template`
  2. The CSP `connect-src` directive in the dashboard nginx config includes the configured API origin so dashboard API calls are not browser-blocked
  3. The Docker Compose file contains no inline plaintext database password — credentials are sourced from `env_file` or Docker secrets
**Plans**: TBD
**UI hint**: yes

### Phase 74: Release Documentation
**Goal**: Documentation accurately reflects the v3.0 codebase so users and contributors have a reliable reference
**Depends on**: Phase 73
**Requirements**: DOCS-01, DOCS-02
**Success Criteria** (what must be TRUE):
  1. README quick-start commands run successfully against the current codebase without modification
  2. CLI command tables in CLAUDE.md match the actual binary flags and subcommands that exist in the codebase
  3. Demo script instructions produce the expected output when followed from a clean checkout
**Plans**: TBD

## Progress

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 71. Platform Code Quality | 1/1 | Complete    | 2026-03-27 |
| 72. Core Crypto Hygiene | 0/TBD | Not started | - |
| 73. Deployment Hardening | 0/TBD | Not started | - |
| 74. Release Documentation | 0/TBD | Not started | - |
