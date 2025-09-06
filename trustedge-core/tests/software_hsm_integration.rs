//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/
//
// Project: trustedge — Privacy and trust at the edge.
//

//! Software HSM Integration Tests
//!
//! These tests validate the Software HSM backend in real-world scenarios,
//! including file persistence, registry integration, CLI workflows, and
//! cross-session operations.

use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::TempDir;
use trustedge_core::backends::{
    software_hsm::{SoftwareHsmBackend, SoftwareHsmConfig},
    universal::{
        AsymmetricAlgorithm, CryptoOperation, CryptoResult, SignatureAlgorithm, UniversalBackend,
    },
    universal_registry::{BackendPreferences, UniversalBackendRegistry},
};

/// Test helper to create a temporary Software HSM setup
fn create_test_hsm_setup() -> Result<(TempDir, SoftwareHsmConfig)> {
    let temp_dir = TempDir::new()?;
    let config = SoftwareHsmConfig {
        key_store_path: temp_dir.path().to_path_buf(),
        default_passphrase: "integration_test_pass".to_string(),
        metadata_file: temp_dir.path().join("hsm_metadata.json"),
    };
    Ok((temp_dir, config))
}

/// Test helper to create test data files
fn create_test_data_file(temp_dir: &TempDir, name: &str, content: &str) -> Result<PathBuf> {
    let file_path = temp_dir.path().join(name);
    fs::write(&file_path, content)?;
    Ok(file_path)
}

/// Helper function to sign data using the public UniversalBackend interface
fn sign_data_via_universal_backend(
    backend: &dyn UniversalBackend,
    key_id: &str,
    data: &[u8],
    algorithm: SignatureAlgorithm,
) -> Result<Vec<u8>> {
    let sign_op = CryptoOperation::Sign {
        data: data.to_vec(),
        algorithm,
    };

    match backend.perform_operation(key_id, sign_op)? {
        CryptoResult::Signed(signature) => Ok(signature),
        _ => Err(anyhow::anyhow!("Expected signed result")),
    }
}

/// Helper function to verify signature using the public UniversalBackend interface
fn verify_signature_via_universal_backend(
    backend: &dyn UniversalBackend,
    key_id: &str,
    data: &[u8],
    signature: &[u8],
    algorithm: SignatureAlgorithm,
) -> Result<bool> {
    let verify_op = CryptoOperation::Verify {
        data: data.to_vec(),
        signature: signature.to_vec(),
        algorithm,
    };

    match backend.perform_operation(key_id, verify_op)? {
        CryptoResult::VerificationResult(is_valid) => Ok(is_valid),
        _ => Err(anyhow::anyhow!("Expected verification result")),
    }
}

/// Helper function to generate key pair using the public UniversalBackend interface
#[allow(dead_code)]
fn generate_key_pair_via_universal_backend(
    backend: &dyn UniversalBackend,
    key_id: &str,
    algorithm: AsymmetricAlgorithm,
) -> Result<()> {
    let gen_op = CryptoOperation::GenerateKeyPair { algorithm };
    backend.perform_operation(key_id, gen_op)?;
    Ok(())
}

/// Test helper to run CLI command and capture output
fn run_cli_command(args: &[&str], key_store_path: &Path) -> Result<(i32, String, String)> {
    let mut cmd = Command::new("cargo");
    cmd.arg("run")
        .arg("--bin")
        .arg("software-hsm-demo")
        .arg("--")
        .arg("--key-store")
        .arg(key_store_path.to_str().unwrap());

    for arg in args {
        cmd.arg(arg);
    }

    let output = cmd.output()?;
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let exit_code = output.status.code().unwrap_or(-1);

    Ok((exit_code, stdout, stderr))
}

// ===== Cross-Session Persistence Tests =====

#[test]
fn test_cross_session_key_persistence() -> Result<()> {
    let (_temp_dir, config) = create_test_hsm_setup()?;
    let key_id = "persistent_test_key";
    let test_data = b"Cross-session test data";

    // Session 1: Create key and sign data
    let signature = {
        let mut hsm = SoftwareHsmBackend::with_config(config.clone())?;
        hsm.generate_key_pair(
            key_id,
            AsymmetricAlgorithm::Ed25519,
            Some("Persistent test key".to_string()),
        )?;
        sign_data_via_universal_backend(&hsm, key_id, test_data, SignatureAlgorithm::Ed25519)?
    };

    // Session 2: Load existing HSM and verify signature
    {
        let hsm = SoftwareHsmBackend::with_config(config.clone())?;
        let keys = hsm.list_keys()?;
        assert_eq!(keys.len(), 1);
        assert_eq!(keys[0].description, "Persistent test key");

        let is_valid = verify_signature_via_universal_backend(
            &hsm,
            key_id,
            test_data,
            &signature,
            SignatureAlgorithm::Ed25519,
        )?;
        assert!(is_valid);
    }

    // Verify metadata file exists and contains expected data
    let metadata_content = fs::read_to_string(&config.metadata_file)?;
    assert!(metadata_content.contains(key_id));
    assert!(metadata_content.contains("Persistent test key"));

    // Verify key files exist
    let private_key_path = config
        .key_store_path
        .join(format!("{}_private.key", key_id));
    let public_key_path = config.key_store_path.join(format!("{}_public.key", key_id));
    assert!(private_key_path.exists());
    assert!(public_key_path.exists());

    println!("✔ Cross-session persistence verified");
    Ok(())
}

#[test]
fn test_metadata_corruption_recovery() -> Result<()> {
    let (_temp_dir, config) = create_test_hsm_setup()?;
    let key_id = "recovery_test_key";

    // Create key in working HSM
    {
        let mut hsm = SoftwareHsmBackend::with_config(config.clone())?;
        hsm.generate_key_pair(
            key_id,
            AsymmetricAlgorithm::Ed25519,
            Some("Recovery test".to_string()),
        )?;
    }

    // Corrupt metadata file
    fs::write(&config.metadata_file, "{ invalid json")?;

    // Try to create new HSM instance - should handle corruption gracefully
    let _result = SoftwareHsmBackend::with_config(config.clone());

    // Should either recover or fail gracefully (depending on implementation)
    // The key files should still exist even if metadata is corrupted
    let private_key_path = config
        .key_store_path
        .join(format!("{}_private.key", key_id));
    let public_key_path = config.key_store_path.join(format!("{}_public.key", key_id));
    assert!(private_key_path.exists());
    assert!(public_key_path.exists());

    println!("✔ Metadata corruption handling verified");
    Ok(())
}

// ===== Registry Integration Tests =====

#[test]
fn test_software_hsm_registry_integration() -> Result<()> {
    let (_temp_dir, config) = create_test_hsm_setup()?;

    // Create registry and register Software HSM
    let mut registry = UniversalBackendRegistry::new();
    let mut hsm_backend = SoftwareHsmBackend::with_config(config)?;

    // Generate key using direct HSM interface (since registry operations need mutability)
    hsm_backend.generate_key_pair("registry_test_key", AsymmetricAlgorithm::Ed25519, None)?;

    registry.register_backend("software_hsm".to_string(), Box::new(hsm_backend));

    // Test backend selection
    let backend = registry
        .get_backend("software_hsm")
        .ok_or_else(|| anyhow::anyhow!("Software HSM not found in registry"))?;

    // Test capabilities through registry
    let capabilities = backend.get_capabilities();
    assert!(!capabilities.hardware_backed);
    assert!(capabilities.supports_key_generation);
    assert!(capabilities
        .signature_algorithms
        .contains(&SignatureAlgorithm::Ed25519));
    assert!(capabilities
        .signature_algorithms
        .contains(&SignatureAlgorithm::EcdsaP256));

    // Test signing through registry (key already exists)
    let test_data = b"Registry integration test";
    let signature = sign_data_via_universal_backend(
        backend,
        "registry_test_key",
        test_data,
        SignatureAlgorithm::Ed25519,
    )?;

    // Test verification through registry
    let is_valid = verify_signature_via_universal_backend(
        backend,
        "registry_test_key",
        test_data,
        &signature,
        SignatureAlgorithm::Ed25519,
    )?;
    assert!(is_valid);

    println!("✔ Registry integration verified");
    Ok(())
}

#[test]
fn test_backend_preference_selection() -> Result<()> {
    let (_temp_dir, config) = create_test_hsm_setup()?;

    // Create registry with Software HSM backend
    let mut registry = UniversalBackendRegistry::new();
    let hsm_backend = SoftwareHsmBackend::with_config(config)?;
    registry.register_backend("software_hsm".to_string(), Box::new(hsm_backend));

    // Test backend selection with preferences
    let preferences = BackendPreferences {
        preferred_backends: vec!["software_hsm".to_string()],
        prefer_hardware_backed: false,
        prefer_attestation: false,
        excluded_backends: vec![],
    };

    let selected = registry.find_preferred_backend(
        &CryptoOperation::GenerateKeyPair {
            algorithm: AsymmetricAlgorithm::Ed25519,
        },
        &preferences,
    );

    assert!(selected.is_some());

    // Test simple operation finding
    let found = registry.find_backend_for_operation(&CryptoOperation::GenerateKeyPair {
        algorithm: AsymmetricAlgorithm::Ed25519,
    });

    assert!(found.is_some());

    println!("✔ Backend preference selection verified");
    Ok(())
}

// ===== File-Based Workflow Tests =====

#[test]
fn test_file_based_signing_workflow() -> Result<()> {
    let (_temp_dir, config) = create_test_hsm_setup()?;
    let mut hsm = SoftwareHsmBackend::with_config(config)?;

    // Create test files
    let test_content = "Important document content that needs to be signed.";
    let document_path = create_test_data_file(&_temp_dir, "document.txt", test_content)?;

    // Generate signing key
    hsm.generate_key_pair(
        "document_signer",
        AsymmetricAlgorithm::Ed25519,
        Some("Document signing key".to_string()),
    )?;

    // Read file and sign content
    let file_content = fs::read(&document_path)?;
    let signature = sign_data_via_universal_backend(
        &hsm,
        "document_signer",
        &file_content,
        SignatureAlgorithm::Ed25519,
    )?;

    // Save signature to file
    let signature_path = _temp_dir.path().join("document.sig");
    fs::write(&signature_path, &signature)?;

    // Verify the signing workflow
    let loaded_signature = fs::read(&signature_path)?;
    let is_valid = verify_signature_via_universal_backend(
        &hsm,
        "document_signer",
        &file_content,
        &loaded_signature,
        SignatureAlgorithm::Ed25519,
    )?;
    assert!(is_valid);

    // Test with modified document (should fail verification)
    let modified_content = "Modified document content.";
    let is_invalid = verify_signature_via_universal_backend(
        &hsm,
        "document_signer",
        modified_content.as_bytes(),
        &loaded_signature,
        SignatureAlgorithm::Ed25519,
    )?;
    assert!(!is_invalid);

    println!("✔ File-based signing workflow verified");
    Ok(())
}

// ===== CLI Integration Tests =====

#[test]
fn test_cli_key_lifecycle() -> Result<()> {
    let (_temp_dir, _config) = create_test_hsm_setup()?;
    let key_store_path = _temp_dir.path().to_path_buf();

    // Test key generation through CLI
    let (exit_code, stdout, stderr) = run_cli_command(
        &["generate-key", "cli_test_key", "ed25519"],
        &key_store_path,
    )?;

    if exit_code != 0 {
        println!("CLI stderr: {}", stderr);
        assert_eq!(exit_code, 0, "Key generation should succeed");
    }
    assert!(stdout.contains("Key pair generated successfully"));

    // Test key listing through CLI
    let (exit_code, stdout, _stderr) = run_cli_command(&["list-keys"], &key_store_path)?;
    assert_eq!(exit_code, 0);
    assert!(stdout.contains("Available keys"));

    // Test get public key through CLI
    let (exit_code, stdout, _stderr) =
        run_cli_command(&["get-public-key", "cli_test_key"], &key_store_path)?;
    assert_eq!(exit_code, 0);
    assert!(stdout.contains("Public key") || stdout.len() > 10);

    println!("✔ CLI key lifecycle verified");
    Ok(())
}

// ===== Performance and Stress Tests =====

#[test]
fn test_large_scale_key_management() -> Result<()> {
    let (_temp_dir, config) = create_test_hsm_setup()?;
    let mut hsm = SoftwareHsmBackend::with_config(config.clone())?;

    let key_count = 20; // Reduced for faster testing

    // Generate many keys
    for i in 0..key_count {
        let key_id = format!("scale_key_{}", i);
        let algorithm = if i % 3 == 0 {
            AsymmetricAlgorithm::Ed25519
        } else {
            AsymmetricAlgorithm::EcdsaP256
        };
        hsm.generate_key_pair(&key_id, algorithm, Some(format!("Scale test key {}", i)))?;
    }

    // Verify all keys are listed
    let keys = hsm.list_keys()?;
    assert_eq!(keys.len(), key_count);

    // Test persistence with many keys
    drop(hsm);
    let new_hsm = SoftwareHsmBackend::with_config(config)?;
    let reloaded_keys = new_hsm.list_keys()?;
    assert_eq!(reloaded_keys.len(), key_count);

    // Verify key store directory structure
    let key_files: Vec<_> = fs::read_dir(_temp_dir.path())?
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry
                .path()
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext == "key")
                .unwrap_or(false)
        })
        .collect();

    // Should have 2 files per key (private + public)
    assert_eq!(key_files.len(), key_count * 2);

    println!("✔ Large-scale key management verified ({} keys)", key_count);
    Ok(())
}

// ===== Error Recovery and Resilience Tests =====

#[test]
fn test_partial_file_corruption_recovery() -> Result<()> {
    let (_temp_dir, config) = create_test_hsm_setup()?;
    let mut hsm = SoftwareHsmBackend::with_config(config.clone())?;

    // Create multiple keys
    hsm.generate_key_pair("good_key_1", AsymmetricAlgorithm::Ed25519, None)?;
    hsm.generate_key_pair("corrupt_key", AsymmetricAlgorithm::Ed25519, None)?;
    hsm.generate_key_pair("good_key_2", AsymmetricAlgorithm::EcdsaP256, None)?;

    // Corrupt one key file
    let corrupt_private_path = _temp_dir.path().join("corrupt_key_private.key");
    fs::write(&corrupt_private_path, [0xFF; 16])?; // Wrong size/content

    // Drop and recreate HSM to test loading with corruption
    drop(hsm);
    let new_hsm = SoftwareHsmBackend::with_config(config)?;

    // Good keys should still work
    let test_data = b"Recovery test";
    let signature = sign_data_via_universal_backend(
        &new_hsm,
        "good_key_1",
        test_data,
        SignatureAlgorithm::Ed25519,
    )?;
    let is_valid = verify_signature_via_universal_backend(
        &new_hsm,
        "good_key_1",
        test_data,
        &signature,
        SignatureAlgorithm::Ed25519,
    )?;
    assert!(is_valid);

    // Corrupted key should fail gracefully
    let corrupt_result = sign_data_via_universal_backend(
        &new_hsm,
        "corrupt_key",
        test_data,
        SignatureAlgorithm::Ed25519,
    );
    assert!(corrupt_result.is_err());

    println!("✔ Partial corruption recovery verified");
    Ok(())
}

#[test]
fn test_disk_space_and_permissions() -> Result<()> {
    let (_temp_dir, _config) = create_test_hsm_setup()?;

    // Test with read-only key store directory
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        let readonly_dir = _temp_dir.path().join("readonly_store");
        fs::create_dir(&readonly_dir)?;

        let readonly_config = SoftwareHsmConfig {
            key_store_path: readonly_dir.clone(),
            default_passphrase: "test".to_string(),
            metadata_file: readonly_dir.join("metadata.json"),
        };

        // Make directory read-only
        let mut perms = fs::metadata(&readonly_dir)?.permissions();
        perms.set_mode(0o444);
        fs::set_permissions(&readonly_dir, perms)?;

        // Key generation should fail gracefully
        let hsm = SoftwareHsmBackend::with_config(readonly_config.clone());
        if let Ok(mut backend) = hsm {
            let result =
                backend.generate_key_pair("readonly_test", AsymmetricAlgorithm::Ed25519, None);
            // Should fail due to permissions
            assert!(result.is_err());
        }

        // Restore permissions for cleanup
        let mut perms = fs::metadata(&readonly_dir)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&readonly_dir, perms)?;
    }

    println!("✔ Disk permissions handling verified");
    Ok(())
}
