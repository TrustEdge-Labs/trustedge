# Phase 52: Code Hardening - Context

**Gathered:** 2026-03-22
**Status:** Ready for planning

<domain>
## Phase Boundary

Fix all P1/P2 code-level findings from the security review: replace custom base64 with standard crate, version encrypted key file format, make timestamp check unidirectional, eliminate unwrap/expect from security paths, enforce 0600 permissions on generated key files, and guard nonce construction against chunk index overflow.

</domain>

<decisions>
## Implementation Decisions

### Base64 replacement (CRYP-01)
- **D-01:** Replace custom `base64_encode()`/`base64_decode()` in `crates/core/src/crypto.rs:485-544` with `base64::engine::general_purpose::STANDARD` from the `base64` crate (already used in 4 other workspace crates: trst-cli, trst-wasm, wasm, platform)
- **D-02:** All 23 call sites in crypto.rs update to use the crate. Wire format (STANDARD alphabet with padding) must match existing encoded values for backward compatibility
- **D-03:** Delete the custom implementations entirely — no fallback or feature gate

### Key file format versioning (CRYP-02)
- **D-04:** Add `"version": 1` field to the TRUSTEDGE-KEY-V1 JSON metadata (alongside existing salt, nonce, iterations fields)
- **D-05:** Keep the `TRUSTEDGE-KEY-V1` header string unchanged — the version field inside JSON is for future iteration/algorithm changes, not a header format bump
- **D-06:** Reader (`import_secret_encrypted`) must accept JSON both with and without the version field — existing v2.2/v2.3 key files lack it, treat missing version as version 1
- **D-07:** Document the iteration policy as a constant with a comment explaining OWASP 2023 rationale

### Timestamp check (AUTH-01)
- **D-08:** Replace `abs_diff()` at `auth.rs:442` with a two-part check: reject if `response.timestamp > now + tolerance` (future) OR `now - response.timestamp > tolerance` (too old). Past tolerance stays at 300 seconds. Future tolerance set to 5 seconds (minimal clock drift)
- **D-09:** Error messages distinguish "timestamp too far in the future" from "timestamp too old" for debuggability

### unwrap/expect audit (AUTH-02)
- **D-10:** Fix 2 critical sites in `envelope.rs:264,274` — `beneficiary()` and `issuer()` return `Result<VerifyingKey>` instead of panicking on invalid key bytes
- **D-11:** The defensive `unwrap_or_default()` calls in auth.rs (lines 360, 369, 532) and `bincode::serialize().unwrap_or_default()` in envelope.rs:257 are acceptable — they have safe fallbacks and are not security-critical
- **D-12:** Test-only expect() calls (24 in envelope.rs tests) are fine — don't touch

### File permissions (KEYF-01)
- **D-13:** Apply 0600 permissions to secret key file only (both encrypted and unencrypted paths). Public key file keeps default permissions (it's public)
- **D-14:** Use `#[cfg(unix)]` with `std::os::unix::fs::PermissionsExt` — set permissions immediately after `fs::write()` succeeds
- **D-15:** On non-Unix (Windows), log a warning to stderr that manual permission restriction is recommended — don't fail
- **D-16:** Existing overwrite protection in handle_keygen (lines 326-337) is preserved as-is

### Chunk index overflow (KEYF-02)
- **D-17:** Add an explicit check before the `sequence as u32` cast at envelope.rs:304 and :444 — if sequence >= 2^24 (since only 3 bytes are used from the u32), return an error
- **D-18:** Error message: "Chunk index exceeds maximum (16,777,215) — envelope too large for current nonce format"
- **D-19:** The check goes in both seal (encryption) and unseal (decryption) paths for consistency

### Claude's Discretion
- Exact base64 crate version (use whatever is already in workspace Cargo.lock)
- Whether to extract the timestamp tolerance as a named constant
- Error type choice for beneficiary()/issuer() return — use existing TrustEdgeError or anyhow
- Whether to use fs::OpenOptions with restricted mode vs post-write chmod (prefer post-write for simplicity)

</decisions>

<specifics>
## Specific Ideas

No specific requirements — user delegated all implementation choices to Claude. Standard security best practices apply.

</specifics>

<canonical_refs>
## Canonical References

No external specs — requirements are fully captured in decisions above and the security review report at repo root (`security-review-report.md`).

### Security review source
- `security-review-report.md` — P1-1 through P1-4, P2-1, P2-5 findings with locations and impact

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `base64` crate already in workspace Cargo.lock via trst-cli, trst-wasm, wasm, platform — no new dependency introduction needed
- `DeviceKeypair` struct in crypto.rs owns all key import/export — single entry point for format changes
- `anyhow` already used throughout CLI and core for error propagation

### Established Patterns
- `with_context(|| ...)` pattern used consistently in trst-cli for IO error wrapping
- `#[cfg(unix)]` platform-conditional code has precedent in the codebase
- TRUSTEDGE-KEY-V1 format is header line + JSON metadata line + binary ciphertext — adding JSON fields is non-breaking
- `serde_json::json!()` macro used for metadata serialization (crypto.rs:163-167)

### Integration Points
- `base64_encode()`/`base64_decode()` are private functions in crypto.rs — no external API surface change
- `beneficiary()`/`issuer()` on `Envelope` are public methods — signature change from `VerifyingKey` to `Result<VerifyingKey>` affects callers in receipts and attestation modules
- `handle_keygen()` in trst-cli is the only key file writer — single change point for permissions
- Nonce construction helper is inline in `seal()`/`unseal()` — consider extracting to a shared function for DRY

### Key file locations
- Export encrypted: crypto.rs:135-170 (`export_secret_encrypted`)
- Import encrypted: crypto.rs:197-260 (`import_secret_encrypted`)
- Write to disk: trst-cli/main.rs:324-368 (`handle_keygen`)
- Test coverage: crypto.rs:832-848 (metadata structure test)

</code_context>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope.

</deferred>

---

*Phase: 52-code-hardening*
*Context gathered: 2026-03-22*
