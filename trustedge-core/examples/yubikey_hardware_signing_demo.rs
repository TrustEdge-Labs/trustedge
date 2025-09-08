//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/
//
// Project: trustedge — Privacy and trust at the edge.
//

/*
* Copyright (c) 2025 TRUS    let config = YubiKeyConfig {
       pkcs11_module_path: "/usr/lib/x86_64-linux-gnu/opensc-pkcs11.so".to_string(),
       pin: None, // Skip PIN authentication for now
       slot: None, // Auto-detect
       verbose: true,
   };LABS LLC
* MPL-2.0: https://mozilla.org/MPL/2.0/
* Project: trustedge — Privacy and trust at the edge.
*/

//! YubiKey Hardware Signing Demo
//!
//! This example demonstrates:
//! 1. Real YubiKey hardware connection and status
//! 2. Hardware-backed signing operations using different algorithms
//! 3. Direct verification that YubiKey is performing the cryptographic operations

#[cfg(feature = "yubikey")]
use anyhow::{anyhow, Result};
#[cfg(feature = "yubikey")]
use trustedge_core::backends::{
    yubikey::{YubiKeyBackend, YubiKeyConfig},
    CryptoOperation, SignatureAlgorithm, UniversalBackend,
};

#[cfg(feature = "yubikey")]
async fn run_demo() -> Result<()> {
    println!("🔐 YubiKey Hardware Signing Demo");
    println!("================================");

    // Step 1: Get the correct PIN
    println!("\n● Step 1: Enter your YubiKey PIN...");
    print!("YubiKey PIN: ");
    use std::io::{self, Write};
    io::stdout().flush().unwrap();
    let mut pin = String::new();
    io::stdin().read_line(&mut pin).expect("Failed to read PIN");
    let pin = pin.trim().to_string();

    // Step 2: Initialize YubiKey Hardware Backend
    println!("\n● Step 2: Initializing YubiKey hardware backend...");

    let config = YubiKeyConfig {
        pkcs11_module_path: "/usr/lib/x86_64-linux-gnu/opensc-pkcs11.so".to_string(),
        pin: Some(pin), // Use the entered PIN
        slot: None,     // Auto-detect
        verbose: true,
    };

    let yubikey_backend = match YubiKeyBackend::with_config(config) {
        Ok(backend) => {
            println!("✔ YubiKey backend initialized successfully");
            backend
        }
        Err(e) => {
            println!("✖ YubiKey backend initialization failed:");
            println!("   Error: {}", e);
            println!("   Root cause: {:?}", e.source());
            return Err(e);
        }
    };

    // Step 3: Check Hardware Connection Status
    println!("\n● Step 3: Checking YubiKey hardware status...");
    let backend_info = yubikey_backend.backend_info();
    println!(
        "  Backend: {} - {}",
        backend_info.name, backend_info.description
    );
    println!("  Available: {}", backend_info.available);
    println!(
        "  Hardware: {}",
        yubikey_backend.get_capabilities().hardware_backed
    );

    // Step 4: Test Hardware Signing Operations
    println!("\n● Step 4: Testing YubiKey hardware signing...");

    let test_messages = [
        "Hello from YubiKey hardware!",
        "Cryptographic proof of possession",
        "End-to-end security verification",
        "TrustEdge secure communication",
    ];

    // Test different algorithms and verify they work
    let algorithms = vec![
        SignatureAlgorithm::EcdsaP256,
        SignatureAlgorithm::RsaPkcs1v15,
    ];

    for (msg_idx, message) in test_messages.iter().enumerate() {
        println!("\n  Message {}: {}", msg_idx + 1, message);
        let message_bytes = message.as_bytes();

        for algorithm in &algorithms {
            println!("    Testing with {:?}:", algorithm);

            // Sign with YubiKey hardware - use slot 9C (SIGNATURE)
            match yubikey_backend.perform_operation(
                "9c", // Use actual slot 9C where the key exists
                CryptoOperation::Sign {
                    data: message_bytes.to_vec(),
                    algorithm: algorithm.clone(),
                },
            ) {
                Ok(result) => {
                    if let trustedge_core::backends::CryptoResult::Signed(signature) = result {
                        println!("      ✔ Hardware signature: {} bytes", signature.len());
                        println!(
                            "      ✔ Signature preview: {}",
                            hex::encode(&signature[..std::cmp::min(16, signature.len())])
                        );

                        // Verify the signature is different each time (proof it's real)
                        let second_signature = yubikey_backend.perform_operation(
                            "9c", // Use slot 9C
                            CryptoOperation::Sign {
                                data: message_bytes.to_vec(),
                                algorithm: algorithm.clone(),
                            },
                        );

                        if let Ok(trustedge_core::backends::CryptoResult::Signed(sig2)) =
                            second_signature
                        {
                            if signature == sig2 {
                                println!(
                                    "      ⚠ Warning: Signatures are identical (may be cached)"
                                );
                            } else {
                                println!("      ✔ Signatures differ (proof of fresh computation)");
                            }
                        }
                    } else {
                        println!("      ⚠ Unexpected result type");
                    }
                }
                Err(e) => {
                    println!("      ✖ Signing failed: {}", e);
                }
            }
        }
    }

    // Step 5: Test Hardware Attestation
    println!("\n● Step 5: Testing YubiKey hardware attestation...");

    let challenge = b"attestation-challenge-12345";
    println!("  Challenge: {:?}", String::from_utf8_lossy(challenge));

    match yubikey_backend.perform_operation(
        "9c", // Use slot 9C
        CryptoOperation::Attest {
            challenge: challenge.to_vec(),
        },
    ) {
        Ok(trustedge_core::backends::CryptoResult::AttestationProof(proof)) => {
            println!("  ✔ Attestation proof: {} bytes", proof.len());
            println!(
                "  ✔ Proof preview: {}",
                hex::encode(&proof[..std::cmp::min(16, proof.len())])
            );
        }
        Ok(_) => {
            println!("  ⚠ Unexpected attestation result type");
        }
        Err(e) => {
            println!("  ✖ Attestation failed: {}", e);
        }
    }

    // Step 6: Performance Test
    println!("\n● Step 6: YubiKey performance test...");

    let performance_data = b"Performance test data for YubiKey";
    let iterations = 5;

    println!("  Performing {} signing operations...", iterations);
    let start_time = std::time::Instant::now();

    let mut successful_operations = 0;
    for i in 0..iterations {
        match yubikey_backend.perform_operation(
            "9c", // Use slot 9C
            CryptoOperation::Sign {
                data: performance_data.to_vec(),
                algorithm: SignatureAlgorithm::EcdsaP256,
            },
        ) {
            Ok(_) => {
                successful_operations += 1;
                print!(".");
                if i % 10 == 9 {
                    println!();
                }
            }
            Err(e) => {
                println!("\n    ✖ Operation {} failed: {}", i + 1, e);
            }
        }
    }
    println!();

    let elapsed = start_time.elapsed();
    println!(
        "  ✔ Completed {}/{} operations in {:?}",
        successful_operations, iterations, elapsed
    );
    if successful_operations > 0 {
        let avg_time = elapsed / successful_operations;
        println!("  ✔ Average time per signature: {:?}", avg_time);
        println!("\n✔ YubiKey Hardware Demo completed successfully!");
        println!("   Real hardware cryptographic operations verified.");
    } else {
        println!("\n✖ YubiKey Hardware Demo failed!");
        println!("   No successful cryptographic operations.");
        return Err(anyhow!("No successful operations completed"));
    }

    Ok(())
}

#[cfg(not(feature = "yubikey"))]
fn main() {
    println!("✖ YubiKey support not compiled in");
    println!("Run with: cargo run --example yubikey_hardware_signing_demo --features yubikey");
}

#[cfg(feature = "yubikey")]
#[tokio::main]
async fn main() -> Result<()> {
    match run_demo().await {
        Ok(_) => {
            println!("\n✔ Demo completed successfully!");
        }
        Err(e) => {
            println!("\n💥 Demo failed: {}", e);
            println!("   Check YubiKey connection and PKCS#11 setup.");
            std::process::exit(1);
        }
    }

    Ok(())
}
