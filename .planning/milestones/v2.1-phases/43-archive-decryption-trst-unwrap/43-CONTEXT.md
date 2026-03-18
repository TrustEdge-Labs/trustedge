# Phase 43: Archive Decryption (trst unwrap) - Context

**Gathered:** 2026-03-17
**Status:** Ready for planning

<domain>
## Phase Boundary

Fix `trst wrap` to use real key derivation (replacing the hardcoded demo key), store nonces alongside chunk ciphertext, and add `trst unwrap` to decrypt and reassemble original data. Verify-before-decrypt is mandatory. This completes the wrap/unwrap data lifecycle.

</domain>

<decisions>
## Implementation Decisions

### Key Derivation
- HKDF-SHA256 Extract+Expand from Ed25519 device signing key bytes
- Domain separation info: `"TRUSTEDGE_TRST_CHUNK_KEY"`
- Empty salt (device key is high-entropy input)
- Produces 32-byte XChaCha20Poly1305 key
- Same HKDF pattern established in envelope.rs v2 (v1.8 milestone)
- Whoever has the device private key can decrypt — no separate key file needed
- Replaces the hardcoded `b"0123456789abcdef0123456789abcdef"` at line 327 of main.rs

### Nonce Storage
- Prepend 24-byte random nonce to each chunk file: `[nonce:24][ciphertext:N]`
- On unwrap: read first 24 bytes as nonce, rest as ciphertext
- Self-contained — no manifest schema changes
- `write_archive()` chunk format changes from raw ciphertext to nonce+ciphertext
- `read_archive()` returns raw bytes; unwrap caller splits nonce from ciphertext

### Unwrap CLI Design
- Command: `trst unwrap <archive.trst> --device-key <key> --out <file>`
- Output: single reassembled file (concatenated decrypted chunks in order)
- Progress output: Signature PASS/FAIL, Continuity PASS/FAIL, chunk count, byte count, output path
- On verification failure: exit non-zero, print failure reason, NEVER write any plaintext
- No `--force` flag — verify-before-decrypt is non-negotiable

### Backward Compatibility
- Archives created with the hardcoded demo key will NOT be decryptable by `trst unwrap`
- This is acceptable — demo-key archives are test artifacts, not production data (documented in REQUIREMENTS.md Out of Scope)
- Old archives without prepended nonces will fail to unwrap (nonce extraction will get wrong data)
- `trst wrap` and `trst verify` continue to work for old archives (verify doesn't need decryption)

### Claude's Discretion
- Whether to add a `derive_chunk_key()` helper in crypto.rs or inline in handle_wrap/handle_unwrap
- How to update `write_archive()` for nonce prepending — modify function or prepend in CLI before calling
- Test structure: unit tests for key derivation + acceptance tests for wrap-then-unwrap round-trip
- Error messages for wrong-key decryption failures
- Whether `--device-pub` is needed for unwrap (needed for verification step, may be extractable from manifest)

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Encryption system
- `crates/core/src/crypto.rs` — `encrypt_segment()`, `decrypt_segment()` (XChaCha20Poly1305 with AAD)
- `crates/core/src/envelope.rs` — HKDF-SHA256 pattern from v1.8 (reference for key derivation approach)

### Archive format
- `crates/core/src/archive.rs` — `write_archive()`, `read_archive()` (chunk file I/O)
- `crates/trst-cli/src/main.rs` — `handle_wrap()` at line ~280 (hardcoded key at line 327, nonce generation, chunk encryption loop)

### Verification
- `crates/trst-cli/src/main.rs` — `handle_verify()` (signature + continuity chain verification logic)
- `crates/core/src/chain.rs` — BLAKE3 continuity chain

### Research
- `.planning/research/FEATURES.md` — Hardcoded key blocker, verify-before-decrypt requirement
- `.planning/research/PITFALLS.md` — Nonce storage gap, key management design debt
- `.planning/research/ARCHITECTURE.md` — decrypt_archive() composition from existing primitives

### Requirements
- `.planning/REQUIREMENTS.md` — UNWRAP-01 through UNWRAP-04

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `decrypt_segment(key, nonce24, ciphertext, aad)` in crypto.rs — already works, tested
- `generate_aad(version, profile, device_id, started_at)` — AAD construction for decryption
- `read_archive()` — reads manifest + chunk files from .trst directory
- `validate_archive()` + `verify_manifest()` — verification pipeline for verify-before-decrypt
- HKDF pattern from `envelope.rs` — `hkdf::Hkdf<sha2::Sha256>` with info parameter

### Established Patterns
- XChaCha20Poly1305 for chunk encryption (24-byte nonces, AAD)
- HKDF-SHA256 for key derivation (established in v1.8 envelope format)
- Ed25519 device key pair for signing (DeviceKeypair struct)
- Chunk files: zero-padded five-digit names (`00000.bin`, `00001.bin`, ...)
- `generate_seeded_nonce24()` for deterministic test output

### Integration Points
- `crates/trst-cli/src/main.rs` — Add `Unwrap` command variant, `handle_unwrap()`, fix `handle_wrap()` key derivation
- `crates/core/src/crypto.rs` — Add `derive_chunk_key()` helper (HKDF from Ed25519 bytes)
- `crates/core/src/archive.rs` — Update `write_archive()` for nonce-prepended chunk format
- `crates/trst-cli/tests/acceptance.rs` — Add wrap-then-unwrap round-trip tests

</code_context>

<specifics>
## Specific Ideas

- The preview from the discussion shows the exact CLI output format for unwrap
- Key derivation uses the same HKDF crate already in the workspace (from envelope.rs)
- The nonce-prepend approach means chunk files grow by exactly 24 bytes each — negligible overhead
- Verification before decryption reuses the existing verify pipeline (`validate_archive` + `verify_manifest`)

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope.

</deferred>

---

*Phase: 43-archive-decryption-trst-unwrap*
*Context gathered: 2026-03-17*
