<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


# Phase 44: YubiKey CLI Integration - Context

**Gathered:** 2026-03-17
**Status:** Ready for planning

<domain>
## Phase Boundary

Expose YubiKey hardware signing in the `trst` CLI. Users can sign archives with ECDSA P-256 via `--backend yubikey`, and `trst verify` accepts both Ed25519 and ECDSA P-256 signatures. Demo script auto-detects YubiKey and adds an optional hardware signing step. This phase does NOT add new crypto algorithms or change the encryption system.

</domain>

<decisions>
## Implementation Decisions

### Signature Format & Verify Dispatch
- Key prefix convention: `"ecdsa-p256:<base64>"` for P-256 public keys (alongside existing `"ed25519:<base64>"`)
- Signature field: `"ecdsa-p256:<base64>"` for ECDSA signatures
- `manifest.device.public_key` uses the prefix to identify algorithm
- `trst verify` reads the prefix and dispatches to Ed25519 or ECDSA P-256 verifier
- `signatures/manifest.sig` stores raw signature bytes (format determined by manifest public_key prefix)
- No changes to the manifest JSON schema structure — only the string values change

### Wrap Workflow with YubiKey
- `trst wrap --backend yubikey --device-key device.key --in data.bin --out archive.trst`
- Chunks encrypted with HKDF from software `--device-key` (same as Phase 43 — YubiKey can't do symmetric crypto)
- Manifest signed by YubiKey ECDSA P-256 via PIV slot 9c (Digital Signature) by default
- `--slot` flag to override PIV slot (9a, 9c, 9d, 9e)
- `manifest.device.public_key` set to YubiKey's P-256 public key (extracted from PIV certificate)
- PIN prompted interactively via `rpassword` (not echoed to terminal, not a CLI flag)
- `trst unwrap` still uses `--device-key` for chunk decryption; verify step uses YubiKey public key from manifest

### Demo Script Integration
- Auto-detect YubiKey: check `ykman list` or equivalent
- If YubiKey present: add bonus step showing hardware signing (wrap + verify with YubiKey)
- If absent: skip with friendly message ("The demo works without hardware. Insert a YubiKey to see hardware signing.")
- Core demo (keygen, wrap, verify with software keys) always runs regardless of hardware

### Claude's Discretion
- How to extract P-256 public key from YubiKey PIV certificate for manifest
- Whether to add `rpassword` as optional dep or always-on for trst-cli
- ECDSA signature encoding (DER vs raw r||s) for the signature field
- How to structure the `--backend` flag in clap (enum vs string)
- Simulation tests vs hardware-only tests for CI

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### YubiKey backend
- `crates/core/src/backends/yubikey.rs` — YubiKeyBackend: ECDSA P-256 signing, public key extraction, PIN management, PIV slot dispatch (995 LOC)
- `crates/core/src/backends/universal.rs` — UniversalBackend trait, SignatureAlgorithm enum

### CLI
- `crates/trst-cli/src/main.rs` — handle_wrap(), handle_verify(), handle_unwrap() (current Ed25519-only signing/verification)
- `crates/core/src/crypto.rs` — sign_manifest(), verify_manifest() (Ed25519 signing/verification functions)

### Manifest
- `crates/trst-protocols/src/archive/manifest.rs` — TrstManifest, DeviceInfo.public_key field

### Research
- `.planning/research/FEATURES.md` — YubiKey P-256 constraint, signature format decision
- `.planning/research/PITFALLS.md` — Ed25519/ECDSA incompatibility, PIN UX, backend-before-CLI pattern warning

### Requirements
- `.planning/REQUIREMENTS.md` — YUBI-01 through YUBI-04

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `YubiKeyBackend::piv_sign()` — already does ECDSA P-256 signing with SHA-256 pre-hash
- `YubiKeyBackend::extract_public_key()` — gets P-256 public key from PIV certificate
- `YubiKeyConfig::builder()` — PIN, slot, verbose configuration
- `sign_manifest()` / `verify_manifest()` in crypto.rs — Ed25519 only, needs P-256 path
- `DeviceKeypair` — currently Ed25519 only, YubiKey signing bypasses this

### Established Patterns
- Fail-closed hardware design: `ensure_connected()` gates every operation
- `#[cfg(feature = "yubikey")]` feature gating
- `Secret<T>` wrapper for PIN handling
- 18 simulation tests (no hardware) + 9 hardware integration tests (#[ignore])

### Integration Points
- `crates/trst-cli/src/main.rs` — Add `--backend` flag, YubiKey signing path in handle_wrap()
- `crates/trst-cli/Cargo.toml` — Add `yubikey` feature flag forwarding to trustedge-core
- `crates/core/src/crypto.rs` — Add P-256 verify function or extend verify_manifest()
- `scripts/demo.sh` — Add YubiKey auto-detect step
- `crates/trst-cli/tests/acceptance.rs` — Simulation tests (no hardware required)

</code_context>

<specifics>
## Specific Ideas

- The preview from the discussion shows the exact demo output format for YubiKey detection
- `rpassword` crate for interactive PIN prompting (no echo) — research recommends v7.3
- The YubiKey backend already works (995 LOC, tested) — this phase is about CLI wiring, not backend work
- CLI-first approach: wire the CLI to the existing backend, not the other way around (avoids repeating v1.1 pattern)

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope.

</deferred>

---

*Phase: 44-yubikey-cli-integration*
*Context gathered: 2026-03-17*
