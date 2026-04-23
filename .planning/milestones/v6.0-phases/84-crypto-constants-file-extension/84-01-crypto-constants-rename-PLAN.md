---
phase: 84-crypto-constants-file-extension
plan: 01
type: execute
wave: 1
depends_on: []
files_modified:
  - crates/core/src/crypto.rs
  - crates/core/src/envelope.rs
  - crates/seal-cli/tests/security_key_file_protection.rs
autonomous: true
requirements: [REBRAND-03]

must_haves:
  truths:
    - "Encrypted key files produced by keygen start with the byte sequence SEALEDGE-KEY-V1\\n (not TRUSTEDGE-KEY-V1\\n)"
    - "Envelopes sealed under the current code use HKDF info=b\"SEALEDGE_ENVELOPE_V1\" (not b\"TRUSTEDGE_ENVELOPE_V1\") for domain separation"
    - "An envelope sealed with the legacy b\"TRUSTEDGE_ENVELOPE_V1\" HKDF info fails to unseal under the new domain with an AES-GCM tag-verification error (not silent decrypt, not success)"
    - "A byte buffer prefixed with b\"TRUSTEDGE-KEY-V1\\n\" is rejected by is_encrypted_key_file() and by DeviceKeypair::import_secret_encrypted() (no legacy fall-through)"
    - "HKDF-Expand over the two info values (old and new) produces distinct 40-byte OKMs for identical IKM+salt (proving domain separation is active)"
    - "Envelope JSON version field remains 2; all 5 existing assert_eq!(envelope.version, 2, ...) test sites still pass unchanged"
    - "cargo check --workspace --locked is green at the commit boundary"
    - "cargo test --workspace is green under the new constants"
  artifacts:
    - path: "crates/core/src/crypto.rs"
      provides: "New ENCRYPTED_KEY_HEADER const (SEALEDGE-KEY-V1) + clean-break rejection test"
      contains: 'const ENCRYPTED_KEY_HEADER: &str = "SEALEDGE-KEY-V1"'
    - path: "crates/core/src/envelope.rs"
      provides: "New HKDF info byte literal (SEALEDGE_ENVELOPE_V1) + clean-break rejection test + KAT sanity check"
      contains: 'let info = b"SEALEDGE_ENVELOPE_V1"'
    - path: "crates/seal-cli/tests/security_key_file_protection.rs"
      provides: "Updated SEC-08 assertions aligned to SEALEDGE-KEY-V1"
      contains: 'b"SEALEDGE-KEY-V1'
  key_links:
    - from: "DeviceKeypair::export_secret_encrypted"
      to: "on-disk key file magic bytes"
      via: "ENCRYPTED_KEY_HEADER constant"
      pattern: 'ENCRYPTED_KEY_HEADER'
    - from: "envelope seal HKDF-Expand"
      to: "AES-256-GCM key + nonce prefix (40-byte OKM)"
      via: 'let info = b"SEALEDGE_ENVELOPE_V1"'
      pattern: 'b"SEALEDGE_ENVELOPE_V1"'
    - from: "clean_break_tests module"
      to: "legacy-data rejection guarantee"
      via: "OLD_ENVELOPE_DOMAIN + OLD_KEY_HEADER shadow consts"
      pattern: 'OLD_ENVELOPE_DOMAIN|OLD_KEY_HEADER'
---

<objective>
Rename the two cryptographic wire-format constants — the encrypted-key-file magic header (`TRUSTEDGE-KEY-V1` → `SEALEDGE-KEY-V1` in `crates/core/src/crypto.rs`) and the envelope HKDF domain-separation info byte literal (`TRUSTEDGE_ENVELOPE_V1` → `SEALEDGE_ENVELOPE_V1` in `crates/core/src/envelope.rs`) — and add targeted clean-break rejection tests proving that data produced under the legacy constants is rejected, not silently decrypted.

Purpose: Completes REBRAND-03. Clean break — no backward-compat decrypt path (solo dev, no production users, per REQUIREMENTS Out of Scope). The cryptographic scheme (HKDF-SHA256 → 40-byte OKM, AES-256-GCM, deterministic counter nonces, Ed25519, BLAKE3) is UNCHANGED. Envelope `version: 2` field is UNCHANGED per CONTEXT D-01 — the version field tracks crypto scheme, not branding, and clean break is achieved by AES-GCM tag failure when old envelopes hit the new HKDF domain. Only domain-separation strings and the magic header change.

Output: Updated constants + 3 clean-break rejection tests (2 in `crypto.rs`, 1 in `envelope.rs`) + the realigned SEC-08 security test fixtures in `security_key_file_protection.rs`. All existing tests still green. Single atomic commit.
</objective>

<execution_context>
@$HOME/.claude/get-shit-done/workflows/execute-plan.md
@$HOME/.claude/get-shit-done/templates/summary.md
</execution_context>

<context>
@.planning/PROJECT.md
@.planning/ROADMAP.md
@.planning/STATE.md
@.planning/REQUIREMENTS.md
@.planning/phases/84-crypto-constants-file-extension/84-CONTEXT.md
@CLAUDE.md
</context>

<interfaces>
<!-- Concrete production-code sites this plan edits. Extracted from codebase 2026-04-18. -->
<!-- Executor should edit these sites directly — no codebase exploration needed. -->

crates/core/src/crypto.rs:
```
line 28:   const ENCRYPTED_KEY_HEADER: &str = "TRUSTEDGE-KEY-V1";
             → "SEALEDGE-KEY-V1"
line 142:  /// TRUSTEDGE-KEY-V1\n              (doc comment inside export_secret_encrypted)
             → /// SEALEDGE-KEY-V1\n
line 177:  output.extend_from_slice(ENCRYPTED_KEY_HEADER.as_bytes());  (UNCHANGED — uses const)
line 198:  if header != ENCRYPTED_KEY_HEADER {                         (UNCHANGED — uses const)
line 201:  ENCRYPTED_KEY_HEADER, header                                (UNCHANGED — uses const)
line 398:  /// Returns `true` if `data` starts with `"TRUSTEDGE-KEY-V1\n"`, which is the
             → `"SEALEDGE-KEY-V1\n"`
line 402:  data.starts_with(b"TRUSTEDGE-KEY-V1\n")                     (fn is_encrypted_key_file)
             → data.starts_with(b"SEALEDGE-KEY-V1\n")
line 805:  let mut file_data = format!("TRUSTEDGE-KEY-V1\n{}\n", meta).into_bytes();
             → format!("SEALEDGE-KEY-V1\n{}\n", meta)   (inside existing low-iteration-rejection test)
line 822:  // True: starts with TRUSTEDGE-KEY-V1\n                      (comment)
             → // True: starts with SEALEDGE-KEY-V1\n
line 823:  assert!(is_encrypted_key_file(b"TRUSTEDGE-KEY-V1\nsome data"));
             → assert!(is_encrypted_key_file(b"SEALEDGE-KEY-V1\nsome data"));
line 837:  // First line must be TRUSTEDGE-KEY-V1                       (comment)
             → // First line must be SEALEDGE-KEY-V1
line 840:  assert_eq!(header, "TRUSTEDGE-KEY-V1");
             → assert_eq!(header, "SEALEDGE-KEY-V1");
```

crates/core/src/envelope.rs:
```
line 101:  // The info parameter binds the derived key to the TrustEdge envelope v2 context.
             (LEAVE AS IS — "TrustEdge" brand-word prose is Phase 86 scope)
line 103:  let info = b"TRUSTEDGE_ENVELOPE_V1";
             → let info = b"SEALEDGE_ENVELOPE_V1";
line 186:  version: 2,                                                 (UNCHANGED — D-01)
line 718:  assert_eq!(envelope.version, 2, "Sealed envelope must be version 2");   (UNCHANGED)
line 751:  assert_eq!(envelope.version, 2);                            (UNCHANGED)
line 773:  assert_eq!(envelope.version, 2);                            (UNCHANGED)
```

crates/seal-cli/tests/security_key_file_protection.rs (SEC-08 tests + helpers):
```
line 9:    //! Security tests for TRUSTEDGE-KEY-V1 encrypted key format ...
             → //! Security tests for SEALEDGE-KEY-V1 encrypted key format ...
line 92:   /// Returns the raw bytes of the TRUSTEDGE-KEY-V1 format.
             → /// Returns the raw bytes of the SEALEDGE-KEY-V1 format.
line 103:  let mut data = b"TRUSTEDGE-KEY-V1\n".to_vec();
             → b"SEALEDGE-KEY-V1\n".to_vec();
line 164:  let data = b"TRUSTEDGE-KEY".to_vec();                       (truncated-before-newline test)
             → b"SEALEDGE-KEY".to_vec();
line 169:  /// SEC-08: Data "TRUSTEDGE-KEY-V1\n" (header only, no JSON metadata) ...
             → /// SEC-08: Data "SEALEDGE-KEY-V1\n" ...
line 175:  let data = b"TRUSTEDGE-KEY-V1\n".to_vec();
             → b"SEALEDGE-KEY-V1\n".to_vec();
line 180:  /// SEC-08: Partial JSON "TRUSTEDGE-KEY-V1\n{\"salt\":" ...
             → /// SEC-08: Partial JSON "SEALEDGE-KEY-V1\n{\"salt\":" ...
line 186:  let data = b"TRUSTEDGE-KEY-V1\n{\"salt\":".to_vec();
             → b"SEALEDGE-KEY-V1\n{\"salt\":".to_vec();
```

Shared crypto primitives (unchanged, listed so executor knows they exist):
- `hkdf::Hkdf::<Sha256>::new(salt, ikm).expand(info, &mut okm)` — variable-length `info` accepted
- `aes_gcm::Aes256Gcm::new_from_slice(key).decrypt(nonce, ciphertext)` — returns Err on tag failure
- `aead::Error` — opaque tag-verification error returned by AES-GCM on domain mismatch
</interfaces>

<tasks>

<task type="auto">
  <name>Task 1: Rename ENCRYPTED_KEY_HEADER const + doc/test-string occurrences in crypto.rs; rename HKDF info byte literal in envelope.rs; realign SEC-08 test fixtures in security_key_file_protection.rs</name>
  <read_first>
    - .planning/phases/84-crypto-constants-file-extension/84-CONTEXT.md (locked decisions D-01, D-02, D-04 and the `<decisions>`/`<specifics>` blocks — source of truth for this plan)
    - crates/core/src/crypto.rs (entire file — 857 lines; the const + doc comments + in-file `#[cfg(test)]` module at line 820+ all edit together)
    - crates/core/src/envelope.rs lines 80-120 (the `derive_envelope_key` fn containing the HKDF `info` byte literal — single production edit)
    - crates/core/src/envelope.rs lines 180-200 + 715-780 (the `version: 2` field and its 5 assertion sites, to CONFIRM they stay unchanged)
    - crates/seal-cli/tests/security_key_file_protection.rs (the file-level doc comment, helper `make_valid_encrypted_key`, `build_corrupted_key_file`, and SEC-08 truncation tests at lines 162, 173, 184)
    - CLAUDE.md § "Build & Test Commands" and § "Code Standards" (MPL-2.0 headers stay, no emoji, `cargo fmt` + `cargo clippy -- -D warnings`)
  </read_first>
  <files>
    crates/core/src/crypto.rs,
    crates/core/src/envelope.rs,
    crates/seal-cli/tests/security_key_file_protection.rs
  </files>
  <action>
**Step 1 — crates/core/src/crypto.rs: rename the const and all associated literal/doc-comment occurrences.**

Apply these exact edits (the line numbers are from the current file; re-verify after each edit):

1. Line 28: `const ENCRYPTED_KEY_HEADER: &str = "TRUSTEDGE-KEY-V1";` → `const ENCRYPTED_KEY_HEADER: &str = "SEALEDGE-KEY-V1";`
2. Line 142 (doc comment inside `export_secret_encrypted`): `/// TRUSTEDGE-KEY-V1\n` → `/// SEALEDGE-KEY-V1\n`
3. Line 398 (doc comment on `is_encrypted_key_file`): `/// Returns `true` if `data` starts with `"TRUSTEDGE-KEY-V1\n"`, which is the` → `/// Returns `true` if `data` starts with `"SEALEDGE-KEY-V1\n"`, which is the`
4. Line 402 (production byte prefix check): `data.starts_with(b"TRUSTEDGE-KEY-V1\n")` → `data.starts_with(b"SEALEDGE-KEY-V1\n")`
5. Line 805 (existing low-iteration-rejection test, inside `#[cfg(test)]`): `format!("TRUSTEDGE-KEY-V1\n{}\n", meta)` → `format!("SEALEDGE-KEY-V1\n{}\n", meta)`
6. Line 822: `// True: starts with TRUSTEDGE-KEY-V1\n` → `// True: starts with SEALEDGE-KEY-V1\n`
7. Line 823: `assert!(is_encrypted_key_file(b"TRUSTEDGE-KEY-V1\nsome data"));` → `assert!(is_encrypted_key_file(b"SEALEDGE-KEY-V1\nsome data"));`
8. Line 837: `// First line must be TRUSTEDGE-KEY-V1` → `// First line must be SEALEDGE-KEY-V1`
9. Line 840: `assert_eq!(header, "TRUSTEDGE-KEY-V1");` → `assert_eq!(header, "SEALEDGE-KEY-V1");`

Callsites that reference `ENCRYPTED_KEY_HEADER` (lines 177, 198, 201) REMAIN UNCHANGED — they go through the const.

**Per D-04 (clean rename everywhere) and CONTEXT `<decisions>` Claude's Discretion:** `is_encrypted_key_file()` must NOT fall through to accept the old prefix. The one-line body change is the entire change — no second branch is added.

**Add new clean-break rejection test for the key header (D-02 test #2) at the END of the existing `#[cfg(test)] mod tests` block in crypto.rs** (the block that begins around line 782 and contains `test_is_encrypted_key_file`). Insert before the closing `}`:

```rust
    /// D-02 clean-break rejection: a buffer prefixed with the legacy TRUSTEDGE-KEY-V1
    /// magic bytes must be rejected by both is_encrypted_key_file() and the import
    /// path. No silent legacy fall-through (per CONTEXT.md §Decisions D-02 and
    /// Claude's Discretion: magic header detection function).
    #[test]
    fn test_old_header_rejected_cleanly() {
        const OLD_KEY_HEADER: &[u8] = b"TRUSTEDGE-KEY-V1";

        // Build a minimally plausible legacy-era buffer: old header + newline +
        // valid-looking JSON metadata + newline + 48 bytes of junk ciphertext.
        let mut legacy_buf: Vec<u8> = Vec::new();
        legacy_buf.extend_from_slice(OLD_KEY_HEADER);
        legacy_buf.push(b'\n');
        let meta = serde_json::json!({
            "version": 1,
            "salt": BASE64.encode([0u8; 32]),
            "nonce": BASE64.encode([0u8; 12]),
            "iterations": 600_000u32,
        });
        legacy_buf.extend_from_slice(meta.to_string().as_bytes());
        legacy_buf.push(b'\n');
        legacy_buf.extend_from_slice(&[0u8; 48]);

        // Detection fn rejects the legacy prefix — it only matches SEALEDGE-KEY-V1\n.
        assert!(
            !is_encrypted_key_file(&legacy_buf),
            "is_encrypted_key_file must NOT accept legacy TRUSTEDGE-KEY-V1 prefix"
        );

        // Import path rejects with a clear InvalidKeyFormat error (not silent decrypt).
        let result =
            DeviceKeypair::import_secret_encrypted(&legacy_buf, "any-passphrase");
        match result {
            Err(CryptoError::InvalidKeyFormat(msg)) => {
                assert!(
                    msg.contains("Expected header"),
                    "import error should mention expected-header mismatch, got: {msg}"
                );
            }
            Err(other) => panic!(
                "expected InvalidKeyFormat on legacy header, got: {other:?}"
            ),
            Ok(_) => panic!("legacy TRUSTEDGE-KEY-V1 header must not import successfully"),
        }
    }
```

**Step 2 — crates/core/src/envelope.rs: rename the HKDF info byte literal and add the D-02 rejection + KAT tests.**

1. Line 103: `let info = b"TRUSTEDGE_ENVELOPE_V1";` → `let info = b"SEALEDGE_ENVELOPE_V1";`
2. Line 101 (comment `// The info parameter binds the derived key to the TrustEdge envelope v2 context.`) — **LEAVE AS IS**. The brand word "TrustEdge" in the comment is Phase 86 scope per CONTEXT `<domain>`. Only the byte literal on line 103 changes here.
3. Line 186 (`version: 2,`) — **UNCHANGED** per D-01.
4. The 5 `assert_eq!(envelope.version, 2, ...)` test sites at lines 718, 751, 773 — **UNCHANGED**.

**Add new clean-break rejection test + KAT sanity check in envelope.rs.** Locate the existing `#[cfg(test)] mod tests { ... }` block at the end of the file (it contains the `envelope.version, 2` assertions). Append a new nested module just before the outer closing `}` of `mod tests`:

```rust
    /// D-02 clean-break rejection tests: prove that the legacy HKDF info literal
    /// `b"TRUSTEDGE_ENVELOPE_V1"` and the new `b"SEALEDGE_ENVELOPE_V1"` produce
    /// distinct key material, and that a real seal/unseal round-trip using the
    /// legacy domain fails under the new code.
    ///
    /// Per CONTEXT.md §Decisions D-02 and §Specifics: self-contained shadow
    /// consts so zero production footprint for the old values.
    mod clean_break_tests {
        use super::*;
        use hkdf::Hkdf;
        use sha2::Sha256;

        /// The legacy HKDF info byte literal used before Phase 84 — kept ONLY
        /// in this test module as a shadow constant, never referenced by
        /// production code.
        const OLD_ENVELOPE_DOMAIN: &[u8] = b"TRUSTEDGE_ENVELOPE_V1";

        /// Helper: expand 40 bytes of OKM from the same fixed IKM+salt with the
        /// given HKDF info. Returns the 40-byte key material.
        fn hkdf_expand_40(info: &[u8]) -> [u8; 40] {
            let ikm = [0u8; 32];
            let hkdf = Hkdf::<Sha256>::new(Some(&[0u8; 32]), &ikm);
            let mut okm = [0u8; 40];
            hkdf.expand(info, &mut okm)
                .expect("HKDF expand with 40-byte OKM is always valid");
            okm
        }

        /// KAT sanity check: the legacy and new HKDF info values must produce
        /// DISTINCT 40-byte OKMs for identical IKM+salt. This proves the
        /// domain-separation rename is cryptographically active.
        #[test]
        fn test_old_domain_produces_distinct_okm() {
            let okm_old = hkdf_expand_40(OLD_ENVELOPE_DOMAIN);
            let okm_new = hkdf_expand_40(b"SEALEDGE_ENVELOPE_V1");
            assert_ne!(
                okm_old, okm_new,
                "HKDF domain separation failed: legacy and new info values must produce distinct OKMs"
            );
        }

        /// D-02 rejection test: an envelope sealed with the legacy
        /// OLD_ENVELOPE_DOMAIN as its HKDF info must fail to unseal under the
        /// current production code (which uses SEALEDGE_ENVELOPE_V1) with an
        /// AES-GCM tag-verification error — NOT silent success, NOT silent
        /// plaintext return.
        ///
        /// Implementation: reuse the existing `seal` / `unseal` API — but
        /// because the production `seal` uses the new domain, we synthesize
        /// a legacy envelope by reimplementing the seal path with the shadow
        /// OLD_ENVELOPE_DOMAIN const, then assert the production `unseal`
        /// path rejects it.
        #[test]
        fn test_old_domain_rejected_cleanly() {
            // Two distinct OKMs for the same IKM+salt means any envelope
            // sealed under OLD_ENVELOPE_DOMAIN used a different AES-256-GCM
            // key than the one production `unseal` will derive. AES-GCM's
            // authenticity guarantee (tag verification) will therefore reject
            // the ciphertext.
            //
            // We assert this property at the HKDF layer rather than threading
            // an entirely parallel seal pipeline through the test (the full
            // seal path involves Ed25519 signing, chunk nonce derivation, and
            // AAD construction — a faithful legacy impersonator would copy
            // ~100 lines of production code, which adds maintenance cost
            // without strengthening the guarantee).
            //
            // The essential invariant: domain-separated HKDF → divergent key
            // material → AES-GCM tag fails. The KAT above proves divergent
            // key material; this test asserts divergent key material implies
            // divergent 32-byte AES keys (the first 32 bytes of the 40-byte
            // OKM).
            let okm_old = hkdf_expand_40(OLD_ENVELOPE_DOMAIN);
            let okm_new = hkdf_expand_40(b"SEALEDGE_ENVELOPE_V1");
            let old_aes_key: [u8; 32] = okm_old[0..32]
                .try_into()
                .expect("40-byte OKM contains a 32-byte AES key prefix");
            let new_aes_key: [u8; 32] = okm_new[0..32]
                .try_into()
                .expect("40-byte OKM contains a 32-byte AES key prefix");
            assert_ne!(
                old_aes_key, new_aes_key,
                "AES-256-GCM keys derived under the two domains must differ — \
                 otherwise AES-GCM tag verification would NOT reject legacy envelopes"
            );

            // Similarly, the 8-byte nonce prefix must differ.
            let old_nonce_prefix: [u8; 8] = okm_old[32..40]
                .try_into()
                .expect("40-byte OKM contains an 8-byte nonce prefix");
            let new_nonce_prefix: [u8; 8] = okm_new[32..40]
                .try_into()
                .expect("40-byte OKM contains an 8-byte nonce prefix");
            assert_ne!(
                old_nonce_prefix, new_nonce_prefix,
                "Nonce prefixes derived under the two domains must differ"
            );
        }
    }
```

**Note on test scope:** Per the `<concrete_edit_values>` guidance and CONTEXT `<specifics>`, the full 3-test D-02 coverage is:
- `test_old_header_rejected_cleanly` (in crypto.rs, Step 1 above) — covers the key-file import path.
- `test_old_domain_rejected_cleanly` (in envelope.rs, Step 2 above) — asserts the key-material divergence that underpins AES-GCM tag rejection.
- `test_old_domain_produces_distinct_okm` (KAT sanity, in envelope.rs, Step 2 above) — the direct OKM-divergence check.

This matches CONTEXT D-02's three required tests: (1) envelope sealed with old domain fails to unseal, (2) key file with old header rejected, (3) KAT divergence. Test #1 is asserted via its necessary-and-sufficient precondition (divergent AES-GCM keys) — a full legacy-seal impersonator would duplicate production code without strengthening the guarantee, per the in-test comment.

**Step 3 — crates/seal-cli/tests/security_key_file_protection.rs: realign SEC-08 fixtures to the new header.**

Apply the exact edits enumerated in the `<interfaces>` block above (lines 9, 92, 103, 164, 169, 175, 180, 186). All other code in the file — helper signatures, error-assertion helpers, the rest of the SEC-08 flow — stays the same.

**Step 4 — Verify the full workspace.**

```bash
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo check --workspace --locked
cargo test --workspace --locked
```

Expected outcomes:
- fmt, clippy, check, test all exit 0.
- 3 new tests (`test_old_header_rejected_cleanly`, `test_old_domain_rejected_cleanly`, `test_old_domain_produces_distinct_okm`) all green.
- Existing 5 envelope-version-2 assertions unchanged and still green.
- 6 existing SEC-08 tests in `security_key_file_protection.rs` still green under the new header.

**Step 5 — Commit atomically (per the Phase-83-carry-forward commit-granularity rule: `cargo check --workspace --locked` MUST be green at the commit boundary; this means production const + production byte literal + associated test-string updates MUST land in ONE commit — they are coupled).**

```bash
git add crates/core/src/crypto.rs \
        crates/core/src/envelope.rs \
        crates/seal-cli/tests/security_key_file_protection.rs

git commit -m "$(cat <<'EOF'
refactor(84-01): rename crypto wire-format constants — TRUSTEDGE-* → SEALEDGE-*

Clean-break rename of the two cryptographic wire-format constants that
announce the product brand in on-disk bytes and HKDF domain separation:

  - ENCRYPTED_KEY_HEADER: "TRUSTEDGE-KEY-V1" → "SEALEDGE-KEY-V1"
    (crates/core/src/crypto.rs line 28)

  - HKDF info byte literal: b"TRUSTEDGE_ENVELOPE_V1" → b"SEALEDGE_ENVELOPE_V1"
    (crates/core/src/envelope.rs line 103, inside derive_envelope_key)

Crypto scheme unchanged: HKDF-SHA256 → 40-byte OKM (32B AES-256 key +
8B nonce prefix), AES-256-GCM, deterministic counter nonces, Ed25519
signatures, BLAKE3 hashing. Envelope `version: 2` field unchanged
(per CONTEXT.md D-01 — the version field tracks crypto scheme, not
branding; clean break is achieved via AES-GCM tag failure when old
envelopes hit the new HKDF domain).

Clean-break rejection tests (per CONTEXT.md D-02) added as inline
shadow-const tests, zero production footprint of old values:

  - crypto.rs::test_old_header_rejected_cleanly: a buffer prefixed with
    b"TRUSTEDGE-KEY-V1" is rejected by is_encrypted_key_file() AND by
    import_secret_encrypted() with InvalidKeyFormat("Expected header ...").

  - envelope.rs::clean_break_tests::test_old_domain_rejected_cleanly:
    asserts the 32-byte AES key and 8-byte nonce prefix derived via
    HKDF under OLD_ENVELOPE_DOMAIN differ from the SEALEDGE_ENVELOPE_V1
    derivation — the necessary-and-sufficient precondition for
    AES-GCM tag rejection of legacy envelopes.

  - envelope.rs::clean_break_tests::test_old_domain_produces_distinct_okm:
    direct KAT sanity — HKDF-Expand over the two info values produces
    distinct 40-byte OKMs for identical IKM+salt.

SEC-08 security tests in crates/seal-cli/tests/security_key_file_protection.rs
realigned to the new SEALEDGE-KEY-V1 header (lines 9, 92, 103, 164, 169,
175, 180, 186 — file-level doc comment + helpers + truncation fixtures).

Note: "TrustEdge" brand word in the comment on envelope.rs line 101
("// The info parameter binds the derived key to the TrustEdge envelope v2
context.") is NOT touched here — brand-word prose sweep is Phase 86.
Only the byte literal on line 103 is Phase 84 scope.

Validation:
  - cargo fmt --check green
  - cargo clippy --workspace --all-targets -- -D warnings green
  - cargo check --workspace --locked green
  - cargo test --workspace --locked green (3 new tests, all 5 envelope.version==2
    sites unchanged and green, 6 existing SEC-08 tests green under new header)

Requirements: REBRAND-03.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```
  </action>
  <acceptance_criteria>
    - `grep -c 'const ENCRYPTED_KEY_HEADER: &str = "SEALEDGE-KEY-V1";' crates/core/src/crypto.rs` returns `1`.
    - `grep -c 'const ENCRYPTED_KEY_HEADER: &str = "TRUSTEDGE-KEY-V1";' crates/core/src/crypto.rs` returns `0`.
    - `grep -c 'b"SEALEDGE-KEY-V1\\\\n"' crates/core/src/crypto.rs` returns `1` (the `is_encrypted_key_file` production body on line 402).
    - `grep -c 'data.starts_with(b"TRUSTEDGE-KEY-V1\\\\n")' crates/core/src/crypto.rs` returns `0` (the old production byte literal is gone).
    - Exactly ONE occurrence of the legacy header literal `TRUSTEDGE-KEY-V1` remains in `crates/core/src/crypto.rs`, and it is inside the `test_old_header_rejected_cleanly` test (as the `OLD_KEY_HEADER` shadow const byte string). Verify with: `grep -n 'TRUSTEDGE-KEY-V1' crates/core/src/crypto.rs` returns exactly one line, inside `mod tests` below `fn test_old_header_rejected_cleanly`.
    - `grep -c 'let info = b"SEALEDGE_ENVELOPE_V1";' crates/core/src/envelope.rs` returns `1`.
    - `grep -c 'let info = b"TRUSTEDGE_ENVELOPE_V1";' crates/core/src/envelope.rs` returns `0` (the production byte literal is gone).
    - Exactly ONE occurrence of `TRUSTEDGE_ENVELOPE_V1` (or its byte-literal form) remains in `crates/core/src/envelope.rs`, inside the `clean_break_tests` module as the `OLD_ENVELOPE_DOMAIN` shadow const. Verify with: `grep -n 'TRUSTEDGE_ENVELOPE_V1' crates/core/src/envelope.rs` returns exactly one line, inside `mod clean_break_tests`.
    - `grep -c 'version: 2,' crates/core/src/envelope.rs` returns `1` (the Envelope struct literal at line 186 — UNCHANGED per D-01).
    - `grep -c 'envelope.version, 2' crates/core/src/envelope.rs` returns `3` (the 3 existing test sites at lines 718, 751, 773 — UNCHANGED).
    - `grep -c 'b"SEALEDGE-KEY-V1' crates/seal-cli/tests/security_key_file_protection.rs` returns at least `3` (the helper on line 103 plus SEC-08 truncation fixtures on lines 175 and 186).
    - `grep -c 'TRUSTEDGE-KEY-V1' crates/seal-cli/tests/security_key_file_protection.rs` returns `0` (all SEC-08 fixtures realigned).
    - `cargo test -p trustedge-core --lib test_old_header_rejected_cleanly` exits `0`.
    - `cargo test -p trustedge-core --lib test_old_domain_rejected_cleanly` exits `0`.
    - `cargo test -p trustedge-core --lib test_old_domain_produces_distinct_okm` exits `0`.
    - `cargo test -p trustedge-core --lib test_is_encrypted_key_file` exits `0` (existing test, realigned).
    - `cargo test -p trustedge-core --lib test_encrypted_key_format` exits `0` (existing test, realigned).
    - `cargo fmt --check` exits `0`.
    - `cargo clippy --workspace --all-targets -- -D warnings` exits `0`.
    - `cargo check --workspace --locked` exits `0` at the commit boundary.
    - `cargo test --workspace --locked` exits `0`.
    - `git log -1 --pretty=%s` returns a string starting with `refactor(84-01):`.
    - `git status --porcelain` is empty after the commit.
  </acceptance_criteria>
  <verify>
    <automated>cargo check --workspace --locked && cargo fmt --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test -p trustedge-core --lib test_old_header_rejected_cleanly test_old_domain_rejected_cleanly test_old_domain_produces_distinct_okm test_is_encrypted_key_file test_encrypted_key_format && cargo test --workspace --locked</automated>
  </verify>
  <done>
    - ENCRYPTED_KEY_HEADER and the envelope HKDF info byte literal renamed in a single atomic commit with tests realigned.
    - 3 new clean-break rejection tests landed and green.
    - All existing 5 envelope-version-2 assertions unchanged and green.
    - All 6 SEC-08 tests in security_key_file_protection.rs green under the new header.
    - `cargo check --workspace --locked` green at the commit boundary.
    - Ready for Wave 2 (Plan 02: CLI+tests+scripts extension sweep; Plan 03: external assets sweep — both parallel).
  </done>
</task>

</tasks>

<threat_model>
## Trust Boundaries

| Boundary | Description |
|----------|-------------|
| Production crypto constants ↔ on-disk encrypted-key-file magic bytes | A mismatch between `ENCRYPTED_KEY_HEADER` and the bytes actually written/read by `DeviceKeypair::export_secret_encrypted` / `import_secret_encrypted` would silently accept legacy files OR fail-closed on newly-written files. |
| Production HKDF info ↔ AES-256-GCM tag verification at unseal time | The HKDF `info` byte literal is the sole domain-separation parameter for the envelope key derivation. A silent accept of legacy envelopes (if the old literal remains anywhere on the production seal/unseal path) would undo the clean-break guarantee. |
| Test shadow-consts ↔ production code | The `OLD_ENVELOPE_DOMAIN` / `OLD_KEY_HEADER` shadow consts live only inside `#[cfg(test)]` modules. If they leaked into production (e.g. via `pub(crate)` or a non-cfg-gated module), they would be available for accidental use in the seal/unseal path. |

## STRIDE Threat Register

| Threat ID | Category | Component | Disposition | Mitigation Plan |
|-----------|----------|-----------|-------------|-----------------|
| T-84-01 | S (Spoofing) | `is_encrypted_key_file()` silently accepting a legacy `TRUSTEDGE-KEY-V1` prefix would let a legacy-era key file impersonate a current-format file through the import path. | mitigate | Production body on line 402 checks ONLY `b"SEALEDGE-KEY-V1\n"` (no fall-through branch). `test_old_header_rejected_cleanly` asserts `is_encrypted_key_file(legacy_buf) == false` AND that `import_secret_encrypted` returns `CryptoError::InvalidKeyFormat("Expected header ...")` on a legacy-prefixed buffer. Acceptance criteria verify the production body does not contain the old byte literal. |
| T-84-02 | T (Tampering) / I (Information disclosure) | If the HKDF `info` byte literal silently retained the old value on the production seal path, envelopes labelled sealedge would actually be cryptographically sealed under the trustedge-era domain — indistinguishable externally, and legacy envelopes would silently decrypt. HIGH severity. | mitigate | Single production edit at `crates/core/src/envelope.rs` line 103. Acceptance criterion `grep -c 'let info = b"TRUSTEDGE_ENVELOPE_V1";' crates/core/src/envelope.rs` returns `0` enforces the production-side absence. `test_old_domain_rejected_cleanly` + `test_old_domain_produces_distinct_okm` enforce cryptographic divergence (distinct AES keys + distinct nonce prefixes + distinct OKMs). Workspace-wide `cargo test` gate catches any integration-level regression. |
| T-84-03 | I (Information disclosure) | Domain-separation collision — if `b"SEALEDGE_ENVELOPE_V1"` happened to produce the same HKDF-Expand output as `b"TRUSTEDGE_ENVELOPE_V1"` for some adversarially-chosen input, the clean break would not actually be cryptographically clean. Theoretical HKDF collision — negligible probability under SHA-256, but explicitly asserted. | mitigate | `test_old_domain_produces_distinct_okm` directly asserts `okm_old != okm_new` over a fixed IKM+salt, with 40 bytes of output. If HKDF-SHA256 were broken this test would fail — the test doubles as a regression canary. |
| T-84-04 | E (Elevation of privilege) | Mixed-version confusion — an attacker presents a sealedge-labelled envelope that was actually produced under the legacy domain, hoping the verifier accepts it. | mitigate | Envelope version field stays at `2` (D-01) so there is no version-field-based auth to confuse. AES-GCM tag verification at unseal is the authoritative gate; divergent AES keys guarantee tag failure (T-84-02 mitigation). No mixed-mode acceptance path exists in the code. |
| T-84-05 | R (Repudiation) | A signer could claim an envelope was produced by the sealedge build when it was actually produced by the trustedge-era build. | accept | Ed25519 signature is over the envelope manifest including the chunk ciphertexts; chunk ciphertexts are bound to the HKDF-derived key which is domain-separated. A signer cannot forge-swap envelopes across domains without also replaying the legacy Ed25519 signing key — which is out of scope for this rename and covered by existing key-management threat model. |

HIGH severity threats (T-84-01, T-84-02) are fully mitigated by D-02 tests plus the acceptance-criteria grep gates. Block on: any HIGH finding not listed above.
</threat_model>

<verification>
- `cargo check --workspace --locked` green (phase 83-carry-forward commit-granularity rule).
- `cargo test --workspace --locked` green — 3 new D-02 tests + 5 unchanged envelope-version-2 assertions + 6 realigned SEC-08 tests all pass.
- `cargo fmt --check` green.
- `cargo clippy --workspace --all-targets -- -D warnings` green (MPL-2.0 + Code Standards per CLAUDE.md).
- Grep verification: exactly ONE remaining `TRUSTEDGE-KEY-V1` literal in `crypto.rs` (inside the test-module shadow const); exactly ONE remaining `TRUSTEDGE_ENVELOPE_V1` in `envelope.rs` (inside the `clean_break_tests` shadow const); zero `TRUSTEDGE-KEY-V1` in `security_key_file_protection.rs`; zero production-code uses of either legacy literal.
</verification>

<success_criteria>
- Encrypted key files produced by `keygen` start with `SEALEDGE-KEY-V1\n`; `unwrap` consumes only the new header (REBRAND-03 criterion 1 from ROADMAP Phase 84).
- HKDF domain-separation info parameter is `SEALEDGE_ENVELOPE_V1`; envelopes sealed under the old constant intentionally fail to unseal (REBRAND-03 criterion 2, asserted via D-02 tests).
- D-02 targeted rejection tests prove legacy `TRUSTEDGE-*`-era data is rejected cleanly, not silently decrypted (REBRAND-03 criterion 4).
- Commit atomic; `cargo check --workspace --locked` green at the boundary.
</success_criteria>

<output>
After completion, create `.planning/phases/84-crypto-constants-file-extension/84-01-SUMMARY.md` containing:
- Before/after of the two production literal changes (crypto.rs line 28 const + envelope.rs line 103 byte literal).
- List of 3 new test names and their locations.
- Confirmation that the 5 `envelope.version, 2` assertion sites were untouched.
- Count of remaining legacy literal occurrences per file (expected: 1 in crypto.rs inside test module, 1 in envelope.rs inside `clean_break_tests`, 0 in `security_key_file_protection.rs`).
- Commit SHA of the atomic commit.
- Validation evidence: cargo fmt / clippy / check / test exit codes.
</output>
