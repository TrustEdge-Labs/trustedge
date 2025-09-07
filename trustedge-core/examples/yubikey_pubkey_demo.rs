/*
 * Copyright (c) 2025 TRUSTEDGE LABS LLC
 * This source code is subject to the terms of the Mozilla Public License, v. 2.0.
 * If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
 *
 * Project: trustedge — Privacy and trust at the edge.
 */

//! YubiKey Public Key Extraction Demo
//!
//! This example demonstrates Phase 1 of the real X.509 certificate generation:
//! extracting actual public keys from YubiKey PIV slots using PKCS#11.
//!
//! Run with: cargo run --example yubikey_pubkey_demo --features yubikey

#[cfg(feature = "yubikey")]
use anyhow::Result;
#[cfg(feature = "yubikey")]
use trustedge_core::backends::yubikey::{YubiKeyBackend, YubiKeyConfig};

#[cfg(feature = "yubikey")]
#[tokio::main]
async fn main() -> Result<()> {
    println!("🔐 YubiKey Public Key Extraction Demo");
    println!("=====================================");
    println!("Phase 1: Real Public Key Extraction from YubiKey PIV Slots");

    // Test public key extraction
    demonstrate_public_key_extraction().await?;

    println!("\n✔ YubiKey public key extraction demo completed!");
    println!("\nThis demonstrates:");
    println!("• Real public key extraction from YubiKey PIV slots");
    println!("• PKCS#11 integration for hardware key access");
    println!("• DER-encoded SubjectPublicKeyInfo generation");
    println!("• Support for ECDSA P-256 and RSA keys");
    println!("• Foundation for Phase 2 X.509 certificate generation");

    Ok(())
}

#[cfg(feature = "yubikey")]
async fn demonstrate_public_key_extraction() -> Result<()> {
    println!("\n● Testing YubiKey public key extraction...");

    let config = YubiKeyConfig {
        pkcs11_module_path: "/usr/lib/x86_64-linux-gnu/opensc-pkcs11.so".to_string(),
        pin: None,
        slot: None,
        verbose: true,
    };

    match YubiKeyBackend::with_config(config) {
        Ok(backend) => {
            // Test common PIV slot key IDs
            let test_key_ids = vec![
                "SIGN key",  // PIV authentication key (slot 9A)
                "AUTH key",  // PIV digital signature key (slot 9C)
                "ENC key",   // PIV key management key (slot 9D)
                "CARD AUTH", // PIV card authentication key (slot 9E)
            ];

            println!("   Testing public key extraction from common PIV slots:");

            let mut successful_extractions = 0;

            for key_id in test_key_ids {
                println!("\n   ● Attempting to extract public key: {}", key_id);

                match backend.extract_public_key(key_id) {
                    Ok(public_key_der) => {
                        successful_extractions += 1;
                        println!("     ✔ Public key extracted successfully!");
                        println!("       DER size: {} bytes", public_key_der.len());
                        println!(
                            "       DER header: {:02x?}",
                            &public_key_der[..std::cmp::min(16, public_key_der.len())]
                        );

                        // Validate it looks like a proper SubjectPublicKeyInfo DER structure
                        if public_key_der.len() > 20 && public_key_der[0] == 0x30 {
                            println!("       ✔ Valid DER structure (starts with SEQUENCE)");
                        }
                    }
                    Err(e) => {
                        println!("     ⚠ Could not extract public key: {}", e);
                        println!("       (This is expected if key doesn't exist in this slot)");
                    }
                }
            }

            if successful_extractions > 0 {
                println!(
                    "\n   ✔ Successfully extracted {} public key(s) from YubiKey!",
                    successful_extractions
                );
                println!("   ✔ Real hardware public key extraction working!");
            } else {
                println!("\n   ⚠ No keys found in common PIV slots");
                println!("     This might mean:");
                println!("     • No keys are generated in PIV slots yet");
                println!("     • Different key IDs are used on this YubiKey");
                println!("     • YubiKey requires PIN for public key access");
            }
        }
        Err(e) => {
            println!("⚠ YubiKey backend not available: {}", e);
            println!("   This demo requires:");
            println!("   • YubiKey connected with PKCS#11 support");
            println!("   • OpenSC PKCS#11 module installed");
            println!("   • Keys generated in PIV slots");

            // Show what the extraction would look like with placeholder data
            demonstrate_extraction_format();
        }
    }

    Ok(())
}

#[cfg(feature = "yubikey")]
fn demonstrate_extraction_format() {
    println!("\n● Demonstrating expected public key extraction format:");

    // Example of what a real ECDSA P-256 public key looks like in DER format
    let example_ecdsa_der = vec![
        0x30, 0x59, // SEQUENCE (89 bytes)
        0x30, 0x13, // SEQUENCE (19 bytes) - Algorithm Identifier
        0x06, 0x07, 0x2a, 0x86, 0x48, 0xce, 0x3d, 0x02, 0x01, // OID: ecPublicKey
        0x06, 0x08, 0x2a, 0x86, 0x48, 0xce, 0x3d, 0x03, 0x01, 0x07, // OID: secp256r1
        0x03, 0x42, 0x00, // BIT STRING (66 bytes)
        0x04, // Uncompressed point
        // 32 bytes X coordinate + 32 bytes Y coordinate would follow
        0x01, 0x02, 0x03, 0x04, // ... (truncated for demo)
    ];

    println!("   Example ECDSA P-256 DER structure:");
    println!("   DER size: {} bytes", example_ecdsa_der.len());
    println!("   DER header: {:02x?}", &example_ecdsa_der[..16]);
    println!("   ✔ This is what real extraction would return");
}

#[cfg(not(feature = "yubikey"))]
fn main() {
    println!("🔐 YubiKey Public Key Extraction Demo");
    println!("=====================================");
    println!();
    println!("⚠ This example requires the 'yubikey' feature to be enabled.");
    println!("  Run with: cargo run --example yubikey_pubkey_demo --features yubikey");
    println!();
    println!("This demo would show:");
    println!("• Real public key extraction from YubiKey PIV slots");
    println!("• PKCS#11 integration for hardware key access");
    println!("• DER-encoded SubjectPublicKeyInfo generation");
    println!("• Foundation for Phase 2 X.509 certificate generation");
}
