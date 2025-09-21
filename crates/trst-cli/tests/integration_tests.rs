//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/
//
// Project: trustedge — Privacy and trust at the edge.
//

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_wrap_and_verify_basic_workflow() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();

    // Create test input file
    let input_file = temp_path.join("test-input.bin");
    fs::write(&input_file, b"Hello, TrustEdge P0 Implementation!").unwrap();

    // Create output archive path
    let output_archive = temp_path.join("test-archive.trst");

    // Run wrap command
    let mut cmd = Command::cargo_bin("trst").unwrap();
    cmd.arg("wrap")
        .arg("--profile").arg("cam.video")
        .arg("--in").arg(&input_file)
        .arg("--out").arg(&output_archive)
        .arg("--chunk-size").arg("1048576")
        .arg("--chunk-seconds").arg("2.0")
        .arg("--fps").arg("30")
        .current_dir(temp_path);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Archive:"))
        .stdout(predicate::str::contains("Signature:"))
        .stdout(predicate::str::contains("Segments: 1"))
        .stdout(predicate::str::contains("Generated device key: device.key"))
        .stdout(predicate::str::contains("Generated device pub: device.pub"));

    // Check that files were created
    assert!(output_archive.exists());
    assert!(output_archive.join("manifest.json").exists());
    assert!(output_archive.join("chunks").join("00000.bin").exists());
    assert!(output_archive.join("signatures").join("manifest.sig").exists());
    assert!(temp_path.join("device.key").exists());
    assert!(temp_path.join("device.pub").exists());

    // Read the generated device public key
    let device_pub = fs::read_to_string(temp_path.join("device.pub")).unwrap();
    let device_pub = device_pub.trim();

    // Run verify command
    let mut verify_cmd = Command::cargo_bin("trst").unwrap();
    verify_cmd.arg("verify")
        .arg(&output_archive)
        .arg("--device-pub").arg(device_pub)
        .current_dir(temp_path);

    verify_cmd.assert()
        .success()
        .stdout(predicate::str::contains("Signature: PASS"))
        .stdout(predicate::str::contains("Continuity: PASS"))
        .stdout(predicate::str::contains("Segments: 1"));
}

#[test]
fn test_wrap_with_existing_device_key() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();

    // Create test input file
    let input_file = temp_path.join("test-input.bin");
    fs::write(&input_file, b"Test with existing key").unwrap();

    // Create device key file
    let device_key_file = temp_path.join("my-device.key");
    fs::write(&device_key_file, "ed25519:bz45Wwv6bA3XzesTxVt3IaKxWk8iC2MrcMS1+dDHQRs=\n").unwrap();

    // Create output archive path
    let output_archive = temp_path.join("test-archive.trst");

    // Run wrap command with existing key
    let mut cmd = Command::cargo_bin("trst").unwrap();
    cmd.arg("wrap")
        .arg("--profile").arg("cam.video")
        .arg("--in").arg(&input_file)
        .arg("--out").arg(&output_archive)
        .arg("--device-key").arg(&device_key_file)
        .current_dir(temp_path);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Archive:"))
        .stdout(predicate::str::contains("Signature:"))
        .stdout(predicate::str::contains("Segments: 1"))
        .stdout(predicate::str::contains("Generated device key").not());

    // The corresponding public key for the test secret key
    let expected_pub_key = "ed25519:fFdywmF53dtS8H34c8x4lfoLsp83trzL7N/x6Rb/xlI=";

    // Run verify command
    let mut verify_cmd = Command::cargo_bin("trst").unwrap();
    verify_cmd.arg("verify")
        .arg(&output_archive)
        .arg("--device-pub").arg(expected_pub_key)
        .current_dir(temp_path);

    verify_cmd.assert()
        .success()
        .stdout(predicate::str::contains("Signature: PASS"))
        .stdout(predicate::str::contains("Continuity: PASS"));
}

#[test]
fn test_verify_with_wrong_public_key() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();

    // Create test input file and archive first
    let input_file = temp_path.join("test-input.bin");
    fs::write(&input_file, b"Test verification failure").unwrap();

    let output_archive = temp_path.join("test-archive.trst");

    // Run wrap command
    let mut cmd = Command::cargo_bin("trst").unwrap();
    cmd.arg("wrap")
        .arg("--profile").arg("cam.video")
        .arg("--in").arg(&input_file)
        .arg("--out").arg(&output_archive)
        .current_dir(temp_path);

    cmd.assert().success();

    // Try to verify with wrong public key
    let wrong_pub_key = "ed25519:AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=";

    let mut verify_cmd = Command::cargo_bin("trst").unwrap();
    verify_cmd.arg("verify")
        .arg(&output_archive)
        .arg("--device-pub").arg(wrong_pub_key)
        .current_dir(temp_path);

    verify_cmd.assert()
        .failure()
        .stdout(predicate::str::contains("Signature: FAIL"))
        .stdout(predicate::str::contains("Continuity: SKIP"));
}

#[test]
fn test_wrap_nonexistent_input_file() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();

    let nonexistent_file = temp_path.join("does-not-exist.bin");
    let output_archive = temp_path.join("test-archive.trst");

    let mut cmd = Command::cargo_bin("trst").unwrap();
    cmd.arg("wrap")
        .arg("--profile").arg("cam.video")
        .arg("--in").arg(&nonexistent_file)
        .arg("--out").arg(&output_archive)
        .current_dir(temp_path);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Failed to read input file"));
}

#[test]
fn test_verify_nonexistent_archive() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();

    let nonexistent_archive = temp_path.join("does-not-exist.trst");
    let pub_key = "ed25519:tQ8g8XOQSKfRFBEzHzQcGsNWz9t4pf04sJMCPElcRZ0=";

    let mut cmd = Command::cargo_bin("trst").unwrap();
    cmd.arg("verify")
        .arg(&nonexistent_archive)
        .arg("--device-pub").arg(pub_key)
        .current_dir(temp_path);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("error"));
}

#[test]
fn test_invalid_output_directory_name() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();

    let input_file = temp_path.join("test-input.bin");
    fs::write(&input_file, b"Test invalid output name").unwrap();

    let invalid_output = temp_path.join("invalid-name-without-trst-extension");

    let mut cmd = Command::cargo_bin("trst").unwrap();
    cmd.arg("wrap")
        .arg("--profile").arg("cam.video")
        .arg("--in").arg(&input_file)
        .arg("--out").arg(&invalid_output)
        .current_dir(temp_path);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Output directory must end with .trst"));
}