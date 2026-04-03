<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


# Phase 56: WASM Fix - Context

**Gathered:** 2026-03-23
**Status:** Ready for planning

<domain>
## Phase Boundary

Fix the double `.decrypt()` call in `trst-wasm/src/crypto.rs` that breaks browser-based archive verification. Single-line bug fix plus verification test.

</domain>

<decisions>
## Implementation Decisions

### Bug fix
- **D-01:** Remove the duplicate `.decrypt()` call at `crates/trst-wasm/src/crypto.rs:187`. Keep line 186 (`.decrypt(nonce_array.into(), ciphertext_bytes.as_slice())`) which uses the correctly-typed nonce. Delete line 187 (`.decrypt(nonce, ciphertext_bytes.as_slice())`) which is the duplicate.

### Test approach
- **D-02:** Add a unit test in `crypto.rs` that performs encrypt → decrypt round-trip and verifies the recovered plaintext matches the original input. This proves the decrypt path works end-to-end.
- **D-03:** Verify that `wasm-pack build` succeeds for the trst-wasm crate after the fix.

### Claude's Discretion
- Whether to also add a test for decrypt failure (wrong key, corrupted ciphertext)
- Test naming convention

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### WASM crypto implementation
- `crates/trst-wasm/src/crypto.rs` — Contains the bug at lines 186-187. The `decrypt()` function (line 149), `encrypt()` function (line 95), and `EncryptedData` struct.
- `crates/trst-wasm/src/lib.rs` — WASM module entry point, archive verification functions
- `crates/trst-wasm/Cargo.toml` — WASM crate dependencies (aes-gcm, base64, etc.)

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `encrypt()` function at line 95 — the counterpart to `decrypt()`, can be used in round-trip test
- `generate_key()` function — generates random 256-bit key for testing
- `EncryptedData` struct — returned by encrypt, consumed by decrypt

### Established Patterns
- `#[wasm_bindgen]` on public functions
- `console_log!` macro for debug logging
- Base64 encoding for keys and nonces
- `JsValue` error returns

### Integration Points
- `decrypt()` is called by browser JavaScript code for archive chunk decryption
- The fix is self-contained within `crypto.rs` — no cross-file changes needed

</code_context>

<specifics>
## Specific Ideas

No specific requirements — straightforward bug fix.

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope.

</deferred>

---

*Phase: 56-wasm-fix*
*Context gathered: 2026-03-23*
