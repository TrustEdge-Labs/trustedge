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
- 🔄 **v2.6 Security Hardening** - Phases 57-60 (active)

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

### v2.6 Security Hardening (Phases 57-60) — Active

- [ ] **Phase 57: Core Crypto Hardening** - Zeroize key-holding structs and enforce PBKDF2 import minimum
- [ ] **Phase 58: Platform Fixes** - Fix postgres verify handler and make CORS origins configurable
- [ ] **Phase 59: CLI & Deploy Hardening** - Suppress key stderr output and add nginx TLS termination
- [ ] **Phase 60: Dashboard Security** - Remove client-side API key from bundle

## Phase Details

### Phase 57: Core Crypto Hardening
**Goal**: Sensitive key material is zeroed from memory when dropped and weak key imports are rejected at the boundary
**Depends on**: Nothing (first phase of v2.6)
**Requirements**: CORE-01, CORE-02
**Success Criteria** (what must be TRUE):
  1. Dropping a `PrivateKey`, `SessionInfo`, `ClientAuthResult`, or `SymmetricKey` instance causes its key bytes to be overwritten in memory (Zeroize + ZeroizeOnDrop implemented)
  2. Calling `import_secret_encrypted()` with a key file containing fewer than 300,000 PBKDF2 iterations returns an error — the key is never loaded
  3. Calling `import_secret_encrypted()` with a key file containing 300,000 or more iterations succeeds as before
  4. All existing tests continue to pass after the zeroize additions
**Plans**: TBD

### Phase 58: Platform Fixes
**Goal**: The platform verification endpoint works correctly in postgres mode and CORS policy is configurable for production deployments
**Depends on**: Phase 57
**Requirements**: PLAT-01, PLAT-02
**Success Criteria** (what must be TRUE):
  1. A POST to `/v1/verify` in postgres mode succeeds and returns a receipt without requiring `OrgContext` to be injected by auth middleware
  2. Setting `CORS_ORIGINS=https://app.example.com` causes the platform to allow that origin and reject unlisted origins
  3. Without `CORS_ORIGINS` set, the platform falls back to a safe default (same-origin / localhost only)
  4. The existing HTTP verify integration tests continue to pass
**Plans**: TBD

### Phase 59: CLI & Deploy Hardening
**Goal**: The CLI never leaks key material to stderr in normal operation and the Docker deployment stack supports HTTPS
**Depends on**: Phase 57
**Requirements**: CLI-01, DEPL-01
**Success Criteria** (what must be TRUE):
  1. Running `trustedge` (without `--show-key`) produces no AES key output on stderr
  2. Running `trustedge --show-key` displays the key on stderr as before
  3. The nginx configuration in the Docker stack accepts HTTPS connections on port 443 when certificate paths are configured via environment variables
  4. HTTP on port 80 continues to work (or redirects to HTTPS) in the Docker stack
**Plans**: TBD

### Phase 60: Dashboard Security
**Goal**: The dashboard JavaScript bundle contains no embedded API credentials; authentication to the platform is not exposed client-side
**Depends on**: Phase 58
**Requirements**: DASH-01
**Success Criteria** (what must be TRUE):
  1. Building the dashboard with `npm run build` produces no bundle file containing the string `VITE_API_KEY` as a value (the key is not embedded)
  2. The dashboard can still communicate with the platform API after the change (either via proxy, token removal, or equivalent)
  3. CI or a build-time check catches any future re-introduction of a client-side API key in the bundle
**Plans**: TBD
**UI hint**: yes

## Progress Table

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 57. Core Crypto Hardening | 0/? | Not started | - |
| 58. Platform Fixes | 0/? | Not started | - |
| 59. CLI & Deploy Hardening | 0/? | Not started | - |
| 60. Dashboard Security | 0/? | Not started | - |

---
*Last updated: 2026-03-23 after v2.6 roadmap created*
