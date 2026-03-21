//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//

//! Security tests for TRUSTEDGE-KEY-V1 encrypted key format — covers threat model T3 (key protection).
//!
//! Tests are organized by requirement:
//!   SEC-08: Truncated encrypted key files are rejected with explicit errors (no panic, no partial key)
//!   SEC-09: Corrupted JSON headers in encrypted key files produce clear parse errors
//!   SEC-10: Wrong passphrase on a valid encrypted key file returns a clear authentication error

use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use trustedge_core::{CryptoError, DeviceKeypair};

// ---------------------------------------------------------------------------
// Test helpers
// ---------------------------------------------------------------------------

/// Generate a valid encrypted key file using a known passphrase.
/// Returns the raw bytes of the TRUSTEDGE-KEY-V1 format.
fn make_valid_encrypted_key() -> Vec<u8> {
    let keypair = DeviceKeypair::generate().expect("generate must succeed");
    keypair
        .export_secret_encrypted("test-passphrase-123")
        .expect("export_secret_encrypted must succeed")
}

/// Build a syntactically valid metadata JSON line with correct base64 fields
/// but allow overriding individual fields for targeted corruption tests.
fn build_corrupted_key_file(meta_json: &str, ciphertext: &[u8]) -> Vec<u8> {
    let mut data = b"TRUSTEDGE-KEY-V1\n".to_vec();
    data.extend_from_slice(meta_json.as_bytes());
    data.push(b'\n');
    data.extend_from_slice(ciphertext);
    data
}

/// Assert that the error is `CryptoError::InvalidKeyFormat` containing the expected substring.
fn assert_invalid_key_format(result: Result<DeviceKeypair, CryptoError>, expected_substr: &str) {
    match result {
        Err(CryptoError::InvalidKeyFormat(msg)) => {
            assert!(
                msg.contains(expected_substr),
                "expected error message to contain {:?}, got: {:?}",
                expected_substr,
                msg
            );
        }
        Err(other) => panic!(
            "expected InvalidKeyFormat({:?}), got: {:?}",
            expected_substr, other
        ),
        Ok(_) => panic!(
            "expected Err(InvalidKeyFormat({:?})), got Ok",
            expected_substr
        ),
    }
}

/// Assert that the error is `CryptoError::DecryptionFailed` containing the expected substring.
fn assert_decryption_failed(result: Result<DeviceKeypair, CryptoError>, expected_substr: &str) {
    match result {
        Err(CryptoError::DecryptionFailed(msg)) => {
            assert!(
                msg.contains(expected_substr),
                "expected error message to contain {:?}, got: {:?}",
                expected_substr,
                msg
            );
        }
        Err(other) => panic!(
            "expected DecryptionFailed({:?}), got: {:?}",
            expected_substr, other
        ),
        Ok(_) => panic!(
            "expected Err(DecryptionFailed({:?})), got Ok",
            expected_substr
        ),
    }
}

// ---------------------------------------------------------------------------
// SEC-08: Truncated files
// ---------------------------------------------------------------------------

/// SEC-08: Data "TRUSTEDGE-KEY" (no newline) is rejected with "Missing header line".
///
/// The format requires a newline-terminated header as its first structural element.
/// A file that ends before the first newline cannot be parsed and must be rejected.
#[test]
fn sec_08_truncated_before_header_newline() {
    let data = b"TRUSTEDGE-KEY".to_vec();
    let result = DeviceKeypair::import_secret_encrypted(&data, "any-passphrase");
    assert_invalid_key_format(result, "Missing header line");
}

/// SEC-08: Data "TRUSTEDGE-KEY-V1\n" (header only, no JSON metadata) is rejected with "Missing metadata line".
///
/// After parsing the header the parser expects a second newline-terminated line
/// containing the JSON metadata. A file that ends after the header newline must be rejected.
#[test]
fn sec_08_truncated_after_header() {
    let data = b"TRUSTEDGE-KEY-V1\n".to_vec();
    let result = DeviceKeypair::import_secret_encrypted(&data, "any-passphrase");
    assert_invalid_key_format(result, "Missing metadata line");
}

/// SEC-08: Partial JSON "TRUSTEDGE-KEY-V1\n{\"salt\":" (no closing newline) is rejected.
///
/// If the metadata line is truncated mid-way (no second newline found), the parser
/// must detect the missing structural delimiter and reject the file.
#[test]
fn sec_08_truncated_mid_json() {
    let data = b"TRUSTEDGE-KEY-V1\n{\"salt\":".to_vec();
    let result = DeviceKeypair::import_secret_encrypted(&data, "any-passphrase");
    assert_invalid_key_format(result, "Missing metadata line");
}

/// SEC-08: Valid header + valid JSON + newline but zero ciphertext bytes is rejected.
///
/// AES-256-GCM requires at least 16 bytes of authentication tag; an empty ciphertext
/// section cannot be decrypted and must produce DecryptionFailed.
#[test]
fn sec_08_truncated_no_ciphertext() {
    let valid = make_valid_encrypted_key();
    // Find the end of the second line (after the metadata newline)
    let first_newline = valid.iter().position(|&b| b == b'\n').unwrap();
    let second_newline = valid[first_newline + 1..]
        .iter()
        .position(|&b| b == b'\n')
        .unwrap()
        + first_newline
        + 1;
    // Truncate to include header and metadata but zero ciphertext bytes
    let truncated = valid[..=second_newline].to_vec();
    let result = DeviceKeypair::import_secret_encrypted(&truncated, "test-passphrase-123");
    assert_decryption_failed(result, "Wrong passphrase");
}

/// SEC-08: Valid header + valid JSON + newline + first half of ciphertext is rejected.
///
/// A file truncated mid-ciphertext loses the AES-GCM authentication tag and must
/// fail authentication, producing DecryptionFailed.
#[test]
fn sec_08_truncated_mid_ciphertext() {
    let valid = make_valid_encrypted_key();
    let first_newline = valid.iter().position(|&b| b == b'\n').unwrap();
    let second_newline = valid[first_newline + 1..]
        .iter()
        .position(|&b| b == b'\n')
        .unwrap()
        + first_newline
        + 1;
    let ciphertext_start = second_newline + 1;
    let ciphertext_len = valid.len() - ciphertext_start;
    // Keep only the first half of the ciphertext
    let half = ciphertext_len / 2;
    let truncated = valid[..ciphertext_start + half].to_vec();
    let result = DeviceKeypair::import_secret_encrypted(&truncated, "test-passphrase-123");
    assert_decryption_failed(result, "Wrong passphrase");
}

// ---------------------------------------------------------------------------
// SEC-09: Corrupted JSON header
// ---------------------------------------------------------------------------

/// SEC-09: Non-JSON second line is rejected with "Invalid JSON metadata".
///
/// When the metadata line is present but cannot be parsed as JSON, the parser
/// must report the structural error clearly.
#[test]
fn sec_09_json_not_json() {
    let data = build_corrupted_key_file("not-json-at-all", b"dummy-ciphertext");
    let result = DeviceKeypair::import_secret_encrypted(&data, "any-passphrase");
    assert_invalid_key_format(result, "Invalid JSON metadata");
}

/// SEC-09: Valid JSON missing the "salt" field is rejected with "Missing salt".
#[test]
fn sec_09_json_missing_salt() {
    let meta = r#"{"nonce":"AAAAAAAAAAAAAAAA","iterations":600000}"#;
    let data = build_corrupted_key_file(meta, b"dummy-ciphertext");
    let result = DeviceKeypair::import_secret_encrypted(&data, "any-passphrase");
    assert_invalid_key_format(result, "Missing salt");
}

/// SEC-09: Valid JSON missing the "nonce" field is rejected with "Missing nonce".
#[test]
fn sec_09_json_missing_nonce() {
    let salt_b64 = BASE64.encode([0u8; 32]);
    let meta = format!(r#"{{"salt":"{salt_b64}","iterations":600000}}"#);
    let data = build_corrupted_key_file(&meta, b"dummy-ciphertext");
    let result = DeviceKeypair::import_secret_encrypted(&data, "any-passphrase");
    assert_invalid_key_format(result, "Missing nonce");
}

/// SEC-09: Valid JSON missing the "iterations" field is rejected with "Missing iterations".
#[test]
fn sec_09_json_missing_iterations() {
    let salt_b64 = BASE64.encode([0u8; 32]);
    let nonce_b64 = BASE64.encode([0u8; 12]);
    let meta = format!(r#"{{"salt":"{salt_b64}","nonce":"{nonce_b64}"}}"#);
    let data = build_corrupted_key_file(&meta, b"dummy-ciphertext");
    let result = DeviceKeypair::import_secret_encrypted(&data, "any-passphrase");
    assert_invalid_key_format(result, "Missing iterations");
}

/// SEC-09: Salt field containing invalid base64 "!!not-base64!!" is rejected with "Invalid salt base64".
#[test]
fn sec_09_json_bad_base64_salt() {
    let nonce_b64 = BASE64.encode([0u8; 12]);
    let meta = format!(r#"{{"salt":"!!not-base64!!","nonce":"{nonce_b64}","iterations":600000}}"#);
    let data = build_corrupted_key_file(&meta, b"dummy-ciphertext");
    let result = DeviceKeypair::import_secret_encrypted(&data, "any-passphrase");
    assert_invalid_key_format(result, "Invalid salt base64");
}

/// SEC-09: Nonce field with valid base64 decoding to 8 bytes (not 12) is rejected with "Nonce must be 12 bytes".
///
/// AES-256-GCM requires exactly 12-byte nonces. A nonce of any other length must
/// be explicitly rejected rather than silently truncated or padded.
#[test]
fn sec_09_json_wrong_nonce_length() {
    let salt_b64 = BASE64.encode([0u8; 32]);
    // 8 bytes of nonce — valid base64, wrong length
    let short_nonce_b64 = BASE64.encode([0u8; 8]);
    let meta =
        format!(r#"{{"salt":"{salt_b64}","nonce":"{short_nonce_b64}","iterations":600000}}"#);
    let data = build_corrupted_key_file(&meta, b"dummy-ciphertext");
    let result = DeviceKeypair::import_secret_encrypted(&data, "any-passphrase");
    assert_invalid_key_format(result, "Nonce must be 12 bytes");
}

// ---------------------------------------------------------------------------
// SEC-10: Wrong passphrase
// ---------------------------------------------------------------------------

/// SEC-10: Encrypting with "correct" and importing with "wrong" returns DecryptionFailed.
///
/// The AES-GCM authentication tag ensures that a wrong passphrase produces a
/// distinct decryption key that fails the integrity check, not garbage plaintext.
#[test]
fn sec_10_wrong_passphrase_returns_error() {
    let valid = make_valid_encrypted_key();
    let result = DeviceKeypair::import_secret_encrypted(&valid, "wrong-passphrase");
    assert_decryption_failed(result, "Wrong passphrase");
}

/// SEC-10: Wrong passphrase produces DecryptionFailed variant (not Ok with garbage key material).
///
/// This test explicitly pattern-matches the error variant to confirm no key material
/// is returned — a AEAD scheme must either authenticate successfully or fail entirely.
#[test]
fn sec_10_wrong_passphrase_no_partial_key() {
    let valid = make_valid_encrypted_key();
    let result = DeviceKeypair::import_secret_encrypted(&valid, "wrong-passphrase");
    assert!(
        matches!(result, Err(CryptoError::DecryptionFailed(_))),
        "expected Err(DecryptionFailed(_)) but got a different variant or Ok"
    );
}

/// SEC-10: Empty passphrase on a key encrypted with a real passphrase returns DecryptionFailed.
///
/// An empty string must not be treated as a special bypass or default credential.
/// PBKDF2 with an empty passphrase derives a different key, which the AES-GCM tag rejects.
#[test]
fn sec_10_empty_passphrase() {
    let valid = make_valid_encrypted_key();
    let result = DeviceKeypair::import_secret_encrypted(&valid, "");
    assert_decryption_failed(result, "Wrong passphrase");
}
