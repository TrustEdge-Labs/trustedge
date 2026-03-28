# Phase 72: Core Crypto Hygiene - Context

**Gathered:** 2026-03-27
**Status:** Ready for planning

<domain>
## Phase Boundary

Replace silent error swallowing in two production crypto paths with explicit error handling: `generate_aad()` unwrap → expect, `Envelope::hash()` unwrap_or_default → Result return type.

</domain>

<decisions>
## Implementation Decisions

### generate_aad() (CORE-01)
- **D-01:** Replace `.unwrap()` with `.expect("AAD serialization is infallible")` on `serde_json::to_string()` at crypto.rs:391. This is a one-line change with zero caller impact.

### Envelope::hash() (CORE-02)
- **D-02:** Change `Envelope::hash()` return type from `[u8; 32]` to `Result<[u8; 32]>`. Replace `bincode::serialize(self).unwrap_or_default()` with `bincode::serialize(self)?` and return `Ok(*blake3::hash(&envelope_bytes).as_bytes())`. This affects:
  - 1 production caller: `receipts/mod.rs:240` — must propagate the error via `?`
  - ~15 test callers — add `.unwrap()` in tests (acceptable per project conventions)

### Claude's Discretion
- Error type for Envelope::hash() — whether to use `anyhow::Result` or a specific `BincodeError` wrapper

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Core crypto paths
- `crates/core/src/crypto.rs` — generate_aad() at line 391 with unwrap
- `crates/core/src/envelope.rs` — Envelope::hash() at line 259-262 with unwrap_or_default

### Callers
- `crates/core/src/applications/receipts/mod.rs` — production caller at line 240, test callers throughout

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `anyhow::Result` used throughout core crate for error returns
- Envelope already has methods returning `Result` (e.g., `beneficiary()`, `issuer()`)

### Established Patterns
- Production code: `anyhow::Result` with descriptive error messages
- Tests: `.unwrap()` is acceptable (project convention)
- Error returns in Envelope: `anyhow::anyhow!("descriptive message: {e}")` pattern

### Integration Points
- `Envelope::hash()` is called in receipt chain construction — error must propagate to receipt builder
- `generate_aad()` is a standalone function — no caller changes needed

</code_context>

<specifics>
## Specific Ideas

No specific requirements — standard error handling patterns.

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope

</deferred>

---

*Phase: 72-core-crypto-hygiene*
*Context gathered: 2026-03-27*
