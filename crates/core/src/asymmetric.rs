// Copyright (c) 2025 TRUSTEDGE LABS LLC
// MPL-2.0: https://mozilla.org/MPL/2.0/
// Project: trustedge — Privacy and trust at the edge.

//! Asymmetric Cryptography Primitives for TrustEdge
//!
//! This module provides the fundamental building blocks for public key cryptography
//! in TrustEdge, including key generation, key exchange, and hybrid encryption.

use crate::backends::AsymmetricAlgorithm;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fmt;

/// A public key for asymmetric cryptography
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicKey {
    /// The algorithm used for this key
    pub algorithm: AsymmetricAlgorithm,
    /// The raw key bytes
    pub key_bytes: Vec<u8>,
    /// Optional key identifier for lookups
    pub key_id: Option<String>,
}

/// A private key for asymmetric cryptography
#[derive(Clone, Serialize, Deserialize)]
pub struct PrivateKey {
    /// The algorithm used for this key
    pub algorithm: AsymmetricAlgorithm,
    /// The raw key bytes (sensitive data)
    pub key_bytes: Vec<u8>,
    /// Optional key identifier for lookups
    pub key_id: Option<String>,
}

/// A key pair containing both public and private keys
#[derive(Clone)]
pub struct KeyPair {
    /// The public key
    pub public: PublicKey,
    /// The private key
    pub private: PrivateKey,
}

/// Errors that can occur during asymmetric operations
#[derive(Debug, thiserror::Error)]
pub enum AsymmetricError {
    #[error("Unsupported algorithm: {0:?}")]
    UnsupportedAlgorithm(AsymmetricAlgorithm),

    #[error("Invalid key format: {0}")]
    InvalidKeyFormat(String),

    #[error("Key exchange failed: {0}")]
    KeyExchangeFailed(String),

    #[error("Backend error: {0}")]
    BackendError(#[from] anyhow::Error),
}

impl PublicKey {
    /// Create a new public key
    pub fn new(algorithm: AsymmetricAlgorithm, key_bytes: Vec<u8>) -> Self {
        Self {
            algorithm,
            key_bytes,
            key_id: None,
        }
    }

    /// Create a new public key with an identifier
    pub fn with_id(algorithm: AsymmetricAlgorithm, key_bytes: Vec<u8>, key_id: String) -> Self {
        Self {
            algorithm,
            key_bytes,
            key_id: Some(key_id),
        }
    }

    /// Get a unique identifier for this public key
    pub fn id(&self) -> String {
        if let Some(ref id) = self.key_id {
            id.clone()
        } else {
            // Generate ID from key hash
            let hash = blake3::hash(&self.key_bytes);
            hex::encode(&hash.as_bytes()[..16]) // First 16 bytes as hex
        }
    }

    /// Get the key bytes
    pub fn as_bytes(&self) -> &[u8] {
        &self.key_bytes
    }

    /// Serialize to bytes
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        bincode::serialize(self).context("Failed to serialize public key")
    }

    /// Deserialize from bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        bincode::deserialize(bytes).context("Failed to deserialize public key")
    }
}

impl PrivateKey {
    /// Create a new private key
    pub fn new(algorithm: AsymmetricAlgorithm, key_bytes: Vec<u8>) -> Self {
        Self {
            algorithm,
            key_bytes,
            key_id: None,
        }
    }

    /// Create a new private key with an identifier
    pub fn with_id(algorithm: AsymmetricAlgorithm, key_bytes: Vec<u8>, key_id: String) -> Self {
        Self {
            algorithm,
            key_bytes,
            key_id: Some(key_id),
        }
    }

    /// Get the key identifier
    pub fn id(&self) -> String {
        if let Some(ref id) = self.key_id {
            id.clone()
        } else {
            // Generate ID from key hash (same as public key)
            let hash = blake3::hash(&self.key_bytes);
            hex::encode(&hash.as_bytes()[..16])
        }
    }

    /// Get the key bytes
    pub fn as_bytes(&self) -> &[u8] {
        &self.key_bytes
    }

    /// Serialize to bytes (encrypted storage recommended)
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        bincode::serialize(self).context("Failed to serialize private key")
    }

    /// Deserialize from bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        bincode::deserialize(bytes).context("Failed to deserialize private key")
    }
}

impl KeyPair {
    /// Create a new key pair
    pub fn new(public: PublicKey, private: PrivateKey) -> Self {
        Self { public, private }
    }

    /// Generate a new key pair using the specified algorithm
    pub fn generate(algorithm: AsymmetricAlgorithm) -> Result<Self> {
        match algorithm {
            AsymmetricAlgorithm::Ed25519 => Self::generate_ed25519(),
            AsymmetricAlgorithm::EcdsaP256 => Self::generate_ecdsa_p256(),
            AsymmetricAlgorithm::Rsa2048 => Self::generate_rsa(2048),
            AsymmetricAlgorithm::Rsa4096 => Self::generate_rsa(4096),
        }
    }

    /// Generate an Ed25519 key pair
    fn generate_ed25519() -> Result<Self> {
        use ed25519_dalek::SigningKey;
        use rand::rngs::OsRng;

        let signing_key = SigningKey::generate(&mut OsRng);
        let verifying_key = signing_key.verifying_key();

        let private = PrivateKey::new(
            AsymmetricAlgorithm::Ed25519,
            signing_key.to_bytes().to_vec(),
        );

        let public = PublicKey::new(
            AsymmetricAlgorithm::Ed25519,
            verifying_key.to_bytes().to_vec(),
        );

        Ok(Self::new(public, private))
    }

    /// Generate an ECDSA P-256 key pair
    fn generate_ecdsa_p256() -> Result<Self> {
        use p256::elliptic_curve::sec1::ToEncodedPoint;
        use p256::SecretKey;
        use rand::rngs::OsRng;

        let secret_key = SecretKey::random(&mut OsRng);
        let public_key = secret_key.public_key();

        let private = PrivateKey::new(
            AsymmetricAlgorithm::EcdsaP256,
            secret_key.to_bytes().to_vec(),
        );

        let public = PublicKey::new(
            AsymmetricAlgorithm::EcdsaP256,
            public_key.to_encoded_point(false).as_bytes().to_vec(),
        );

        Ok(Self::new(public, private))
    }

    /// Generate an RSA key pair
    fn generate_rsa(bits: usize) -> Result<Self> {
        use rand::rngs::OsRng;
        use rsa::pkcs8::{EncodePrivateKey, EncodePublicKey};
        use rsa::{RsaPrivateKey, RsaPublicKey};

        let private_key =
            RsaPrivateKey::new(&mut OsRng, bits).context("Failed to generate RSA private key")?;
        let public_key = RsaPublicKey::from(&private_key);

        let algorithm = match bits {
            2048 => AsymmetricAlgorithm::Rsa2048,
            4096 => AsymmetricAlgorithm::Rsa4096,
            _ => return Err(anyhow::anyhow!("Unsupported RSA key size: {}", bits)),
        };

        let private_der = private_key
            .to_pkcs8_der()
            .context("Failed to encode RSA private key")?;
        let public_der = public_key
            .to_public_key_der()
            .context("Failed to encode RSA public key")?;

        let private = PrivateKey::new(algorithm, private_der.as_bytes().to_vec());
        let public = PublicKey::new(algorithm, public_der.as_bytes().to_vec());

        Ok(Self::new(public, private))
    }
}

/// Perform ECDH key exchange to derive a shared secret
pub fn key_exchange(
    my_private_key: &PrivateKey,
    peer_public_key: &PublicKey,
) -> Result<Vec<u8>, AsymmetricError> {
    // Ensure both keys use compatible algorithms
    match (&my_private_key.algorithm, &peer_public_key.algorithm) {
        (AsymmetricAlgorithm::EcdsaP256, AsymmetricAlgorithm::EcdsaP256) => {
            ecdh_p256(my_private_key, peer_public_key)
        }
        _ => Err(AsymmetricError::UnsupportedAlgorithm(
            my_private_key.algorithm,
        )),
    }
}

/// Perform ECDH with P-256 keys
fn ecdh_p256(private_key: &PrivateKey, public_key: &PublicKey) -> Result<Vec<u8>, AsymmetricError> {
    use p256::elliptic_curve::sec1::FromEncodedPoint;
    use p256::{ecdh::diffie_hellman, PublicKey as P256PublicKey, SecretKey};

    // Parse private key
    let secret = SecretKey::from_slice(&private_key.key_bytes)
        .map_err(|e| AsymmetricError::InvalidKeyFormat(format!("Invalid private key: {}", e)))?;

    // Parse public key
    let point = p256::EncodedPoint::from_bytes(&public_key.key_bytes)
        .map_err(|e| AsymmetricError::InvalidKeyFormat(format!("Invalid public key: {}", e)))?;

    let peer_public = P256PublicKey::from_encoded_point(&point)
        .into_option()
        .ok_or_else(|| AsymmetricError::InvalidKeyFormat("Invalid public key point".to_string()))?;

    // Perform ECDH
    let shared_secret = diffie_hellman(secret.to_nonzero_scalar(), peer_public.as_affine());

    // Return the raw bytes of the shared secret
    Ok(shared_secret.raw_secret_bytes().to_vec())
}

/// Encrypt a symmetric key using asymmetric cryptography
pub fn encrypt_key_asymmetric(
    session_key: &[u8; 32],
    recipient_public_key: &PublicKey,
) -> Result<Vec<u8>, AsymmetricError> {
    match recipient_public_key.algorithm {
        AsymmetricAlgorithm::Rsa2048 | AsymmetricAlgorithm::Rsa4096 => {
            rsa_encrypt_key(session_key, recipient_public_key)
        }
        _ => Err(AsymmetricError::UnsupportedAlgorithm(
            recipient_public_key.algorithm,
        )),
    }
}

/// Decrypt a symmetric key using asymmetric cryptography
pub fn decrypt_key_asymmetric(
    encrypted_key: &[u8],
    my_private_key: &PrivateKey,
) -> Result<[u8; 32], AsymmetricError> {
    match my_private_key.algorithm {
        AsymmetricAlgorithm::Rsa2048 | AsymmetricAlgorithm::Rsa4096 => {
            rsa_decrypt_key(encrypted_key, my_private_key)
        }
        _ => Err(AsymmetricError::UnsupportedAlgorithm(
            my_private_key.algorithm,
        )),
    }
}

/// Encrypt a session key using RSA
fn rsa_encrypt_key(
    session_key: &[u8; 32],
    public_key: &PublicKey,
) -> Result<Vec<u8>, AsymmetricError> {
    use rand::rngs::OsRng;
    use rsa::pkcs8::DecodePublicKey;
    use rsa::{Pkcs1v15Encrypt, RsaPublicKey};

    let rsa_public = RsaPublicKey::from_public_key_der(&public_key.key_bytes)
        .map_err(|e| AsymmetricError::InvalidKeyFormat(format!("Invalid RSA public key: {}", e)))?;

    let encrypted = rsa_public
        .encrypt(&mut OsRng, Pkcs1v15Encrypt, session_key)
        .map_err(|e| AsymmetricError::KeyExchangeFailed(format!("RSA encryption failed: {}", e)))?;

    Ok(encrypted)
}

/// Decrypt a session key using RSA
fn rsa_decrypt_key(
    encrypted_key: &[u8],
    private_key: &PrivateKey,
) -> Result<[u8; 32], AsymmetricError> {
    use rsa::pkcs8::DecodePrivateKey;
    use rsa::{Pkcs1v15Encrypt, RsaPrivateKey};

    let rsa_private = RsaPrivateKey::from_pkcs8_der(&private_key.key_bytes).map_err(|e| {
        AsymmetricError::InvalidKeyFormat(format!("Invalid RSA private key: {}", e))
    })?;

    let decrypted = rsa_private
        .decrypt(Pkcs1v15Encrypt, encrypted_key)
        .map_err(|e| AsymmetricError::KeyExchangeFailed(format!("RSA decryption failed: {}", e)))?;

    if decrypted.len() != 32 {
        return Err(AsymmetricError::KeyExchangeFailed(format!(
            "Invalid session key length: expected 32, got {}",
            decrypted.len()
        )));
    }

    let mut key = [0u8; 32];
    key.copy_from_slice(&decrypted);
    Ok(key)
}

// Implement Debug for PrivateKey without exposing key material
impl fmt::Debug for PrivateKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PrivateKey")
            .field("algorithm", &self.algorithm)
            .field("key_id", &self.key_id)
            .field("key_bytes", &format!("[{} bytes]", self.key_bytes.len()))
            .finish()
    }
}

impl fmt::Debug for KeyPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("KeyPair")
            .field("public", &self.public)
            .field(
                "private",
                &format!("[PrivateKey: {} bytes]", self.private.key_bytes.len()),
            )
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ed25519_key_generation() {
        let keypair = KeyPair::generate(AsymmetricAlgorithm::Ed25519)
            .expect("Failed to generate Ed25519 key pair");

        assert_eq!(keypair.public.algorithm, AsymmetricAlgorithm::Ed25519);
        assert_eq!(keypair.private.algorithm, AsymmetricAlgorithm::Ed25519);
        assert_eq!(keypair.public.key_bytes.len(), 32);
        assert_eq!(keypair.private.key_bytes.len(), 32);
    }

    #[test]
    fn test_ecdsa_p256_key_generation() {
        let keypair = KeyPair::generate(AsymmetricAlgorithm::EcdsaP256)
            .expect("Failed to generate ECDSA P-256 key pair");

        assert_eq!(keypair.public.algorithm, AsymmetricAlgorithm::EcdsaP256);
        assert_eq!(keypair.private.algorithm, AsymmetricAlgorithm::EcdsaP256);
        assert_eq!(keypair.private.key_bytes.len(), 32); // P-256 private key
    }

    #[test]
    fn test_rsa_key_generation() {
        let keypair = KeyPair::generate(AsymmetricAlgorithm::Rsa2048)
            .expect("Failed to generate RSA-2048 key pair");

        assert_eq!(keypair.public.algorithm, AsymmetricAlgorithm::Rsa2048);
        assert_eq!(keypair.private.algorithm, AsymmetricAlgorithm::Rsa2048);
        assert!(keypair.public.key_bytes.len() > 200); // RSA public key DER
        assert!(keypair.private.key_bytes.len() > 1000); // RSA private key DER
    }

    #[test]
    fn test_public_key_serialization() {
        let keypair =
            KeyPair::generate(AsymmetricAlgorithm::Ed25519).expect("Failed to generate key pair");

        let serialized = keypair
            .public
            .to_bytes()
            .expect("Failed to serialize public key");

        let deserialized =
            PublicKey::from_bytes(&serialized).expect("Failed to deserialize public key");

        assert_eq!(keypair.public, deserialized);
    }

    #[test]
    fn test_rsa_key_encryption() {
        let keypair = KeyPair::generate(AsymmetricAlgorithm::Rsa2048)
            .expect("Failed to generate RSA key pair");

        let session_key = [42u8; 32];

        let encrypted = encrypt_key_asymmetric(&session_key, &keypair.public)
            .expect("Failed to encrypt session key");

        let decrypted = decrypt_key_asymmetric(&encrypted, &keypair.private)
            .expect("Failed to decrypt session key");

        assert_eq!(session_key, decrypted);
    }

    #[test]
    fn test_ecdh_p256() {
        let alice_keypair = KeyPair::generate(AsymmetricAlgorithm::EcdsaP256)
            .expect("Failed to generate Alice's key pair");

        let bob_keypair = KeyPair::generate(AsymmetricAlgorithm::EcdsaP256)
            .expect("Failed to generate Bob's key pair");

        let alice_shared = key_exchange(&alice_keypair.private, &bob_keypair.public)
            .expect("Alice's key exchange failed");

        let bob_shared = key_exchange(&bob_keypair.private, &alice_keypair.public)
            .expect("Bob's key exchange failed");

        assert_eq!(alice_shared, bob_shared);
        assert!(!alice_shared.is_empty());
    }
}
