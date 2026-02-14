---
phase: 21-core-stub-elimination
plan: 01
subsystem: core
tags: [refactor, cleanup, stub-removal]
dependency_graph:
  requires: []
  provides: [clean-hash-algorithm-enum, no-stub-modules]
  affects: [trustedge-core]
tech_stack:
  added: []
  patterns: [fail-closed-design]
key_files:
  created: []
  modified:
    - crates/core/src/lib.rs
    - crates/core/src/protocols/mod.rs
    - crates/core/src/backends/universal.rs
    - crates/core/src/backends/software_hsm.rs
    - crates/core/src/backends/yubikey.rs
  deleted:
    - crates/core/src/envelope_v2_bridge.rs
decisions:
  - Remove incomplete features rather than leaving TODOs
  - Fail-closed error messages with actionable guidance
  - HashAlgorithm enum reduced to implemented variants only
metrics:
  duration: 3 min 59 sec
  tasks_completed: 2
  files_modified: 5
  files_deleted: 1
  lines_deleted: 193
  lines_added: 0
  tests_passing: 219
  completed_at: 2026-02-14T03:41:03Z
---

# Phase 21 Plan 01: Core Stub Elimination Summary

**One-liner:** Deleted envelope_v2_bridge stub module, removed unimplemented Blake2b hash variant, and replaced YubiKey TODO with actionable error message.

## Objective

Eliminate incomplete features from trustedge-core by removing stub modules, unimplemented enum variants, and TODO comments that suggest future implementation of features not on the roadmap.

## Tasks Completed

### Task 1: Delete envelope_v2_bridge.rs and remove all references

**Status:** ✔ Complete
**Commit:** e92e66d
**Duration:** ~2 min

Deleted the envelope_v2_bridge.rs stub module entirely:

- **Deleted:** `crates/core/src/envelope_v2_bridge.rs` (193 lines)
  - 3 TODO comments
  - 2 of 3 envelope formats returning errors
  - Pubky integration stub with no external consumers

- **Modified:** `crates/core/src/lib.rs`
  - Removed `pub mod envelope_v2_bridge;` module declaration
  - Removed re-exports: `detect_envelope_format`, `EnvelopeFormat`, `EnvelopeInfo`, `UnifiedEnvelope`

- **Modified:** `crates/core/src/protocols/mod.rs`
  - Removed envelope_v2_bridge documentation block from Layer 3 protocols section

**Verification:**
- File does not exist: ✔
- Zero references to envelope_v2_bridge in codebase: ✔
- All 146 core tests pass: ✔
- Zero build warnings: ✔

### Task 2: Remove Blake2b stub and clean YubiKey generate_key TODO

**Status:** ✔ Complete
**Commit:** 49a0940
**Duration:** ~2 min

Removed unimplemented Blake2b hash variant:

- **Modified:** `crates/core/src/backends/universal.rs`
  - Removed `Blake2b` variant from `HashAlgorithm` enum
  - Enum now has exactly 3 variants: Sha256, Sha384, Sha512

- **Modified:** `crates/core/src/backends/software_hsm.rs`
  - Removed Blake2b match arm from `hash_data()` function (4 lines)
  - Removed Blake2b unsupported assertion from `test_operation_support` test (6 lines)

Cleaned up YubiKey piv_generate TODO:

- **Modified:** `crates/core/src/backends/yubikey.rs`
  - Replaced 5-line TODO comment block with definitive 2-line comment
  - Changed "will be addressed in a future update" to "Use ykman CLI instead"
  - Improved error message:
    - OLD: "Key generation not yet implemented. Generate keys using 'ykman piv keys generate' and import certificates."
    - NEW: "Key generation is not supported by TrustEdge. Use the YubiKey Manager CLI instead: `ykman piv keys generate -a ECCP256 9a pubkey.pem`"
  - Changed "not yet implemented" (implies future work) to "not supported by TrustEdge" (definitive)
  - Added concrete, copy-pasteable command example

**Verification:**
- Zero Blake2b references in codebase: ✔
- Zero YubiKey generate/policy TODOs: ✔
- Error message contains "not supported by TrustEdge": ✔
- Error message contains "ykman piv keys generate": ✔
- All 219 workspace tests pass: ✔
- Zero build warnings: ✔
- Zero clippy warnings: ✔

## Deviations from Plan

None - plan executed exactly as written.

## Overall Verification

All success criteria met:

1. ✔ envelope_v2_bridge.rs deleted, no references remain anywhere in crates/
2. ✔ HashAlgorithm has exactly 3 variants (Sha256, Sha384, Sha512), no Blake2b anywhere
3. ✔ YubiKey piv_generate error says "not supported" with concrete ykman command, no TODO
4. ✔ All 219 workspace tests pass
5. ✔ Zero build warnings
6. ✔ Zero clippy warnings

## Impact

**Before:**
- envelope_v2_bridge.rs: 193 lines of stub code with 3 TODOs
- HashAlgorithm: 4 variants (1 unimplemented)
- YubiKey error: "not yet implemented" (implies future work)

**After:**
- envelope_v2_bridge.rs: Deleted
- HashAlgorithm: 3 variants (all implemented)
- YubiKey error: "not supported by TrustEdge" with actionable ykman command

**Net change:**
- 193 lines deleted
- 0 lines added
- 6 files modified (3 lib.rs, protocols/mod.rs edits; 3 backend edits)
- 1 file deleted
- 0 tests removed (all 219 still passing)

## Key Decisions

1. **Complete deletion over deprecation:** Deleted envelope_v2_bridge entirely rather than deprecating it, since it had zero external consumers and was a Pubky integration stub.

2. **Fail-closed error design:** YubiKey error now definitively states "not supported" rather than "not yet implemented", following fail-closed design principles.

3. **Actionable error messages:** Provided concrete ykman command users can copy-paste, rather than vague "use ykman" guidance.

## Self-Check: PASSED

**Created files:** None expected, none created.

**Deleted files:**
- ✔ crates/core/src/envelope_v2_bridge.rs: DELETED (verified with `test ! -f`)

**Modified files:**
- ✔ crates/core/src/lib.rs: MODIFIED (verified with git status)
- ✔ crates/core/src/protocols/mod.rs: MODIFIED (verified with git status)
- ✔ crates/core/src/backends/universal.rs: MODIFIED (verified with git status)
- ✔ crates/core/src/backends/software_hsm.rs: MODIFIED (verified with git status)
- ✔ crates/core/src/backends/yubikey.rs: MODIFIED (verified with git status)

**Commits:**
- ✔ e92e66d: EXISTS (Task 1 - delete envelope_v2_bridge)
- ✔ 49a0940: EXISTS (Task 2 - remove Blake2b and clean YubiKey error)

All claimed files and commits verified.
