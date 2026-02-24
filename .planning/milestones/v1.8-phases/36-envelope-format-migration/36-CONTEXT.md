# Phase 36: Envelope Format Migration - Context

**Gathered:** 2026-02-23
**Status:** Ready for planning

<domain>
## Phase Boundary

Migrate envelope encryption from per-chunk HKDF-salt key derivation to single HKDF derivation with deterministic counter nonces. Add a version field to distinguish v1 (legacy per-chunk) from v2 (HKDF-once) envelopes. Backward-compatible decryption of existing v1 envelopes must be preserved.

</domain>

<decisions>
## Implementation Decisions

### Version field design
- Add `version: u8` as a top-level field on the `Envelope` struct (not in metadata)
- Use `#[serde(default)]` so deserialization of old v1 envelopes (which lack the field) defaults to version 1
- `seal()` always produces v2 envelopes — no opt-out or version parameter
- Decryption uses try-then-fallback: attempt v2 decryption first, if it fails retry as v1. More resilient to corrupt version fields

### Nonce construction layout
- NoncePrefix is HKDF-derived (not random-once stored in envelope) — fully deterministic, no extra field on wire format
- 12-byte AES-256-GCM nonce layout: 8-byte prefix + 3-byte chunk_index + 1-byte last_flag
- Supports up to 16M chunks (3 bytes = 16,777,216)
- chunk_index encoded big-endian (network byte order, standard for crypto protocols)
- last_flag: 0x00 for normal chunks, 0xFF for the final chunk

### Per-envelope salt & manifest changes
- Add `hkdf_salt: [u8; 32]` as a top-level field on the `Envelope` struct (alongside version)
- Salt size stays at 32 bytes, same as existing per-chunk salts
- ChunkManifest keeps `key_derivation_salt` and `pbkdf2_iterations` fields — zero them out in v2 envelopes ([0; 32] and 0). Simplest serde compat, no conditional serialization logic
- NetworkChunk nonce field is still populated with the computed deterministic nonce — consistent with v1 layout, useful for debugging/inspection

### HKDF info strings
- Keep `TRUSTEDGE_ENVELOPE_V1` as the info string for both v1 and v2 envelopes — version separation comes from format structure, not KDF domain
- Single HKDF-Expand call with 40-byte output: bytes 0..32 = encryption key, bytes 32..40 = nonce prefix. One call, no separate info strings
- AAD header hash keeps `b"ENVELOPE_V1"` for both versions — version separation not via AAD

### Claude's Discretion
- Internal function signatures and helper organization
- Error message wording
- Test structure and naming

</decisions>

<specifics>
## Specific Ideas

- Try-then-fallback for version detection means the decrypt path should attempt v2 first (expected common case) then fall back to v1 — keep v1 decrypt path as-is for backward compat
- The 8-byte nonce prefix + 3-byte index + 0xFF last flag gives visual distinctness in hex dumps
- Zeroing out ChunkManifest fields (rather than removing them) keeps the struct stable across both versions

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope

</deferred>

---

*Phase: 36-envelope-format-migration*
*Context gathered: 2026-02-23*
