<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


# Phase 61: CI Supply Chain Hardening - Context

**Gathered:** 2026-03-24
**Status:** Ready for planning

<domain>
## Phase Boundary

Harden all 4 GitHub Actions workflow files against supply chain attacks. Pin every third-party action to full commit SHAs, remove the `curl | sh` wasm-pack installer, and replace the archived `actions-rs/toolchain` with `dtolnay/rust-toolchain`. CI must pass after all changes.

</domain>

<decisions>
## Implementation Decisions

### wasm-pack installation
- **D-01:** Replace `curl | sh` wasm-pack installer with `cargo-binstall` for pre-built binary download. Install cargo-binstall first (e.g., via `curl` with checksum or `taiki-e/install-action`), then `cargo binstall wasm-pack --no-confirm`.
- **D-02:** Both `wasm-size-check` and `wasm-build-check` jobs in `wasm-tests.yml` use the same approach (currently both have the curl|sh pattern at lines 44 and 93).

### SHA pinning
- **D-03:** Pin all GitHub Actions to full commit SHAs with a version comment on the same line. Format: `uses: actions/checkout@<full-sha> # v4.2.2`
- **D-04:** Pin to the commit SHA that corresponds to the currently-used tag version (don't upgrade versions in this phase — only pin what's already in use).
- **D-05:** Actions to pin across all 4 workflows:
  - `actions/checkout@v4` (ci.yml x3, wasm-tests.yml x2, semver.yml x1)
  - `dtolnay/rust-toolchain@stable` (ci.yml x2, semver.yml x1, + new in wasm-tests.yml x2)
  - `Swatinem/rust-cache@v2` (ci.yml x1, semver.yml x1)
  - `taiki-e/install-action@cargo-audit` (ci.yml x1)
  - `taiki-e/install-action@cargo-semver-checks` (semver.yml x1)
  - `contributor-assistant/github-action@v2.6.1` (cla.yml x1)
  - `cargo-binstall` installer action if used (new in wasm-tests.yml)

### Archived action replacement
- **D-06:** Replace `actions-rs/toolchain@v1` with `dtolnay/rust-toolchain@<sha> # stable` in both wasm-tests.yml jobs (lines 37 and 85). Match the pattern already used in ci.yml and semver.yml.
- **D-07:** Preserve the `target: wasm32-unknown-unknown` configuration when replacing the action. `dtolnay/rust-toolchain` supports `targets:` parameter.

### Claude's Discretion
- Exact commit SHAs to use for each action (look up current tag → SHA mapping at pin time)
- Whether to install cargo-binstall via `taiki-e/install-action` (consistent pattern) or direct curl with checksum
- Order of steps within each job after refactoring

</decisions>

<canonical_refs>
## Canonical References

No external specs — requirements fully captured in decisions above and in the security review findings table provided by the user.

### Workflow files (modify in place)
- `.github/workflows/ci.yml` — Main CI: lint, build-and-test, security audit
- `.github/workflows/wasm-tests.yml` — WASM size check and build verification (primary target: findings 1 and 3)
- `.github/workflows/semver.yml` — Weekly semver compatibility check
- `.github/workflows/cla.yml` — CLA assistant for pull requests

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `taiki-e/install-action` pattern already used in ci.yml and semver.yml for cargo-audit and cargo-semver-checks — extend to cargo-binstall or wasm-pack
- `dtolnay/rust-toolchain` pattern already used in ci.yml and semver.yml — extend to wasm-tests.yml

### Established Patterns
- ci.yml already has the cleanest structure (dtolnay/rust-toolchain, Swatinem/rust-cache) — use as reference for wasm-tests.yml refactoring
- Each workflow has MPL-2.0 copyright header — preserve

### Integration Points
- wasm-tests.yml `wasm-pack build` commands depend on wasm-pack being in PATH — verify cargo-binstall puts it in the right place
- `$HOME/.cargo/bin` PATH addition (wasm-tests.yml line 46) may still be needed with cargo-binstall

</code_context>

<specifics>
## Specific Ideas

No specific requirements — standard supply chain hardening practices.

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope.

</deferred>

---

*Phase: 61-ci-supply-chain-hardening*
*Context gathered: 2026-03-24*
