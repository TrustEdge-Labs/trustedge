//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//

use crate::{ArchiveError, Manifest, VerifyOutcome};
use std::path::Path;

pub fn verify_archive_impl<P: AsRef<Path>>(
    archive_path: P,
    device_pub: &str,
) -> Result<VerifyOutcome, ArchiveError> {
    let archive_path = archive_path.as_ref();

    // Read manifest.json from archive
    let manifest_path = archive_path.join("manifest.json");
    let manifest_bytes = std::fs::read(&manifest_path)
        .map_err(|e| ArchiveError::Format(format!("Cannot read manifest: {}", e)))?;

    // Parse manifest
    let manifest: Manifest = serde_json::from_slice(&manifest_bytes).map_err(ArchiveError::Json)?;

    // Verify signature first
    verify_signature(&manifest, device_pub)?;

    // Verify all chunks exist and have correct hashes
    verify_chunks(archive_path, &manifest)?;

    // Verify continuity and duration sanity
    verify_continuity(&manifest)?;
    verify_duration_sanity(&manifest)?;

    // Calculate duration
    let duration = manifest.segments.iter().map(|s| s.t1 - s.t0).sum::<f64>();

    Ok(VerifyOutcome {
        signature: true,
        continuity: true,
        segment_count: manifest.segments.len(),
        duration_seconds: duration,
    })
}

pub fn verify_manifest_bytes_impl(
    manifest_bytes: &[u8],
    device_pub: &str,
) -> Result<VerifyOutcome, ArchiveError> {
    // Parse manifest
    let manifest: Manifest = serde_json::from_slice(manifest_bytes).map_err(ArchiveError::Json)?;

    // Verify signature only (no chunk verification since we don't have access to files)
    verify_signature(&manifest, device_pub)?;

    // Basic continuity and duration checks that don't require files
    verify_continuity(&manifest)?;
    verify_duration_sanity(&manifest)?;

    // Calculate duration
    let duration = manifest.segments.iter().map(|s| s.t1 - s.t0).sum::<f64>();

    Ok(VerifyOutcome {
        signature: true,
        continuity: true,
        segment_count: manifest.segments.len(),
        duration_seconds: duration,
    })
}

fn verify_signature(manifest: &Manifest, device_pub: &str) -> Result<(), ArchiveError> {
    use base64::Engine;
    use ed25519_dalek::{Signature, VerifyingKey};

    // Parse device public key
    let device_pub = device_pub.trim();
    let pub_key_bytes = if let Some(b64_part) = device_pub.strip_prefix("ed25519:") {
        base64::engine::general_purpose::STANDARD
            .decode(b64_part)
            .map_err(|e| ArchiveError::Signature(format!("Invalid public key: {}", e)))?
    } else {
        return Err(ArchiveError::Signature(
            "Public key must start with 'ed25519:'".to_string(),
        ));
    };

    let verifying_key = VerifyingKey::from_bytes(
        &pub_key_bytes
            .try_into()
            .map_err(|_| ArchiveError::Signature("Invalid public key length".to_string()))?,
    )
    .map_err(|e| ArchiveError::Signature(format!("Invalid public key: {}", e)))?;

    // Get signature from manifest
    let signature_str = manifest
        .signature
        .as_ref()
        .ok_or_else(|| ArchiveError::Signature("No signature in manifest".to_string()))?;

    let signature_bytes = if let Some(b64_part) = signature_str.strip_prefix("ed25519:") {
        base64::engine::general_purpose::STANDARD
            .decode(b64_part)
            .map_err(|e| ArchiveError::Signature(format!("Invalid signature: {}", e)))?
    } else {
        return Err(ArchiveError::Signature(
            "Signature must start with 'ed25519:'".to_string(),
        ));
    };

    let signature = Signature::from_bytes(
        &signature_bytes
            .try_into()
            .map_err(|_| ArchiveError::Signature("Invalid signature length".to_string()))?,
    );

    // Get canonical bytes for verification (without signature)
    let canonical_bytes = manifest
        .to_canonical_bytes(false)
        .map_err(|e| ArchiveError::Format(format!("Cannot create canonical bytes: {}", e)))?;

    // Verify signature
    use ed25519_dalek::Verifier;
    verifying_key
        .verify(&canonical_bytes, &signature)
        .map_err(|_| ArchiveError::Signature("signature error".to_string()))?;

    Ok(())
}

fn verify_chunks<P: AsRef<Path>>(archive_path: P, manifest: &Manifest) -> Result<(), ArchiveError> {
    let archive_path = archive_path.as_ref();
    let chunks_dir = archive_path.join("chunks");

    for (i, segment) in manifest.segments.iter().enumerate() {
        // Check if chunk file exists
        let chunk_filename = format!("{:05}.bin", segment.id);
        let chunk_path = chunks_dir.join(&chunk_filename);

        let chunk_data = std::fs::read(&chunk_path).map_err(|_| {
            // If this is the last segment, it's a truncated chain
            if i == manifest.segments.len() - 1 {
                ArchiveError::UnexpectedEnd(format!(
                    "unexpected end: missing final chunk {}",
                    chunk_filename
                ))
            } else {
                ArchiveError::MissingChunk(format!("missing chunk: {}", chunk_filename))
            }
        })?;

        // Verify chunk hash
        let actual_hash = blake3::hash(&chunk_data);
        let actual_hash_hex = hex::encode(actual_hash.as_bytes());

        if actual_hash_hex != segment.hash {
            return Err(ArchiveError::HashMismatch(format!(
                "hash mismatch in chunk {}",
                chunk_filename
            )));
        }

        // Verify chunk size
        if chunk_data.len() as u64 != segment.bytes {
            return Err(ArchiveError::Format(format!(
                "unexpected end: chunk {} size mismatch",
                chunk_filename
            )));
        }
    }

    Ok(())
}

fn verify_continuity(manifest: &Manifest) -> Result<(), ArchiveError> {
    // Check that segments are sequential
    for (i, segment) in manifest.segments.iter().enumerate() {
        if segment.id != i as u32 {
            return Err(ArchiveError::Continuity(format!(
                "unexpected end: segment {} out of order",
                segment.id
            )));
        }
    }

    // Check temporal continuity
    for window in manifest.segments.windows(2) {
        let current = &window[0];
        let next = &window[1];

        if current.t1 != next.t0 {
            return Err(ArchiveError::Continuity(format!(
                "unexpected end: temporal gap between segments {} and {}",
                current.id, next.id
            )));
        }
    }

    Ok(())
}

fn verify_duration_sanity(manifest: &Manifest) -> Result<(), ArchiveError> {
    // Check for unreasonably long segment durations (> 3.0 seconds per segment is suspicious)
    for segment in &manifest.segments {
        let duration = segment.t1 - segment.t0;
        if duration > 3.0 {
            return Err(ArchiveError::UnexpectedEnd(format!(
                "unexpected end: segment {} duration {} exceeds sanity limit",
                segment.id, duration
            )));
        }
    }

    Ok(())
}
