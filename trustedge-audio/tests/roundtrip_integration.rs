// Copyright (c) 2025 John Turner
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//

//! Comprehensive roundtrip encryption/decryption tests
//!
//! These tests validate the complete encrypt/decrypt workflow with real data,
//! ensuring data integrity across different input types and sizes.

use anyhow::Result;
use std::fs;
use std::io::Write;
use tempfile::NamedTempFile;

/// Test helper to create test data of specified size
fn create_test_data(size: usize) -> Vec<u8> {
    (0..size).map(|i| (i % 256) as u8).collect()
}

/// Test helper to create text test data
fn create_text_data(content: &str) -> Vec<u8> {
    content.as_bytes().to_vec()
}

/// Test helper to create JSON test data
fn create_json_data() -> Vec<u8> {
    let json_obj = serde_json::json!({
        "timestamp": "2025-09-02T12:00:00Z",
        "data": {
            "temperature": 23.5,
            "humidity": 45.2,
            "location": "edge-device-001"
        },
        "metadata": {
            "version": "1.0",
            "checksum": "abc123"
        }
    });
    serde_json::to_vec_pretty(&json_obj).unwrap()
}

/// Helper function to find the binary path
fn get_binary_path() -> std::path::PathBuf {
    let release_path = std::env::current_dir()
        .unwrap()
        .join("target/release/trustedge-audio");
    if release_path.exists() {
        release_path
    } else {
        std::env::current_dir()
            .unwrap()
            .join("target/debug/trustedge-audio")
    }
}

/// Helper function to run full encrypt/decrypt roundtrip test
fn test_roundtrip(test_data: Vec<u8>, test_name: &str) -> Result<()> {
    let mut input_file = NamedTempFile::new()?;
    input_file.write_all(&test_data)?;
    let input_path = input_file.path();

    let encrypted_file = NamedTempFile::new()?;
    let encrypted_path = encrypted_file.path();

    let decrypted_file = NamedTempFile::new()?;
    let decrypted_path = decrypted_file.path();

    let key_file = NamedTempFile::new()?;
    let key_path = key_file.path();

    let binary_path = get_binary_path();

    // Encrypt the file
    let encrypt_output = std::process::Command::new(&binary_path)
        .args(&[
            "--input",
            input_path.to_str().unwrap(),
            "--out",
            "/dev/null", // We don't need the plaintext copy during encryption
            "--envelope",
            encrypted_path.to_str().unwrap(),
            "--key-out",
            key_path.to_str().unwrap(),
            "--verbose",
        ])
        .output()?;

    assert!(
        encrypt_output.status.success(),
        "{} encryption failed: {}",
        test_name,
        String::from_utf8_lossy(&encrypt_output.stderr)
    );

    // Read the generated key
    let key_content = fs::read_to_string(key_path)?;
    let key_hex = key_content.trim();

    // Decrypt the file
    let decrypt_output = std::process::Command::new(&binary_path)
        .args(&[
            "--decrypt",
            "--input",
            encrypted_path.to_str().unwrap(),
            "--out",
            decrypted_path.to_str().unwrap(),
            "--key-hex",
            key_hex,
            "--verbose",
        ])
        .output()?;

    assert!(
        decrypt_output.status.success(),
        "{} decryption failed: {}",
        test_name,
        String::from_utf8_lossy(&decrypt_output.stderr)
    );

    // Verify the decrypted data matches original
    let decrypted_data = fs::read(decrypted_path)?;
    assert_eq!(
        test_data, decrypted_data,
        "{} roundtrip data mismatch",
        test_name
    );

    println!("✔ {} roundtrip test passed!", test_name);
    Ok(())
}

#[test]
fn test_small_file_roundtrip() -> Result<()> {
    let test_data = create_test_data(1024); // 1KB
    test_roundtrip(test_data, "Small file (1KB)")
}

#[test]
fn test_medium_file_roundtrip() -> Result<()> {
    let test_data = create_test_data(100 * 1024); // 100KB
    test_roundtrip(test_data, "Medium file (100KB)")
}

#[test]
fn test_text_file_roundtrip() -> Result<()> {
    let test_content = "Hello, TrustEdge!\nThis is a test text file.\nIt contains multiple lines.\n■ Security test ■\n";
    let test_data = create_text_data(test_content);
    test_roundtrip(test_data, "Text file")
}

#[test]
fn test_json_file_roundtrip() -> Result<()> {
    let test_data = create_json_data();
    test_roundtrip(test_data, "JSON file")
}

#[test]
fn test_empty_file_roundtrip() -> Result<()> {
    let test_data = Vec::new();
    test_roundtrip(test_data, "Empty file")
}

#[test]
fn test_binary_file_roundtrip() -> Result<()> {
    // Create binary test data with various byte patterns
    let mut test_data = Vec::new();

    // Add various byte patterns
    test_data.extend_from_slice(&[0x00, 0xFF, 0xAA, 0x55]); // Edge cases
    test_data.extend_from_slice(b"\x00\x01\x02\x03\x04\x05\x06\x07"); // Sequential
    test_data.extend_from_slice(&[0x80, 0x81, 0x82, 0x83]); // High bit set

    // Add some random-looking data
    for i in 0..256 {
        test_data.push((i ^ 0xAA) as u8);
    }

    test_roundtrip(test_data, "Binary file")
}

#[test]
fn test_inspect_encrypted_file() -> Result<()> {
    // Create test file and encrypt it
    let test_data = create_text_data("Test data for inspection");

    let mut input_file = NamedTempFile::new()?;
    input_file.write_all(&test_data)?;
    let input_path = input_file.path();

    let encrypted_file = NamedTempFile::new()?;
    let encrypted_path = encrypted_file.path();

    let key_file = NamedTempFile::new()?;
    let key_path = key_file.path();

    let binary_path = get_binary_path();

    // Encrypt
    let encrypt_output = std::process::Command::new(&binary_path)
        .args(&[
            "--input",
            input_path.to_str().unwrap(),
            "--out",
            "/dev/null",
            "--envelope",
            encrypted_path.to_str().unwrap(),
            "--key-out",
            key_path.to_str().unwrap(),
        ])
        .output()?;

    assert!(
        encrypt_output.status.success(),
        "Encryption failed: {}",
        String::from_utf8_lossy(&encrypt_output.stderr)
    );

    // Inspect the encrypted file
    let inspect_output = std::process::Command::new(&binary_path)
        .args(&[
            "--input",
            encrypted_path.to_str().unwrap(),
            "--inspect",
            "--verbose",
        ])
        .output()?;

    assert!(
        inspect_output.status.success(),
        "Inspection failed: {}",
        String::from_utf8_lossy(&inspect_output.stderr)
    );

    let inspect_stdout = String::from_utf8_lossy(&inspect_output.stdout);

    // Verify inspection output contains expected information
    assert!(
        inspect_stdout.contains("TrustEdge Archive Information")
            || inspect_stdout.contains("File")
            || inspect_stdout.contains("MIME")
            || inspect_stdout.contains("Data Type"),
        "Inspection output should contain metadata information. Got: {}",
        inspect_stdout
    );

    println!("✔ Inspect encrypted file test passed!");
    println!("Inspection output:\n{}", inspect_stdout);
    Ok(())
}

#[test]
fn test_multiple_chunk_sizes() -> Result<()> {
    let test_data = create_test_data(25000); // 25KB, should span multiple chunks

    // Test different chunk sizes
    let chunk_sizes = [1024, 4096, 8192];

    for &chunk_size in &chunk_sizes {
        let mut input_file = NamedTempFile::new()?;
        input_file.write_all(&test_data)?;
        let input_path = input_file.path();

        let encrypted_file = NamedTempFile::new()?;
        let encrypted_path = encrypted_file.path();

        let decrypted_file = NamedTempFile::new()?;
        let decrypted_path = decrypted_file.path();

        let key_file = NamedTempFile::new()?;
        let key_path = key_file.path();

        let binary_path = get_binary_path();

        // Encrypt with specific chunk size
        let encrypt_output = std::process::Command::new(&binary_path)
            .args(&[
                "--input",
                input_path.to_str().unwrap(),
                "--out",
                "/dev/null",
                "--envelope",
                encrypted_path.to_str().unwrap(),
                "--key-out",
                key_path.to_str().unwrap(),
                "--chunk",
                &chunk_size.to_string(),
            ])
            .output()?;

        assert!(
            encrypt_output.status.success(),
            "Encryption failed for chunk size {}: {}",
            chunk_size,
            String::from_utf8_lossy(&encrypt_output.stderr)
        );

        // Decrypt
        let key_content = fs::read_to_string(key_path)?;
        let key_hex = key_content.trim();

        let decrypt_output = std::process::Command::new(&binary_path)
            .args(&[
                "--decrypt",
                "--input",
                encrypted_path.to_str().unwrap(),
                "--out",
                decrypted_path.to_str().unwrap(),
                "--key-hex",
                key_hex,
            ])
            .output()?;

        assert!(
            decrypt_output.status.success(),
            "Decryption failed for chunk size {}: {}",
            chunk_size,
            String::from_utf8_lossy(&decrypt_output.stderr)
        );

        // Verify
        let decrypted_data = fs::read(decrypted_path)?;
        assert_eq!(
            test_data, decrypted_data,
            "Roundtrip data mismatch for chunk size {}",
            chunk_size
        );

        println!("✔ Chunk size {} test passed!", chunk_size);
    }

    Ok(())
}
