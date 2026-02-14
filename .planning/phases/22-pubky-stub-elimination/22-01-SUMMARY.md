---
phase: 22-pubky-stub-elimination
plan: 01
subsystem: pubky-crates
tags: [cleanup, experimental, code-quality]
dependency_graph:
  requires: []
  provides:
    - "Clean Pubky CLI with only implemented commands"
    - "PubkyClient without placeholder methods"
  affects:
    - trustedge-pubky
    - trustedge-pubky-advanced
tech_stack:
  added: []
  patterns: [fail-closed-design]
key_files:
  created: []
  modified:
    - crates/pubky/src/bin/trustedge-pubky.rs
    - crates/pubky-advanced/src/pubky_client.rs
decisions:
  - "Removed Publish and Migrate CLI commands from trustedge-pubky binary (unimplemented stubs)"
  - "Deleted discover_identities() method from PubkyClient (unused placeholder)"
  - "Documented batch_resolve as sequential-by-design (Pubky protocol limitation)"
  - "Homeserver parameter retained for forward compatibility"
metrics:
  duration: "2 min 47 sec"
  completed: "2026-02-14"
  tasks_completed: 2
  commits: 2
  files_modified: 2
  lines_removed: 114
  tests_passing: 17
---

# Phase 22 Plan 01: Pubky Stub Elimination Summary

**One-liner:** Removed all placeholder code, unimplemented commands, and misleading TODOs from experimental Pubky crates (publish, migrate, discover_identities).

## What Was Done

### Task 1: Remove unimplemented CLI commands from trustedge-pubky (Commit: be4164d)

**Scope:** Eliminated non-functional CLI commands from the trustedge-pubky binary.

**Changes:**
1. Deleted `Publish` variant from Commands enum (lines 92-101)
2. Deleted `Migrate` variant from Commands enum (lines 197-218)
3. Removed `Commands::Publish` match arm in main()
4. Removed `Commands::Migrate` match arm in main()
5. Deleted `publish_key()` function (4 lines - just a TODO bail)
6. Deleted `migrate_envelope()` function (47 lines - placeholder with example workflow)
7. Updated CLI long_about description to remove references to removed features

**Rationale:** Users should never see commands that bail with "not yet implemented". The v1.4 principle: if it doesn't work, it doesn't exist. The Publish command required async Pubky client integration that was never implemented. The Migrate command was a 47-line placeholder that just printed instructions.

**Result:** CLI now shows only 4 working commands: generate, resolve, encrypt, decrypt.

**Verification:**
- `cargo build -p trustedge-pubky` succeeds
- All 7 lib tests pass
- `cargo run -p trustedge-pubky -- --help` shows only implemented commands
- No TODO/placeholder/stub comments remain

### Task 2: Remove placeholders and resolve TODOs in pubky-advanced (Commit: df73954)

**Scope:** Cleaned up PubkyClient implementation in pubky-advanced.

**Changes:**
1. **Deleted discover_identities() method** (13 lines) - Unused placeholder that returned empty Vec with TODO comment. No callers existed.

2. **Resolved batch_resolve_encryption_keys TODO** - Replaced "TODO: Implement actual batch resolution for efficiency" with clear documentation: "Sequential resolution — Pubky protocol resolves one identity at a time. Errors are logged but do not abort resolution of remaining IDs." The existing implementation is correct and intentional.

3. **Fixed with_config homeserver comment** - Replaced placeholder comment with forward compatibility explanation: "Homeserver configuration is accepted but not yet supported by the pubky client SDK. The parameter is retained for forward compatibility."

**Rationale:** TODOs imply future work is needed when the code is actually complete. The batch_resolve method IS the correct implementation - Pubky protocol doesn't provide a batch API, so sequential resolution is the only option. The homeserver parameter is kept for API stability when pubky SDK adds support.

**Result:** Zero TODO or placeholder comments remain in pubky_client.rs.

**Verification:**
- `cargo build -p trustedge-pubky-advanced` succeeds
- All 10 lib tests pass
- No TODO/placeholder comments remain

## Overall Verification

All success criteria met:

- ✓ PUBK-01: publish_key CLI command removed — no Publish variant in Commands enum
- ✓ PUBK-02: discover_identities method removed from PubkyClient
- ✓ PUBK-03: migrate CLI command removed — no Migrate variant in Commands enum
- ✓ PUBK-04: batch_resolve TODO replaced with documented rationale
- ✓ All 17 tests (7 pubky + 10 pubky-advanced) pass
- ✓ Zero placeholder/TODO comments in modified files
- ✓ `cargo clippy -p trustedge-pubky -p trustedge-pubky-advanced -- -D warnings` passes

## Deviations from Plan

None - plan executed exactly as written.

## Impact

**Before:**
- trustedge-pubky CLI had 6 commands (2 non-functional)
- PubkyClient had 1 unused placeholder method
- 3 TODO comments suggesting incomplete work
- 61 lines of misleading stub code

**After:**
- trustedge-pubky CLI has 4 working commands
- PubkyClient has only functional methods
- Zero TODOs in modified files
- 114 lines of dead code eliminated

**User Experience:**
- Users only see commands that actually work
- No confusion from "not yet implemented" errors
- Clear documentation where limitations exist (homeserver, batch resolution)

## Commits

1. **be4164d** - refactor(22-01): remove unimplemented CLI commands from trustedge-pubky
2. **df73954** - refactor(22-01): remove placeholders and resolve TODOs in pubky-advanced

## Self-Check: PASSED

### Files Modified
```bash
# Verify modified files exist
✓ FOUND: crates/pubky/src/bin/trustedge-pubky.rs
✓ FOUND: crates/pubky-advanced/src/pubky_client.rs
```

### Commits Exist
```bash
# Verify commits in git log
✓ FOUND: be4164d (Task 1)
✓ FOUND: df73954 (Task 2)
```

### Tests Passing
```bash
# Verify test counts
✓ trustedge-pubky: 7 tests passed
✓ trustedge-pubky-advanced: 10 tests passed
✓ Total: 17 tests passed (matches plan expectation)
```

### Code Quality
```bash
# Verify no placeholders remain
✓ grep -i "TODO" crates/pubky/src/bin/trustedge-pubky.rs → No matches
✓ grep -i "placeholder" crates/pubky-advanced/src/pubky_client.rs → No matches
✓ cargo clippy passes with -D warnings
```

All verification checks passed.
