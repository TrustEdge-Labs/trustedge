---
phase: 85-code-sweep-headers-text-metadata
verified: 2026-04-20T00:00:00Z
status: gaps_found
score: 3/5 must-haves verified
overrides_applied: 0
gaps:
  - truth: "CLI help output, error messages, and log lines visible to a user contain no trustedge strings; environment variable prefixes are SEALEDGE_* (not TRUSTEDGE_*)"
    status: partial
    reason: "Platform-server wiring test test_config_from_env_defaults asserts the old default jwt_audience value 'trustedge-platform' but production code now returns 'sealedge-platform'. Test FAILS (cargo test --workspace --locked). The production rename is correct; the test assertion was not updated in Plan 05."
    artifacts:
      - path: "crates/platform-server/tests/wiring.rs"
        issue: "Line 56 asserts jwt_audience == 'trustedge-platform'; production code.rs:43 returns 'sealedge-platform'"
    missing:
      - "Update wiring.rs:56 assertion to 'sealedge-platform' and update the doc comment on line 44 to match"
  - truth: "A repo-wide grep (outside of archived .planning/milestones/ history) for case-insensitive trustedge returns only intentional references to the TrustEdge-Labs org/brand"
    status: failed
    reason: "Multiple non-org trustedge strings remain in committed source that are not in the documented Plan 05 deferral list: (a) crates/wasm/src/lib.rs lines 71/78/85 have 'TrustEdge WASM' strings compiled into the WASM binary; (b) crates/wasm/js/sealedge.d.ts has public JS class name TrustEdge; (c) crates/wasm/package.json has internal trustedge URLs; (d) crates/seal-protocols/src/archive/manifest.rs:1162 test fixture 'trustedge-agent'; (e) verify_integration.rs temp dir names 'trustedge_test_'; (f) pubky_client.rs /trustedge/ API path literals (experimental). Only the items in the Plan 05 Known Deferrals list are acceptable."
    artifacts:
      - path: "crates/wasm/src/lib.rs"
        issue: "Lines 71/78/85 have 'TrustEdge WASM' user-visible strings compiled into WASM binary — in Phase 85 scope per D-15/D-12 (strings baked into compiled binary)"
      - path: "crates/wasm/js/sealedge.d.ts"
        issue: "Public JS class TrustEdge (line 47), TrustEdgeConfig (line 160), TrustEdgeError (line 175) — user-visible API surface not in deferral list"
      - path: "crates/wasm/package.json"
        issue: "Internal URLs still point to github.com/trustedge-labs/trustedge (lines 35, 38, 40) and keyword 'trustedge' (line 29)"
      - path: "crates/seal-protocols/src/archive/manifest.rs"
        issue: "Line 1162: test fixture sets application = 'trustedge-agent' and line 1173/1382 assert it — test-only fixture brand word"
      - path: "crates/platform/tests/verify_integration.rs"
        issue: "Lines 1552/1616/1641: temp dir names 'trustedge_test_*' / 'trustedge_perm_test_*' / 'trustedge_colocate_test_*'"
      - path: "crates/experimental/pubky-advanced/src/pubky_client.rs"
        issue: "Lines 98/120/172/187: HTTP API path literal '/trustedge/identity' — experimental workspace but still in-repo; SC5 says repo-wide"
    missing:
      - "Update crates/wasm/src/lib.rs greet()/init()/test_basic_functionality() strings to 'Sealedge WASM' per D-11/D-12"
      - "Rename TrustEdge class to Sealedge in crates/wasm/js/sealedge.d.ts (and corresponding sealedge.js entry points if they re-export TrustEdge)"
      - "Update crates/wasm/package.json internal URLs and 'trustedge' keyword"
      - "Update crates/seal-protocols/src/archive/manifest.rs test fixture string 'trustedge-agent' to 'sealedge-agent'"
      - "Update crates/platform/tests/verify_integration.rs temp dir name strings to 'sealedge_test_*'"
      - "Decision needed on /trustedge/identity API path in pubky_client.rs (experimental) — rename to /sealedge/identity or document as intentional protocol path"
---

# Phase 85: Code Sweep — Headers, Text, Metadata Verification Report

**Phase Goal:** Every human-readable string emitted from the codebase or written in its source says "sealedge" — copyright headers, error messages, log lines, CLI help text, env var prefixes, dashboard UI labels, and Cargo.toml metadata URLs all match the new brand.
**Verified:** 2026-04-20
**Status:** gaps_found
**Re-verification:** No — initial verification

---

## Goal Achievement

### Observable Truths (ROADMAP Success Criteria)

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | Every .rs file MPL-2.0 header reads `Project: sealedge` — zero `// Project: trustedge` hits | ✓ VERIFIED | `grep -rn "Project: trustedge" crates/ examples/ --include='*.rs' \| grep -v target/` returns 0 hits. fix-copyright.sh templates updated. 123 .rs files + 8 .sh files swept in Plan 04. |
| 2 | CLI help output, error messages, log lines have no trustedge strings; env vars are SEALEDGE_* | ✗ FAILED | Production env vars, clap attributes, JWT audience default all correctly renamed to SEALEDGE_*/sealedge-platform. However `crates/platform-server/tests/wiring.rs:56` asserts `"trustedge-platform"` against production default `"sealedge-platform"` — test FAILS with `cargo test --workspace --locked`. |
| 3 | Every Cargo.toml `description`, `repository`, `homepage`, `documentation` points at sealedge | ✓ VERIFIED | `cargo metadata --format-version 1 --no-deps` returns zero `TrustEdge-Labs/trustedge` hits in repository field. All 14 Cargo.toml files updated by Plan 03. `package.metadata.trustedge` sections (11 files) are documented Plan 03/05 deferrals — not read by any Rust code. |
| 4 | SvelteKit dashboard UI renders "Sealedge" in all user-facing copy | ✓ VERIFIED | `grep -ri 'trustedge' web/dashboard/src/ web/dashboard/static/` returns 0 hits (excluding TrustEdge-Labs org). `npm run check` = 0 errors. Plan 06 confirmed. |
| 5 | Repo-wide grep for case-insensitive trustedge returns only TrustEdge-Labs org/brand references | ✗ FAILED | 322 hits remaining after applying all documented exclusions. While many are acceptable (rustdoc per D-18, documented Plan 05 deferrals, Phase 84 shadow consts), several are NOT in the deferral list: WASM binary strings, TypeScript class names, package.json metadata, test fixture strings, temp dir names. |

**Score:** 3/5 truths verified

---

## Verification Check Results

### Check 1: SC1 — `Project: trustedge` in line-comment form

**Command:** `grep -rn "Project: trustedge" crates/ examples/ --include='*.rs' | grep -v '///' | grep -v '//!' | grep -v '/target/'`
**Result:** 0 hits
**Status:** ✓ PASSED

Note: 4 hits appear in `crates/experimental/target/debug/build/*/out/` (build-generated files from old cached artifacts) — these are `/target/` paths and correctly excluded. No source `.rs` files have stale headers.

The 6 `/// Project: trustedge` rustdoc hits documented in Plan 04 SUMMARY are correctly deferred to Phase 86 (D-18 boundary).

---

### Check 2: SC2 — TRUSTEDGE_ env vars and clap regressions

**Env vars:** `SEALEDGE_DEVICE_ID` and `SEALEDGE_SALT` correctly renamed at all call sites (sealedge-client.rs:609-610, cli/main.rs:986-987). Fallback strings use `sealedge-*` prefixes.

**JWT audience:** Production code (`platform/src/http/config.rs:43`) correctly returns `"sealedge-platform"`.

**Clap regressions (D-14a):** All 6 `#[command(name = "trustedge-*")]` attributes renamed per Plan 05 (sealedge-client, sealedge-server, attest, verify, platform-server, cli root).

**CRITICAL FAILURE:** `crates/platform-server/tests/wiring.rs:56` asserts `config.jwt_audience == "trustedge-platform"` — this is wrong since production now returns `"sealedge-platform"`. Test fails with:
```
assertion `left == right` failed: default jwt_audience should be 'trustedge-platform'
  left: "sealedge-platform"
 right: "trustedge-platform"
```
`cargo test --workspace --locked` exits non-zero.

---

### Check 3: SC3 — Cargo.toml metadata URLs

**Command output:** `cargo metadata --format-version 1 --no-deps | grep 'trustedge"'` — zero hits in repository field.

Repository URLs: All point to `https://github.com/TrustEdge-Labs/sealedge`. Documentation: `https://docs.rs/sealedge-core`.

**Preserved per D-03/D-06:** `authors = ["TrustEdge Labs"]`, `homepage = "https://trustedgelabs.com"`, copyright line `TRUSTEDGE LABS LLC`.

**Documented deferral:** `[package.metadata.trustedge]` namespace sections in 11 Cargo.toml files — not read by any Rust code, flagged in Plan 03/05 SUMMARY as follow-up.

**Status:** ✓ PASSED

---

### Check 4: SC4 — Dashboard compiled text

**Command:** `grep -ri 'trustedge' web/dashboard/src/ web/dashboard/static/` (excluding TrustEdge-Labs/trustedgelabs.com)
**Result:** 0 hits
**npm run check:** 0 errors, 1 pre-existing unused-CSS warning
**Status:** ✓ PASSED

---

### Check 5: SC5 — Repo-wide case-insensitive grep classification

**Total hits (after standard exclusions, excluding .planning/, target/, .claude/, node_modules/):** 322

**Acceptable references (per D-17, D-18, documented deferrals):**

| Category | Count | Disposition |
|----------|-------|-------------|
| Phase 84 D-02 shadow consts (`OLD_ENVELOPE_DOMAIN`, `OLD_KEY_HEADER`) | ~8 | Intentional — permanently preserved |
| Phase 85 D-02 shadow consts (`OLD_CHUNK_KEY_DOMAIN`, `OLD_SESSION_KEY_DOMAIN`, `OLD_GENESIS_SEED`, `OLD_MANIFEST_DOMAIN_SEP`, `OLD_X25519_DERIVATION`, `OLD_V2_SESSION_KEY`) | ~12 | Intentional — permanently preserved |
| D-02 rejection test assert messages referencing `TRUSTEDGE-KEY-V1` (`crypto.rs:883,896`) | 2 | Intentional — Phase 84 clean-break artifacts |
| `TrustEdgeError` public type + lib.rs re-export | 3 | Documented Plan 05 deferral (semver-breaking rename) |
| `HYBRID_MAGIC = b"TRHY"` comment `// TRustEdge HYbrid` | 1 | Documented Plan 05 deferral (missed by D-01a audit) |
| `[package.metadata.trustedge]` Cargo.toml sections | 11 | Documented Plan 03/05 deferral (dead-code metadata) |
| `crates/wasm/examples/basic-usage.html` | 31 | Documented Plan 05 deferral (Phase 86 ambiguous scope) |
| `"trustedge"` label in `benches/crypto_benchmarks.rs:353` | 1 | Documented Plan 05 deferral (legacy-file detection fixture scenario) |
| `format identifier "te-point-attestation-v1"` | (excluded by grep filter) | Documented Plan 05 deferral |
| `verify.trustedge.dev` URL | (excluded by grep filter) | Documented Plan 05 deferral (Phase 88) |
| Rustdoc `///`/`//!` comments | (excluded by grep filter) | Phase 86 scope per D-18 |

**NOT-acceptable / NOT-in-deferral-list (gaps):**

| File | Lines | Issue | Severity |
|------|-------|-------|----------|
| `crates/platform-server/tests/wiring.rs` | 44, 56-57 | Test asserts `"trustedge-platform"` — test FAILS | Blocker |
| `crates/wasm/src/lib.rs` | 71, 78, 85 | `"TrustEdge WASM"` strings compiled into WASM binary | Blocker |
| `crates/wasm/js/sealedge.d.ts` | 47, 160, 175 | Public JS class names `TrustEdge`, `TrustEdgeConfig`, `TrustEdgeError` | Warning |
| `crates/wasm/package.json` | 29, 35, 38, 40 | Internal npm metadata URLs still reference `trustedge-labs/trustedge` + keyword `"trustedge"` | Warning |
| `crates/wasm/test.html`, `test-crypto.html` | multiple | HTML test harness files (committed source, ~47 hits) | Warning |
| `crates/seal-protocols/src/archive/manifest.rs` | 1162, 1173, 1382 | Test fixture `"trustedge-agent"` (inside `#[cfg(test)]`) | Warning |
| `crates/platform/tests/verify_integration.rs` | 1552, 1616, 1641 | Temp dir names `"trustedge_test_*"` (test-only) | Warning |
| `crates/experimental/pubky-advanced/src/pubky_client.rs` | 98, 120, 172, 187 | HTTP API path literal `"/trustedge/identity"` (experimental workspace) | Info |
| `crates/experimental/pubky/src/lib.rs`, `bin/`, `examples/` | multiple | Various pubky experimental crate hits | Info |

---

### Check 6: Phase 84 Shadow Consts Preserved

| Const | File | Line | Status |
|-------|------|------|--------|
| `OLD_ENVELOPE_DOMAIN = b"TRUSTEDGE_ENVELOPE_V1"` | `crates/core/src/envelope.rs` | 858 | ✓ Preserved (grep count = 6) |
| `OLD_KEY_HEADER = b"TRUSTEDGE-KEY-V1"` | `crates/core/src/crypto.rs` | ~863 | ✓ Preserved (grep count = 2) |

Phase 85 shadow consts also verified:

| Const | Location | Status |
|-------|----------|--------|
| `OLD_CHUNK_KEY_DOMAIN` | `crypto.rs` `mod clean_break_chunk_key_tests` | ✓ Present (4 hits) |
| `OLD_SESSION_KEY_DOMAIN` | `auth.rs` `mod clean_break_session_key_tests` | ✓ Present (3 hits) |
| `OLD_GENESIS_SEED` | `chain.rs` `mod clean_break_genesis_tests` | ✓ Present (4 hits) |
| `OLD_MANIFEST_DOMAIN_SEP` | `domain_separation_test.rs` (top-level const) | ✓ Present (5 hits) |
| `OLD_X25519_DERIVATION` | `pubky-advanced/src/keys.rs` | ✓ Present |
| `OLD_V2_SESSION_KEY` | `pubky-advanced/src/envelope.rs` | ✓ Present |

---

### Check 7: Legal Entity Preservation (D-03)

`Copyright (c) 2025 TRUSTEDGE LABS LLC` preserved in all files — `grep -rln '// Copyright (c) 2025 TRUSTEDGE LABS LLC' crates examples` = 127 files (Plan 04 confirmed).
`authors = ["TrustEdge Labs"]` preserved in Cargo.toml files.
`homepage = "https://trustedgelabs.com"` preserved.

**Status:** ✓ PASSED

---

### Check 8: D-02 Clean-Break Tests (12 total)

**Command:** `cargo test -p sealedge-core --lib clean_break`
**Result:** 8 tests pass (sealedge-core lib)

| Test Name | Location | Status |
|-----------|----------|--------|
| `test_old_chunk_key_domain_produces_distinct_okm` | `crypto.rs::clean_break_chunk_key_tests` | ✓ PASS |
| `test_old_chunk_key_domain_rejected_cleanly` | `crypto.rs::clean_break_chunk_key_tests` | ✓ PASS |
| `test_old_session_key_domain_produces_distinct_okm` | `auth.rs::clean_break_session_key_tests` | ✓ PASS |
| `test_old_session_key_domain_rejected_cleanly` | `auth.rs::clean_break_session_key_tests` | ✓ PASS |
| `test_old_genesis_seed_produces_distinct_hash` | `chain.rs::clean_break_genesis_tests` | ✓ PASS |
| `test_old_genesis_seed_rejected_cleanly` | `chain.rs::clean_break_genesis_tests` | ✓ PASS |
| `test_old_manifest_domain_produces_distinct_signature` | `domain_separation_test.rs` (top-level) | ✓ PASS |
| `test_old_manifest_domain_rejected_cleanly` | `domain_separation_test.rs` (top-level) | ✓ PASS |
| `test_old_x25519_derivation_produces_distinct_key` | `pubky-advanced/keys.rs::clean_break_x25519_tests` | ✓ PASS |
| `test_old_x25519_derivation_rejected_cleanly` | `pubky-advanced/keys.rs::clean_break_x25519_tests` | ✓ PASS |
| `test_old_v2_session_key_produces_distinct_okm` | `pubky-advanced/envelope.rs::clean_break_v2_session_key_tests` | ✓ PASS |
| `test_old_v2_session_key_rejected_cleanly` | `pubky-advanced/envelope.rs::clean_break_v2_session_key_tests` | ✓ PASS |

All 12 D-02 tests pass.

---

### Check 9: Dashboard Build + Typecheck

**npm run check:** 0 errors, 1 pre-existing unused-CSS warning (`+page.svelte:32`)
**Status:** ✓ PASSED (npm run build confirmed in Plan 06 SUMMARY)

---

### Check 10: Full Test Suite

**`cargo test --workspace --locked --lib`:** 279 tests pass (208 + 27 + 30 + 2 + 12 + 0)
**`cd crates/experimental && cargo test --workspace --locked --lib`:** 21 tests pass (7 + 14)

**`cargo test --workspace --locked` (including integration tests):** FAILS

```
FAILED: sealedge-platform-server wiring::test_config_from_env_defaults
  left: "sealedge-platform"
 right: "trustedge-platform"
```

All other integration tests pass. The single failure is the stale assertion in `crates/platform-server/tests/wiring.rs:56`.

---

## Required Artifacts

| Artifact | Status | Details |
|----------|--------|---------|
| `scripts/fix-copyright.sh` | ✓ VERIFIED | Contains `Project: sealedge` in 4 template variants, 0 `Project: trustedge` hits |
| `crates/core/src/crypto.rs` | ✓ VERIFIED | `b"SEALEDGE_SEAL_CHUNK_KEY"` at production site, `OLD_CHUNK_KEY_DOMAIN` in test-only shadow |
| `crates/core/src/auth.rs` | ✓ VERIFIED | `"SEALEDGE_SESSION_KEY_V1"` at production site |
| `crates/core/src/chain.rs` | ✓ VERIFIED | `b"sealedge:genesis"` at production GENESIS_SEED const |
| `crates/core/src/format.rs` | ✓ VERIFIED | `b"SEAL"` MAGIC, `b"sealedge.manifest.v1"` MANIFEST_DOMAIN_SEP |
| `crates/core/tests/domain_separation_test.rs` | ✓ VERIFIED | `OLD_MANIFEST_DOMAIN_SEP` test-only const, 2 D-02 tests, line-128 assertion updated |
| `crates/experimental/pubky-advanced/src/keys.rs` | ✓ VERIFIED | `b"SEALEDGE_X25519_DERIVATION"` at production site |
| `crates/experimental/pubky-advanced/src/envelope.rs` | ✓ VERIFIED | `b"SEALEDGE_V2_SESSION_KEY"` at production site |
| `crates/cli/src/main.rs` | ✓ VERIFIED | `SEALEDGE_DEVICE_ID` env var, `sealedge-abc123` fallback |
| `crates/core/src/bin/sealedge-client.rs` | ✓ VERIFIED | `SEALEDGE_DEVICE_ID`/`SEALEDGE_SALT` env vars, `#[command(name = "sealedge-client")]` |
| `web/dashboard/src/routes/+layout.svelte` | ✓ VERIFIED | Contains `Sealedge` |
| `crates/platform-server/tests/wiring.rs` | ✗ STUB/STALE | Line 56 asserts `"trustedge-platform"` — test fails against production `"sealedge-platform"` |
| `crates/wasm/src/lib.rs` | ✗ PARTIAL | Header `// Project: sealedge` correct; lines 71/78/85 still say `TrustEdge WASM` |

---

## Requirements Coverage

| Requirement | Plans | Description | Status | Evidence |
|-------------|-------|-------------|--------|----------|
| REBRAND-05 | 01, 02, 04 | Copyright/license headers + crypto byte literals renamed | ✓ SATISFIED | `grep -rn "Project: trustedge" crates/ --include='*.rs' \| grep -v '///' \| grep -v target/` = 0. All 5 core crypto constants renamed. 4 experimental constants renamed. |
| REBRAND-06 | 05, 06 | User-facing text, env vars, clap attrs, dashboard UI | ✗ BLOCKED (partial) | Env vars, clap, CLI strings, dashboard correct. One test assertion left stale (`wiring.rs:56`). WASM binary strings and TypeScript class names not updated. |
| REBRAND-07 | 03 | Cargo.toml metadata URLs and descriptions | ✓ SATISFIED | All Cargo.toml repository fields point to TrustEdge-Labs/sealedge. cargo metadata returns zero old-URL hits. |

---

## Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
|------|------|---------|----------|--------|
| `crates/platform-server/tests/wiring.rs` | 56-57 | Stale assertion `"trustedge-platform"` vs production `"sealedge-platform"` | Blocker | `cargo test --workspace --locked` FAILS |
| `crates/wasm/src/lib.rs` | 71, 78, 85 | `"TrustEdge WASM"` strings compiled into WASM binary | Blocker | WASM binary emits old brand name; SC5 criterion fails |
| `crates/wasm/js/sealedge.d.ts` | 47, 160, 175 | `TrustEdge` / `TrustEdgeConfig` / `TrustEdgeError` public JS class names | Warning | User-facing TypeScript API surface retains old brand |
| `crates/wasm/package.json` | 29, 35, 38, 40 | Internal `trustedge` npm keyword and `trustedge-labs/trustedge` bug/homepage URLs | Warning | npm metadata inconsistency |
| `crates/seal-protocols/src/archive/manifest.rs` | 1162, 1173, 1382 | `"trustedge-agent"` in test fixture code | Warning | Test-only; does not affect binary |
| `crates/platform/tests/verify_integration.rs` | 1552, 1616, 1641 | `"trustedge_test_*"` temp dir names in tests | Warning | Test-only; does not affect binary |

---

## Gaps Summary

**Two root causes for 2 failed success criteria:**

**Root cause A — Test not updated (blocker):** Plan 05 updated the production JWT audience default from `"trustedge-platform"` to `"sealedge-platform"` in `platform/src/http/config.rs`, but did not update the corresponding test assertion in `platform-server/tests/wiring.rs:56`. The fix is a one-line change to the test string. This causes `cargo test --workspace --locked` to exit non-zero.

**Root cause B — WASM crate swept incompletely:** The `crates/wasm` (sealedge-wasm) crate was not fully swept in Phase 85. The header `// Project: sealedge` is correct (Plan 04 hit it), but production strings in `src/lib.rs` that emit at WASM runtime (`greet()` alert, `init()` console_log, `test_basic_functionality()` return value) still say `"TrustEdge WASM"`. Additionally, the handwritten TypeScript declaration file `js/sealedge.d.ts` still exports public classes named `TrustEdge`, `TrustEdgeConfig`, and `TrustEdgeError` — these are the user-visible WASM API surface. `package.json` internal metadata (bug URL, homepage, keyword) also retains trustedge strings.

**Secondary gaps (warning-level):** Test fixtures in `seal-protocols/manifest.rs` and `verify_integration.rs` use `"trustedge-*"` as fixture strings. These are test-only (not compiled into binaries) and do not affect ROADMAP SC criterion 1-4, but they appear in SC5 repo-wide grep.

**Documented deferrals NOT counted as gaps:** `TrustEdgeError` type (Plan 05 deferral, semver-breaking), `HYBRID_MAGIC b"TRHY"` comment (Plan 05 deferral, missed by D-01a audit), `[package.metadata.trustedge]` sections (Plan 03/05 deferral, dead-code TOML), `crates/wasm/examples/basic-usage.html` (Plan 05 deferral, Phase 86 scope), `"trustedge"` bench fixture label (Plan 05 deferral), rustdoc `///`/`//!` comments (D-18, Phase 86 scope).

**Smallest fix set to close gaps:**
1. `crates/platform-server/tests/wiring.rs:44,56-57` — update doc comment and assertion to `"sealedge-platform"` (3 lines)
2. `crates/wasm/src/lib.rs:71,78,85` — update `"TrustEdge WASM"` strings to `"Sealedge WASM"` (3 lines)
3. `crates/wasm/js/sealedge.d.ts` — rename `TrustEdge` class, `TrustEdgeConfig`, `TrustEdgeError` to `Sealedge*` (or accept with override if JS class rename is out of Phase 85 scope)
4. `crates/wasm/package.json` — update internal URLs and keyword (4 lines)
5. Test fixture strings in `manifest.rs` and `verify_integration.rs` — optional (test-only, no SC1-4 impact)

---

## Human Verification Required

None — all checks above are programmatic.

---

_Verified: 2026-04-20_
_Verifier: Claude (gsd-verifier)_
