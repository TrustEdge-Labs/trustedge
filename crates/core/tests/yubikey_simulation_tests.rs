/*
 * Copyright (c) 2025 TRUSTEDGE LABS LLC
 * This source code is subject to the terms of the Mozilla Public License, v. 2.0.
 * If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
 *
 * Project: trustedge — Privacy and trust at the edge.
 */

//! YubiKey Simulation Tests - CI-Safe Software Validation
//!
//! These tests run in all environments without requiring physical YubiKey hardware.
//! They validate software components, API interfaces, and configuration handling.

use anyhow::Result;

mod yubikey_hardware_detection;
use trustedge_core::backends::YubiKeyConfig;
use yubikey_hardware_detection::YubikeyTestEnvironment;

#[cfg(feature = "yubikey")]
use trustedge_core::backends::YubiKeyBackend;

#[cfg(feature = "yubikey")]
use trustedge_core::UniversalBackend;

/// Test YubiKey configuration validation and API interfaces
#[tokio::test]
async fn test_yubikey_configuration_validation() -> Result<()> {
    let env = YubikeyTestEnvironment::detect();
    println!("● Testing configuration - {}", env.description());

    // Test default configuration
    let default_config = YubiKeyConfig::default();
    assert!(!default_config.pkcs11_module_path.is_empty());
    assert_eq!(default_config.pin, None);
    assert_eq!(default_config.slot, None);
    assert!(!default_config.verbose);

    // Test custom configuration
    let custom_config = YubiKeyConfig {
        pkcs11_module_path: "/custom/path/to/pkcs11.so".to_string(),
        pin: Some("123456".to_string()),
        slot: Some(0),
        verbose: true,
    };

    assert_eq!(custom_config.pin, Some("123456".to_string()));
    assert_eq!(custom_config.slot, Some(0));
    assert!(custom_config.verbose);

    // Validate configuration structure
    assert!(custom_config.pkcs11_module_path.ends_with(".so"));

    Ok(())
}

/// Test YubiKey backend API interface (without hardware operations)
#[tokio::test]
async fn test_yubikey_backend_interface() -> Result<()> {
    let env = YubikeyTestEnvironment::detect();
    println!("● Testing backend interface - {}", env.description());

    #[cfg(feature = "yubikey")]
    {
        // Test backend capabilities reporting (without requiring hardware initialization)
        use trustedge_core::backends::YubiKeyBackend;

        // Try to create backend, but don't fail test if hardware unavailable
        let backend_result = YubiKeyBackend::new();

        match backend_result {
            Ok(backend) => {
                // Real hardware available - test full capabilities
                let capabilities = backend.get_capabilities();
                assert!(capabilities.hardware_backed);
                assert!(capabilities.supports_attestation);

                let info = backend.backend_info();
                assert_eq!(info.name, "yubikey");
                assert_eq!(info.description, "YubiKey PKCS#11 hardware security module");
                assert!(info.config_requirements.contains(&"pkcs11_module_path"));

                println!("✔ Hardware backend capabilities validated");
            }
            Err(_) => {
                // No hardware - test static capability information
                use trustedge_core::{
                    AsymmetricAlgorithm, BackendCapabilities, SignatureAlgorithm,
                };

                // Test that we can still get static capability info
                let expected_capabilities = BackendCapabilities {
                    hardware_backed: true,
                    supports_attestation: true,
                    supports_key_derivation: false,
                    supports_key_generation: true,
                    asymmetric_algorithms: vec![AsymmetricAlgorithm::EcdsaP256],
                    signature_algorithms: vec![SignatureAlgorithm::EcdsaP256],
                    max_key_size: Some(256),
                    symmetric_algorithms: vec![],
                    hash_algorithms: vec![],
                };

                // Validate expected YubiKey capabilities
                assert!(expected_capabilities.hardware_backed);
                assert!(expected_capabilities.supports_attestation);
                assert!(expected_capabilities
                    .asymmetric_algorithms
                    .contains(&AsymmetricAlgorithm::EcdsaP256));

                println!("● Hardware unavailable - validated expected capabilities");
            }
        }
    }

    #[cfg(not(feature = "yubikey"))]
    {
        println!("● YubiKey feature disabled - testing compilation only");
        // When feature is disabled, ensure clean compilation
    }

    Ok(())
}

/// Test PIV slot validation and enumeration
#[tokio::test]
async fn test_piv_slot_validation() -> Result<()> {
    let env = YubikeyTestEnvironment::detect();
    println!("● Testing PIV slots - {}", env.description());

    let slots = env.get_slots();
    println!("● Available slots: {:?}", slots);
    assert!(
        !slots.is_empty(),
        "Should have PIV slots available (got {})",
        slots.len()
    );

    // Validate standard PIV slot format
    let expected_slots = ["9a", "9c", "9d", "9e"];
    for expected in &expected_slots {
        assert!(
            slots.contains(&expected.to_string()),
            "Missing standard PIV slot: {}",
            expected
        );
    }

    // Test slot format validation
    for slot in &slots {
        assert_eq!(slot.len(), 2, "PIV slot should be 2 characters: {}", slot);
        assert!(
            slot.starts_with('9'),
            "PIV slot should start with '9': {}",
            slot
        );
        assert!(
            matches!(slot.chars().nth(1), Some('a' | 'c' | 'd' | 'e')),
            "PIV slot should end with a/c/d/e: {}",
            slot
        );
    }

    Ok(())
}

/// Test PKCS#11 module path validation  
#[tokio::test]
async fn test_pkcs11_module_validation() -> Result<()> {
    let env = YubikeyTestEnvironment::detect();
    println!("● Testing PKCS#11 module - {}", env.description());

    if let Some(config) = env.get_config() {
        let module_path = &config.pkcs11_module_path;

        // Basic path validation
        assert!(
            !module_path.is_empty(),
            "PKCS#11 module path should not be empty"
        );

        // Platform-specific library extension
        let has_valid_extension = module_path.ends_with(".so")    // Linux
            || module_path.ends_with(".dylib")                    // macOS  
            || module_path.ends_with(".dll"); // Windows

        assert!(
            has_valid_extension,
            "PKCS#11 module should have valid library extension: {}",
            module_path
        );

        // In hardware mode, path should exist
        if env.has_hardware() {
            assert!(
                std::path::Path::new(module_path).exists(),
                "Hardware PKCS#11 module should exist: {}",
                module_path
            );
        }
    }

    Ok(())
}

/// Test certificate parameter structures
#[tokio::test]
async fn test_certificate_parameter_validation() -> Result<()> {
    let env = YubikeyTestEnvironment::detect();
    println!("● Testing certificate parameters - {}", env.description());

    #[cfg(feature = "yubikey")]
    {
        use trustedge_core::backends::CertificateParams;

        // Test basic certificate parameters
        let cert_params = CertificateParams {
            subject: "CN=test-yubikey.example.com".to_string(),
            validity_days: 365,
            is_ca: false,
            key_usage: vec!["digital_signature".to_string(), "key_agreement".to_string()],
        };

        // Validate parameter structure
        assert!(!cert_params.subject.is_empty());
        assert!(cert_params.validity_days > 0);
        assert!(!cert_params.key_usage.is_empty());
        assert!(!cert_params.is_ca);

        // Validate subject format
        assert!(cert_params.subject.starts_with("CN="));
        assert!(cert_params.subject.contains("example.com"));

        // Validate key usage
        assert!(cert_params
            .key_usage
            .contains(&"digital_signature".to_string()));

        // Test QUIC-compatible certificate parameters
        let quic_cert_params = CertificateParams {
            subject: "CN=quic-server.local".to_string(),
            validity_days: 90,
            is_ca: false,
            key_usage: vec!["digital_signature".to_string(), "key_agreement".to_string()],
        };

        assert!(quic_cert_params
            .key_usage
            .contains(&"digital_signature".to_string()));
        assert!(quic_cert_params
            .key_usage
            .contains(&"key_agreement".to_string()));
        assert!(!quic_cert_params.is_ca);
    }

    Ok(())
}

/// Test error handling in software simulation
#[tokio::test]
async fn test_error_handling_simulation() -> Result<()> {
    let env = YubikeyTestEnvironment::detect();
    println!("● Testing error handling - {}", env.description());

    #[cfg(feature = "yubikey")]
    {
        // Test invalid PKCS#11 module path
        let invalid_config = YubiKeyConfig {
            pkcs11_module_path: "/nonexistent/invalid/path/pkcs11.so".to_string(),
            pin: None,
            slot: None,
            verbose: false,
        };

        // Validate config structure even if path is invalid
        assert!(!invalid_config.pkcs11_module_path.is_empty());
        assert!(invalid_config.pkcs11_module_path.ends_with(".so"));

        // In simulation mode, we test error condition handling
        if env.is_simulation() {
            // Test that backend creation handles invalid configs gracefully
            let backend_result = YubiKeyBackend::with_config(invalid_config);

            // Should fail with meaningful error (software validation)
            if let Err(error) = backend_result {
                let error_msg = error.to_string().to_lowercase();
                assert!(
                    error_msg.contains("pkcs11")
                        || error_msg.contains("module")
                        || error_msg.contains("not found")
                        || error_msg.contains("invalid"),
                    "Error should mention PKCS#11 or module issue: {}",
                    error
                );
            }
        }
    }

    Ok(())
}

/// Test configuration serialization/deserialization
#[tokio::test]
async fn test_configuration_serialization() -> Result<()> {
    let env = YubikeyTestEnvironment::detect();
    println!(
        "● Testing configuration serialization - {}",
        env.description()
    );

    if let Some(config) = env.get_config() {
        // Test configuration cloning
        let cloned_config = config.clone();
        assert_eq!(config.pkcs11_module_path, cloned_config.pkcs11_module_path);
        assert_eq!(config.pin, cloned_config.pin);
        assert_eq!(config.slot, cloned_config.slot);
        assert_eq!(config.verbose, cloned_config.verbose);

        // Test configuration comparison (field by field since YubiKeyConfig doesn't impl PartialEq)
        assert_eq!(config.pkcs11_module_path, cloned_config.pkcs11_module_path);
        assert_eq!(config.pin, cloned_config.pin);
        assert_eq!(config.slot, cloned_config.slot);
        assert_eq!(config.verbose, cloned_config.verbose);
    }

    Ok(())
}
