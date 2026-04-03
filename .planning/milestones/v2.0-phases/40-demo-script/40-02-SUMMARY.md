<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


---
phase: 40-demo-script
plan: "02"
subsystem: demo
tags: [bash, demo, trst, keygen, wrap, verify, lifecycle]

# Dependency graph
requires:
  - phase: 40-01
    provides: trst keygen subcommand (Ed25519 key pair generation)
  - phase: 38-archive-profiles
    provides: trst wrap/verify with generic profile support
provides:
  - End-to-end demo script demonstrating full TrustEdge lifecycle
  - scripts/demo.sh with auto-detection of docker vs local mode
  - Human-verified output: colored banners, checkmarks, DEMO COMPLETE banner
affects: []

# Tech tracking
tech-stack:
  added: []
  patterns:
    - Bash demo script with ANSI colors, step banners, pass/fail tracking
    - Auto-detection of platform server (curl health check) for docker vs local mode
    - set -euo pipefail with per-command error trapping for demo resilience

key-files:
  created:
    - scripts/demo.sh
  modified: []

key-decisions:
  - "Demo uses cargo run directly (not installed binary) so it works from repo without installation"
  - "TOTAL_STEPS computed dynamically (5 local, 6 with server) before rendering any banner"
  - "set -euo pipefail at top but per-step if/then guards let demo continue past individual failures and report count"

patterns-established:
  - "Demo script pattern: step_banner/pass/fail functions + FAILURES counter + final conditional exit"

requirements-completed: [DEMO-01, DEMO-02, DEMO-03, DEMO-04]

# Metrics
duration: 10min
completed: 2026-03-16
---

# Phase 40 Plan 02: Demo Script Summary

**Bash demo script (scripts/demo.sh) that runs the full TrustEdge lifecycle (keygen, sample data, wrap, local verify, receipt emit) with ANSI-colored step banners and human-verified DEMO COMPLETE output**

## Performance

- **Duration:** ~10 min (continuation from checkpoint)
- **Started:** 2026-03-16T00:06:04Z
- **Completed:** 2026-03-16T00:06:04Z
- **Tasks:** 2 (Task 1: implementation, Task 2: human-verify checkpoint)
- **Files modified:** 1

## Accomplishments
- Created scripts/demo.sh with 5-step local lifecycle (keygen, sample data, wrap, local verify, summary)
- Auto-detection of platform server via curl health check; graceful skip with instructions when not running
- --local and --docker flags for explicit mode forcing
- ANSI colors: blue step banners, green checkmarks, red failures, bold final DEMO COMPLETE banner
- User verified output: all 5 steps with colored banners and checkmarks, "DEMO COMPLETE -- ALL PASSED", artifacts in demo-output/

## Task Commits

Each task was committed atomically:

1. **Task 1: Create demo.sh script** - `83fb275` (feat)
2. **Task 2: Verify demo script output and flow** - checkpoint approved by user (no code commit)

**Plan metadata:** (docs commit below)

## Files Created/Modified
- `scripts/demo.sh` - End-to-end demo script: keygen, wrap, verify, optional server emit; 5 or 6 steps depending on server availability

## Decisions Made
- Demo uses `cargo run -p trustedge-trst-cli --` directly (not installed binary) so it works from repo root without installation
- TOTAL_STEPS computed dynamically (5 local, 6 with server) before rendering any banner, so step counters are always accurate
- `set -euo pipefail` at script top but per-step `if/then` guards let the demo continue past individual failures and report a count at the end

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
None.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Demo script is complete and human-verified
- v2.0 milestone is fully delivered: keygen, wrap, verify, receipt, deployment stack, demo script all working
- Ready for milestone tagging (git tag v2.0)

---
*Phase: 40-demo-script*
*Completed: 2026-03-16*
