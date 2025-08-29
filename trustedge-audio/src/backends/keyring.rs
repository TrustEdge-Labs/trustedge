//! Keyring backend implementation
//!
//! This backend uses the OS keyring for passphrase storage and PBKDF2 for key derivation.

use crate::backends::traits::{BackendInfo, KeyBackend, KeyContext, KeyMetadata};
use anyhow::{anyhow, Result};
use keyring::Entry;
use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};

/// Keyring-based key backend using PBKDF2 key derivation
pub struct KeyringBackend {
    service_name: String,
    username: String,
}

impl KeyringBackend {
    /// Create a new keyring backend with default service/username
    pub fn new() -> Result<Self> {
        Ok(Self {
            service_name: "trustedge".to_string(),
            username: "encryption_key".to_string(),
        })
    }

    /// Create a new keyring backend with custom service/username
    pub fn new_with_service(service_name: String, username: String) -> Result<Self> {
        Ok(Self {
            service_name,
            username,
        })
    }

    /// Store a passphrase in the OS keyring
    pub fn store_passphrase(&self, passphrase: &str) -> Result<()> {
        let entry = Entry::new(&self.service_name, &self.username)?;
        entry.set_password(passphrase)?;
        Ok(())
    }

    /// Get the passphrase from the OS keyring
    pub fn get_passphrase(&self) -> Result<String> {
        let entry = Entry::new(&self.service_name, &self.username)?;
        let passphrase = entry.get_password()?;
        Ok(passphrase)
    }
}

impl KeyBackend for KeyringBackend {
    fn derive_key(&self, key_id: &[u8; 16], context: &KeyContext) -> Result<[u8; 32]> {
        // Get passphrase from keyring
        let passphrase = self
            .get_passphrase()
            .map_err(|e| anyhow!("Failed to get passphrase from keyring: {}", e))?;

        // Validate salt length
        if context.salt.len() != 16 {
            return Err(anyhow!("Salt must be exactly 16 bytes for keyring backend"));
        }

        // Convert salt to array
        let mut salt_array = [0u8; 16];
        salt_array.copy_from_slice(&context.salt);

        // Use PBKDF2 with SHA256
        let iterations = context.iterations.unwrap_or(100_000);
        let mut key = [0u8; 32];

        // Include key_id in the derivation for key isolation
        let mut input = passphrase.as_bytes().to_vec();
        input.extend_from_slice(key_id);
        input.extend_from_slice(&context.additional_data);

        pbkdf2_hmac::<Sha256>(&input, &salt_array, iterations, &mut key);

        Ok(key)
    }

    fn store_key(&self, _key_id: &[u8; 16], _key_data: &[u8; 32]) -> Result<()> {
        // Keyring backend doesn't store raw keys, only passphrases
        Err(anyhow!("Keyring backend does not support storing raw keys"))
    }

    fn rotate_key(&self, _old_id: &[u8; 16], _new_id: &[u8; 16]) -> Result<()> {
        // Key rotation for keyring backend would involve changing the passphrase
        // This is a manual process that requires user interaction
        Err(anyhow!(
            "Keyring backend does not support automatic key rotation"
        ))
    }

    fn list_keys(&self) -> Result<Vec<KeyMetadata>> {
        // Keyring backend can only report if a passphrase is available
        match self.get_passphrase() {
            Ok(_) => {
                let metadata = KeyMetadata {
                    key_id: [0u8; 16], // No specific key ID for keyring
                    description: "Keyring-derived encryption key".to_string(),
                    created_at: 0, // Unknown creation time
                    last_used: Some(
                        SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_secs(),
                    ),
                    backend_data: self.service_name.as_bytes().to_vec(),
                };
                Ok(vec![metadata])
            }
            Err(_) => Ok(vec![]), // No passphrase available
        }
    }

    fn backend_info(&self) -> BackendInfo {
        BackendInfo::keyring()
    }

    fn supports_storage(&self) -> bool {
        false // Only stores passphrases, not raw keys
    }

    fn supports_rotation(&self) -> bool {
        false // Manual process only
    }
}

impl Default for KeyringBackend {
    fn default() -> Self {
        Self::new().expect("Failed to create default keyring backend")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyring_backend_creation() {
        let backend = KeyringBackend::new();
        assert!(backend.is_ok());
    }

    #[test]
    fn test_backend_info() {
        let backend = KeyringBackend::new().unwrap();
        let info = backend.backend_info();
        assert_eq!(info.name, "keyring");
        assert!(info.available);
    }

    #[test]
    fn test_key_derivation_requires_16_byte_salt() {
        let backend = KeyringBackend::new().unwrap();
        let key_id = [1u8; 16];

        // Test with wrong salt length
        let context = KeyContext::new(vec![1, 2, 3]); // Only 3 bytes
        let result = backend.derive_key(&key_id, &context);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("16 bytes"));
    }
}
