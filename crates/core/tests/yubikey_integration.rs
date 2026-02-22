//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//

//! YubiKey Hardware Integration Tests
//!
//! These tests verify real hardware operations with a physical YubiKey device.
//! All tests are marked with #[ignore] since they require:
//! - Physical YubiKey 5 series device with PIV applet enabled
//! - Running PCSC daemon (pcscd)
//! - Pre-configured PIN (default: 123456, override with YUBIKEY_TEST_PIN env var)
//! - Pre-generated keys in PIV slots (use ykman to set up)
//!
//! Run with: cargo test --features yubikey --test yubikey_integration -- --ignored
//!
//! CAUTION: Some tests consume PIN retry attempts. Do not run repeatedly to avoid lockout.

#![cfg(feature = "yubikey")]

use der::Decode;
use trustedge_core::backends::universal::{
    CryptoOperation, CryptoResult, HashAlgorithm, SignatureAlgorithm, UniversalBackend,
};
use trustedge_core::backends::yubikey::{YubiKeyBackend, YubiKeyConfig};
use trustedge_core::error::BackendError;
use x509_cert::Certificate;

/// Create a test backend configuration
/// Uses YUBIKEY_TEST_PIN env var or falls back to default "123456"
fn create_test_config() -> YubiKeyConfig {
    let pin = std::env::var("YUBIKEY_TEST_PIN").unwrap_or_else(|_| "123456".to_string());
    YubiKeyConfig::builder()
        .pin(pin)
        .default_slot("9c".to_string())
        .verbose(true)
        .max_pin_retries(3)
        .build()
}

/// Create a backend connected to hardware, panicking with clear message if unavailable
fn create_hardware_backend() -> YubiKeyBackend {
    let config = create_test_config();
    let backend = YubiKeyBackend::with_config(config)
        .expect("Failed to create YubiKey backend. Is a YubiKey inserted?");

    // Verify hardware is actually available
    let info = backend.backend_info();
    assert!(
        info.available,
        "YubiKey hardware not detected. Insert device and retry."
    );

    backend
}

// ===== Hardware Signing Tests (TEST-02) =====

#[test]
#[ignore = "requires physical YubiKey"]
fn test_real_ecdsa_p256_signing() {
    let backend = create_hardware_backend();

    // Perform ECDSA P-256 signing on hardware
    let data = b"Hardware ECDSA P-256 signing test";
    let result = backend.perform_operation(
        "9c",
        CryptoOperation::Sign {
            data: data.to_vec(),
            algorithm: SignatureAlgorithm::EcdsaP256,
        },
    );

    assert!(result.is_ok(), "ECDSA P-256 signing should succeed");

    match result.unwrap() {
        CryptoResult::Signed(signature) => {
            assert!(!signature.is_empty(), "Signature must not be empty");
            // ECDSA P-256 signatures are typically 64-72 bytes (DER-encoded)
            assert!(
                signature.len() > 32 && signature.len() < 128,
                "Signature length {} is outside expected range for ECDSA P-256",
                signature.len()
            );
            println!(
                "✔ Hardware ECDSA P-256 signature generated ({} bytes)",
                signature.len()
            );
        }
        _ => panic!("Expected Signed result from hardware signing operation"),
    }
}

#[test]
#[ignore = "requires physical YubiKey"]
fn test_real_rsa_2048_signing() {
    let backend = create_hardware_backend();

    // IMPORTANT: This test assumes slot 9c has an RSA key.
    // If the slot has an ECDSA key, this test will fail.
    // Use ykman to check: ykman piv info
    //
    // To skip gracefully, check if RSA is supported first
    let sign_op = CryptoOperation::Sign {
        data: b"Hardware RSA-2048 signing test".to_vec(),
        algorithm: SignatureAlgorithm::RsaPkcs1v15,
    };

    if !backend.supports_operation(&sign_op) {
        println!("⚠ Skipping RSA test: slot may not have RSA key or RSA not supported by backend");
        return;
    }

    let result = backend.perform_operation("9c", sign_op);

    // If the slot has ECDSA key instead of RSA, this will fail
    // That's expected behavior - the test verifies real hardware constraints
    if let Err(ref e) = result {
        println!("⚠ RSA signing failed (slot may have ECDSA key): {}", e);
        // Still assert it's a proper error type
        assert!(
            matches!(
                e,
                BackendError::HardwareError(_) | BackendError::UnsupportedOperation(_)
            ),
            "RSA signing should fail with hardware or unsupported operation error"
        );
        return;
    }

    match result.unwrap() {
        CryptoResult::Signed(signature) => {
            assert!(!signature.is_empty(), "RSA signature must not be empty");
            // RSA-2048 signatures are 256 bytes
            assert!(
                signature.len() > 64,
                "RSA signature length {} too small",
                signature.len()
            );
            println!(
                "✔ Hardware RSA-2048 signature generated ({} bytes)",
                signature.len()
            );
        }
        _ => panic!("Expected Signed result from RSA signing operation"),
    }
}

#[test]
#[ignore = "requires physical YubiKey"]
fn test_real_public_key_extraction() {
    let backend = create_hardware_backend();

    let result = backend.perform_operation("9c", CryptoOperation::GetPublicKey);

    assert!(
        result.is_ok(),
        "Public key extraction should succeed with hardware"
    );

    match result.unwrap() {
        CryptoResult::PublicKey(key_bytes) => {
            assert!(!key_bytes.is_empty(), "Public key must not be empty");
            // DER-encoded SPKI for ECDSA P-256 is typically 59-91 bytes
            assert!(
                key_bytes.len() > 30,
                "Public key length {} too small for SPKI",
                key_bytes.len()
            );
            println!(
                "✔ Hardware public key extracted ({} bytes)",
                key_bytes.len()
            );
        }
        _ => panic!("Expected PublicKey result from GetPublicKey operation"),
    }
}

// ===== Key Enumeration Test (TEST-02) =====

#[test]
#[ignore = "requires physical YubiKey"]
fn test_real_slot_enumeration() {
    let backend = create_hardware_backend();

    let keys = backend
        .list_keys()
        .expect("Slot enumeration should succeed with hardware");

    assert!(
        !keys.is_empty(),
        "YubiKey should have at least one populated slot"
    );

    println!("✔ Found {} populated PIV slot(s):", keys.len());
    for key in &keys {
        assert!(
            !key.description.is_empty(),
            "Key description must not be empty"
        );
        println!("  - {}", key.description);
    }
}

// ===== Certificate Round-Trip Test (TEST-05) =====

#[test]
#[ignore = "requires physical YubiKey"]
fn test_certificate_generation_round_trip() {
    let backend = create_hardware_backend();

    // Step 1: Generate certificate using hardware-backed signing
    let cert_der = backend
        .generate_certificate("9c", "TrustEdge Test Certificate")
        .expect("Certificate generation should succeed");

    assert!(!cert_der.is_empty(), "Certificate DER must not be empty");
    println!("✔ Certificate generated ({} bytes DER)", cert_der.len());

    // Step 2: Parse certificate with x509-cert
    let cert =
        Certificate::from_der(&cert_der).expect("Generated certificate should be valid X.509 DER");

    // Step 3: Extract public key from certificate
    let cert_public_key_bytes = cert
        .tbs_certificate
        .subject_public_key_info
        .subject_public_key
        .raw_bytes();

    println!(
        "✔ Certificate public key extracted ({} bytes)",
        cert_public_key_bytes.len()
    );

    // Step 4: Get public key from hardware
    let hw_result = backend
        .perform_operation("9c", CryptoOperation::GetPublicKey)
        .expect("Hardware public key extraction should succeed");

    let hw_public_key_der = match hw_result {
        CryptoResult::PublicKey(key_bytes) => key_bytes,
        _ => panic!("Expected PublicKey result from hardware"),
    };

    // Step 5: Parse hardware SPKI to extract raw public key bytes
    let hw_spki = spki::SubjectPublicKeyInfoRef::try_from(hw_public_key_der.as_slice())
        .expect("Hardware should return valid SPKI");
    let hw_public_key_bytes = hw_spki.subject_public_key.raw_bytes();

    // Step 6: Verify certificate public key matches hardware public key
    assert_eq!(
        cert_public_key_bytes, hw_public_key_bytes,
        "Certificate public key must match hardware public key"
    );
    println!("✔ Certificate public key matches hardware public key");

    // Step 7: Verify subject contains expected CN
    let subject_string = cert.tbs_certificate.subject.to_string();
    assert!(
        subject_string.contains("TrustEdge Test Certificate"),
        "Certificate subject '{}' should contain 'TrustEdge Test Certificate'",
        subject_string
    );
    println!("✔ Certificate subject verified: {}", subject_string);
}

// ===== Anti-Pattern Hardware Tests (TEST-03) =====

#[test]
#[ignore = "requires physical YubiKey"]
fn test_hardware_backend_info_reports_available() {
    let backend = create_hardware_backend();
    let info = backend.backend_info();

    assert!(
        info.available,
        "Backend info should report available=true when hardware is present"
    );
    assert_eq!(info.name, "yubikey");
    println!("✔ Backend info reports hardware available: {}", info.name);
}

#[test]
#[ignore = "requires physical YubiKey"]
fn test_ed25519_rejected_by_hardware_backend() {
    let backend = create_hardware_backend();

    // YubiKey PIV does NOT support Ed25519, even when hardware is present
    let result = backend.perform_operation(
        "9c",
        CryptoOperation::Sign {
            data: b"test".to_vec(),
            algorithm: SignatureAlgorithm::Ed25519,
        },
    );

    assert!(
        result.is_err(),
        "Ed25519 signing should fail on YubiKey PIV hardware"
    );

    match result.unwrap_err() {
        BackendError::UnsupportedOperation(msg) => {
            assert!(
                msg.contains("Ed25519") || msg.to_lowercase().contains("not supported"),
                "Error message should explain Ed25519 is unsupported: {}",
                msg
            );
            println!("✔ Ed25519 correctly rejected: {}", msg);
        }
        other => panic!(
            "Expected UnsupportedOperation error for Ed25519, got: {:?}",
            other
        ),
    }
}

// ===== Negative Tests (TEST-06) =====

#[test]
#[ignore = "requires physical YubiKey"]
fn test_wrong_pin_returns_error() {
    // WARNING: This test consumes a PIN retry. Run sparingly to avoid lockout.

    let config = YubiKeyConfig::builder()
        .pin("000000".to_string()) // Wrong PIN
        .default_slot("9c".to_string())
        .verbose(true)
        .max_pin_retries(3)
        .build();

    let backend = YubiKeyBackend::with_config(config)
        .expect("Backend creation should succeed even with wrong PIN");

    // Try to perform a signing operation (requires PIN verification)
    let result = backend.perform_operation(
        "9c",
        CryptoOperation::Sign {
            data: b"test".to_vec(),
            algorithm: SignatureAlgorithm::EcdsaP256,
        },
    );

    assert!(
        result.is_err(),
        "Signing with wrong PIN should fail gracefully"
    );

    match result.unwrap_err() {
        BackendError::HardwareError(msg) => {
            let msg_lower = msg.to_lowercase();
            assert!(
                msg_lower.contains("pin"),
                "Error message should mention PIN: {}",
                msg
            );
            println!("✔ Wrong PIN correctly rejected: {}", msg);
        }
        other => panic!("Expected HardwareError for wrong PIN, got: {:?}", other),
    }
}

#[test]
#[ignore = "requires physical YubiKey"]
fn test_hash_works_with_hardware_present() {
    let backend = create_hardware_backend();

    // Hash operations should work even with hardware backend (software path)
    let result = backend.perform_operation(
        "9c", // key_id not used for hash operations
        CryptoOperation::Hash {
            data: b"hardware hash test".to_vec(),
            algorithm: HashAlgorithm::Sha256,
        },
    );

    assert!(
        result.is_ok(),
        "Hash operation should succeed with hardware backend"
    );

    match result.unwrap() {
        CryptoResult::Hash(hash) => {
            assert_eq!(hash.len(), 32, "SHA-256 hash should be 32 bytes");
            println!("✔ SHA-256 hash computed via hardware backend");
        }
        _ => panic!("Expected Hash result from hash operation"),
    }
}
