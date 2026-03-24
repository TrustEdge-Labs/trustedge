# Phase 57: Core Crypto Hardening - Context

**Gathered:** 2026-03-23
**Status:** Ready for planning

<domain>
## Phase Boundary

Add Zeroize/ZeroizeOnDrop to 4 key-holding structs and enforce minimum PBKDF2 iteration count (600k) on encrypted key import. No legacy compatibility required.

</domain>

<decisions>
## Implementation Decisions

### Zeroize approach
- **D-01:** Use `#[derive(Zeroize)]` on all 4 structs and implement `Drop` manually to call `.zeroize()`. Do NOT use `#[derive(ZeroizeOnDrop)]` — it conflicts with `Clone` derive. This matches the existing pattern in envelope.rs.
- **D-02:** `PrivateKey` (asymmetric.rs:30) — add `#[derive(Zeroize)]`, implement `Drop` that calls `self.key_bytes.zeroize()`. Keep existing `Clone, Serialize, Deserialize` derives.
- **D-03:** `SessionInfo` (auth.rs:338) — add `#[derive(Zeroize)]`, implement `Drop` that calls `self.session_key.zeroize()`. Keep existing `Debug, Clone` derives.
- **D-04:** `ClientAuthResult` (auth.rs:327) — add `#[derive(Zeroize)]`, implement `Drop` that calls `self.session_key.zeroize()`. No existing derives to conflict with.
- **D-05:** `SymmetricKey` (hybrid.rs:39) — add `#[derive(Zeroize)]`, implement `Drop` that calls `self.0.zeroize()`. Keep existing `Debug, Clone, PartialEq, Eq` derives.

### PBKDF2 import minimum
- **D-06:** Enforce 600,000 minimum iterations in `import_secret_encrypted()` (crypto.rs). Use the existing `PBKDF2_MIN_ITERATIONS` constant (line 32). Reject with `CryptoError::InvalidKeyFormat` if iterations < 600,000.
- **D-07:** No legacy compatibility — solo builder, no older key files to support. The validation check goes immediately after parsing the `iterations` field from JSON metadata (after line 236).

### Claude's Discretion
- Whether to add a test that constructs a low-iteration key file and verifies rejection
- Whether to add `#[zeroize(drop)]` attribute vs manual `Drop` impl (both achieve the same result)
- Debug impl adjustments if Zeroize derive affects field visibility

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Target structs
- `crates/core/src/asymmetric.rs` — `PrivateKey` struct (line 30), `key_bytes: Vec<u8>` field
- `crates/core/src/auth.rs` — `ClientAuthResult` (line 327) with `session_key: [u8; 32]`, `SessionInfo` (line 338) with `session_key: [u8; 32]`
- `crates/core/src/hybrid.rs` — `SymmetricKey` tuple struct (line 39), `([u8; 32])`

### PBKDF2 import
- `crates/core/src/crypto.rs` — `import_secret_encrypted()` (line 190), `PBKDF2_MIN_ITERATIONS` constant (line 32, value 600,000), iterations parsed at line 233-236 with no validation

### Existing zeroize patterns
- `crates/core/src/secret.rs` — `Secret<T>` wrapper with `#[derive(Zeroize, ZeroizeOnDrop)]` (v1.7)
- `crates/core/src/envelope.rs` — manual `.zeroize()` calls on derived keys (lines 182, 244)

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `zeroize` crate already in workspace deps — no new dependency needed
- `Secret<T>` in secret.rs — not being used here (D-01 chose derive + manual Drop instead)
- `PBKDF2_MIN_ITERATIONS` constant already defined — reuse for import validation

### Established Patterns
- envelope.rs uses manual `.zeroize()` calls (not derive) for local variables
- secret.rs uses `#[derive(Zeroize, ZeroizeOnDrop)]` for the Secret wrapper (no Clone needed there)
- `CryptoError::InvalidKeyFormat(String)` is the existing error variant for key format issues

### Integration Points
- `PrivateKey` is used across crypto.rs, asymmetric.rs — Clone is needed by callers
- `SessionInfo` and `ClientAuthResult` are used in auth.rs — Clone used in session management
- `SymmetricKey` is used in hybrid.rs — Clone + PartialEq used in tests

</code_context>

<specifics>
## Specific Ideas

- The user explicitly stated: 600k minimum, no legacy compatibility, solo builder. This simplifies the import validation to a single comparison.

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope.

</deferred>

---

*Phase: 57-core-crypto-hardening*
*Context gathered: 2026-03-23*
