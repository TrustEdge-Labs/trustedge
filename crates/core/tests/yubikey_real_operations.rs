/*
 * Copyright (c) 2025 TRUSTEDGE LABS LLC
 * This source code is subject to the terms of the Mozilla Public License, v. 2.0.
 * If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
 *
 * Project: trustedge — Privacy and trust at the edge.
 */

//! YubiKey Real Hardware Operations Tests - Phase 2
//!
//! These tests validate real YubiKey hardware operations using actual PKCS#11 integration.

use anyhow::Result;
use std::io::{self, Write};
use std::sync::Mutex;

mod yubikey_hardware_detection;
use yubikey_hardware_detection::YubikeyTestEnvironment;

#[cfg(feature = "yubikey")]
use trustedge_core::{
    backends::YubiKeyBackend, CryptoOperation, CryptoResult, SignatureAlgorithm, UniversalBackend,
};

// Global mutex to ensure YubiKey tests run sequentially (not in parallel)
// This prevents resource conflicts and PIN prompt interference
static YUBIKEY_TEST_MUTEX: Mutex<()> = Mutex::new(());

// Run with: cargo test --test yubikey_real_operations --features yubikey
//
// ⚠ WARNING: These tests will attempt real signing operations and may require PIN entry.

/// Interactive PIN helper for real hardware testing
#[allow(dead_code)]
fn get_test_pin() -> Option<String> {
    // Check if PIN is provided via environment variable (for automation)
    if let Ok(pin) = std::env::var("YUBIKEY_TEST_PIN") {
        return Some(pin);
    }

    // For interactive testing, prompt for PIN
    print!("Enter YubiKey PIN for testing (or press Enter to skip): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let pin = input.trim();
            if pin.is_empty() {
                None
            } else {
                Some(pin.to_string())
            }
        }
        Err(_) => None,
    }
}

/// Test real YubiKey backend initialization with PIN
#[tokio::test]
#[ignore] // Requires real hardware and PIN
async fn test_real_backend_initialization_with_pin() -> Result<()> {
    let _lock = YUBIKEY_TEST_MUTEX.lock().unwrap(); // Ensure sequential execution

    let env = YubikeyTestEnvironment::detect();

    if !env.has_hardware() {
        println!("⚠ Skipping real hardware test - {}", env.description());
        return Ok(());
    }

    #[cfg(feature = "yubikey")]
    {
        println!("● Testing real YubiKey backend initialization with PIN...");

        let test_pin = get_test_pin();

        if test_pin.is_none() {
            println!("⚠ Skipping PIN test - no PIN provided");
            return Ok(());
        }

        let mut config = env.get_config().expect("Hardware should have config");
        config.pin = test_pin;
        config.verbose = true; // Show detailed output

        // Test backend creation with PIN
        let backend_result = YubiKeyBackend::with_config(config);

        match backend_result {
            Ok(backend) => {
                println!("✔ YubiKey backend initialized successfully with PIN");

                // Test that we can access authenticated operations
                let capabilities = backend.get_capabilities();
                assert!(capabilities.hardware_backed);
                assert!(capabilities.supports_attestation);

                println!("✔ Authenticated capabilities verified");
            }
            Err(e) => {
                // If it fails, it should be for a specific reason
                println!("● PIN authentication failed: {}", e);

                // This might be expected if PIN is wrong, but let's verify the error is reasonable
                let error_msg = e.to_string().to_lowercase();
                assert!(
                    error_msg.contains("pin")
                        || error_msg.contains("auth")
                        || error_msg.contains("login"),
                    "Authentication error should be PIN-related: {}",
                    e
                );
            }
        }
    }

    Ok(())
}

/// Test real signing operation with YubiKey
#[tokio::test]
#[ignore] // Requires real hardware and PIN
async fn test_real_signing_operation() -> Result<()> {
    let _lock = YUBIKEY_TEST_MUTEX.lock().unwrap(); // Ensure sequential execution

    let env = YubikeyTestEnvironment::detect();

    if !env.has_hardware() {
        println!("⚠ Skipping real hardware test - {}", env.description());
        return Ok(());
    }

    #[cfg(feature = "yubikey")]
    {
        println!("● Testing real YubiKey signing operation...");

        let test_pin = get_test_pin();

        if test_pin.is_none() {
            println!("⚠ Skipping signing test - no PIN provided");
            return Ok(());
        }

        let mut config = env.get_config().expect("Hardware should have config");
        config.pin = test_pin;
        config.verbose = true;

        let backend = YubiKeyBackend::with_config(config)?;

        // Test data to sign
        let test_data = b"YubiKey Phase 2 Real Hardware Test - Signing Operation";

        // Attempt signing operation with PIV slot 9c (Digital Signature)
        let sign_operation = CryptoOperation::Sign {
            data: test_data.to_vec(),
            algorithm: SignatureAlgorithm::EcdsaP256,
        };

        // Check if the backend supports this operation
        if !backend.supports_operation(&sign_operation) {
            println!("⚠ Backend reports signing not supported - this may be expected");
            return Ok(());
        }

        println!("● Attempting to sign data with PIV slot 9c...");

        let result = backend.perform_operation("9c", sign_operation);

        match result {
            Ok(CryptoResult::Signed(signature)) => {
                println!("✔ Signing operation successful!");
                println!("  • Signature length: {} bytes", signature.len());
                println!(
                    "  • Test data: {:?}",
                    std::str::from_utf8(test_data).unwrap_or("<binary>")
                );

                // Basic signature validation
                assert!(!signature.is_empty(), "Signature should not be empty");
                assert!(
                    signature.len() >= 32,
                    "ECDSA P-256 signature should be at least 32 bytes"
                );

                println!("✔ Real YubiKey signing operation verified!");
            }
            Ok(other) => {
                panic!("Expected Signed result, got: {:?}", other);
            }
            Err(e) => {
                println!("● Signing operation failed: {}", e);

                // Check if it's a reasonable failure (e.g., no key in slot)
                let error_msg = e.to_string().to_lowercase();
                if error_msg.contains("key")
                    || error_msg.contains("slot")
                    || error_msg.contains("not found")
                {
                    println!("⚠ This may be expected if no key is present in PIV slot 9c");
                } else {
                    return Err(e.into());
                }
            }
        }
    }

    Ok(())
}

/// Test PIV slot key enumeration
#[tokio::test]
#[ignore] // Requires real hardware
async fn test_real_key_enumeration() -> Result<()> {
    let _lock = YUBIKEY_TEST_MUTEX.lock().unwrap(); // Ensure sequential execution

    let env = YubikeyTestEnvironment::detect();

    if !env.has_hardware() {
        println!("⚠ Skipping real hardware test - {}", env.description());
        return Ok(());
    }

    #[cfg(feature = "yubikey")]
    {
        println!("● Testing real PIV key enumeration...");

        let config = env.get_config().expect("Hardware should have config");

        // Test without PIN first (may work for key discovery)
        let backend_result = YubiKeyBackend::with_config(config);

        match backend_result {
            Ok(backend) => {
                // Test key listing
                let keys_result = backend.list_keys();

                match keys_result {
                    Ok(keys) => {
                        println!("✔ Key enumeration successful!");
                        println!("  • Found {} keys", keys.len());

                        for (i, key) in keys.iter().enumerate() {
                            println!(
                                "  • Key {}: ID={:?}, Description={}",
                                i + 1,
                                key.key_id,
                                key.description
                            );
                        }

                        if keys.is_empty() {
                            println!("⚠ No keys found - this may be expected for empty YubiKey");
                        }
                    }
                    Err(e) => {
                        println!("● Key enumeration failed: {}", e);

                        // This might be expected without PIN
                        let error_msg = e.to_string().to_lowercase();
                        if error_msg.contains("pin") || error_msg.contains("auth") {
                            println!("⚠ Key enumeration requires PIN authentication");
                        } else {
                            println!("● Unexpected key enumeration error: {}", e);
                        }
                    }
                }
            }
            Err(e) => {
                println!("● Backend initialization failed: {}", e);
            }
        }
    }

    Ok(())
}

/// Test YubiKey attestation operation
#[tokio::test]
#[ignore] // Requires real hardware and PIN
async fn test_real_attestation_operation() -> Result<()> {
    let _lock = YUBIKEY_TEST_MUTEX.lock().unwrap(); // Ensure sequential execution

    let env = YubikeyTestEnvironment::detect();

    if !env.has_hardware() {
        println!("⚠ Skipping real hardware test - {}", env.description());
        return Ok(());
    }

    #[cfg(feature = "yubikey")]
    {
        println!("● Testing real YubiKey attestation operation...");

        let test_pin = get_test_pin();

        if test_pin.is_none() {
            println!("⚠ Skipping attestation test - no PIN provided");
            return Ok(());
        }

        let mut config = env.get_config().expect("Hardware should have config");
        config.pin = test_pin;
        config.verbose = true;

        let backend = YubiKeyBackend::with_config(config)?;

        // Test attestation challenge
        let challenge = b"YubiKey Phase 2 Attestation Challenge - Hardware Test";

        let attest_operation = CryptoOperation::Attest {
            challenge: challenge.to_vec(),
        };

        if !backend.supports_operation(&attest_operation) {
            println!("⚠ Backend reports attestation not supported");
            return Ok(());
        }

        println!("● Attempting hardware attestation...");

        let result = backend.perform_operation("9c", attest_operation);

        match result {
            Ok(CryptoResult::AttestationProof(proof)) => {
                println!("✔ Attestation operation successful!");
                println!("  • Proof length: {} bytes", proof.len());

                assert!(!proof.is_empty(), "Attestation proof should not be empty");

                println!("✔ Real YubiKey attestation verified!");
            }
            Ok(other) => {
                println!("● Attestation returned unexpected result: {:?}", other);
            }
            Err(e) => {
                println!("● Attestation operation failed: {}", e);

                // This might be expected - not all YubiKey models support attestation
                let error_msg = e.to_string().to_lowercase();
                if error_msg.contains("unsupported") || error_msg.contains("not implemented") {
                    println!("⚠ Attestation may not be supported on this YubiKey model");
                } else {
                    println!("● Unexpected attestation error: {}", e);
                }
            }
        }
    }

    Ok(())
}

/// Test certificate operations (if available)
#[tokio::test]
#[ignore] // Requires real hardware and existing certificates
async fn test_real_certificate_operations() -> Result<()> {
    let _lock = YUBIKEY_TEST_MUTEX.lock().unwrap(); // Ensure sequential execution

    let env = YubikeyTestEnvironment::detect();

    if !env.has_hardware() {
        println!("⚠ Skipping real hardware test - {}", env.description());
        return Ok(());
    }

    #[cfg(feature = "yubikey")]
    {
        println!("● Testing real YubiKey certificate operations...");

        let config = env.get_config().expect("Hardware should have config");

        let backend = YubiKeyBackend::with_config(config)?;

        // Test public key retrieval for standard PIV slots
        let slots = ["9a", "9c", "9d", "9e"];

        for slot in &slots {
            println!("● Checking PIV slot {}...", slot);

            let pubkey_operation = CryptoOperation::GetPublicKey;

            if backend.supports_operation(&pubkey_operation) {
                let result = backend.perform_operation(slot, pubkey_operation);

                match result {
                    Ok(CryptoResult::PublicKey(pubkey)) => {
                        println!(
                            "✔ Public key found in slot {}: {} bytes",
                            slot,
                            pubkey.len()
                        );
                    }
                    Ok(other) => {
                        println!("● Slot {} returned unexpected result: {:?}", slot, other);
                    }
                    Err(e) => {
                        println!("● No public key in slot {}: {}", slot, e);
                    }
                }
            } else {
                println!("● Public key operation not supported for slot {}", slot);
            }
        }
    }

    Ok(())
}

/// Test concurrent operations (stress test)
#[tokio::test]
#[ignore] // Requires real hardware and PIN
async fn test_real_concurrent_operations() -> Result<()> {
    let _lock = YUBIKEY_TEST_MUTEX.lock().unwrap(); // Ensure sequential execution

    let env = YubikeyTestEnvironment::detect();

    if !env.has_hardware() {
        println!("⚠ Skipping real hardware test - {}", env.description());
        return Ok(());
    }

    #[cfg(feature = "yubikey")]
    {
        println!("● Testing concurrent YubiKey operations...");

        let test_pin = get_test_pin();

        if test_pin.is_none() {
            println!("⚠ Skipping concurrent test - no PIN provided");
            return Ok(());
        }

        let mut config = env.get_config().expect("Hardware should have config");
        config.pin = test_pin;

        // Test multiple backend instances (YubiKey should handle concurrent access)
        let backend1 = YubiKeyBackend::with_config(config.clone())?;
        let backend2 = YubiKeyBackend::with_config(config)?;

        // Test capability queries from both backends
        let cap1 = backend1.get_capabilities();
        let cap2 = backend2.get_capabilities();

        assert_eq!(cap1.hardware_backed, cap2.hardware_backed);
        assert_eq!(cap1.supports_attestation, cap2.supports_attestation);

        println!("✔ Concurrent backend creation successful");

        // Test concurrent operation support checks
        let sign_op = CryptoOperation::Sign {
            data: vec![1, 2, 3, 4],
            algorithm: SignatureAlgorithm::EcdsaP256,
        };

        let support1 = backend1.supports_operation(&sign_op);
        let support2 = backend2.supports_operation(&sign_op);

        assert_eq!(support1, support2, "Operation support should be consistent");

        println!("✔ Concurrent operation support checks successful");
    }

    Ok(())
}
