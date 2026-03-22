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
- **v2.4 Security Review Remediation** - Phases 52-53 (active)

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

### v2.4 Security Review Remediation (Phases 52-53)

- [x] **Phase 52: Code Hardening** - Replace unsafe patterns, fix timestamp replay, enforce key permissions, guard nonce overflow (completed 2026-03-22)
- [x] **Phase 53: Error Path Tests** - Negative tests for passphrase errors, malformed metadata, and clock skew rejection (completed 2026-03-22)

## Phase Details

### Phase 52: Code Hardening
**Goal**: All P1/P2 code-level findings from the security review are fixed — standard library used for base64, key file format versioned, timestamp check unidirectional, panic paths eliminated from security code, key files protected by OS permissions, and nonce overflow guarded.
**Depends on**: Nothing (first phase of v2.4)
**Requirements**: CRYP-01, CRYP-02, AUTH-01, AUTH-02, KEYF-01, KEYF-02
**Success Criteria** (what must be TRUE):
  1. Running `trst keygen` produces a key file readable only by the owner (mode 0600 on Unix); other-user read attempts fail with permission denied.
  2. The encrypted key file JSON contains a version field and an iteration count field that a reader can inspect without decrypting.
  3. Attempting to wrap an archive whose chunk count would exceed 2^32 returns an explicit error instead of silently wrapping a broken or panicking archive.
  4. Auth handshake with a response timestamp in the future is rejected; a timestamp slightly in the past within the tolerance window is accepted.
  5. `cargo clippy` and `cargo test --workspace` pass with no panics introduced by the changed code paths (no unwrap/expect in auth.rs or envelope.rs security paths).
**Plans:** 2/2 plans complete

Plans:
- [x] 52-01-PLAN.md — Replace custom base64, version key file format, fix auth timestamp
- [x] 52-02-PLAN.md — Eliminate envelope panics, guard nonce overflow, enforce key file permissions

### Phase 53: Error Path Tests
**Goal**: All negative/error paths introduced or exposed by Phase 52 are covered by automated tests that actively exercise the rejection behavior — wrong passphrase, truncated key files, corrupted key JSON, malformed archive metadata, and clock skew rejection.
**Depends on**: Phase 52
**Requirements**: TEST-01, TEST-02
**Success Criteria** (what must be TRUE):
  1. A test that feeds a wrong passphrase to key file decryption receives a typed error (not a panic or generic IO error) and the test passes.
  2. A test that truncates a key file at multiple byte boundaries confirms the parser returns a descriptive error at every truncation point without panicking.
  3. A test that corrupts specific JSON fields in an encrypted key file confirms each variant is rejected with a distinct, actionable error message.
  4. A test for archive unwrap with malformed profile metadata (e.g., missing required sensor fields) receives a parse error before any decryption is attempted.
  5. A test for the auth handshake with a clock-skewed future timestamp receives a rejection error matching the AUTH-01 enforcement from Phase 52.
**Plans:** 1/1 plans complete

Plans:
- [x] 53-01-PLAN.md — Key file error paths, sensor metadata validation, auth clock skew rejection

## Progress Table

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 52. Code Hardening | 2/2 | Complete    | 2026-03-22 |
| 53. Error Path Tests | 1/1 | Complete   | 2026-03-22 |

---
*Last updated: 2026-03-22 after phase 53 planning*
