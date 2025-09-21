//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/
//
// Project: trustedge — Privacy and trust at the edge.
//

use std::env;
use std::error::Error;
use std::fs;

use trustedge_core::{read_archive, validate_archive, verify_manifest};

fn main() -> Result<(), Box<dyn Error>> {
    println!("TrustEdge P0 cam.video Example: Verify CLI");

    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    let archive_path = args.get(1)
        .unwrap_or(&"examples/cam.video/clip.trst".to_string())
        .clone();
    let key_path = args.get(2)
        .unwrap_or(&"examples/cam.video/device.pub".to_string())
        .clone();

    println!("Archive: {}", archive_path);
    println!("Device key: {}", key_path);

    // Read device public key
    let device_pub = fs::read_to_string(&key_path)
        .map_err(|e| format!("Failed to read device public key from '{}': {}", key_path, e))?;
    let device_pub = device_pub.trim();

    // Ensure device public key has proper format
    let device_pub_key = if device_pub.starts_with("ed25519:") {
        device_pub.to_string()
    } else {
        format!("ed25519:{}", device_pub)
    };

    println!("Device public key: {}", device_pub_key);
    println!();

    // Read and validate archive
    let (manifest, _chunks) = read_archive(&archive_path)
        .map_err(|e| format!("Failed to read archive '{}': {}", archive_path, e))?;

    // Get signature and canonical bytes
    let signature = manifest.signature.as_ref()
        .ok_or("Manifest has no signature")?;

    let canonical_bytes = manifest.to_canonical_bytes()
        .map_err(|e| format!("Failed to canonicalize manifest: {}", e))?;

    // Verify signature
    print!("Signature: ");
    match verify_manifest(&device_pub_key, &canonical_bytes, signature) {
        Ok(true) => {
            println!("✔ PASS");

            // Validate archive structure and continuity
            print!("Continuity: ");
            match validate_archive(&archive_path) {
                Ok(()) => {
                    println!("✔ PASS");

                    // Print summary information
                    let segment_count = manifest.segments.len();
                    let duration_seconds: f64 = manifest.segments.iter()
                        .map(|s| s.duration_seconds)
                        .sum();

                    println!();
                    println!("● Archive Summary:");
                    println!("   Segments: {}", segment_count);
                    println!("   Duration: {:.1}s", duration_seconds);
                    println!("   Chunk size: {:.1}s per segment",
                             if segment_count > 0 { duration_seconds / segment_count as f64 } else { 0.0 });
                    println!("   Profile: {}", manifest.profile);
                    println!("   Device: {} ({})", manifest.device.id, manifest.device.model);
                    println!("   Resolution: {} @ {} fps", manifest.capture.resolution, manifest.capture.fps);
                    println!("   Started: {}", manifest.capture.started_at);
                    println!("   Ended: {}", manifest.capture.ended_at);

                    println!();
                    println!("🎉 Archive verification successful!");
                }
                Err(err) => {
                    println!("❌ FAIL");
                    eprintln!("Archive validation failed: {}", err);
                    std::process::exit(1);
                }
            }
        }
        Ok(false) => {
            println!("❌ FAIL");
            println!("Continuity: ⏭️  SKIP");
            eprintln!("Signature verification failed");
            std::process::exit(1);
        }
        Err(err) => {
            println!("❌ FAIL");
            println!("Continuity: ⏭️  SKIP");
            eprintln!("Signature verification error: {}", err);
            std::process::exit(1);
        }
    }

    Ok(())
}