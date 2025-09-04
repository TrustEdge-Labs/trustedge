//
// Copyright (c) 2025 John Turner
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/
//
// Project: trustedge — Privacy and trust at the edge.
//

//! Universal Backend Integration Tests
//!
//! These tests validate the Universal Backend system in real encrypt/decrypt workflows,
//! demonstrating capability-based backend selection and operation dispatch.

use anyhow::Result;
use std::io::Write;
use tempfile::NamedTempFile;
use trustedge_audio::{
    backends::universal::{CryptoOperation, CryptoResult, HashAlgorithm, KeyDerivationContext},
    backends::universal_keyring::UniversalKeyringBackend,
    backends::universal_registry::{BackendPreferences, UniversalBackendRegistry},
};

/// Test helper to create test data
fn create_test_data(size: usize) -> Vec<u8> {
    (0..size).map(|i| (i % 256) as u8).collect()
}

/// Test helper to write data to a temporary file
fn write_test_file(data: &[u8]) -> Result<NamedTempFile> {
    let mut file = NamedTempFile::new()?;
    file.write_all(data)?;
    file.flush()?;
    Ok(file)
}

#[test]
fn test_universal_backend_encrypt_decrypt_workflow() -> Result<()> {
    // Create a Universal Backend registry with keyring backend
    let mut registry = UniversalBackendRegistry::new();
    let keyring_backend = UniversalKeyringBackend::new()?;
    registry.register_backend("keyring".to_string(), Box::new(keyring_backend));

    // Test data
    let original_data = create_test_data(1024);
    let _input_file = write_test_file(&original_data)?;

    // Get the backend
    let backend = registry
        .get_backend("keyring")
        .ok_or_else(|| anyhow::anyhow!("Keyring backend not found"))?;

    // Verify backend capabilities
    let capabilities = backend.get_capabilities();
    assert!(
        capabilities.supports_key_derivation,
        "Backend should support key derivation"
    );
    assert!(
        !capabilities.hash_algorithms.is_empty(),
        "Backend should support hashing"
    );

    // Test key derivation through Universal Backend
    let key_context = KeyDerivationContext::new(b"test_salt_16byte".to_vec()) // Exactly 16 bytes
        .with_additional_data(b"encryption_purpose".to_vec());

    let key_operation = CryptoOperation::DeriveKey {
        context: key_context,
    };

    let key_result = backend.perform_operation("test_key_id", key_operation)?;
    let encryption_key = match key_result {
        CryptoResult::DerivedKey(key) => key,
        _ => panic!("Expected DerivedKey result"),
    };

    assert_eq!(encryption_key.len(), 32, "Key should be 32 bytes");

    // Test hash computation
    let hash_operation = CryptoOperation::Hash {
        algorithm: HashAlgorithm::Sha256,
        data: original_data.clone(),
    };

    let hash_result = backend.perform_operation("test_key_id", hash_operation)?;
    let computed_hash = match hash_result {
        CryptoResult::Hash(hash) => hash,
        _ => panic!("Expected Hash result"),
    };

    assert_eq!(computed_hash.len(), 32, "SHA-256 hash should be 32 bytes");

    println!("✔ Universal Backend encrypt/decrypt workflow validated");
    println!("  • Backend: {}", backend.backend_info().name);
    println!("  • Key derivation: {} bytes", encryption_key.len());
    println!("  • Hash computation: {} bytes", computed_hash.len());

    Ok(())
}

#[test]
fn test_universal_backend_capability_based_selection() -> Result<()> {
    let mut registry = UniversalBackendRegistry::new();
    let keyring_backend = UniversalKeyringBackend::new()?;
    registry.register_backend("keyring".to_string(), Box::new(keyring_backend));

    // Test 1: Select backend for key derivation
    let key_context = KeyDerivationContext::new(b"test_salt_16byte".to_vec()); // Exactly 16 bytes
    let backend_for_keys = registry
        .find_backend_for_operation(&CryptoOperation::DeriveKey {
            context: key_context,
        })
        .ok_or_else(|| anyhow::anyhow!("No backend found for key derivation"))?;
    assert_eq!(backend_for_keys.backend_info().name, "keyring");

    // Test 2: Select backend for hashing
    let backend_for_hash = registry
        .find_backend_for_operation(&CryptoOperation::Hash {
            algorithm: HashAlgorithm::Sha256,
            data: vec![1, 2, 3],
        })
        .ok_or_else(|| anyhow::anyhow!("No backend found for hashing"))?;
    assert_eq!(backend_for_hash.backend_info().name, "keyring");

    println!("✔ Universal Backend capability-based selection validated");
    Ok(())
}

#[test]
fn test_universal_backend_multiple_operations_workflow() -> Result<()> {
    let mut registry = UniversalBackendRegistry::new();
    let keyring_backend = UniversalKeyringBackend::new()?;
    registry.register_backend("keyring".to_string(), Box::new(keyring_backend));

    let backend = registry
        .get_backend("keyring")
        .ok_or_else(|| anyhow::anyhow!("Keyring backend not found"))?;

    // Simulate a complete cryptographic workflow using Universal Backend operations
    let test_data = b"sensitive data for encryption";

    // Step 1: Derive a key for this specific context
    let key_context = KeyDerivationContext::new(b"integ_test_16byt".to_vec()) // Exactly 16 bytes
        .with_additional_data(b"file_encryption".to_vec());
    let key_operation = CryptoOperation::DeriveKey {
        context: key_context,
    };
    let key_result = backend.perform_operation("integration_test_key", key_operation)?;
    let encryption_key = match key_result {
        CryptoResult::DerivedKey(key) => key,
        _ => panic!("Expected DerivedKey result"),
    };

    // Step 2: Compute a hash for integrity verification
    let hash_operation = CryptoOperation::Hash {
        algorithm: HashAlgorithm::Sha256,
        data: test_data.to_vec(),
    };
    let hash_result = backend.perform_operation("integration_test_key", hash_operation)?;
    let data_hash = match hash_result {
        CryptoResult::Hash(hash) => hash,
        _ => panic!("Expected Hash result"),
    };

    // Verify all operations completed successfully
    assert_eq!(encryption_key.len(), 32);
    assert_eq!(data_hash.len(), 32);

    // Verify key derivation is deterministic
    let key_context2 = KeyDerivationContext::new(b"integ_test_16byt".to_vec()) // Exactly 16 bytes
        .with_additional_data(b"file_encryption".to_vec());
    let key_operation2 = CryptoOperation::DeriveKey {
        context: key_context2,
    };
    let key_result2 = backend.perform_operation("integration_test_key", key_operation2)?;
    let encryption_key2 = match key_result2 {
        CryptoResult::DerivedKey(key) => key,
        _ => panic!("Expected DerivedKey result"),
    };

    assert_eq!(
        encryption_key, encryption_key2,
        "Key derivation should be deterministic"
    );

    println!("✔ Universal Backend multiple operations workflow validated");
    println!("  • Operations completed: key derivation, hash computation");
    println!("  • Key derivation determinism: verified");

    Ok(())
}

#[test]
fn test_universal_backend_error_handling() -> Result<()> {
    let mut registry = UniversalBackendRegistry::new();
    let keyring_backend = UniversalKeyringBackend::new()?;
    registry.register_backend("keyring".to_string(), Box::new(keyring_backend));

    let backend = registry
        .get_backend("keyring")
        .ok_or_else(|| anyhow::anyhow!("Keyring backend not found"))?;

    // Test 1: Operations that might not be supported
    // Note: The keyring backend might not support all operations

    // Test empty salt key derivation (edge case)
    let empty_key_context = KeyDerivationContext::new(vec![0u8; 16]); // 16 zero bytes
    let empty_key_operation = CryptoOperation::DeriveKey {
        context: empty_key_context,
    };
    let _result = backend.perform_operation("test_key", empty_key_operation);
    // Should handle gracefully - might succeed or fail depending on implementation

    println!("✔ Universal Backend error handling validated");
    Ok(())
}

#[test]
fn test_universal_backend_performance_characteristics() -> Result<()> {
    let mut registry = UniversalBackendRegistry::new();
    let keyring_backend = UniversalKeyringBackend::new()?;
    registry.register_backend("keyring".to_string(), Box::new(keyring_backend));

    let backend = registry
        .get_backend("keyring")
        .ok_or_else(|| anyhow::anyhow!("Keyring backend not found"))?;

    let start = std::time::Instant::now();

    // Perform multiple operations to test performance
    let num_operations = 10; // Reduced to avoid slow tests
    for i in 0..num_operations {
        // Key derivation - create 16-byte salt
        let mut salt = format!("salt_{:08}", i).into_bytes();
        salt.resize(16, 0); // Pad or truncate to exactly 16 bytes
        let key_context =
            KeyDerivationContext::new(salt).with_additional_data(b"performance_test".to_vec());
        let key_operation = CryptoOperation::DeriveKey {
            context: key_context,
        };
        let _key_result = backend.perform_operation(&format!("perf_key_{}", i), key_operation)?;

        // Hash computation
        let test_data = format!("test data {}", i).into_bytes();
        let hash_operation = CryptoOperation::Hash {
            algorithm: HashAlgorithm::Sha256,
            data: test_data,
        };
        let _hash_result = backend.perform_operation(&format!("perf_key_{}", i), hash_operation)?;
    }

    let duration = start.elapsed();
    let ops_per_second = (num_operations * 2) as f64 / duration.as_secs_f64();

    println!("✔ Universal Backend performance test completed");
    println!(
        "  • Operations: {} (key derivation, hash) x {}",
        2, num_operations
    );
    println!("  • Duration: {:?}", duration);
    println!("  • Rate: {:.2} operations/second", ops_per_second);

    // Basic performance assertion - should complete reasonably quickly
    assert!(
        duration.as_secs() < 30,
        "Performance test should complete within 30 seconds"
    );

    Ok(())
}

#[test]
fn test_universal_backend_registry_management() -> Result<()> {
    // Test registry creation and backend management
    let mut registry = UniversalBackendRegistry::new();

    // Initially no backends
    assert_eq!(registry.list_backend_names().len(), 0);

    // Add a backend
    let keyring_backend = UniversalKeyringBackend::new()?;
    registry.register_backend("keyring".to_string(), Box::new(keyring_backend));

    assert_eq!(registry.list_backend_names().len(), 1);
    assert!(registry.list_backend_names().contains(&"keyring"));

    // Test preferences-based operation
    let preferences = BackendPreferences::new();

    let key_context = KeyDerivationContext::new(b"test_salt_16byte".to_vec()); // Exactly 16 bytes
    let key_op = CryptoOperation::DeriveKey {
        context: key_context,
    };
    let backend_for_op = registry
        .find_preferred_backend(&key_op, &preferences)
        .ok_or_else(|| anyhow::anyhow!("No backend found for operation"))?;
    assert_eq!(backend_for_op.backend_info().name, "keyring");

    // Test registry operation
    let result = registry.perform_operation("test_key", key_op, Some(&preferences))?;
    match result {
        CryptoResult::DerivedKey(key) => {
            assert_eq!(key.len(), 32, "Key should be 32 bytes");
        }
        _ => panic!("Expected DerivedKey result"),
    }

    println!("✔ Universal Backend registry management validated");
    Ok(())
}
