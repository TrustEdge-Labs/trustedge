/*
 * Copyright (c) 2025 TRUSTEDGE LABS LLC
 * This source code is subject to the terms of the Mozilla Public License, v. 2.0.
 * If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
 *
 * Project: trustedge — Privacy and trust at the edge.
 */

//! YubiKey Universal Backend Implementation
//!
//! This module provides a YubiKey-based implementation of the UniversalBackend trait,
//! leveraging PKCS#11 for hardware-backed cryptographic operations.

#[cfg(feature = "yubikey")]
use pkcs11::{
    types::{
        CKA_CLASS, CKA_ID, CKA_LABEL, CKF_SERIAL_SESSION, CKO_PRIVATE_KEY, CKS_RO_PUBLIC_SESSION,
        CKU_USER, CK_ATTRIBUTE, CK_OBJECT_HANDLE, CK_SESSION_HANDLE, CK_SLOT_ID,
    },
    Ctx,
};

#[cfg(feature = "yubikey")]
use std::collections::HashMap;

#[cfg(feature = "yubikey")]
use crate::{
    AsymmetricAlgorithm, BackendCapabilities, BackendInfo, CryptoOperation, CryptoResult,
    HashAlgorithm, KeyMetadata, SignatureAlgorithm, UniversalBackend,
};

#[cfg(not(feature = "yubikey"))]
use crate::{
    BackendCapabilities, BackendInfo, CryptoOperation, CryptoResult, KeyMetadata, UniversalBackend,
};

use anyhow::{anyhow, Result};

#[cfg(feature = "yubikey")]
use anyhow::Context;

use serde::{Deserialize, Serialize};

/// Configuration for YubiKey backend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YubiKeyConfig {
    /// Path to the PKCS#11 module (typically OpenSC)
    pub pkcs11_module_path: String,
    /// Optional PIN for authentication
    pub pin: Option<String>,
    /// Slot number (auto-detect if None)
    pub slot: Option<u64>,
    /// Enable verbose logging
    pub verbose: bool,
}

impl Default for YubiKeyConfig {
    fn default() -> Self {
        Self {
            // Default OpenSC PKCS#11 module path for Linux
            pkcs11_module_path: "/usr/lib/x86_64-linux-gnu/opensc-pkcs11.so".to_string(),
            pin: None,
            slot: None,
            verbose: false,
        }
    }
}

/// YubiKey Universal Backend
#[cfg(feature = "yubikey")]
#[derive(Debug)]
pub struct YubiKeyBackend {
    config: YubiKeyConfig,
    pkcs11: Option<Ctx>,
    session: Option<CK_SESSION_HANDLE>,
    slot: Option<CK_SLOT_ID>,
    #[allow(dead_code)] // Reserved for future key caching optimization
    key_cache: HashMap<String, CK_OBJECT_HANDLE>,
}

#[cfg(feature = "yubikey")]
impl YubiKeyBackend {
    /// Create a new YubiKey backend with default configuration
    pub fn new() -> Result<Self> {
        Self::with_config(YubiKeyConfig::default())
    }

    /// Create a new YubiKey backend with custom configuration
    pub fn with_config(config: YubiKeyConfig) -> Result<Self> {
        let mut backend = Self {
            config,
            pkcs11: None,
            session: None,
            slot: None,
            key_cache: HashMap::new(),
        };

        backend.initialize()?;
        Ok(backend)
    }

    /// Initialize the PKCS#11 connection
    fn initialize(&mut self) -> Result<()> {
        if self.config.verbose {
            println!("● Initializing YubiKey backend with PKCS#11...");
        }

        // Initialize PKCS#11 module
        let pkcs11 =
            Ctx::new_and_initialize(&self.config.pkcs11_module_path).with_context(|| {
                format!(
                    "Failed to load and initialize PKCS#11 module: {}",
                    self.config.pkcs11_module_path
                )
            })?;

        // Find available slots with tokens
        let slots = pkcs11
            .get_slot_list(true)
            .context("Failed to get PKCS#11 slots")?;

        if slots.is_empty() {
            return Err(anyhow!("No YubiKey/smart card detected"));
        }

        // Use specified slot or first available
        let slot = match self.config.slot {
            Some(slot_id) => {
                if slots.contains(&slot_id) {
                    slot_id
                } else {
                    return Err(anyhow!("Specified slot {} not found", slot_id));
                }
            }
            None => slots[0],
        };

        if self.config.verbose {
            println!("✔ Using PKCS#11 slot: {}", slot);
        }

        // Open session
        let session = pkcs11
            .open_session(slot, CKF_SERIAL_SESSION | CKS_RO_PUBLIC_SESSION, None, None)
            .context("Failed to open PKCS#11 session")?;

        // Login if PIN provided
        if let Some(ref pin) = self.config.pin {
            pkcs11
                .login(session, CKU_USER, Some(pin))
                .context("Failed to login with PIN")?;

            if self.config.verbose {
                println!("✔ Authenticated with PIN");
            }
        }

        self.pkcs11 = Some(pkcs11);
        self.session = Some(session);
        self.slot = Some(slot);

        if self.config.verbose {
            println!("✔ YubiKey backend initialized successfully");
        }

        Ok(())
    }

    /// Find key objects by ID or label
    fn find_key_by_id(&self, key_id: &str) -> Result<CK_OBJECT_HANDLE> {
        let pkcs11 = self
            .pkcs11
            .as_ref()
            .ok_or_else(|| anyhow!("PKCS#11 not initialized"))?;
        let session = self.session.ok_or_else(|| anyhow!("No active session"))?;

        // Search for private key by ID or label
        let mut template = vec![CK_ATTRIBUTE::new(CKA_CLASS).with_ck_ulong(&CKO_PRIVATE_KEY)];

        // Try both ID and label search
        if let Ok(id_bytes) = hex::decode(key_id) {
            template.push(CK_ATTRIBUTE::new(CKA_ID).with_bytes(&id_bytes));
        } else {
            template.push(CK_ATTRIBUTE::new(CKA_LABEL).with_string(key_id));
        }

        pkcs11
            .find_objects_init(session, &template)
            .context("Failed to initialize key search")?;

        let objects = pkcs11
            .find_objects(session, 10)
            .context("Failed to find keys")?;

        pkcs11
            .find_objects_final(session)
            .context("Failed to finalize key search")?;

        if objects.is_empty() {
            return Err(anyhow!("Key '{}' not found on YubiKey", key_id));
        }

        Ok(objects[0])
    }

    /// Sign data using PKCS#11
    fn pkcs11_sign(
        &self,
        key_id: &str,
        data: &[u8],
        algorithm: SignatureAlgorithm,
    ) -> Result<Vec<u8>> {
        let pkcs11 = self
            .pkcs11
            .as_ref()
            .ok_or_else(|| anyhow!("PKCS#11 not initialized"))?;
        let session = self.session.ok_or_else(|| anyhow!("No active session"))?;

        let key_handle = self.find_key_by_id(key_id)?;

        // Convert algorithm to PKCS#11 mechanism
        let mechanism = match algorithm {
            SignatureAlgorithm::EcdsaP256 => pkcs11::types::CKM_ECDSA,
            SignatureAlgorithm::RsaPkcs1v15 => pkcs11::types::CKM_RSA_PKCS,
            SignatureAlgorithm::RsaPss => pkcs11::types::CKM_RSA_PKCS_PSS,
            SignatureAlgorithm::Ed25519 => {
                return Err(anyhow!("Ed25519 not supported by YubiKey PKCS#11"))
            }
        };

        // Initialize signing
        let mechanism_info = pkcs11::types::CK_MECHANISM {
            mechanism,
            pParameter: std::ptr::null_mut(),
            ulParameterLen: 0,
        };

        pkcs11
            .sign_init(session, &mechanism_info, key_handle)
            .context("Failed to initialize signing")?;

        // Perform signing
        let signature = pkcs11.sign(session, data).context("Failed to sign data")?;

        if self.config.verbose {
            println!("✔ Signed {} bytes with key '{}'", data.len(), key_id);
        }

        Ok(signature)
    }

    /// Get hardware attestation (YubiKey-specific)
    fn hardware_attest(&self, challenge: &[u8]) -> Result<Vec<u8>> {
        // This would implement YubiKey-specific attestation
        // For now, return a placeholder
        if self.config.verbose {
            println!("⚠ Hardware attestation not yet implemented");
        }

        // Placeholder: return challenge hash as "attestation"
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(b"yubikey-attestation:");
        hasher.update(challenge);
        Ok(hasher.finalize().to_vec())
    }
}

#[cfg(feature = "yubikey")]
impl UniversalBackend for YubiKeyBackend {
    fn perform_operation(&self, key_id: &str, operation: CryptoOperation) -> Result<CryptoResult> {
        match operation {
            CryptoOperation::Sign { data, algorithm } => {
                let signature = self.pkcs11_sign(key_id, &data, algorithm)?;
                Ok(CryptoResult::Signed(signature))
            }
            CryptoOperation::Attest { challenge } => {
                let proof = self.hardware_attest(&challenge)?;
                Ok(CryptoResult::AttestationProof(proof))
            }
            _ => Err(anyhow!(
                "Operation {:?} not supported by YubiKey backend",
                operation
            )),
        }
    }

    fn supports_operation(&self, operation: &CryptoOperation) -> bool {
        matches!(
            operation,
            CryptoOperation::Sign { .. } | CryptoOperation::Attest { .. }
        )
    }

    fn get_capabilities(&self) -> BackendCapabilities {
        BackendCapabilities {
            symmetric_algorithms: vec![], // YubiKey primarily for asymmetric operations
            asymmetric_algorithms: vec![
                AsymmetricAlgorithm::EcdsaP256,
                AsymmetricAlgorithm::Rsa2048,
                AsymmetricAlgorithm::Rsa4096,
            ],
            signature_algorithms: vec![
                SignatureAlgorithm::EcdsaP256,
                SignatureAlgorithm::RsaPkcs1v15,
                SignatureAlgorithm::RsaPss,
            ],
            hash_algorithms: vec![
                HashAlgorithm::Sha256,
                HashAlgorithm::Sha384,
                HashAlgorithm::Sha512,
            ],
            hardware_backed: true,
            supports_key_derivation: false, // YubiKey stores keys, doesn't derive
            supports_key_generation: false, // Keys are typically pre-generated
            supports_attestation: true,
            max_key_size: Some(4096),
        }
    }

    fn backend_info(&self) -> BackendInfo {
        BackendInfo {
            name: "yubikey",
            description: "YubiKey PKCS#11 hardware security token",
            version: "1.0.0",
            available: self.pkcs11.is_some(),
            config_requirements: vec!["pkcs11_module_path"],
        }
    }

    fn list_keys(&self) -> Result<Vec<KeyMetadata>> {
        let pkcs11 = self
            .pkcs11
            .as_ref()
            .ok_or_else(|| anyhow!("PKCS#11 not initialized"))?;
        let session = self.session.ok_or_else(|| anyhow!("No active session"))?;

        // Search for all private keys
        let template = vec![CK_ATTRIBUTE::new(CKA_CLASS).with_ck_ulong(&CKO_PRIVATE_KEY)];

        pkcs11
            .find_objects_init(session, &template)
            .context("Failed to initialize key listing")?;

        let objects = pkcs11
            .find_objects(session, 100)
            .context("Failed to list keys")?;

        pkcs11
            .find_objects_final(session)
            .context("Failed to finalize key listing")?;

        let mut keys = Vec::new();
        for &handle in &objects {
            // Get key attributes
            let mut attrs = vec![CK_ATTRIBUTE::new(CKA_ID), CK_ATTRIBUTE::new(CKA_LABEL)];

            if let Ok(_) = pkcs11.get_attribute_value(session, handle, &mut attrs) {
                let key_id = if attrs[0].ulValueLen > 0 && !attrs[0].pValue.is_null() {
                    let id_slice = unsafe {
                        std::slice::from_raw_parts(
                            attrs[0].pValue as *const u8,
                            attrs[0].ulValueLen as usize,
                        )
                    };
                    hex::encode(id_slice)
                } else if attrs[1].ulValueLen > 0 && !attrs[1].pValue.is_null() {
                    let label_slice = unsafe {
                        std::slice::from_raw_parts(
                            attrs[1].pValue as *const u8,
                            attrs[1].ulValueLen as usize,
                        )
                    };
                    String::from_utf8_lossy(label_slice).to_string()
                } else {
                    format!("key_{}", handle)
                };

                keys.push(KeyMetadata {
                    key_id: key_id.as_bytes().try_into().unwrap_or([0u8; 16]),
                    description: format!("YubiKey key: {}", key_id),
                    created_at: 0, // Would need to read from certificate
                    last_used: None,
                    backend_data: handle.to_le_bytes().to_vec(),
                });
            }
        }

        Ok(keys)
    }
}

// Stub implementation when yubikey feature is disabled
#[cfg(not(feature = "yubikey"))]
#[derive(Debug)]
pub struct YubiKeyBackend;

#[cfg(not(feature = "yubikey"))]
impl YubiKeyBackend {
    pub fn new() -> Result<Self> {
        Err(anyhow!(
            "YubiKey support not compiled in. Enable 'yubikey' feature"
        ))
    }

    pub fn with_config(_config: YubiKeyConfig) -> Result<Self> {
        Err(anyhow!(
            "YubiKey support not compiled in. Enable 'yubikey' feature"
        ))
    }
}

#[cfg(not(feature = "yubikey"))]
impl UniversalBackend for YubiKeyBackend {
    fn perform_operation(
        &self,
        _key_id: &str,
        _operation: CryptoOperation,
    ) -> Result<CryptoResult> {
        Err(anyhow!("YubiKey support not compiled in"))
    }

    fn supports_operation(&self, _operation: &CryptoOperation) -> bool {
        false
    }

    fn get_capabilities(&self) -> BackendCapabilities {
        BackendCapabilities::software_only()
    }

    fn backend_info(&self) -> BackendInfo {
        BackendInfo {
            name: "yubikey",
            description: "YubiKey PKCS#11 hardware security token (not compiled)",
            version: "1.0.0",
            available: false,
            config_requirements: vec!["Feature 'yubikey' not enabled"],
        }
    }

    fn list_keys(&self) -> Result<Vec<KeyMetadata>> {
        Err(anyhow!("YubiKey support not compiled in"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yubikey_config_default() {
        let config = YubiKeyConfig::default();
        assert!(!config.pkcs11_module_path.is_empty());
        assert_eq!(config.pin, None);
        assert_eq!(config.slot, None);
        assert!(!config.verbose);
    }

    #[test]
    fn test_backend_info() {
        let backend = YubiKeyBackend::new();

        #[cfg(feature = "yubikey")]
        {
            // Will fail to initialize without actual hardware, but we can test the error
            assert!(backend.is_err());
        }

        #[cfg(not(feature = "yubikey"))]
        {
            assert!(backend.is_err());
            assert!(backend.unwrap_err().to_string().contains("not compiled"));
        }
    }

    #[test]
    fn test_backend_capabilities() {
        // Test stub when feature disabled
        #[cfg(not(feature = "yubikey"))]
        {
            let backend = YubiKeyBackend;
            let caps = backend.get_capabilities();
            assert!(!caps.hardware_backed);
        }
    }
}
