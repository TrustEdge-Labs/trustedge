//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//

//! Verification engine — BLAKE3 continuity chaining and Ed25519 signature verification.
//!
//! All cryptographic operations delegate to trustedge_core's chain and crypto modules.
//! No direct blake3 or ed25519_dalek calls remain in this module.

use anyhow::{anyhow, Result};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct SegmentDigest {
    pub index: u32,
    pub hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct VerifyReport {
    pub signature_verification: VerificationResult,
    pub continuity_verification: VerificationResult,
    pub metadata: VerificationMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct VerificationResult {
    pub passed: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct VerificationMetadata {
    pub total_segments: u32,
    pub verified_segments: u32,
    pub chain_tip: String,
    pub genesis_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct ReceiptClaims {
    pub verification_id: String,
    pub device_id: String,
    pub manifest_digest: String,
    pub chain_tip: String,
    pub timestamp: String,
    pub kid: String,
    pub result: VerifyReport,
}

pub fn verify_to_report(
    manifest: &serde_json::Value,
    segments: &[SegmentDigest],
    device_pub: &str,
) -> Result<VerifyReport> {
    let signature_result = verify_signature(manifest, device_pub)?;
    let continuity_result = verify_continuity(segments)?;

    let genesis_hash = compute_genesis_hash();
    let chain_tip = if segments.is_empty() {
        genesis_hash.clone()
    } else {
        compute_chain_tip(segments)?
    };

    Ok(VerifyReport {
        signature_verification: signature_result,
        continuity_verification: continuity_result,
        metadata: VerificationMetadata {
            total_segments: segments.len() as u32,
            verified_segments: segments.len() as u32,
            chain_tip,
            genesis_hash,
        },
    })
}

pub fn receipt_from_report(
    report: &VerifyReport,
    manifest_digest: &str,
    device_id: &str,
    kid: &str,
    now_rfc3339: &str,
    chain_tip: &str,
) -> ReceiptClaims {
    ReceiptClaims {
        verification_id: format!("v_{}", uuid::Uuid::new_v4().simple()),
        device_id: device_id.to_string(),
        manifest_digest: manifest_digest.to_string(),
        chain_tip: chain_tip.to_string(),
        timestamp: now_rfc3339.to_string(),
        kid: kid.to_string(),
        result: report.clone(),
    }
}

fn verify_signature(manifest: &serde_json::Value, device_pub: &str) -> Result<VerificationResult> {
    // device_pub must have "ed25519:" prefix — core's verify_manifest expects it present
    if !device_pub.starts_with("ed25519:") {
        return Err(anyhow!("Device public key must have ed25519: prefix"));
    }

    let signature_b64 = manifest
        .get("signature")
        .and_then(|s| s.as_str())
        .ok_or_else(|| anyhow!("Missing signature in manifest"))?;

    let canonicalized = canonicalize_manifest_for_signature(manifest)?;

    // Core's verify_manifest expects "ed25519:BASE64" format for the signature.
    // The manifest stores the raw base64 without the prefix, so we prepend it.
    let signature_str = format!("ed25519:{}", signature_b64);

    match trustedge_core::crypto::verify_manifest(
        device_pub,
        canonicalized.as_bytes(),
        &signature_str,
    ) {
        Ok(true) => Ok(VerificationResult {
            passed: true,
            error: None,
        }),
        Ok(false) => Ok(VerificationResult {
            passed: false,
            error: Some("Signature verification failed".to_string()),
        }),
        Err(e) => Ok(VerificationResult {
            passed: false,
            error: Some(format!("Signature verification failed: {}", e)),
        }),
    }
}

fn verify_continuity(segments: &[SegmentDigest]) -> Result<VerificationResult> {
    if segments.is_empty() {
        return Ok(VerificationResult {
            passed: true,
            error: None,
        });
    }

    let mut sorted_segments = segments.to_vec();
    sorted_segments.sort_by_key(|s| s.index);

    for (i, segment) in sorted_segments.iter().enumerate() {
        if segment.index != i as u32 {
            return Ok(VerificationResult {
                passed: false,
                error: Some(format!("Missing segment at index {}", i)),
            });
        }
    }

    let genesis = compute_genesis_hash();
    let mut chain_value = genesis;

    for segment in &sorted_segments {
        let hash = segment.hash.strip_prefix("b3:").unwrap_or(&segment.hash);
        let next_chain = compute_chain_link(&chain_value, hash);
        chain_value = next_chain;
    }

    Ok(VerificationResult {
        passed: true,
        error: None,
    })
}

fn canonicalize_manifest_for_signature(manifest: &serde_json::Value) -> Result<String> {
    let mut manifest_copy = manifest.clone();

    if let Some(obj) = manifest_copy.as_object_mut() {
        obj.remove("signature");
    }

    let canonical = serde_json::to_string(&manifest_copy)?;
    Ok(canonical)
}

/// Compute the genesis chain hash using trustedge_core's chain module.
///
/// Uses BASE64 (standard alphabet with padding) to match the existing wire format.
/// Core's `genesis()` returns the raw `[u8; 32]` BLAKE3 hash bytes, which we
/// format with the "b3:" prefix and standard base64 encoding.
fn compute_genesis_hash() -> String {
    format_b3(&trustedge_core::chain::genesis())
}

/// Compute a chain link using trustedge_core's chain module.
fn compute_chain_link(prev: &str, hash: &str) -> String {
    let prev_clean = prev.strip_prefix("b3:").unwrap_or(prev);
    let hash_clean = hash.strip_prefix("b3:").unwrap_or(hash);

    let prev_bytes = BASE64.decode(prev_clean).unwrap_or_default();
    let hash_bytes = BASE64.decode(hash_clean).unwrap_or_default();

    let mut prev_arr = [0u8; 32];
    let mut hash_arr = [0u8; 32];
    if prev_bytes.len() == 32 {
        prev_arr.copy_from_slice(&prev_bytes);
    }
    if hash_bytes.len() == 32 {
        hash_arr.copy_from_slice(&hash_bytes);
    }

    format_b3(&trustedge_core::chain::chain_next(&prev_arr, &hash_arr))
}

fn compute_chain_tip(segments: &[SegmentDigest]) -> Result<String> {
    let mut sorted_segments = segments.to_vec();
    sorted_segments.sort_by_key(|s| s.index);

    let genesis = compute_genesis_hash();
    let mut chain_value = genesis;

    for segment in &sorted_segments {
        let hash = segment.hash.strip_prefix("b3:").unwrap_or(&segment.hash);
        chain_value = compute_chain_link(&chain_value, hash);
    }

    Ok(chain_value)
}

/// Format a 32-byte hash as "b3:BASE64" using the standard base64 alphabet.
///
/// Uses the `base64` crate's STANDARD encoder (RFC 4648 with padding) to ensure
/// consistent output with callers that decode using the same encoder.
fn format_b3(bytes: &[u8; 32]) -> String {
    format!("b3:{}", BASE64.encode(bytes))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_genesis_hash_computation() {
        let genesis = compute_genesis_hash();
        assert!(genesis.starts_with("b3:"));

        // Verify using trustedge_core's chain primitives directly
        let expected_hash = format_b3(&trustedge_core::chain::genesis());
        assert_eq!(genesis, expected_hash);
    }

    #[test]
    fn test_chain_link_computation() {
        let prev = "b3:abc123";
        let hash = "b3:def456";
        let result = compute_chain_link(prev, hash);
        assert!(result.starts_with("b3:"));
    }

    #[test]
    fn test_continuity_verification_empty() {
        let segments = vec![];
        let result = verify_continuity(&segments).unwrap();
        assert!(result.passed);
    }

    #[test]
    fn test_continuity_verification_sequential() {
        let segments = vec![
            SegmentDigest {
                index: 0,
                hash: "b3:hash0".to_string(),
            },
            SegmentDigest {
                index: 1,
                hash: "b3:hash1".to_string(),
            },
            SegmentDigest {
                index: 2,
                hash: "b3:hash2".to_string(),
            },
        ];
        let result = verify_continuity(&segments).unwrap();
        assert!(result.passed);
    }

    #[test]
    fn test_continuity_verification_missing_segment() {
        let segments = vec![
            SegmentDigest {
                index: 0,
                hash: "b3:hash0".to_string(),
            },
            SegmentDigest {
                index: 2,
                hash: "b3:hash2".to_string(),
            },
        ];
        let result = verify_continuity(&segments).unwrap();
        assert!(!result.passed);
        assert!(result.error.is_some());
    }

    #[test]
    fn test_manifest_canonicalization() {
        let manifest = json!({
            "version": "1.0",
            "segments": 3,
            "signature": "should_be_removed"
        });

        let canonical = canonicalize_manifest_for_signature(&manifest).unwrap();
        assert!(!canonical.contains("signature"));
        assert!(canonical.contains("version"));
        assert!(canonical.contains("segments"));
    }

    #[test]
    fn test_receipt_from_report() {
        let report = VerifyReport {
            signature_verification: VerificationResult {
                passed: true,
                error: None,
            },
            continuity_verification: VerificationResult {
                passed: true,
                error: None,
            },
            metadata: VerificationMetadata {
                total_segments: 0,
                verified_segments: 0,
                chain_tip: "b3:test".to_string(),
                genesis_hash: "b3:genesis".to_string(),
            },
        };

        let receipt = receipt_from_report(
            &report,
            "digest123",
            "device_abc",
            "key_001",
            "2026-02-21T00:00:00Z",
            "b3:test",
        );

        assert!(receipt.verification_id.starts_with("v_"));
        assert_eq!(receipt.device_id, "device_abc");
        assert_eq!(receipt.manifest_digest, "digest123");
        assert_eq!(receipt.kid, "key_001");
    }
}
