//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/
//
// Project: trustedge — Privacy and trust at the edge.
//

use anyhow::Result;
use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use std::fs;
use tempfile::TempDir;
use trustedge_attestation::{create_attestation, Attestation};

fn main() -> Result<()> {
    println!("🔐 TrustEdge Step 3 Demo - Create Attestation Function");
    println!("=====================================================");

    // Create a temporary directory and test artifact
    let temp_dir = TempDir::new()?;
    let artifact_path = temp_dir.path().join("software-artifact.bin");

    let test_software =
        b"This is a software binary that needs attestation and provenance tracking.";
    fs::write(&artifact_path, test_software)?;

    println!("📁 Created test artifact: {}", artifact_path.display());

    // Generate signing keys (in real use, these would be from YubiKey/HSM)
    let signing_key = SigningKey::generate(&mut OsRng);
    let beneficiary_key = signing_key.verifying_key(); // Self-sign for demo

    println!("🔑 Generated signing keys");

    // Call the main create_attestation function from Step 3
    match create_attestation(
        &artifact_path,
        "demo-builder-ci-job-123",
        &signing_key,
        &beneficiary_key,
    ) {
        Ok(envelope) => {
            println!("✅ Successfully created attested software birth certificate!");
            println!("🔒 Attestation sealed in cryptographic envelope");

            // Demonstrate that we can verify the envelope
            if envelope.verify() {
                println!("✅ Envelope cryptographic verification: PASSED");
            } else {
                println!("❌ Envelope cryptographic verification: FAILED");
            }

            // Try to unseal and inspect the attestation
            match envelope.unseal(&signing_key) {
                Ok(payload) => {
                    println!("🔓 Successfully unsealed envelope");

                    // Deserialize the attestation
                    if let Ok(attestation) = serde_json::from_slice::<Attestation>(&payload) {
                        println!("\n📋 Software Birth Certificate Details:");
                        println!("   📁 Artifact: {}", attestation.artifact_name);
                        println!("   🔑 SHA-256: {}", attestation.artifact_hash);
                        println!("   📍 Git Commit: {}", attestation.source_commit_hash);
                        println!("   👤 Builder: {}", attestation.builder_id);
                        println!("   ⏰ Timestamp: {}", attestation.timestamp);

                        // Verify the hash matches our artifact
                        if attestation.verify_file(artifact_path.to_str().unwrap())? {
                            println!("✅ Artifact integrity verification: PASSED");
                        } else {
                            println!("❌ Artifact integrity verification: FAILED");
                        }
                    }
                }
                Err(e) => {
                    println!("❌ Failed to unseal envelope: {}", e);
                }
            }
        }
        Err(e) => {
            println!("❌ Failed to create attestation: {}", e);

            // If Git is not available, show what the error looks like
            if e.to_string().contains("Git") {
                println!("\n💡 Note: This demo requires a Git repository.");
                println!("   In CI/CD, ensure the workspace has .git directory.");
                println!("   For production use, attestation captures full Git provenance.");
            }

            return Ok(()); // Don't fail the demo entirely
        }
    }

    println!("\n🎉 Step 3 implementation complete!");
    println!("   ✅ Artifact hashing");
    println!("   ✅ Git commit capture");
    println!("   ✅ Attestation payload creation");
    println!("   ✅ Envelope sealing");
    println!("\n🚀 Ready for hardware-backed signing with YubiKey/HSM!");

    Ok(())
}
