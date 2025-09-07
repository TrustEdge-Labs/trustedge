//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/
//
// Project: trustedge — Privacy and trust at the edge.
// GitHub: https://github.com/johnzilla/trustedge
//

//! Backend abstraction for key management
//!
//! This module provides a pluggable backend system for key management operations.
//! Currently supports:
//! - Keyring backend (PBKDF2 with OS keyring)
//! - Software HSM backend (file-based key storage)
//! - Universal backend registry system
//! - YubiKey backend (PKCS#11 hardware tokens)
//!
//! Planned backends:
//! - TPM 2.0 backend
//! - Hardware HSM backend (additional PKCS#11 devices)

pub mod keyring;
pub mod software_hsm;
pub mod traits;
pub mod universal;
pub mod universal_keyring;
pub mod universal_registry;
pub mod yubikey;

pub use keyring::KeyringBackend;
pub use software_hsm::SoftwareHsmBackend;
pub use traits::*;
pub use universal::*;
pub use universal_keyring::UniversalKeyringBackend;
pub use universal_registry::{BackendPreferences, UniversalBackendRegistry};
pub use yubikey::{CertificateParams, HardwareCertificate, YubiKeyBackend, YubiKeyConfig};

use anyhow::Result;

/// Backend registry for selecting and instantiating key backends
pub struct BackendRegistry {
    // Future: registry of available backends
}

impl Default for BackendRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl BackendRegistry {
    pub fn new() -> Self {
        Self {}
    }

    /// Create a backend based on CLI arguments or configuration
    pub fn create_backend(&self, backend_type: &str) -> Result<Box<dyn KeyBackend>> {
        match backend_type {
            "keyring" => Ok(Box::new(KeyringBackend::new()?)),
            // Future backends:
            // "tpm" => Ok(Box::new(TpmBackend::new(device_path)?)),
            // "hsm" => Ok(Box::new(HsmBackend::new(pkcs11_lib, slot_id)?)),
            // "matter" => Ok(Box::new(MatterBackend::new(fabric_id, cert_path)?)),
            _ => Err(anyhow::anyhow!("Unknown backend type: {}", backend_type)),
        }
    }

    /// List available backends on this system
    pub fn list_available_backends(&self) -> Vec<&'static str> {
        let backends = vec!["keyring"]; // Always available

        // Future: detect TPM, HSM availability
        // if tpm_available() { backends.push("tpm"); }
        // if hsm_available() { backends.push("hsm"); }

        backends
    }
}
