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
use tokio::runtime::Runtime;
use trustedge_core::backends::{
    AsymmetricAlgorithm, BackendCapabilities, BackendInfo, CryptoOperation, CryptoResult,
    KeyMetadata, UniversalBackend,
};
use trustedge_core::error::BackendError;
use trustedge_core::{PrivateKey, PublicKey};

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
    CoreError(#[from] trustedge_core::HybridEncryptionError),

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

/// Backend for Pubky network operations implementing UniversalBackend
pub struct PubkyBackend {
    /// The Pubky client
    client: Client,
    /// Our Pubky keypair
    keypair: Keypair,
    /// Async runtime for network operations
    runtime: Runtime,
}

impl PubkyBackend {
    /// Create a new Pubky backend
    pub async fn new(keypair: Keypair) -> Result<Self, PubkyAdapterError> {
        let client = ClientBuilder::default().build().map_err(|e| {
            PubkyAdapterError::Network(anyhow::anyhow!("Failed to build Pubky client: {:?}", e))
        })?;

        let runtime = Runtime::new().map_err(|e| {
            PubkyAdapterError::Network(anyhow::anyhow!("Failed to create async runtime: {:?}", e))
        })?;

        Ok(Self {
            client,
            keypair,
            runtime,
        })
    }

    /// Create a new Pubky backend synchronously
    pub fn new_sync(keypair: Keypair) -> Result<Self, PubkyAdapterError> {
        let runtime = Runtime::new().map_err(|e| {
            PubkyAdapterError::Network(anyhow::anyhow!("Failed to create async runtime: {:?}", e))
        })?;

        let client = runtime.block_on(async {
            ClientBuilder::default().build().map_err(|e| {
                PubkyAdapterError::Network(anyhow::anyhow!("Failed to build Pubky client: {:?}", e))
            })
        })?;

        Ok(Self {
            client,
            keypair,
            runtime,
        })
    }

    /// Publish a TrustEdge public key to the Pubky network
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
        let path = "/trustedge/public_key";

        // Store the record in Pubky network
        self.client
            .put(path)
            .body(record_json.into_bytes())
            .send()
            .await
            .map_err(|e| {
                PubkyAdapterError::Network(anyhow::anyhow!("Failed to publish key: {:?}", e))
            })?;

        // Return the Pubky ID
        Ok(hex::encode(self.keypair.public_key().to_bytes()))
    }

    /// Resolve a Pubky ID to get the TrustEdge public key (async)
    pub async fn resolve_public_key(&self, pubky_id: &str) -> Result<PublicKey, PubkyAdapterError> {
        let path = "/trustedge/public_key";
        let url = format!("pubky://{}{}", pubky_id, path);

        // Retrieve the record from Pubky network
        let response = self.client.get(&url).send().await.map_err(|e| {
            PubkyAdapterError::Network(anyhow::anyhow!("Failed to resolve key: {:?}", e))
        })?;

        let record_bytes = response.bytes().await.map_err(|e| {
            PubkyAdapterError::Network(anyhow::anyhow!("Failed to read response: {:?}", e))
        })?;

        let record_str = String::from_utf8(record_bytes.to_vec())
            .map_err(|e| PubkyAdapterError::InvalidPubkyId(format!("Invalid UTF-8: {:?}", e)))?;

        let record: TrustEdgeKeyRecord = serde_json::from_str(&record_str)?;

        // Convert back to TrustEdge PublicKey
        let algorithm = match record.public_key.algorithm.as_str() {
            "Ed25519" => AsymmetricAlgorithm::Ed25519,
            "EcdsaP256" => AsymmetricAlgorithm::EcdsaP256,
            "Rsa2048" => AsymmetricAlgorithm::Rsa2048,
            "Rsa4096" => AsymmetricAlgorithm::Rsa4096,
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

    /// Resolve a Pubky ID to get the TrustEdge public key (sync)
    pub fn resolve_public_key_sync(&self, pubky_id: &str) -> Result<PublicKey, PubkyAdapterError> {
        self.runtime.block_on(self.resolve_public_key(pubky_id))
    }

    /// Get our Pubky ID
    pub fn our_pubky_id(&self) -> String {
        hex::encode(self.keypair.public_key().to_bytes())
    }
}

impl UniversalBackend for PubkyBackend {
    fn perform_operation(
        &self,
        key_id: &str,
        operation: CryptoOperation,
    ) -> Result<CryptoResult, BackendError> {
        match operation {
            CryptoOperation::GetPublicKey => {
                // key_id is the Pubky ID
                let public_key = self.resolve_public_key_sync(key_id).map_err(|e| {
                    BackendError::KeyNotFound(format!(
                        "Failed to resolve Pubky ID {}: {}",
                        key_id, e
                    ))
                })?;
                Ok(CryptoResult::PublicKey(public_key.key_bytes))
            }
            _ => Err(BackendError::UnsupportedOperation(format!(
                "Operation not supported by PubkyBackend: {:?}",
                operation
            ))),
        }
    }

    fn supports_operation(&self, operation: &CryptoOperation) -> bool {
        matches!(operation, CryptoOperation::GetPublicKey)
    }

    fn get_capabilities(&self) -> BackendCapabilities {
        BackendCapabilities {
            symmetric_algorithms: vec![],
            asymmetric_algorithms: vec![
                AsymmetricAlgorithm::Ed25519,
                AsymmetricAlgorithm::EcdsaP256,
                AsymmetricAlgorithm::Rsa2048,
                AsymmetricAlgorithm::Rsa4096,
            ],
            signature_algorithms: vec![],
            hash_algorithms: vec![],
            hardware_backed: false,
            supports_key_derivation: false,
            supports_key_generation: false,
            supports_attestation: false,
            max_key_size: Some(4096),
        }
    }

    fn backend_info(&self) -> BackendInfo {
        BackendInfo {
            name: "pubky",
            description: "Decentralized key resolution via Pubky network",
            version: "0.1.0",
            available: true,
            config_requirements: vec!["pubky_keypair"],
        }
    }

    fn list_keys(&self) -> Result<Vec<KeyMetadata>, BackendError> {
        // Pubky backend doesn't enumerate keys - they're resolved by ID
        Ok(vec![])
    }
}

/// Send trusted data to a recipient via Pubky network resolution
///
/// This is the main high-level function that:
/// 1. Uses the pubky backend to resolve the ID and get the public key
/// 2. Calls the core library function to perform the hybrid encryption
pub fn send_trusted_data(
    data: &[u8],
    recipient_id: &str, // e.g., "abc123..." (hex-encoded Pubky ID)
    pubky_backend: &PubkyBackend,
) -> Result<Vec<u8>, PubkyAdapterError> {
    // 1. Use the pubky backend to resolve the ID and get the public key
    let recipient_public_key = pubky_backend.resolve_public_key_sync(recipient_id)?;

    // 2. Call the core library function to perform the hybrid encryption
    let sealed_envelope = trustedge_core::seal_for_recipient(data, &recipient_public_key)?;

    Ok(sealed_envelope)
}

/// Receive trusted data using our private key
///
/// This function:
/// 1. Uses the core library to decrypt the envelope
/// 2. Returns the original data
pub fn receive_trusted_data(
    envelope: &[u8],
    my_private_key: &PrivateKey,
) -> Result<Vec<u8>, PubkyAdapterError> {
    // Use the core library function to decrypt the envelope
    let decrypted_data = trustedge_core::open_envelope(envelope, my_private_key)?;

    Ok(decrypted_data)
}

/// Convenience function to create a Pubky backend from a seed
pub fn create_pubky_backend_from_seed(seed: &[u8; 32]) -> Result<PubkyBackend, PubkyAdapterError> {
    let keypair = Keypair::from_secret_key(seed);
    PubkyBackend::new_sync(keypair)
}

/// Extract the private key seed from a PubkyBackend
/// This is needed for key export functionality
pub fn extract_private_key_seed(backend: &PubkyBackend) -> [u8; 32] {
    // This is a temporary implementation - in a real system,
    // private keys should be handled more securely
    backend.keypair.secret_key()
}

/// Convenience function to create a Pubky backend with a random keypair
pub fn create_pubky_backend_random() -> Result<PubkyBackend, PubkyAdapterError> {
    let keypair = Keypair::random();
    PubkyBackend::new_sync(keypair)
}

#[cfg(test)]
mod tests {
    use super::*;
    use trustedge_core::{AsymmetricAlgorithm, KeyPair};

    #[test]
    fn test_public_key_serialization() {
        let keypair =
            KeyPair::generate(AsymmetricAlgorithm::Rsa2048).expect("Failed to generate key pair");

        // Use current timestamp instead of fake data
        let current_timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        let record = TrustEdgeKeyRecord {
            public_key: PublicKeyData {
                algorithm: format!("{:?}", keypair.public.algorithm),
                key_bytes: hex::encode(&keypair.public.key_bytes),
                key_id: keypair.public.key_id.clone(),
            },
            created_at: current_timestamp,
            metadata: None,
        };

        let json = serde_json::to_string(&record).expect("Failed to serialize record");

        let deserialized: TrustEdgeKeyRecord =
            serde_json::from_str(&json).expect("Failed to deserialize record");

        // Verify all fields are preserved correctly
        assert_eq!(
            record.public_key.algorithm,
            deserialized.public_key.algorithm
        );
        assert_eq!(
            record.public_key.key_bytes,
            deserialized.public_key.key_bytes
        );
        assert_eq!(record.public_key.key_id, deserialized.public_key.key_id);
        assert_eq!(record.created_at, deserialized.created_at);
        assert_eq!(record.metadata, deserialized.metadata);

        // Verify the algorithm string is correct
        assert_eq!(record.public_key.algorithm, "Rsa2048");

        // Verify the hex encoding is valid
        let decoded_bytes =
            hex::decode(&record.public_key.key_bytes).expect("Key bytes should be valid hex");
        assert_eq!(decoded_bytes, keypair.public.key_bytes);

        // Verify timestamp is reasonable (within last hour and not in future)
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        assert!(record.created_at <= now);
        assert!(record.created_at > now - 3600); // Within last hour
    }

    #[test]
    fn test_backend_creation() {
        let backend = create_pubky_backend_random().expect("Failed to create backend");

        let pubky_id = backend.our_pubky_id();

        // Verify Pubky ID format
        assert_eq!(pubky_id.len(), 64); // 32 bytes * 2 hex chars
        assert!(
            pubky_id.chars().all(|c| c.is_ascii_hexdigit()),
            "Pubky ID should be valid hex: {}",
            pubky_id
        );

        // Verify backend capabilities
        let capabilities = backend.get_capabilities();
        assert!(!capabilities.hardware_backed);
        assert!(!capabilities.supports_key_derivation);
        assert!(!capabilities.supports_key_generation);
        assert!(!capabilities.supports_attestation);

        // Verify backend info
        let info = backend.backend_info();
        assert_eq!(info.name, "pubky");
        assert!(info.available);
        assert!(info.description.contains("Pubky network"));

        // Test that multiple backends generate different IDs
        let backend2 = create_pubky_backend_random().expect("Failed to create second backend");
        let pubky_id2 = backend2.our_pubky_id();
        assert_ne!(
            pubky_id, pubky_id2,
            "Random backends should generate different IDs"
        );
    }

    #[test]
    fn test_receive_trusted_data() {
        // Test with algorithms supported by hybrid encryption (excluding RSA4096 due to slow key generation)
        let algorithms = vec![AsymmetricAlgorithm::Rsa2048];

        for &algorithm in &algorithms {
            // Create a test envelope using core functions
            let alice_keypair = KeyPair::generate(algorithm).unwrap_or_else(|_| {
                panic!(
                    "Failed to generate Alice's key for algorithm {:?}",
                    algorithm
                )
            });

            // Test with various data sizes and types
            let test_cases = [
                b"".to_vec(),                                            // Empty data
                b"A".to_vec(),                                           // Single byte
                b"Test message for receive function".to_vec(),           // Text
                (0..1000).map(|i| (i % 256) as u8).collect::<Vec<u8>>(), // Binary data
                serde_json::to_vec(&serde_json::json!({
                    "test": "data",
                    "number": 42,
                    "array": [1, 2, 3]
                }))
                .unwrap(), // JSON data
            ];

            for (i, data) in test_cases.iter().enumerate() {
                let envelope = trustedge_core::seal_for_recipient(data, &alice_keypair.public)
                    .unwrap_or_else(|_| panic!("Failed to seal envelope for case {}", i));

                // Verify envelope is not empty and different from original data
                assert!(!envelope.is_empty(), "Envelope should not be empty");
                assert_ne!(
                    envelope, *data,
                    "Envelope should be different from original data"
                );

                // Test the receive function
                let decrypted = receive_trusted_data(&envelope, &alice_keypair.private)
                    .unwrap_or_else(|_| panic!("Failed to receive trusted data for case {}", i));

                assert_eq!(
                    data, &decrypted,
                    "Decrypted data should match original for case {}",
                    i
                );
            }
        }
    }

    #[test]
    fn test_mock_integration() {
        use crate::mock::{mock_send_trusted_data, MockPubkyBackend};
        use std::collections::HashMap;
        use std::sync::{Arc, Mutex};

        let storage = Arc::new(Mutex::new(HashMap::new()));

        let _alice_backend = MockPubkyBackend::with_shared_storage(storage.clone());
        let bob_backend = MockPubkyBackend::with_shared_storage(storage.clone());

        // Generate keys
        let _alice_keypair = KeyPair::generate(AsymmetricAlgorithm::Rsa2048)
            .expect("Failed to generate Alice's key");
        let bob_keypair =
            KeyPair::generate(AsymmetricAlgorithm::Rsa2048).expect("Failed to generate Bob's key");

        // Publish Bob's key
        let bob_pubky_id = bob_backend
            .publish_public_key(&bob_keypair.public)
            .expect("Failed to publish Bob's key");

        // Test the clean API with mock
        let message = b"Test message for mock integration";

        let envelope = mock_send_trusted_data(message, &bob_pubky_id, storage)
            .expect("Failed to send trusted data");

        let decrypted = receive_trusted_data(&envelope, &bob_keypair.private)
            .expect("Failed to receive trusted data");

        assert_eq!(message, decrypted.as_slice());
    }

    #[test]
    fn test_deterministic_key_generation() {
        let seed = [0x42; 32]; // Fixed seed for deterministic testing

        // Create two backends with the same seed
        let backend1 =
            create_pubky_backend_from_seed(&seed).expect("Failed to create first backend");
        let backend2 =
            create_pubky_backend_from_seed(&seed).expect("Failed to create second backend");

        // Should produce identical Pubky IDs
        let id1 = backend1.our_pubky_id();
        let id2 = backend2.our_pubky_id();
        assert_eq!(id1, id2, "Same seed should produce same Pubky ID");

        // Verify the ID format
        assert_eq!(id1.len(), 64);
        assert!(id1.chars().all(|c| c.is_ascii_hexdigit()));

        // Test with different seed produces different ID
        let different_seed = [0x24; 32];
        let backend3 = create_pubky_backend_from_seed(&different_seed)
            .expect("Failed to create third backend");
        let id3 = backend3.our_pubky_id();
        assert_ne!(
            id1, id3,
            "Different seeds should produce different Pubky IDs"
        );
    }

    #[test]
    fn test_extract_private_key_seed() {
        let original_seed = [0x13; 32];
        let backend =
            create_pubky_backend_from_seed(&original_seed).expect("Failed to create backend");

        let extracted_seed = extract_private_key_seed(&backend);
        assert_eq!(
            original_seed, extracted_seed,
            "Extracted seed should match original"
        );

        // Verify we can recreate the same backend
        let recreated_backend =
            create_pubky_backend_from_seed(&extracted_seed).expect("Failed to recreate backend");

        assert_eq!(
            backend.our_pubky_id(),
            recreated_backend.our_pubky_id(),
            "Recreated backend should have same Pubky ID"
        );
    }
}
