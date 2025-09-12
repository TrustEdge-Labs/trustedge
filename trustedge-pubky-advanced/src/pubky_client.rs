// Copyright (c) 2025 TRUSTEDGE LABS LLC
// MPL-2.0: https://mozilla.org/MPL/2.0/
// Project: trustedge — Privacy and trust at the edge.

//! Pubky Client for TrustEdge Integration
//!
//! This module provides a client for interacting with the Pubky network
//! to store and retrieve public keys for decentralized key discovery.

use crate::keys::PubkyIdentity;
use anyhow::{Context, Result};
use pubky::{Client, ClientBuilder, Keypair, PublicKey};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Errors that can occur during Pubky operations
#[derive(Debug, thiserror::Error)]
pub enum PubkyError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Pubky client error: {0}")]
    Client(String),
    
    #[error("Identity not found: {0}")]
    IdentityNotFound(String),
    
    #[error("Invalid identity format: {0}")]
    InvalidIdentity(String),
}

/// Client for interacting with the Pubky network
pub struct PubkyClient {
    /// The underlying Pubky client
    client: Client,
    /// Our keypair for signing operations
    keypair: Keypair,
}

/// Record stored in Pubky for TrustEdge identities
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TrustEdgeIdentityRecord {
    /// The TrustEdge identity (dual keys)
    pub identity: PubkyIdentity,
    /// Timestamp when this record was created
    pub created_at: u64,
    /// Version of the record format
    pub version: u8,
    /// Optional additional metadata
    pub metadata: Option<HashMap<String, String>>,
}

impl PubkyClient {
    /// Create a new Pubky client
    pub async fn new(keypair: Keypair) -> Result<Self> {
        let client = ClientBuilder::default()
            .build()
            .map_err(|e| PubkyError::Client(format!("Failed to build client: {:?}", e)))?;

        Ok(Self { client, keypair })
    }

    /// Create a client with custom configuration
    pub async fn with_config(keypair: Keypair, homeserver: Option<String>) -> Result<Self> {
        let builder = ClientBuilder::default();
        
        if let Some(_homeserver) = homeserver {
            // Note: This is a placeholder - actual Pubky client configuration may differ
            // builder = builder.homeserver(homeserver);
        }

        let client = builder
            .build()
            .map_err(|e| PubkyError::Client(format!("Failed to build client: {:?}", e)))?;

        Ok(Self { client, keypair })
    }

    /// Publish a TrustEdge identity to the Pubky network
    pub async fn publish_identity(&self, identity: &PubkyIdentity) -> Result<String> {
        let record = TrustEdgeIdentityRecord {
            identity: identity.clone(),
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            version: 1,
            metadata: None,
        };

        let record_json = serde_json::to_string(&record)
            .context("Failed to serialize identity record")?;

        // Store the identity record at a well-known path
        let path = format!("/trustedge/identity");
        
        // Use Pubky client to store the record
        // Note: This is a simplified version - actual Pubky API may differ
        let record_bytes = record_json.into_bytes();
        self.client
            .put(&path)
            .body(record_bytes)
            .send()
            .await
            .map_err(|e| PubkyError::Client(format!("Failed to publish identity: {:?}", e)))?;

        Ok(identity.pubky_id())
    }

    /// Retrieve a TrustEdge identity from the Pubky network
    pub async fn get_identity(&self, pubky_id: &str) -> Result<PubkyIdentity> {
        // Parse the pubky ID to get the public key
        let _public_key = PublicKey::try_from(pubky_id)
            .map_err(|e| PubkyError::InvalidIdentity(format!("Invalid pubky ID: {:?}", e)))?;

        // Construct the path for the identity record
        let path = format!("/trustedge/identity");

        // Retrieve the record from the network
        let url = format!("pubky://{}{}", pubky_id, path);
        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| PubkyError::Client(format!("Failed to retrieve identity: {:?}", e)))?;
            
        let record_bytes = response
            .bytes()
            .await
            .map_err(|e| PubkyError::Client(format!("Failed to read response: {:?}", e)))?;

        // Parse the record
        let record_str = String::from_utf8(record_bytes.to_vec())
            .map_err(|e| PubkyError::InvalidIdentity(format!("Invalid UTF-8: {:?}", e)))?;

        let record: TrustEdgeIdentityRecord = serde_json::from_str(&record_str)
            .context("Failed to deserialize identity record")?;

        // Verify the identity is valid
        if !record.identity.verify() {
            return Err(PubkyError::InvalidIdentity("Identity verification failed".to_string()).into());
        }

        Ok(record.identity)
    }

    /// Update an existing identity record
    pub async fn update_identity(&self, identity: &PubkyIdentity, metadata: Option<HashMap<String, String>>) -> Result<()> {
        let record = TrustEdgeIdentityRecord {
            identity: identity.clone(),
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            version: 1,
            metadata,
        };

        let record_json = serde_json::to_string(&record)
            .context("Failed to serialize identity record")?;

        let path = format!("/trustedge/identity");
        let record_bytes = record_json.into_bytes();
        
        self.client
            .put(&path)
            .body(record_bytes)
            .send()
            .await
            .map_err(|e| PubkyError::Client(format!("Failed to update identity: {:?}", e)))?;

        Ok(())
    }

    /// Delete an identity record
    pub async fn delete_identity(&self) -> Result<()> {
        let path = format!("/trustedge/identity");
        
        self.client
            .delete(&path)
            .send()
            .await
            .map_err(|e| PubkyError::Client(format!("Failed to delete identity: {:?}", e)))?;

        Ok(())
    }

    /// List all TrustEdge identities we can discover
    pub async fn discover_identities(&self, limit: Option<usize>) -> Result<Vec<PubkyIdentity>> {
        // This is a placeholder implementation
        // In practice, this would involve querying the Pubky network for TrustEdge identity records
        // The actual implementation would depend on Pubky's discovery mechanisms
        
        let _limit = limit.unwrap_or(100);
        
        // For now, return empty list
        // TODO: Implement actual discovery logic based on Pubky capabilities
        Ok(Vec::new())
    }

    /// Get our own public key
    pub fn public_key(&self) -> PublicKey {
        self.keypair.public_key()
    }

    /// Get our Pubky ID
    pub fn pubky_id(&self) -> String {
        hex::encode(self.keypair.public_key().to_bytes())
    }

    /// Resolve a Pubky ID to get the X25519 public key for encryption
    pub async fn resolve_encryption_key(&self, pubky_id: &str) -> Result<x25519_dalek::PublicKey> {
        let identity = self.get_identity(pubky_id).await?;
        Ok(identity.x25519_public_key())
    }

    /// Batch resolve multiple Pubky IDs
    pub async fn batch_resolve_encryption_keys(&self, pubky_ids: &[String]) -> Result<HashMap<String, x25519_dalek::PublicKey>> {
        let mut results = HashMap::new();
        
        // TODO: Implement actual batch resolution for efficiency
        // For now, resolve one by one
        for pubky_id in pubky_ids {
            match self.resolve_encryption_key(pubky_id).await {
                Ok(key) => {
                    results.insert(pubky_id.clone(), key);
                }
                Err(e) => {
                    // Log error but continue with other IDs
                    eprintln!("Failed to resolve {}: {:?}", pubky_id, e);
                }
            }
        }
        
        Ok(results)
    }
}

/// Helper functions for working with Pubky identities
impl TrustEdgeIdentityRecord {
    /// Create a new identity record
    pub fn new(identity: PubkyIdentity) -> Self {
        Self {
            identity,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            version: 1,
            metadata: None,
        }
    }

    /// Add metadata to the record
    pub fn with_metadata(mut self, metadata: HashMap<String, String>) -> Self {
        self.metadata = Some(metadata);
        self
    }

    /// Get the age of this record in seconds
    pub fn age_seconds(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
            .saturating_sub(self.created_at)
    }

    /// Check if this record is expired (older than given seconds)
    pub fn is_expired(&self, max_age_seconds: u64) -> bool {
        self.age_seconds() > max_age_seconds
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::keys::DualKeyPair;

    #[tokio::test]
    async fn test_identity_record_creation() {
        let keys = DualKeyPair::generate();
        let identity = keys.to_pubky_identity(Some("test_user".to_string()));
        
        let record = TrustEdgeIdentityRecord::new(identity.clone());
        
        assert_eq!(record.identity.pubky_id(), identity.pubky_id());
        assert_eq!(record.version, 1);
        assert!(record.metadata.is_none());
        
        // Test with metadata
        let mut metadata = HashMap::new();
        metadata.insert("role".to_string(), "admin".to_string());
        
        let record_with_metadata = TrustEdgeIdentityRecord::new(identity)
            .with_metadata(metadata.clone());
        
        assert_eq!(record_with_metadata.metadata, Some(metadata));
    }

    #[test]
    fn test_record_serialization() {
        let keys = DualKeyPair::generate();
        let identity = keys.to_pubky_identity(Some("test_user".to_string()));
        let record = TrustEdgeIdentityRecord::new(identity);
        
        // Test JSON serialization
        let json = serde_json::to_string(&record).expect("Failed to serialize");
        let deserialized: TrustEdgeIdentityRecord = serde_json::from_str(&json)
            .expect("Failed to deserialize");
        
        assert_eq!(record.identity.pubky_id(), deserialized.identity.pubky_id());
        assert_eq!(record.version, deserialized.version);
    }

    #[test]
    fn test_record_expiration() {
        let keys = DualKeyPair::generate();
        let identity = keys.to_pubky_identity(Some("test_user".to_string()));
        let record = TrustEdgeIdentityRecord::new(identity);
        
        // Fresh record should not be expired
        assert!(!record.is_expired(3600)); // 1 hour
        
        // Test age calculation
        assert!(record.age_seconds() < 10); // Should be very recent
    }
}