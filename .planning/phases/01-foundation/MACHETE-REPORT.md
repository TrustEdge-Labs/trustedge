<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


# Unused Dependency Report (cargo-machete)

**Generated:** 2026-02-10

## Findings

```
trustedge-attestation -- ./crates/attestation/Cargo.toml:
	thiserror

trustedge-core -- ./crates/core/Cargo.toml:
	serde_bytes

trustedge-trst-cli -- ./crates/trst-cli/Cargo.toml:
	trustedge-trst-core

trustedge-trst-wasm -- ./crates/trst-wasm/Cargo.toml:
	blake3
	getrandom
	hex

trustedge-wasm -- ./crates/wasm/Cargo.toml:
	getrandom
	serde-wasm-bindgen
	web-sys
```

## Analysis

| Package | Unused Dep | Likely False Positive? | Notes |
|---------|-----------|----------------------|-------|
| trustedge-attestation | thiserror | Possible | May be used in derive macros not detected by regex |
| trustedge-core | serde_bytes | Possible | May be used via `#[serde(with = "serde_bytes")]` attribute |
| trustedge-trst-cli | trustedge-trst-core | **Likely false positive** | Workspace dep — may import types transitively through trustedge-core |
| trustedge-trst-wasm | blake3, getrandom, hex | Possible | WASM crates often import via wasm-bindgen indirection |
| trustedge-wasm | getrandom, serde-wasm-bindgen, web-sys | Possible | WASM feature flags and platform-specific imports |

## Decision

**Defer removals to Phase 8 (Validation)** per research recommendation. Removing dependencies during consolidation risks introducing build failures — better to clean up after all merges are complete.

False positives should be verified with `cargo machete --with-metadata` which uses cargo metadata for better accuracy. Some dependencies (serde_bytes, thiserror) are used via derive macros which cargo-machete's regex-based approach cannot detect.
