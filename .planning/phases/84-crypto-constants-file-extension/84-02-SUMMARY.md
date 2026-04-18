---
phase: 84-crypto-constants-file-extension
plan: "02"
subsystem: cli-attestation
tags: [rebrand, attestation-extension, cli-sweep, clean-break]
one_liner: "Internal-facing sweep of `.te-attestation.json` → `.se-attestation.json` in seal-cli help + default path, acceptance test fixtures, point_attestation.rs doc comment, and demo-attestation.sh"
dependency_graph:
  requires: [84-01]
  provides: [REBRAND-04b-internal-surface]
  affects:
    - crates/core/src/point_attestation.rs
    - crates/seal-cli/src/main.rs
    - crates/seal-cli/tests/acceptance.rs
    - scripts/demo-attestation.sh
tech_stack:
  added: []
  patterns: [string-literal-sweep]
key_files:
  created: []
  modified:
    - crates/core/src/point_attestation.rs
    - crates/seal-cli/src/main.rs
    - crates/seal-cli/tests/acceptance.rs
    - scripts/demo-attestation.sh
requirements: [REBRAND-04b]
---

# Plan 84-02 — CLI + Rust tests + demo script attestation-extension sweep

## What was done

12 occurrences of the literal `.te-attestation.json` renamed to `.se-attestation.json` across four internal-facing files:

| File | Sites | What changed |
|------|-------|--------------|
| `crates/core/src/point_attestation.rs` | 1 | `to_json()` doc comment extension reference |
| `crates/seal-cli/src/main.rs` | 3 | `--out` help text, verify-attestation positional arg help, default output path literal at `handle_attest_sbom` |
| `crates/seal-cli/tests/acceptance.rs` | 7 | Fixture paths across attest-sbom creation test, default-output test, valid-inputs test, verify-success test, verify-wrong-key test, verify-with-file-hashes test |
| `scripts/demo-attestation.sh` | 1 | `ATTESTATION_PATH` variable |

## Decisions preserved

- **"attestation." prefix stays** — generic English noun, not a brand (per CONTEXT Claude's Discretion).
- **Brand word "TrustEdge" in echo strings of demo-attestation.sh stays** — Phase 85/86 prose scope.
- **D-04 clean rename** — no dual-accept paths; tests that previously matched `.te-attestation.json` now match `.se-attestation.json` only.

## Self-Check: PASSED

- `cargo check --workspace --locked` green at the commit boundary
- Extension-literal grep on the four modified files: 12 occurrences of `.se-attestation.json` present, 0 occurrences of `.te-attestation.json` remaining
- CLI default-output test now asserts `attestation.se-attestation.json`
- Brand word "TrustEdge" still present in `scripts/demo-attestation.sh` echo strings (Phase 85/86 boundary honored)

## Commits

- `refactor(84-02): rename .te-attestation.json -> .se-attestation.json in CLI and scripts` — 4 files changed, 12 replacements
- `docs(84-02): complete CLI attestation-ext sweep plan summary`

## Notes

Executor agent hit a permission gate mid-run; orchestrator committed the verified file edits and authored this SUMMARY.md directly. File content is identical to what the agent produced — only the `git commit` call was taken over. Combined with Plan 84-03, REBRAND-04b is fully closed.
