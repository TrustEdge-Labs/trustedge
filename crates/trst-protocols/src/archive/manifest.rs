//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//

//! Profile-agnostic manifest types for .trst archives.
//!
//! `TrstManifest` is the unified manifest type supporting both `generic` and
//! `cam.video` profiles via the `ProfileMetadata` enum.  The `CamVideoManifest`
//! and `CaptureInfo` names remain as type aliases for backward compatibility.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ManifestFormatError {
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Invalid field value: {0}")]
    InvalidField(String),
}

// ─── Profile metadata variants ───────────────────────────────────────────────

/// Metadata for the `generic` profile.  All content-specific fields are
/// optional; `started_at` and `ended_at` are the only required timestamps.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GenericMetadata {
    pub started_at: String,
    pub ended_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// Free-form key/value labels.  BTreeMap ensures sorted keys in canonical output.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub labels: BTreeMap<String, String>,
}

/// Metadata for the `cam.video` profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CamVideoMetadata {
    pub started_at: String,
    pub ended_at: String,
    pub timezone: String,
    pub fps: f64,
    pub resolution: String,
    pub codec: String,
}

/// Union of all profile-specific metadata.
///
/// `#[serde(untagged)]` means serde tries each variant in declaration order.
/// `CamVideo` is listed first because it has required fields (`timezone`,
/// `fps`, `resolution`, `codec`) that unambiguously distinguish it from
/// `Generic`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProfileMetadata {
    CamVideo(CamVideoMetadata),
    Generic(GenericMetadata),
}

// ─── Supporting structs (unchanged) ──────────────────────────────────────────

/// Device information embedded in the manifest.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub id: String,
    pub model: String,
    pub firmware_version: String,
    pub public_key: String,
}

/// Chunk configuration for the archive.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkInfo {
    pub size_bytes: u64,
    pub duration_seconds: f64,
}

/// Individual segment information within the archive.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentInfo {
    pub chunk_file: String,
    pub blake3_hash: String,
    pub start_time: String,
    pub duration_seconds: f64,
    pub continuity_hash: String,
}

// ─── Main manifest type ───────────────────────────────────────────────────────

/// Profile-agnostic manifest for a `.trst` archive.
///
/// Supports `"generic"` and `"cam.video"` profiles via the `metadata` field.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrstManifest {
    pub trst_version: String,
    pub profile: String,
    pub device: DeviceInfo,
    pub metadata: ProfileMetadata,
    pub chunk: ChunkInfo,
    pub segments: Vec<SegmentInfo>,
    pub claims: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_archive_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

// ─── Backward-compatibility aliases ──────────────────────────────────────────

/// Backward-compatible alias for `TrstManifest`.
pub type CamVideoManifest = TrstManifest;

/// Backward-compatible alias for `CamVideoMetadata` (the old `CaptureInfo` struct).
pub type CaptureInfo = CamVideoMetadata;

// ─── TrstManifest implementation ──────────────────────────────────────────────

impl TrstManifest {
    /// Create a new manifest with the `generic` profile and empty metadata.
    pub fn new() -> Self {
        Self {
            trst_version: "0.1.0".to_string(),
            profile: "generic".to_string(),
            device: DeviceInfo {
                id: String::new(),
                model: "TrustEdgeRefCam".to_string(),
                firmware_version: "1.0.0".to_string(),
                public_key: String::new(),
            },
            metadata: ProfileMetadata::Generic(GenericMetadata::default()),
            chunk: ChunkInfo {
                size_bytes: 1_048_576, // 1 MB default
                duration_seconds: 2.0,
            },
            segments: Vec::new(),
            claims: Vec::new(),
            prev_archive_hash: None,
            signature: None,
        }
    }

    /// Create a new manifest pre-configured for the `cam.video` profile.
    pub fn new_cam_video() -> Self {
        Self {
            trst_version: "0.1.0".to_string(),
            profile: "cam.video".to_string(),
            device: DeviceInfo {
                id: String::new(),
                model: "TrustEdgeRefCam".to_string(),
                firmware_version: "1.0.0".to_string(),
                public_key: String::new(),
            },
            metadata: ProfileMetadata::CamVideo(CamVideoMetadata {
                started_at: String::new(),
                ended_at: String::new(),
                timezone: "UTC".to_string(),
                fps: 30.0,
                resolution: "1920x1080".to_string(),
                codec: "raw".to_string(),
            }),
            chunk: ChunkInfo {
                size_bytes: 1_048_576,
                duration_seconds: 2.0,
            },
            segments: Vec::new(),
            claims: Vec::new(),
            prev_archive_hash: None,
            signature: None,
        }
    }

    /// Convert manifest to canonical bytes for signing/verification.
    /// The `signature` field is excluded from canonicalization.
    pub fn to_canonical_bytes(&self) -> Result<Vec<u8>, ManifestFormatError> {
        let mut canonical = self.clone();
        canonical.signature = None;
        let json_string = Self::serialize_canonical(&canonical)?;
        Ok(json_string.into_bytes())
    }

    /// Build a deterministically-ordered JSON string for a manifest.
    fn serialize_canonical(manifest: &TrstManifest) -> Result<String, ManifestFormatError> {
        let mut result = String::from("{");

        result.push_str(&format!(
            "\"trst_version\":{}",
            serde_json::to_string(&manifest.trst_version)?
        ));
        result.push_str(&format!(
            ",\"profile\":{}",
            serde_json::to_string(&manifest.profile)?
        ));

        // Device — ordered fields
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
        result.push('}');

        // Metadata — dispatch on variant
        match &manifest.metadata {
            ProfileMetadata::CamVideo(m) => {
                result.push_str(",\"metadata\":{");
                result.push_str(&format!(
                    "\"started_at\":{}",
                    serde_json::to_string(&m.started_at)?
                ));
                result.push_str(&format!(
                    ",\"ended_at\":{}",
                    serde_json::to_string(&m.ended_at)?
                ));
                result.push_str(&format!(
                    ",\"timezone\":{}",
                    serde_json::to_string(&m.timezone)?
                ));
                result.push_str(&format!(",\"fps\":{}", m.fps));
                result.push_str(&format!(
                    ",\"resolution\":{}",
                    serde_json::to_string(&m.resolution)?
                ));
                result.push_str(&format!(",\"codec\":{}", serde_json::to_string(&m.codec)?));
                result.push('}');
            }
            ProfileMetadata::Generic(m) => {
                result.push_str(",\"metadata\":{");
                result.push_str(&format!(
                    "\"started_at\":{}",
                    serde_json::to_string(&m.started_at)?
                ));
                result.push_str(&format!(
                    ",\"ended_at\":{}",
                    serde_json::to_string(&m.ended_at)?
                ));
                if let Some(ref v) = m.data_type {
                    result.push_str(&format!(",\"data_type\":{}", serde_json::to_string(v)?));
                }
                if let Some(ref v) = m.source {
                    result.push_str(&format!(",\"source\":{}", serde_json::to_string(v)?));
                }
                if let Some(ref v) = m.description {
                    result.push_str(&format!(",\"description\":{}", serde_json::to_string(v)?));
                }
                if let Some(ref v) = m.mime_type {
                    result.push_str(&format!(",\"mime_type\":{}", serde_json::to_string(v)?));
                }
                if !m.labels.is_empty() {
                    // BTreeMap guarantees sorted keys
                    result.push_str(",\"labels\":{");
                    let mut first = true;
                    for (k, v) in &m.labels {
                        if !first {
                            result.push(',');
                        }
                        first = false;
                        result.push_str(&format!(
                            "{}:{}",
                            serde_json::to_string(k)?,
                            serde_json::to_string(v)?
                        ));
                    }
                    result.push('}');
                }
                result.push('}');
            }
        }

        // Chunk
        result.push_str(",\"chunk\":{");
        result.push_str(&format!("\"size_bytes\":{}", manifest.chunk.size_bytes));
        result.push_str(&format!(
            ",\"duration_seconds\":{}",
            manifest.chunk.duration_seconds
        ));
        result.push('}');

        // Segments
        result.push_str(",\"segments\":[");
        for (i, segment) in manifest.segments.iter().enumerate() {
            if i > 0 {
                result.push(',');
            }
            result.push('{');
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
            result.push('}');
        }
        result.push(']');

        // Claims
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

        result.push('}');
        Ok(result)
    }

    /// Set the detached signature on this manifest.
    pub fn set_signature(&mut self, signature: String) {
        self.signature = Some(signature);
    }

    /// Validate manifest structure and required fields.
    pub fn validate(&self) -> Result<(), ManifestFormatError> {
        if self.trst_version.is_empty() {
            return Err(ManifestFormatError::InvalidField(
                "trst_version cannot be empty".to_string(),
            ));
        }

        // Accept both "generic" and "cam.video" — no longer restricted to cam.video
        if self.profile != "generic" && self.profile != "cam.video" {
            return Err(ManifestFormatError::InvalidField(format!(
                "profile must be 'generic' or 'cam.video', got '{}'",
                self.profile
            )));
        }

        if self.device.id.is_empty() {
            return Err(ManifestFormatError::InvalidField(
                "device.id cannot be empty".to_string(),
            ));
        }

        if self.device.public_key.is_empty() {
            return Err(ManifestFormatError::InvalidField(
                "device.public_key cannot be empty".to_string(),
            ));
        }

        // Validate metadata timestamps based on variant
        match &self.metadata {
            ProfileMetadata::CamVideo(m) => {
                if m.started_at.is_empty() {
                    return Err(ManifestFormatError::InvalidField(
                        "metadata.started_at cannot be empty".to_string(),
                    ));
                }
                if m.ended_at.is_empty() {
                    return Err(ManifestFormatError::InvalidField(
                        "metadata.ended_at cannot be empty".to_string(),
                    ));
                }
            }
            ProfileMetadata::Generic(m) => {
                if m.started_at.is_empty() {
                    return Err(ManifestFormatError::InvalidField(
                        "metadata.started_at cannot be empty".to_string(),
                    ));
                }
                if m.ended_at.is_empty() {
                    return Err(ManifestFormatError::InvalidField(
                        "metadata.ended_at cannot be empty".to_string(),
                    ));
                }
            }
        }

        if self.segments.is_empty() {
            return Err(ManifestFormatError::InvalidField(
                "segments cannot be empty".to_string(),
            ));
        }

        for (i, segment) in self.segments.iter().enumerate() {
            if segment.chunk_file.is_empty() {
                return Err(ManifestFormatError::InvalidField(format!(
                    "segment[{}].chunk_file cannot be empty",
                    i
                )));
            }
            if segment.blake3_hash.is_empty() {
                return Err(ManifestFormatError::InvalidField(format!(
                    "segment[{}].blake3_hash cannot be empty",
                    i
                )));
            }
            if segment.continuity_hash.is_empty() {
                return Err(ManifestFormatError::InvalidField(format!(
                    "segment[{}].continuity_hash cannot be empty",
                    i
                )));
            }
        }

        Ok(())
    }
}

impl Default for TrstManifest {
    fn default() -> Self {
        Self::new()
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn cam_video_manifest() -> TrstManifest {
        let mut m = TrstManifest::new_cam_video();
        m.device.id = "TEST001".to_string();
        m.device.public_key = "ed25519:test_key".to_string();
        if let ProfileMetadata::CamVideo(ref mut meta) = m.metadata {
            meta.started_at = "2025-01-15T10:30:00Z".to_string();
            meta.ended_at = "2025-01-15T10:30:02Z".to_string();
        }
        m.segments.push(SegmentInfo {
            chunk_file: "00000.bin".to_string(),
            blake3_hash: "abc123".to_string(),
            start_time: "2025-01-15T10:30:00Z".to_string(),
            duration_seconds: 2.0,
            continuity_hash: "def456".to_string(),
        });
        m
    }

    fn generic_manifest() -> TrstManifest {
        let mut m = TrstManifest::new();
        m.device.id = "GENERIC001".to_string();
        m.device.public_key = "ed25519:generic_key".to_string();
        if let ProfileMetadata::Generic(ref mut meta) = m.metadata {
            meta.started_at = "2025-01-15T10:30:00Z".to_string();
            meta.ended_at = "2025-01-15T10:30:02Z".to_string();
        }
        m.segments.push(SegmentInfo {
            chunk_file: "00000.bin".to_string(),
            blake3_hash: "abc123".to_string(),
            start_time: "2025-01-15T10:30:00Z".to_string(),
            duration_seconds: 2.0,
            continuity_hash: "def456".to_string(),
        });
        m
    }

    // ── New profile-agnostic tests ──

    #[test]
    fn test_generic_metadata_canonical_serialization() {
        let mut m = TrstManifest::new();
        m.device.id = "G001".to_string();
        m.device.public_key = "ed25519:k".to_string();
        if let ProfileMetadata::Generic(ref mut meta) = m.metadata {
            meta.started_at = "2025-01-15T10:00:00Z".to_string();
            meta.ended_at = "2025-01-15T10:00:10Z".to_string();
            meta.data_type = Some("sensor".to_string());
            meta.source = Some("drone-cam-01".to_string());
            meta.description = Some("Test capture".to_string());
            meta.mime_type = Some("application/octet-stream".to_string());
            meta.labels.insert("env".to_string(), "prod".to_string());
            meta.labels
                .insert("camera".to_string(), "front".to_string());
        }
        m.segments.push(SegmentInfo {
            chunk_file: "00000.bin".to_string(),
            blake3_hash: "abc".to_string(),
            start_time: "2025-01-15T10:00:00Z".to_string(),
            duration_seconds: 10.0,
            continuity_hash: "def".to_string(),
        });

        let bytes = m.to_canonical_bytes().unwrap();
        let json = String::from_utf8(bytes).unwrap();

        // Must contain metadata section
        assert!(json.contains("\"metadata\""));
        assert!(json.contains("\"data_type\""));
        assert!(json.contains("\"source\""));
        // Labels must be sorted: "camera" before "env"
        let camera_pos = json.find("\"camera\"").unwrap();
        let env_pos = json.find("\"env\"").unwrap();
        assert!(camera_pos < env_pos, "labels must have sorted keys");
    }

    #[test]
    fn test_cam_video_metadata_canonical_serialization() {
        let m = cam_video_manifest();
        let bytes = m.to_canonical_bytes().unwrap();
        let json = String::from_utf8(bytes).unwrap();

        assert!(json.contains("\"metadata\""));
        assert!(json.contains("\"fps\""));
        assert!(json.contains("\"resolution\""));
        assert!(json.contains("\"codec\""));
        assert!(json.contains("\"timezone\""));
    }

    #[test]
    fn test_generic_metadata_round_trip() {
        let mut m = TrstManifest::new();
        if let ProfileMetadata::Generic(ref mut meta) = m.metadata {
            meta.started_at = "2025-01-15T10:00:00Z".to_string();
            meta.ended_at = "2025-01-15T10:00:10Z".to_string();
            meta.data_type = Some("video".to_string());
            meta.source = Some("cam-01".to_string());
            meta.labels
                .insert("region".to_string(), "us-east".to_string());
        }

        let json = serde_json::to_string(&m).unwrap();
        let decoded: TrstManifest = serde_json::from_str(&json).unwrap();

        if let ProfileMetadata::Generic(meta) = decoded.metadata {
            assert_eq!(meta.data_type, Some("video".to_string()));
            assert_eq!(meta.source, Some("cam-01".to_string()));
            assert_eq!(meta.labels.get("region"), Some(&"us-east".to_string()));
        } else {
            panic!("Expected Generic variant");
        }
    }

    #[test]
    fn test_generic_metadata_minimal_round_trip() {
        // All optional fields absent
        let mut m = TrstManifest::new();
        if let ProfileMetadata::Generic(ref mut meta) = m.metadata {
            meta.started_at = "2025-01-15T10:00:00Z".to_string();
            meta.ended_at = "2025-01-15T10:00:10Z".to_string();
        }

        let json = serde_json::to_string(&m).unwrap();
        let decoded: TrstManifest = serde_json::from_str(&json).unwrap();

        if let ProfileMetadata::Generic(meta) = decoded.metadata {
            assert!(meta.data_type.is_none());
            assert!(meta.source.is_none());
            assert!(meta.labels.is_empty());
        } else {
            panic!("Expected Generic variant");
        }
    }

    #[test]
    fn test_validation_accepts_generic_profile() {
        let m = generic_manifest();
        assert!(
            m.validate().is_ok(),
            "validate() must accept profile='generic'"
        );
    }

    #[test]
    fn test_validation_accepts_cam_video_profile() {
        let m = cam_video_manifest();
        assert!(
            m.validate().is_ok(),
            "validate() must accept profile='cam.video'"
        );
    }

    #[test]
    fn test_validation_rejects_unknown_profile() {
        let mut m = TrstManifest::new();
        m.profile = "unknown".to_string();
        m.device.id = "X".to_string();
        m.device.public_key = "ed25519:k".to_string();
        if let ProfileMetadata::Generic(ref mut meta) = m.metadata {
            meta.started_at = "2025-01-15T10:00:00Z".to_string();
            meta.ended_at = "2025-01-15T10:00:01Z".to_string();
        }
        m.segments.push(SegmentInfo {
            chunk_file: "00000.bin".to_string(),
            blake3_hash: "h".to_string(),
            start_time: "t".to_string(),
            duration_seconds: 1.0,
            continuity_hash: "c".to_string(),
        });
        assert!(m.validate().is_err());
    }

    #[test]
    fn test_validation_rejects_empty_trst_version() {
        let mut m = generic_manifest();
        m.trst_version = String::new();
        assert!(m.validate().is_err());
    }

    #[test]
    fn test_validation_rejects_empty_device_id() {
        let mut m = generic_manifest();
        m.device.id = String::new();
        assert!(m.validate().is_err());
    }

    #[test]
    fn test_validation_rejects_empty_public_key() {
        let mut m = generic_manifest();
        m.device.public_key = String::new();
        assert!(m.validate().is_err());
    }

    #[test]
    fn test_cam_video_manifest_alias_compiles() {
        // CamVideoManifest is a type alias for TrstManifest — this must compile
        let m: CamVideoManifest = TrstManifest::new_cam_video();
        assert_eq!(m.profile, "cam.video");
    }

    #[test]
    fn test_labels_sorted_in_canonical_output() {
        let mut m = TrstManifest::new();
        m.device.id = "X".to_string();
        m.device.public_key = "ed25519:k".to_string();
        if let ProfileMetadata::Generic(ref mut meta) = m.metadata {
            meta.started_at = "t".to_string();
            meta.ended_at = "t".to_string();
            meta.labels.insert("z-key".to_string(), "1".to_string());
            meta.labels.insert("a-key".to_string(), "2".to_string());
            meta.labels.insert("m-key".to_string(), "3".to_string());
        }
        m.segments.push(SegmentInfo {
            chunk_file: "00000.bin".to_string(),
            blake3_hash: "h".to_string(),
            start_time: "t".to_string(),
            duration_seconds: 1.0,
            continuity_hash: "c".to_string(),
        });

        let bytes = m.to_canonical_bytes().unwrap();
        let json = String::from_utf8(bytes).unwrap();

        let a_pos = json.find("\"a-key\"").unwrap();
        let m_pos = json.find("\"m-key\"").unwrap();
        let z_pos = json.find("\"z-key\"").unwrap();
        assert!(a_pos < m_pos && m_pos < z_pos, "labels must be sorted");
    }

    // ── Retained tests (updated for new API) ──

    #[test]
    fn test_manifest_creation() {
        let manifest = TrstManifest::new_cam_video();
        assert_eq!(manifest.trst_version, "0.1.0");
        assert_eq!(manifest.profile, "cam.video");
        assert_eq!(manifest.device.model, "TrustEdgeRefCam");
        if let ProfileMetadata::CamVideo(m) = &manifest.metadata {
            assert_eq!(m.fps, 30.0);
        } else {
            panic!("Expected CamVideo variant");
        }
    }

    #[test]
    fn test_canonical_bytes_excludes_signature() {
        let mut manifest = cam_video_manifest();

        let bytes_without_sig = manifest.to_canonical_bytes().unwrap();
        let json_str = String::from_utf8(bytes_without_sig.clone()).unwrap();
        assert!(!json_str.contains("signature"));

        manifest.set_signature("test_signature".to_string());
        let bytes_with_sig = manifest.to_canonical_bytes().unwrap();

        assert_eq!(bytes_without_sig, bytes_with_sig);
    }

    #[test]
    fn test_key_ordering() {
        let manifest = cam_video_manifest();
        let canonical_bytes = manifest.to_canonical_bytes().unwrap();
        let json_str = String::from_utf8(canonical_bytes).unwrap();

        let trst_pos = json_str.find("\"trst_version\"").unwrap();
        let profile_pos = json_str.find("\"profile\"").unwrap();
        let device_pos = json_str.find("\"device\"").unwrap();
        let metadata_pos = json_str.find("\"metadata\"").unwrap();
        let chunk_pos = json_str.find("\"chunk\"").unwrap();
        let segments_pos = json_str.find("\"segments\"").unwrap();
        let claims_pos = json_str.find("\"claims\"").unwrap();

        assert!(trst_pos < profile_pos);
        assert!(profile_pos < device_pos);
        assert!(device_pos < metadata_pos);
        assert!(metadata_pos < chunk_pos);
        assert!(chunk_pos < segments_pos);
        assert!(segments_pos < claims_pos);
    }

    #[test]
    fn test_validation() {
        let manifest = cam_video_manifest();
        assert!(manifest.validate().is_ok());
    }

    #[test]
    fn test_stable_canonicalization() {
        let manifest = cam_video_manifest();

        let bytes1 = manifest.to_canonical_bytes().unwrap();
        let bytes2 = manifest.to_canonical_bytes().unwrap();
        let bytes3 = manifest.to_canonical_bytes().unwrap();

        assert_eq!(bytes1, bytes2);
        assert_eq!(bytes2, bytes3);
    }

    #[test]
    fn test_decimal_precision() {
        let mut manifest = TrstManifest::new_cam_video();
        manifest.device.id = "TEST001".to_string();
        manifest.device.public_key = "ed25519:test_key".to_string();
        if let ProfileMetadata::CamVideo(ref mut m) = manifest.metadata {
            m.fps = 29.97;
            m.started_at = "2025-01-15T10:30:00Z".to_string();
            m.ended_at = "2025-01-15T10:30:02Z".to_string();
        }
        manifest.segments.push(SegmentInfo {
            chunk_file: "00000.bin".to_string(),
            blake3_hash: "abc123".to_string(),
            start_time: "2025-01-15T10:30:00Z".to_string(),
            duration_seconds: 2.0,
            continuity_hash: "def456".to_string(),
        });

        let canonical_bytes = manifest.to_canonical_bytes().unwrap();
        let json_str = String::from_utf8(canonical_bytes).unwrap();
        assert!(json_str.contains("29.97"));
    }
}
