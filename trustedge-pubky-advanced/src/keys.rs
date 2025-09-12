// Copyright (c) 2025 TRUSTEDGE LABS LLC
// MPL-2.0: https://mozilla.org/MPL/2.0/
// Project: trustedge — Privacy and trust at the edge.

//! Dual Key Architecture for TrustEdge Pubky Integration
//!
//! This module implements the dual key system where each identity has:
//! - Ed25519 key for identity/signing (Pubky identity)
//! - X25519 key for encryption (ECDH key exchange)

use anyhow::{Context, Result};
use ed25519_dalek::{SigningKey, VerifyingKey};
use serde::{Deserialize, Serialize};
use x25519_dalek::{PublicKey as X25519PublicKey, StaticSecret};
use zeroize::Zeroize;

/// A dual key pair for TrustEdge Pubky integration
///
/// This combines an Ed25519 identity key (for Pubky identity and signatures)
/// with an X25519 encryption key (for ECDH key exchange).
pub struct DualKeyPair {
    /// Ed25519 key for identity and signatures
    pub ed25519_key: SigningKey,
    /// X25519 key for encryption (ECDH)
    pub x25519_key: StaticSecret,
}

/// Serializable representation of public keys for storage/transmission
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PubkyIdentity {
    /// Ed25519 public key (Pubky identity)
    pub ed25519_pubkey: [u8; 32],
    /// X25519 public key (for encryption)
    pub x25519_pubkey: [u8; 32],
    /// Optional metadata
    pub metadata: Option<IdentityMetadata>,
}

/// Optional metadata associated with a Pubky identity
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IdentityMetadata {
    /// Human-readable name or handle
    pub name: Option<String>,
    /// Creation timestamp
    pub created_at: u64,
    /// Version of the identity format
    pub version: u8,
}

impl DualKeyPair {
    /// Generate a new dual key pair
    pub fn generate() -> Self {
        use rand::rngs::OsRng;
        
        Self {
            ed25519_key: SigningKey::generate(&mut OsRng),
            x25519_key: StaticSecret::random_from_rng(OsRng),
        }
    }

    /// Create from existing keys
    pub fn from_keys(ed25519_key: SigningKey, x25519_key: StaticSecret) -> Self {
        Self {
            ed25519_key,
            x25519_key,
        }
    }

    /// Get the Ed25519 public key
    pub fn ed25519_public(&self) -> VerifyingKey {
        self.ed25519_key.verifying_key()
    }

    /// Get the X25519 public key
    pub fn x25519_public(&self) -> X25519PublicKey {
        X25519PublicKey::from(&self.x25519_key)
    }

    /// Get the Pubky identity string (hex-encoded Ed25519 public key)
    pub fn pubky_identity(&self) -> String {
        hex::encode(self.ed25519_public().to_bytes())
    }

    /// Create a PubkyIdentity for storage/transmission
    pub fn to_pubky_identity(&self, name: Option<String>) -> PubkyIdentity {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let metadata = Some(IdentityMetadata {
            name,
            created_at: timestamp,
            version: 1,
        });

        PubkyIdentity {
            ed25519_pubkey: self.ed25519_public().to_bytes(),
            x25519_pubkey: self.x25519_public().to_bytes(),
            metadata,
        }
    }

    /// Export keys to bytes for secure storage
    pub fn to_bytes(&self) -> DualKeyBytes {
        DualKeyBytes {
            ed25519_key: self.ed25519_key.to_bytes(),
            x25519_key: self.x25519_key.to_bytes(),
        }
    }

    /// Import keys from bytes
    pub fn from_bytes(bytes: &DualKeyBytes) -> Result<Self> {
        let ed25519_key = SigningKey::from_bytes(&bytes.ed25519_key);
        let x25519_key = StaticSecret::from(bytes.x25519_key);

        Ok(Self {
            ed25519_key,
            x25519_key,
        })
    }

    /// Derive X25519 key from Ed25519 key (deterministic)
    ///
    /// This allows creating a deterministic X25519 key from an Ed25519 key,
    /// useful for backwards compatibility or when you only have an Ed25519 key.
    pub fn derive_x25519_from_ed25519(ed25519_key: &SigningKey) -> StaticSecret {
        use blake3::Hasher;
        
        // Create deterministic X25519 key from Ed25519 key
        let mut hasher = Hasher::new();
        hasher.update(b"TRUSTEDGE_X25519_DERIVATION");
        hasher.update(&ed25519_key.to_bytes());
        
        let hash = hasher.finalize();
        let mut x25519_bytes = [0u8; 32];
        x25519_bytes.copy_from_slice(&hash.as_bytes()[..32]);
        
        StaticSecret::from(x25519_bytes)
    }

    /// Create a DualKeyPair with derived X25519 key
    pub fn from_ed25519_with_derived_x25519(ed25519_key: SigningKey) -> Self {
        let x25519_key = Self::derive_x25519_from_ed25519(&ed25519_key);
        
        Self {
            ed25519_key,
            x25519_key,
        }
    }
}

/// Serializable byte representation of dual keys
#[derive(Serialize, Deserialize, Zeroize)]
pub struct DualKeyBytes {
    pub ed25519_key: [u8; 32],
    pub x25519_key: [u8; 32],
}

impl PubkyIdentity {
    /// Get the Pubky identity string
    pub fn pubky_id(&self) -> String {
        hex::encode(self.ed25519_pubkey)
    }

    /// Get the Ed25519 public key
    pub fn ed25519_public_key(&self) -> Result<VerifyingKey> {
        VerifyingKey::from_bytes(&self.ed25519_pubkey)
            .map_err(|e| anyhow::anyhow!("Invalid Ed25519 public key: {:?}", e))
    }

    /// Get the X25519 public key
    pub fn x25519_public_key(&self) -> X25519PublicKey {
        X25519PublicKey::from(self.x25519_pubkey)
    }

    /// Serialize to JSON
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string(self)
            .context("Failed to serialize PubkyIdentity to JSON")
    }

    /// Deserialize from JSON
    pub fn from_json(json: &str) -> Result<Self> {
        serde_json::from_str(json)
            .context("Failed to deserialize PubkyIdentity from JSON")
    }

    /// Serialize to bytes
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        bincode::serialize(self)
            .context("Failed to serialize PubkyIdentity to bytes")
    }

    /// Deserialize from bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        bincode::deserialize(bytes)
            .context("Failed to deserialize PubkyIdentity from bytes")
    }

    /// Verify this identity is valid
    pub fn verify(&self) -> bool {
        // Verify Ed25519 public key is valid
        if VerifyingKey::from_bytes(&self.ed25519_pubkey).is_err() {
            return false;
        }

        // X25519 public keys are always valid if they're 32 bytes
        if self.x25519_pubkey.len() != 32 {
            return false;
        }

        true
    }
}

impl std::fmt::Debug for DualKeyPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DualKeyPair")
            .field("ed25519_public", &self.ed25519_public())
            .field("x25519_public", &self.x25519_public())
            .finish_non_exhaustive()
    }
}

impl Drop for DualKeyPair {
    fn drop(&mut self) {
        // Note: ed25519_dalek::SigningKey doesn't implement Zeroize directly
        // The underlying key material is zeroized when the SigningKey is dropped
        // x25519_dalek::StaticSecret implements Zeroize automatically
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dual_key_generation() {
        let keys = DualKeyPair::generate();
        
        // Verify we can get both public keys
        let ed25519_pub = keys.ed25519_public();
        let _x25519_pub = keys.x25519_public();
        let pubky_id = keys.pubky_identity();

        // Verify pubky_id is hex-encoded Ed25519 public key
        assert_eq!(pubky_id, hex::encode(ed25519_pub.to_bytes()));
        assert_eq!(pubky_id.len(), 64); // 32 bytes * 2 hex chars
    }

    #[test]
    fn test_pubky_identity_creation() {
        let keys = DualKeyPair::generate();
        let identity = keys.to_pubky_identity(Some("test_user".to_string()));

        assert_eq!(identity.ed25519_pubkey, keys.ed25519_public().to_bytes());
        assert_eq!(identity.x25519_pubkey, keys.x25519_public().to_bytes());
        assert!(identity.verify());
        
        // Test serialization
        let json = identity.to_json().expect("Failed to serialize to JSON");
        let deserialized = PubkyIdentity::from_json(&json).expect("Failed to deserialize from JSON");
        
        assert_eq!(identity.pubky_id(), deserialized.pubky_id());
    }

    #[test]
    fn test_key_derivation() {
        let ed25519_key = SigningKey::generate(&mut rand::rngs::OsRng);
        
        // Derive X25519 key twice - should be deterministic
        let x25519_key1 = DualKeyPair::derive_x25519_from_ed25519(&ed25519_key);
        let x25519_key2 = DualKeyPair::derive_x25519_from_ed25519(&ed25519_key);
        
        assert_eq!(x25519_key1.to_bytes(), x25519_key2.to_bytes());
        
        // Create dual key pair with derived X25519
        let dual_keys = DualKeyPair::from_ed25519_with_derived_x25519(ed25519_key);
        let x25519_key3 = DualKeyPair::derive_x25519_from_ed25519(&dual_keys.ed25519_key);
        
        assert_eq!(dual_keys.x25519_key.to_bytes(), x25519_key3.to_bytes());
    }

    #[test]
    fn test_key_serialization() {
        let keys = DualKeyPair::generate();
        
        // Export and import keys
        let key_bytes = keys.to_bytes();
        let restored_keys = DualKeyPair::from_bytes(&key_bytes)
            .expect("Failed to restore keys from bytes");
        
        // Verify keys are identical
        assert_eq!(
            keys.ed25519_public().to_bytes(),
            restored_keys.ed25519_public().to_bytes()
        );
        assert_eq!(
            keys.x25519_public().to_bytes(),
            restored_keys.x25519_public().to_bytes()
        );
    }
}