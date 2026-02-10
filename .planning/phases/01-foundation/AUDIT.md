# Code Duplication Audit

**Generated:** 2026-02-10
**Scope:** All 10 workspace crates

## Section 1: Duplicate Types

| Type | Location 1 | Location 2 | Kind | Recommendation |
|------|-----------|-----------|------|----------------|
| `CamVideoManifest` | `core::manifest` | `trst-core::manifest` | exact | Keep trst-core (WASM-compatible by design) |
| `DeviceInfo` | `core::manifest` | `trst-core::manifest` | exact | Keep trst-core |
| `CaptureInfo` | `core::manifest` | `trst-core::manifest` | exact | Keep trst-core |
| `ChunkInfo` | `core::manifest` | `trst-core::manifest` | exact | Keep trst-core |
| `SegmentInfo` | `core::manifest` | `trst-core::manifest` | exact | Keep trst-core |
| `ManifestError` | `core::manifest` | `trst-core::manifest` | exact | Merge into unified TrustEdgeError (Phase 2) |
| `EncryptedData` | `wasm::crypto` | `trst-wasm::crypto` | exact | Consolidate into single WASM binding |
| `Timer` | `wasm::utils` | `trst-wasm::utils` | exact | Consolidate into single WASM binding |

**Total: 8 exact type duplicates across crate boundaries**

### Manifest Duplication Detail

The entire manifest module is duplicated between `core/src/manifest.rs` and `trst-core/src/manifest.rs`. Both files define identical structs and serialization logic. `trst-core` was designed as the WASM-compatible canonical source but the same types were also added to core.

## Section 2: Duplicate Functions

| Function | Location 1 | Location 2+ | Kind | Recommendation |
|----------|-----------|-------------|------|----------------|
| `generate_nonce` | `core::crypto` | `wasm::crypto`, `trst-wasm::crypto` | exact | Keep core, import in WASM |
| `verify_manifest` | `core::manifest` | `trst-wasm::lib` | near | Keep core (has domain separation variant) |
| `to_canonical_bytes` | `core::manifest` | `trst-core::manifest` | exact | Keep trst-core (WASM target) |
| `set_signature` | `core::manifest` | `trst-core::manifest` | exact | Keep trst-core |
| `validate` | `core::manifest`, `receipts::lib`, `trst-core::manifest` | — | pattern | Each validates its own domain — not true duplication |
| `to_json` / `from_json` | `pubky-advanced` | `wasm`, `trst-wasm` | near | Different serialization targets — keep separate |
| `seal` / `unseal` | `core::envelope` | `pubky-advanced::envelope` | near | core uses AES-256-GCM, pubky uses X25519+AES — different protocols |
| `safe_log` | `wasm::utils` | `trst-wasm::utils` | exact | Consolidate into single WASM binding |
| `log_elapsed` | `wasm::utils` | `trst-wasm::utils` | exact | Consolidate into single WASM binding |
| `validate_key` | `wasm::crypto` | `trst-wasm::crypto` | exact | Consolidate into single WASM binding |
| `validate_nonce` | `wasm::crypto` | `trst-wasm::crypto` | exact | Consolidate into single WASM binding |
| `nonce` | `wasm::crypto` | `trst-wasm::crypto` | exact | Consolidate into single WASM binding |
| `key_id` | `wasm::crypto` | `trst-wasm::crypto` | exact | Consolidate into single WASM binding |

**Total: 9 exact function duplicates, 4 near-duplicates**

### WASM Crypto Duplication Detail

`crates/wasm/src/crypto.rs` and `crates/trst-wasm/src/crypto.rs` are identical files — every function signature matches exactly. These should be a shared module imported by both WASM crates, or consolidated into a single trustedge-wasm crate.

## Section 3: Cross-Crate Dependency Usage

### Dependency Graph (who depends on whom)

```
trustedge-cli       → trustedge-core
trustedge-receipts  → trustedge-core
trustedge-attestation → trustedge-core (optional, feature-gated "envelope")
trustedge-pubky     → trustedge-core
trustedge-pubky-advanced → trustedge-core
trustedge-trst-cli  → trustedge-core + trustedge-trst-core
trustedge-trst-wasm → trustedge-trst-core
trustedge-wasm      → (standalone, no workspace deps)
trustedge-trst-core → (standalone, no workspace deps)
```

### Import Usage Table

| Consumer Crate | Imports From | Specific Items Used |
|----------------|-------------|---------------------|
| trustedge-cli | trustedge-core | `format`, `AudioCapture`, `AudioConfig`, `BackendRegistry`, `KeyBackend`, `KeyContext`, `KeyringBackend`, `Envelope`, `DeviceKeypair`, `encrypt_segment`, `sign_manifest` |
| trustedge-receipts | trustedge-core | `Envelope` |
| trustedge-attestation | trustedge-core | `Envelope` (optional, feature-gated) |
| trustedge-pubky | trustedge-core | `backends::*`, `PrivateKey`, `PublicKey`, `AsymmetricAlgorithm`, `KeyPair` |
| trustedge-pubky-advanced | trustedge-core | `format::AeadAlgorithm`, `NetworkChunk`, `NONCE_LEN` |
| trustedge-trst-cli | trustedge-core | `DeviceKeypair`, `sign_manifest`, `encrypt_segment`, `format::*` |
| trustedge-trst-cli | trustedge-trst-core | `CamVideoManifest` (CONFLICT: imports manifest from both core and trst-core!) |
| trustedge-trst-wasm | trustedge-trst-core | `CamVideoManifest` |

### Key Findings

1. **trustedge-core is the hub** — 6 crates depend on it directly (FanIn=6)
2. **trustedge-trst-core has limited consumers** — only trst-cli and trst-wasm (FanIn=2), making it safe to merge first
3. **trustedge-receipts uses minimal core API** — only imports `Envelope`, simple merge
4. **trustedge-attestation is loosely coupled** — optional dependency, feature-gated
5. **CONFLICT in trst-cli** — imports manifest types from BOTH core and trst-core, will need resolution during Phase 3

## Section 4: Merge Order Recommendation

The ROADMAP phase order is **validated as correct**:

| Phase | Merge Target | Risk | Rationale |
|-------|-------------|------|-----------|
| Phase 3: trst-core → core | Low | Only 2 consumers (trst-cli, trst-wasm). WASM-safe types, no complex deps. Resolves the biggest exact duplication (manifest types). |
| Phase 4: receipts → core | Low | Only uses `Envelope` from core. 1,281 LOC, self-contained. 23 tests well-isolated. |
| Phase 5: attestation → core | Low | Optional dependency on core. Feature-gated. Small surface area. |

**No changes recommended to ROADMAP order.**

### Additional Observations

- **WASM crate consolidation** (wasm + trst-wasm): Not in current ROADMAP but represents significant duplication. Consider adding as Phase 6.5 or handling during Phase 7 (Backward Compatibility).
- **Pubky crates**: Have their own `EnvelopeV2` and `DualKeyPair` types that are intentionally different from core. These are out of scope per v1 requirements.

## Section 5: Quantitative Summary

| Metric | Count |
|--------|-------|
| Exact type duplicates | 8 |
| Exact function duplicates | 9 |
| Near function duplicates | 4 |
| Total duplicate lines (estimated) | ~1,200+ |
| Affected crate pairs | core/trst-core, wasm/trst-wasm |
| Cross-crate dependencies | 8 (intra-workspace) |
| Circular dependencies | 0 |
