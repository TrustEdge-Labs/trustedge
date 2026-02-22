//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//

//! Simple YubiKey Verification Test
//!
//! The simplest possible test to verify YubiKey connectivity.
//! Run with: cargo run --example verify_yubikey --features yubikey

#[cfg(feature = "yubikey")]
fn main() -> anyhow::Result<()> {
    use trustedge_core::{
        backends::yubikey::{YubiKeyBackend, YubiKeyConfig},
        CryptoOperation, CryptoResult, UniversalBackend,
    };

    println!("🔑 TrustEdge YubiKey Verification Test");
    println!("=====================================\n");

    // Configure YubiKey backend with default slot
    let config = YubiKeyConfig::builder()
        .pin("123456".to_string()) // Default YubiKey PIN
        .default_slot("9c".to_string()) // Digital Signature slot
        .verbose(true)
        .max_pin_retries(3)
        .build();

    println!("📋 Configuration:");
    println!("   PIN: Provided (using default: 123456)");
    println!("   Default Slot: {}", config.default_slot);
    println!("   Scanning all PIV slots\n");

    // Initialize YubiKey backend
    println!("● Connecting to YubiKey...");
    let backend = match YubiKeyBackend::with_config(config) {
        Ok(b) => {
            println!("✔ YubiKey backend initialized successfully!\n");
            b
        }
        Err(e) => {
            println!("✖ Failed to initialize YubiKey backend:");
            println!("   Error: {}\n", e);
            println!("Troubleshooting:");
            println!("  1. Is your YubiKey plugged in?");
            println!("  2. Run: ykman list");
            println!("  3. Run: ykman piv info");
            println!("  4. Check if PIN is correct (default: 123456)");
            return Ok(());
        }
    };

    // Test all standard PIV slots
    let slots = [
        ("9a", "PIV Authentication"),
        ("9c", "Key Management"),
        ("9d", "Card Authentication"),
        ("9e", "Digital Signature"),
    ];

    println!("🔍 Scanning PIV Slots:");
    let mut found_keys = 0;

    for (slot, name) in &slots {
        print!("   {} ({}): ", slot, name);

        match backend.perform_operation(slot, CryptoOperation::GetPublicKey) {
            Ok(CryptoResult::PublicKey(pubkey)) => {
                println!("✔ Key found! ({} bytes)", pubkey.len());
                found_keys += 1;

                // Show first few bytes of public key
                if pubkey.len() >= 16 {
                    print!("      First 16 bytes: ");
                    for byte in &pubkey[..16] {
                        print!("{:02x}", byte);
                    }
                    println!();
                }
            }
            Ok(_) => {
                println!("✖ Unexpected result type");
            }
            Err(_) => {
                println!("○ No key in this slot");
            }
        }
    }

    println!("\n📊 Summary:");
    println!("   Total keys found: {}/{}", found_keys, slots.len());

    if found_keys == 0 {
        println!("\n⚠ No keys found in any PIV slots!");
        println!("\nTo generate a test key in slot 9c:");
        println!("   ykman piv keys generate 9c /tmp/pubkey.pem --algorithm ECCP256");
        println!("   ykman piv certificates generate 9c /tmp/pubkey.pem --subject \"CN=Test\"");
    } else {
        println!("\n✅ SUCCESS! Your YubiKey is working with TrustEdge!");
        println!("\nNext steps:");
        println!("   • Try: cargo run --example yubikey_demo --features yubikey");
        println!("   • Try: cargo run --example yubikey_certificate_demo --features yubikey");
    }

    Ok(())
}

#[cfg(not(feature = "yubikey"))]
fn main() {
    println!("❌ This example requires the 'yubikey' feature.");
    println!("\nRun with:");
    println!("   cargo run --example verify_yubikey --features yubikey");
}
