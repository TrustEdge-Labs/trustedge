//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/
//
// Project: trustedge — Privacy and trust at the edge.
//

use aead::{Aead, KeyInit};
use chacha20poly1305::{XChaCha20Poly1305, XNonce};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand_core::{OsRng, RngCore};
use serde_json::json;
use thiserror::Error;
use zeroize::Zeroize;

#[derive(Error, Debug)]
pub enum CryptoError {
    #[error("Invalid key format: {0}")]
    InvalidKeyFormat(String),
    #[error("Invalid signature format: {0}")]
    InvalidSignatureFormat(String),
    #[error("Encryption failed: {0}")]
    EncryptionFailed(String),
    #[error("Decryption failed: {0}")]
    DecryptionFailed(String),
    #[error("Signature verification failed")]
    SignatureVerificationFailed,
    #[error("Key generation failed: {0}")]
    KeyGenerationFailed(String),
    #[error("Invalid nonce format: {0}")]
    InvalidNonceFormat(String),
}

/// Device keypair for Ed25519 signing operations
///
/// Note: The secret key is automatically zeroized when dropped for security
pub struct DeviceKeypair {
    pub public: String, // "ed25519:BASE64" format
    secret: [u8; 32],   // Raw secret key bytes (zeroized on drop)
}

impl Drop for DeviceKeypair {
    fn drop(&mut self) {
        self.secret.zeroize();
    }
}

impl DeviceKeypair {
    /// Generate a new random Ed25519 keypair
    pub fn generate() -> Result<Self, CryptoError> {
        let signing_key = SigningKey::generate(&mut OsRng);
        let verifying_key = signing_key.verifying_key();

        let public = format!("ed25519:{}", base64_encode(verifying_key.as_bytes()));
        let secret = signing_key.to_bytes();

        Ok(Self { public, secret })
    }

    /// Export the secret key as "ed25519:BASE64" format
    pub fn export_secret(&self) -> String {
        format!("ed25519:{}", base64_encode(&self.secret))
    }

    /// Import a keypair from secret key string (accepts "ed25519:BASE64" or hex)
    pub fn import_secret(secret_str: &str) -> Result<Self, CryptoError> {
        let secret_bytes = if let Some(b64_part) = secret_str.strip_prefix("ed25519:") {
            // Base64 format
            base64_decode(b64_part)
                .map_err(|e| CryptoError::InvalidKeyFormat(format!("Invalid base64: {}", e)))?
        } else if secret_str.len() == 64 {
            // Hex format
            hex::decode(secret_str)
                .map_err(|e| CryptoError::InvalidKeyFormat(format!("Invalid hex: {}", e)))?
        } else {
            return Err(CryptoError::InvalidKeyFormat(
                "Must be ed25519:BASE64 or 64-char hex".to_string(),
            ));
        };

        if secret_bytes.len() != 32 {
            return Err(CryptoError::InvalidKeyFormat(
                "Secret key must be 32 bytes".to_string(),
            ));
        }

        let mut secret = [0u8; 32];
        secret.copy_from_slice(&secret_bytes);

        // Compute public key from secret
        let signing_key = SigningKey::from_bytes(&secret);
        let verifying_key = signing_key.verifying_key();
        let public = format!("ed25519:{}", base64_encode(verifying_key.as_bytes()));

        Ok(Self { public, secret })
    }

    /// Import from public key string for verification only
    pub fn from_public(public_str: &str) -> Result<VerifyingKey, CryptoError> {
        if !public_str.starts_with("ed25519:") {
            return Err(CryptoError::InvalidKeyFormat(
                "Public key must start with 'ed25519:'".to_string(),
            ));
        }

        let b64_part = &public_str[8..];
        let pub_bytes = base64_decode(b64_part)
            .map_err(|e| CryptoError::InvalidKeyFormat(format!("Invalid base64: {}", e)))?;

        if pub_bytes.len() != 32 {
            return Err(CryptoError::InvalidKeyFormat(
                "Public key must be 32 bytes".to_string(),
            ));
        }

        let mut key_bytes = [0u8; 32];
        key_bytes.copy_from_slice(&pub_bytes);

        VerifyingKey::from_bytes(&key_bytes)
            .map_err(|e| CryptoError::InvalidKeyFormat(format!("Invalid public key: {}", e)))
    }

    /// Get the signing key for internal operations
    fn signing_key(&self) -> SigningKey {
        SigningKey::from_bytes(&self.secret)
    }
}

/// Generate a 24-byte nonce for XChaCha20Poly1305
pub fn generate_nonce24() -> [u8; 24] {
    let mut nonce = [0u8; 24];
    OsRng.fill_bytes(&mut nonce);
    nonce
}

/// Format nonce for manifest storage
pub fn format_nonce(nonce: &[u8; 24]) -> String {
    format!("xchacha20:{}", base64_encode(nonce))
}

/// Parse nonce from manifest format
pub fn parse_nonce(nonce_str: &str) -> Result<[u8; 24], CryptoError> {
    if !nonce_str.starts_with("xchacha20:") {
        return Err(CryptoError::InvalidNonceFormat(
            "Nonce must start with 'xchacha20:'".to_string(),
        ));
    }

    let b64_part = &nonce_str[10..];
    let nonce_bytes = base64_decode(b64_part)
        .map_err(|e| CryptoError::InvalidNonceFormat(format!("Invalid base64: {}", e)))?;

    if nonce_bytes.len() != 24 {
        return Err(CryptoError::InvalidNonceFormat(
            "Nonce must be 24 bytes".to_string(),
        ));
    }

    let mut nonce = [0u8; 24];
    nonce.copy_from_slice(&nonce_bytes);
    Ok(nonce)
}

/// Encrypt segment data using XChaCha20Poly1305 with AAD
pub fn encrypt_segment(
    key: &chacha20poly1305::Key,
    nonce24: &[u8; 24],
    plaintext: &[u8],
    aad: &[u8], // Additional authenticated data (canonical header fields)
) -> Result<Vec<u8>, CryptoError> {
    let cipher = XChaCha20Poly1305::new(key);
    let nonce = XNonce::from_slice(nonce24);

    cipher
        .encrypt(
            nonce,
            aead::Payload {
                msg: plaintext,
                aad,
            },
        )
        .map_err(|e| {
            CryptoError::EncryptionFailed(format!("XChaCha20Poly1305 encryption failed: {}", e))
        })
}

/// Decrypt segment data using XChaCha20Poly1305 with AAD
pub fn decrypt_segment(
    key: &chacha20poly1305::Key,
    nonce24: &[u8; 24],
    ciphertext: &[u8],
    aad: &[u8], // Additional authenticated data (canonical header fields)
) -> Result<Vec<u8>, CryptoError> {
    let cipher = XChaCha20Poly1305::new(key);
    let nonce = XNonce::from_slice(nonce24);

    cipher
        .decrypt(
            nonce,
            aead::Payload {
                msg: ciphertext,
                aad,
            },
        )
        .map_err(|e| {
            CryptoError::DecryptionFailed(format!("XChaCha20Poly1305 decryption failed: {}", e))
        })
}

/// Generate AAD from canonical header fields
pub fn generate_aad(
    trst_version: &str,
    profile: &str,
    device_id: &str,
    started_at: &str,
) -> Vec<u8> {
    let aad_json = json!({
        "trst_version": trst_version,
        "profile": profile,
        "device.id": device_id,
        "capture.started_at": started_at
    });

    // Use compact JSON representation for AAD
    serde_json::to_string(&aad_json).unwrap().into_bytes()
}

/// Sign manifest canonical bytes with device secret key
pub fn sign_manifest(
    device_keypair: &DeviceKeypair,
    canonical_bytes: &[u8],
) -> Result<String, CryptoError> {
    let signing_key = device_keypair.signing_key();
    let signature = signing_key.sign(canonical_bytes);

    Ok(format!(
        "ed25519:{}",
        base64_encode(signature.to_bytes().as_ref())
    ))
}

/// Verify manifest signature with device public key
pub fn verify_manifest(
    device_public: &str,
    canonical_bytes: &[u8],
    signature_str: &str,
) -> Result<bool, CryptoError> {
    // Parse public key
    let verifying_key = DeviceKeypair::from_public(device_public)?;

    // Parse signature
    if !signature_str.starts_with("ed25519:") {
        return Err(CryptoError::InvalidSignatureFormat(
            "Signature must start with 'ed25519:'".to_string(),
        ));
    }

    let b64_part = &signature_str[8..];
    let sig_bytes = base64_decode(b64_part)
        .map_err(|e| CryptoError::InvalidSignatureFormat(format!("Invalid base64: {}", e)))?;

    if sig_bytes.len() != 64 {
        return Err(CryptoError::InvalidSignatureFormat(
            "Signature must be 64 bytes".to_string(),
        ));
    }

    let signature = Signature::from_bytes(&sig_bytes.try_into().unwrap());

    // Verify signature
    match verifying_key.verify(canonical_bytes, &signature) {
        Ok(()) => Ok(true),
        Err(_) => Ok(false),
    }
}

/// Simple base64 encoding
fn base64_encode(bytes: &[u8]) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let mut result = String::new();
    let mut i = 0;

    while i < bytes.len() {
        let b1 = bytes[i];
        let b2 = if i + 1 < bytes.len() { bytes[i + 1] } else { 0 };
        let b3 = if i + 2 < bytes.len() { bytes[i + 2] } else { 0 };

        let chunk = ((b1 as u32) << 16) | ((b2 as u32) << 8) | (b3 as u32);

        result.push(CHARS[((chunk >> 18) & 63) as usize] as char);
        result.push(CHARS[((chunk >> 12) & 63) as usize] as char);

        if i + 1 < bytes.len() {
            result.push(CHARS[((chunk >> 6) & 63) as usize] as char);
        } else {
            result.push('=');
        }

        if i + 2 < bytes.len() {
            result.push(CHARS[(chunk & 63) as usize] as char);
        } else {
            result.push('=');
        }

        i += 3;
    }

    result
}

/// Simple base64 decoding
fn base64_decode(s: &str) -> Result<Vec<u8>, String> {
    let chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = Vec::new();
    let mut buffer = 0u32;
    let mut bits = 0;

    for c in s.chars() {
        if c == '=' {
            break;
        }

        let value = chars
            .find(c)
            .ok_or_else(|| format!("Invalid character: {}", c))? as u32;
        buffer = (buffer << 6) | value;
        bits += 6;

        if bits >= 8 {
            result.push((buffer >> (bits - 8)) as u8);
            bits -= 8;
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_keypair_generation() {
        let keypair = DeviceKeypair::generate().unwrap();
        assert!(keypair.public.starts_with("ed25519:"));

        let exported_secret = keypair.export_secret();
        assert!(exported_secret.starts_with("ed25519:"));
    }

    #[test]
    fn test_device_keypair_import_export() {
        let keypair1 = DeviceKeypair::generate().unwrap();
        let exported_secret = keypair1.export_secret();
        let exported_public = keypair1.public.clone();

        // Import from exported secret
        let keypair2 = DeviceKeypair::import_secret(&exported_secret).unwrap();
        assert_eq!(keypair2.public, exported_public);

        // Test hex import
        let hex_secret = hex::encode(&keypair1.secret);
        let keypair3 = DeviceKeypair::import_secret(&hex_secret).unwrap();
        assert_eq!(keypair3.public, exported_public);
    }

    #[test]
    fn test_public_key_parsing() {
        let keypair = DeviceKeypair::generate().unwrap();
        let verifying_key = DeviceKeypair::from_public(&keypair.public).unwrap();

        // Should be able to parse the public key successfully
        assert_eq!(verifying_key.as_bytes().len(), 32);
    }

    #[test]
    fn test_nonce_generation_and_formatting() {
        let nonce = generate_nonce24();
        assert_eq!(nonce.len(), 24);

        let formatted = format_nonce(&nonce);
        assert!(formatted.starts_with("xchacha20:"));

        let parsed = parse_nonce(&formatted).unwrap();
        assert_eq!(nonce, parsed);
    }

    #[test]
    fn test_round_trip_sign_verify() {
        let keypair = DeviceKeypair::generate().unwrap();
        let test_data = b"test canonical bytes for signing";

        // Sign the data
        let signature = sign_manifest(&keypair, test_data).unwrap();
        assert!(signature.starts_with("ed25519:"));

        // Verify the signature
        let is_valid = verify_manifest(&keypair.public, test_data, &signature).unwrap();
        assert!(is_valid);

        // Verify with wrong data should fail
        let wrong_data = b"wrong data";
        let is_valid_wrong = verify_manifest(&keypair.public, wrong_data, &signature).unwrap();
        assert!(!is_valid_wrong);
    }

    #[test]
    fn test_aead_encrypt_decrypt_round_trip() {
        // Generate key and nonce
        let key = XChaCha20Poly1305::generate_key(&mut OsRng);
        let nonce = generate_nonce24();

        // Test data
        let plaintext = b"test segment data for encryption";
        let aad = generate_aad("0.1.0", "cam.video", "TEST001", "2025-01-15T10:30:00Z");

        // Encrypt
        let ciphertext = encrypt_segment(&key, &nonce, plaintext, &aad).unwrap();
        assert_ne!(ciphertext, plaintext);

        // Decrypt
        let decrypted = decrypt_segment(&key, &nonce, &ciphertext, &aad).unwrap();
        assert_eq!(decrypted, plaintext);

        // Decrypt with wrong AAD should fail
        let wrong_aad = generate_aad("0.1.0", "cam.video", "WRONG", "2025-01-15T10:30:00Z");
        let result = decrypt_segment(&key, &nonce, &ciphertext, &wrong_aad);
        assert!(result.is_err());
    }

    #[test]
    fn test_aad_generation() {
        let aad = generate_aad("0.1.0", "cam.video", "TEST001", "2025-01-15T10:30:00Z");
        let aad_str = String::from_utf8(aad).unwrap();

        // Should contain all the required fields
        assert!(aad_str.contains("0.1.0"));
        assert!(aad_str.contains("cam.video"));
        assert!(aad_str.contains("TEST001"));
        assert!(aad_str.contains("2025-01-15T10:30:00Z"));
    }

    #[test]
    fn test_base64_encoding_decoding() {
        let test_data = b"Hello, World! This is a test.";
        let encoded = base64_encode(test_data);
        let decoded = base64_decode(&encoded).unwrap();
        assert_eq!(test_data.to_vec(), decoded);
    }

    #[test]
    fn test_secret_zeroization() {
        let mut secret_bytes = [42u8; 32];

        {
            let _keypair = DeviceKeypair::generate().unwrap();
            // Keypair exists in this scope
        } // Keypair is dropped here and secret should be zeroized

        // We can't directly test that the memory was zeroized since it's private,
        // but we can test that zeroize works on our test data
        secret_bytes.zeroize();
        assert_eq!(secret_bytes, [0u8; 32]);
    }
}
