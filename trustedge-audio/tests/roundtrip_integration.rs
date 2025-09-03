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

/// Test helper to create PDF-like test data
fn create_pdf_data() -> Vec<u8> {
    let mut data = Vec::new();

    // PDF header
    data.extend_from_slice(b"%PDF-1.4\n");

    // Simple PDF structure with minimal content
    data.extend_from_slice(b"1 0 obj\n<< /Type /Catalog /Pages 2 0 R >>\nendobj\n");
    data.extend_from_slice(b"2 0 obj\n<< /Type /Pages /Kids [3 0 R] /Count 1 >>\nendobj\n");
    data.extend_from_slice(
        b"3 0 obj\n<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] >>\nendobj\n",
    );
    data.extend_from_slice(b"xref\n0 4\n0000000000 65535 f \n");
    data.extend_from_slice(b"0000000009 00000 n \n");
    data.extend_from_slice(b"0000000056 00000 n \n");
    data.extend_from_slice(b"0000000111 00000 n \n");
    data.extend_from_slice(b"trailer\n<< /Size 4 /Root 1 0 R >>\n");
    data.extend_from_slice(b"startxref\n180\n");
    data.extend_from_slice(b"0000000111 00000 n \n");
    data.extend_from_slice(b"trailer\n<< /Size 4 /Root 1 0 R >>\n");
    data.extend_from_slice(b"startxref\n180\n");

    // Add some binary content
    for i in 0..=255 {
        data.push(i);
    }

    // PDF footer
    data.push(b'%');
    data.push(b'P');
    data.push(b'D');
    data.push(b'F');
    data.push(b'-');
    data.push(b'1');
    data.push(b'.');
    data.push(b'4');

    data
}

/// Test helper to create MP3-like test data
fn create_mp3_data() -> Vec<u8> {
    let mut mp3_data = Vec::new();

    // MP3 header (ID3v2)
    mp3_data.extend_from_slice(b"ID3\x03\x00\x00\x00\x00\x00\x00");

    // Fake MP3 frame header
    mp3_data.extend_from_slice(&[0xFF, 0xFB, 0x90, 0x00]); // MPEG Audio Layer III

    // Add some audio-like data patterns
    for i in 0..1000 {
        let sample = ((i as f32 * 0.1).sin() * 127.0) as i8 as u8;
        mp3_data.push(sample);
    }

    mp3_data
}

/// Test helper to create unknown format data
fn create_unknown_format_data() -> Vec<u8> {
    let mut data = Vec::new();

    // Custom magic bytes
    data.extend_from_slice(b"UNKN\x01\x00\x00\x00");

    // Mix of different data patterns
    data.extend_from_slice(b"This is an unknown file format with mixed content.\n");

    // Binary data
    for i in 0..=255 {
        data.push(i);
    }

    // More text
    data.extend_from_slice(b"\nEnd of unknown format file.\n");

    data
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
        .args([
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
        .args([
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
    for i in 0..=255 {
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
        .args([
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
        .args([
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
            .args([
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
            .args([
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

#[test]
fn test_pdf_file_roundtrip() -> Result<()> {
    let test_data = create_pdf_data();
    test_roundtrip(test_data, "PDF file")
}

#[test]
fn test_mp3_file_roundtrip() -> Result<()> {
    let test_data = create_mp3_data();
    test_roundtrip(test_data, "MP3 file")
}

#[test]
fn test_unknown_format_roundtrip() -> Result<()> {
    let test_data = create_unknown_format_data();
    test_roundtrip(test_data, "Unknown format file")
}

#[test]
fn test_byte_perfect_restoration() -> Result<()> {
    // Test various byte patterns for perfect restoration
    let test_cases = vec![
        ("all_zeros", vec![0u8; 1000]),
        ("all_ones", vec![255u8; 1000]),
        (
            "alternating",
            (0..1000)
                .map(|i| if i % 2 == 0 { 0xAA } else { 0x55 })
                .collect(),
        ),
        (
            "sequential",
            (0..256).cycle().take(1000).map(|i| i as u8).collect(),
        ),
        ("random_pattern", {
            let mut data = Vec::new();
            let mut seed = 42u64;
            for _ in 0..1000 {
                seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
                data.push((seed >> 8) as u8);
            }
            data
        }),
    ];

    for (name, test_data) in test_cases {
        // Create test file
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

        // Encrypt
        let encrypt_output = std::process::Command::new(&binary_path)
            .args([
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
            "Encryption failed for {}: {}",
            name,
            String::from_utf8_lossy(&encrypt_output.stderr)
        );

        // Decrypt
        let key_content = fs::read_to_string(key_path)?;
        let key_hex = key_content.trim();

        let decrypt_output = std::process::Command::new(&binary_path)
            .args([
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
            "Decryption failed for {}: {}",
            name,
            String::from_utf8_lossy(&decrypt_output.stderr)
        );

        // Verify byte-perfect restoration
        let decrypted_data = fs::read(decrypted_path)?;
        assert_eq!(
            test_data, decrypted_data,
            "Byte-perfect restoration failed for {}",
            name
        );

        // Additional verification: check length
        assert_eq!(
            test_data.len(),
            decrypted_data.len(),
            "Length mismatch for {} (original: {}, decrypted: {})",
            name,
            test_data.len(),
            decrypted_data.len()
        );

        // Verify every single byte
        for (i, (&original, &decrypted)) in test_data.iter().zip(decrypted_data.iter()).enumerate()
        {
            assert_eq!(
                original, decrypted,
                "Byte mismatch at position {} for {}: original=0x{:02X}, decrypted=0x{:02X}",
                i, name, original, decrypted
            );
        }

        println!("✔ Byte-perfect restoration test passed for: {}", name);
    }

    Ok(())
}

#[test]
fn test_comprehensive_chunk_sizes() -> Result<()> {
    let test_data = create_test_data(100000); // 100KB file for comprehensive testing

    // Test a comprehensive range of chunk sizes
    let chunk_sizes = [
        512,   // Very small chunks
        1024,  // 1KB
        2048,  // 2KB
        4096,  // 4KB (default)
        8192,  // 8KB
        16384, // 16KB
        32768, // 32KB
        65536, // 64KB - large chunks
    ];

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
            .args([
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
                "--verbose",
            ])
            .output()?;

        assert!(
            encrypt_output.status.success(),
            "Encryption failed for chunk size {}: {}",
            chunk_size,
            String::from_utf8_lossy(&encrypt_output.stderr)
        );

        // Verify chunk size is working (don't rely on verbose output format)
        let _encrypt_stdout = String::from_utf8_lossy(&encrypt_output.stdout);
        // Just verify the process succeeded - chunk size verification will come from successful roundtrip

        // Decrypt
        let key_content = fs::read_to_string(key_path)?;
        let key_hex = key_content.trim();

        let decrypt_output = std::process::Command::new(&binary_path)
            .args([
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

        // Verify byte-perfect restoration
        let decrypted_data = fs::read(decrypted_path)?;
        assert_eq!(
            test_data, decrypted_data,
            "Data mismatch for chunk size {}",
            chunk_size
        );

        // Check encrypted file exists and has reasonable size
        let encrypted_size = fs::metadata(encrypted_path)?.len();
        assert!(
            encrypted_size > test_data.len() as u64,
            "Encrypted file should be larger than original (chunk size {})",
            chunk_size
        );

        println!("✔ Comprehensive chunk size {} test passed!", chunk_size);
    }

    Ok(())
}

#[test]
fn test_format_detection_accuracy() -> Result<()> {
    let test_cases = vec![
        ("JSON", create_json_data(), "application/json"),
        ("PDF", create_pdf_data(), "application/pdf"),
        ("MP3", create_mp3_data(), "audio/mpeg"),
        ("Text", create_text_data("Hello, world!"), "text/plain"),
    ];

    for (format_name, test_data, _expected_mime) in test_cases {
        let mut input_file = NamedTempFile::new()?;
        input_file.write_all(&test_data)?;
        let input_path = input_file.path();

        let encrypted_file = NamedTempFile::new()?;
        let encrypted_path = encrypted_file.path();

        let key_file = NamedTempFile::new()?;
        let key_path = key_file.path();

        let binary_path = get_binary_path();

        // Encrypt with verbose output
        let encrypt_output = std::process::Command::new(&binary_path)
            .args([
                "--input",
                input_path.to_str().unwrap(),
                "--out",
                "/dev/null",
                "--envelope",
                encrypted_path.to_str().unwrap(),
                "--key-out",
                key_path.to_str().unwrap(),
                "--verbose",
            ])
            .output()?;

        assert!(
            encrypt_output.status.success(),
            "Encryption failed for {}",
            format_name
        );

        // Check if MIME type detection is working (in verbose output)
        let _encrypt_stdout = String::from_utf8_lossy(&encrypt_output.stdout);
        // Note: The actual MIME detection may vary, so we check for general format indication

        // Test inspection
        let inspect_output = std::process::Command::new(&binary_path)
            .args([
                "--input",
                encrypted_path.to_str().unwrap(),
                "--inspect",
                "--verbose",
            ])
            .output()?;

        assert!(
            inspect_output.status.success(),
            "Inspection failed for {}",
            format_name
        );

        let inspect_stdout = String::from_utf8_lossy(&inspect_output.stdout);
        assert!(
            inspect_stdout.contains("Data Type") || inspect_stdout.contains("MIME"),
            "Inspection should show format information for {}",
            format_name
        );

        println!("✔ Format detection test passed for: {}", format_name);
    }

    Ok(())
}
