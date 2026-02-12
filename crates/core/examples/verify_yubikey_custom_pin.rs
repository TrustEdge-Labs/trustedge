//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//

//! YubiKey Verification with Custom PIN
//!
//! Run with: cargo run --example verify_yubikey_custom_pin --features yubikey -- YOUR_PIN_HERE
//! Or for public key operations only (no PIN): cargo run --example verify_yubikey_custom_pin --features yubikey -- no-pin

#[cfg(feature = "yubikey")]
fn main() -> anyhow::Result<()> {
    use trustedge_core::{
        backends::yubikey::{YubiKeyBackend, YubiKeyConfig},
        CryptoOperation, CryptoResult, UniversalBackend,
    };

    println!("🔑 TrustEdge YubiKey Verification (Custom PIN)");
    println!("==============================================\n");

    let args: Vec<String> = std::env::args().collect();

    let pin = if args.len() > 1 {
        if args[1] == "no-pin" {
            println!("⚠ Running without PIN (public key operations only)");
            None
        } else {
            println!("✔ Using provided PIN");
            Some(args[1].clone())
        }
    } else {
        println!("Usage:");
        println!("  With PIN:     cargo run --example verify_yubikey_custom_pin --features yubikey -- YOUR_PIN");
        println!("  Without PIN:  cargo run --example verify_yubikey_custom_pin --features yubikey -- no-pin");
        println!("\nNote: Default YubiKey PIN is 123456");
        println!("      Check remaining tries with: ykman piv info\n");
        return Ok(());
    };

    // Configure YubiKey backend
    let config = YubiKeyConfig {
        pin,
        default_slot: "9c".to_string(),
        verbose: true,
        max_pin_retries: 3,
    };

    println!("\n📋 Configuration:");
    println!(
        "   PIN: {}",
        if config.pin.is_some() {
            "Provided"
        } else {
            "None (public ops only)"
        }
    );
    println!("   Default Slot: {}", config.default_slot);
    println!("   Scanning all PIV slots\n");

    // Initialize YubiKey backend
    println!("● Connecting to YubiKey...");
    let backend = match YubiKeyBackend::with_config(config) {
        Ok(b) => {
            println!("✔ YubiKey backend initialized!\n");
            b
        }
        Err(e) => {
            println!("✖ Failed to initialize YubiKey backend:");
            println!("   Error: {}\n", e);
            println!("Troubleshooting:");
            println!("  1. Check remaining PIN tries: ykman piv info");
            println!("  2. If PIN is locked, reset with: ykman piv access change-pin");
            println!("  3. Try without PIN: cargo run --example verify_yubikey_custom_pin --features yubikey -- no-pin");
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
            Err(e) => {
                println!("○ No key ({:?})", e);
            }
        }
    }

    println!("\n📊 Summary:");
    println!("   Total keys found: {}/{}", found_keys, slots.len());

    if found_keys > 0 {
        println!("\n✅ SUCCESS! Your YubiKey is working with TrustEdge!");
        println!("\nNext steps:");
        println!("   • Try: cargo run --example yubikey_demo --features yubikey");
        println!("   • Try: cargo run --example yubikey_certificate_demo --features yubikey");
    } else {
        println!("\n⚠ No keys found. But ykman shows a key in slot 9c!");
        println!("\nThis might be a certificate-only slot (no private key access).");
        println!("Or the PIN authentication is blocking access.");
    }

    Ok(())
}

#[cfg(not(feature = "yubikey"))]
fn main() {
    println!("❌ This example requires the 'yubikey' feature.");
    println!("\nRun with:");
    println!("   cargo run --example verify_yubikey_custom_pin --features yubikey -- YOUR_PIN");
}
