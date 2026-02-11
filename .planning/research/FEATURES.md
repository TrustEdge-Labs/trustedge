# Feature Research: YubiKey PIV Backend

**Domain:** Hardware cryptographic backend (YubiKey PIV)
**Researched:** 2026-02-11
**Confidence:** HIGH

## Feature Landscape

### Table Stakes (Users Expect These)

Features users assume exist. Missing these = product feels incomplete.

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| **Ed25519 Signing** | Standard PIV operation, existing Universal Backend trait supports it | MEDIUM | Via PKCS#11 C_Sign with slot-based key selection. Requires mapping PIV slots (9a/9c/9d/9e) to PKCS#11 object IDs |
| **ECDSA P-256 Signing** | PIV standard algorithm (NIST SP 800-73-4 mandates P-256 support) | MEDIUM | Similar to Ed25519 but different key type. YubiKey PIV natively supports P-256 |
| **RSA-2048/4096 Signing** | PIV standard for compatibility with legacy systems | MEDIUM | PKCS#1 v1.5 and PSS padding schemes. Larger signature sizes (~256/512 bytes) |
| **Public Key Extraction** | Required to verify signatures, build certificates, export keys | LOW | Via PKCS#11 C_GetAttributeValue for public key objects or certificate parsing |
| **PIV Slot Enumeration** | Users need to know which slots (9a/9c/9d/9e) have keys | MEDIUM | Standard PIV slots: 9A=Auth, 9C=Signature, 9D=Key Management, 9E=Card Auth |
| **Fail-Closed Hardware Check** | MUST error if hardware unavailable, never silently fall back to software | HIGH | Critical security property. Detection via PKCS#11 slot enumeration + yubikey crate hardware probe |
| **PKCS#11 Session Management** | Standard interface for HSM operations | MEDIUM | C_Initialize, C_OpenSession, C_Login with PIN, session cleanup on Drop |
| **Certificate Management (Read)** | Read existing X.509 certificates from PIV slots | MEDIUM | Certificates prove key ownership, needed for TLS/QUIC integration |
| **Error Handling (Hardware-Specific)** | Distinguish "no hardware", "wrong PIN", "slot empty", "operation failed" | MEDIUM | Map PKCS#11 error codes to BackendError variants. NO unwrap() calls |

### Differentiators (Competitive Advantage)

Features that set the product apart. Not required, but valuable.

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| **Hardware Attestation** | Cryptographic proof operation happened in hardware, not software | HIGH | YubiKey attestation via slot F9 certificate chain. Proves key is hardware-bound |
| **Certificate Generation (Self-Signed)** | Generate X.509 certificates directly from PIV keys for TLS/QUIC | HIGH | Use x509-cert crate (battle-tested) NOT manual DER encoding. Must sign with hardware |
| **Multi-Slot Operations** | Use different PIV slots for different purposes (signing vs TLS) | LOW | Already supported by PIV standard, just needs key_id = slot mapping |
| **PIN Caching (Secure)** | Cache PIN in memory with zeroize for session duration | MEDIUM | Avoids repeated prompts. MUST use zeroize::Zeroizing<String> to clear on drop |
| **Key Usage Validation** | Ensure slot usage matches PIV spec (9C for signing, 9D for key management) | LOW | Validates against PIV standard slot purposes. Prevents misuse |
| **Verbose Diagnostics Mode** | Detailed logging for debugging hardware issues | LOW | Config flag for println! diagnostics. Helps diagnose PKCS#11 issues |

### Anti-Features (Commonly Requested, Often Problematic)

Features that seem good but create problems.

| Feature | Why Requested | Why Problematic | Alternative |
|---------|---------------|-----------------|-------------|
| **Software Fallback** | "Make it work without hardware" | Silently downgrades security. User thinks they have hardware protection but don't | Fail-closed: Return error if hardware unavailable. Let caller decide fallback |
| **Placeholder Keys** | "Let tests pass without hardware" | Insecure. Tests pass but don't validate real behavior | Separate test suites: simulation tests (no hardware) vs #[ignore] hardware tests |
| **Manual Crypto (DER encoding, etc.)** | "We can do it ourselves" | Bug-prone, unaudited, violates "battle-tested only" rule | Use x509-cert, der, spki crates from RustCrypto |
| **Auto-Generated PINs** | "Convenience for demos" | Defeats YubiKey security model. User must control PIN | Require PIN via config or prompt. Document that tests need manual PIN entry |
| **Key Generation On-Device** | "Generate keys in YubiKey" | Complex, requires management key, PIV initialization, not needed for v1 | Import existing keys or use pre-initialized YubiKeys. Defer to v2+ |
| **Symmetric Encryption (AES in YubiKey)** | "YubiKey has AES" | PIV applet doesn't expose AES for general encryption, only for internal operations | Use YubiKey for signing/attestation only. Symmetric ops go to Software HSM backend |

## Feature Dependencies

```
[Fail-Closed Hardware Check]
    └──requires──> [PKCS#11 Session Management]
                       └──requires──> [PIV Slot Enumeration]

[Certificate Generation]
    └──requires──> [Ed25519/ECDSA/RSA Signing]
    └──requires──> [Public Key Extraction]

[Hardware Attestation]
    └──requires──> [Certificate Management (Read)]
    └──requires──> [PIV Slot Enumeration]

[PIN Caching] ──enhances──> [PKCS#11 Session Management]

[Multi-Slot Operations] ──enhances──> [PIV Slot Enumeration]

[Software Fallback] ──conflicts──> [Fail-Closed Hardware Check]
[Placeholder Keys] ──conflicts──> [Hardware Attestation]
```

### Dependency Notes

- **Fail-Closed Hardware Check requires PKCS#11 Session Management:** Must attempt real session creation to determine hardware presence. Cannot rely on stubs.
- **Certificate Generation requires Signing + Public Key Extraction:** X.509 certificate = public key + metadata + signature. Need both to build valid cert.
- **Hardware Attestation requires Certificate Management:** Attestation chains back to Yubico factory certificates stored in slot F9.
- **PIN Caching enhances PKCS#11 Session Management:** Avoids C_Login on every operation. Improves UX without compromising security (PIN still required once per session).
- **Software Fallback conflicts with Fail-Closed:** Core design decision. Rewrite eliminates fallbacks entirely.

## MVP Definition

### Launch With (v1 - YubiKey Backend Rewrite)

Minimum viable product — what's needed to validate the concept.

- [x] **Ed25519 Signing** — Core operation, already in Universal Backend trait
- [x] **ECDSA P-256 Signing** — PIV standard, needed for TLS compatibility
- [x] **RSA-2048 Signing** — Legacy compatibility (many systems require RSA)
- [x] **Public Key Extraction** — Required to verify signatures, export keys
- [x] **PIV Slot Enumeration** — Users must know which slots have keys (9a/9c/9d/9e)
- [x] **Fail-Closed Hardware Check** — Critical security property, non-negotiable
- [x] **PKCS#11 Session Management** — Standard interface, table stakes
- [x] **Certificate Management (Read)** — Read existing certs from PIV slots
- [x] **Error Handling (Hardware-Specific)** — Distinguish error types, no unwrap()
- [x] **Verbose Diagnostics Mode** — Config flag for debugging (low complexity)

**Launch Criteria:** All table stakes features working with real hardware tests (#[ignore] tests pass with YubiKey plugged in).

### Add After Validation (v1.x)

Features to add once core is working.

- [ ] **Hardware Attestation** — HIGH complexity, add when attestation flows are designed (trigger: QUIC/TLS integration needs attestation)
- [ ] **Certificate Generation (Self-Signed)** — HIGH complexity, add when TLS certificate workflow is defined (trigger: network integration milestone)
- [ ] **PIN Caching (Secure)** — MEDIUM complexity, add if UX feedback indicates PIN prompts are annoying (trigger: user testing)
- [ ] **Multi-Slot Operations** — LOW complexity, add when users need to differentiate slot purposes (trigger: request for separate signing vs auth keys)
- [ ] **Key Usage Validation** — LOW complexity, add as hardening after core works (trigger: post-MVP security review)

### Future Consideration (v2+)

Features to defer until product-market fit is established.

- [ ] **Key Generation On-Device** — Requires PIV management key handling, initialization flows. Defer until user requests it.
- [ ] **RSA-4096 Support** — Slower, less common. Defer unless compatibility requirement emerges.
- [ ] **Certificate Revocation Checking** — Complex OCSP/CRL integration. Defer to dedicated PKI milestone.
- [ ] **Smart Card Reader Auto-Detection** — Handle multiple readers, auto-select YubiKey. Defer until multi-reader scenarios emerge.

## Feature Prioritization Matrix

| Feature | User Value | Implementation Cost | Priority |
|---------|------------|---------------------|----------|
| Fail-Closed Hardware Check | HIGH | MEDIUM | P1 |
| Ed25519 Signing | HIGH | MEDIUM | P1 |
| ECDSA P-256 Signing | HIGH | MEDIUM | P1 |
| RSA-2048 Signing | MEDIUM | MEDIUM | P1 |
| Public Key Extraction | HIGH | LOW | P1 |
| PIV Slot Enumeration | HIGH | MEDIUM | P1 |
| PKCS#11 Session Management | HIGH | MEDIUM | P1 |
| Certificate Management (Read) | HIGH | MEDIUM | P1 |
| Error Handling (Hardware-Specific) | HIGH | MEDIUM | P1 |
| Verbose Diagnostics Mode | MEDIUM | LOW | P1 |
| Hardware Attestation | HIGH | HIGH | P2 |
| Certificate Generation (Self-Signed) | HIGH | HIGH | P2 |
| PIN Caching (Secure) | MEDIUM | MEDIUM | P2 |
| Multi-Slot Operations | LOW | LOW | P2 |
| Key Usage Validation | LOW | LOW | P2 |
| Key Generation On-Device | LOW | HIGH | P3 |
| RSA-4096 Support | LOW | MEDIUM | P3 |
| Certificate Revocation Checking | LOW | HIGH | P3 |
| Smart Card Reader Auto-Detection | LOW | MEDIUM | P3 |

**Priority key:**
- P1: Must have for v1 launch (YubiKey backend rewrite)
- P2: Should have, add in v1.x when triggered
- P3: Nice to have, defer to v2+

## Testing Strategy

### Real Hardware Tests (#[ignore])

Tests that REQUIRE physical YubiKey. Run with `cargo test --ignored --features yubikey`.

| Test | What It Validates | Complexity |
|------|-------------------|------------|
| **Strict Hardware Detection** | YubiKey is plugged in, PKCS#11 detects it | LOW |
| **Sign with Ed25519** | C_Sign produces valid Ed25519 signature from slot 9a | MEDIUM |
| **Sign with ECDSA P-256** | C_Sign produces valid P-256 signature from slot 9c | MEDIUM |
| **Sign with RSA-2048** | C_Sign produces valid RSA signature from slot 9d | MEDIUM |
| **Extract Public Key** | C_GetAttributeValue retrieves public key DER | MEDIUM |
| **Slot Enumeration** | Detects all 4 PIV slots (9a/9c/9d/9e) | LOW |
| **Read Certificate** | Retrieves X.509 cert from slot with cert | MEDIUM |
| **PIN Authentication** | C_Login succeeds with correct PIN, fails with wrong PIN | LOW |
| **Error on Empty Slot** | Returns KeyNotFound when slot has no key | LOW |
| **Error on No Hardware** | Returns HardwareError when YubiKey unplugged mid-operation | LOW |

### Simulation Tests (No Hardware)

Tests that validate logic WITHOUT requiring YubiKey. Run with `cargo test --features yubikey`.

| Test | What It Validates | Complexity |
|------|-------------------|------------|
| **Capability Reporting** | get_capabilities() returns correct algorithms | LOW |
| **Supports Operation Check** | supports_operation() matches capability flags | LOW |
| **Backend Info** | backend_info() reports correct name/version | LOW |
| **Slot ID Mapping** | Maps "9a" string to PKCS#11 object ID correctly | LOW |
| **Config Validation** | Validates PKCS#11 module path exists | LOW |
| **Error Code Mapping** | Maps PKCS#11 errors to BackendError variants | MEDIUM |

### Anti-Pattern Tests (Must Fail)

Tests that verify anti-features DON'T exist.

| Test | What It Validates | Complexity |
|------|-------------------|------------|
| **No Software Fallback** | Sign operation MUST fail if hardware unavailable (no silent fallback) | LOW |
| **No Placeholder Keys** | MUST NOT return synthetic keys when slot is empty | LOW |
| **No Auto-Pass Tests** | Hardware tests MUST fail if YubiKey not plugged in | LOW |

## Implementation Guidelines

### Use Battle-Tested Libraries

| Operation | Library | Rationale |
|-----------|---------|-----------|
| PKCS#11 Interface | `pkcs11 = "0.5"` | Existing, already in Cargo.toml |
| YubiKey Detection | `yubikey = "0.7"` | Official Yubico Rust crate |
| X.509 Certificates | `x509-cert = "0.2"` | RustCrypto, NIST-standard DER encoding |
| DER Encoding | `der = "0.7"` | RustCrypto, audited ASN.1 DER |
| Public Key Formats | `spki = "0.7"` | RustCrypto, SubjectPublicKeyInfo handling |
| Signature Traits | `signature = "2.2"` | RustCrypto, standard signing interface |
| Ed25519 Operations | `ed25519-dalek = "2"` | De facto standard, constant-time |
| ECDSA P-256 Operations | `p256 = "0.13"` | RustCrypto NIST P-256 |
| RSA Operations | `rsa = "0.9"` | RustCrypto RSA PKCS#1 |
| Secure Memory | `zeroize = "1.7"` | Clear sensitive data (PINs) on drop |

**Rule:** NO manual cryptography. NO manual DER encoding. Use audited libraries only.

### Fail-Closed Behavior

Every operation MUST follow this pattern:

```rust
fn perform_operation(&self, key_id: &str, operation: CryptoOperation) -> Result<CryptoResult, BackendError> {
    // 1. Check hardware is available
    let pkcs11 = self.pkcs11.as_ref()
        .ok_or_else(|| BackendError::HardwareError("YubiKey not initialized".into()))?;

    // 2. Check session is active
    let session = self.session
        .ok_or_else(|| BackendError::HardwareError("No active PKCS#11 session".into()))?;

    // 3. Perform operation via PKCS#11
    // NO software fallback if C_Sign/C_GetAttributeValue fails

    // 4. Return result or error (never silently degrade)
}
```

**Anti-Pattern:**
```rust
// WRONG: Silent fallback
match hardware_sign(data) {
    Ok(sig) => sig,
    Err(_) => software_sign(data), // ← NEVER DO THIS
}
```

### Testing Requirements

1. **Every P1 feature MUST have:**
   - At least 1 simulation test (no hardware required)
   - At least 1 #[ignore] hardware test (requires YubiKey)
   - At least 1 negative test (error path)

2. **Hardware tests MUST:**
   - Use `#[ignore]` attribute
   - Use `YUBIKEY_HARDWARE_TEST_MUTEX` for sequential execution
   - Detect hardware with `YubikeyTestEnvironment::detect()`
   - Skip gracefully if no hardware: `if !env.has_hardware() { return Ok(()); }`
   - Document required YubiKey state (e.g., "slot 9a must have Ed25519 key")

3. **Simulation tests MUST:**
   - Test pure logic (capability checks, config validation, error mapping)
   - NOT require YubiKey hardware
   - Run in CI without feature flags

4. **NO placeholder/auto-pass tests:**
   - Every test MUST validate actual behavior
   - NO `Ok(())` stubs that always pass
   - NO mock objects that return hardcoded success

## Existing Universal Backend Integration

### Trait Methods to Implement

From `/home/john/vault/projects/github.com/trustedge/crates/core/src/backends/universal.rs`:

```rust
pub trait UniversalBackend: Send + Sync {
    fn perform_operation(&self, key_id: &str, operation: CryptoOperation) -> Result<CryptoResult, BackendError>;
    fn supports_operation(&self, operation: &CryptoOperation) -> bool;
    fn get_capabilities(&self) -> BackendCapabilities;
    fn backend_info(&self) -> BackendInfo;
    fn list_keys(&self) -> Result<Vec<KeyMetadata>, BackendError>;
}
```

### Operations to Support (v1)

From `CryptoOperation` enum:

- `Sign { data, algorithm }` — P1, table stakes
- `GetPublicKey` — P1, table stakes
- `Attest { challenge }` — P2, differentiator (defer to v1.x)

### Operations to NOT Support

- `Encrypt/Decrypt` — YubiKey PIV doesn't expose symmetric crypto
- `DeriveKey` — YubiKey stores keys, doesn't derive
- `GenerateKeyPair` — Defer to v2+ (requires management key)
- `KeyExchange` — Not standard PIV operation
- `Verify` — Verification happens in software with public key
- `Hash` — Software operation, use Software HSM backend

## Sources

**Codebase Analysis (HIGH Confidence):**
- `/home/john/vault/projects/github.com/trustedge/crates/core/src/backends/universal.rs` — Universal Backend trait definition, capability system
- `/home/john/vault/projects/github.com/trustedge/crates/core/src/backends/yubikey.rs` — Existing YubiKey implementation (lines 1-200, 1082-1200), identified anti-patterns
- `/home/john/vault/projects/github.com/trustedge/crates/core/src/backends/software_hsm.rs` — Reference implementation showing proper signing/verification patterns
- `/home/john/vault/projects/github.com/trustedge/crates/core/tests/yubikey_strict_hardware.rs` — Strict hardware testing patterns
- `/home/john/vault/projects/github.com/trustedge/crates/core/tests/yubikey_hardware_tests.rs` — Hardware test structure with #[ignore]
- `/home/john/vault/projects/github.com/trustedge/improvement-plan.md` — Security audit identifying manual crypto, software fallbacks, placeholder keys as critical issues
- `/home/john/vault/projects/github.com/trustedge/Cargo.toml` — Workspace dependencies (pkcs11, yubikey, x509-cert, der, spki)

**PIV/YubiKey Standards (MEDIUM Confidence - Training Data):**
- NIST SP 800-73-4: PIV specification defining mandatory algorithms (P-256, RSA-2048), slot purposes (9A/9C/9D/9E), PKCS#11 interface
- YubiKey PIV documentation: Slot mappings (9A=Auth, 9C=Signature, 9D=Key Management, 9E=Card Auth), attestation via slot F9

**Note on Confidence:**
- **HIGH** for codebase-specific features (read directly from source)
- **MEDIUM** for PIV standard operations (based on training data, unable to verify with WebSearch/WebFetch due to permission denial)
- All implementation recommendations use battle-tested libraries from existing Cargo.toml

---

**Research Methodology:**
1. Read Universal Backend trait to understand required operations
2. Analyzed existing YubiKey backend to identify current capabilities and anti-patterns
3. Read Software HSM backend as reference for proper backend implementation
4. Examined test files to understand testing strategy (simulation vs hardware vs strict)
5. Read improvement plan to identify critical issues (software fallbacks, placeholder keys, manual crypto)
6. Cross-referenced workspace dependencies to identify available libraries

**Limitations:**
- WebSearch/WebFetch unavailable (permission denied), relied on training data for PIV standards
- Cannot verify current YubiKey PIV specification (assumed NIST SP 800-73-4 still current)
- Attestation details based on training data, should verify with official Yubico docs during implementation
