# Phase 68: Insecure Defaults - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-03-26
**Phase:** 68-insecure-defaults
**Areas discussed:** CAConfig guard strategy, SoftwareHsm guard strategy, Test ergonomics

---

## CAConfig Guard Strategy

| Option | Description | Selected |
|--------|-------------|----------|
| Remove Default impl | Delete impl Default for CAConfig entirely. Move field defaults into CAConfigBuilder::default() directly. External code must use the builder. | ✓ |
| #[cfg(test)] on Default impl | Keep Default but gate it behind #[cfg(test)]. Tests get ergonomic defaults, production code must use builder. | |
| Panic in jwt_secret() accessor | Keep Default but add runtime check in jwt_secret() that panics if value is placeholder. Catches use-time, not construction-time. | |

**User's choice:** Remove Default impl (Recommended)
**Notes:** Cleanest approach — eliminates the bypass entirely rather than adding more runtime guards.

---

## SoftwareHsm Guard Strategy

| Option | Description | Selected |
|--------|-------------|----------|
| Remove Default + fix new() | Delete impl Default for SoftwareHsmConfig. Make new() take a config parameter. Universal registry passes builder-constructed config. | ✓ |
| #[cfg(test)] on Default impl | Gate Default behind #[cfg(test)]. SoftwareHsmBackend::new() becomes test-only or takes config in production. | |
| Panic on demo passphrase outside test | Keep Default but add cfg!(test) guard in default_passphrase() accessor. | |

**User's choice:** Remove Default + fix new() (Recommended)
**Notes:** Consistent with CAConfig approach — both configs lose their Default impls.

---

## Test Ergonomics

| Option | Description | Selected |
|--------|-------------|----------|
| test_default() methods | Add #[cfg(test)] pub fn test_default() on both configs with explicit test-only secrets. | ✓ |
| Always use builder in tests | Tests always use Config::builder().passphrase(...).build(). More verbose. | |
| You decide | Claude picks whichever approach is cleanest. | |

**User's choice:** test_default() methods (Recommended)
**Notes:** Clean and discoverable — test code can call Config::test_default() instead of verbose builder chains.

## Claude's Discretion

- How to handle SoftwareHsmBackend::new() refactoring (delete vs convert signature)
- Whether to simplify CAConfigBuilder::build() placeholder guard after Default removal

## Deferred Ideas

None.
