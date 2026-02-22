# Phase 31: Secret Hardening - Context

**Gathered:** 2026-02-22
**Status:** Ready for planning

<domain>
## Phase Boundary

Ensure sensitive values (PINs, passphrases, JWT secrets, passwords, encryption keys) cannot leak through Debug output, serialization, or memory reuse. Introduce a unified `Secret<T>` wrapper type that handles redaction, zeroization, and access control. Does not add new features or change external APIs.

</domain>

<decisions>
## Implementation Decisions

### Redaction format
- All redacted fields output identical `[REDACTED]` string — no field names, no type hints
- No distinction between secret types (pin, password, jwt all show same `[REDACTED]`)
- Implement via a reusable `Secret<T>` wrapper type, not manual Debug impls per struct

### Secret<T> wrapper design
- `Secret<T>` is a unified type that implements: redacted Debug (`[REDACTED]`), Zeroize + ZeroizeOnDrop, access via `.expose_secret() -> &T`
- No Display impl — using `{}` format on `Secret<T>` is a compile error
- No Deref — must explicitly call `.expose_secret()` to access inner value
- `Secret::new(value)` constructor wraps values immediately

### Sensitivity scope
- Broadly scan all structs, not just the 3 named in requirements — any field that holds a secret, key, or credential gets wrapped
- For key material stored as raw bytes/String in TrustEdge structs: wrap with `Secret<T>`
- Trust upstream crate types (ed25519-dalek SigningKey etc.) to handle their own Debug
- Wrap env var values at read time, not just at struct storage — no transient plain String for secrets

### Config loading after serde removal
- Remove Serialize/Deserialize from YubiKeyConfig and SoftwareHsmConfig
- Replace with builder pattern: `YubiKeyConfig::builder().pin(secret).slot(slot).build()`
- Runtime validation: `build()` returns `Result`, checks required fields at runtime
- LoginRequest uses builder pattern too (consistency across all secret-holding structs)
- LoginRequest keeps Deserialize with custom deserializer (`#[serde(deserialize_with)]`) that wraps password into `Secret<String>` immediately at the HTTP boundary
- LoginRequest does NOT get Serialize (no accidental serialization of passwords)

### Verification approach
- Both compile-time and runtime verification:
  - Compile-time: `Secret<T>` has no derived Debug on inner value; struct-level Debug is manually implemented
  - Runtime: tests format structs with `{:?}` and assert output contains `[REDACTED]`, not actual secret values
- CI grep step: scan for forbidden `derive(Debug)` or `derive(Serialize)` on known secret-holding structs to prevent regression
- Tests verify that `expose_secret()` is the only way to access the inner value

### Claude's Discretion
- Where to place the `Secret<T>` type in the crate hierarchy (likely trustedge-core)
- Exact builder API ergonomics and error types
- Which additional structs beyond the named ones need hardening (based on codebase audit)
- CI grep implementation details (script vs cargo-deny vs custom check)

</decisions>

<specifics>
## Specific Ideas

- Follow the `secrecy` crate's API convention (expose_secret pattern) but implement in-house since zeroize is already in the workspace
- Secret<T> should feel like a zero-cost security wrapper — minimal API surface, maximum protection

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope

</deferred>

---

*Phase: 31-secret-hardening*
*Context gathered: 2026-02-22*
