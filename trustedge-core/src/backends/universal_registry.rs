//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/
//
// Project: trustedge — Privacy and trust at the edge.
//

//! Universal backend registry for managing multiple crypto backends
//!
//! This module provides capability-based backend selection and management.

use crate::backends::software_hsm::SoftwareHsmBackend;
use crate::backends::universal::*;
use crate::backends::universal_keyring::UniversalKeyringBackend;
use anyhow::{anyhow, Result};
use std::collections::HashMap;

/// Registry for managing universal crypto backends
///
/// The UniversalBackendRegistry provides centralized management of cryptographic
/// backends, allowing applications to discover capabilities and select appropriate
/// backends for specific operations at runtime.
pub struct UniversalBackendRegistry {
    backends: HashMap<String, Box<dyn UniversalBackend>>,
}

impl UniversalBackendRegistry {
    /// Create a new registry
    pub fn new() -> Self {
        Self {
            backends: HashMap::new(),
        }
    }

    /// Create a registry with default backends
    pub fn with_defaults() -> Result<Self> {
        let mut registry = Self::new();

        // Add keyring backend if available
        if let Ok(keyring_backend) = UniversalKeyringBackend::new() {
            registry.register_backend("keyring".to_string(), Box::new(keyring_backend));
        }

        // Add Software HSM backend if available
        if let Ok(software_hsm_backend) = SoftwareHsmBackend::new() {
            registry.register_backend("software_hsm".to_string(), Box::new(software_hsm_backend));
        }

        // Future: Add other backends
        // if let Ok(yubikey_backend) = YubiKeyBackend::new() {
        //     registry.register_backend("yubikey".to_string(), Box::new(yubikey_backend));
        // }

        Ok(registry)
    }

    /// Register a backend with the registry
    pub fn register_backend(&mut self, name: String, backend: Box<dyn UniversalBackend>) {
        self.backends.insert(name, backend);
    }

    /// Get a backend by name
    pub fn get_backend(&self, name: &str) -> Option<&dyn UniversalBackend> {
        self.backends.get(name).map(|b| b.as_ref())
    }

    /// Find the first backend that supports a specific operation
    pub fn find_backend_for_operation(
        &self,
        operation: &CryptoOperation,
    ) -> Option<&dyn UniversalBackend> {
        self.backends
            .values()
            .find(|backend| backend.supports_operation(operation))
            .map(|b| b.as_ref())
    }

    /// Find all backends that support a specific operation
    pub fn find_all_backends_for_operation(
        &self,
        operation: &CryptoOperation,
    ) -> Vec<(&str, &dyn UniversalBackend)> {
        self.backends
            .iter()
            .filter(|(_, backend)| backend.supports_operation(operation))
            .map(|(name, backend)| (name.as_str(), backend.as_ref()))
            .collect()
    }

    /// List all registered backend names
    pub fn list_backend_names(&self) -> Vec<&str> {
        self.backends.keys().map(|s| s.as_str()).collect()
    }

    /// Get capabilities for all backends
    pub fn get_all_capabilities(&self) -> HashMap<String, BackendCapabilities> {
        self.backends
            .iter()
            .map(|(name, backend)| (name.clone(), backend.get_capabilities()))
            .collect()
    }

    /// Find the best backend for an operation based on preferences
    pub fn find_preferred_backend(
        &self,
        operation: &CryptoOperation,
        preferences: &BackendPreferences,
    ) -> Option<&dyn UniversalBackend> {
        let mut candidates: Vec<_> = self
            .backends
            .iter()
            .filter(|(_, backend)| backend.supports_operation(operation))
            .collect();

        if candidates.is_empty() {
            return None;
        }

        // Sort by preferences
        candidates.sort_by(|(_, a), (_, b)| {
            let a_caps = a.get_capabilities();
            let b_caps = b.get_capabilities();

            // Prefer hardware-backed if requested
            if preferences.prefer_hardware_backed {
                match (a_caps.hardware_backed, b_caps.hardware_backed) {
                    (true, false) => return std::cmp::Ordering::Less,
                    (false, true) => return std::cmp::Ordering::Greater,
                    _ => {}
                }
            }

            // Prefer higher security (more algorithms supported)
            let a_score = a_caps.symmetric_algorithms.len()
                + a_caps.asymmetric_algorithms.len()
                + a_caps.signature_algorithms.len();
            let b_score = b_caps.symmetric_algorithms.len()
                + b_caps.asymmetric_algorithms.len()
                + b_caps.signature_algorithms.len();

            b_score.cmp(&a_score) // Higher score first
        });

        // Return the best candidate, avoiding lifetime issues
        if let Some((_, backend)) = candidates.first() {
            // Get the backend name to look it up again
            let backend_name = backend.backend_info().name;
            self.get_backend(backend_name)
        } else {
            None
        }
    }

    /// Perform an operation using the best available backend
    pub fn perform_operation(
        &self,
        key_id: &str,
        operation: CryptoOperation,
        preferences: Option<&BackendPreferences>,
    ) -> Result<CryptoResult> {
        let backend = if let Some(prefs) = preferences {
            self.find_preferred_backend(&operation, prefs)
        } else {
            self.find_backend_for_operation(&operation)
        };

        match backend {
            Some(backend) => backend.perform_operation(key_id, operation),
            None => Err(anyhow!(
                "No backend supports the requested operation: {:?}",
                operation
            )),
        }
    }
}

impl Default for UniversalBackendRegistry {
    fn default() -> Self {
        Self::with_defaults().unwrap_or_else(|_| Self::new())
    }
}

/// Preferences for backend selection
///
/// BackendPreferences allows applications to specify their requirements
/// for backend selection, such as preferring hardware-backed operations
/// or excluding certain backend types.
#[derive(Debug, Clone)]
pub struct BackendPreferences {
    /// Prefer hardware-backed backends over software ones
    pub prefer_hardware_backed: bool,
    /// Prefer backends with attestation capabilities
    pub prefer_attestation: bool,
    /// Exclude specific backend types
    pub excluded_backends: Vec<String>,
    /// Prefer specific backend types (in order of preference)
    pub preferred_backends: Vec<String>,
}

impl BackendPreferences {
    /// Create preferences suitable for most applications
    ///
    /// Returns preferences that accept any available backend, with no
    /// particular preference for hardware or software implementations.
    pub fn new() -> Self {
        Self {
            prefer_hardware_backed: false,
            prefer_attestation: false,
            excluded_backends: vec![],
            preferred_backends: vec![],
        }
    }

    /// Create preferences favoring hardware security
    ///
    /// Returns preferences that favor hardware-backed backends with
    /// attestation capabilities, suitable for high-security applications.
    pub fn hardware_preferred() -> Self {
        Self {
            prefer_hardware_backed: true,
            prefer_attestation: true,
            excluded_backends: vec![],
            preferred_backends: vec!["yubikey".to_string(), "tpm".to_string(), "hsm".to_string()],
        }
    }

    /// Create preferences for maximum compatibility
    ///
    /// Returns preferences that prioritize compatibility and availability
    /// over security features, suitable for development and testing.
    pub fn maximum_compatibility() -> Self {
        Self {
            prefer_hardware_backed: false,
            prefer_attestation: false,
            excluded_backends: vec![],
            preferred_backends: vec!["keyring".to_string()],
        }
    }
}

impl Default for BackendPreferences {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_creation() {
        let registry = UniversalBackendRegistry::new();
        assert_eq!(registry.list_backend_names().len(), 0);
    }

    #[test]
    fn test_registry_with_defaults() {
        let registry = UniversalBackendRegistry::with_defaults();
        assert!(registry.is_ok());

        let registry = registry.unwrap();
        let backends = registry.list_backend_names();

        // Should have keyring backend
        assert!(backends.contains(&"keyring"));
    }

    #[test]
    fn test_find_backend_for_operation() {
        let registry = UniversalBackendRegistry::with_defaults().unwrap();

        // Test key derivation (should find keyring)
        let derive_op = CryptoOperation::DeriveKey {
            context: KeyDerivationContext::new(vec![1; 16]),
        };
        let backend = registry.find_backend_for_operation(&derive_op);
        assert!(backend.is_some());

        // Test signing (should find Software HSM)
        let sign_op = CryptoOperation::Sign {
            data: vec![1, 2, 3],
            algorithm: SignatureAlgorithm::Ed25519,
        };
        let backend = registry.find_backend_for_operation(&sign_op);
        assert!(backend.is_some());
    }

    #[test]
    fn test_backend_preferences() {
        let prefs = BackendPreferences::hardware_preferred();
        assert!(prefs.prefer_hardware_backed);
        assert!(prefs.prefer_attestation);

        let prefs = BackendPreferences::maximum_compatibility();
        assert!(!prefs.prefer_hardware_backed);
        assert!(!prefs.prefer_attestation);
    }

    #[test]
    fn test_perform_operation_via_registry() {
        let registry = UniversalBackendRegistry::with_defaults().unwrap();

        // Test hash operation (should work)
        let hash_op = CryptoOperation::Hash {
            data: b"test data".to_vec(),
            algorithm: HashAlgorithm::Sha256,
        };

        let result = registry.perform_operation("test_key", hash_op, None);
        assert!(result.is_ok());

        if let Ok(CryptoResult::Hash(hash)) = result {
            assert_eq!(hash.len(), 32); // SHA-256 hash length
        } else {
            panic!("Expected hash result");
        }
    }
}
