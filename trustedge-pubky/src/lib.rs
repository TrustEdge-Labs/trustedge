// Copyright (c) 2025 TRUSTEDGE LABS LLC
// MPL-2.0: https://mozilla.org/MPL/2.0/
// Project: trustedge — Privacy and trust at the edge.

//! TrustEdge Pubky Adapter
//!
//! This crate provides a clean bridge between TrustEdge core cryptographic functions
//! and the Pubky decentralized network. It maintains clean architecture by keeping
//! Pubky network logic separate from the core crypto primitives.

pub mod mock;

use anyhow::Result;
use pubky::{Client, ClientBuilder, Keypair};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use trustedge_core::{PublicKey, PrivateKey};
use trustedge_core::backends::AsymmetricAlgorithm;

/// Errors that can occur during Pubky operations
#[derive(Debug, thiserror::Error)]
pub enum PubkyAdapterError {
    #[error("Network error: {0}")]
    Network(#[from] anyhow::Error),
    
    #[error("Key resolution failed for ID: {0}")]
    KeyResolutionFailed(String),
    
    #[error("Invalid Pubky ID format: {0}")]
    InvalidPubkyId(String),
    
    #[error("TrustEdge core error: {0}")]
    CoreError(#[from] trustedge_core::TrustEdgeError),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

/// A TrustEdge public key record stored in the Pubky network
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TrustEdgeKeyRecord {
    /// The TrustEdge public key
    pub public_key: PublicKeyData,
    /// When this record was created
    pub created_at: u64,
    /// Optional metadata
    pub metadata: Option<HashMap<String, String>>,
}

/// Serializable public key data
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PublicKeyData {
    /// The algorithm used
    pub algorithm: String,
    /// The key bytes (hex-encoded)
    pub key_bytes: String,
    /// Optional key identifier
    pub key_id: Option<String>,
}

/// Client for interacting with the Pubky network
pub struct PubkyAdapter {
    /// The Pubky client
    client: Client,
    /// Our Pubky keypair
    keypair: Keypair,
}

impl PubkyAdapter {
    /// Create a new Pubky adapter
    pub async fn new(keypair: Keypair) -> Result<Self, PubkyAdapterError> {
        let client = ClientBuilder::default()
            .build()
            .map_err(|e| PubkyAdapterError::Network(anyhow::anyhow!("Failed to build Pubky client: {:?}", e)))?;

        Ok(Self { client, keypair })
    }

    /// Publish a TrustEdge public key to the Pubky network
    pub async fn publish_public_key(&self, public_key: &PublicKey) -> Result<String, PubkyAdapterError> {
        let record = TrustEdgeKeyRecord {
            public_key: PublicKeyData {
                algorithm: format!("{:?}", public_key.algorithm),
                key_bytes: hex::encode(&public_key.key_bytes),
                key_id: public_key.key_id.clone(),
            },
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            metadata: None,
        };

        let record_json = serde_json::to_string(&record)?;
        let path = "/trustedge/public_key";

        // Store the record in Pubky network
        self.client
            .put(path)
            .body(record_json.into_bytes())
            .send()
            .await
            .map_err(|e| PubkyAdapterError::Network(anyhow::anyhow!("Failed to publish key: {:?}", e)))?;

        // Return the Pubky ID
        Ok(hex::encode(self.keypair.public_key().to_bytes()))
    }

    /// Resolve a Pubky ID to get the TrustEdge public key
    pub async fn resolve_public_key(&self, pubky_id: &str) -> Result<PublicKey, PubkyAdapterError> {
        let path = "/trustedge/public_key";
        let url = format!("pubky://{}{}", pubky_id, path);

        // Retrieve the record from Pubky network
        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| PubkyAdapterError::Network(anyhow::anyhow!("Failed to resolve key: {:?}", e)))?;

        let record_bytes = response
            .bytes()
            .await
            .map_err(|e| PubkyAdapterError::Network(anyhow::anyhow!("Failed to read response: {:?}", e)))?;

        let record_str = String::from_utf8(record_bytes.to_vec())
            .map_err(|e| PubkyAdapterError::InvalidPubkyId(format!("Invalid UTF-8: {:?}", e)))?;

        let record: TrustEdgeKeyRecord = serde_json::from_str(&record_str)?;

        // Convert back to TrustEdge PublicKey
        let algorithm = match record.public_key.algorithm.as_str() {
            "Ed25519" => AsymmetricAlgorithm::Ed25519,
            "EcdsaP256" => AsymmetricAlgorithm::EcdsaP256,
            "Rsa2048" => AsymmetricAlgorithm::Rsa2048,
            "Rsa4096" => AsymmetricAlgorithm::Rsa4096,
            _ => return Err(PubkyAdapterError::InvalidPubkyId(
                format!("Unsupported algorithm: {}", record.public_key.algorithm)
            )),
        };

        let key_bytes = hex::decode(&record.public_key.key_bytes)
            .map_err(|e| PubkyAdapterError::InvalidPubkyId(format!("Invalid hex: {:?}", e)))?;

        let public_key = if let Some(key_id) = record.public_key.key_id {
            PublicKey::with_id(algorithm, key_bytes, key_id)
        } else {
            PublicKey::new(algorithm, key_bytes)
        };

        Ok(public_key)
    }

    /// Get our Pubky ID
    pub fn our_pubky_id(&self) -> String {
        hex::encode(self.keypair.public_key().to_bytes())
    }
}

/// Send trusted data to a recipient via Pubky network resolution
///
/// This is the main high-level function that:
/// 1. Uses the pubky client to resolve the ID and get the public key
/// 2. Calls the core library function to perform the hybrid encryption
pub async fn send_trusted_data(
    data: &[u8],
    recipient_id: &str, // e.g., "abc123..." (hex-encoded Pubky ID)
    pubky_adapter: &PubkyAdapter,
) -> Result<Vec<u8>, PubkyAdapterError> {
    // 1. Use the pubky client to resolve the ID and get the public key
    let recipient_public_key = pubky_adapter.resolve_public_key(recipient_id).await?;

    // 2. Call the core library function to perform the hybrid encryption
    let sealed_envelope = trustedge_core::seal_for_recipient(data, &recipient_public_key)?;

    Ok(sealed_envelope)
}

/// Receive trusted data using our private key
///
/// This function:
/// 1. Uses the core library to decrypt the envelope
/// 2. Returns the original data
pub async fn receive_trusted_data(
    envelope: &[u8],
    my_private_key: &PrivateKey,
) -> Result<Vec<u8>, PubkyAdapterError> {
    // Use the core library function to decrypt the envelope
    let decrypted_data = trustedge_core::open_envelope(envelope, my_private_key)?;

    Ok(decrypted_data)
}

/// Convenience function to create a Pubky adapter from a seed
pub async fn create_pubky_adapter_from_seed(seed: &[u8; 32]) -> Result<PubkyAdapter, PubkyAdapterError> {
    let keypair = Keypair::from_secret_key(seed);
    PubkyAdapter::new(keypair).await
}

/// Convenience function to create a Pubky adapter with a random keypair
pub async fn create_pubky_adapter_random() -> Result<PubkyAdapter, PubkyAdapterError> {
    let keypair = Keypair::random();
    PubkyAdapter::new(keypair).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use trustedge_core::{KeyPair, AsymmetricAlgorithm};

    #[test]
    fn test_public_key_serialization() {
        let keypair = KeyPair::generate(AsymmetricAlgorithm::Rsa2048)
            .expect("Failed to generate key pair");

        let record = TrustEdgeKeyRecord {
            public_key: PublicKeyData {
                algorithm: format!("{:?}", keypair.public.algorithm),
                key_bytes: hex::encode(&keypair.public.key_bytes),
                key_id: keypair.public.key_id.clone(),
            },
            created_at: 1234567890,
            metadata: None,
        };

        let json = serde_json::to_string(&record)
            .expect("Failed to serialize record");

        let deserialized: TrustEdgeKeyRecord = serde_json::from_str(&json)
            .expect("Failed to deserialize record");

        assert_eq!(record.public_key.algorithm, deserialized.public_key.algorithm);
        assert_eq!(record.public_key.key_bytes, deserialized.public_key.key_bytes);
        assert_eq!(record.created_at, deserialized.created_at);
    }

    #[tokio::test]
    async fn test_adapter_creation() {
        let adapter = create_pubky_adapter_random().await
            .expect("Failed to create adapter");

        let pubky_id = adapter.our_pubky_id();
        assert_eq!(pubky_id.len(), 64); // 32 bytes * 2 hex chars
    }

    #[tokio::test]
    async fn test_receive_trusted_data() {
        // Create a test envelope using core functions
        let alice_keypair = KeyPair::generate(AsymmetricAlgorithm::Rsa2048)
            .expect("Failed to generate Alice's key");

        let data = b"Test message for receive function";
        let envelope = trustedge_core::seal_for_recipient(data, &alice_keypair.public)
            .expect("Failed to seal envelope");

        // Test the receive function
        let decrypted = receive_trusted_data(&envelope, &alice_keypair.private).await
            .expect("Failed to receive trusted data");

        assert_eq!(data, decrypted.as_slice());
    }

    #[tokio::test]
    async fn test_mock_integration() {
        use crate::mock::{MockPubkyAdapter, mock_send_trusted_data};
        use std::sync::{Arc, Mutex};
        use std::collections::HashMap;

        let storage = Arc::new(Mutex::new(HashMap::new()));
        
        let alice_adapter = MockPubkyAdapter::with_shared_storage(storage.clone());
        let bob_adapter = MockPubkyAdapter::with_shared_storage(storage.clone());

        // Generate keys
        let alice_keypair = KeyPair::generate(AsymmetricAlgorithm::Rsa2048)
            .expect("Failed to generate Alice's key");
        let bob_keypair = KeyPair::generate(AsymmetricAlgorithm::Rsa2048)
            .expect("Failed to generate Bob's key");

        // Publish Bob's key
        let bob_pubky_id = bob_adapter.publish_public_key(&bob_keypair.public).await
            .expect("Failed to publish Bob's key");

        // Test the clean API with mock
        let message = b"Test message for mock integration";
        
        let envelope = mock_send_trusted_data(message, &bob_pubky_id, storage).await
            .expect("Failed to send trusted data");

        let decrypted = receive_trusted_data(&envelope, &bob_keypair.private).await
            .expect("Failed to receive trusted data");

        assert_eq!(message, decrypted.as_slice());
    }
}