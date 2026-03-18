# Phase 46: Envelope Hardening - Context

**Gathered:** 2026-03-18
**Status:** Ready for planning

<domain>
## Phase Boundary

Remove v1 envelope format entirely (not just deprecate — delete the code), enforce PBKDF2 minimum iterations at 300k in the keyring backend, and ensure seal() only produces v2 envelopes. No v1 envelopes exist in production (solo dev, no external users).

</domain>

<decisions>
## Implementation Decisions

### v1 Envelope Format
- **Remove entirely** — not deprecate. Delete the v1 fallback code path in unseal(), delete decrypt_chunk_v1(), delete v1 test builders
- No v1 envelopes exist in production (solo developer, no external users)
- unseal() becomes v2-only: derive key, decrypt chunks, done
- v1 round-trip tests that build v1 envelopes should be deleted (they test dead code)
- Keep the v2 path clean — no conditional branching for legacy formats
- ENV-01 satisfied by removal (stronger than deprecation warning)

### seal() v2-only
- seal() already produces v2 envelopes (from v1.8 milestone)
- Verify no code path creates v1 format — remove any remaining v1 seal code if found
- ENV-02 is likely already satisfied; verify and mark complete

### PBKDF2 Minimum Enforcement
- Enforce 300,000 minimum iterations in the keyring backend's derive_key()
- Default stays at 600,000 (OWASP 2023)
- If caller passes iterations < 300,000: return BackendError::InvalidParameter with clear message
- Update the test at line 311 that uses `with_iterations(1000)` — it should either use 300k+ or test the rejection
- KDF-01 satisfied by backend enforcement

### Claude's Discretion
- Whether to also enforce the minimum in KeyDerivationContext::with_iterations() (belt + suspenders)
- How to handle the v1 test cleanup — which tests to delete vs update
- Error message wording for PBKDF2 violation

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Envelope code
- `crates/core/src/envelope.rs` — unseal() v1 fallback at line 274, decrypt_chunk_v1() method, seal() at line 137, v1 test at line 923

### Keyring backend
- `crates/core/src/backends/universal_keyring.rs` — PBKDF2 at line 77 (600k default), derive_key() implementation, test with 1000 iterations at line 311

### Requirements
- `.planning/REQUIREMENTS.md` — ENV-01, ENV-02, KDF-01

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- v2 decrypt path (lines 241-268) — stays as-is, becomes the only path
- tracing crate already in workspace — for any logging needs

### Established Patterns
- `BackendError::InvalidParameter` — exists for parameter validation errors
- `Zeroize` on key material after use — maintain in cleaned-up unseal()
- v1.8 established v2 as default; this phase completes the migration by removing v1

### Integration Points
- `crates/core/src/envelope.rs` — Remove v1 fallback in unseal(), delete decrypt_chunk_v1()
- `crates/core/src/backends/universal_keyring.rs` — Add iteration minimum check
- Tests in both files need updating

</code_context>

<specifics>
## Specific Ideas

- The user explicitly said "there are no v1 envelopes...solo dev and builder, no need to support legacy just make it better"
- Removing v1 code makes the unseal() function simpler and removes dead code paths
- The PBKDF2 test with 1000 iterations (line 311) should become a test that verifies the minimum is enforced

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope.

</deferred>

---

*Phase: 46-envelope-hardening*
*Context gathered: 2026-03-18*
