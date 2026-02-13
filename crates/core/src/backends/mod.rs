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
//!
//! Planned backends:
//! - YubiKey backend (v1.1 rewrite in progress)
//! - TPM 2.0 backend
//! - Hardware HSM backend (additional PKCS#11 devices)

#[cfg(feature = "keyring")]
pub mod keyring;
pub mod software_hsm;
pub mod traits;
pub mod universal;
#[cfg(feature = "keyring")]
pub mod universal_keyring;
pub mod universal_registry;
#[cfg(feature = "yubikey")]
pub mod yubikey;

#[cfg(feature = "keyring")]
pub use keyring::KeyringBackend;
pub use software_hsm::SoftwareHsmBackend;
pub use traits::*;
pub use universal::*;
#[cfg(feature = "keyring")]
pub use universal_keyring::UniversalKeyringBackend;
pub use universal_registry::{BackendPreferences, UniversalBackendRegistry};
#[cfg(feature = "yubikey")]
pub use yubikey::YubiKeyBackend;

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
            #[cfg(feature = "keyring")]
            "keyring" => Ok(Box::new(KeyringBackend::new()?)),
            #[cfg(not(feature = "keyring"))]
            "keyring" => Err(anyhow::anyhow!("Keyring backend requires the 'keyring' feature. Build with: --features keyring")),
            "pubky" => Err(anyhow::anyhow!(
                "❌ Pubky backend not available in trustedge-core.\n\
                \n\
                🔗 Use the separate 'trustedge-pubky' binary for Pubky operations:\n\
                \n\
                ● Generate identity:\n\
                  trustedge-pubky generate --output my-key.txt\n\
                \n\
                ● Encrypt for someone:\n\
                  trustedge-pubky encrypt --input <file> --output <envelope> --recipient <pubky-id>\n\
                \n\
                ● Decrypt received file:\n\
                  trustedge-pubky decrypt --input <envelope> --output <file> --key <private-key>\n\
                \n\
                ● Resolve Pubky ID:\n\
                  trustedge-pubky resolve <pubky-id> --info\n\
                \n\
                ● Get help:\n\
                  trustedge-pubky --help"
            )),
            // Future backends:
            // "tpm" => Ok(Box::new(TpmBackend::new(device_path)?)),
            // "hsm" => Ok(Box::new(HsmBackend::new(pkcs11_lib, slot_id)?)),
            // "matter" => Ok(Box::new(MatterBackend::new(fabric_id, cert_path)?)),
            _ => Err(anyhow::anyhow!("Unknown backend type: {}", backend_type)),
        }
    }

    /// List available backends on this system
    pub fn list_available_backends(&self) -> Vec<&'static str> {
        #[cfg(feature = "keyring")]
        {
            vec!["keyring"]
        }
        #[cfg(not(feature = "keyring"))]
        {
            vec![]
        }

        // Note: pubky backend is available via separate trustedge-pubky binary
        // Future: detect TPM, HSM availability
        // if tpm_available() { backends.push("tpm"); }
        // if hsm_available() { backends.push("hsm"); }
    }
}
