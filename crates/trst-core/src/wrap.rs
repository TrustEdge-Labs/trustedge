//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//

use crate::{ArchiveError, Manifest, ManifestChunk, Segment, WrapConfig, WrapResult};
use std::fs;
use std::path::Path;

pub fn wrap_file_impl<P: AsRef<Path>>(
    input_path: P,
    output_dir: P,
    signing_key: &ed25519_dalek::SigningKey,
    config: WrapConfig,
) -> Result<WrapResult, ArchiveError> {
    let input_path = input_path.as_ref();
    let output_dir = output_dir.as_ref();

    // Read input file
    let input_data = fs::read(input_path)?;

    // Create output directory structure
    fs::create_dir_all(output_dir)?;
    fs::create_dir_all(output_dir.join("chunks"))?;
    fs::create_dir_all(output_dir.join("signatures"))?;

    // Create chunks
    let mut segments = Vec::new();
    let chunk_size = config.chunk_bytes;
    let mut t = 0.0f64;

    for (chunk_id, chunk_data) in input_data.chunks(chunk_size).enumerate() {
        let chunk_id = chunk_id as u32;
        // Create chunk hash
        let hash = blake3::hash(chunk_data);
        let prev_hash = if chunk_id == 0 {
            [1u8; 32] // Initial hash
        } else {
            [2u8; 32] // Simplified - should be previous chunk hash
        };
        let nonce = [0u8; 24]; // Simplified nonce

        // Write chunk file
        let chunk_filename = format!("{:05}.bin", chunk_id);
        let chunk_path = output_dir.join("chunks").join(&chunk_filename);
        fs::write(&chunk_path, chunk_data)?;

        // Create segment
        let t1 = t + config.chunk_seconds;
        let segment = Segment::new(
            chunk_id,
            t,
            t1,
            hash.as_bytes(),
            &prev_hash,
            chunk_data.len() as u64,
            &nonce,
        );
        segments.push(segment);

        t = t1;
    }

    // Create manifest
    let manifest = Manifest {
        trst_version: "0.1.0".to_string(),
        profile: config.profile.clone(),
        device: config.device.clone(),
        capture: config.capture.clone(),
        chunk: ManifestChunk {
            approx_duration_s: config.chunk_seconds,
            bytes: chunk_size as u64,
        },
        segments: segments.clone(),
        claims: config.claims.clone(),
        prev_archive_hash: config.prev_archive_hash.clone(),
        signature: None, // Will be added after signing
    };

    // Sign the manifest
    use base64::Engine;
    use ed25519_dalek::Signer;
    let canonical_bytes = manifest.to_canonical_bytes(false)?;
    let signature = signing_key.sign(&canonical_bytes);
    let signature_str = format!(
        "ed25519:{}",
        base64::engine::general_purpose::STANDARD.encode(signature.to_bytes())
    );

    // Create final manifest with signature
    let signed_manifest = manifest.with_signature(signature_str.clone());
    let final_manifest_bytes = signed_manifest.to_canonical_bytes(true)?;

    // Write manifest
    fs::write(output_dir.join("manifest.json"), &final_manifest_bytes)?;

    // Write signature file
    fs::write(
        output_dir.join("signatures").join("manifest.sig"),
        signature_str.as_bytes(),
    )?;

    Ok(WrapResult {
        output_dir: output_dir.to_path_buf(),
        signature: signature_str,
        chunk_count: segments.len(),
    })
}
