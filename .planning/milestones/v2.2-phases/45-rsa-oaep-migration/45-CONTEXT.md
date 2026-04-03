<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


# Phase 45: RSA OAEP Migration - Context

**Gathered:** 2026-03-18
**Status:** Ready for planning

<domain>
## Phase Boundary

Replace RSA PKCS#1 v1.5 padding with OAEP-SHA256 in the two RSA functions in asymmetric.rs (`rsa_encrypt_key` and `rsa_decrypt_key`). Hard reject old ciphertext — no fallback. Update existing tests.

</domain>

<decisions>
## Implementation Decisions

### Padding Migration
- Replace `Pkcs1v15Encrypt` with `Oaep::new::<Sha256>()` in both encrypt and decrypt functions
- Hard reject: old PKCS#1 v1.5 ciphertext fails to decrypt with an error (no fallback)
- No transition period — RSA path is for hybrid key exchange, not long-term storage, no production users have v1.5 ciphertext

### OAEP Configuration
- Hash algorithm: SHA-256 (standard, matches the rest of the codebase's SHA-256 usage)
- Use the `rsa` crate's built-in `Oaep` type from `rsa::Oaep`

### Backward Compatibility
- NOT required — clean break
- The RSA Marvin Attack advisory (RUSTSEC-2023-0071) can be removed from `.cargo/audit.toml` risk-accepted list after this change

### Claude's Discretion
- Whether to add `sha2` as a direct dependency or use it through the `rsa` crate's re-export
- Exact error message text for OAEP decryption failures
- Test structure changes (existing tests just need padding swap)

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### RSA code
- `crates/core/src/asymmetric.rs` — `rsa_encrypt_key()` at line 300, `rsa_decrypt_key()` at line 318, tests at line 395+
- `.cargo/audit.toml` — RUSTSEC-2023-0071 risk acceptance (can be removed after fix)

### Requirements
- `.planning/REQUIREMENTS.md` — RSA-01, RSA-02

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `rsa` crate already in workspace — has `Oaep` type built-in
- `sha2` crate already in workspace — provides `Sha256` for OAEP label hash
- Two functions to change: `rsa_encrypt_key` (line 300) and `rsa_decrypt_key` (line 318)

### Established Patterns
- `use rsa::{Pkcs1v15Encrypt, RsaPublicKey}` → change to `use rsa::{Oaep, RsaPublicKey}`
- `sha2::Sha256` already used elsewhere in the codebase

### Integration Points
- `crates/core/src/asymmetric.rs` — The only file that needs code changes
- `.cargo/audit.toml` — Remove RUSTSEC-2023-0071 acceptance
- `crates/core/src/asymmetric.rs` tests — Update `test_rsa_key_encryption` to use OAEP

</code_context>

<specifics>
## Specific Ideas

- This is a 2-line change per function (swap `Pkcs1v15Encrypt` for `Oaep::new::<Sha256>()`)
- The `rsa` crate's `Oaep` type handles all the OAEP encoding internally
- After this change, the RUSTSEC-2023-0071 advisory is fully mitigated (not just risk-accepted)

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope.

</deferred>

---

*Phase: 45-rsa-oaep-migration*
*Context gathered: 2026-03-18*
