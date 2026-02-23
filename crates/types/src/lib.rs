//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//

//! Shared wire types for TrustEdge platform services.
//!
//! This crate provides the canonical type definitions used across TrustEdge
//! services for verification, receipts, and policy. Types are serializable
//! via serde and have JSON schema support via schemars.
//!
//! # Quick Start
//!
//! ```rust
//! use trustedge_types::prelude::*;
//!
//! let report = VerifyReport {
//!     signature: "pass".to_string(),
//!     continuity: "pass".to_string(),
//!     segments: 10,
//!     duration_s: 30.0,
//!     profile: "cam.video".to_string(),
//!     device_id: "device_01".to_string(),
//!     first_gap_index: None,
//!     out_of_order: None,
//!     error: None,
//!     verify_time_ms: 500,
//!     chain_tip: None,
//! };
//! ```

pub mod policy;
pub mod receipt;
pub mod schema;
pub mod verification;
pub mod verify_report;

// Re-export primitive type aliases — direct re-exports, no newtype wrappers.
pub use chrono::{DateTime, Utc};
pub use uuid::Uuid;

/// Prelude module for convenient glob imports.
pub mod prelude {
    pub use crate::policy::PolicyV0;
    pub use crate::receipt::VerificationReceipt;
    pub use crate::verification::{SegmentRef, VerifyOptions, VerifyRequest, VerifyResponse};
    pub use crate::verify_report::{OutOfOrder, VerifyReport};
    pub use crate::{DateTime, Utc, Uuid};
}

#[cfg(test)]
mod tests {
    use super::prelude::*;

    #[test]
    fn test_verify_report_round_trip() {
        let original = VerifyReport {
            signature: "pass".to_string(),
            continuity: "pass".to_string(),
            segments: 150,
            duration_s: 45.2,
            profile: "cam.video".to_string(),
            device_id: "device_12345".to_string(),
            first_gap_index: Some(42),
            out_of_order: Some(OutOfOrder {
                expected: 45,
                found: 43,
            }),
            error: Some("Test error".to_string()),
            verify_time_ms: 1250,
            chain_tip: Some("b3:a1b2c3d4e5f6789abcdef0123456789".to_string()),
        };

        let json = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: VerifyReport =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(original.signature, deserialized.signature);
        assert_eq!(original.continuity, deserialized.continuity);
        assert_eq!(original.segments, deserialized.segments);
        assert_eq!(original.duration_s, deserialized.duration_s);
        assert_eq!(original.profile, deserialized.profile);
        assert_eq!(original.device_id, deserialized.device_id);
        assert_eq!(original.first_gap_index, deserialized.first_gap_index);
        assert_eq!(
            original.out_of_order.as_ref().unwrap().expected,
            deserialized.out_of_order.as_ref().unwrap().expected
        );
        assert_eq!(
            original.out_of_order.as_ref().unwrap().found,
            deserialized.out_of_order.as_ref().unwrap().found
        );
        assert_eq!(original.error, deserialized.error);
        assert_eq!(original.verify_time_ms, deserialized.verify_time_ms);
        assert_eq!(original.chain_tip, deserialized.chain_tip);
    }

    #[test]
    fn test_verify_report_minimal_round_trip() {
        let original = VerifyReport {
            signature: "fail".to_string(),
            continuity: "skip".to_string(),
            segments: 0,
            duration_s: 0.0,
            profile: "cam.audio".to_string(),
            device_id: "device_minimal".to_string(),
            first_gap_index: None,
            out_of_order: None,
            error: None,
            verify_time_ms: 100,
            chain_tip: None,
        };

        let json = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: VerifyReport =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(original.signature, deserialized.signature);
        assert_eq!(original.continuity, deserialized.continuity);
        assert_eq!(original.segments, deserialized.segments);
        assert_eq!(original.duration_s, deserialized.duration_s);
        assert_eq!(original.profile, deserialized.profile);
        assert_eq!(original.device_id, deserialized.device_id);
        assert_eq!(original.first_gap_index, deserialized.first_gap_index);
        assert_eq!(original.out_of_order, deserialized.out_of_order);
        assert_eq!(original.error, deserialized.error);
        assert_eq!(original.verify_time_ms, deserialized.verify_time_ms);
        assert_eq!(original.chain_tip, deserialized.chain_tip);
    }

    #[test]
    fn test_receipt_round_trip() {
        let original = VerificationReceipt {
            verification_id: "verify_abc123def456".to_string(),
            profile: "cam.video".to_string(),
            device_id: "device_12345".to_string(),
            manifest_digest:
                "sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
                    .to_string(),
            segments: 150,
            duration_s: 45.2,
            signature: "pass".to_string(),
            continuity: "pass".to_string(),
            issued_at: "2023-12-01T10:30:00Z".to_string(),
            service_kid: "service_key_001".to_string(),
            chain_tip: "b3:a1b2c3d4e5f6789abcdef0123456789".to_string(),
        };

        let json = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: VerificationReceipt =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(original.verification_id, deserialized.verification_id);
        assert_eq!(original.profile, deserialized.profile);
        assert_eq!(original.device_id, deserialized.device_id);
        assert_eq!(original.manifest_digest, deserialized.manifest_digest);
        assert_eq!(original.segments, deserialized.segments);
        assert_eq!(original.duration_s, deserialized.duration_s);
        assert_eq!(original.signature, deserialized.signature);
        assert_eq!(original.continuity, deserialized.continuity);
        assert_eq!(original.issued_at, deserialized.issued_at);
        assert_eq!(original.service_kid, deserialized.service_kid);
        assert_eq!(original.chain_tip, deserialized.chain_tip);
    }

    #[test]
    fn test_policy_v0_round_trip() {
        let original = PolicyV0 {
            required_profile: Some("cam.video".to_string()),
            min_segments: Some(10),
            chunk_seconds_range: Some((1.0, 60.0)),
            allowed_codecs: Some(vec!["h264".to_string(), "h265".to_string()]),
        };

        let json = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: PolicyV0 = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(original.required_profile, deserialized.required_profile);
        assert_eq!(original.min_segments, deserialized.min_segments);
        assert_eq!(
            original.chunk_seconds_range,
            deserialized.chunk_seconds_range
        );
        assert_eq!(original.allowed_codecs, deserialized.allowed_codecs);
    }

    #[test]
    fn test_policy_v0_default_round_trip() {
        let original = PolicyV0::default();

        let json = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: PolicyV0 = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(original.required_profile, deserialized.required_profile);
        assert_eq!(original.min_segments, deserialized.min_segments);
        assert_eq!(
            original.chunk_seconds_range,
            deserialized.chunk_seconds_range
        );
        assert_eq!(original.allowed_codecs, deserialized.allowed_codecs);
    }

    #[test]
    fn test_json_key_preservation() {
        let json_input = r#"{
            "signature": "pass",
            "continuity": "fail",
            "segments": 100,
            "duration_s": 30.5,
            "profile": "cam.video",
            "device_id": "test_device",
            "verify_time_ms": 500
        }"#;

        let verify_report: VerifyReport =
            serde_json::from_str(json_input).expect("Failed to deserialize");
        let json_output = serde_json::to_string(&verify_report).expect("Failed to serialize");

        assert!(json_output.contains("\"signature\":\"pass\""));
        assert!(json_output.contains("\"continuity\":\"fail\""));
        assert!(json_output.contains("\"segments\":100"));
        assert!(json_output.contains("\"duration_s\":30.5"));
        assert!(json_output.contains("\"profile\":\"cam.video\""));
        assert!(json_output.contains("\"device_id\":\"test_device\""));
        assert!(json_output.contains("\"verify_time_ms\":500"));
    }

    #[test]
    fn test_segment_ref_round_trip() {
        let original = SegmentRef {
            index: 42,
            hash: "sha256:a1b2c3d4e5f6".to_string(),
        };

        let json = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: SegmentRef = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(original.index, deserialized.index);
        assert_eq!(original.hash, deserialized.hash);
    }

    #[test]
    fn test_verify_options_round_trip() {
        let original = VerifyOptions {
            return_receipt: true,
            device_id: Some("device_12345".to_string()),
        };

        let json = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: VerifyOptions =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(original.return_receipt, deserialized.return_receipt);
        assert_eq!(original.device_id, deserialized.device_id);
    }

    #[test]
    fn test_verify_options_default() {
        let options = VerifyOptions::default();
        assert!(!options.return_receipt);
        assert_eq!(options.device_id, None);
    }

    #[test]
    fn test_verify_request_round_trip() {
        let original = VerifyRequest {
            device_pub: "ed25519:GAUpGXoor5gP".to_string(),
            manifest: serde_json::json!({
                "version": "1.0",
                "profile": "cam.video",
                "segments": 10
            }),
            segments: vec![
                SegmentRef {
                    index: 0,
                    hash: "sha256:a1b2c3".to_string(),
                },
                SegmentRef {
                    index: 1,
                    hash: "sha256:d4e5f6".to_string(),
                },
            ],
            options: VerifyOptions {
                return_receipt: true,
                device_id: Some("device_123".to_string()),
            },
        };

        let json = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: VerifyRequest =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(original.device_pub, deserialized.device_pub);
        assert_eq!(original.manifest, deserialized.manifest);
        assert_eq!(original.segments.len(), deserialized.segments.len());
        assert_eq!(original.segments[0].index, deserialized.segments[0].index);
        assert_eq!(original.segments[0].hash, deserialized.segments[0].hash);
        assert_eq!(
            original.options.return_receipt,
            deserialized.options.return_receipt
        );
        assert_eq!(original.options.device_id, deserialized.options.device_id);
    }

    #[test]
    fn test_verify_response_round_trip() {
        let original = VerifyResponse {
            verification_id: "verify_abc123def456".to_string(),
            result: VerifyReport {
                signature: "pass".to_string(),
                continuity: "pass".to_string(),
                segments: 150,
                duration_s: 45.2,
                profile: "cam.video".to_string(),
                device_id: "device_12345".to_string(),
                first_gap_index: None,
                out_of_order: None,
                error: None,
                verify_time_ms: 1250,
                chain_tip: Some("b3:a1b2c3d4e5f6789abcdef0123456789".to_string()),
            },
            receipt: Some("receipt_data_here".to_string()),
        };

        let json = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: VerifyResponse =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(original.verification_id, deserialized.verification_id);
        assert_eq!(original.result.signature, deserialized.result.signature);
        assert_eq!(original.result.continuity, deserialized.result.continuity);
        assert_eq!(original.result.segments, deserialized.result.segments);
        assert_eq!(original.receipt, deserialized.receipt);
    }

    #[test]
    fn test_verify_request_with_defaults() {
        let json_input = r#"{
            "device_pub": "ed25519:GAUpGXoor5gP",
            "manifest": {"version": "1.0"},
            "segments": [{"index": 0, "hash": "sha256:abc123"}]
        }"#;

        let verify_request: VerifyRequest =
            serde_json::from_str(json_input).expect("Failed to deserialize");

        assert_eq!(verify_request.device_pub, "ed25519:GAUpGXoor5gP");
        assert!(!verify_request.options.return_receipt);
        assert_eq!(verify_request.options.device_id, None);
        assert_eq!(verify_request.segments.len(), 1);
        assert_eq!(verify_request.segments[0].index, 0);
    }
}
