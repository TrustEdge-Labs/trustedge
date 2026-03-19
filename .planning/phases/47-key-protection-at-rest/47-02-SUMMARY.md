---
phase: 47-key-protection-at-rest
plan: 02
subsystem: cli
tags: [trst-cli, passphrase, encrypted-keys, --unencrypted, acceptance-tests]

# Dependency graph
requires:
  - "47-01: DeviceKeypair::export_secret_encrypted / import_secret_encrypted / is_encrypted_key_file"
provides:
  - "--unencrypted flag on trst keygen, wrap, unwrap"
  - "Passphrase prompting on keygen (write) and wrap/unwrap (read)"
  - "Rejection of plaintext keys without --unencrypted (error: 'Key file is not encrypted. Use --unencrypted to bypass.')"
  - "Acceptance tests and integration tests pass non-interactively via --unencrypted"
  - "Demo script runs non-interactively via --unencrypted"
affects: [47-03]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "--unencrypted flag as automation/CI escape hatch from passphrase prompting"
    - "is_encrypted_key_file() pre-flight check before key format dispatch"
    - "rpassword::prompt_password() for passphrase entry without echo (already a dependency)"

key-files:
  created: []
  modified:
    - crates/trst-cli/src/main.rs
    - crates/trst-cli/tests/acceptance.rs
    - crates/trst-cli/tests/integration_tests.rs
    - scripts/demo.sh

key-decisions:
  - "integration_tests.rs also required --unencrypted on all wrap commands (Rule 2 auto-fix — pre-existing tests broken by new security enforcement)"
  - "acceptance_keygen_no_overwrite does not need --unencrypted: overwrite check fires before any key operation or passphrase prompt"
  - "YubiKey wrap in demo.sh uses plaintext device.key for chunk encryption even with hardware signing backend — needs --unencrypted"

patterns-established:
  - "--unencrypted as the canonical escape hatch for non-interactive use of key-encrypted commands"

requirements-completed: [KEY-02, KEY-03]

# Metrics
duration: 18min
completed: 2026-03-19
---

# Phase 47 Plan 02: CLI Integration for Encrypted Key Files Summary

**--unencrypted flag on trst keygen/wrap/unwrap with passphrase prompting via rpassword; plaintext key rejection by default; all 28 acceptance tests and 16 integration tests pass**

## Performance

- **Duration:** 18 min
- **Started:** 2026-03-19T02:10:00Z
- **Completed:** 2026-03-19T02:28:00Z
- **Tasks:** 2
- **Files modified:** 4

## Accomplishments

- `trst keygen`: prompts for passphrase + confirmation, writes TRUSTEDGE-KEY-V1 encrypted file; `--unencrypted` bypasses to write plaintext `ed25519:...` format
- `trst wrap` (load_or_generate_keypair): detects key file format via `is_encrypted_key_file()`:
  - Encrypted key file: prompts for passphrase to decrypt (ignores `--unencrypted`, rejects it if flagged)
  - Plaintext key file: requires `--unencrypted` or exits with "Key file is not encrypted. Use --unencrypted to bypass."
  - No key provided (auto-generate): with `--unencrypted` writes plaintext; otherwise prompts for passphrase + confirm and writes encrypted
- `trst unwrap`: same logic as wrap for key loading; `--unencrypted` flag added
- All 28 acceptance tests pass with `--unencrypted` flag in test commands
- All 16 integration tests updated with `--unencrypted` and now pass
- Demo script (`scripts/demo.sh`) runs non-interactively with `--unencrypted` on keygen, wrap, and YubiKey wrap steps

## Task Commits

Each task was committed atomically:

1. **Task 1: --unencrypted flag and passphrase prompts to CLI commands** - `0ff5d68` (feat)
2. **Task 2: Update acceptance tests, integration tests, and demo script** - `16d6cab` (feat)

## Files Created/Modified

- `crates/trst-cli/src/main.rs` - Added `--unencrypted` to KeygenCmd/WrapCmd/UnwrapCmd; updated handle_keygen, load_or_generate_keypair (new `unencrypted: bool` param), handle_unwrap; added is_encrypted_key_file to import list
- `crates/trst-cli/tests/acceptance.rs` - Added `--unencrypted` to all wrap/keygen/unwrap invocations (28 tests, all pass)
- `crates/trst-cli/tests/integration_tests.rs` - Added `--unencrypted` to all wrap command invocations (16 tests, all pass)
- `scripts/demo.sh` - Added `--unencrypted` to keygen, generic wrap, and yubikey wrap steps

## Decisions Made

- Integration tests (`integration_tests.rs`) also needed `--unencrypted` — this was a pre-existing test file not mentioned in the plan's file list; treated as Rule 2 auto-fix (missing security-correct flag to avoid non-interactive hang)
- `acceptance_keygen_no_overwrite` does not need `--unencrypted` because the file-exists check fires before any key operation; test correctly still asserts failure with "overwrite"
- YubiKey wrap command in demo.sh uses plaintext `device.key` for chunk encryption (even though signing is via hardware); requires `--unencrypted` because the key file is plaintext

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - Missing Critical] Updated integration_tests.rs (not in plan file list)**
- **Found during:** Task 2
- **Issue:** `crates/trst-cli/tests/integration_tests.rs` is a second test file (16 tests) that also calls `trst wrap`. Without `--unencrypted`, all 14 wrap-dependent tests fail with "Failed to read passphrase" in non-interactive mode.
- **Fix:** Added `--unencrypted` to all wrap command invocations in integration_tests.rs
- **Files modified:** crates/trst-cli/tests/integration_tests.rs
- **Commit:** 16d6cab

---

**Total deviations:** 1 auto-fixed (Rule 2 — second test file not listed in plan)
**Impact on plan:** No scope creep. Workspace test suite now fully passes.

## Self-Check

---
## Self-Check: PASSED

Files verified to exist:
- crates/trst-cli/src/main.rs: FOUND
- crates/trst-cli/tests/acceptance.rs: FOUND
- crates/trst-cli/tests/integration_tests.rs: FOUND
- scripts/demo.sh: FOUND

Commits verified:
- 0ff5d68: FOUND
- 16d6cab: FOUND

---
*Phase: 47-key-protection-at-rest*
*Completed: 2026-03-19*
