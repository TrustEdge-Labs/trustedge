# Phase 26: Crypto Deduplication - Context

**Gathered:** 2026-02-22
**Status:** Ready for planning

<domain>
## Phase Boundary

Replace direct calls to blake3 and ed25519-dalek in trustedge-platform's verify module with calls through trustedge_core::chain and trustedge_core::crypto. Delete the direct crypto crate dependencies from trustedge-platform. All verification tests continue to pass. This is import consolidation, not a crypto rewrite -- the underlying libraries are the same proven crates, just accessed through core's API instead of directly.

</domain>

<decisions>
## Implementation Decisions

### Replacement scope
- Researcher decides full scope: verify engine is the primary target, but researcher should check whether signing.rs (JWS), jwks.rs (key generation), or CA module have direct crypto imports that should also go through core
- Adapt engine to core's API (core is the source of truth), not the other way around
- Remove blake3 and ed25519-dalek from trustedge-platform's Cargo.toml after replacement (if nothing else imports them directly)
- Public API of the verify module can change if using core's types makes it cleaner -- tests will be adapted

### CA module cleanup
- Leave `// Phase 26:` placeholder markers as-is (do NOT implement the placeholder auth/database functions)
- Rename all `// Phase 26:` markers to `// Future:` since this phase won't address them -- avoids confusion
- Researcher should check whether the CA module has any direct crypto imports beyond UniversalBackend that need consolidation

### Test strategy
- Keep existing unit tests unchanged where possible (same test cases, same assertions, different internals)
- Tests that directly call blake3::hash() or ed25519_dalek functions (removed imports) get rewritten to call through trustedge_core API
- Full test suite as final validation: unit + integration + workspace -- maximum confidence nothing broke

### Claude's Discretion
- Exact mapping of verify-core functions to trustedge_core equivalents
- Whether signing.rs and jwks.rs crypto should go through core (researcher decides based on API fit)
- How to handle any API mismatches between engine's current interface and core's API

</decisions>

<specifics>
## Specific Ideas

- This is import consolidation, not a crypto rewrite -- the same proven libraries (blake3, ed25519-dalek) are used, just accessed through core's unified API
- The principle is "trustedge-core owns all cryptographic operations" -- platform should not maintain its own crypto dependency tree
- After this phase, trustedge-platform's only crypto dependency should be trustedge-core itself

</specifics>

<deferred>
## Deferred Ideas

None -- discussion stayed within phase scope

</deferred>

---

*Phase: 26-crypto-deduplication*
*Context gathered: 2026-02-22*
