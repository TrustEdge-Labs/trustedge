// Copyright (c) 2025 TRUSTEDGE LABS LLC
// MPL-2.0: https://mozilla.org/MPL/2.0/
// Project: trustedge — Privacy and trust at the edge.

//! Hybrid Encryption API for TrustEdge
//!
//! This module provides the high-level API for hybrid encryption that combines
//! the efficiency of symmetric encryption with the convenience of public key cryptography.

use crate::asymmetric::{decrypt_key_asymmetric, encrypt_key_asymmetric, PrivateKey, PublicKey};
use crate::format::AeadAlgorithm;
use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Errors that can occur during hybrid encryption operations
#[derive(Debug, thiserror::Error)]
pub enum HybridEncryptionError {
    #[error("Encryption failed: {0}")]
    EncryptionFailed(String),

    #[error("Decryption failed: {0}")]
    DecryptionFailed(String),

    #[error("Invalid envelope format: {0}")]
    InvalidEnvelope(String),

    #[error("Asymmetric operation failed: {0}")]
    AsymmetricError(#[from] crate::asymmetric::AsymmetricError),

    #[error("Serialization failed: {0}")]
    SerializationError(#[from] bincode::Error),

    #[error("Internal error: {0}")]
    InternalError(#[from] anyhow::Error),
}

/// A symmetric encryption key
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymmetricKey([u8; 32]);

impl SymmetricKey {
    /// Generate a new random symmetric key
    pub fn generate() -> Self {
        use rand::{rngs::OsRng, RngCore};
        let mut key = [0u8; 32];
        OsRng.fill_bytes(&mut key);
        Self(key)
    }

    /// Create a symmetric key from bytes
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Get the key as bytes
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

/// The structure of a hybrid-encrypted envelope
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HybridEnvelope {
    /// Magic number identifying this as a TrustEdge hybrid envelope
    pub magic: [u8; 4],
    /// Version of the envelope format
    pub version: u8,
    /// ID of the recipient's public key
    pub recipient_key_id: String,
    /// The session key encrypted with the recipient's public key
    pub encrypted_session_key: Vec<u8>,
    /// The payload encrypted with the session key
    pub encrypted_payload: Vec<u8>,
    /// Nonce used for symmetric encryption
    pub nonce: [u8; 12],
    /// Algorithm used for symmetric encryption
    pub algorithm: u8, // AeadAlgorithm as u8
}

/// Magic number for hybrid envelopes
const HYBRID_MAGIC: [u8; 4] = *b"TRHY"; // TRustEdge HYbrid

/// Current version of the hybrid envelope format
const HYBRID_VERSION: u8 = 1;

/// Seal a payload for a specific recipient using hybrid encryption
///
/// This function:
/// 1. Generates a new, random one-time symmetric key (AES-256-GCM)
/// 2. Encrypts the main data payload with this session key
/// 3. Uses the recipient's public key to encrypt the session key
/// 4. Assembles the final .trst file structure
pub fn seal_for_recipient(
    data: &[u8],
    recipient_public_key: &PublicKey,
) -> Result<Vec<u8>, HybridEncryptionError> {
    // 1. Generate a new, random one-time symmetric key (AES-256-GCM)
    let session_key = SymmetricKey::generate();

    // 2. Encrypt the main data payload with this session key
    let encrypted_payload = encrypt_symmetric(data, &session_key)?;

    // 3. Use the recipient's public key to encrypt the session_key
    let encrypted_session_key =
        encrypt_key_asymmetric(session_key.as_bytes(), recipient_public_key).map_err(|e| {
            HybridEncryptionError::EncryptionFailed(format!("Key encryption failed: {}", e))
        })?;

    // 4. Assemble the new .trst file structure
    let final_envelope = assemble_envelope(
        &recipient_public_key.id(),
        &encrypted_session_key,
        &encrypted_payload.ciphertext,
        &encrypted_payload.nonce,
    )?;

    Ok(final_envelope)
}

/// Open a sealed envelope using hybrid decryption
///
/// This function:
/// 1. Parses the envelope to get the encrypted_session_key
/// 2. Uses the private key to decrypt the session key
/// 3. Uses the decrypted session key to decrypt the main payload
pub fn open_envelope(
    envelope: &[u8],
    my_private_key: &PrivateKey,
) -> Result<Vec<u8>, HybridEncryptionError> {
    // 1. Parse the envelope to get the encrypted_session_key
    let parsed_envelope = parse_envelope(envelope)?;

    // 2. Use my private key to decrypt the session key
    let session_key_bytes =
        decrypt_key_asymmetric(&parsed_envelope.encrypted_session_key, my_private_key).map_err(
            |e| HybridEncryptionError::DecryptionFailed(format!("Key decryption failed: {}", e)),
        )?;

    let session_key = SymmetricKey::from_bytes(session_key_bytes);

    // 3. Use the decrypted session key to decrypt the main payload
    let encrypted_data = EncryptedData {
        ciphertext: parsed_envelope.encrypted_payload,
        nonce: parsed_envelope.nonce,
    };

    let decrypted_data = decrypt_symmetric(&encrypted_data, &session_key)?;

    Ok(decrypted_data)
}

/// Encrypted data with nonce
#[derive(Debug, Clone)]
struct EncryptedData {
    ciphertext: Vec<u8>,
    nonce: [u8; 12],
}

/// Encrypt data using symmetric encryption (AES-256-GCM)
fn encrypt_symmetric(data: &[u8], key: &SymmetricKey) -> Result<EncryptedData, HybridEncryptionError> {
    use aes_gcm::{AeadInPlace, Aes256Gcm, KeyInit};
    use rand::{rngs::OsRng, RngCore};

    let cipher = Aes256Gcm::new_from_slice(key.as_bytes())
        .map_err(|e| HybridEncryptionError::EncryptionFailed(format!("Failed to create cipher: {}", e)))?;

    // Generate random nonce
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);

    // Encrypt data
    let mut ciphertext = data.to_vec();
    cipher
        .encrypt_in_place((&nonce_bytes).into(), b"", &mut ciphertext)
        .map_err(|e| {
            HybridEncryptionError::EncryptionFailed(format!("AES-GCM encryption failed: {}", e))
        })?;

    Ok(EncryptedData {
        ciphertext,
        nonce: nonce_bytes,
    })
}

/// Decrypt data using symmetric encryption (AES-256-GCM)
fn decrypt_symmetric(
    encrypted: &EncryptedData,
    key: &SymmetricKey,
) -> Result<Vec<u8>, HybridEncryptionError> {
    use aes_gcm::{AeadInPlace, Aes256Gcm, KeyInit};

    let cipher = Aes256Gcm::new_from_slice(key.as_bytes())
        .map_err(|e| HybridEncryptionError::DecryptionFailed(format!("Failed to create cipher: {}", e)))?;

    let nonce_array: &[u8; 12] = encrypted
        .nonce
        .as_slice()
        .try_into()
        .map_err(|_| HybridEncryptionError::DecryptionFailed("Nonce conversion failed".to_string()))?;

    // Decrypt data
    let mut plaintext = encrypted.ciphertext.clone();
    cipher
        .decrypt_in_place(nonce_array.into(), b"", &mut plaintext)
        .map_err(|e| {
            HybridEncryptionError::DecryptionFailed(format!("AES-GCM decryption failed: {}", e))
        })?;

    Ok(plaintext)
}

/// Assemble the final envelope structure
fn assemble_envelope(
    recipient_key_id: &str,
    encrypted_session_key: &[u8],
    encrypted_payload: &[u8],
    nonce: &[u8; 12],
) -> Result<Vec<u8>, HybridEncryptionError> {
    let envelope = HybridEnvelope {
        magic: HYBRID_MAGIC,
        version: HYBRID_VERSION,
        recipient_key_id: recipient_key_id.to_string(),
        encrypted_session_key: encrypted_session_key.to_vec(),
        encrypted_payload: encrypted_payload.to_vec(),
        nonce: *nonce,
        algorithm: AeadAlgorithm::Aes256Gcm as u8,
    };

    bincode::serialize(&envelope).map_err(HybridEncryptionError::SerializationError)
}

/// Parse an envelope from bytes
fn parse_envelope(envelope_bytes: &[u8]) -> Result<HybridEnvelope, HybridEncryptionError> {
    let envelope: HybridEnvelope = bincode::deserialize(envelope_bytes)
        .map_err(|e| HybridEncryptionError::InvalidEnvelope(format!("Deserialization failed: {}", e)))?;

    // Validate envelope
    if envelope.magic != HYBRID_MAGIC {
        return Err(HybridEncryptionError::InvalidEnvelope(
            "Invalid magic number".to_string(),
        ));
    }

    if envelope.version != HYBRID_VERSION {
        return Err(HybridEncryptionError::InvalidEnvelope(format!(
            "Unsupported version: {}",
            envelope.version
        )));
    }

    Ok(envelope)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::asymmetric::KeyPair;
    use crate::backends::AsymmetricAlgorithm;

    #[test]
    fn test_symmetric_key_generation() {
        let key1 = SymmetricKey::generate();
        let key2 = SymmetricKey::generate();

        // Keys should be different
        assert_ne!(key1, key2);

        // Keys should be 32 bytes
        assert_eq!(key1.as_bytes().len(), 32);
        assert_eq!(key2.as_bytes().len(), 32);
    }

    #[test]
    fn test_symmetric_encryption_roundtrip() {
        let key = SymmetricKey::generate();
        let data = b"Hello, symmetric world!";

        let encrypted = encrypt_symmetric(data, &key).expect("Failed to encrypt data");

        let decrypted = decrypt_symmetric(&encrypted, &key).expect("Failed to decrypt data");

        assert_eq!(data, decrypted.as_slice());
    }

    #[test]
    fn test_hybrid_encryption_rsa() {
        let keypair = KeyPair::generate(AsymmetricAlgorithm::Rsa2048)
            .expect("Failed to generate RSA key pair");

        let data = b"Hello, hybrid world with RSA!";

        let envelope = seal_for_recipient(data, &keypair.public).expect("Failed to seal envelope");

        let decrypted =
            open_envelope(&envelope, &keypair.private).expect("Failed to open envelope");

        assert_eq!(data, decrypted.as_slice());
    }

    #[test]
    fn test_envelope_structure() {
        let keypair =
            KeyPair::generate(AsymmetricAlgorithm::Rsa2048).expect("Failed to generate key pair");

        let data = b"Test data for envelope structure";

        let envelope_bytes =
            seal_for_recipient(data, &keypair.public).expect("Failed to seal envelope");

        // Parse the envelope to check structure
        let envelope = parse_envelope(&envelope_bytes).expect("Failed to parse envelope");

        assert_eq!(envelope.magic, HYBRID_MAGIC);
        assert_eq!(envelope.version, HYBRID_VERSION);
        assert_eq!(envelope.recipient_key_id, keypair.public.id());
        assert!(!envelope.encrypted_session_key.is_empty());
        assert!(!envelope.encrypted_payload.is_empty());
        assert_eq!(envelope.algorithm, AeadAlgorithm::Aes256Gcm as u8);
    }

    #[test]
    fn test_invalid_envelope() {
        let invalid_data = b"This is not a valid envelope";

        let result = parse_envelope(invalid_data);
        assert!(result.is_err());

        match result {
            Err(HybridEncryptionError::InvalidEnvelope(_)) => {} // Expected
            _ => panic!("Expected InvalidEnvelope error"),
        }
    }

    #[test]
    fn test_wrong_private_key() {
        let alice_keypair = KeyPair::generate(AsymmetricAlgorithm::Rsa2048)
            .expect("Failed to generate Alice's key pair");

        let bob_keypair = KeyPair::generate(AsymmetricAlgorithm::Rsa2048)
            .expect("Failed to generate Bob's key pair");

        let data = b"Secret message for Alice";

        // Encrypt for Alice
        let envelope =
            seal_for_recipient(data, &alice_keypair.public).expect("Failed to seal envelope");

        // Try to decrypt with Bob's key (should fail)
        let result = open_envelope(&envelope, &bob_keypair.private);
        assert!(result.is_err());
    }
}
