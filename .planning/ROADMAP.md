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
- ✅ **v1.8 KDF Architecture Fix** - Phases 35-37 (shipped 2026-02-24)
- ✅ **v2.0 End-to-End Demo** - Phases 38-41 (shipped 2026-03-16)
- ✅ **v2.1 Data Lifecycle** - Phases 42-44 (shipped 2026-03-18)
- ✅ **v2.2 Security Remediation** - Phases 45-47 (shipped 2026-03-19)
- ✅ **v2.3 Security Testing** - Phases 48-51 (shipped 2026-03-21)
- ✅ **v2.4 Security Review Remediation** - Phases 52-53 (shipped 2026-03-22)
- ✅ **v2.5 Critical Security Fixes** - Phases 54-56 (shipped 2026-03-23)
- ✅ **v2.6 Security Hardening** - Phases 57-60 (shipped 2026-03-24)
- ✅ **v2.7 CI & Config Security** - Phases 61-63 (shipped 2026-03-25)
- 🚧 **v2.8 High Priority Hardening** - Phases 64-67 (in progress)

## Phases

### v2.8 High Priority Hardening (In Progress)

**Milestone Goal:** Fix 9 P1 security review findings across rate limiting proxy awareness, key material safety, nonce construction, CLI hardening, and deployment security.

- [x] **Phase 64: Platform HTTP Hardening** - Rate limiter reads real client IP from X-Forwarded-For behind trusted proxies; 429 responses include Retry-After header (completed 2026-03-25)
- [x] **Phase 65: Key Material Safety** - Auto-generated key files get 0600 permissions; PrivateKey serde derives removed to prevent accidental serialization (completed 2026-03-25)
- [ ] **Phase 66: Crypto & CLI Hardening** - NetworkChunk::new() requires mandatory nonce; process::exit() replaced with error returns; chunk-size ceiling enforced
- [ ] **Phase 67: Deployment Security** - Dashboard nginx runs as non-root; CI workflow guards against credential leakage in bundles

## Phase Details

### Phase 64: Platform HTTP Hardening
**Goal**: The rate limiter correctly identifies clients behind reverse proxies and communicates retry timing to callers
**Depends on**: Nothing (first phase of milestone)
**Requirements**: HTTP-01, HTTP-02
**Success Criteria** (what must be TRUE):
  1. A request arriving with X-Forwarded-For from a configured trusted proxy is rate-limited against the forwarded client IP, not the proxy IP
  2. When a client is rate-limited, the 429 response includes a Retry-After header with a positive integer value
  3. A test or integration check proves both behaviors — trusted-proxy IP extraction and Retry-After header presence
**Plans**: 1 plan
Plans:
- [x] 64-01-PLAN.md — Proxy-aware rate limiting with Retry-After header

### Phase 65: Key Material Safety
**Goal**: Key files created by trst wrap have restrictive permissions and PrivateKey cannot be accidentally serialized to JSON or other formats
**Depends on**: Phase 64
**Requirements**: KEY-01, KEY-02
**Success Criteria** (what must be TRUE):
  1. After trst wrap auto-generates a key file, the file's Unix permissions are 0600 (matching keygen behavior)
  2. Attempting to serialize a PrivateKey value (via serde_json::to_string or equivalent) either fails to compile or produces no key material output
  3. A test verifies the 0600 permission is set on the auto-generated key path
**Plans**: 1 plan
Plans:
- [x] 65-01-PLAN.md — Key file permissions and PrivateKey serde removal

### Phase 66: Crypto & CLI Hardening
**Goal**: NetworkChunk construction requires an explicit nonce, process exits in the CLI surface proper errors, and chunk sizes are bounded
**Depends on**: Phase 65
**Requirements**: CRYPT-01, CLI-01, CLI-02
**Success Criteria** (what must be TRUE):
  1. NetworkChunk::new() signature requires a nonce argument — callers that previously relied on zero-nonce default fail to compile until updated
  2. All 11 process::exit() call sites in trst-cli are replaced with error propagation; the CLI binary exits non-zero on errors without calling std::process::exit directly
  3. Passing --chunk-size with a value above 256 MB (268435456 bytes) to trst wrap produces a clear error message and non-zero exit, not a panic or silent truncation
**Plans**: 2 plans
Plans:
- [ ] 66-01-PLAN.md — Mandatory nonce for NetworkChunk constructor
- [ ] 66-02-PLAN.md — CLI process::exit removal and chunk-size ceiling

### Phase 67: Deployment Security
**Goal**: The dashboard container runs nginx as a non-root user and the CI workflow prevents credential leakage into production bundles
**Depends on**: Phase 66
**Requirements**: DEPL-01, DEPL-02
**Success Criteria** (what must be TRUE):
  1. The dashboard Docker image runs the nginx process as a non-root user (nginx-unprivileged base or explicit USER directive)
  2. The GitHub Actions ci.yml workflow includes a step that greps the dashboard build output for credential patterns and fails the build if any are found
  3. A docker inspect or image metadata check confirms no root-run nginx process in the container
**Plans**: TBD
**UI hint**: yes

## Progress

**Execution Order:**
Phases execute in numeric order: 64 → 65 → 66 → 67

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 64. Platform HTTP Hardening | 1/1 | Complete    | 2026-03-25 |
| 65. Key Material Safety | 1/1 | Complete    | 2026-03-25 |
| 66. Crypto & CLI Hardening | 0/2 | Not started | - |
| 67. Deployment Security | 0/TBD | Not started | - |
