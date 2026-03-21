# Project Retrospective

*A living document updated after each milestone. Lessons feed forward into future planning.*

## Milestone: v2.3 — Security Testing

**Shipped:** 2026-03-21
**Phases:** 4 | **Plans:** 4 | **Sessions:** ~3

### What Was Built
- 31 security tests across 4 threat model categories (T1/T2/T3/T5/T6/T8)
- Archive integrity attack suite: byte mutation, chunk injection, reordering, manifest modification
- Nonce uniqueness and HKDF key-binding verification
- TRUSTEDGE-KEY-V1 format rejection tests (truncation, corruption, wrong passphrase)
- Receipt replay resistance and content binding tests

### What Worked
- Test-only milestone (no production code changes except SEC-02 fix) executed cleanly in 2 days
- SEC-NN naming convention made requirement traceability trivial — every test maps directly to a requirement ID
- assert_cmd black-box CLI testing pattern from Phase 48 was reusable across phases 48-49
- Phase 50 correctly chose library-level API testing over CLI testing for key file format — faster, more precise
- Phase 51 reused existing `create_test_app()` pattern from platform integration tests — zero new infrastructure
- Balanced model profile for executor agents kept costs low for straightforward test-writing work

### What Was Inefficient
- Phase 51 VERIFICATION.md was never created — verifier agent wasn't run after execution, creating a process gap that the milestone audit caught
- Nyquist VALIDATION.md files were created retroactively for all 4 phases — should have been part of execute-phase workflow
- Plan done-criteria off-by-one (Phase 49 said 7 tests, actual was 6; Phase 50 said 14-15, actual was 14) — minor but wastes audit time

### Patterns Established
- SEC-NN_ test function prefix for requirement traceability (adopted across all 4 phases)
- Separate security test files per threat category (`security_archive_integrity.rs`, `security_nonce_key_derivation.rs`, `security_key_file_protection.rs`)
- CLI tests via assert_cmd for user-facing behavior; library tests via direct API calls for internal contracts
- `--unencrypted` flag in test helpers to avoid passphrase prompt blocking

### Key Lessons
1. Test-only milestones are fast (2 days for 4 phases) because there's no production code design debate — the interface is already defined
2. Always run the verifier agent after phase execution — missing VERIFICATION.md creates audit overhead later
3. Plan done-criteria test counts should match the task body's enumerated test list, not be estimated separately

### Cost Observations
- Model mix: ~70% sonnet (executor agents), ~30% opus (orchestration, audit, integration check)
- Sessions: ~3
- Notable: Sonnet-powered executor agents completed each phase plan in 4-35 minutes — appropriate for test-writing tasks

---

## Cross-Milestone Trends

### Process Evolution

| Milestone | Phases | Plans | Key Change |
|-----------|--------|-------|------------|
| v2.0 | 4 | 8 | End-to-end demo with Docker deployment |
| v2.1 | 3 | 6 | Data lifecycle completion (unwrap, YubiKey CLI, profiles) |
| v2.2 | 3 | 5 | Cryptographic remediation (RSA OAEP, PBKDF2, encrypted keys) |
| v2.3 | 4 | 4 | Security test suite (31 tests, 4 threat categories) |

### Cumulative Quality

| Milestone | New Tests | Total Security Tests | Threat Vectors Covered |
|-----------|-----------|---------------------|----------------------|
| v2.2 | 0 (fixes only) | 0 | 0 |
| v2.3 | 31 | 31 | T1, T2, T3, T5, T6, T8 |

### Top Lessons (Verified Across Milestones)

1. Monolith core + thin shells architecture continues to pay off — security tests could call `trustedge_core` APIs directly without wrapper overhead
2. Test infrastructure (assert_cmd, tempfile, create_test_app) established in early milestones compounds — Phase 51 needed zero new infrastructure
3. `--unencrypted` escape hatch (added in v2.2) was essential for CI-compatible test setup across all 4 v2.3 phases
