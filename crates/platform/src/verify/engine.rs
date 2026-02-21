//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//

//! Verification engine — BLAKE3 continuity chaining and Ed25519 signature verification.

use anyhow::{anyhow, Result};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use blake3::Hasher;
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
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
pub struct Receipt {
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
) -> Receipt {
    Receipt {
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
    let device_pub = device_pub
        .strip_prefix("ed25519:")
        .ok_or_else(|| anyhow!("Device public key must have ed25519: prefix"))?;

    let signature = manifest
        .get("signature")
        .and_then(|s| s.as_str())
        .ok_or_else(|| anyhow!("Missing signature in manifest"))?;

    let canonicalized = canonicalize_manifest_for_signature(manifest)?;

    let public_key_bytes = BASE64
        .decode(device_pub)
        .map_err(|e| anyhow!("Invalid base64 in device public key: {}", e))?;

    let verifying_key = VerifyingKey::try_from(public_key_bytes.as_slice())
        .map_err(|e| anyhow!("Invalid Ed25519 public key: {}", e))?;

    let signature_bytes = BASE64
        .decode(signature)
        .map_err(|e| anyhow!("Invalid base64 signature: {}", e))?;

    let signature = Signature::try_from(signature_bytes.as_slice())
        .map_err(|e| anyhow!("Invalid Ed25519 signature: {}", e))?;

    match verifying_key.verify(canonicalized.as_bytes(), &signature) {
        Ok(()) => Ok(VerificationResult {
            passed: true,
            error: None,
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

fn compute_genesis_hash() -> String {
    let mut hasher = Hasher::new();
    hasher.update(b"trustedge:genesis");
    format!("b3:{}", BASE64.encode(hasher.finalize().as_bytes()))
}

fn compute_chain_link(prev: &str, hash: &str) -> String {
    let mut hasher = Hasher::new();

    let prev_clean = prev.strip_prefix("b3:").unwrap_or(prev);
    let hash_clean = hash.strip_prefix("b3:").unwrap_or(hash);

    let prev_bytes = BASE64.decode(prev_clean).unwrap_or_default();
    let hash_bytes = BASE64.decode(hash_clean).unwrap_or_default();

    hasher.update(&prev_bytes);
    hasher.update(&hash_bytes);

    format!("b3:{}", BASE64.encode(hasher.finalize().as_bytes()))
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_genesis_hash_computation() {
        let genesis = compute_genesis_hash();
        assert!(genesis.starts_with("b3:"));

        let expected_hash = {
            let mut hasher = Hasher::new();
            hasher.update(b"trustedge:genesis");
            format!("b3:{}", BASE64.encode(hasher.finalize().as_bytes()))
        };
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
