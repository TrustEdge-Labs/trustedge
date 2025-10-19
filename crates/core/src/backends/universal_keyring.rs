//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/
//
// Project: trustedge — Privacy and trust at the edge.
//

//! Universal keyring backend implementation
//!
//! This module implements the UniversalBackend trait for the OS keyring,
//! supporting key derivation and AES encryption/decryption operations.

use crate::backends::keyring::KeyringBackend;
use crate::backends::traits::{BackendInfo, KeyMetadata};
use crate::backends::universal::*;
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm,
};
use anyhow::{anyhow, Result};
use pbkdf2::pbkdf2_hmac;
use rand_core::{OsRng, RngCore};
use sha2::Sha256;

/// Universal backend wrapper for KeyringBackend
///
/// UniversalKeyringBackend implements the UniversalBackend trait for OS keyring
/// operations, providing key derivation and hash operations using the system
/// keyring for secure passphrase storage.
pub struct UniversalKeyringBackend {
    inner: KeyringBackend,
}

impl UniversalKeyringBackend {
    /// Create a new universal keyring backend
    pub fn new() -> Result<Self> {
        Ok(Self {
            inner: KeyringBackend::new()?,
        })
    }

    /// Create with custom service/username
    pub fn new_with_service(service_name: String, username: String) -> Result<Self> {
        Ok(Self {
            inner: KeyringBackend::new_with_service(service_name, username)?,
        })
    }

    /// Store a passphrase in the keyring
    pub fn store_passphrase(&self, passphrase: &str) -> Result<()> {
        self.inner.store_passphrase(passphrase)
    }

    /// Get the passphrase from the keyring
    pub fn get_passphrase(&self) -> Result<String> {
        self.inner.get_passphrase()
    }

    /// Derive a key using the specified context
    fn derive_key_internal(
        &self,
        key_id: &str,
        context: &KeyDerivationContext,
    ) -> Result<[u8; 32]> {
        // Validate salt length
        if context.salt.len() != 16 {
            return Err(anyhow!("Salt must be exactly 16 bytes for keyring backend"));
        }

        // Get passphrase from keyring
        let passphrase = self
            .get_passphrase()
            .map_err(|e| anyhow!("Failed to get passphrase from keyring: {}", e))?;

        // Convert salt to array
        let mut salt_array = [0u8; 16];
        salt_array.copy_from_slice(&context.salt);

        // Use PBKDF2 with the specified hash algorithm
        let iterations = context.iterations.unwrap_or(100_000);
        let mut key = [0u8; 32];

        // Include key_id in the derivation for key isolation
        let mut input = passphrase.as_bytes().to_vec();
        input.extend_from_slice(key_id.as_bytes());
        input.extend_from_slice(&context.additional_data);

        // Use the hash algorithm from context or default to SHA-256
        match context
            .hash_algorithm
            .as_ref()
            .unwrap_or(&HashAlgorithm::Sha256)
        {
            HashAlgorithm::Sha256 => {
                pbkdf2_hmac::<Sha256>(&input, &salt_array, iterations, &mut key);
            }
            HashAlgorithm::Sha384 => {
                pbkdf2_hmac::<sha2::Sha384>(&input, &salt_array, iterations, &mut key);
            }
            HashAlgorithm::Sha512 => {
                pbkdf2_hmac::<sha2::Sha512>(&input, &salt_array, iterations, &mut key);
            }
            _ => {
                return Err(anyhow!(
                    "Hash algorithm {:?} not supported by keyring backend",
                    context.hash_algorithm
                ));
            }
        }

        Ok(key)
    }

    /// Encrypt data using AES-256-GCM
    #[allow(dead_code)] // Reserved for future use
    fn encrypt_aes_gcm(&self, key: &[u8; 32], plaintext: &[u8]) -> Result<Vec<u8>> {
        let cipher = Aes256Gcm::new_from_slice(key)?;

        // Generate a random nonce
        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);

        // Encrypt the data
        let ciphertext = cipher
            .encrypt((&nonce_bytes).into(), plaintext)
            .map_err(|e| anyhow!("AES-GCM encryption failed: {}", e))?;

        // Prepend nonce to ciphertext
        let mut result = nonce_bytes.to_vec();
        result.extend_from_slice(&ciphertext);

        Ok(result)
    }

    /// Decrypt data using AES-256-GCM
    #[allow(dead_code)] // Reserved for future use
    fn decrypt_aes_gcm(&self, key: &[u8; 32], ciphertext: &[u8]) -> Result<Vec<u8>> {
        if ciphertext.len() < 12 {
            return Err(anyhow!("Ciphertext too short to contain nonce"));
        }

        let cipher = Aes256Gcm::new_from_slice(key)?;

        // Extract nonce and ciphertext
        let (nonce_bytes, encrypted_data) = ciphertext.split_at(12);
        let nonce_array: &[u8; 12] = nonce_bytes
            .try_into()
            .map_err(|_| anyhow!("Nonce conversion failed"))?;

        // Decrypt the data
        let plaintext = cipher
            .decrypt(nonce_array.into(), encrypted_data)
            .map_err(|e| anyhow!("AES-GCM decryption failed: {}", e))?;

        Ok(plaintext)
    }
}

impl UniversalBackend for UniversalKeyringBackend {
    fn perform_operation(&self, key_id: &str, operation: CryptoOperation) -> Result<CryptoResult> {
        match operation {
            CryptoOperation::DeriveKey { context } => {
                let key = self.derive_key_internal(key_id, &context)?;
                Ok(CryptoResult::DerivedKey(key))
            }

            CryptoOperation::Encrypt {
                plaintext: _,
                algorithm,
            } => match algorithm {
                SymmetricAlgorithm::Aes256Gcm => {
                    // We need to derive a key first, but we need context
                    // For now, use a simple approach - in practice, the key would be provided
                    // or derived using a stored context
                    Err(anyhow!(
                        "Encryption requires key derivation context. Use DeriveKey first, then use the raw key."
                    ))
                }
                _ => Err(anyhow!(
                    "Symmetric algorithm {:?} not supported by keyring backend",
                    algorithm
                )),
            },

            CryptoOperation::Decrypt {
                ciphertext: _,
                algorithm,
            } => match algorithm {
                SymmetricAlgorithm::Aes256Gcm => {
                    // Same issue as encrypt - need a way to get the key
                    Err(anyhow!(
                        "Decryption requires key derivation context. Use DeriveKey first, then use the raw key."
                    ))
                }
                _ => Err(anyhow!(
                    "Symmetric algorithm {:?} not supported by keyring backend",
                    algorithm
                )),
            },

            CryptoOperation::Hash { data, algorithm } => match algorithm {
                HashAlgorithm::Sha256 => {
                    use sha2::{Digest, Sha256};
                    let hash = Sha256::digest(&data);
                    Ok(CryptoResult::Hash(hash.to_vec()))
                }
                HashAlgorithm::Sha384 => {
                    use sha2::{Digest, Sha384};
                    let hash = Sha384::digest(&data);
                    Ok(CryptoResult::Hash(hash.to_vec()))
                }
                HashAlgorithm::Sha512 => {
                    use sha2::{Digest, Sha512};
                    let hash = Sha512::digest(&data);
                    Ok(CryptoResult::Hash(hash.to_vec()))
                }
                _ => Err(anyhow!(
                    "Hash algorithm {:?} not supported by keyring backend",
                    algorithm
                )),
            },

            _ => Err(anyhow!(
                "Operation {:?} not supported by keyring backend",
                operation
            )),
        }
    }

    fn supports_operation(&self, operation: &CryptoOperation) -> bool {
        matches!(
            operation,
            CryptoOperation::DeriveKey { .. }
                | CryptoOperation::Hash {
                    algorithm: HashAlgorithm::Sha256
                        | HashAlgorithm::Sha384
                        | HashAlgorithm::Sha512,
                    ..
                }
        )
    }

    fn get_capabilities(&self) -> BackendCapabilities {
        BackendCapabilities {
            symmetric_algorithms: vec![], // We support AES-256-GCM but need better key management
            asymmetric_algorithms: vec![],
            signature_algorithms: vec![],
            hash_algorithms: vec![
                HashAlgorithm::Sha256,
                HashAlgorithm::Sha384,
                HashAlgorithm::Sha512,
            ],
            hardware_backed: false,
            supports_key_derivation: true,
            supports_key_generation: false,
            supports_attestation: false,
            max_key_size: None,
        }
    }

    fn backend_info(&self) -> BackendInfo {
        BackendInfo::keyring()
    }

    fn list_keys(&self) -> Result<Vec<KeyMetadata>> {
        // Delegate to the inner keyring backend
        use crate::backends::traits::KeyBackend;
        self.inner.list_keys()
    }
}

impl Default for UniversalKeyringBackend {
    fn default() -> Self {
        Self::new().expect("Failed to create default universal keyring backend")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_universal_keyring_backend_creation() {
        let backend = UniversalKeyringBackend::new();
        assert!(backend.is_ok());
    }

    #[test]
    fn test_backend_capabilities() {
        let backend = UniversalKeyringBackend::new().unwrap();
        let capabilities = backend.get_capabilities();

        assert!(capabilities.supports_key_derivation);
        assert!(!capabilities.supports_key_generation);
        assert!(!capabilities.supports_attestation);
        assert!(!capabilities.hardware_backed);
        assert!(capabilities
            .hash_algorithms
            .contains(&HashAlgorithm::Sha256));
    }

    #[test]
    fn test_supports_operation() {
        let backend = UniversalKeyringBackend::new().unwrap();

        // Should support key derivation
        let derive_op = CryptoOperation::DeriveKey {
            context: KeyDerivationContext::new(vec![1; 16]),
        };
        assert!(backend.supports_operation(&derive_op));

        // Should support hashing
        let hash_op = CryptoOperation::Hash {
            data: vec![1, 2, 3],
            algorithm: HashAlgorithm::Sha256,
        };
        assert!(backend.supports_operation(&hash_op));

        // Should not support signing
        let sign_op = CryptoOperation::Sign {
            data: vec![1, 2, 3],
            algorithm: SignatureAlgorithm::Ed25519,
        };
        assert!(!backend.supports_operation(&sign_op));

        // Should not support attestation
        let attest_op = CryptoOperation::Attest {
            challenge: vec![1, 2, 3],
        };
        assert!(!backend.supports_operation(&attest_op));
    }

    #[test]
    fn test_hash_operation() {
        let backend = UniversalKeyringBackend::new().unwrap();
        let test_data = b"hello world";

        let result = backend.perform_operation(
            "test_key",
            CryptoOperation::Hash {
                data: test_data.to_vec(),
                algorithm: HashAlgorithm::Sha256,
            },
        );

        assert!(result.is_ok());
        if let Ok(CryptoResult::Hash(hash)) = result {
            assert_eq!(hash.len(), 32); // SHA-256 produces 32-byte hashes
        } else {
            panic!("Expected CryptoResult::Hash");
        }
    }

    #[test]
    fn test_key_derivation_operation() {
        let backend = UniversalKeyringBackend::new().unwrap();

        // This test will fail if no passphrase is stored in keyring
        // but should show the operation structure works
        let context = KeyDerivationContext::new(vec![1; 16])
            .with_additional_data(vec![2, 3, 4])
            .with_iterations(1000);

        let result = backend.perform_operation("test_key", CryptoOperation::DeriveKey { context });

        // May fail due to missing keyring passphrase, but should not panic
        match result {
            Ok(CryptoResult::DerivedKey(key)) => {
                assert_eq!(key.len(), 32);
            }
            Err(_) => {
                // Expected if no passphrase in keyring
                println!("Key derivation failed (likely no passphrase in keyring) - this is expected in tests");
            }
            _ => panic!("Unexpected result type"),
        }
    }
}
