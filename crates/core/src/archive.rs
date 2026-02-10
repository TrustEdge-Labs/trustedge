//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/
//
// Project: trustedge — Privacy and trust at the edge.
//

use crate::manifest::CamVideoManifest;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;

pub use crate::error::{ArchiveError, ChainError, ManifestError};

/// Type alias for chunk data (index, bytes)
type ChunkData = Vec<(usize, Vec<u8>)>;

/// Write a complete .trst archive with manifest, signature, and chunk files
pub fn write_archive<P: AsRef<Path>>(
    base_dir: P,
    manifest: &CamVideoManifest,
    chunk_ciphertexts: Vec<Vec<u8>>,
    detached_sig: &[u8],
) -> Result<(), ArchiveError> {
    let base_path = base_dir.as_ref();

    // Validate inputs
    if chunk_ciphertexts.len() != manifest.segments.len() {
        return Err(ArchiveError::SchemaMismatch(format!(
            "Chunk count mismatch: {} chunks provided, {} segments in manifest",
            chunk_ciphertexts.len(),
            manifest.segments.len()
        )));
    }

    // Create directory structure
    fs::create_dir_all(base_path)?;
    fs::create_dir_all(base_path.join("signatures"))?;
    fs::create_dir_all(base_path.join("chunks"))?;

    // Write manifest.json
    let manifest_json = serde_json::to_string_pretty(manifest)?;
    let mut manifest_file = File::create(base_path.join("manifest.json"))?;
    manifest_file.write_all(manifest_json.as_bytes())?;

    // Write detached signature
    let mut sig_file = File::create(base_path.join("signatures/manifest.sig"))?;
    sig_file.write_all(detached_sig)?;

    // Write chunk files with zero-padded five-digit names
    for (index, chunk_data) in chunk_ciphertexts.iter().enumerate() {
        let chunk_filename = format!("{:05}.bin", index);
        let chunk_path = base_path.join("chunks").join(chunk_filename);
        let mut chunk_file = File::create(chunk_path)?;
        chunk_file.write_all(chunk_data)?;
    }

    Ok(())
}

/// Read a complete .trst archive and return manifest and chunk data
pub fn read_archive<P: AsRef<Path>>(
    base_dir: P,
) -> Result<(CamVideoManifest, ChunkData), ArchiveError> {
    let base_path = base_dir.as_ref();

    // Read and parse manifest.json
    let manifest_path = base_path.join("manifest.json");
    let mut manifest_file = File::open(manifest_path)?;
    let mut manifest_content = String::new();
    manifest_file.read_to_string(&mut manifest_content)?;
    let manifest: CamVideoManifest = serde_json::from_str(&manifest_content)?;

    // Read detached signature
    let sig_path = base_path.join("signatures/manifest.sig");
    let mut sig_file = File::open(sig_path)?;
    let mut detached_sig = Vec::new();
    sig_file.read_to_end(&mut detached_sig)?;

    // Validate signature consistency
    if let Some(ref embedded_sig) = manifest.signature {
        let detached_sig_str = String::from_utf8_lossy(&detached_sig);
        if embedded_sig != &detached_sig_str {
            return Err(ArchiveError::SignatureMismatch);
        }
    }

    // Read chunk files
    let chunks_dir = base_path.join("chunks");
    let mut chunk_data = Vec::new();

    for (expected_index, segment) in manifest.segments.iter().enumerate() {
        let chunk_filename = format!("{:05}.bin", expected_index);
        let chunk_path = chunks_dir.join(&chunk_filename);

        // Check if chunk file exists
        if !chunk_path.exists() {
            return Err(ArchiveError::MissingChunk(chunk_filename));
        }

        // Validate that segment.chunk_file matches expected name
        if segment.chunk_file != chunk_filename {
            return Err(ArchiveError::InvalidChunkIndex {
                expected: expected_index,
                found: parse_chunk_index(&segment.chunk_file)?,
            });
        }

        // Read chunk data
        let mut chunk_file = File::open(chunk_path)?;
        let mut chunk_bytes = Vec::new();
        chunk_file.read_to_end(&mut chunk_bytes)?;

        chunk_data.push((expected_index, chunk_bytes));
    }

    Ok((manifest, chunk_data))
}

/// Validate archive integrity including continuity chain
pub fn validate_archive<P: AsRef<Path>>(base_dir: P) -> Result<(), ArchiveError> {
    let (manifest, chunk_data) = read_archive(base_dir)?;

    // Validate manifest structure
    manifest.validate().map_err(|e| {
        ArchiveError::ValidationFailed(format!("Manifest validation failed: {}", e))
    })?;

    // Validate chunk hashes and continuity chain
    let mut chain_segments = Vec::new();

    for ((index, chunk_bytes), segment) in chunk_data.iter().zip(manifest.segments.iter()) {
        // Compute BLAKE3 hash of chunk
        let computed_hash = crate::chain::segment_hash(chunk_bytes);
        let computed_hash_hex = hex::encode(computed_hash);

        // Check if stored hash matches computed hash
        if segment.blake3_hash != computed_hash_hex {
            return Err(ArchiveError::ValidationFailed(format!(
                "Chunk {} hash mismatch: expected {}, computed {}",
                index, segment.blake3_hash, computed_hash_hex
            )));
        }

        // Parse stored continuity hash
        let stored_continuity = hex::decode(&segment.continuity_hash).map_err(|_| {
            ArchiveError::ValidationFailed(format!(
                "Invalid continuity hash format: {}",
                segment.continuity_hash
            ))
        })?;

        if stored_continuity.len() != 32 {
            return Err(ArchiveError::ValidationFailed(format!(
                "Continuity hash must be 32 bytes, got {}",
                stored_continuity.len()
            )));
        }

        let mut continuity_array = [0u8; 32];
        continuity_array.copy_from_slice(&stored_continuity);

        chain_segments.push(crate::chain::ChainSegment {
            index: *index,
            stored_hash: computed_hash,
            stored_continuity: continuity_array,
        });
    }

    // Validate continuity chain
    crate::chain::validate_chain(&chain_segments)?;

    Ok(())
}

/// Parse chunk index from filename (e.g., "00002.bin" -> 2)
fn parse_chunk_index(filename: &str) -> Result<usize, ArchiveError> {
    if !filename.ends_with(".bin") || filename.len() != 9 {
        return Err(ArchiveError::SchemaMismatch(format!(
            "Invalid chunk filename format: {}",
            filename
        )));
    }

    let index_str = &filename[0..5];
    index_str.parse::<usize>().map_err(|_| {
        ArchiveError::SchemaMismatch(format!("Invalid chunk index in filename: {}", filename))
    })
}

/// Get the expected archive directory name for a given ID
pub fn archive_dir_name(id: &str) -> String {
    format!("clip-{}.trst", id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::{CamVideoManifest, SegmentInfo};
    use tempfile::TempDir;

    fn create_test_manifest() -> CamVideoManifest {
        let mut manifest = CamVideoManifest::new();
        manifest.device.id = "TEST001".to_string();
        manifest.device.public_key = "ed25519:test_key".to_string();
        manifest.capture.started_at = "2025-01-15T10:30:00Z".to_string();
        manifest.capture.ended_at = "2025-01-15T10:30:06Z".to_string();

        // Add test segments
        manifest.segments = vec![
            SegmentInfo {
                chunk_file: "00000.bin".to_string(),
                blake3_hash: hex::encode(crate::chain::segment_hash(b"test_chunk_0")),
                start_time: "2025-01-15T10:30:00Z".to_string(),
                duration_seconds: 2.0,
                continuity_hash: "placeholder".to_string(),
            },
            SegmentInfo {
                chunk_file: "00001.bin".to_string(),
                blake3_hash: hex::encode(crate::chain::segment_hash(b"test_chunk_1")),
                start_time: "2025-01-15T10:30:02Z".to_string(),
                duration_seconds: 2.0,
                continuity_hash: "placeholder".to_string(),
            },
            SegmentInfo {
                chunk_file: "00002.bin".to_string(),
                blake3_hash: hex::encode(crate::chain::segment_hash(b"test_chunk_2")),
                start_time: "2025-01-15T10:30:04Z".to_string(),
                duration_seconds: 2.0,
                continuity_hash: "placeholder".to_string(),
            },
        ];

        // Compute proper continuity chain
        let genesis = crate::chain::genesis();
        let hash0 = crate::chain::segment_hash(b"test_chunk_0");
        let hash1 = crate::chain::segment_hash(b"test_chunk_1");
        let hash2 = crate::chain::segment_hash(b"test_chunk_2");

        let continuity0 = crate::chain::chain_next(&genesis, &hash0);
        let continuity1 = crate::chain::chain_next(&continuity0, &hash1);
        let continuity2 = crate::chain::chain_next(&continuity1, &hash2);

        manifest.segments[0].continuity_hash = hex::encode(continuity0);
        manifest.segments[1].continuity_hash = hex::encode(continuity1);
        manifest.segments[2].continuity_hash = hex::encode(continuity2);

        manifest.signature = Some("ed25519:test_signature".to_string());

        manifest
    }

    #[test]
    fn test_write_and_read_archive_round_trip() {
        let temp_dir = TempDir::new().unwrap();
        let archive_path = temp_dir.path().join("test.trst");

        let manifest = create_test_manifest();
        let chunk_data = vec![
            b"test_chunk_0".to_vec(),
            b"test_chunk_1".to_vec(),
            b"test_chunk_2".to_vec(),
        ];
        let detached_sig = b"ed25519:test_signature";

        // Write archive
        write_archive(&archive_path, &manifest, chunk_data.clone(), detached_sig).unwrap();

        // Verify directory structure exists
        assert!(archive_path.join("manifest.json").exists());
        assert!(archive_path.join("signatures/manifest.sig").exists());
        assert!(archive_path.join("chunks/00000.bin").exists());
        assert!(archive_path.join("chunks/00001.bin").exists());
        assert!(archive_path.join("chunks/00002.bin").exists());

        // Read archive back
        let (read_manifest, read_chunks) = read_archive(&archive_path).unwrap();

        // Verify manifest matches
        assert_eq!(read_manifest.device.id, manifest.device.id);
        assert_eq!(read_manifest.segments.len(), manifest.segments.len());

        // Verify chunks match
        assert_eq!(read_chunks.len(), 3);
        for (i, (index, chunk_bytes)) in read_chunks.iter().enumerate() {
            assert_eq!(*index, i);
            assert_eq!(*chunk_bytes, chunk_data[i]);
        }
    }

    #[test]
    fn test_archive_validation() {
        let temp_dir = TempDir::new().unwrap();
        let archive_path = temp_dir.path().join("test.trst");

        let manifest = create_test_manifest();
        let chunk_data = vec![
            b"test_chunk_0".to_vec(),
            b"test_chunk_1".to_vec(),
            b"test_chunk_2".to_vec(),
        ];
        let detached_sig = b"ed25519:test_signature";

        // Write archive
        write_archive(&archive_path, &manifest, chunk_data, detached_sig).unwrap();

        // Validate should pass
        validate_archive(&archive_path).unwrap();
    }

    #[test]
    fn test_mutation_missing_chunk_causes_validation_failure() {
        let temp_dir = TempDir::new().unwrap();
        let archive_path = temp_dir.path().join("test.trst");

        let manifest = create_test_manifest();
        let chunk_data = vec![
            b"test_chunk_0".to_vec(),
            b"test_chunk_1".to_vec(),
            b"test_chunk_2".to_vec(),
        ];
        let detached_sig = b"ed25519:test_signature";

        // Write archive
        write_archive(&archive_path, &manifest, chunk_data, detached_sig).unwrap();

        // Delete chunks/00002.bin
        let chunk_to_delete = archive_path.join("chunks/00002.bin");
        fs::remove_file(chunk_to_delete).unwrap();

        // Validation should fail
        let result = validate_archive(&archive_path);
        assert!(result.is_err());
        match result.unwrap_err() {
            ArchiveError::MissingChunk(filename) => {
                assert_eq!(filename, "00002.bin");
            }
            other => panic!("Expected MissingChunk error, got {:?}", other),
        }
    }

    #[test]
    fn test_schema_mismatch_chunk_count() {
        let temp_dir = TempDir::new().unwrap();
        let archive_path = temp_dir.path().join("test.trst");

        let manifest = create_test_manifest();
        let wrong_chunk_data = vec![
            b"test_chunk_0".to_vec(),
            b"test_chunk_1".to_vec(),
            // Missing chunk 2
        ];
        let detached_sig = b"ed25519:test_signature";

        // Should fail to write
        let result = write_archive(&archive_path, &manifest, wrong_chunk_data, detached_sig);
        assert!(result.is_err());
        match result.unwrap_err() {
            ArchiveError::SchemaMismatch(msg) => {
                assert!(msg.contains("Chunk count mismatch"));
            }
            other => panic!("Expected SchemaMismatch error, got {:?}", other),
        }
    }

    #[test]
    fn test_signature_mismatch() {
        let temp_dir = TempDir::new().unwrap();
        let archive_path = temp_dir.path().join("test.trst");

        let mut manifest = create_test_manifest();
        manifest.signature = Some("ed25519:different_signature".to_string());
        let chunk_data = vec![
            b"test_chunk_0".to_vec(),
            b"test_chunk_1".to_vec(),
            b"test_chunk_2".to_vec(),
        ];
        let detached_sig = b"ed25519:test_signature"; // Different from manifest

        // Write archive
        write_archive(&archive_path, &manifest, chunk_data, detached_sig).unwrap();

        // Read should fail due to signature mismatch
        let result = read_archive(&archive_path);
        assert!(result.is_err());
        match result.unwrap_err() {
            ArchiveError::SignatureMismatch => (),
            other => panic!("Expected SignatureMismatch error, got {:?}", other),
        }
    }

    #[test]
    fn test_parse_chunk_index() {
        assert_eq!(parse_chunk_index("00000.bin").unwrap(), 0);
        assert_eq!(parse_chunk_index("00042.bin").unwrap(), 42);
        assert_eq!(parse_chunk_index("99999.bin").unwrap(), 99999);

        // Invalid formats should fail
        assert!(parse_chunk_index("0.bin").is_err());
        assert!(parse_chunk_index("chunk.bin").is_err());
        assert!(parse_chunk_index("00000.txt").is_err());
    }

    #[test]
    fn test_archive_dir_name() {
        assert_eq!(archive_dir_name("test123"), "clip-test123.trst");
        assert_eq!(archive_dir_name("CAM-001"), "clip-CAM-001.trst");
    }
}
