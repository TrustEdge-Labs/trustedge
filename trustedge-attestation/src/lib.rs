// Copyright (c) 2025 TRUSTEDGE LABS LLC
// MPL-2.0: https://mozilla.org/MPL/2.0/
// Project: trustedge — Privacy and trust at the edge.

//! TrustEdge Software Attestation Library
//!
//! Core attestation functionality for creating software "birth certificates".

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Simple software attestation - the "birth certificate" payload
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Attestation {
    /// The SHA-256 hash of the artifact being attested to.
    pub artifact_hash: String,
    /// The name of the artifact file.
    pub artifact_name: String,
    /// The Git commit hash from which the artifact was built.
    pub source_commit_hash: String,
    /// An identifier for the entity that created the attestation.
    pub builder_id: String,
    /// The ISO 8601 timestamp of when the attestation was created.
    pub timestamp: String,
}

/// Create a software attestation from an artifact file
pub fn create_attestation_data(artifact_path: &Path, builder_id: &str) -> Result<Attestation> {
    use sha2::{Digest, Sha256};

    // 1. Hash the artifact
    let artifact_data = std::fs::read(artifact_path)
        .with_context(|| format!("Failed to read artifact: {}", artifact_path.display()))?;

    let artifact_hash = format!("{:x}", Sha256::digest(&artifact_data));

    // 2. Get commit hash (simplified - just use a placeholder if not in git repo)
    let source_commit_hash = get_git_commit_hash().unwrap_or_else(|_| "unknown".to_string());

    // 3. Create attestation
    let attestation = Attestation {
        artifact_hash,
        artifact_name: artifact_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string(),
        source_commit_hash,
        builder_id: builder_id.to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
    };

    Ok(attestation)
}

/// Get the current Git commit hash
fn get_git_commit_hash() -> Result<String> {
    use git2::Repository;

    let repo = Repository::discover(".").context("Failed to find Git repository")?;

    let head = repo.head().context("Failed to get HEAD reference")?;

    let commit = head
        .peel_to_commit()
        .context("Failed to get commit from HEAD")?;

    Ok(commit.id().to_string())
}

/// Configuration for creating signed attestations
#[derive(Debug)]
pub struct AttestationConfig {
    /// Path to the software artifact to attest
    pub artifact_path: PathBuf,
    /// Builder identifier (e.g., email, CI job ID)
    pub builder_id: String,
    /// Output format for the attestation
    pub output_format: OutputFormat,
    /// Source of cryptographic keys
    pub key_source: KeySource,
}

/// Output format options for attestations
#[derive(Debug, Clone)]
pub enum OutputFormat {
    /// Plain JSON attestation (no cryptographic envelope)
    JsonOnly,
    /// Cryptographically sealed envelope with attestation
    SealedEnvelope,
}

/// Source of cryptographic keys for signing
#[derive(Debug)]
pub enum KeySource {
    /// Generate ephemeral keys (demo mode)
    Generate,
    /// Use provided signing key (production mode)
    Provided { signing_key: ed25519_dalek::SigningKey },
}

/// Result of creating a signed attestation
#[derive(Debug)]
pub struct AttestationResult {
    /// The attestation data
    pub attestation: Attestation,
    /// Serialized output ready to write to disk
    pub serialized_output: Vec<u8>,
    /// Verification information (if envelope format)
    pub verification_info: Option<VerificationInfo>,
}

/// Information needed to verify an attestation
#[derive(Debug, Clone)]
pub struct VerificationInfo {
    /// Public key for verification (hex encoded)
    pub verification_key: String,
    /// Private key for unsealing (demo only - hex encoded)
    pub private_key: Option<String>,
}

/// Create a cryptographically signed software attestation
///
/// This is the main entry point for creating attestations. It handles:
/// - Analyzing the artifact and extracting metadata
/// - Creating the attestation data structure
/// - Optionally sealing in a cryptographic envelope
/// - Serializing for output
///
/// # Arguments
/// * `config` - Configuration specifying artifact, builder, output format, and keys
///
/// # Returns
/// * `AttestationResult` containing the attestation, serialized output, and verification info
///
/// # Example
/// ```rust
/// use trustedge_attestation::{AttestationConfig, OutputFormat, KeySource, create_signed_attestation};
/// use std::path::PathBuf;
/// use std::io::Write;
/// use tempfile::NamedTempFile;
///
/// // Create a test artifact
/// let mut test_file = NamedTempFile::new()?;
/// test_file.write_all(b"test binary content")?;
/// let artifact_path = test_file.path().to_path_buf();
///
/// let config = AttestationConfig {
///     artifact_path,
///     builder_id: "ci-job-123".to_string(),
///     output_format: OutputFormat::SealedEnvelope,
///     key_source: KeySource::Generate,
/// };
///
/// let result = create_signed_attestation(config)?;
/// // result.serialized_output contains the attestation ready to write to disk
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn create_signed_attestation(config: AttestationConfig) -> Result<AttestationResult> {
    // Step 1: Create the attestation data
    let attestation = create_attestation_data(&config.artifact_path, &config.builder_id)
        .context("Failed to create attestation data")?;

    // Step 2: Handle output format
    match config.output_format {
        OutputFormat::JsonOnly => {
            let serialized_output = serde_json::to_vec_pretty(&attestation)
                .context("Failed to serialize attestation to JSON")?;

            Ok(AttestationResult {
                attestation,
                serialized_output,
                verification_info: None,
            })
        }
        OutputFormat::SealedEnvelope => {
            create_sealed_attestation(attestation, config.key_source)
        }
    }
}

/// Create a cryptographically sealed attestation envelope
#[cfg(feature = "envelope")]
fn create_sealed_attestation(
    attestation: Attestation,
    key_source: KeySource,
) -> Result<AttestationResult> {
    use trustedge_core::Envelope;

    // Get or generate signing key
    let signing_key = match key_source {
        KeySource::Generate => {
            let mut csprng = rand::rngs::OsRng;
            ed25519_dalek::SigningKey::generate(&mut csprng)
        }
        KeySource::Provided { signing_key } => signing_key,
    };

    // Serialize attestation to JSON for the envelope payload
    let payload = serde_json::to_vec(&attestation)
        .context("Failed to serialize attestation")?;

    // For attestations, we use the same key as both sender and beneficiary
    // This makes it a publicly verifiable signature rather than encrypted message
    let beneficiary_key = signing_key.verifying_key();

    // Create the cryptographic envelope
    let envelope = Envelope::seal(&payload, &signing_key, &beneficiary_key)
        .context("Failed to create envelope")?;

    // Create attestation file format
    #[derive(serde::Serialize)]
    struct AttestationFile {
        envelope: Envelope,
        verification_key: [u8; 32], // Public key for verification
        private_key: [u8; 32],      // Private key for unsealing (demo only!)
    }

    let attestation_file = AttestationFile {
        envelope,
        verification_key: signing_key.verifying_key().to_bytes(),
        private_key: signing_key.to_bytes(),
    };

    // Serialize using bincode
    let serialized_output = bincode::serialize(&attestation_file)
        .context("Failed to serialize attestation file")?;

    // Create verification info
    let verification_info = Some(VerificationInfo {
        verification_key: hex::encode(signing_key.verifying_key().to_bytes()),
        private_key: Some(hex::encode(signing_key.to_bytes())), // Demo only
    });

    Ok(AttestationResult {
        attestation,
        serialized_output,
        verification_info,
    })
}

/// Fallback when envelope feature is not enabled
#[cfg(not(feature = "envelope"))]
fn create_sealed_attestation(
    attestation: Attestation,
    _key_source: KeySource,
) -> Result<AttestationResult> {
    // Fallback to JSON when envelope feature is not available
    let serialized_output = serde_json::to_vec_pretty(&attestation)
        .context("Failed to serialize attestation to JSON")?;

    Ok(AttestationResult {
        attestation,
        serialized_output,
        verification_info: None,
    })
}

/// Configuration for verifying attestations
#[derive(Debug)]
pub struct VerificationConfig {
    /// Path to the software artifact to verify
    pub artifact_path: PathBuf,
    /// Path to the attestation file
    pub attestation_path: PathBuf,
    /// Force treating attestation as JSON (not envelope)
    pub force_json: bool,
}

/// Result of verifying an attestation
#[derive(Debug)]
pub struct VerificationResult {
    /// The attestation that was verified
    pub attestation: Attestation,
    /// Whether verification succeeded
    pub is_valid: bool,
    /// Details about what was verified
    pub verification_details: VerificationDetails,
}

/// Details about the verification process
#[derive(Debug)]
pub struct VerificationDetails {
    /// The computed hash of the artifact
    pub computed_hash: String,
    /// The expected hash from the attestation
    pub expected_hash: String,
    /// Size of the artifact in bytes
    pub artifact_size: u64,
    /// Whether the envelope signature was verified (if applicable)
    pub envelope_verified: Option<bool>,
}

/// Verify a software attestation against an artifact
///
/// This function handles both JSON and envelope attestation formats,
/// and verifies the artifact hash against the attestation.
///
/// # Arguments
/// * `config` - Configuration specifying artifact and attestation paths
///
/// # Returns
/// * `VerificationResult` with verification status and details
pub fn verify_attestation(config: VerificationConfig) -> Result<VerificationResult> {
    // Read and parse the attestation
    let attestation = if config.force_json {
        read_json_attestation(&config.attestation_path)?
    } else {
        // Try envelope first, fallback to JSON
        #[cfg(feature = "envelope")]
        {
            match read_envelope_attestation(&config.attestation_path) {
                Ok(att) => att,
                Err(_) => read_json_attestation(&config.attestation_path)?,
            }
        }

        #[cfg(not(feature = "envelope"))]
        {
            read_json_attestation(&config.attestation_path)?
        }
    };

    // Compute artifact hash
    let artifact_data = std::fs::read(&config.artifact_path)
        .with_context(|| format!("Failed to read artifact: {}", config.artifact_path.display()))?;

    use sha2::{Digest, Sha256};
    let computed_hash = format!("{:x}", Sha256::digest(&artifact_data));

    // Check if hashes match
    let is_valid = computed_hash == attestation.artifact_hash;

    let verification_details = VerificationDetails {
        computed_hash: computed_hash.clone(),
        expected_hash: attestation.artifact_hash.clone(),
        artifact_size: artifact_data.len() as u64,
        envelope_verified: None, // Will be set by envelope reading if applicable
    };

    Ok(VerificationResult {
        attestation,
        is_valid,
        verification_details,
    })
}

/// Read attestation from envelope format
#[cfg(feature = "envelope")]
fn read_envelope_attestation(path: &PathBuf) -> Result<Attestation> {
    use std::fs::File;
    use std::io::BufReader;
    use trustedge_core::Envelope;

    #[derive(serde::Deserialize)]
    struct AttestationFile {
        envelope: Envelope,
        #[allow(dead_code)]
        verification_key: [u8; 32],
        private_key: [u8; 32],
    }

    let file = File::open(path)
        .with_context(|| format!("Failed to open attestation file: {}", path.display()))?;
    let mut reader = BufReader::new(file);

    let attestation_file: AttestationFile =
        bincode::deserialize_from(&mut reader)
            .context("Failed to read attestation file")?;

    // Verify the envelope signature
    if !attestation_file.envelope.verify() {
        return Err(anyhow::anyhow!("Envelope signature verification failed"));
    }

    // Reconstruct the private key for unsealing
    let private_key = ed25519_dalek::SigningKey::from_bytes(&attestation_file.private_key);

    let payload = attestation_file
        .envelope
        .unseal(&private_key)
        .context("Failed to unseal envelope")?;

    let attestation: Attestation = serde_json::from_slice(&payload)
        .context("Failed to parse attestation from envelope payload")?;

    Ok(attestation)
}

/// Read attestation from JSON format
fn read_json_attestation(path: &PathBuf) -> Result<Attestation> {
    let json_data = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read attestation file: {}", path.display()))?;

    serde_json::from_str::<Attestation>(&json_data)
        .context("Failed to parse JSON attestation")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_centralized_json_attestation() -> Result<()> {
        // Create a test artifact
        let mut test_file = NamedTempFile::new()?;
        test_file.write_all(b"test artifact content")?;
        let test_path = test_file.path().to_path_buf();

        // Create attestation config
        let config = AttestationConfig {
            artifact_path: test_path.clone(),
            builder_id: "test-builder".to_string(),
            output_format: OutputFormat::JsonOnly,
            key_source: KeySource::Generate,
        };

        // Create attestation
        let result = create_signed_attestation(config)?;

        // Verify the result
        assert_eq!(result.attestation.builder_id, "test-builder");
        assert_eq!(result.attestation.artifact_name, test_file.path().file_name().unwrap().to_str().unwrap());
        assert!(result.verification_info.is_none()); // JSON only should not have verification info

        // Verify the serialized output is valid JSON
        let parsed: Attestation = serde_json::from_slice(&result.serialized_output)?;
        assert_eq!(parsed, result.attestation);

        Ok(())
    }

    #[test]
    #[cfg(feature = "envelope")]
    fn test_centralized_envelope_attestation() -> Result<()> {
        // Create a test artifact
        let mut test_file = NamedTempFile::new()?;
        test_file.write_all(b"test artifact for envelope")?;
        let test_path = test_file.path().to_path_buf();

        // Create attestation config
        let config = AttestationConfig {
            artifact_path: test_path.clone(),
            builder_id: "envelope-builder".to_string(),
            output_format: OutputFormat::SealedEnvelope,
            key_source: KeySource::Generate,
        };

        // Create attestation
        let result = create_signed_attestation(config)?;

        // Verify the result
        assert_eq!(result.attestation.builder_id, "envelope-builder");
        assert!(result.verification_info.is_some()); // Envelope should have verification info

        if let Some(verification_info) = &result.verification_info {
            assert!(!verification_info.verification_key.is_empty());
            assert!(verification_info.private_key.is_some());
        }

        // Write the attestation to a temporary file
        let mut attestation_file = NamedTempFile::new()?;
        attestation_file.write_all(&result.serialized_output)?;
        let attestation_path = attestation_file.path().to_path_buf();

        // Test verification
        let verification_config = VerificationConfig {
            artifact_path: test_path,
            attestation_path,
            force_json: false,
        };

        let verification_result = verify_attestation(verification_config)?;
        assert!(verification_result.is_valid);
        assert_eq!(verification_result.attestation.builder_id, "envelope-builder");

        Ok(())
    }

    #[test]
    fn test_verification_failure() -> Result<()> {
        // Create a test artifact
        let mut test_file = NamedTempFile::new()?;
        test_file.write_all(b"original content")?;
        let test_path = test_file.path().to_path_buf();

        // Create attestation
        let config = AttestationConfig {
            artifact_path: test_path.clone(),
            builder_id: "test-builder".to_string(),
            output_format: OutputFormat::JsonOnly,
            key_source: KeySource::Generate,
        };

        let result = create_signed_attestation(config)?;

        // Write attestation to file
        let mut attestation_file = NamedTempFile::new()?;
        attestation_file.write_all(&result.serialized_output)?;
        let attestation_path = attestation_file.path().to_path_buf();

        // Modify the artifact
        let mut modified_file = NamedTempFile::new()?;
        modified_file.write_all(b"modified content")?;
        let modified_path = modified_file.path().to_path_buf();

        // Test verification against modified artifact (should fail)
        let verification_config = VerificationConfig {
            artifact_path: modified_path,
            attestation_path,
            force_json: true,
        };

        let verification_result = verify_attestation(verification_config)?;
        assert!(!verification_result.is_valid);
        assert_ne!(verification_result.verification_details.computed_hash,
                   verification_result.verification_details.expected_hash);

        Ok(())
    }

    #[test]
    fn test_provided_key_source() -> Result<()> {
        // Test production mode with provided keys
        let mut test_file = NamedTempFile::new()?;
        test_file.write_all(b"test with provided key")?;
        let test_path = test_file.path().to_path_buf();

        // Generate a specific key for testing
        let mut csprng = rand::rngs::OsRng;
        let signing_key = ed25519_dalek::SigningKey::generate(&mut csprng);

        let config = AttestationConfig {
            artifact_path: test_path,
            builder_id: "provided-key-builder".to_string(),
            output_format: OutputFormat::SealedEnvelope,
            key_source: KeySource::Provided { signing_key: signing_key.clone() },
        };

        let result = create_signed_attestation(config)?;

        // Verify that the verification key matches our provided key
        if let Some(verification_info) = &result.verification_info {
            let expected_public_key = hex::encode(signing_key.verifying_key().to_bytes());
            assert_eq!(verification_info.verification_key, expected_public_key);
        } else {
            panic!("Expected verification info for envelope output");
        }

        Ok(())
    }

    #[test]
    fn test_json_only_with_provided_key() -> Result<()> {
        // Test that KeySource::Provided works even with JSON-only output
        let mut test_file = NamedTempFile::new()?;
        test_file.write_all(b"json with provided key")?;
        let test_path = test_file.path().to_path_buf();

        let mut csprng = rand::rngs::OsRng;
        let signing_key = ed25519_dalek::SigningKey::generate(&mut csprng);

        let config = AttestationConfig {
            artifact_path: test_path,
            builder_id: "json-provided-key".to_string(),
            output_format: OutputFormat::JsonOnly,
            key_source: KeySource::Provided { signing_key },
        };

        let result = create_signed_attestation(config)?;

        // JSON-only should not have verification info even with provided key
        assert!(result.verification_info.is_none());
        assert_eq!(result.attestation.builder_id, "json-provided-key");

        Ok(())
    }

    #[test]
    fn test_force_json_verification() -> Result<()> {
        // Test the force_json flag in verification
        let mut test_file = NamedTempFile::new()?;
        test_file.write_all(b"force json test")?;
        let test_path = test_file.path().to_path_buf();

        // Create JSON attestation
        let config = AttestationConfig {
            artifact_path: test_path.clone(),
            builder_id: "force-json-test".to_string(),
            output_format: OutputFormat::JsonOnly,
            key_source: KeySource::Generate,
        };

        let result = create_signed_attestation(config)?;

        // Write attestation to file
        let mut attestation_file = NamedTempFile::new()?;
        attestation_file.write_all(&result.serialized_output)?;
        let attestation_path = attestation_file.path().to_path_buf();

        // Verify with force_json = true
        let verification_config = VerificationConfig {
            artifact_path: test_path,
            attestation_path,
            force_json: true,
        };

        let verification_result = verify_attestation(verification_config)?;
        assert!(verification_result.is_valid);
        assert_eq!(verification_result.attestation.builder_id, "force-json-test");

        Ok(())
    }

    #[test]
    fn test_attestation_data_fields() -> Result<()> {
        // Test that all attestation fields are populated correctly
        let mut test_file = NamedTempFile::new()?;
        test_file.write_all(b"field validation test")?;
        let test_path = test_file.path().to_path_buf();

        let config = AttestationConfig {
            artifact_path: test_path.clone(),
            builder_id: "field-validator".to_string(),
            output_format: OutputFormat::JsonOnly,
            key_source: KeySource::Generate,
        };

        let result = create_signed_attestation(config)?;

        // Verify all required fields are present and valid
        assert_eq!(result.attestation.builder_id, "field-validator");
        assert!(!result.attestation.artifact_hash.is_empty());
        assert!(!result.attestation.artifact_name.is_empty());
        assert!(!result.attestation.timestamp.is_empty());

        // Verify hash is correct SHA-256 format (64 hex chars)
        assert_eq!(result.attestation.artifact_hash.len(), 64);
        assert!(result.attestation.artifact_hash.chars().all(|c| c.is_ascii_hexdigit()));

        // Verify timestamp is valid ISO 8601
        assert!(chrono::DateTime::parse_from_rfc3339(&result.attestation.timestamp).is_ok());

        // Verify artifact name matches file
        let expected_name = test_file.path().file_name().unwrap().to_str().unwrap();
        assert_eq!(result.attestation.artifact_name, expected_name);

        Ok(())
    }

    #[test]
    fn test_file_not_found_error() -> Result<()> {
        // Test error handling for non-existent files
        let config = AttestationConfig {
            artifact_path: PathBuf::from("/nonexistent/file.bin"),
            builder_id: "error-test".to_string(),
            output_format: OutputFormat::JsonOnly,
            key_source: KeySource::Generate,
        };

        let result = create_signed_attestation(config);
        assert!(result.is_err());

        let error_message = format!("{}", result.unwrap_err());
        // The error should mention either "Failed to read artifact" or "Failed to create attestation data"
        assert!(error_message.contains("Failed to read artifact") ||
                error_message.contains("Failed to create attestation data"));

        Ok(())
    }

    #[test]
    fn test_verification_with_missing_attestation() -> Result<()> {
        // Test verification error handling for missing attestation file
        let mut test_file = NamedTempFile::new()?;
        test_file.write_all(b"test content")?;
        let test_path = test_file.path().to_path_buf();

        let verification_config = VerificationConfig {
            artifact_path: test_path,
            attestation_path: PathBuf::from("/nonexistent/attestation.json"),
            force_json: true,
        };

        let result = verify_attestation(verification_config);
        assert!(result.is_err());

        let error_message = format!("{}", result.unwrap_err());
        assert!(error_message.contains("Failed to read attestation file"));

        Ok(())
    }

    #[test]
    fn test_verification_details() -> Result<()> {
        // Test that verification details are populated correctly
        let mut test_file = NamedTempFile::new()?;
        let test_content = b"verification details test";
        test_file.write_all(test_content)?;
        let test_path = test_file.path().to_path_buf();

        // Create attestation
        let config = AttestationConfig {
            artifact_path: test_path.clone(),
            builder_id: "details-test".to_string(),
            output_format: OutputFormat::JsonOnly,
            key_source: KeySource::Generate,
        };

        let result = create_signed_attestation(config)?;

        // Write attestation to file
        let mut attestation_file = NamedTempFile::new()?;
        attestation_file.write_all(&result.serialized_output)?;
        let attestation_path = attestation_file.path().to_path_buf();

        // Verify
        let verification_config = VerificationConfig {
            artifact_path: test_path,
            attestation_path,
            force_json: true,
        };

        let verification_result = verify_attestation(verification_config)?;

        // Check verification details
        assert!(verification_result.is_valid);
        assert_eq!(verification_result.verification_details.artifact_size, test_content.len() as u64);
        assert_eq!(verification_result.verification_details.computed_hash,
                   verification_result.verification_details.expected_hash);
        assert_eq!(verification_result.verification_details.expected_hash,
                   result.attestation.artifact_hash);
        assert!(verification_result.verification_details.envelope_verified.is_none()); // JSON mode

        Ok(())
    }

    #[test]
    #[cfg(not(feature = "envelope"))]
    fn test_sealed_envelope_fallback_without_feature() -> Result<()> {
        // Test that SealedEnvelope falls back to JSON when envelope feature is disabled
        let mut test_file = NamedTempFile::new()?;
        test_file.write_all(b"fallback test")?;
        let test_path = test_file.path().to_path_buf();

        let config = AttestationConfig {
            artifact_path: test_path,
            builder_id: "fallback-test".to_string(),
            output_format: OutputFormat::SealedEnvelope, // Request envelope but feature disabled
            key_source: KeySource::Generate,
        };

        let result = create_signed_attestation(config)?;

        // Should fall back to JSON
        assert!(result.verification_info.is_none());

        // Should be valid JSON
        let parsed: Attestation = serde_json::from_slice(&result.serialized_output)?;
        assert_eq!(parsed.builder_id, "fallback-test");

        Ok(())
    }
}
