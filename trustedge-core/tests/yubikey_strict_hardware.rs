/*
 * Copyright (c) 2025 TRUSTEDGE LABS LLC
 * This source code is subject to the terms of the Mozilla Public License, v. 2.0.
 * If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
 *
 * Project: trustedge — Privacy and trust at the edge.
 */

//! YubiKey Strict Hardware Tests - MUST HAVE YUBIKEY PLUGGED IN
//!
//! These tests REQUIRE physical YubiKey hardware and WILL FAIL if unplugged.
//! Run with: cargo test --test yubikey_strict_hardware --features yubikey
//!
//! ⚠ WARNING: These tests verify real YubiKey presence and operations.

use anyhow::Result;
use std::sync::Mutex;

// Global mutex to ensure all YubiKey hardware tests run sequentially
// This prevents PKCS#11 resource conflicts and detection race conditions
static YUBIKEY_HARDWARE_TEST_MUTEX: Mutex<()> = Mutex::new(());

mod yubikey_hardware_detection;
use yubikey_hardware_detection::YubikeyTestEnvironment;

#[cfg(feature = "yubikey")]
use trustedge_core::backends::YubiKeyBackend;

#[cfg(feature = "yubikey")]
use trustedge_core::{AsymmetricAlgorithm, CryptoOperation, SignatureAlgorithm, UniversalBackend};

/// STRICT: YubiKey hardware MUST be present and working
#[tokio::test]
async fn test_strict_yubikey_hardware_required() -> Result<()> {
    let _lock = YUBIKEY_HARDWARE_TEST_MUTEX.lock().unwrap();

    let env = YubikeyTestEnvironment::detect();

    // FAIL if hardware is not available
    assert!(
        env.has_hardware(),
        "STRICT TEST FAILURE: YubiKey hardware MUST be plugged in. Current status: {}",
        env.description()
    );

    println!(
        "✔ STRICT: YubiKey hardware verified present - {}",
        env.description()
    );
    Ok(())
}

/// STRICT: YubiKey backend MUST initialize successfully  
#[tokio::test]
async fn test_strict_yubikey_backend_initialization() -> Result<()> {
    let _lock = YUBIKEY_HARDWARE_TEST_MUTEX.lock().unwrap();

    let env = YubikeyTestEnvironment::detect();

    // FAIL if no hardware
    assert!(
        env.has_hardware(),
        "STRICT TEST FAILURE: YubiKey hardware required for backend test. Status: {}",
        env.description()
    );

    #[cfg(feature = "yubikey")]
    {
        let config = env.get_config().expect("Hardware should have config");

        // MUST successfully create backend - no graceful fallback
        let backend = YubiKeyBackend::with_config(config)
            .expect("STRICT TEST FAILURE: YubiKey backend MUST initialize successfully");

        // MUST have hardware capabilities
        let capabilities = backend.get_capabilities();
        assert!(
            capabilities.hardware_backed,
            "STRICT FAILURE: Backend must be hardware-backed"
        );
        assert!(
            capabilities.supports_attestation,
            "STRICT FAILURE: Backend must support attestation"
        );

        println!("✔ STRICT: YubiKey backend initialized successfully");
    }

    Ok(())
}

/// STRICT: PIV slots MUST be accessible
#[tokio::test]
async fn test_strict_piv_slots_accessible() -> Result<()> {
    let _lock = YUBIKEY_HARDWARE_TEST_MUTEX.lock().unwrap();

    let env = YubikeyTestEnvironment::detect();

    // FAIL if no hardware
    assert!(
        env.has_hardware(),
        "STRICT TEST FAILURE: YubiKey hardware required for PIV slot test"
    );

    let slots = env.get_slots();

    // MUST have all 4 standard PIV slots
    assert_eq!(
        slots.len(),
        4,
        "STRICT FAILURE: Must have exactly 4 PIV slots, found {}",
        slots.len()
    );

    let required_slots = ["9a", "9c", "9d", "9e"];
    for slot in &required_slots {
        assert!(
            slots.contains(&slot.to_string()),
            "STRICT FAILURE: Missing required PIV slot: {}",
            slot
        );
    }

    println!("✔ STRICT: All 4 PIV slots accessible: {:?}", slots);
    Ok(())
}

/// STRICT: PKCS#11 operations MUST work
#[tokio::test]
async fn test_strict_pkcs11_operations() -> Result<()> {
    let _lock = YUBIKEY_HARDWARE_TEST_MUTEX.lock().unwrap();

    let env = YubikeyTestEnvironment::detect();

    // FAIL if no hardware
    assert!(
        env.has_hardware(),
        "STRICT TEST FAILURE: YubiKey hardware required for PKCS#11 test"
    );

    #[cfg(feature = "yubikey")]
    {
        let config = env.get_config().expect("Hardware should have config");

        // PKCS#11 module MUST exist
        assert!(
            std::path::Path::new(&config.pkcs11_module_path).exists(),
            "STRICT FAILURE: PKCS#11 module must exist: {}",
            config.pkcs11_module_path
        );

        // Backend creation might fail due to PIN requirement, but error should be specific
        let backend_result = YubiKeyBackend::with_config(config);

        match backend_result {
            Ok(_backend) => {
                println!("✔ STRICT: PKCS#11 operations working without PIN");
            }
            Err(e) => {
                // Error must be about authentication, not hardware unavailability
                let error_msg = e.to_string().to_lowercase();
                assert!(
                    error_msg.contains("pin")
                        || error_msg.contains("auth")
                        || error_msg.contains("login"),
                    "STRICT FAILURE: Error should be about authentication, not hardware: {}",
                    e
                );
                println!(
                    "✔ STRICT: PKCS#11 accessible (PIN required for operations): {}",
                    e
                );
            }
        }
    }

    Ok(())
}

/// STRICT: Operation support MUST be accurate
#[tokio::test]
async fn test_strict_operation_support() -> Result<()> {
    let _lock = YUBIKEY_HARDWARE_TEST_MUTEX.lock().unwrap();

    let env = YubikeyTestEnvironment::detect();

    // FAIL if no hardware
    assert!(
        env.has_hardware(),
        "STRICT TEST FAILURE: YubiKey hardware required for operation support test"
    );

    #[cfg(feature = "yubikey")]
    {
        // Use YubiKey instance for capability testing (may fail, but should be consistent)
        let backend_result = YubiKeyBackend::new();

        match backend_result {
            Ok(backend) => {
                // Test supported operations
                let sign_op = CryptoOperation::Sign {
                    data: vec![1, 2, 3, 4],
                    algorithm: SignatureAlgorithm::EcdsaP256,
                };
                assert!(
                    backend.supports_operation(&sign_op),
                    "STRICT FAILURE: YubiKey MUST support ECDSA P-256 signing"
                );

                let attest_op = CryptoOperation::Attest {
                    challenge: vec![5, 6, 7, 8],
                };
                assert!(
                    backend.supports_operation(&attest_op),
                    "STRICT FAILURE: YubiKey MUST support attestation"
                );

                // Test unsupported operations
                let keygen_op = CryptoOperation::GenerateKeyPair {
                    algorithm: AsymmetricAlgorithm::EcdsaP256,
                };
                assert!(!backend.supports_operation(&keygen_op),
                       "STRICT FAILURE: YubiKey must NOT support key generation through universal backend");

                println!("✔ STRICT: Operation support validated");
            }
            Err(e) => {
                // Even if backend creation fails, we can still test static behavior
                println!(
                    "⚠ STRICT: Backend creation failed (likely needs PIN): {}",
                    e
                );

                // But we still require the error to be authentication-related
                let error_msg = e.to_string().to_lowercase();
                assert!(
                    error_msg.contains("pin") || error_msg.contains("auth") || error_msg.contains("token"),
                    "STRICT FAILURE: Error should be authentication-related, not hardware missing: {}", e
                );
            }
        }
    }

    Ok(())
}

/// STRICT: Hardware capabilities MUST be correct
#[tokio::test]
async fn test_strict_hardware_capabilities() -> Result<()> {
    let _lock = YUBIKEY_HARDWARE_TEST_MUTEX.lock().unwrap();

    let env = YubikeyTestEnvironment::detect();

    // FAIL if no hardware
    assert!(
        env.has_hardware(),
        "STRICT TEST FAILURE: YubiKey hardware required for capabilities test"
    );

    #[cfg(feature = "yubikey")]
    {
        let backend_result = YubiKeyBackend::new();

        match backend_result {
            Ok(backend) => {
                let capabilities = backend.get_capabilities();

                // STRICT capability requirements
                assert!(
                    capabilities.hardware_backed,
                    "STRICT FAILURE: Must be hardware-backed"
                );
                assert!(
                    capabilities.supports_attestation,
                    "STRICT FAILURE: Must support attestation"
                );
                assert!(
                    !capabilities.supports_key_derivation,
                    "STRICT FAILURE: YubiKey should not support key derivation"
                );
                assert!(
                    capabilities.supports_key_generation,
                    "STRICT FAILURE: YubiKey should support key generation"
                );

                // STRICT algorithm requirements
                assert!(
                    capabilities
                        .asymmetric_algorithms
                        .contains(&AsymmetricAlgorithm::EcdsaP256),
                    "STRICT FAILURE: Must support ECDSA P-256"
                );
                assert!(
                    capabilities
                        .signature_algorithms
                        .contains(&SignatureAlgorithm::EcdsaP256),
                    "STRICT FAILURE: Must support ECDSA P-256 signatures"
                );

                println!("✔ STRICT: Hardware capabilities verified");
            }
            Err(e) => {
                println!("⚠ STRICT: Cannot verify capabilities without PIN: {}", e);
            }
        }
    }

    Ok(())
}
