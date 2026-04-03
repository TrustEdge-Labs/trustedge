<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


---
phase: 40-demo-script
verified: 2026-03-16T00:30:00Z
status: passed
score: 5/5 must-haves verified
re_verification: false
---

# Phase 40: Demo Script Verification Report

**Phase Goal:** Users can see the complete TrustEdge lifecycle in action by running a single script
**Verified:** 2026-03-16T00:30:00Z
**Status:** passed
**Re-verification:** No — initial verification

## Goal Achievement

### Observable Truths (from Success Criteria)

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | User runs `./scripts/demo.sh` and sees the full lifecycle: key generation, data wrapping, submission to verification service, receipt returned | VERIFIED | `scripts/demo.sh` (170 lines, executable): Steps 1-5 cover keygen, sample data, wrap, local verify, summary; Step 6 (server verify + receipt) fires when platform is reachable |
| 2 | Demo works against docker-compose stack and also local cargo builds | VERIFIED | `TRST="cargo run -q -p trustedge-trst-cli --"` used for all local commands; `SERVER_AVAILABLE` auto-detected via `curl -sf http://localhost:3001/healthz`; `--local` and `--docker` flags for explicit override |
| 3 | Each step prints clear output showing what is happening and ends with a visible PASS or FAIL result | VERIFIED | `step_banner()` prints `[Step N/M] Title` in bold blue; `pass()` prints green ✔; `fail()` prints red ✖ and increments FAILURES; final banner is `DEMO COMPLETE — ALL PASSED` or `DEMO FAILED — N step(s) failed` (human-verified by user) |
| 4 | Demo script runs without any manual file preparation — it generates or includes its own sample data | VERIFIED | Step 1 runs `trst keygen` to create `demo-output/device.key` and `demo-output/device.pub`; Step 2 runs `dd if=/dev/urandom` to create `demo-output/sample.bin`; no pre-existing files required |
| 5 | `trst keygen --out-key K --out-pub P` generates valid Ed25519 key pairs compatible with `trst wrap` and `trst verify` | VERIFIED | `handle_keygen()` in `crates/trst-cli/src/main.rs:212-239` calls `DeviceKeypair::generate()` + `export_secret()`; 3 acceptance tests (creates_files, roundtrip, no_overwrite) pass; round-trip test exercises keygen -> wrap -> verify |

**Score:** 5/5 truths verified

### Required Artifacts

| Artifact | Expected | Status | Details |
|----------|----------|--------|---------|
| `scripts/demo.sh` | End-to-end demo script | VERIFIED | 170 lines, executable bit set, MPL-2.0 header present, all lifecycle steps implemented |
| `crates/trst-cli/src/main.rs` | Keygen subcommand implementation | VERIFIED | `KeygenCmd` struct at line 149; `Keygen(KeygenCmd)` in `Commands` enum at line 73; `handle_keygen()` at line 212; `DeviceKeypair::generate()` called at line 227 |
| `crates/trst-cli/tests/acceptance.rs` | Acceptance tests for keygen round-trip | VERIFIED | `acceptance_keygen_creates_files` (line 344), `acceptance_keygen_roundtrip` (line 380), `acceptance_keygen_no_overwrite` (line 430) — all substantive, exercise binary directly |

### Key Link Verification

| From | To | Via | Status | Details |
|------|----|-----|--------|---------|
| `scripts/demo.sh` | `trst keygen` | `cargo run -p trustedge-trst-cli -- keygen` | WIRED | Line 92: `$TRST keygen --out-key ... --out-pub ...`; TRST set to `cargo run -q -p trustedge-trst-cli --` at line 45 |
| `scripts/demo.sh` | `trst wrap` | `cargo run -p trustedge-trst-cli -- wrap` | WIRED | Line 113: `$TRST wrap --profile generic --in ... --out ... --device-key ... --device-pub ...` |
| `scripts/demo.sh` | `trst verify` | `cargo run -p trustedge-trst-cli -- verify` | WIRED | Line 130: `$TRST verify "$DEMO_DIR/sample.trst" --device-pub "$DEVICE_PUB"` |
| `scripts/demo.sh` | `trst emit-request --post` | `cargo run -p trustedge-trst-cli -- emit-request --post` | WIRED | Lines 142-146: `$TRST emit-request --archive ... --device-pub ... --out ... --post http://localhost:3001/v1/verify` (conditional on SERVER_AVAILABLE) |
| `crates/trst-cli/src/main.rs` | `trustedge_core::DeviceKeypair` | `DeviceKeypair::generate() + export_secret()` | WIRED | Line 26 import; line 227 `DeviceKeypair::generate()`; line 230 `device_keypair.export_secret()` |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
|-------------|------------|-------------|--------|----------|
| DEMO-01 | 40-02-PLAN.md | User can run `./scripts/demo.sh` to see the full lifecycle: keygen -> wrap -> verify -> receipt | SATISFIED | `scripts/demo.sh` implements all 5 local steps + optional Step 6 (server receipt) |
| DEMO-02 | 40-02-PLAN.md | Demo script works against both docker-compose stack and local cargo builds | SATISFIED | Auto-detection via curl health check; `--local`/`--docker` flags; `cargo run` used in both modes |
| DEMO-03 | 40-02-PLAN.md | Demo output clearly shows each step and the verification result (PASS/FAIL) | SATISFIED | `step_banner()`, `pass()`, `fail()` functions; final conditional banner; human-verified by user |
| DEMO-04 | 40-01-PLAN.md, 40-02-PLAN.md | Demo script generates or uses sample data (no manual file prep required) | SATISFIED | `trst keygen` generates keys in Step 1; `dd if=/dev/urandom` generates sample data in Step 2; archive wrapped in Step 3 |

No orphaned requirements: REQUIREMENTS.md traceability table maps DEMO-01 through DEMO-04 exclusively to Phase 40. All four are covered by plans 40-01 and 40-02. No additional IDs are mapped to this phase.

### Anti-Patterns Found

| File | Pattern | Severity | Impact |
|------|---------|----------|--------|
| — | None found | — | — |

No TODO/FIXME/PLACEHOLDER markers found in any phase 40 files. No empty implementations or stub returns.

Note: `set -uo pipefail` at line 13 omits `-e` intentionally. The plan explicitly specifies per-command `if/then` error guards so the demo continues past individual step failures and reports a total count. This is correct design for a demo script — not an anti-pattern.

### Human Verification Required

The human checkpoint (Task 2 in Plan 02) was completed by the user prior to this verification. User confirmed:

1. 5 colored step banners with checkmarks displayed correctly
2. Final banner read "DEMO COMPLETE — ALL PASSED"
3. Artifacts present in `demo-output/`

No additional human verification needed — the automated checks fully cover the remaining behaviors.

### Gaps Summary

None. All truths verified, all artifacts substantive and wired, all requirements satisfied, no anti-patterns found.

---

## Supporting Evidence: Commits

| Commit | Description | Verified |
|--------|-------------|---------|
| `b64fd76` | test(40-01): add failing acceptance tests for trst keygen | Present in git log |
| `ed049d5` | feat(40-01): add trst keygen subcommand for Ed25519 key pair generation | Present in git log |
| `83fb275` | feat(40-02): add end-to-end demo script | Present in git log |

---

_Verified: 2026-03-16T00:30:00Z_
_Verifier: Claude (gsd-verifier)_
