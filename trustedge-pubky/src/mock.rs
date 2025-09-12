// Copyright (c) 2025 TRUSTEDGE LABS LLC
// MPL-2.0: https://mozilla.org/MPL/2.0/
// Project: trustedge — Privacy and trust at the edge.

//! Mock Pubky adapter for testing
//!
//! This module provides a mock implementation that doesn't require actual
//! network connectivity, useful for testing and development.

use crate::{PubkyAdapterError, PublicKeyData, TrustEdgeKeyRecord};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use trustedge_core::PublicKey;

/// Mock storage for testing
type MockStorage = Arc<Mutex<HashMap<String, String>>>;

/// Mock Pubky adapter that stores data in memory instead of the network
pub struct MockPubkyAdapter {
    /// Our mock Pubky ID
    pubky_id: String,
    /// Shared storage for all mock adapters
    storage: MockStorage,
}

impl MockPubkyAdapter {
    /// Create a new mock adapter
    pub fn new() -> Self {
        Self {
            pubky_id: hex::encode(&rand::random::<[u8; 32]>()),
            storage: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Create a mock adapter with shared storage
    pub fn with_shared_storage(storage: MockStorage) -> Self {
        Self {
            pubky_id: hex::encode(&rand::random::<[u8; 32]>()),
            storage,
        }
    }

    /// Publish a public key (stores in mock storage)
    pub async fn publish_public_key(
        &self,
        public_key: &PublicKey,
    ) -> Result<String, PubkyAdapterError> {
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

        // Store in mock storage
        let mut storage = self.storage.lock().unwrap();
        storage.insert(self.pubky_id.clone(), record_json);

        Ok(self.pubky_id.clone())
    }

    /// Resolve a public key (retrieves from mock storage)
    pub async fn resolve_public_key(&self, pubky_id: &str) -> Result<PublicKey, PubkyAdapterError> {
        let storage = self.storage.lock().unwrap();
        let record_json = storage
            .get(pubky_id)
            .ok_or_else(|| PubkyAdapterError::KeyResolutionFailed(pubky_id.to_string()))?;

        let record: TrustEdgeKeyRecord = serde_json::from_str(record_json)?;

        // Convert back to TrustEdge PublicKey
        let algorithm = match record.public_key.algorithm.as_str() {
            "Ed25519" => trustedge_core::backends::AsymmetricAlgorithm::Ed25519,
            "EcdsaP256" => trustedge_core::backends::AsymmetricAlgorithm::EcdsaP256,
            "Rsa2048" => trustedge_core::backends::AsymmetricAlgorithm::Rsa2048,
            "Rsa4096" => trustedge_core::backends::AsymmetricAlgorithm::Rsa4096,
            _ => {
                return Err(PubkyAdapterError::InvalidPubkyId(format!(
                    "Unsupported algorithm: {}",
                    record.public_key.algorithm
                )))
            }
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

    /// Get our mock Pubky ID
    pub fn our_pubky_id(&self) -> String {
        self.pubky_id.clone()
    }
}

/// Mock version of send_trusted_data for testing
pub async fn mock_send_trusted_data(
    data: &[u8],
    recipient_id: &str,
    storage: MockStorage,
) -> Result<Vec<u8>, PubkyAdapterError> {
    // Create a temporary adapter to resolve the key
    let adapter = MockPubkyAdapter::with_shared_storage(storage);
    let recipient_public_key = adapter.resolve_public_key(recipient_id).await?;

    // Use the core library function
    let sealed_envelope = trustedge_core::seal_for_recipient(data, &recipient_public_key)?;

    Ok(sealed_envelope)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::receive_trusted_data;
    use trustedge_core::{backends::AsymmetricAlgorithm, KeyPair};

    #[tokio::test]
    async fn test_mock_adapter() {
        let storage = Arc::new(Mutex::new(HashMap::new()));

        let alice_adapter = MockPubkyAdapter::with_shared_storage(storage.clone());
        let bob_adapter = MockPubkyAdapter::with_shared_storage(storage.clone());

        // Generate keys
        let alice_keypair = KeyPair::generate(AsymmetricAlgorithm::Rsa2048)
            .expect("Failed to generate Alice's key");
        let bob_keypair =
            KeyPair::generate(AsymmetricAlgorithm::Rsa2048).expect("Failed to generate Bob's key");

        // Publish keys
        let alice_pubky_id = alice_adapter
            .publish_public_key(&alice_keypair.public)
            .await
            .expect("Failed to publish Alice's key");
        let bob_pubky_id = bob_adapter
            .publish_public_key(&bob_keypair.public)
            .await
            .expect("Failed to publish Bob's key");

        // Test key resolution
        let resolved_alice = bob_adapter
            .resolve_public_key(&alice_pubky_id)
            .await
            .expect("Failed to resolve Alice's key");

        assert_eq!(alice_keypair.public.id(), resolved_alice.id());

        // Test full workflow
        let message = b"Test message via mock adapter";

        let envelope = mock_send_trusted_data(message, &bob_pubky_id, storage)
            .await
            .expect("Failed to send trusted data");

        let decrypted = receive_trusted_data(&envelope, &bob_keypair.private)
            .await
            .expect("Failed to receive trusted data");

        assert_eq!(message, decrypted.as_slice());
    }
}
