---
phase: "77"
plan: "03"
subsystem: scripts
tags: [demo, sbom, attestation, shell-script]
dependency_graph:
  requires: [76-02]
  provides: [demo-attestation-flow]
  affects: [README, quick-start]
tech_stack:
  added: []
  patterns: [demo-script-pattern, auto-detection, colorized-output]
key_files:
  created:
    - scripts/demo-attestation.sh
  modified: []
decisions:
  - "Script does NOT auto-install syft — errors with install instructions instead (safer for production machines)"
  - "Default remote endpoint set to https://verify.trustedge.dev (overridable via --endpoint)"
  - "Uses demo-attestation-output/ directory (separate from demo-output/ used by demo.sh)"
  - "Remote verification capture uses subshell to avoid pipefail on curl non-zero exit"
metrics:
  duration_seconds: 67
  completed_date: "2026-04-03"
  tasks_completed: 1
  tasks_total: 1
  files_created: 1
  files_modified: 0
---

# Phase 77 Plan 03: SBOM Attestation Demo Script Summary

**One-liner:** End-to-end SBOM attestation demo script (250 lines) following demo.sh patterns — keygen, syft SBOM generation, attest-sbom, local verify, and optional remote verify with auto-detection.

## What Was Built

Created `scripts/demo-attestation.sh`, a copy-pasteable demo script that runs the full SBOM attestation flow without manual steps:

1. **Prerequisites check** — verifies `cargo` and `syft` are installed; exits with install instructions if syft is missing (no auto-install)
2. **Key generation** — `trst keygen --unencrypted` creates demo key pair in `demo-attestation-output/`
3. **Build trst binary** — `cargo build -p trustedge-trst-cli --release` for self-attestation target
4. **SBOM generation** — `syft target/release/trst -o cyclonedx-json` (~10-20s step)
5. **Attestation creation** — `trst attest-sbom` with binary + SBOM + keys
6. **Local verification** — `trst verify-attestation` validates signature and hashes
7. **Remote verification (optional)** — auto-detects `https://verify.trustedge.dev/healthz`; skips gracefully if unreachable or `--local` flag set
8. **Summary** — lists artifacts and prints elapsed time

## Script Characteristics

- **250 lines** (well above 80-line minimum)
- Executable: `chmod +x` applied
- MPL-2.0 copyright header
- `set -uo pipefail` with proper quoting throughout
- Matches demo.sh patterns: same color constants, step_banner/pass/fail/warn helpers, final COMPLETE/FAILED banner
- Timing: `START_TIME=$(date +%s)` at top, elapsed shown in final banner
- `--endpoint <url>` argument for overriding remote URL

## Verification Passed

- `test -x scripts/demo-attestation.sh` — executable
- `grep -q "attest-sbom"` — contains attest-sbom command
- `grep -q "verify-attestation"` — contains verify-attestation command
- `grep -q "syft"` — contains syft usage
- `grep -q "healthz"` — contains remote endpoint auto-detection
- `bash -n scripts/demo-attestation.sh` — syntax check passed

## Deviations from Plan

None — plan executed exactly as written.

## Known Stubs

None — the script is fully wired. Remote endpoint `https://verify.trustedge.dev` will only work once the DigitalOcean deployment (plan 77-02) is live, but the auto-detection logic handles unavailability gracefully.

## Self-Check

- `scripts/demo-attestation.sh` exists and is executable: confirmed
- Commit `ce20017` exists: confirmed

## Self-Check: PASSED
