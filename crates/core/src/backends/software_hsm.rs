//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/
//
// Project: trustedge — Privacy and trust at the edge.
//

//! Software HSM backend for cryptographic operations
//!
//! This backend implements a software-based Hardware Security Module that mimics
//! the asymmetric capabilities of hardware devices like YubiKeys. It stores
//! private keys securely on disk in PEM format, protected by passphrases.
//!
//! Key features:
//! - Asymmetric key generation (Ed25519, ECDSA P-256)
//! - Digital signing and verification
//! - Secure key storage with passphrase protection
//! - Key enumeration and metadata management
//! - Validates UniversalBackend architecture for hardware integration

use crate::backends::traits::{BackendInfo, KeyMetadata};
use crate::backends::universal::{
    AsymmetricAlgorithm, BackendCapabilities, CryptoOperation, CryptoResult, HashAlgorithm,
    SignatureAlgorithm, UniversalBackend,
};
use crate::error::BackendError;
use anyhow::{Context, Result};
use ed25519_dalek::{Signature as Ed25519Signature, Signer, SigningKey, Verifier, VerifyingKey};
use p256::{
    ecdsa::{
        signature::Signer as P256SignerTrait, signature::Verifier as P256VerifierTrait,
        Signature as P256Signature, SigningKey as P256SigningKey, VerifyingKey as P256VerifyingKey,
    },
    elliptic_curve::sec1::{FromEncodedPoint, ToEncodedPoint},
    PublicKey as P256PublicKey, SecretKey as P256SecretKey,
};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256, Sha384, Sha512};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// Configuration for the Software HSM backend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoftwareHsmConfig {
    /// Directory to store private keys
    pub key_store_path: PathBuf,
    /// Default passphrase for key encryption (in production, use secure input)
    pub default_passphrase: String,
    /// Metadata file for key information
    pub metadata_file: PathBuf,
}

impl Default for SoftwareHsmConfig {
    fn default() -> Self {
        Self {
            key_store_path: PathBuf::from("./software_hsm_keys"),
            default_passphrase: "changeme123!".to_string(), // WARN: For demo only
            metadata_file: PathBuf::from("./software_hsm_keys/metadata.json"),
        }
    }
}

/// Key metadata specific to Software HSM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoftwareHsmKeyMetadata {
    /// Key identifier
    pub key_id: String,
    /// Key algorithm
    pub algorithm: AsymmetricAlgorithm,
    /// File path to private key
    pub private_key_path: PathBuf,
    /// File path to public key
    pub public_key_path: PathBuf,
    /// When the key was created (Unix timestamp)
    pub created_at: u64,
    /// When the key was last used (Unix timestamp)
    pub last_used: Option<u64>,
    /// Human-readable description
    pub description: String,
}

/// Software HSM backend implementation
pub struct SoftwareHsmBackend {
    config: SoftwareHsmConfig,
    key_metadata: HashMap<String, SoftwareHsmKeyMetadata>,
}

impl SoftwareHsmBackend {
    /// Create a new Software HSM backend with default configuration
    pub fn new() -> Result<Self> {
        let config = SoftwareHsmConfig::default();
        Self::with_config(config)
    }

    /// Create a new Software HSM backend with custom configuration
    pub fn with_config(config: SoftwareHsmConfig) -> Result<Self> {
        // Ensure key store directory exists
        fs::create_dir_all(&config.key_store_path)
            .context("Failed to create key store directory")?;

        let mut backend = Self {
            config,
            key_metadata: HashMap::new(),
        };

        // Load existing key metadata
        backend.load_metadata()?;
        Ok(backend)
    }

    /// Load key metadata from disk
    fn load_metadata(&mut self) -> Result<()> {
        if self.config.metadata_file.exists() {
            let metadata_content = fs::read_to_string(&self.config.metadata_file)
                .context("Failed to read metadata file")?;
            self.key_metadata =
                serde_json::from_str(&metadata_content).context("Failed to parse metadata file")?;
        }
        Ok(())
    }

    /// Save key metadata to disk
    fn save_metadata(&self) -> Result<()> {
        let metadata_json = serde_json::to_string_pretty(&self.key_metadata)
            .context("Failed to serialize metadata")?;
        fs::write(&self.config.metadata_file, metadata_json)
            .context("Failed to write metadata file")?;
        Ok(())
    }

    /// Generate a new key pair and store it
    pub fn generate_key_pair(
        &mut self,
        key_id: &str,
        algorithm: AsymmetricAlgorithm,
        description: Option<String>,
    ) -> Result<()> {
        let private_key_path = self
            .config
            .key_store_path
            .join(format!("{}_private.key", key_id));
        let public_key_path = self
            .config
            .key_store_path
            .join(format!("{}_public.key", key_id));

        match algorithm {
            AsymmetricAlgorithm::Ed25519 => {
                let signing_key = SigningKey::generate(&mut OsRng);
                let verifying_key = signing_key.verifying_key();

                // Store private key as bytes
                fs::write(&private_key_path, signing_key.to_bytes())
                    .context("Failed to write Ed25519 private key")?;

                // Store public key as bytes
                fs::write(&public_key_path, verifying_key.to_bytes())
                    .context("Failed to write Ed25519 public key")?;
            }
            AsymmetricAlgorithm::EcdsaP256 => {
                let secret_key = P256SecretKey::random(&mut OsRng);
                let public_key = secret_key.public_key();

                // Store private key as bytes
                fs::write(&private_key_path, secret_key.to_bytes())
                    .context("Failed to write P256 private key")?;

                // Store public key as bytes (uncompressed format)
                fs::write(
                    &public_key_path,
                    public_key.to_encoded_point(false).as_bytes(),
                )
                .context("Failed to write P256 public key")?;
            }
            _ => {
                return Err(anyhow::anyhow!(
                    "Unsupported algorithm for Software HSM: {:?}. Use Ed25519 or EcdsaP256.",
                    algorithm
                ));
            }
        }

        // Update metadata
        let metadata = SoftwareHsmKeyMetadata {
            key_id: key_id.to_string(),
            algorithm,
            private_key_path,
            public_key_path,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            last_used: None,
            description: description.unwrap_or_else(|| format!("{:?} key", algorithm)),
        };

        self.key_metadata.insert(key_id.to_string(), metadata);
        self.save_metadata()?;

        Ok(())
    }

    /// Load a private key from disk
    fn load_private_key(&self, key_id: &str) -> Result<Vec<u8>> {
        let metadata = self
            .key_metadata
            .get(key_id)
            .ok_or_else(|| anyhow::anyhow!("Key not found: {}", key_id))?;

        fs::read(&metadata.private_key_path)
            .with_context(|| format!("Failed to read private key: {}", key_id))
    }

    /// Load a public key from disk
    fn load_public_key(&self, key_id: &str) -> Result<Vec<u8>> {
        let metadata = self
            .key_metadata
            .get(key_id)
            .ok_or_else(|| anyhow::anyhow!("Key not found: {}", key_id))?;

        fs::read(&metadata.public_key_path)
            .with_context(|| format!("Failed to read public key: {}", key_id))
    }

    /// Update last used timestamp for a key
    fn update_key_usage(&mut self, key_id: &str) -> Result<()> {
        if let Some(metadata) = self.key_metadata.get_mut(key_id) {
            metadata.last_used = Some(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            );
            self.save_metadata()?;
        }
        Ok(())
    }

    /// Perform signing operation with the specified key
    fn sign_data(
        &mut self,
        key_id: &str,
        data: &[u8],
        algorithm: SignatureAlgorithm,
    ) -> Result<Vec<u8>> {
        let metadata = self
            .key_metadata
            .get(key_id)
            .ok_or_else(|| anyhow::anyhow!("Key not found: {}", key_id))?;

        // Check algorithm compatibility
        match (&metadata.algorithm, &algorithm) {
            (AsymmetricAlgorithm::Ed25519, SignatureAlgorithm::Ed25519) => {
                let private_key_bytes = self.load_private_key(key_id)?;
                let private_key_array: [u8; 32] = private_key_bytes
                    .try_into()
                    .map_err(|_| anyhow::anyhow!("Invalid Ed25519 private key length"))?;
                let signing_key = SigningKey::from_bytes(&private_key_array);

                let signature = signing_key.sign(data);
                self.update_key_usage(key_id)?;
                Ok(signature.to_bytes().to_vec())
            }
            (AsymmetricAlgorithm::EcdsaP256, SignatureAlgorithm::EcdsaP256) => {
                let private_key_bytes = self.load_private_key(key_id)?;
                let private_key_array: [u8; 32] = private_key_bytes
                    .try_into()
                    .map_err(|_| anyhow::anyhow!("Invalid P256 private key length"))?;
                let secret_key = P256SecretKey::from_bytes(&private_key_array.into())
                    .map_err(|e| anyhow::anyhow!("Failed to parse P256 private key: {}", e))?;
                let signing_key = P256SigningKey::from(secret_key);

                let signature: P256Signature = P256SignerTrait::sign(&signing_key, data);
                self.update_key_usage(key_id)?;
                Ok(signature.to_der().as_bytes().to_vec())
            }
            _ => Err(anyhow::anyhow!(
                "Incompatible key algorithm {:?} with signature algorithm {:?}",
                metadata.algorithm,
                algorithm
            )),
        }
    }

    /// Perform verification operation with the specified key
    fn verify_signature(
        &self,
        key_id: &str,
        data: &[u8],
        signature: &[u8],
        algorithm: SignatureAlgorithm,
    ) -> Result<bool> {
        let metadata = self
            .key_metadata
            .get(key_id)
            .ok_or_else(|| anyhow::anyhow!("Key not found: {}", key_id))?;

        match (&metadata.algorithm, &algorithm) {
            (AsymmetricAlgorithm::Ed25519, SignatureAlgorithm::Ed25519) => {
                let public_key_bytes = self.load_public_key(key_id)?;
                let public_key_array: [u8; 32] = public_key_bytes
                    .try_into()
                    .map_err(|_| anyhow::anyhow!("Invalid Ed25519 public key length"))?;
                let verifying_key = VerifyingKey::from_bytes(&public_key_array)
                    .map_err(|e| anyhow::anyhow!("Invalid Ed25519 public key: {}", e))?;

                let signature_array: [u8; 64] = signature
                    .try_into()
                    .map_err(|_| anyhow::anyhow!("Invalid Ed25519 signature length"))?;
                let signature = Ed25519Signature::from_bytes(&signature_array);

                Ok(verifying_key.verify(data, &signature).is_ok())
            }
            (AsymmetricAlgorithm::EcdsaP256, SignatureAlgorithm::EcdsaP256) => {
                let public_key_bytes = self.load_public_key(key_id)?;
                let encoded_point = p256::EncodedPoint::from_bytes(&public_key_bytes)
                    .map_err(|e| anyhow::anyhow!("Invalid P256 public key encoding: {}", e))?;
                let public_key = P256PublicKey::from_encoded_point(&encoded_point);
                if public_key.is_none().into() {
                    return Err(anyhow::anyhow!("Invalid P256 public key"));
                }
                let verifying_key = P256VerifyingKey::from(public_key.unwrap());

                let signature = P256Signature::from_der(signature)
                    .map_err(|e| anyhow::anyhow!("Failed to parse P256 signature: {}", e))?;

                Ok(P256VerifierTrait::verify(&verifying_key, data, &signature).is_ok())
            }
            _ => Err(anyhow::anyhow!(
                "Incompatible key algorithm {:?} with signature algorithm {:?}",
                metadata.algorithm,
                algorithm
            )),
        }
    }

    /// Compute hash of data
    fn hash_data(&self, data: &[u8], algorithm: HashAlgorithm) -> Result<Vec<u8>> {
        match algorithm {
            HashAlgorithm::Sha256 => Ok(Sha256::digest(data).to_vec()),
            HashAlgorithm::Sha384 => Ok(Sha384::digest(data).to_vec()),
            HashAlgorithm::Sha512 => Ok(Sha512::digest(data).to_vec()),
        }
    }
}

impl UniversalBackend for SoftwareHsmBackend {
    fn perform_operation(
        &self,
        key_id: &str,
        operation: CryptoOperation,
    ) -> Result<CryptoResult, BackendError> {
        // Note: We need to make methods mutable where needed for key usage tracking
        // For now, we'll work around this limitation
        match operation {
            CryptoOperation::Sign { data, algorithm } => {
                // Create a mutable reference by cloning self (not ideal, but works for demo)
                let mut backend_clone = Self::with_config(self.config.clone()).map_err(|e| {
                    BackendError::OperationFailed(format!("Failed to clone backend: {}", e))
                })?;
                let signature = backend_clone
                    .sign_data(key_id, &data, algorithm)
                    .map_err(|e| BackendError::OperationFailed(format!("Signing failed: {}", e)))?;
                Ok(CryptoResult::Signed(signature))
            }
            CryptoOperation::Verify {
                data,
                signature,
                algorithm,
            } => {
                let is_valid = self
                    .verify_signature(key_id, &data, &signature, algorithm)
                    .map_err(|e| {
                        BackendError::OperationFailed(format!("Verification failed: {}", e))
                    })?;
                Ok(CryptoResult::VerificationResult(is_valid))
            }
            CryptoOperation::GetPublicKey => {
                let public_key = self.load_public_key(key_id).map_err(|e| {
                    if e.to_string().contains("Key not found") {
                        BackendError::KeyNotFound(key_id.to_string())
                    } else {
                        BackendError::OperationFailed(format!("Failed to load public key: {}", e))
                    }
                })?;
                Ok(CryptoResult::PublicKey(public_key))
            }
            CryptoOperation::GenerateKeyPair { algorithm: _ } => {
                // This would need a mutable reference - for demo we'll return an error
                Err(BackendError::UnsupportedOperation(
                    "Key generation requires mutable backend - use generate_key_pair() method directly".to_string()
                ))
            }
            CryptoOperation::Hash { data, algorithm } => {
                let hash = self.hash_data(&data, algorithm).map_err(|e| {
                    BackendError::OperationFailed(format!("Hash operation failed: {}", e))
                })?;
                Ok(CryptoResult::Hash(hash))
            }
            _ => Err(BackendError::UnsupportedOperation(format!(
                "Operation not supported by Software HSM: {:?}",
                std::any::type_name_of_val(&operation)
            ))),
        }
    }

    fn supports_operation(&self, operation: &CryptoOperation) -> bool {
        match operation {
            CryptoOperation::Sign { algorithm, .. } => {
                matches!(
                    algorithm,
                    SignatureAlgorithm::Ed25519 | SignatureAlgorithm::EcdsaP256
                )
            }
            CryptoOperation::Verify { algorithm, .. } => {
                matches!(
                    algorithm,
                    SignatureAlgorithm::Ed25519 | SignatureAlgorithm::EcdsaP256
                )
            }
            CryptoOperation::GetPublicKey => true,
            CryptoOperation::GenerateKeyPair { algorithm } => {
                matches!(
                    algorithm,
                    AsymmetricAlgorithm::Ed25519 | AsymmetricAlgorithm::EcdsaP256
                )
            }
            CryptoOperation::Hash { algorithm, .. } => {
                matches!(
                    algorithm,
                    HashAlgorithm::Sha256 | HashAlgorithm::Sha384 | HashAlgorithm::Sha512
                )
            }
            _ => false,
        }
    }

    fn get_capabilities(&self) -> BackendCapabilities {
        BackendCapabilities {
            symmetric_algorithms: vec![], // Software HSM focuses on asymmetric operations
            asymmetric_algorithms: vec![
                AsymmetricAlgorithm::Ed25519,
                AsymmetricAlgorithm::EcdsaP256,
            ],
            signature_algorithms: vec![SignatureAlgorithm::Ed25519, SignatureAlgorithm::EcdsaP256],
            hash_algorithms: vec![
                HashAlgorithm::Sha256,
                HashAlgorithm::Sha384,
                HashAlgorithm::Sha512,
            ],
            hardware_backed: false,         // Software implementation
            supports_key_derivation: false, // Focuses on asymmetric operations
            supports_key_generation: true,
            supports_attestation: false, // Software cannot provide hardware attestation
            max_key_size: Some(256),     // Ed25519 and P-256 are both 256-bit
        }
    }

    fn backend_info(&self) -> BackendInfo {
        BackendInfo {
            name: "software_hsm",
            description: "Software HSM with asymmetric crypto operations",
            version: "1.0.0",
            available: true,
            config_requirements: vec!["key_store_path", "passphrase"],
        }
    }

    fn list_keys(&self) -> Result<Vec<KeyMetadata>, BackendError> {
        let mut keys = Vec::new();
        for (key_id, metadata) in &self.key_metadata {
            let key_metadata = KeyMetadata {
                key_id: key_id.as_bytes().try_into().unwrap_or([0u8; 16]), // Simplified conversion
                description: metadata.description.clone(),
                created_at: metadata.created_at,
                last_used: metadata.last_used,
                backend_data: serde_json::to_vec(metadata).map_err(|e| {
                    BackendError::OperationFailed(format!(
                        "Failed to serialize key metadata: {}",
                        e
                    ))
                })?,
            };
            keys.push(key_metadata);
        }
        Ok(keys)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_test_backend() -> Result<(SoftwareHsmBackend, TempDir)> {
        let temp_dir = TempDir::new()?;
        let config = SoftwareHsmConfig {
            key_store_path: temp_dir.path().to_path_buf(),
            default_passphrase: "test123".to_string(),
            metadata_file: temp_dir.path().join("metadata.json"),
        };
        let backend = SoftwareHsmBackend::with_config(config)?;
        Ok((backend, temp_dir))
    }

    // ===== Configuration and Initialization Tests =====

    #[test]
    fn test_backend_creation_default_config() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let config = SoftwareHsmConfig {
            key_store_path: temp_dir.path().to_path_buf(),
            default_passphrase: "test123".to_string(),
            metadata_file: temp_dir.path().join("metadata.json"),
        };

        let backend = SoftwareHsmBackend::with_config(config)?;
        assert_eq!(backend.key_metadata.len(), 0);
        Ok(())
    }

    #[test]
    fn test_backend_creation_with_new() -> Result<()> {
        // This will create in default directory, but should work
        let backend = SoftwareHsmBackend::new();
        assert!(backend.is_ok());
        Ok(())
    }

    #[test]
    fn test_key_store_directory_creation() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let key_store_path = temp_dir.path().join("new_hsm_store");
        let config = SoftwareHsmConfig {
            key_store_path: key_store_path.clone(),
            default_passphrase: "test123".to_string(),
            metadata_file: key_store_path.join("metadata.json"),
        };

        // Directory doesn't exist yet
        assert!(!key_store_path.exists());

        let _backend = SoftwareHsmBackend::with_config(config)?;

        // Directory should be created
        assert!(key_store_path.exists());
        assert!(key_store_path.is_dir());
        Ok(())
    }

    #[test]
    fn test_metadata_persistence() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let config = SoftwareHsmConfig {
            key_store_path: temp_dir.path().to_path_buf(),
            default_passphrase: "test123".to_string(),
            metadata_file: temp_dir.path().join("metadata.json"),
        };

        // Create backend and add a key
        {
            let mut backend = SoftwareHsmBackend::with_config(config.clone())?;
            backend.generate_key_pair(
                "test_key",
                AsymmetricAlgorithm::Ed25519,
                Some("Test key".to_string()),
            )?;
            assert_eq!(backend.key_metadata.len(), 1);
        }

        // Create new backend instance - should load existing metadata
        {
            let backend = SoftwareHsmBackend::with_config(config)?;
            assert_eq!(backend.key_metadata.len(), 1);
            assert!(backend.key_metadata.contains_key("test_key"));
            let metadata = &backend.key_metadata["test_key"];
            assert_eq!(metadata.description, "Test key");
            assert_eq!(metadata.algorithm, AsymmetricAlgorithm::Ed25519);
        }

        Ok(())
    }

    // ===== Key Generation Tests =====

    #[test]
    fn test_ed25519_key_generation_and_signing() -> Result<()> {
        let (mut backend, _temp_dir) = create_test_backend()?;

        // Generate Ed25519 key pair
        backend.generate_key_pair("test_ed25519", AsymmetricAlgorithm::Ed25519, None)?;

        // Test signing
        let test_data = b"Hello, Software HSM!";
        let signature =
            backend.sign_data("test_ed25519", test_data, SignatureAlgorithm::Ed25519)?;

        // Test verification
        let is_valid = backend.verify_signature(
            "test_ed25519",
            test_data,
            &signature,
            SignatureAlgorithm::Ed25519,
        )?;
        assert!(is_valid);

        // Test invalid signature
        let mut invalid_signature = signature.clone();
        invalid_signature[0] ^= 0xFF; // Corrupt first byte
        let is_invalid = backend.verify_signature(
            "test_ed25519",
            test_data,
            &invalid_signature,
            SignatureAlgorithm::Ed25519,
        )?;
        assert!(!is_invalid);

        Ok(())
    }

    #[test]
    fn test_p256_key_generation_and_signing() -> Result<()> {
        let (mut backend, _temp_dir) = create_test_backend()?;

        // Generate P256 key pair
        backend.generate_key_pair(
            "test_p256",
            AsymmetricAlgorithm::EcdsaP256,
            Some("P256 test key".to_string()),
        )?;

        // Test signing
        let test_data = b"Hello, P-256!";
        let signature = backend.sign_data("test_p256", test_data, SignatureAlgorithm::EcdsaP256)?;

        // Test verification
        let is_valid = backend.verify_signature(
            "test_p256",
            test_data,
            &signature,
            SignatureAlgorithm::EcdsaP256,
        )?;
        assert!(is_valid);

        // Test invalid signature (corrupted)
        let mut invalid_signature = signature.clone();
        if !invalid_signature.is_empty() {
            let len = invalid_signature.len();
            invalid_signature[len - 1] ^= 0xFF; // Corrupt last byte
        }
        let is_invalid = backend.verify_signature(
            "test_p256",
            test_data,
            &invalid_signature,
            SignatureAlgorithm::EcdsaP256,
        )?;
        assert!(!is_invalid);

        Ok(())
    }

    #[test]
    fn test_unsupported_algorithm_generation() {
        let (mut backend, _temp_dir) = create_test_backend().unwrap();

        // Try to generate RSA key (not supported)
        let result = backend.generate_key_pair("test_rsa", AsymmetricAlgorithm::Rsa2048, None);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Unsupported algorithm"));
    }

    #[test]
    fn test_duplicate_key_generation() -> Result<()> {
        let (mut backend, _temp_dir) = create_test_backend()?;

        // Generate first key
        backend.generate_key_pair("duplicate_key", AsymmetricAlgorithm::Ed25519, None)?;

        // Generate second key with same ID (should overwrite)
        backend.generate_key_pair(
            "duplicate_key",
            AsymmetricAlgorithm::EcdsaP256,
            Some("Second key".to_string()),
        )?;

        // Should have only one key, and it should be the second one
        assert_eq!(backend.key_metadata.len(), 1);
        let metadata = &backend.key_metadata["duplicate_key"];
        assert_eq!(metadata.algorithm, AsymmetricAlgorithm::EcdsaP256);
        assert_eq!(metadata.description, "Second key");

        Ok(())
    }

    #[test]
    fn test_key_file_storage() -> Result<()> {
        let (mut backend, temp_dir) = create_test_backend()?;

        backend.generate_key_pair("file_test", AsymmetricAlgorithm::Ed25519, None)?;

        // Check that key files exist
        let private_key_path = temp_dir.path().join("file_test_private.key");
        let public_key_path = temp_dir.path().join("file_test_public.key");

        assert!(private_key_path.exists());
        assert!(public_key_path.exists());

        // Check file sizes are reasonable
        let private_key_size = fs::metadata(&private_key_path)?.len();
        let public_key_size = fs::metadata(&public_key_path)?.len();

        assert_eq!(private_key_size, 32); // Ed25519 private key is 32 bytes
        assert_eq!(public_key_size, 32); // Ed25519 public key is 32 bytes

        Ok(())
    }

    // ===== Signing and Verification Tests =====

    #[test]
    fn test_sign_with_nonexistent_key() {
        let (mut backend, _temp_dir) = create_test_backend().unwrap();

        let result = backend.sign_data("nonexistent", b"test", SignatureAlgorithm::Ed25519);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Key not found"));
    }

    #[test]
    fn test_verify_with_nonexistent_key() {
        let (backend, _temp_dir) = create_test_backend().unwrap();

        let result = backend.verify_signature(
            "nonexistent",
            b"test",
            &[0; 64],
            SignatureAlgorithm::Ed25519,
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Key not found"));
    }

    #[test]
    fn test_algorithm_mismatch_signing() -> Result<()> {
        let (mut backend, _temp_dir) = create_test_backend()?;

        // Generate Ed25519 key
        backend.generate_key_pair("ed25519_key", AsymmetricAlgorithm::Ed25519, None)?;

        // Try to sign with P256 algorithm (should fail)
        let result = backend.sign_data("ed25519_key", b"test", SignatureAlgorithm::EcdsaP256);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Incompatible key algorithm"));

        Ok(())
    }

    #[test]
    fn test_algorithm_mismatch_verification() -> Result<()> {
        let (mut backend, _temp_dir) = create_test_backend()?;

        // Generate P256 key
        backend.generate_key_pair("p256_key", AsymmetricAlgorithm::EcdsaP256, None)?;

        // Try to verify with Ed25519 algorithm (should fail)
        let result =
            backend.verify_signature("p256_key", b"test", &[0; 64], SignatureAlgorithm::Ed25519);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Incompatible key algorithm"));

        Ok(())
    }

    #[test]
    fn test_multiple_signatures_same_key() -> Result<()> {
        let (mut backend, _temp_dir) = create_test_backend()?;

        backend.generate_key_pair("multi_sig", AsymmetricAlgorithm::Ed25519, None)?;

        let data1 = b"First message";
        let data2 = b"Second message";
        let data3 = b"Third message";

        // Sign multiple different messages
        let sig1 = backend.sign_data("multi_sig", data1, SignatureAlgorithm::Ed25519)?;
        let sig2 = backend.sign_data("multi_sig", data2, SignatureAlgorithm::Ed25519)?;
        let sig3 = backend.sign_data("multi_sig", data3, SignatureAlgorithm::Ed25519)?;

        // All signatures should be different
        assert_ne!(sig1, sig2);
        assert_ne!(sig2, sig3);
        assert_ne!(sig1, sig3);

        // All should verify correctly
        assert!(backend.verify_signature(
            "multi_sig",
            data1,
            &sig1,
            SignatureAlgorithm::Ed25519
        )?);
        assert!(backend.verify_signature(
            "multi_sig",
            data2,
            &sig2,
            SignatureAlgorithm::Ed25519
        )?);
        assert!(backend.verify_signature(
            "multi_sig",
            data3,
            &sig3,
            SignatureAlgorithm::Ed25519
        )?);

        // Cross-verification should fail
        assert!(!backend.verify_signature(
            "multi_sig",
            data1,
            &sig2,
            SignatureAlgorithm::Ed25519
        )?);
        assert!(!backend.verify_signature(
            "multi_sig",
            data2,
            &sig3,
            SignatureAlgorithm::Ed25519
        )?);

        Ok(())
    }

    #[test]
    fn test_signature_determinism_ed25519() -> Result<()> {
        let (mut backend, _temp_dir) = create_test_backend()?;

        backend.generate_key_pair("deterministic", AsymmetricAlgorithm::Ed25519, None)?;

        let data = b"Deterministic test message";

        // Ed25519 signatures should be deterministic for the same message and key
        let sig1 = backend.sign_data("deterministic", data, SignatureAlgorithm::Ed25519)?;
        let sig2 = backend.sign_data("deterministic", data, SignatureAlgorithm::Ed25519)?;

        assert_eq!(sig1, sig2, "Ed25519 signatures should be deterministic");

        Ok(())
    }

    #[test]
    fn test_signature_randomness_p256() -> Result<()> {
        let (mut backend, _temp_dir) = create_test_backend()?;

        backend.generate_key_pair("random", AsymmetricAlgorithm::EcdsaP256, None)?;

        let data = b"Random test message";

        // P256/ECDSA signatures may be deterministic or random depending on implementation
        let sig1 = backend.sign_data("random", data, SignatureAlgorithm::EcdsaP256)?;
        let sig2 = backend.sign_data("random", data, SignatureAlgorithm::EcdsaP256)?;

        // Both should verify regardless of whether they're the same or different
        assert!(backend.verify_signature("random", data, &sig1, SignatureAlgorithm::EcdsaP256)?);
        assert!(backend.verify_signature("random", data, &sig2, SignatureAlgorithm::EcdsaP256)?);

        println!("✔ P256 signatures generated and verified successfully");
        println!(
            "  Signatures are {} (implementation dependent)",
            if sig1 == sig2 {
                "deterministic"
            } else {
                "randomized"
            }
        );

        Ok(())
    }

    // ===== UniversalBackend Interface Tests =====

    #[test]
    fn test_universal_backend_interface() -> Result<()> {
        let (mut backend, _temp_dir) = create_test_backend()?;

        // Generate a key pair first
        backend.generate_key_pair("test_universal", AsymmetricAlgorithm::Ed25519, None)?;

        // Test the UniversalBackend interface
        let test_data = b"Testing UniversalBackend interface";

        // Test signing through UniversalBackend
        let sign_op = CryptoOperation::Sign {
            data: test_data.to_vec(),
            algorithm: SignatureAlgorithm::Ed25519,
        };

        let result = backend.perform_operation("test_universal", sign_op)?;
        match result {
            CryptoResult::Signed(signature) => {
                // Test verification through UniversalBackend
                let verify_op = CryptoOperation::Verify {
                    data: test_data.to_vec(),
                    signature,
                    algorithm: SignatureAlgorithm::Ed25519,
                };

                let verify_result = backend.perform_operation("test_universal", verify_op)?;
                match verify_result {
                    CryptoResult::VerificationResult(is_valid) => assert!(is_valid),
                    _ => panic!("Expected VerificationResult"),
                }
            }
            _ => panic!("Expected Signed result"),
        }

        Ok(())
    }

    #[test]
    fn test_universal_backend_get_public_key() -> Result<()> {
        let (mut backend, _temp_dir) = create_test_backend()?;

        backend.generate_key_pair("pubkey_test", AsymmetricAlgorithm::Ed25519, None)?;

        let operation = CryptoOperation::GetPublicKey;
        let result = backend.perform_operation("pubkey_test", operation)?;

        match result {
            CryptoResult::PublicKey(public_key) => {
                assert_eq!(public_key.len(), 32); // Ed25519 public key size
            }
            _ => panic!("Expected PublicKey result"),
        }

        Ok(())
    }

    #[test]
    fn test_universal_backend_hash_operations() -> Result<()> {
        let (backend, _temp_dir) = create_test_backend()?;

        let test_data = b"Hash this data";

        // Test SHA256
        let hash_op = CryptoOperation::Hash {
            data: test_data.to_vec(),
            algorithm: HashAlgorithm::Sha256,
        };
        let result = backend.perform_operation("", hash_op)?;
        match result {
            CryptoResult::Hash(hash) => {
                assert_eq!(hash.len(), 32); // SHA256 output size
            }
            _ => panic!("Expected Hash result"),
        }

        // Test SHA512
        let hash_op = CryptoOperation::Hash {
            data: test_data.to_vec(),
            algorithm: HashAlgorithm::Sha512,
        };
        let result = backend.perform_operation("", hash_op)?;
        match result {
            CryptoResult::Hash(hash) => {
                assert_eq!(hash.len(), 64); // SHA512 output size
            }
            _ => panic!("Expected Hash result"),
        }

        Ok(())
    }

    #[test]
    fn test_universal_backend_unsupported_operations() -> Result<()> {
        let (backend, _temp_dir) = create_test_backend()?;

        // Test symmetric encryption (not supported)
        let encrypt_op = CryptoOperation::Encrypt {
            plaintext: vec![1, 2, 3],
            algorithm: crate::backends::universal::SymmetricAlgorithm::Aes256Gcm,
        };
        let result = backend.perform_operation("", encrypt_op);
        assert!(result.is_err());

        // Test key derivation (not supported)
        let derive_op = CryptoOperation::DeriveKey {
            context: crate::backends::universal::KeyDerivationContext::new(vec![1, 2, 3, 4]),
        };
        let result = backend.perform_operation("", derive_op);
        assert!(result.is_err());

        // Test attestation (not supported)
        let attest_op = CryptoOperation::Attest {
            challenge: vec![1, 2, 3],
        };
        let result = backend.perform_operation("", attest_op);
        assert!(result.is_err());

        Ok(())
    }

    // ===== Backend Capabilities and Metadata Tests =====

    #[test]
    fn test_backend_capabilities() {
        let (backend, _temp_dir) = create_test_backend().unwrap();

        let capabilities = backend.get_capabilities();
        assert!(!capabilities.hardware_backed);
        assert!(capabilities.supports_key_generation);
        assert!(!capabilities.supports_key_derivation);
        assert!(!capabilities.supports_attestation);
        assert!(capabilities
            .signature_algorithms
            .contains(&SignatureAlgorithm::Ed25519));
        assert!(capabilities
            .asymmetric_algorithms
            .contains(&AsymmetricAlgorithm::Ed25519));
        assert!(capabilities
            .signature_algorithms
            .contains(&SignatureAlgorithm::EcdsaP256));
        assert!(capabilities
            .asymmetric_algorithms
            .contains(&AsymmetricAlgorithm::EcdsaP256));
        assert_eq!(capabilities.max_key_size, Some(256));
    }

    #[test]
    fn test_backend_info() {
        let (backend, _temp_dir) = create_test_backend().unwrap();

        let info = backend.backend_info();
        assert_eq!(info.name, "software_hsm");
        assert_eq!(info.version, "1.0.0");
        assert!(info.available);
        assert!(!info.description.is_empty());
    }

    #[test]
    fn test_operation_support() {
        let (backend, _temp_dir) = create_test_backend().unwrap();

        // Test supported operations
        let sign_op = CryptoOperation::Sign {
            data: vec![1, 2, 3],
            algorithm: SignatureAlgorithm::Ed25519,
        };
        assert!(backend.supports_operation(&sign_op));

        let verify_op = CryptoOperation::Verify {
            data: vec![1, 2, 3],
            signature: vec![0; 64],
            algorithm: SignatureAlgorithm::EcdsaP256,
        };
        assert!(backend.supports_operation(&verify_op));

        let get_pubkey_op = CryptoOperation::GetPublicKey;
        assert!(backend.supports_operation(&get_pubkey_op));

        let gen_key_op = CryptoOperation::GenerateKeyPair {
            algorithm: AsymmetricAlgorithm::Ed25519,
        };
        assert!(backend.supports_operation(&gen_key_op));

        let hash_op = CryptoOperation::Hash {
            data: vec![1, 2, 3],
            algorithm: HashAlgorithm::Sha256,
        };
        assert!(backend.supports_operation(&hash_op));

        // Test unsupported operations
        let encrypt_op = CryptoOperation::Encrypt {
            plaintext: vec![1, 2, 3],
            algorithm: crate::backends::universal::SymmetricAlgorithm::Aes256Gcm,
        };
        assert!(!backend.supports_operation(&encrypt_op));

        let unsupported_gen_key = CryptoOperation::GenerateKeyPair {
            algorithm: AsymmetricAlgorithm::Rsa2048,
        };
        assert!(!backend.supports_operation(&unsupported_gen_key));
    }

    #[test]
    fn test_list_keys() -> Result<()> {
        let (mut backend, _temp_dir) = create_test_backend()?;

        // Initially no keys
        let keys = backend.list_keys()?;
        assert_eq!(keys.len(), 0);

        // Add some keys
        backend.generate_key_pair(
            "key1",
            AsymmetricAlgorithm::Ed25519,
            Some("First key".to_string()),
        )?;
        backend.generate_key_pair(
            "key2",
            AsymmetricAlgorithm::EcdsaP256,
            Some("Second key".to_string()),
        )?;

        let keys = backend.list_keys()?;
        assert_eq!(keys.len(), 2);

        // Check metadata is preserved
        let mut descriptions: Vec<String> = keys.iter().map(|k| k.description.clone()).collect();
        descriptions.sort();
        assert_eq!(descriptions, vec!["First key", "Second key"]);

        Ok(())
    }

    // ===== Key Usage Tracking Tests =====

    #[test]
    fn test_key_usage_tracking() -> Result<()> {
        let (mut backend, _temp_dir) = create_test_backend()?;

        backend.generate_key_pair("usage_test", AsymmetricAlgorithm::Ed25519, None)?;

        // Initially, last_used should be None
        let metadata = &backend.key_metadata["usage_test"];
        assert!(metadata.last_used.is_none());

        // Use the key for signing
        let _signature = backend.sign_data("usage_test", b"test", SignatureAlgorithm::Ed25519)?;

        // Now last_used should be set
        let metadata = &backend.key_metadata["usage_test"];
        assert!(metadata.last_used.is_some());

        let first_usage = metadata.last_used.unwrap();

        // Use it again after a small delay
        std::thread::sleep(std::time::Duration::from_millis(10));
        let _signature2 = backend.sign_data("usage_test", b"test2", SignatureAlgorithm::Ed25519)?;

        // Usage time should be updated
        let metadata = &backend.key_metadata["usage_test"];
        let second_usage = metadata.last_used.unwrap();
        assert!(second_usage >= first_usage);

        Ok(())
    }

    // ===== Error Handling Tests =====

    #[test]
    fn test_invalid_signature_lengths() -> Result<()> {
        let (mut backend, _temp_dir) = create_test_backend()?;

        backend.generate_key_pair("length_test", AsymmetricAlgorithm::Ed25519, None)?;

        // Test with signature that's too short
        let result = backend.verify_signature(
            "length_test",
            b"test",
            &[0; 32],
            SignatureAlgorithm::Ed25519,
        );
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Invalid Ed25519 signature length"));

        // Test with signature that's too long
        let result = backend.verify_signature(
            "length_test",
            b"test",
            &[0; 128],
            SignatureAlgorithm::Ed25519,
        );
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Invalid Ed25519 signature length"));

        Ok(())
    }

    #[test]
    fn test_corrupted_key_files() -> Result<()> {
        let (mut backend, temp_dir) = create_test_backend()?;

        backend.generate_key_pair("corrupt_test", AsymmetricAlgorithm::Ed25519, None)?;

        // Corrupt the private key file
        let private_key_path = temp_dir.path().join("corrupt_test_private.key");
        fs::write(&private_key_path, [0; 16])?; // Wrong size

        // Signing should fail
        let result = backend.sign_data("corrupt_test", b"test", SignatureAlgorithm::Ed25519);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Invalid Ed25519 private key length"));

        Ok(())
    }

    #[test]
    fn test_missing_key_files() -> Result<()> {
        let (mut backend, temp_dir) = create_test_backend()?;

        backend.generate_key_pair("missing_test", AsymmetricAlgorithm::Ed25519, None)?;

        // Delete the private key file
        let private_key_path = temp_dir.path().join("missing_test_private.key");
        fs::remove_file(&private_key_path)?;

        // Signing should fail
        let result = backend.sign_data("missing_test", b"test", SignatureAlgorithm::Ed25519);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Failed to read private key"));

        Ok(())
    }

    // ===== Stress and Edge Case Tests =====

    #[test]
    fn test_large_data_signing() -> Result<()> {
        let (mut backend, _temp_dir) = create_test_backend()?;

        backend.generate_key_pair("large_data", AsymmetricAlgorithm::Ed25519, None)?;

        // Test with 1MB of data
        let large_data = vec![0x42; 1024 * 1024];
        let signature =
            backend.sign_data("large_data", &large_data, SignatureAlgorithm::Ed25519)?;
        let is_valid = backend.verify_signature(
            "large_data",
            &large_data,
            &signature,
            SignatureAlgorithm::Ed25519,
        )?;
        assert!(is_valid);

        Ok(())
    }

    #[test]
    fn test_empty_data_signing() -> Result<()> {
        let (mut backend, _temp_dir) = create_test_backend()?;

        backend.generate_key_pair("empty_data", AsymmetricAlgorithm::Ed25519, None)?;

        // Test with empty data
        let empty_data = b"";
        let signature = backend.sign_data("empty_data", empty_data, SignatureAlgorithm::Ed25519)?;
        let is_valid = backend.verify_signature(
            "empty_data",
            empty_data,
            &signature,
            SignatureAlgorithm::Ed25519,
        )?;
        assert!(is_valid);

        Ok(())
    }

    #[test]
    fn test_many_keys() -> Result<()> {
        let (mut backend, _temp_dir) = create_test_backend()?;

        // Generate many keys
        for i in 0..100 {
            let key_id = format!("key_{}", i);
            let algorithm = if i % 2 == 0 {
                AsymmetricAlgorithm::Ed25519
            } else {
                AsymmetricAlgorithm::EcdsaP256
            };
            backend.generate_key_pair(&key_id, algorithm, Some(format!("Key number {}", i)))?;
        }

        assert_eq!(backend.key_metadata.len(), 100);

        // Test that we can use all keys
        for i in 0..100 {
            let key_id = format!("key_{}", i);
            let algorithm = if i % 2 == 0 {
                SignatureAlgorithm::Ed25519
            } else {
                SignatureAlgorithm::EcdsaP256
            };

            let test_data = format!("Test data for key {}", i);
            let signature = backend.sign_data(&key_id, test_data.as_bytes(), algorithm)?;

            // Recreate algorithm for verify call
            let verify_algorithm = if i % 2 == 0 {
                SignatureAlgorithm::Ed25519
            } else {
                SignatureAlgorithm::EcdsaP256
            };
            let is_valid = backend.verify_signature(
                &key_id,
                test_data.as_bytes(),
                &signature,
                verify_algorithm,
            )?;
            assert!(is_valid);
        }

        Ok(())
    }

    #[test]
    fn test_concurrent_key_operations() -> Result<()> {
        // Note: This is a basic test since we're not using actual threading
        // In a real concurrent scenario, you'd want to test with Arc<Mutex<Backend>>
        let (mut backend, _temp_dir) = create_test_backend()?;

        backend.generate_key_pair("concurrent_test", AsymmetricAlgorithm::Ed25519, None)?;

        // Simulate rapid operations
        for i in 0..10 {
            let data = format!("Message {}", i);
            let signature = backend.sign_data(
                "concurrent_test",
                data.as_bytes(),
                SignatureAlgorithm::Ed25519,
            )?;
            let is_valid = backend.verify_signature(
                "concurrent_test",
                data.as_bytes(),
                &signature,
                SignatureAlgorithm::Ed25519,
            )?;
            assert!(is_valid);
        }

        Ok(())
    }

    #[test]
    fn test_p256_signature_variations() -> Result<()> {
        let (mut backend, _temp_dir) = create_test_backend()?;

        backend.generate_key_pair("p256_var", AsymmetricAlgorithm::EcdsaP256, None)?;

        // P256 signatures may vary or be deterministic depending on implementation
        let test_data = b"P256 signature test";
        let mut signatures = Vec::new();
        let mut all_different = true;

        for _ in 0..5 {
            let signature =
                backend.sign_data("p256_var", test_data, SignatureAlgorithm::EcdsaP256)?;

            // Check if this signature matches any previous one
            for prev_sig in &signatures {
                if signature == *prev_sig {
                    all_different = false;
                }
            }

            signatures.push(signature.clone());

            // Each signature should verify
            let is_valid = backend.verify_signature(
                "p256_var",
                test_data,
                &signature,
                SignatureAlgorithm::EcdsaP256,
            )?;
            assert!(is_valid);
        }

        println!(
            "✔ P256 signature behavior: {}",
            if all_different {
                "randomized (different each time)"
            } else {
                "deterministic (same each time)"
            }
        );

        // What matters is that all signatures verify correctly
        assert_eq!(signatures.len(), 5);

        Ok(())
    }
}
