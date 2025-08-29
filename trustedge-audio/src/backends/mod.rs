//! Backend abstraction for key management
//!
//! This module provides a pluggable backend system for key management operations.
//! Currently supports:
//! - Keyring backend (PBKDF2 with OS keyring)
//! 
//! Planned backends:
//! - TPM 2.0 backend
//! - HSM backend (PKCS#11)
//! - Matter certificate backend

pub mod traits;
pub mod keyring;

pub use traits::*;
pub use keyring::KeyringBackend;

use anyhow::Result;

/// Backend registry for selecting and instantiating key backends
pub struct BackendRegistry {
    // Future: registry of available backends
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
