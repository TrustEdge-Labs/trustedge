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

    verify_manifest_bytes_impl(&manifest_bytes, device_pub)
}

pub fn verify_manifest_bytes_impl(
    manifest_bytes: &[u8],
    device_pub: &str,
) -> Result<VerifyOutcome, ArchiveError> {
    // Parse manifest
    let manifest: Manifest = serde_json::from_slice(manifest_bytes).map_err(ArchiveError::Json)?;

    // Verify signature
    let signature_valid = verify_signature(&manifest, device_pub)?;

    // Verify continuity (simplified - check that segments are sequential)
    let continuity_valid = verify_continuity(&manifest);

    // Calculate duration
    let duration = manifest.segments.iter().map(|s| s.t1 - s.t0).sum::<f64>();

    Ok(VerifyOutcome {
        signature: signature_valid,
        continuity: continuity_valid,
        segment_count: manifest.segments.len(),
        duration_seconds: duration,
    })
}

fn verify_signature(manifest: &Manifest, device_pub: &str) -> Result<bool, ArchiveError> {
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
    match verifying_key.verify(&canonical_bytes, &signature) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

fn verify_continuity(manifest: &Manifest) -> bool {
    // Simple continuity check - segments should be sequential
    for (i, segment) in manifest.segments.iter().enumerate() {
        if segment.id != i as u32 {
            return false;
        }
    }
    true
}
