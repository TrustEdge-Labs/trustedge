//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//

//! Request validation for the verification service.

use regex::Regex;
use serde::{Deserialize, Serialize};

use super::engine::SegmentDigest;
use super::types::VerifyRequest;

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct ValidationError {
    pub error: String,
    pub detail: String,
}

impl ValidationError {
    pub fn new(error: &str, detail: &str) -> Self {
        Self {
            error: error.to_string(),
            detail: detail.to_string(),
        }
    }
}

pub fn validate_verify_request(request: &VerifyRequest) -> Result<(), ValidationError> {
    // Validate segments are non-empty
    if request.segments.is_empty() {
        return Err(ValidationError::new(
            "invalid_segments",
            "segments array cannot be empty",
        ));
    }

    // Validate hash format for each segment
    validate_segment_hashes(&request.segments)?;

    // Note: Index validation is handled by continuity verification in the core library

    Ok(())
}

pub fn validate_segment_hashes(segments: &[SegmentDigest]) -> Result<(), ValidationError> {
    let hash_regex = Regex::new(r"^b3:[0-9a-f]{64}$").unwrap();

    for (i, segment) in segments.iter().enumerate() {
        if !hash_regex.is_match(&segment.hash) {
            return Err(ValidationError::new(
                "invalid_segments",
                &format!(
                    "segments[{}].hash must match ^b3:[0-9a-f]{{64}}$, got '{}'",
                    i, segment.hash
                ),
            ));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_segment(index: u32, hash: &str) -> SegmentDigest {
        SegmentDigest {
            index,
            hash: hash.to_string(),
        }
    }

    #[test]
    fn test_validate_empty_segments() {
        let request = VerifyRequest {
            device_pub: "ed25519:test".to_string(),
            manifest: serde_json::json!({}),
            segments: vec![],
            options: None,
        };

        let result = validate_verify_request(&request);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.error, "invalid_segments");
        assert!(err.detail.contains("cannot be empty"));
    }

    #[test]
    fn test_validate_invalid_hash_format() {
        let segments = vec![create_test_segment(0, "invalid_hash")];
        let request = VerifyRequest {
            device_pub: "ed25519:test".to_string(),
            manifest: serde_json::json!({}),
            segments,
            options: None,
        };

        let result = validate_verify_request(&request);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.error, "invalid_segments");
        assert!(err.detail.contains("must match ^b3:[0-9a-f]{64}$"));
    }

    #[test]
    fn test_validate_wrong_hash_length() {
        let segments = vec![create_test_segment(0, "b3:123")];
        let request = VerifyRequest {
            device_pub: "ed25519:test".to_string(),
            manifest: serde_json::json!({}),
            segments,
            options: None,
        };

        let result = validate_verify_request(&request);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.error, "invalid_segments");
        assert!(err.detail.contains("must match ^b3:[0-9a-f]{64}$"));
    }

    // Note: Index validation tests removed because index validation
    // is now handled by continuity verification in the core library

    #[test]
    fn test_validate_valid_request() {
        let segments = vec![
            create_test_segment(
                0,
                "b3:1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
            ),
            create_test_segment(
                1,
                "b3:fedcba0987654321fedcba0987654321fedcba0987654321fedcba0987654321",
            ),
        ];
        let request = VerifyRequest {
            device_pub: "ed25519:test".to_string(),
            manifest: serde_json::json!({}),
            segments,
            options: None,
        };

        let result = validate_verify_request(&request);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_hash_with_uppercase() {
        let segments = vec![create_test_segment(
            0,
            "b3:1234567890ABCDEF1234567890abcdef1234567890abcdef1234567890abcdef",
        )];
        let request = VerifyRequest {
            device_pub: "ed25519:test".to_string(),
            manifest: serde_json::json!({}),
            segments,
            options: None,
        };

        let result = validate_verify_request(&request);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.error, "invalid_segments");
        assert!(err.detail.contains("must match ^b3:[0-9a-f]{64}$"));
    }
}
