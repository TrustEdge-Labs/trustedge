# Phase 61: CI Supply Chain Hardening - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-03-24
**Phase:** 61-ci-supply-chain-hardening
**Areas discussed:** wasm-pack install method, SHA pinning approach

---

## wasm-pack Install Method

| Option | Description | Selected |
|--------|-------------|----------|
| cargo-binstall (Recommended) | Fast pre-built binary download via cargo-binstall. Used widely in Rust CI. ~5s vs 2min cargo install. | ✓ |
| taiki-e/install-action | Same action already used for cargo-audit and cargo-semver-checks. Consistent pattern, SHA-pinnable. | |
| cargo install wasm-pack | Build from source. Slow (~2min) but zero external trust. Most paranoid option. | |

**User's choice:** cargo-binstall (Recommended)
**Notes:** Fast binary download preferred over slow source build. Acceptable trust trade-off for CI speed.

---

## SHA Pinning Approach

| Option | Description | Selected |
|--------|-------------|----------|
| SHA + version comment (Recommended) | e.g., actions/checkout@a5ac7e5... # v4.2.2 — human-readable, easy to audit | ✓ |
| Bare SHA only | No comments — maximally concise, harder to tell what version | |
| SHA + Dependabot config | Pin with comments AND add .github/dependabot.yml for auto SHA updates | |

**User's choice:** SHA + version comment (Recommended)
**Notes:** Human-readable version comments for auditability. No Dependabot config for now.

---

## Claude's Discretion

- Exact commit SHAs for each action (look up at implementation time)
- Whether to use taiki-e/install-action for cargo-binstall or direct installation
- Step ordering within refactored jobs

## Deferred Ideas

None — discussion stayed within phase scope.
