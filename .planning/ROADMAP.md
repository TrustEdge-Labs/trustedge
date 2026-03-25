<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


# Roadmap: TrustEdge

## Milestones

- ✅ **v1.0 Consolidation** - Phases 1-8 (shipped 2026-02-11)
- ✅ **v1.1 YubiKey Integration Overhaul** - Phases 9-12 (shipped 2026-02-11)
- ✅ **v1.2 Scope Reduction** - Phases 13-14 (shipped 2026-02-12)
- ✅ **v1.3 Dependency Audit** - Phases 15-18 (shipped 2026-02-13)
- ✅ **v1.4 Placeholder Elimination** - Phases 19-23 (shipped 2026-02-13)
- ✅ **v1.5 Platform Consolidation** - Phases 24-27 (shipped 2026-02-22)
- ✅ **v1.6 Final Consolidation** - Phases 28-30 (shipped 2026-02-22)
- ✅ **v1.7 Security & Quality Hardening** - Phases 31-34 (shipped 2026-02-23)
- ✅ **v1.8 KDF Architecture Fix** - Phases 35-37 (shipped 2026-02-24)
- ✅ **v2.0 End-to-End Demo** - Phases 38-41 (shipped 2026-03-16)
- ✅ **v2.1 Data Lifecycle & Hardware Integration** - Phases 42-44 (shipped 2026-03-18)
- ✅ **v2.2 Security Remediation** - Phases 45-47 (shipped 2026-03-19)
- ✅ **v2.3 Security Testing** - Phases 48-51 (shipped 2026-03-21)
- ✅ **v2.4 Security Review Remediation** - Phases 52-53 (shipped 2026-03-22)
- ✅ **v2.5 Critical Security Fixes** - Phases 54-56 (shipped 2026-03-23)
- ✅ **v2.6 Security Hardening** - Phases 57-60 (shipped 2026-03-24)
- 🚧 **v2.7 CI & Config Security** - Phases 61-63 (in progress)

## Phases

<details>
<summary>v1.0-v1.8 (Phases 1-37) - See milestone archives</summary>

See `.planning/milestones/v1.0-ROADMAP.md` through `.planning/milestones/v1.8-ROADMAP.md` for full phase details.

</details>

<details>
<summary>v2.0 End-to-End Demo (Phases 38-41) - SHIPPED 2026-03-16</summary>

Delivered working end-to-end demonstration of TrustEdge's full value proposition. Generic archive profiles, one-command Docker stack, demo script, and README rewrite. 4 phases, 8 plans, 17/17 requirements complete.

**See:** `.planning/milestones/v2.0-ROADMAP.md` for full phase details.

</details>

<details>
<summary>v2.1 Data Lifecycle & Hardware Integration (Phases 42-44) - SHIPPED 2026-03-18</summary>

Completed the data lifecycle with decryption capability, exposed YubiKey hardware signing in the CLI, and added named archive profiles. 3 phases, 6 plans, 12/12 requirements complete.

**See:** `.planning/milestones/v2.1-ROADMAP.md` for full phase details.

</details>

<details>
<summary>v2.2 Security Remediation (Phases 45-47) - SHIPPED 2026-03-19</summary>

Fixed critical cryptographic flaws. RSA OAEP-SHA256 replaces PKCS#1 v1.5, v1 envelope format removed entirely, PBKDF2 minimum 300k iterations enforced, device keys encrypted at rest with passphrase protection. RUSTSEC-2023-0071 fully resolved. 3 phases, 5 plans, 8/8 requirements complete, 23 commits.

**See:** `.planning/milestones/v2.2-ROADMAP.md` for full phase details.

</details>


<details>
<summary>v2.3 Security Testing (Phases 48-51) - SHIPPED 2026-03-21</summary>

31 new security tests proving TrustEdge's tamper-evidence, nonce uniqueness, key protection, and replay resistance claims across 4 threat model categories (T1/T2/T3/T5/T6/T8). 4 phases, 4 plans, 12/12 requirements complete, 6 commits.

**See:** `.planning/milestones/v2.3-ROADMAP.md` for full phase details.

</details>

<details>
<summary>v2.4 Security Review Remediation (Phases 52-53) - SHIPPED 2026-03-22</summary>

Addressed all P1/P2 findings from the code & security review. Custom base64 replaced, key format versioned, timestamp replay fixed, envelope panics eliminated, key file permissions enforced, nonce overflow guarded, 14 error path tests added. 2 phases, 3 plans, 8/8 requirements complete, 19 commits.

**See:** `.planning/milestones/v2.4-ROADMAP.md` for full phase details.

</details>

<details>
<summary>v2.5 Critical Security Fixes (Phases 54-56) - SHIPPED 2026-03-23</summary>

Fixed 5 P0 security findings: QUIC TLS MITM vulnerability closed (real signature verification), 2 MB body limit + per-IP rate limiting on platform HTTP, JWKS signing key configurable (no more target/dev/ plaintext), WASM double-decrypt bug fixed. 3 phases, 4 plans, 8/8 requirements complete, 43 commits.

**See:** `.planning/milestones/v2.5-ROADMAP.md` for full phase details.

</details>

<details>
<summary>v2.6 Security Hardening (Phases 57-60) - SHIPPED 2026-03-24</summary>

Addressed 7 P1 security hardening findings: Zeroize on 4 key structs, 600k PBKDF2 import minimum, postgres verify fix, configurable CORS, CLI key leak prevention, nginx TLS termination, dashboard API key removed from bundle. 4 phases, 5 plans, 7/7 requirements complete, 39 commits.

**See:** `.planning/milestones/v2.6-ROADMAP.md` for full phase details.

</details>

### v2.7 CI & Config Security (In Progress)

**Milestone Goal:** Fix all 7 P0 security review findings — CI supply chain hardening, credential hygiene, and error information leakage.

- [x] **Phase 61: CI Supply Chain Hardening** - SHA-pin all GitHub Actions, remove curl-pipe installer, replace archived toolchain action (completed 2026-03-25)
- [x] **Phase 62: Config & Credential Hygiene** - Require explicit DATABASE_URL in release builds, remove postgres host port, reject placeholder JWT secret (completed 2026-03-25)
- [ ] **Phase 63: Error Response Sanitization** - Return generic crypto error messages to clients; log details server-side only

## Phase Details

### Phase 61: CI Supply Chain Hardening
**Goal**: CI workflows are protected against supply chain attacks — no unpinned action tags, no shell-pipe installers, no archived third-party actions
**Depends on**: Phase 60 (v2.6 complete)
**Requirements**: CISC-01, CISC-02, CISC-03
**Success Criteria** (what must be TRUE):
  1. All GitHub Actions across all 4 workflow files reference full commit SHAs, not mutable tags
  2. wasm-pack is installed without `curl | sh` (uses cargo-binstall, cargo install, or pre-built binary with checksum)
  3. `actions-rs/toolchain` is absent from all workflow files; `dtolnay/rust-toolchain` is used in its place
  4. CI passes after all workflow changes
**Plans:** 1/1 plans complete
Plans:
- [x] 61-01-PLAN.md — SHA-pin all actions, replace curl-pipe wasm-pack installer, replace actions-rs/toolchain

### Phase 62: Config & Credential Hygiene
**Goal**: Production deployments cannot start with hardcoded or placeholder credentials — database URL requires explicit config, postgres is not exposed to the host network, and the CA service rejects its default placeholder JWT secret
**Depends on**: Phase 60 (v2.6 complete)
**Requirements**: CONF-01, CONF-02, CONF-03
**Success Criteria** (what must be TRUE):
  1. The platform server fails to start in release builds when DATABASE_URL is not explicitly set (no hardcoded credential fallback)
  2. docker-compose.yml does not expose the postgres port to the host; the database is reachable only from containers on the internal network
  3. `CAConfig::default()` (or equivalent) panics or returns an error when the placeholder value `"your-secret-key"` is used as the JWT secret outside of test code
**Plans:** 1/1 plans complete
Plans:
- [x] 62-01-PLAN.md — Gate DATABASE_URL fallback, remove postgres host port, reject placeholder JWT secret

### Phase 63: Error Response Sanitization
**Goal**: Crypto verification errors never leak raw library error messages to API clients — clients receive a generic message, full details are logged server-side
**Depends on**: Phase 62
**Requirements**: ERRH-01
**Success Criteria** (what must be TRUE):
  1. A request that fails signature or integrity verification receives a generic error response (e.g., "verification failed") with no internal library detail
  2. The full error detail (library message, error chain) appears in server-side logs for the same failed request
  3. A successful verification response is unaffected by the sanitization change
**Plans:** 1 plan
Plans:
- [ ] 62-01-PLAN.md — Gate DATABASE_URL fallback, remove postgres host port, reject placeholder JWT secret

## Progress

| Phase | Milestone | Plans Complete | Status | Completed |
|-------|-----------|----------------|--------|-----------|
| 61. CI Supply Chain Hardening | v2.7 | 1/1 | Complete    | 2026-03-25 |
| 62. Config & Credential Hygiene | v2.7 | 1/1 | Complete    | 2026-03-25 |
| 63. Error Response Sanitization | v2.7 | 0/TBD | Not started | - |

---
*Last updated: 2026-03-25 after Phase 62 planned (1 plan)*
