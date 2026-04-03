<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


# Phase 54: Transport Security - Context

**Gathered:** 2026-03-22
**Status:** Ready for planning

<domain>
## Phase Boundary

Fix the QUIC TLS signature verification no-op in `HardwareBackedVerifier` so that MITM attacks are cryptographically rejected. Both `verify_tls12_signature` and `verify_tls13_signature` must perform actual signature verification. The `accept_any_hardware()` dev mode must be gated behind a compile-time feature flag.

</domain>

<decisions>
## Implementation Decisions

### Verification approach
- **D-01:** Delegate TLS handshake signature verification to the rustls default provider (aws-lc-rs). The code already looks up the matching `SignatureScheme` — the fix is to call `verify()` on the actual message, cert, and signature instead of discarding the verifier with `_verifier`.
- **D-02:** Both `verify_tls12_signature` and `verify_tls13_signature` must use the same approach — call the provider's verify method with the real `message`, `cert`, and `dss` parameters.

### Dev mode policy
- **D-03:** Gate `accept_any_hardware()` behind the existing `insecure-tls` compile-time feature flag. This is consistent with how `SkipServerVerification` is already gated. Dev mode remains available for development but cannot ship in release builds.
- **D-04:** When `insecure-tls` is not enabled, `accept_any_hardware()` must not be callable — remove it from the public API surface in default builds.

### Test strategy
- **D-05:** Unit tests: construct invalid `DigitallySignedStruct` payloads and verify they are rejected by `verify_tls12_signature` and `verify_tls13_signature`. Also test that valid signatures pass.
- **D-06:** Integration tests: attempt a QUIC connection with a wrong/forged certificate and verify the handshake fails. Verify legitimate connections still succeed.

### Claude's Discretion
- Exact test certificate generation approach (self-signed vs rcgen)
- Whether to refactor common verification logic into a shared helper
- Error message text for verification failures

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Transport implementation
- `crates/core/src/transport/quic.rs` — Contains `HardwareBackedVerifier` (line 339), the no-op `verify_tls12_signature` (line 409) and `verify_tls13_signature` (line 431), `accept_any_hardware()` (line 357), and `SkipServerVerification` (line 471, behind `insecure-tls`)
- `crates/core/src/transport/` — Transport module root, may contain mod.rs with feature-gate patterns

### Prior phase context
- `.planning/phases/19-quic-security-hardening/19-01-PLAN.md` — v1.4 QUIC hardening plan (webpki-roots, insecure-tls feature gate)
- `.planning/phases/19-quic-security-hardening/19-VERIFICATION.md` — Verification of v1.4 QUIC work

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `rustls::crypto::aws_lc_rs::default_provider()` — Already imported and used in the no-op code. The provider's `signature_verification_algorithms` has the actual `verify()` capability.
- `SkipServerVerification` (line 471) — Reference implementation for the `insecure-tls` feature-gate pattern. `accept_any_hardware()` should follow the same `#[cfg(feature = "insecure-tls")]` pattern.
- 8 existing QUIC tests in the module — test infrastructure is in place.

### Established Patterns
- Feature gating: `#[cfg(feature = "insecure-tls")]` already used for `SkipServerVerification`
- Error types: `rustls::Error::InvalidCertificate` and `rustls::Error::UnsupportedNameType` used in existing code
- The `verify_server_cert` method already does real validation (checks cert against trusted list) — only the signature methods are broken

### Integration Points
- `create_hardware_verified_endpoint()` (line 114) — The public API that creates QUIC endpoints with `HardwareBackedVerifier`. This is where `accept_any_hardware()` is called.
- The standard QUIC path uses `webpki-roots` and doesn't go through `HardwareBackedVerifier` — only the hardware-verified endpoint path is affected.

</code_context>

<specifics>
## Specific Ideas

No specific requirements — the fix is well-defined: use the verifier that's already looked up instead of discarding it.

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope.

</deferred>

---

*Phase: 54-transport-security*
*Context gathered: 2026-03-22*
