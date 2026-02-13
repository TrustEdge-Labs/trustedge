---
phase: 17-security-hardening
plan: 01
subsystem: security
tags: [cargo-audit, vulnerability-scanning, dependency-security, ci-hardening]

# Dependency graph
requires:
  - phase: 16-dependency-audit
    provides: cargo-machete cleanup and dependency baseline
provides:
  - Automated vulnerability scanning with cargo-audit
  - Risk acceptance documentation for unfixable advisories
  - CI blocking on new vulnerabilities
affects: [all future phases - security scanning enforced]

# Tech tracking
tech-stack:
  added: [cargo-audit 0.22.1]
  patterns:
    - ".cargo/audit.toml for documented risk acceptance"
    - "Graceful degradation for optional CI tools in ci-check.sh"

key-files:
  created:
    - .cargo/audit.toml
    - Cargo.lock
  modified:
    - Cargo.toml
    - .github/workflows/ci.yml
    - scripts/ci-check.sh

key-decisions:
  - "Accepted RSA Marvin Attack advisory (RUSTSEC-2023-0071) with documented rationale"
  - "TrustEdge does not use RSA for production encryption (Ed25519 + AES-256-GCM only)"
  - "Cargo.lock now tracked in git to pin dependency versions for audit consistency"

patterns-established:
  - "All security advisories require documented risk acceptance in .cargo/audit.toml"
  - "CI fails fast on security issues (audit runs early, before copyright checks)"
  - "Local ci-check.sh mirrors CI behavior with graceful tool skipping"

# Metrics
duration: 6min
completed: 2026-02-13
---

# Phase 17 Plan 01: Security Hardening Summary

**cargo-audit integrated as blocking CI check with RSA advisory risk acceptance documented**

## Performance

- **Duration:** 6 minutes
- **Started:** 2026-02-13T03:15:41Z
- **Completed:** 2026-02-13T03:21:42Z
- **Tasks:** 2
- **Files modified:** 5

## Accomplishments

- Fixed 2 critical/medium vulnerabilities via dependency updates (bytes, time)
- Documented risk acceptance for 2 unfixable RSA advisories (no patched versions exist)
- Integrated cargo-audit as blocking check in CI pipeline and local ci-check.sh
- Cargo.lock now tracked in git for reproducible security scans

## Task Commits

Each task was committed atomically:

1. **Task 1: Install cargo-audit, run audit, and resolve all advisories** - `91bd5d7` (feat)
2. **Task 2: Add cargo-audit to CI pipeline and local ci-check.sh** - `5806d89` (feat)

## Files Created/Modified

- `.cargo/audit.toml` - cargo-audit configuration with documented risk acceptance for RUSTSEC-2023-0071
- `Cargo.lock` - Now tracked in git (previously ignored) for reproducible security audits
- `Cargo.toml` - Updated RSA version pin to 0.9.10 for workspace consistency
- `.github/workflows/ci.yml` - Added cargo-audit installation and blocking security audit step
- `scripts/ci-check.sh` - Added Step 2 security audit with graceful skip, renumbered all steps

## Decisions Made

**RSA Marvin Attack Advisory Acceptance (RUSTSEC-2023-0071):**
- **Affected:** rsa 0.7.2 (via yubikey 0.7.0), rsa 0.9.10 (direct dependency)
- **Severity:** 5.9 MEDIUM - timing sidechannel attack on RSA decryption
- **Why unfixable:** Advisory states "No fixed upgrade is available!" across all RSA versions
- **Risk acceptance rationale:**
  - TrustEdge does NOT use RSA for production encryption
  - Core security model: Ed25519 signatures + AES-256-GCM encryption
  - RSA only used in:
    1. YubiKey PIV operations (feature-gated, hardware-backed, experimental)
    2. Pubky hybrid encryption (tier-2 experimental crate, not production)
  - Marvin Attack requires observing RSA decryption timing patterns, not applicable to our use cases
- **Mitigation:** Monitor upstream yubikey and rsa crates for future fixes

**Dependency Updates:**
- bytes 1.11.0 → 1.11.1 (fixed RUSTSEC-2026-0007: integer overflow in BytesMut::reserve)
- time 0.3.46 → 0.3.47 (fixed RUSTSEC-2026-0009: DoS via stack exhaustion)

**Cargo.lock tracking:**
- Previously ignored, now tracked in git
- Ensures reproducible security audits across development environments
- CI will detect dependency changes that introduce vulnerabilities

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None. All advisories were either fixed via dependency updates or documented with risk acceptance as planned.

## Verification

All verification criteria met:
- ✔ cargo audit exits 0 (clean with documented exceptions)
- ✔ cargo build --workspace --no-default-features succeeds
- ✔ cargo test --workspace --lib --no-default-features passes (171 tests)
- ✔ cargo clippy --workspace --no-default-features passes
- ✔ .github/workflows/ci.yml has non-continue-on-error cargo-audit step
- ✔ scripts/ci-check.sh has cargo-audit step with graceful skip
- ✔ bash -n scripts/ci-check.sh validates syntax
- ✔ Step numbers in ci-check.sh are sequential (1-19)

## Next Phase Readiness

Security audit infrastructure complete. All future PRs will be blocked if they introduce dependencies with known vulnerabilities. Developers can run `cargo audit` locally or via `./scripts/ci-check.sh` to catch issues before CI.

## Self-Check: PASSED

All claimed files and commits verified:
- ✔ .cargo/audit.toml exists
- ✔ Cargo.lock exists
- ✔ commit 91bd5d7 exists
- ✔ commit 5806d89 exists

---
*Phase: 17-security-hardening*
*Completed: 2026-02-13*
