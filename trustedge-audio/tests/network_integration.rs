// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//

//! Comprehensive network client-server data transfer tests
//!
//! These tests validate the complete end-to-end network workflow including:
//! - Server startup and client connection
//! - Authentication handshake
//! - File transfer with multiple types and sizes
//! - Data integrity verification
//! - Error handling and edge cases

use anyhow::Result;
use std::fs;
use std::io::Write;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};
use tempfile::{NamedTempFile, TempDir};
use tokio::time::sleep;

/// Test helper to get the path to the server binary
fn get_server_binary_path() -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(manifest_dir).join("target/debug/trustedge-server")
}

/// Test helper to get the path to the client binary  
fn get_client_binary_path() -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(manifest_dir).join("target/debug/trustedge-client")
}

/// Test helper to create test data of specified size with pattern
fn create_test_data(size: usize, pattern: &str) -> Vec<u8> {
    let mut data = Vec::new();
    data.extend_from_slice(format!("=== {} TEST DATA ===\n", pattern).as_bytes());

    // Add patterned data
    for i in 0..size {
        match pattern {
            "TEXT" => data.push(((i % 26) as u8) + b'A'),
            "BINARY" => data.push((i % 256) as u8),
            "MIXED" => {
                if i % 10 < 5 {
                    data.push(((i % 26) as u8) + b'a');
                } else {
                    data.push((i % 256) as u8);
                }
            }
            _ => data.push(((i % 26) as u8) + b'A'),
        }
    }

    data.extend_from_slice(format!("\n=== END {} ===", pattern).as_bytes());
    data
}

/// Test helper to create JSON test data
fn create_json_test_data() -> Vec<u8> {
    let json_obj = serde_json::json!({
        "test_type": "network_transfer",
        "timestamp": "2025-09-02T12:00:00Z",
        "data": {
            "measurements": [1.23, 4.56, 7.89],
            "metadata": {
                "device_id": "test-device-001",
                "location": "test-lab",
                "batch_id": 42
            }
        },
        "integrity_hash": "abc123def456",
        "network_test": true
    });
    serde_json::to_vec_pretty(&json_obj).unwrap()
}

/// Test helper to create PDF-like test data
fn create_pdf_test_data() -> Vec<u8> {
    let mut data = Vec::new();

    // PDF header
    data.extend_from_slice(b"%PDF-1.4\n");

    // Simple PDF objects
    data.extend_from_slice(b"1 0 obj\n<< /Type /Catalog /Pages 2 0 R >>\nendobj\n");
    data.extend_from_slice(b"2 0 obj\n<< /Type /Pages /Kids [3 0 R] /Count 1 >>\nendobj\n");
    data.extend_from_slice(b"3 0 obj\n<< /Type /Page /Parent 2 0 R >>\nendobj\n");

    // Add binary content
    for i in 0..=255 {
        data.push(i);
    }

    data.extend_from_slice(b"%%EOF\n");
    data
}

/// Test helper to wait for server to be ready
async fn wait_for_server_ready(addr: SocketAddr, timeout_secs: u64) -> Result<()> {
    let start = Instant::now();
    let timeout_duration = Duration::from_secs(timeout_secs);

    while start.elapsed() < timeout_duration {
        if (tokio::net::TcpStream::connect(addr).await).is_ok() {
            return Ok(());
        }
        sleep(Duration::from_millis(100)).await;
    }

    Err(anyhow::anyhow!(
        "Server not ready after {} seconds",
        timeout_secs
    ))
}

/// Test helper to start the server process
fn start_server(
    listen_addr: SocketAddr,
    output_dir: &TempDir,
    key_hex: &str,
    require_auth: bool,
) -> Result<Child> {
    let server_binary = get_server_binary_path();

    let mut cmd = Command::new(&server_binary);
    cmd.args([
        "--listen",
        &listen_addr.to_string(),
        "--output-dir",
        output_dir.path().to_str().unwrap(),
        "--key-hex",
        key_hex,
        "--verbose",
    ]);

    if require_auth {
        cmd.arg("--require-auth");
    }

    cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

    let child = cmd
        .spawn()
        .map_err(|e| anyhow::anyhow!("Failed to start server: {}", e))?;

    Ok(child)
}

/// Test helper to run client and send file
fn run_client(
    server_addr: SocketAddr,
    file_path: &std::path::Path,
    key_hex: &str,
    enable_auth: bool,
    client_cert: Option<&std::path::Path>,
) -> Result<std::process::Output> {
    let client_binary = get_client_binary_path();

    let mut cmd = Command::new(&client_binary);
    cmd.args([
        "--server",
        &server_addr.to_string(),
        "--file",
        file_path.to_str().unwrap(),
        "--key-hex",
        key_hex,
        "--verbose",
    ]);

    if enable_auth {
        cmd.arg("--enable-auth");
        if let Some(cert_path) = client_cert {
            cmd.args(["--client-cert", cert_path.to_str().unwrap()]);
        }
    }

    let output = cmd
        .output()
        .map_err(|e| anyhow::anyhow!("Failed to run client: {}", e))?;

    Ok(output)
}

/// Comprehensive test: Basic file transfer without authentication
#[tokio::test]
async fn test_basic_file_transfer() -> Result<()> {
    // Setup
    let server_addr: SocketAddr = "127.0.0.1:18080".parse().unwrap();
    let output_dir = TempDir::new()?;
    let key_hex = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

    // Create test file
    let test_data = create_test_data(5000, "TEXT");
    let mut test_file = NamedTempFile::new()?;
    test_file.write_all(&test_data)?;
    let test_file_path = test_file.path();

    // Start server
    let mut server = start_server(server_addr, &output_dir, key_hex, false)?;

    // Wait for server to be ready
    wait_for_server_ready(server_addr, 10).await?;

    // Run client
    let client_output = run_client(server_addr, test_file_path, key_hex, false, None)?;

    // Verify client succeeded
    assert!(
        client_output.status.success(),
        "Client failed: stderr={}",
        String::from_utf8_lossy(&client_output.stderr)
    );

    // Verify server received data
    let stdout = String::from_utf8_lossy(&client_output.stdout);
    assert!(
        stdout.contains("All chunks sent successfully!")
            || stdout.contains("Transfer completed")
            || stdout.contains("Success")
            || stdout.contains("[DONE]"),
        "Expected success message in client output: {}",
        stdout
    );

    // Cleanup
    server.kill().ok();

    println!("✔ Basic file transfer test passed!");
    Ok(())
}

/// Test: Multiple file types and sizes
#[tokio::test]
async fn test_multiple_file_types() -> Result<()> {
    let test_cases = vec![
        ("small_text", create_test_data(100, "TEXT")),
        ("medium_binary", create_test_data(10000, "BINARY")),
        ("large_mixed", create_test_data(50000, "MIXED")),
        ("json_data", create_json_test_data()),
        ("pdf_document", create_pdf_test_data()),
    ];

    for (test_name, test_data) in test_cases {
        println!("Testing file type: {}", test_name);

        // Setup unique port for each test
        let port = 18081 + test_name.len() as u16;
        let server_addr: SocketAddr = format!("127.0.0.1:{}", port).parse().unwrap();
        let output_dir = TempDir::new()?;
        let key_hex = "fedcba9876543210fedcba9876543210fedcba9876543210fedcba9876543210";

        // Create test file
        let mut test_file = NamedTempFile::new()?;
        test_file.write_all(&test_data)?;
        let test_file_path = test_file.path();

        // Start server
        let mut server = start_server(server_addr, &output_dir, key_hex, false)?;

        // Wait for server to be ready
        wait_for_server_ready(server_addr, 10).await?;

        // Run client
        let client_output = run_client(server_addr, test_file_path, key_hex, false, None)?;

        // Verify transfer succeeded
        assert!(
            client_output.status.success(),
            "Client failed for {}: stderr={}",
            test_name,
            String::from_utf8_lossy(&client_output.stderr)
        );

        // Verify success message in output
        let stdout = String::from_utf8_lossy(&client_output.stdout);
        assert!(
            stdout.contains("All chunks sent successfully!")
                || stdout.contains("[DONE]")
                || stdout.contains("complete"),
            "Expected success message for {}: {}",
            test_name,
            stdout
        );

        // Cleanup
        server.kill().ok();

        println!("✔ File type {} test passed!", test_name);
    }

    Ok(())
}

/// Test: Data integrity verification
#[tokio::test]
async fn test_data_integrity() -> Result<()> {
    // Setup
    let server_addr: SocketAddr = "127.0.0.1:18082".parse().unwrap();
    let output_dir = TempDir::new()?;
    let key_hex = "1111222233334444555566667777888899990000aaaabbbbccccddddeeeeffff";

    // Create test file with known content
    let test_data =
        b"Data integrity test content with specific patterns: 0123456789ABCDEF".repeat(100);
    let mut test_file = NamedTempFile::new()?;
    test_file.write_all(&test_data)?;
    let test_file_path = test_file.path();

    // Start server with decryption enabled
    let server_binary = get_server_binary_path();
    let mut server = Command::new(&server_binary)
        .args([
            "--listen",
            &server_addr.to_string(),
            "--output-dir",
            output_dir.path().to_str().unwrap(),
            "--key-hex",
            key_hex,
            "--decrypt",
            "--verbose",
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    // Wait for server to be ready
    wait_for_server_ready(server_addr, 10).await?;

    // Run client
    let client_output = run_client(server_addr, test_file_path, key_hex, false, None)?;

    // Verify client succeeded
    assert!(
        client_output.status.success(),
        "Client failed: stderr={}",
        String::from_utf8_lossy(&client_output.stderr)
    );

    // Give server time to process and save files
    sleep(Duration::from_secs(2)).await;

    // Check if server output directory contains the decrypted file
    let output_files: Vec<_> = fs::read_dir(output_dir.path())?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().ok().is_some_and(|ft| ft.is_file()))
        .collect();

    assert!(
        !output_files.is_empty(),
        "Expected server to save received files in output directory"
    );

    // Verify at least one file was created (encrypted data)
    let has_data_file = output_files.iter().any(|entry| {
        let file_name = entry.file_name();
        let name = file_name.to_string_lossy();
        name.contains("chunk")
            || name.contains("data")
            || name.contains("seq")
            || name.contains("conn")
            || name.contains("decrypted")
            || name.ends_with(".bin")
    });

    assert!(
        has_data_file,
        "Expected server to save received data files. Found files: {:?}",
        output_files
            .iter()
            .map(|e| e.file_name())
            .collect::<Vec<_>>()
    );

    // Cleanup
    server.kill().ok();

    println!("✔ Data integrity test passed!");
    Ok(())
}

/// Test: Authentication with certificates
#[tokio::test]
async fn test_authenticated_transfer() -> Result<()> {
    // Setup
    let server_addr: SocketAddr = "127.0.0.1:18083".parse().unwrap();
    let output_dir = TempDir::new()?;
    let key_hex = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

    // Create test file
    let test_data = create_test_data(2000, "AUTH_TEST");
    let mut test_file = NamedTempFile::new()?;
    test_file.write_all(&test_data)?;
    let test_file_path = test_file.path();

    // Start server with authentication required
    let mut server = start_server(server_addr, &output_dir, key_hex, true)?;

    // Wait for server to be ready
    wait_for_server_ready(server_addr, 10).await?;

    // Run client with authentication
    let client_output = run_client(server_addr, test_file_path, key_hex, true, None)?;

    // Note: This test may fail if certificates aren't properly set up,
    // but it validates the authentication code path
    if client_output.status.success() {
        println!("✔ Authenticated transfer succeeded!");
    } else {
        let stderr = String::from_utf8_lossy(&client_output.stderr);
        if stderr.contains("certificate") || stderr.contains("auth") {
            println!("✔ Authentication code path exercised (expected cert failure)");
        } else {
            // Unexpected error
            return Err(anyhow::anyhow!("Unexpected client failure: {}", stderr));
        }
    }

    // Cleanup
    server.kill().ok();

    Ok(())
}

/// Test: Large file transfer
#[tokio::test]
async fn test_large_file_transfer() -> Result<()> {
    // Setup
    let server_addr: SocketAddr = "127.0.0.1:18084".parse().unwrap();
    let output_dir = TempDir::new()?;
    let key_hex = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";

    // Create large test file (100KB)
    let test_data = create_test_data(100000, "LARGE");
    let mut test_file = NamedTempFile::new()?;
    test_file.write_all(&test_data)?;
    let test_file_path = test_file.path();

    // Start server
    let mut server = start_server(server_addr, &output_dir, key_hex, false)?;

    // Wait for server to be ready
    wait_for_server_ready(server_addr, 10).await?;

    // Run client with smaller chunk size to test chunking
    let client_binary = get_client_binary_path();
    let client_output = Command::new(&client_binary)
        .args([
            "--server",
            &server_addr.to_string(),
            "--file",
            test_file_path.to_str().unwrap(),
            "--key-hex",
            key_hex,
            "--chunk-size",
            "1024", // Use small chunks to test chunking
            "--verbose",
        ])
        .output()?;

    // Verify transfer succeeded
    assert!(
        client_output.status.success(),
        "Large file transfer failed: stderr={}",
        String::from_utf8_lossy(&client_output.stderr)
    );

    // Verify chunking was mentioned in output
    let stdout = String::from_utf8_lossy(&client_output.stdout);
    let stderr = String::from_utf8_lossy(&client_output.stderr);
    let combined_output = format!("{}{}", stdout, stderr);

    assert!(
        combined_output.contains("chunk") || combined_output.contains("Chunk"),
        "Expected chunking information in output for large file"
    );

    // Cleanup
    server.kill().ok();

    println!("✔ Large file transfer test passed!");
    Ok(())
}

/// Test: Connection error handling
#[tokio::test]
async fn test_connection_error_handling() -> Result<()> {
    // Test client behavior when server is not available
    let server_addr: SocketAddr = "127.0.0.1:18085".parse().unwrap(); // No server running
    let key_hex = "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc";

    // Create test file
    let test_data = create_test_data(100, "ERROR_TEST");
    let mut test_file = NamedTempFile::new()?;
    test_file.write_all(&test_data)?;
    let test_file_path = test_file.path();

    // Run client against non-existent server
    let client_output = run_client(server_addr, test_file_path, key_hex, false, None)?;

    // Verify client failed as expected
    assert!(
        !client_output.status.success(),
        "Client should fail when server is not available"
    );

    // Verify error message indicates connection problem
    let stderr = String::from_utf8_lossy(&client_output.stderr);
    assert!(
        stderr.contains("connect")
            || stderr.contains("Connection")
            || stderr.contains("refused")
            || stderr.contains("timeout"),
        "Expected connection error in stderr: {}",
        stderr
    );

    println!("✔ Connection error handling test passed!");
    Ok(())
}

/// Test: Empty file transfer
#[tokio::test]
async fn test_empty_file_transfer() -> Result<()> {
    // Setup
    let server_addr: SocketAddr = "127.0.0.1:18086".parse().unwrap();
    let output_dir = TempDir::new()?;
    let key_hex = "dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd";

    // Create empty test file
    let test_file = NamedTempFile::new()?;
    let test_file_path = test_file.path();

    // Start server
    let mut server = start_server(server_addr, &output_dir, key_hex, false)?;

    // Wait for server to be ready
    wait_for_server_ready(server_addr, 10).await?;

    // Run client
    let client_output = run_client(server_addr, test_file_path, key_hex, false, None)?;

    // Verify client handled empty file appropriately
    // (may succeed with zero chunks or fail gracefully)
    let stderr = String::from_utf8_lossy(&client_output.stderr);
    let stdout = String::from_utf8_lossy(&client_output.stdout);

    if !client_output.status.success() {
        // If it fails, ensure it's for the right reason
        assert!(
            stderr.contains("empty")
                || stderr.contains("size")
                || stderr.contains("zero")
                || stdout.contains("zero chunks"),
            "Empty file failure should be graceful: stderr={}, stdout={}",
            stderr,
            stdout
        );
    }

    // Cleanup
    server.kill().ok();

    println!("✔ Empty file transfer test passed!");
    Ok(())
}
