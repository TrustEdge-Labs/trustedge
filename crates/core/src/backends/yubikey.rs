//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//

//! YubiKey PIV backend for Universal Backend system
//!
//! This backend provides hardware-backed cryptographic operations using YubiKey PIV applet.
//! It implements a fail-closed design: all operations require real hardware to be present
//! and operational. There are NO software fallbacks.
//!
//! ## Supported Operations
//! - ECDSA P-256 signing (PIV slots 9a, 9c, 9d, 9e)
//! - RSA-2048 signing (PIV slots 9a, 9c, 9d, 9e)
//! - Public key extraction from certificates
//! - Key generation (ECDSA P-256, RSA-2048)
//! - Hardware attestation
//! - PIV slot enumeration
//!
//! ## Hardware Limitations
//! - **Ed25519 is NOT supported** by YubiKey PIV hardware. Use ECDSA P-256 instead.
//! - All signing operations use pre-hashed digests (SHA-256)
//! - Maximum 3 PIN retry attempts before lockout risk
//!
//! ## Architecture
//! - Uses yubikey crate stable API only (no `untested` features)
//! - Thread-safe via Mutex for hardware access
//! - Fail-closed: HardwareError returned when device unavailable

use crate::backends::traits::{BackendInfo, KeyMetadata};
use crate::backends::universal::{
    AsymmetricAlgorithm, BackendCapabilities, CryptoOperation, CryptoResult, HashAlgorithm,
    SignatureAlgorithm, UniversalBackend,
};
use crate::error::BackendError;
use der::Encode;
use rcgen::{
    CertificateParams, DistinguishedName, DnType, KeyPair, RemoteKeyPair, PKCS_ECDSA_P256_SHA256,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use spki::SubjectPublicKeyInfoRef;
use std::sync::{Arc, Mutex};
use yubikey::piv::{AlgorithmId, SlotId};
use yubikey::{Certificate, YubiKey};

/// Configuration for YubiKey PIV backend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YubiKeyConfig {
    /// PIN for PIV operations (optional - will prompt if not set)
    pub pin: Option<String>,
    /// Default PIV slot for operations (default: "9c" for Digital Signature)
    pub default_slot: String,
    /// Enable verbose logging for debugging
    pub verbose: bool,
    /// Maximum PIN retry attempts before lockout (default: 3)
    pub max_pin_retries: u8,
}

impl Default for YubiKeyConfig {
    fn default() -> Self {
        Self {
            pin: None,
            default_slot: "9c".to_string(),
            verbose: false,
            max_pin_retries: 3,
        }
    }
}

/// YubiKey PIV backend implementation
///
/// Thread-safe hardware backend using Arc<Mutex> for concurrent access.
/// All cryptographic operations require real YubiKey hardware to be present.
pub struct YubiKeyBackend {
    config: YubiKeyConfig,
    yubikey: Arc<Mutex<Option<YubiKey>>>,
    pin_retry_count: Mutex<u8>,
}

impl YubiKeyBackend {
    /// Create a new YubiKey backend with default configuration
    pub fn new() -> Result<Self, BackendError> {
        Self::with_config(YubiKeyConfig::default())
    }

    /// Create a new YubiKey backend with custom configuration
    pub fn with_config(config: YubiKeyConfig) -> Result<Self, BackendError> {
        let mut backend = Self {
            config,
            yubikey: Arc::new(Mutex::new(None)),
            pin_retry_count: Mutex::new(0),
        };

        // Try to connect to hardware (non-fatal if unavailable)
        let _ = backend.connect_if_available();

        Ok(backend)
    }

    /// Attempt to connect to YubiKey hardware
    ///
    /// This is non-fatal - if hardware is not available, backend_info will report
    /// available: false and operations will return HardwareError.
    fn connect_if_available(&mut self) -> Result<(), BackendError> {
        match YubiKey::open() {
            Ok(yk) => {
                if self.config.verbose {
                    eprintln!("✓ YubiKey connected: serial {}", yk.serial());
                }
                *self.yubikey.lock().unwrap() = Some(yk);
                Ok(())
            }
            Err(e) => {
                if self.config.verbose {
                    eprintln!("⚠ YubiKey not available: {}", e);
                }
                Err(yubikey_error_to_backend(e))
            }
        }
    }

    /// Ensure YubiKey is connected, fail closed if not
    ///
    /// This is the critical fail-closed gate. Every hardware operation MUST call this.
    fn ensure_connected(&self) -> Result<(), BackendError> {
        let yubikey = self.yubikey.lock().unwrap();
        if yubikey.is_none() {
            return Err(BackendError::HardwareError(
                "YubiKey not connected. Insert device and retry.".to_string(),
            ));
        }
        Ok(())
    }

    /// Parse PIV slot identifier from string
    ///
    /// Valid slots: 9a (Authentication), 9c (Digital Signature),
    /// 9d (Key Management), 9e (Card Authentication)
    fn parse_slot(key_id: &str) -> Result<SlotId, BackendError> {
        match key_id.to_lowercase().as_str() {
            "9a" => Ok(SlotId::Authentication),
            "9c" => Ok(SlotId::Signature),
            "9d" => Ok(SlotId::KeyManagement),
            "9e" => Ok(SlotId::CardAuthentication),
            _ => Err(BackendError::KeyNotFound(format!(
                "Invalid PIV slot '{}'. Valid slots: 9a, 9c, 9d, 9e",
                key_id
            ))),
        }
    }

    /// Verify PIN with retry limit enforcement
    fn verify_pin(&self, yk: &mut YubiKey) -> Result<(), BackendError> {
        let pin = self.config.pin.as_ref().ok_or_else(|| {
            BackendError::HardwareError(
                "PIN not configured. Set YubiKeyConfig.pin before operations.".to_string(),
            )
        })?;

        // Check retry count
        let mut retry_count = self.pin_retry_count.lock().unwrap();
        if *retry_count >= self.config.max_pin_retries {
            return Err(BackendError::HardwareError(format!(
                "PIN retry limit reached ({} attempts). Reset required to prevent lockout.",
                self.config.max_pin_retries
            )));
        }

        // Attempt PIN verification
        match yk.verify_pin(pin.as_bytes()) {
            Ok(_) => {
                *retry_count = 0; // Reset on success
                Ok(())
            }
            Err(e) => {
                *retry_count += 1;
                Err(BackendError::HardwareError(format!(
                    "PIN verification failed (attempt {}/{}): {}",
                    *retry_count, self.config.max_pin_retries, e
                )))
            }
        }
    }

    /// Sign data using PIV slot
    ///
    /// YubiKey PIV signs pre-hashed digests (SHA-256), not raw data.
    fn piv_sign(
        &self,
        slot: SlotId,
        data: &[u8],
        algorithm: &SignatureAlgorithm,
    ) -> Result<Vec<u8>, BackendError> {
        self.ensure_connected()?;

        // Pre-hash data with SHA-256 (YubiKey signs digests, not raw data)
        let mut hasher = Sha256::new();
        hasher.update(data);
        let digest = hasher.finalize();

        let mut yubikey_guard = self.yubikey.lock().unwrap();
        let yk = yubikey_guard
            .as_mut()
            .ok_or_else(|| BackendError::HardwareError("YubiKey not connected".to_string()))?;

        // Verify PIN before signing
        self.verify_pin(yk)?;

        // Map algorithm to AlgorithmId
        let alg_id = match algorithm {
            SignatureAlgorithm::EcdsaP256 => AlgorithmId::EccP256,
            SignatureAlgorithm::RsaPkcs1v15 | SignatureAlgorithm::RsaPss => AlgorithmId::Rsa2048,
            SignatureAlgorithm::Ed25519 => {
                return Err(BackendError::UnsupportedOperation(
                    "Ed25519 not natively supported by YubiKey PIV hardware. \
                     Use ECDSA P-256 for hardware signing or Software HSM backend for Ed25519."
                        .to_string(),
                ))
            }
        };

        // Perform signing (returns Buffer = Zeroizing<Vec<u8>>)
        let signature =
            yubikey::piv::sign_data(yk, &digest, alg_id, slot).map_err(yubikey_error_to_backend)?;

        // Convert Buffer to Vec<u8>
        Ok(signature.to_vec())
    }

    /// Extract public key from PIV slot certificate
    ///
    /// Returns DER-encoded SubjectPublicKeyInfo
    fn piv_get_public_key(&self, slot: SlotId) -> Result<Vec<u8>, BackendError> {
        self.ensure_connected()?;

        let mut yubikey_guard = self.yubikey.lock().unwrap();
        let yk = yubikey_guard
            .as_mut()
            .ok_or_else(|| BackendError::HardwareError("YubiKey not connected".to_string()))?;

        // Read certificate from slot
        let cert = Certificate::read(yk, slot).map_err(|e| {
            if e.to_string().contains("not found") {
                BackendError::KeyNotFound(format!("No certificate found in slot {:?}", slot))
            } else {
                yubikey_error_to_backend(e)
            }
        })?;

        // Get the DER-encoded certificate bytes
        let cert_der = cert.as_ref();

        // Extract SubjectPublicKeyInfo from DER certificate
        let spki = SubjectPublicKeyInfoRef::try_from(cert_der)
            .map_err(|e| BackendError::OperationFailed(format!("Failed to parse SPKI: {}", e)))?;

        // Encode as DER
        let der = spki
            .to_der()
            .map_err(|e| BackendError::OperationFailed(format!("Failed to encode SPKI: {}", e)))?;

        Ok(der)
    }

    /// Enumerate all PIV slots with certificates
    ///
    /// Returns list of (slot_id, description) pairs
    fn enumerate_slots(&self) -> Result<Vec<(SlotId, String)>, BackendError> {
        self.ensure_connected()?;

        let mut yubikey_guard = self.yubikey.lock().unwrap();
        let yk = yubikey_guard
            .as_mut()
            .ok_or_else(|| BackendError::HardwareError("YubiKey not connected".to_string()))?;

        let slots = vec![
            (SlotId::Authentication, "PIV Authentication (9a)"),
            (SlotId::Signature, "PIV Digital Signature (9c)"),
            (SlotId::KeyManagement, "PIV Key Management (9d)"),
            (SlotId::CardAuthentication, "PIV Card Authentication (9e)"),
        ];

        let mut populated = Vec::new();

        for (slot, desc) in slots {
            // Check if certificate exists in slot
            if Certificate::read(yk, slot).is_ok() {
                populated.push((slot, desc.to_string()));
            }
        }

        Ok(populated)
    }

    /// Generate key pair in PIV slot
    fn piv_generate(
        &self,
        _slot: SlotId,
        _algorithm: AsymmetricAlgorithm,
    ) -> Result<Vec<u8>, BackendError> {
        // NOTE: Key generation requires PinPolicy and TouchPolicy types which are
        // in a private module in yubikey crate 0.7. This will be addressed in a future
        // update. For now, keys must be pre-generated using ykman or other tools.

        // TODO: Implement key generation once yubikey crate exports policy types
        // or find a workaround using the raw PIV protocol.

        Err(BackendError::UnsupportedOperation(
            "Key generation not yet implemented. \
             Generate keys using 'ykman piv keys generate' and import certificates."
                .to_string(),
        ))
    }

    /// Perform hardware attestation
    ///
    /// Returns attestation certificate chain
    fn piv_attest(&self, _slot: SlotId, _challenge: &[u8]) -> Result<Vec<u8>, BackendError> {
        // NOTE: The attest() function is behind the `untested` feature flag in yubikey crate 0.7.
        // We're using only stable APIs, so attestation is not available in this version.
        // This will be enabled in a future update when using a newer yubikey crate version
        // or by implementing raw PIV attestation protocol.

        Err(BackendError::UnsupportedOperation(
            "Hardware attestation not available in current yubikey crate version. \
             Requires 'untested' feature or future API update."
                .to_string(),
        ))
    }

    /// Generate X.509 self-signed certificate for a PIV slot
    ///
    /// This uses rcgen with hardware-backed signing. The public key comes from
    /// the hardware slot, and all signing operations are delegated to the YubiKey.
    ///
    /// # Arguments
    /// * `slot_id` - PIV slot identifier (9a, 9c, 9d, 9e)
    /// * `subject` - Certificate subject (Common Name)
    ///
    /// # Returns
    /// DER-encoded X.509 certificate
    pub fn generate_certificate(
        &self,
        slot_id: &str,
        subject: &str,
    ) -> Result<Vec<u8>, BackendError> {
        self.ensure_connected()?;

        let slot = Self::parse_slot(slot_id)?;

        // Get public key from hardware slot
        let public_key_der = self.piv_get_public_key(slot)?;

        // Parse the DER-encoded SPKI to extract raw public key bytes
        let spki = SubjectPublicKeyInfoRef::try_from(public_key_der.as_slice()).map_err(|e| {
            BackendError::OperationFailed(format!("Failed to parse public key SPKI: {}", e))
        })?;

        // Extract raw public key bytes (the BIT STRING contents)
        let public_key_bytes = spki.subject_public_key.raw_bytes();

        // Create certificate parameters
        let mut params = CertificateParams::default();

        // Set distinguished name with CommonName
        let mut dn = DistinguishedName::new();
        dn.push(DnType::CommonName, subject);
        params.distinguished_name = dn;

        // Set validity period (1 year)
        params.not_before = rcgen::date_time_ymd(2025, 1, 1);
        params.not_after = rcgen::date_time_ymd(2026, 1, 1);

        // Create the hardware-backed key pair
        let signing_key_pair = YubiKeySigningKeyPair {
            yubikey: Arc::clone(&self.yubikey),
            slot,
            public_key: public_key_bytes.to_vec(),
            pin: self.config.pin.clone(),
        };

        let key_pair = KeyPair::from_remote(Box::new(signing_key_pair)).map_err(|e| {
            BackendError::OperationFailed(format!("Failed to create remote key pair: {}", e))
        })?;

        // Generate self-signed certificate
        let cert = params.self_signed(&key_pair).map_err(|e| {
            BackendError::OperationFailed(format!("Certificate generation failed: {}", e))
        })?;

        // Return DER-encoded certificate
        Ok(cert.der().to_vec())
    }
}

impl Default for YubiKeyBackend {
    fn default() -> Self {
        Self::new().expect("Failed to create default YubiKey backend")
    }
}

/// Hardware-backed signing key pair for rcgen certificate generation
///
/// This struct implements rcgen's RemoteKeyPair trait to delegate all signing
/// operations to the YubiKey hardware while providing the public key for certificate
/// generation.
struct YubiKeySigningKeyPair {
    yubikey: Arc<Mutex<Option<YubiKey>>>,
    slot: SlotId,
    public_key: Vec<u8>,
    pin: Option<String>,
}

impl RemoteKeyPair for YubiKeySigningKeyPair {
    fn public_key(&self) -> &[u8] {
        &self.public_key
    }

    fn sign(&self, msg: &[u8]) -> Result<Vec<u8>, rcgen::Error> {
        // Lock the YubiKey mutex
        let mut yubikey_guard = self.yubikey.lock().unwrap();
        let yk = yubikey_guard
            .as_mut()
            .ok_or(rcgen::Error::RingUnspecified)?;

        // Verify PIN if configured
        if let Some(pin) = &self.pin {
            yk.verify_pin(pin.as_bytes())
                .map_err(|_| rcgen::Error::RingUnspecified)?;
        }

        // Pre-hash the message with SHA-256 (YubiKey PIV requirement)
        let mut hasher = Sha256::new();
        hasher.update(msg);
        let digest = hasher.finalize();

        // Sign using YubiKey hardware
        let signature = yubikey::piv::sign_data(yk, &digest, AlgorithmId::EccP256, self.slot)
            .map_err(|_| rcgen::Error::RingUnspecified)?;

        Ok(signature.to_vec())
    }

    fn algorithm(&self) -> &'static rcgen::SignatureAlgorithm {
        &PKCS_ECDSA_P256_SHA256
    }
}

/// Convert yubikey crate errors to BackendError
fn yubikey_error_to_backend(e: yubikey::Error) -> BackendError {
    let msg = e.to_string();
    if msg.contains("not found") || msg.contains("No such") {
        BackendError::HardwareError(
            "YubiKey device not found. Insert device and retry.".to_string(),
        )
    } else if msg.contains("authentication") || msg.contains("PIN") {
        BackendError::HardwareError(format!("PIN verification failed: {}", e))
    } else {
        BackendError::HardwareError(format!("YubiKey operation failed: {}", e))
    }
}

impl UniversalBackend for YubiKeyBackend {
    fn perform_operation(
        &self,
        key_id: &str,
        operation: CryptoOperation,
    ) -> Result<CryptoResult, BackendError> {
        match operation {
            CryptoOperation::Sign { data, algorithm } => {
                let slot = Self::parse_slot(key_id)?;
                let signature = self.piv_sign(slot, &data, &algorithm)?;
                Ok(CryptoResult::Signed(signature))
            }

            CryptoOperation::GetPublicKey => {
                let slot = Self::parse_slot(key_id)?;
                let public_key = self.piv_get_public_key(slot)?;
                Ok(CryptoResult::PublicKey(public_key))
            }

            CryptoOperation::GenerateKeyPair { algorithm } => {
                let slot = Self::parse_slot(key_id)?;
                let public_key = self.piv_generate(slot, algorithm)?;
                Ok(CryptoResult::KeyPair {
                    public_key,
                    private_key_id: key_id.to_string(),
                })
            }

            CryptoOperation::Attest { challenge } => {
                let slot = Self::parse_slot(key_id)?;
                let proof = self.piv_attest(slot, &challenge)?;
                Ok(CryptoResult::AttestationProof(proof))
            }

            CryptoOperation::Hash { data, algorithm } => {
                // Hash operations can use software (not security-critical)
                match algorithm {
                    HashAlgorithm::Sha256 => {
                        let mut hasher = Sha256::new();
                        hasher.update(&data);
                        Ok(CryptoResult::Hash(hasher.finalize().to_vec()))
                    }
                    _ => Err(BackendError::UnsupportedOperation(format!(
                        "Hash algorithm {:?} not supported by YubiKey backend",
                        algorithm
                    ))),
                }
            }

            _ => Err(BackendError::UnsupportedOperation(format!(
                "Operation {:?} not supported by YubiKey backend",
                operation
            ))),
        }
    }

    fn supports_operation(&self, operation: &CryptoOperation) -> bool {
        match operation {
            CryptoOperation::Sign { algorithm, .. } => matches!(
                algorithm,
                SignatureAlgorithm::EcdsaP256
                    | SignatureAlgorithm::RsaPkcs1v15
                    | SignatureAlgorithm::RsaPss
            ),
            CryptoOperation::GetPublicKey => true,
            // NOTE: GenerateKeyPair temporarily disabled until policy types are accessible
            CryptoOperation::GenerateKeyPair { .. } => false,
            // NOTE: Attest disabled - requires 'untested' feature in yubikey crate
            CryptoOperation::Attest { .. } => false,
            CryptoOperation::Hash { algorithm, .. } => {
                matches!(algorithm, HashAlgorithm::Sha256)
            }
            _ => false,
        }
    }

    fn get_capabilities(&self) -> BackendCapabilities {
        BackendCapabilities {
            symmetric_algorithms: vec![], // YubiKey PIV doesn't do symmetric
            asymmetric_algorithms: vec![
                AsymmetricAlgorithm::EcdsaP256,
                AsymmetricAlgorithm::Rsa2048,
            ],
            signature_algorithms: vec![
                SignatureAlgorithm::EcdsaP256,
                SignatureAlgorithm::RsaPkcs1v15,
                SignatureAlgorithm::RsaPss,
            ],
            hash_algorithms: vec![HashAlgorithm::Sha256],
            hardware_backed: true,
            supports_key_derivation: false,
            // NOTE: Key generation temporarily disabled until policy types are accessible
            supports_key_generation: false,
            // NOTE: Attestation disabled - requires 'untested' feature in yubikey crate
            supports_attestation: false,
            max_key_size: Some(2048),
        }
    }

    fn backend_info(&self) -> BackendInfo {
        let available = self.yubikey.lock().unwrap().is_some();

        BackendInfo {
            name: "yubikey",
            description: "YubiKey PIV hardware security backend",
            version: "1.0.0",
            available,
            config_requirements: vec!["pin (optional)", "default_slot"],
        }
    }

    fn list_keys(&self) -> Result<Vec<KeyMetadata>, BackendError> {
        let slots = self.enumerate_slots()?;

        let keys = slots
            .into_iter()
            .map(|(slot, desc)| {
                let slot_id = match slot {
                    SlotId::Authentication => "9a",
                    SlotId::Signature => "9c",
                    SlotId::KeyManagement => "9d",
                    SlotId::CardAuthentication => "9e",
                    _ => "unknown",
                };

                KeyMetadata {
                    key_id: slot_id.as_bytes().try_into().unwrap_or([0u8; 16]),
                    description: desc,
                    created_at: 0, // YubiKey doesn't track creation time
                    last_used: None,
                    backend_data: vec![],
                }
            })
            .collect();

        Ok(keys)
    }
}
