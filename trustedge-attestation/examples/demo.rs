//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/
//
// Project: trustedge — Privacy and trust at the edge.
//


use anyhow::Result;
use trustedge_attestation::Attestation;

fn main() -> Result<()> {
    println!("🔐 TrustEdge Attestation Demo - Software Birth Certificates");
    println!("===========================================================");

    // Create a simple test file
    let test_content = b"This is a software artifact that needs attestation.";
    std::fs::write("demo-artifact.bin", test_content)?;

    // Create an attestation for the file
    let attestation = Attestation::from_file(
        "demo-artifact.bin",
        "abc123def456".to_string(),      // Git commit hash
        "demo-builder-v1.0".to_string(), // Builder ID
    )?;

    println!("✅ Created attestation for artifact:");
    println!("   📁 File: {}", attestation.artifact_name);
    println!("   🔑 Hash: {}", attestation.artifact_hash);
    println!("   📍 Commit: {}", attestation.source_commit_hash);
    println!("   👤 Builder: {}", attestation.builder_id);
    println!("   ⏰ Time: {}", attestation.timestamp);

    // Verify the attestation
    let is_valid = attestation.verify_file("demo-artifact.bin")?;
    println!(
        "\n🔍 Verification result: {}",
        if is_valid { "✅ VALID" } else { "❌ INVALID" }
    );

    // Demonstrate tamper detection
    println!("\n🔧 Demonstrating tamper detection...");
    std::fs::write("demo-artifact.bin", b"TAMPERED CONTENT")?;

    let is_valid_after_tampering = attestation.verify_file("demo-artifact.bin")?;
    println!(
        "🔍 Verification after tampering: {}",
        if is_valid_after_tampering {
            "✅ VALID"
        } else {
            "❌ INVALID (as expected)"
        }
    );

    // Serialize to JSON for storage/transmission
    let json = serde_json::to_string_pretty(&attestation)?;
    std::fs::write("attestation.json", &json)?;
    println!("\n💾 Attestation saved to attestation.json");

    // Clean up
    std::fs::remove_file("demo-artifact.bin").ok();
    std::fs::remove_file("attestation.json").ok();

    println!("\n🎉 Demo complete! Software birth certificate functionality working.");

    Ok(())
}
