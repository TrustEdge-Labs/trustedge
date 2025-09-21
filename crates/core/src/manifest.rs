//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/
//
// Project: trustedge — Privacy and trust at the edge.
//

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ManifestError {
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Invalid field value: {0}")]
    InvalidField(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CamVideoManifest {
    pub trst_version: String,
    pub profile: String,
    pub device: DeviceInfo,
    pub capture: CaptureInfo,
    pub chunk: ChunkInfo,
    pub segments: Vec<SegmentInfo>,
    pub claims: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_archive_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub id: String,
    pub model: String,
    pub firmware_version: String,
    pub public_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureInfo {
    pub started_at: String,
    pub ended_at: String,
    pub timezone: String,
    pub fps: f64,
    pub resolution: String,
    pub codec: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkInfo {
    pub size_bytes: u64,
    pub duration_seconds: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentInfo {
    pub chunk_file: String,
    pub blake3_hash: String,
    pub start_time: String,
    pub duration_seconds: f64,
    pub continuity_hash: String,
}

impl CamVideoManifest {
    pub fn new() -> Self {
        Self {
            trst_version: "0.1.0".to_string(),
            profile: "cam.video".to_string(),
            device: DeviceInfo {
                id: String::new(),
                model: "TrustEdgeRefCam".to_string(),
                firmware_version: "1.0.0".to_string(),
                public_key: String::new(),
            },
            capture: CaptureInfo {
                started_at: String::new(),
                ended_at: String::new(),
                timezone: "UTC".to_string(),
                fps: 30.0,
                resolution: "1920x1080".to_string(),
                codec: "raw".to_string(),
            },
            chunk: ChunkInfo {
                size_bytes: 1048576, // 1MB default
                duration_seconds: 2.0,
            },
            segments: Vec::new(),
            claims: Vec::new(),
            prev_archive_hash: None,
            signature: None,
        }
    }

    /// Convert manifest to canonical bytes for signing/verification
    /// Signature field is excluded from canonicalization
    pub fn to_canonical_bytes(&self) -> Result<Vec<u8>, ManifestError> {
        // Create a copy without the signature field
        let mut canonical_manifest = self.clone();
        canonical_manifest.signature = None;

        // Serialize to JSON and manually reorder keys
        let json_string = self.serialize_with_ordered_keys(&canonical_manifest)?;

        Ok(json_string.into_bytes())
    }

    /// Serialize the manifest with explicitly ordered keys
    fn serialize_with_ordered_keys(
        &self,
        manifest: &CamVideoManifest,
    ) -> Result<String, ManifestError> {
        // Build the JSON object with explicit key ordering
        let mut result = String::from("{");

        // Add fields in the specified order
        result.push_str(&format!(
            "\"trst_version\":{}",
            serde_json::to_string(&manifest.trst_version)?
        ));
        result.push_str(&format!(
            ",\"profile\":{}",
            serde_json::to_string(&manifest.profile)?
        ));

        // Device object with ordered keys
        result.push_str(",\"device\":{");
        result.push_str(&format!(
            "\"id\":{}",
            serde_json::to_string(&manifest.device.id)?
        ));
        result.push_str(&format!(
            ",\"model\":{}",
            serde_json::to_string(&manifest.device.model)?
        ));
        result.push_str(&format!(
            ",\"firmware_version\":{}",
            serde_json::to_string(&manifest.device.firmware_version)?
        ));
        result.push_str(&format!(
            ",\"public_key\":{}",
            serde_json::to_string(&manifest.device.public_key)?
        ));
        result.push_str("}");

        // Capture object with ordered keys
        result.push_str(",\"capture\":{");
        result.push_str(&format!(
            "\"started_at\":{}",
            serde_json::to_string(&manifest.capture.started_at)?
        ));
        result.push_str(&format!(
            ",\"ended_at\":{}",
            serde_json::to_string(&manifest.capture.ended_at)?
        ));
        result.push_str(&format!(
            ",\"timezone\":{}",
            serde_json::to_string(&manifest.capture.timezone)?
        ));
        result.push_str(&format!(",\"fps\":{}", manifest.capture.fps));
        result.push_str(&format!(
            ",\"resolution\":{}",
            serde_json::to_string(&manifest.capture.resolution)?
        ));
        result.push_str(&format!(
            ",\"codec\":{}",
            serde_json::to_string(&manifest.capture.codec)?
        ));
        result.push_str("}");

        // Chunk object with ordered keys
        result.push_str(",\"chunk\":{");
        result.push_str(&format!("\"size_bytes\":{}", manifest.chunk.size_bytes));
        result.push_str(&format!(
            ",\"duration_seconds\":{}",
            manifest.chunk.duration_seconds
        ));
        result.push_str("}");

        // Segments array
        result.push_str(",\"segments\":[");
        for (i, segment) in manifest.segments.iter().enumerate() {
            if i > 0 {
                result.push_str(",");
            }
            result.push_str("{");
            result.push_str(&format!(
                "\"chunk_file\":{}",
                serde_json::to_string(&segment.chunk_file)?
            ));
            result.push_str(&format!(
                ",\"blake3_hash\":{}",
                serde_json::to_string(&segment.blake3_hash)?
            ));
            result.push_str(&format!(
                ",\"start_time\":{}",
                serde_json::to_string(&segment.start_time)?
            ));
            result.push_str(&format!(
                ",\"duration_seconds\":{}",
                segment.duration_seconds
            ));
            result.push_str(&format!(
                ",\"continuity_hash\":{}",
                serde_json::to_string(&segment.continuity_hash)?
            ));
            result.push_str("}");
        }
        result.push_str("]");

        // Claims array
        result.push_str(&format!(
            ",\"claims\":{}",
            serde_json::to_string(&manifest.claims)?
        ));

        // Optional prev_archive_hash
        if let Some(ref prev_hash) = manifest.prev_archive_hash {
            result.push_str(&format!(
                ",\"prev_archive_hash\":{}",
                serde_json::to_string(prev_hash)?
            ));
        }

        // Note: signature is explicitly excluded from canonicalization

        result.push_str("}");

        Ok(result)
    }

    /// Set signature on the manifest
    pub fn set_signature(&mut self, signature: String) {
        self.signature = Some(signature);
    }

    /// Validate manifest structure and required fields
    pub fn validate(&self) -> Result<(), ManifestError> {
        if self.trst_version.is_empty() {
            return Err(ManifestError::InvalidField(
                "trst_version cannot be empty".to_string(),
            ));
        }

        if self.profile != "cam.video" {
            return Err(ManifestError::InvalidField(
                "profile must be 'cam.video'".to_string(),
            ));
        }

        if self.device.id.is_empty() {
            return Err(ManifestError::InvalidField(
                "device.id cannot be empty".to_string(),
            ));
        }

        if self.device.public_key.is_empty() {
            return Err(ManifestError::InvalidField(
                "device.public_key cannot be empty".to_string(),
            ));
        }

        if self.capture.started_at.is_empty() {
            return Err(ManifestError::InvalidField(
                "capture.started_at cannot be empty".to_string(),
            ));
        }

        if self.capture.ended_at.is_empty() {
            return Err(ManifestError::InvalidField(
                "capture.ended_at cannot be empty".to_string(),
            ));
        }

        if self.segments.is_empty() {
            return Err(ManifestError::InvalidField(
                "segments cannot be empty".to_string(),
            ));
        }

        for (i, segment) in self.segments.iter().enumerate() {
            if segment.chunk_file.is_empty() {
                return Err(ManifestError::InvalidField(format!(
                    "segment[{}].chunk_file cannot be empty",
                    i
                )));
            }
            if segment.blake3_hash.is_empty() {
                return Err(ManifestError::InvalidField(format!(
                    "segment[{}].blake3_hash cannot be empty",
                    i
                )));
            }
            if segment.continuity_hash.is_empty() {
                return Err(ManifestError::InvalidField(format!(
                    "segment[{}].continuity_hash cannot be empty",
                    i
                )));
            }
        }

        Ok(())
    }
}

impl Default for CamVideoManifest {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manifest_creation() {
        let manifest = CamVideoManifest::new();
        assert_eq!(manifest.trst_version, "0.1.0");
        assert_eq!(manifest.profile, "cam.video");
        assert_eq!(manifest.device.model, "TrustEdgeRefCam");
        assert_eq!(manifest.capture.fps, 30.0);
    }

    #[test]
    fn test_canonical_bytes_excludes_signature() {
        let mut manifest = CamVideoManifest::new();
        manifest.device.id = "TEST001".to_string();
        manifest.device.public_key = "ed25519:test_key".to_string();
        manifest.capture.started_at = "2025-01-15T10:30:00Z".to_string();
        manifest.capture.ended_at = "2025-01-15T10:30:02Z".to_string();
        manifest.segments.push(SegmentInfo {
            chunk_file: "00000.bin".to_string(),
            blake3_hash: "abc123".to_string(),
            start_time: "2025-01-15T10:30:00Z".to_string(),
            duration_seconds: 2.0,
            continuity_hash: "def456".to_string(),
        });

        // Test without signature
        let bytes_without_sig = manifest.to_canonical_bytes().unwrap();
        let json_str_without_sig = String::from_utf8(bytes_without_sig.clone()).unwrap();
        assert!(!json_str_without_sig.contains("signature"));

        // Test with signature - should produce same canonical bytes
        manifest.set_signature("test_signature".to_string());
        let bytes_with_sig = manifest.to_canonical_bytes().unwrap();

        assert_eq!(bytes_without_sig, bytes_with_sig);
    }

    #[test]
    fn test_key_ordering() {
        let mut manifest = CamVideoManifest::new();
        manifest.device.id = "TEST001".to_string();
        manifest.device.public_key = "ed25519:test_key".to_string();
        manifest.capture.started_at = "2025-01-15T10:30:00Z".to_string();
        manifest.capture.ended_at = "2025-01-15T10:30:02Z".to_string();
        manifest.segments.push(SegmentInfo {
            chunk_file: "00000.bin".to_string(),
            blake3_hash: "abc123".to_string(),
            start_time: "2025-01-15T10:30:00Z".to_string(),
            duration_seconds: 2.0,
            continuity_hash: "def456".to_string(),
        });

        let canonical_bytes = manifest.to_canonical_bytes().unwrap();
        let json_str = String::from_utf8(canonical_bytes).unwrap();

        // Verify root-level key ordering
        let trst_pos = json_str.find("\"trst_version\"").unwrap();
        let profile_pos = json_str.find("\"profile\"").unwrap();
        let device_pos = json_str.find("\"device\"").unwrap();
        let capture_pos = json_str.find("\"capture\"").unwrap();
        let chunk_pos = json_str.find("\"chunk\"").unwrap();
        let segments_pos = json_str.find("\"segments\"").unwrap();
        let claims_pos = json_str.find("\"claims\"").unwrap();

        assert!(trst_pos < profile_pos);
        assert!(profile_pos < device_pos);
        assert!(device_pos < capture_pos);
        assert!(capture_pos < chunk_pos);
        assert!(chunk_pos < segments_pos);
        assert!(segments_pos < claims_pos);
    }

    #[test]
    fn test_decimal_precision() {
        let mut manifest = CamVideoManifest::new();
        manifest.capture.fps = 29.97;
        manifest.capture.started_at = "2025-01-15T10:30:00Z".to_string();
        manifest.capture.ended_at = "2025-01-15T10:30:02Z".to_string();
        manifest.device.id = "TEST001".to_string();
        manifest.device.public_key = "ed25519:test_key".to_string();
        manifest.segments.push(SegmentInfo {
            chunk_file: "00000.bin".to_string(),
            blake3_hash: "abc123".to_string(),
            start_time: "2025-01-15T10:30:00Z".to_string(),
            duration_seconds: 2.0,
            continuity_hash: "def456".to_string(),
        });

        let canonical_bytes = manifest.to_canonical_bytes().unwrap();
        let json_str = String::from_utf8(canonical_bytes).unwrap();

        // Verify decimal precision is preserved
        assert!(json_str.contains("29.97"));
    }

    #[test]
    fn test_validation() {
        let mut manifest = CamVideoManifest::new();

        // Should fail validation initially (missing required fields)
        assert!(manifest.validate().is_err());

        // Fill in required fields
        manifest.device.id = "TEST001".to_string();
        manifest.device.public_key = "ed25519:test_key".to_string();
        manifest.capture.started_at = "2025-01-15T10:30:00Z".to_string();
        manifest.capture.ended_at = "2025-01-15T10:30:02Z".to_string();
        manifest.segments.push(SegmentInfo {
            chunk_file: "00000.bin".to_string(),
            blake3_hash: "abc123".to_string(),
            start_time: "2025-01-15T10:30:00Z".to_string(),
            duration_seconds: 2.0,
            continuity_hash: "def456".to_string(),
        });

        // Should pass validation now
        assert!(manifest.validate().is_ok());
    }

    #[test]
    fn test_stable_canonicalization() {
        let mut manifest = CamVideoManifest::new();
        manifest.device.id = "TEST001".to_string();
        manifest.device.public_key = "ed25519:test_key".to_string();
        manifest.capture.started_at = "2025-01-15T10:30:00Z".to_string();
        manifest.capture.ended_at = "2025-01-15T10:30:02Z".to_string();
        manifest.segments.push(SegmentInfo {
            chunk_file: "00000.bin".to_string(),
            blake3_hash: "abc123".to_string(),
            start_time: "2025-01-15T10:30:00Z".to_string(),
            duration_seconds: 2.0,
            continuity_hash: "def456".to_string(),
        });

        // Multiple calls should produce identical results
        let bytes1 = manifest.to_canonical_bytes().unwrap();
        let bytes2 = manifest.to_canonical_bytes().unwrap();
        let bytes3 = manifest.to_canonical_bytes().unwrap();

        assert_eq!(bytes1, bytes2);
        assert_eq!(bytes2, bytes3);
    }
}
