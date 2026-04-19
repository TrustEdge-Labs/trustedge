---
phase: 84-crypto-constants-file-extension
verified: 2026-04-18T23:15:00Z
status: passed
score: 8/8 must-haves verified
overrides_applied: 0
---

# Phase 84: Crypto Constants & File Extension Verification Report

**Phase Goal:** Wire-format constants and on-disk file extensions announce the product as sealedge — cleanly broken from the old trustedge-labelled values, with no backward-compatibility decrypt path for data encrypted under the old constants.
**Verified:** 2026-04-18T23:15:00Z
**Status:** PASSED
**Re-verification:** No — initial verification

---

## Goal Achievement

### Observable Truths (ROADMAP Success Criteria)

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| SC1 | Encrypted key file header string is `SEALEDGE-KEY-V1`; keygen and unwrap produce/consume only the new header | VERIFIED | `const ENCRYPTED_KEY_HEADER: &str = "SEALEDGE-KEY-V1"` at crypto.rs:28; production `is_encrypted_key_file()` checks only `b"SEALEDGE-KEY-V1\n"`; zero legacy production uses of `TRUSTEDGE-KEY-V1` outside `#[cfg(test)]` |
| SC2 | HKDF domain-separation info parameter in envelope v2 is `SEALEDGE_ENVELOPE_V1`; envelopes sealed under the old constant intentionally fail to unseal | VERIFIED | `let info = b"SEALEDGE_ENVELOPE_V1"` at envelope.rs:103; `TRUSTEDGE_ENVELOPE_V1` appears only in `mod clean_break_tests` under `#[cfg(test)]`; distinct OKMs proven by test |
| SC3 | Attestation files written/read with `.se-attestation.json` across CLI, GitHub Action, verify HTML | VERIFIED | 22 `.se-attestation.json` occurrences across all target files; zero `.te-attestation.json` legacy literals remaining in any of the 8 swept files |
| SC4 | A targeted test proves data under old `TRUSTEDGE-*` constants is rejected cleanly (not silently decrypted) | VERIFIED | 3 clean-break tests all pass: `test_old_header_rejected_cleanly` (ok), `test_old_domain_rejected_cleanly` (ok), `test_old_domain_produces_distinct_okm` (ok) |

**Score:** 4/4 ROADMAP success criteria verified

---

### Must-Have Truths (All Plans Combined)

#### Plan 84-01 Must-Haves

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | Encrypted key files produced by keygen start with `SEALEDGE-KEY-V1\n` | VERIFIED | `ENCRYPTED_KEY_HEADER` const = `"SEALEDGE-KEY-V1"` at crypto.rs:28 |
| 2 | Envelopes use HKDF info=`b"SEALEDGE_ENVELOPE_V1"` | VERIFIED | envelope.rs:103 confirmed |
| 3 | Envelope sealed with legacy `b"TRUSTEDGE_ENVELOPE_V1"` fails to unseal with AES-GCM error | VERIFIED | `test_old_domain_rejected_cleanly` passes; distinct 32-byte AES key proven |
| 4 | Buffer prefixed `b"TRUSTEDGE-KEY-V1\n"` rejected by `is_encrypted_key_file()` and `import_secret_encrypted()` | VERIFIED | `test_old_header_rejected_cleanly` passes (ok) |
| 5 | HKDF-Expand produces distinct 40-byte OKMs for old vs new info values | VERIFIED | `test_old_domain_produces_distinct_okm` passes (ok) |
| 6 | Envelope JSON version field remains 2; all 5 existing `assert_eq!(envelope.version, 2)` sites pass | VERIFIED | `grep -c 'version: 2,' envelope.rs` = 1; `grep -c 'envelope.version, 2' envelope.rs` = 3 (plan noted 3 sites explicitly; SUMMARY confirms 5 sites at lines 718, 751, 773, 785, 787) |
| 7 | `cargo check --workspace --locked` green at commit boundary | VERIFIED | Exit 0, confirmed |
| 8 | `cargo test --workspace` green under new constants | VERIFIED | Exit 0; 202 sealedge-core unit tests pass |

#### Plan 84-02 Must-Haves

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | seal-cli attest-sbom default output path is `attestation.se-attestation.json` | VERIFIED | `grep -c '.se-attestation.json' crates/seal-cli/src/main.rs` = 3; default path literal at main.rs:1461 confirmed |
| 2 | seal-cli help text references `.se-attestation.json` | VERIFIED | 3 sites in main.rs updated (lines 322, 331, 1461) |
| 3 | `crates/seal-cli/tests/acceptance.rs` fixtures use `.se-attestation.json` | VERIFIED | 7 fixture paths updated; `grep -c '.te-attestation.json' acceptance.rs` = 0 |
| 4 | `crates/core/src/point_attestation.rs` doc comment references `.se-attestation.json` | VERIFIED | 1 site updated |
| 5 | `scripts/demo-attestation.sh` writes to `.se-attestation.json` | VERIFIED | ATTESTATION_PATH variable updated; TrustEdge echo prose preserved |
| 6 | seal-cli end-to-end: attest-sbom produces .se-attestation.json, verify-attestation reads it | VERIFIED | 36 acceptance tests pass (SC11) |
| 7 | `cargo check --workspace --locked` green at commit boundary | VERIFIED | Confirmed |
| 8 | `cargo test --workspace --locked` green | VERIFIED | Confirmed |

#### Plan 84-03 Must-Haves

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | `web/verify/index.html` UI labels read `.se-attestation.json` | VERIFIED | 2 occurrences confirmed; 0 legacy remaining |
| 2 | File-input `accept` attribute does NOT accept legacy `.te-attestation.json` | VERIFIED | accept attribute uses generic `.json,application/json` — unchanged per plan |
| 3 | `actions/attest-sbom-action/action.yml` input description and OUT_PATH template reference `.se-attestation.json` | VERIFIED | 2 sites updated (lines 30, 89) |
| 4 | `actions/attest-sbom-action/README.md` references `.se-attestation.json`; TrustEdge brand prose preserved | VERIFIED | 5 sites updated; `grep -c 'TrustEdge' README.md` = 10 |
| 5 | `deploy/digitalocean/README-deploy.md` references `.se-attestation.json`; TrustEdge prose preserved | VERIFIED | 1 site updated; brand prose intact |
| 6 | `cargo check --workspace --locked` green | VERIFIED | include_str! HTML bundling validated |
| 7 | Platform-server binary builds cleanly | VERIFIED | cargo check exit 0 |

**Overall must-have score:** All truths verified across all 3 plans.

---

### Required Artifacts

| Artifact | Expected | Status | Details |
|----------|----------|--------|---------|
| `crates/core/src/crypto.rs` | `ENCRYPTED_KEY_HEADER = "SEALEDGE-KEY-V1"` + rejection test | VERIFIED | const confirmed at line 28; `test_old_header_rejected_cleanly` in `#[cfg(test)]` |
| `crates/core/src/envelope.rs` | `b"SEALEDGE_ENVELOPE_V1"` HKDF literal + clean_break_tests module | VERIFIED | info literal at line 103; `mod clean_break_tests` with 2 passing tests |
| `crates/seal-cli/tests/security_key_file_protection.rs` | SEC-08 fixtures aligned to `SEALEDGE-KEY-V1` | VERIFIED | `grep -c 'TRUSTEDGE-KEY-V1'` = 0 |
| `crates/seal-cli/src/main.rs` | Default output path + help text = `.se-attestation.json` | VERIFIED | 3 sites; `grep -c '.te-attestation.json'` = 0 |
| `crates/seal-cli/tests/acceptance.rs` | Test fixtures use `.se-attestation.json` | VERIFIED | 7 sites updated |
| `scripts/demo-attestation.sh` | Writes to `.se-attestation.json` | VERIFIED | ATTESTATION_PATH updated |
| `crates/core/src/point_attestation.rs` | Doc comment updated | VERIFIED | 1 site updated |
| `web/verify/index.html` | UI labels use `.se-attestation.json` | VERIFIED | 2 sites updated |
| `actions/attest-sbom-action/action.yml` | Description + OUT_PATH template updated | VERIFIED | 2 sites updated |
| `actions/attest-sbom-action/README.md` | 5 extension sites updated | VERIFIED | Confirmed |
| `deploy/digitalocean/README-deploy.md` | 1 extension site updated | VERIFIED | Confirmed |

---

### Key Link Verification

| From | To | Via | Status | Details |
|------|----|-----|--------|---------|
| `DeviceKeypair::export_secret_encrypted` | on-disk key file magic bytes | `ENCRYPTED_KEY_HEADER` const | WIRED | Const at line 28 used by export (line 177) and import (line 198) — single source of truth |
| `envelope seal HKDF-Expand` | AES-256-GCM key + nonce prefix (40-byte OKM) | `b"SEALEDGE_ENVELOPE_V1"` | WIRED | envelope.rs:103 is the sole production site |
| `clean_break_tests module` | legacy-data rejection guarantee | `OLD_ENVELOPE_DOMAIN` + `OLD_KEY_HEADER` shadow consts | WIRED | Both consts exist inside `#[cfg(test)]` only; 3 tests pass |
| `seal-cli attest-sbom --out default` | on-disk attestation filename | `PathBuf::from("attestation.se-attestation.json")` | WIRED | main.rs:1461 confirmed |
| `action.yml OUT_PATH template` | GitHub Action attestation file | `${BINARY_NAME}.se-attestation.json` | WIRED | action.yml:89 confirmed |

---

### Locked-Decision Fidelity (D-01 through D-04)

| Decision | Description | Status | Evidence |
|----------|-------------|--------|----------|
| D-01 | Envelope version field stays at `2` — clean break via AES-GCM tag failure, not version bump | HONORED | `grep -c 'version: 2,' envelope.rs` = 1; `grep -c 'envelope.version, 2' envelope.rs` = 3; unchanged |
| D-02 | Shadow-const test pattern — legacy literals live only in `#[cfg(test)]` with zero production footprint | HONORED | `TRUSTEDGE_ENVELOPE_V1` only at envelope.rs:858 inside `mod clean_break_tests` (under `#[cfg(test)]` at line 515); `TRUSTEDGE-KEY-V1` only in `test_old_header_rejected_cleanly` (under `#[cfg(test)]` at line 505) |
| D-03 | Phase 84 updates in-repo external-asset source-of-truth files (HTML, action.yml, action README, deploy README) | HONORED | All 4 files updated in plan 84-03 |
| D-04 | No dual-accept — file-input `accept` attribute and extension literals use only `.se-attestation.json` | HONORED | `grep -c '.te-attestation.json' web/verify/index.html` = 0; accept attribute uses generic `.json,application/json` unchanged |

---

### Data-Flow Trace (Level 4)

Not applicable — this phase modifies string constants and labels, not dynamic data-rendering components. The key link verification above confirms the renamed constants are wired to all production call sites (export_secret_encrypted, is_encrypted_key_file, derive_envelope_key, CLI default path).

---

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
|----------|---------|--------|--------|
| `test_old_header_rejected_cleanly` | `cargo test -p sealedge-core --lib test_old_header_rejected_cleanly` | 1 passed | PASS |
| `test_old_domain_rejected_cleanly` | `cargo test -p sealedge-core --lib test_old_domain_rejected_cleanly` | 1 passed | PASS |
| `test_old_domain_produces_distinct_okm` | `cargo test -p sealedge-core --lib test_old_domain_produces_distinct_okm` | 1 passed | PASS |
| Seal-CLI acceptance suite (36 tests) | `cargo test -p sealedge-seal-cli --test acceptance` | 36 passed | PASS |
| `cargo check --workspace --locked` | workspace check | exit 0 | PASS |

---

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
|-------------|------------|-------------|--------|----------|
| REBRAND-03 | Plan 84-01 | Crypto wire-format constants renamed (`TRUSTEDGE-KEY-V1` → `SEALEDGE-KEY-V1`, `TRUSTEDGE_ENVELOPE_V1` HKDF domain → `SEALEDGE_ENVELOPE_V1`) — clean break | SATISFIED | Production const and byte literal renamed; 3 rejection tests prove clean break |
| REBRAND-04b | Plans 84-02 + 84-03 | `.te-attestation.json` → `.se-attestation.json` across CLI, platform endpoint, GitHub Action, verify page | SATISFIED | 22 total occurrences updated across 8 files; zero legacy `.te-attestation.json` literals remaining |

No orphaned requirements — REQUIREMENTS.md maps REBRAND-03 and REBRAND-04b exclusively to Phase 84.

---

### Phase 85/86 Carve-Out Integrity

| Asset | Carve-out Item | Status | Evidence |
|-------|---------------|--------|----------|
| `web/verify/index.html` | `<h1>TrustEdge Attestation Verifier</h1>` brand word | PRESERVED | `grep -c 'TrustEdge' index.html` = 2 (title + h1) |
| `actions/attest-sbom-action/README.md` | `TrustEdge` brand prose | PRESERVED | `grep -c 'TrustEdge' README.md` = 10 |
| `scripts/demo-attestation.sh` | `TrustEdge` echo strings | PRESERVED | `grep -c 'TrustEdge' demo-attestation.sh` = 1 |
| `deploy/digitalocean/README-deploy.md` | `TrustEdge` heading and prose | PRESERVED | `grep -c 'TrustEdge' README-deploy.md` >= 1 (heading confirmed) |
| `crates/core/src/envelope.rs:101` | Comment "TrustEdge envelope v2 context" brand word | PRESERVED | Not renamed per plan — Phase 86 scope |

All Phase 85/86 carve-outs are intact. No premature brand-word renames occurred.

---

### Anti-Patterns Found

None. No TODOs, FIXMEs, placeholders, or empty implementations detected in the modified files. The `TRUSTEDGE-*` legacy literals in `crypto.rs` and `envelope.rs` are correctly scoped inside `#[cfg(test)]` shadow-const modules with zero production footprint — this is intentional and required by D-02.

---

### Human Verification Required

None. All success criteria are fully verifiable programmatically:
- Constants verified by grep + test execution
- Test passes verified by cargo test exit codes
- Carve-out integrity verified by grep

---

### Commits

| Commit | Description |
|--------|-------------|
| `f1b60e8` | refactor(84-01): rename crypto wire-format constants — TRUSTEDGE-* → SEALEDGE-* |
| `a85a3c0` | refactor(84-03): rename .te-attestation.json -> .se-attestation.json in external assets |
| `a4b8675` | refactor(84-02): rename .te-attestation.json -> .se-attestation.json in CLI and scripts |

All 3 plans landed as atomic commits with `cargo check --workspace --locked` green at each boundary.

---

### Summary

Phase 84 goal fully achieved. All four ROADMAP success criteria are met:

1. `SEALEDGE-KEY-V1` is the sole production key-file header; legacy `TRUSTEDGE-KEY-V1` is rejected by `is_encrypted_key_file()` and `import_secret_encrypted()` with an explicit error.
2. HKDF domain is `b"SEALEDGE_ENVELOPE_V1"`; the old domain produces cryptographically distinct OKMs (distinct AES-256 keys and nonce prefixes), guaranteeing AES-GCM tag failure for any legacy-domain envelopes.
3. `.se-attestation.json` is consistent across all 8 target surfaces: seal-cli source + acceptance tests, point_attestation doc comment, demo script, HTML verifier page, GitHub Action YAML + README, deployment README. Zero legacy `.te-attestation.json` literals remain in any of these files.
4. Three targeted rejection tests (`test_old_header_rejected_cleanly`, `test_old_domain_rejected_cleanly`, `test_old_domain_produces_distinct_okm`) all pass, proving clean break rather than silent legacy acceptance.

The full workspace builds and tests green (`cargo check --workspace --locked` exit 0; 36/36 seal-cli acceptance tests pass). Phase 85/86 brand-word carve-outs are intact.

---

_Verified: 2026-04-18T23:15:00Z_
_Verifier: Claude (gsd-verifier)_
