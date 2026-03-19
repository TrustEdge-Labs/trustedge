# Phase 47: Key Protection at Rest - Context

**Gathered:** 2026-03-18
**Status:** Ready for planning

<domain>
## Phase Boundary

Encrypt device private key files at rest with a passphrase. `trst keygen` prompts for passphrase and writes encrypted key files. `trst wrap` and `trst unwrap` prompt for passphrase to decrypt key before use. Plaintext keys are rejected by default; `--unencrypted` flag provides CI/automation escape hatch.

</domain>

<decisions>
## Implementation Decisions

### Encrypted Key File Format
- Custom header format: `TRUSTEDGE-KEY-V1\n` magic bytes
- JSON metadata line: `{"salt":"<base64>","nonce":"<base64>","iterations":600000}`
- Followed by AES-256-GCM encrypted key bytes
- PBKDF2-SHA256 derives encryption key from passphrase (600k iterations, per OWASP 2023 + Phase 46 enforcement)
- Self-contained: salt, nonce, and iteration count stored in the file
- Detection: file starting with `TRUSTEDGE-KEY-V1` = encrypted, file starting with `ed25519:` = plaintext

### Passphrase UX
- **keygen**: Prompt for passphrase + confirm ("Confirm passphrase:"), reject on mismatch. Uses rpassword (no echo).
- **wrap/unwrap**: Single passphrase prompt (no confirm — key already exists, wrong passphrase just fails to decrypt)
- No passphrase stored anywhere — must be entered each time

### --unencrypted Escape Hatch
- `trst keygen --unencrypted`: writes plaintext `ed25519:BASE64` key (current behavior)
- `trst wrap --unencrypted` / `trst unwrap --unencrypted`: accepts plaintext key files without prompting
- Without `--unencrypted`: plaintext key file → error with message "Key file is not encrypted. Use --unencrypted to bypass."
- Default behavior is secure (encrypted required); insecure is opt-in

### Demo Script Update
- `scripts/demo.sh` must use `--unencrypted` since it runs non-interactively
- Acceptance tests must use `--unencrypted` for non-interactive test runs

### Claude's Discretion
- Whether to add encrypt/decrypt helpers to DeviceKeypair or keep them in CLI code
- AES-256-GCM nonce size (12 bytes standard)
- How to handle the passphrase confirmation mismatch error message
- Whether the public key file (.pub) format changes (it shouldn't — public keys are not secret)

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Key handling code
- `crates/core/src/crypto.rs` — DeviceKeypair, export_secret(), import_secret() (current plaintext format)
- `crates/trst-cli/src/main.rs` — handle_keygen() at line ~316, handle_wrap() key loading, handle_unwrap() key loading

### Dependencies already in workspace
- `rpassword` — already in trst-cli (from Phase 44 YubiKey PIN)
- `aes-gcm` — already in workspace (envelope encryption)
- `pbkdf2` — already in workspace (keyring backend)

### Requirements
- `.planning/REQUIREMENTS.md` — KEY-01, KEY-02, KEY-03

### Demo script
- `scripts/demo.sh` — needs `--unencrypted` flag added to keygen/wrap/unwrap calls

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `rpassword::prompt_password()` — already used for YubiKey PIN in handle_wrap()
- `aes_gcm::Aes256Gcm` — already in workspace for envelope encryption
- `pbkdf2::pbkdf2_hmac::<Sha256>()` — already in keyring backend with 600k iterations
- `DeviceKeypair::export_secret()` / `import_secret()` — current plaintext format (need encrypted variants)
- `base64_encode()` / `base64_decode()` — already in crypto.rs

### Established Patterns
- Key format uses `ed25519:` prefix for plaintext — encrypted format uses `TRUSTEDGE-KEY-V1` header
- rpassword used without echo for sensitive input
- PBKDF2 at 600k iterations enforced by Phase 46

### Integration Points
- `crates/core/src/crypto.rs` — Add encrypted export/import functions to DeviceKeypair
- `crates/trst-cli/src/main.rs` — Update handle_keygen(), handle_wrap(), handle_unwrap() for passphrase + --unencrypted flag
- `crates/trst-cli/tests/acceptance.rs` — All existing tests need `--unencrypted` flag
- `scripts/demo.sh` — Add `--unencrypted` to keygen/wrap/unwrap calls

</code_context>

<specifics>
## Specific Ideas

- The previews from discussion show the exact CLI experience users should see
- Public key files (.pub) are NOT encrypted — only the private key file
- Detection is simple: read first line, check for `TRUSTEDGE-KEY-V1` vs `ed25519:`
- PBKDF2 salt should be 32 bytes random (per Phase 46 OWASP standards)

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope.

</deferred>

---

*Phase: 47-key-protection-at-rest*
*Context gathered: 2026-03-18*
