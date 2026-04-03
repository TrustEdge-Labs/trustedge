<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


# Phase 68: Insecure Defaults - Context

**Gathered:** 2026-03-26
**Status:** Ready for planning

<domain>
## Phase Boundary

Guard dangerous default configurations in CAConfig and SoftwareHsmConfig so that placeholder/demo credentials cannot silently reach production. Both `impl Default` blocks must be removed; construction must go through builders with explicit secrets.

</domain>

<decisions>
## Implementation Decisions

### CAConfig Guard
- **D-01:** Remove `impl Default for CAConfig` entirely. The Default trait is public and bypasses the builder's placeholder guard (`CAConfigBuilder::build()` line 154 already panics on `"your-secret-key"` outside tests, but `CAConfig::default()` sidesteps the builder).
- **D-02:** Move the non-secret field defaults (database_url, ca_name, ca_organization, ca_country, certificate_validity_days) into `CAConfigBuilder::default()` directly — the builder no longer delegates to `CAConfig::default()`.

### SoftwareHsmConfig Guard
- **D-03:** Remove `impl Default for SoftwareHsmConfig` entirely. The demo passphrase `"changeme123!"` must not be reachable without explicit construction.
- **D-04:** Remove or refactor `SoftwareHsmBackend::new()` — it currently calls `SoftwareHsmConfig::default()`. Either delete `new()` and always require `with_config()`, or make `new()` take an explicit config parameter.
- **D-05:** Update `universal_registry.rs:53` to construct a `SoftwareHsmConfig` via the builder with an explicit passphrase instead of relying on `SoftwareHsmBackend::new()`.

### Test Ergonomics
- **D-06:** Add `#[cfg(test)] pub fn test_default()` methods on both `CAConfig` and `SoftwareHsmConfig` that return configs with explicit test-only secrets (e.g., `"test-jwt-secret-do-not-use"`, `"test-passphrase-do-not-use"`). This preserves test ergonomics without exposing insecure defaults to production.

### Claude's Discretion
- How to handle `SoftwareHsmBackend::new()` — whether to delete it entirely (breaking change within workspace) or convert it to require a config parameter. Either approach is acceptable as long as the default passphrase path is eliminated.
- Whether `CAConfigBuilder::build()` placeholder guard (line 154) can be simplified after Default removal, since the builder will have its own inline defaults.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Platform CA Module
- `crates/platform/src/ca/mod.rs` — CAConfig struct, Default impl (lines 85-96), builder (lines 98-169), tests (line 171+)

### Core Software HSM
- `crates/core/src/backends/software_hsm.rs` — SoftwareHsmConfig struct (lines 59-91), SoftwareHsmBackend::new() (lines 184-189), builder (lines 108+), tests (line 670+)
- `crates/core/src/backends/universal_registry.rs` — Auto-discovery calls SoftwareHsmBackend::new() at line 53

### Existing Pattern
- `crates/platform/src/ca/mod.rs:153-159` — Existing `cfg!(test)` guard pattern in `CAConfigBuilder::build()` for reference

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `Secret<T>` wrapper (trustedge-core) — already used by both configs for passphrase/JWT secret wrapping
- `cfg!(test)` guard pattern — established in `CAConfigBuilder::build()` line 154, reuse for test_default() gating

### Established Patterns
- Builder pattern with `Secret<T>` wrapping — both configs already have builders (`CAConfigBuilder`, `SoftwareHsmConfigBuilder`)
- Test configs use explicit builder construction with test-specific secrets (see `software_hsm.rs` test module, `ca/mod.rs` test module)

### Integration Points
- `universal_registry.rs:53` — must be updated to pass explicit config to SoftwareHsmBackend
- `software-hsm-demo.rs:119` — demo binary already uses builder (no change needed)
- `software_hsm_integration.rs:31` — integration tests already use builder (no change needed)

</code_context>

<specifics>
## Specific Ideas

No specific requirements — standard security hardening following established builder patterns.

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope.

</deferred>

---

*Phase: 68-insecure-defaults*
*Context gathered: 2026-03-26*
