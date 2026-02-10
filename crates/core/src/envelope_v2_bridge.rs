// Copyright (c) 2025 TRUSTEDGE LABS LLC
// MPL-2.0: https://mozilla.org/MPL/2.0/
// Project: trustedge — Privacy and trust at the edge.

//! Bridge between trustedge-core and trustedge-pubky v2 envelopes
//!
//! This module provides integration between the core hybrid encryption API
//! and the advanced v2 envelope system with Pubky support.

use crate::asymmetric::{PrivateKey, PublicKey};
use crate::hybrid::{open_envelope, seal_for_recipient, HybridEncryptionError};
use anyhow::Result;

/// Envelope format detection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnvelopeFormat {
    /// Original TrustEdge v1 format (symmetric only)
    V1,
    /// Core hybrid format (RSA + AES)
    CoreHybrid,
    /// Advanced v2 format with Pubky integration (X25519 ECDH + AES)
    V2Pubky,
}

/// Detect the format of an envelope from its bytes
pub fn detect_envelope_format(envelope_bytes: &[u8]) -> Result<EnvelopeFormat> {
    if envelope_bytes.len() < 4 {
        return Err(anyhow::anyhow!("Envelope too short to determine format"));
    }

    let magic = &envelope_bytes[0..4];
    match magic {
        b"TRHY" => Ok(EnvelopeFormat::CoreHybrid),
        b"TRS2" => Ok(EnvelopeFormat::V2Pubky),
        _ => {
            // Try to detect v1 format by attempting to deserialize
            // V1 envelopes don't have a magic number, so this is heuristic
            Ok(EnvelopeFormat::V1)
        }
    }
}

/// Unified envelope operations that work with multiple formats
pub struct UnifiedEnvelope;

impl UnifiedEnvelope {
    /// Seal data using the best available format
    ///
    /// This will use the v2 Pubky format if trustedge-pubky is available,
    /// otherwise falls back to the core hybrid format.
    pub fn seal_auto(
        data: &[u8],
        recipient_public_key: &PublicKey,
    ) -> Result<Vec<u8>, HybridEncryptionError> {
        // For now, use the core hybrid format
        // TODO: Integrate with trustedge-pubky when available
        seal_for_recipient(data, recipient_public_key)
    }

    /// Open an envelope of any supported format
    pub fn open_auto(
        envelope_bytes: &[u8],
        private_key: &PrivateKey,
    ) -> Result<Vec<u8>, HybridEncryptionError> {
        let format = detect_envelope_format(envelope_bytes)
            .map_err(|e| HybridEncryptionError::InvalidEnvelope(e.to_string()))?;

        match format {
            EnvelopeFormat::CoreHybrid => open_envelope(envelope_bytes, private_key),
            EnvelopeFormat::V1 => {
                // TODO: Implement v1 envelope opening
                Err(HybridEncryptionError::InvalidEnvelope(
                    "V1 envelope format not yet supported in unified API".to_string(),
                ))
            }
            EnvelopeFormat::V2Pubky => {
                // TODO: Integrate with trustedge-pubky
                Err(HybridEncryptionError::InvalidEnvelope(
                    "V2 Pubky envelope format requires trustedge-pubky integration".to_string(),
                ))
            }
        }
    }

    /// Get information about an envelope without opening it
    pub fn inspect(envelope_bytes: &[u8]) -> Result<EnvelopeInfo> {
        let format = detect_envelope_format(envelope_bytes)?;

        match format {
            EnvelopeFormat::CoreHybrid => Self::inspect_core_hybrid(envelope_bytes),
            EnvelopeFormat::V1 => Ok(EnvelopeInfo {
                format,
                recipient_id: None,
                size: envelope_bytes.len(),
                algorithm: "AES-256-GCM (symmetric)".to_string(),
            }),
            EnvelopeFormat::V2Pubky => Self::inspect_v2_pubky(envelope_bytes),
        }
    }

    fn inspect_core_hybrid(envelope_bytes: &[u8]) -> Result<EnvelopeInfo> {
        use crate::hybrid::HybridEnvelope;

        let envelope: HybridEnvelope = bincode::deserialize(envelope_bytes)
            .map_err(|e| anyhow::anyhow!("Failed to parse core hybrid envelope: {}", e))?;

        Ok(EnvelopeInfo {
            format: EnvelopeFormat::CoreHybrid,
            recipient_id: Some(envelope.recipient_key_id),
            size: envelope_bytes.len(),
            algorithm: "RSA + AES-256-GCM".to_string(),
        })
    }

    fn inspect_v2_pubky(_envelope_bytes: &[u8]) -> Result<EnvelopeInfo> {
        // TODO: Implement v2 Pubky envelope inspection
        Ok(EnvelopeInfo {
            format: EnvelopeFormat::V2Pubky,
            recipient_id: None,
            size: _envelope_bytes.len(),
            algorithm: "X25519 ECDH + AES-256-GCM".to_string(),
        })
    }
}

/// Information about an envelope
#[derive(Debug, Clone)]
pub struct EnvelopeInfo {
    /// The format of the envelope
    pub format: EnvelopeFormat,
    /// The recipient's key ID (if available)
    pub recipient_id: Option<String>,
    /// The total size of the envelope in bytes
    pub size: usize,
    /// Description of the encryption algorithm used
    pub algorithm: String,
}

// Make HybridEnvelope accessible for inspection
pub use crate::hybrid::HybridEnvelope;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::asymmetric::KeyPair;
    use crate::backends::AsymmetricAlgorithm;

    #[test]
    fn test_format_detection() {
        let alice =
            KeyPair::generate(AsymmetricAlgorithm::Rsa2048).expect("Failed to generate key");

        let data = b"Test data";
        let envelope = seal_for_recipient(data, &alice.public).expect("Failed to seal envelope");

        let format = detect_envelope_format(&envelope).expect("Failed to detect format");

        assert_eq!(format, EnvelopeFormat::CoreHybrid);
    }

    #[test]
    fn test_unified_envelope() {
        let alice =
            KeyPair::generate(AsymmetricAlgorithm::Rsa2048).expect("Failed to generate key");

        let data = b"Test data for unified envelope";

        let envelope =
            UnifiedEnvelope::seal_auto(data, &alice.public).expect("Failed to seal envelope");

        let decrypted =
            UnifiedEnvelope::open_auto(&envelope, &alice.private).expect("Failed to open envelope");

        assert_eq!(data, decrypted.as_slice());
    }

    #[test]
    fn test_envelope_inspection() {
        let alice =
            KeyPair::generate(AsymmetricAlgorithm::Rsa2048).expect("Failed to generate key");

        let data = b"Test data for inspection";
        let envelope = seal_for_recipient(data, &alice.public).expect("Failed to seal envelope");

        let info = UnifiedEnvelope::inspect(&envelope).expect("Failed to inspect envelope");

        assert_eq!(info.format, EnvelopeFormat::CoreHybrid);
        assert_eq!(info.recipient_id, Some(alice.public.id()));
        assert_eq!(info.algorithm, "RSA + AES-256-GCM");
        assert!(info.size > data.len());
    }
}
