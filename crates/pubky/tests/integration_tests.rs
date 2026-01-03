// Copyright (c) 2025 TRUSTEDGE LABS LLC
// MPL-2.0: https://mozilla.org/MPL/2.0/
// Project: trustedge — Privacy and trust at the edge.

//! Integration tests for trustedge-pubky CLI and library functionality
//!
//! These tests validate real functionality without mocks or fake data.

// Allow deprecated cargo_bin usage - the replacement cargo_bin_cmd! macro
// is not yet stable across all assert_cmd versions
#![allow(deprecated)]

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

/// Test CLI binary exists and shows help
#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("trustedge-pubky").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("TrustEdge Pubky CLI"))
        .stdout(predicate::str::contains("generate"))
        .stdout(predicate::str::contains("encrypt"))
        .stdout(predicate::str::contains("decrypt"));
}

/// Test key generation produces valid output
#[test]
fn test_key_generation() {
    let temp_dir = TempDir::new().unwrap();
    let key_file = temp_dir.path().join("test_key.txt");

    let mut cmd = Command::cargo_bin("trustedge-pubky").unwrap();
    cmd.arg("generate")
        .arg("--output")
        .arg(&key_file)
        .assert()
        .success()
        .stdout(predicate::str::contains("Generated Pubky Identity"))
        .stdout(predicate::str::contains("Private key saved"));

    // Verify key file was created and has correct format
    assert!(key_file.exists());
    let key_content = fs::read_to_string(&key_file).unwrap();

    // Should be 64 hex characters (32 bytes)
    assert_eq!(key_content.trim().len(), 64);
    assert!(key_content.trim().chars().all(|c| c.is_ascii_hexdigit()));
}

/// Test key generation with seed produces deterministic results
#[test]
fn test_key_generation_with_seed() {
    let seed = "1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";

    // Generate key twice with same seed
    let mut cmd1 = Command::cargo_bin("trustedge-pubky").unwrap();
    let output1 = cmd1
        .arg("generate")
        .arg("--seed")
        .arg(seed)
        .arg("--id-only")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let mut cmd2 = Command::cargo_bin("trustedge-pubky").unwrap();
    let output2 = cmd2
        .arg("generate")
        .arg("--seed")
        .arg(seed)
        .arg("--id-only")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    // Should produce identical Pubky IDs
    assert_eq!(output1, output2);

    // Should be 64 hex characters
    let pubky_id = String::from_utf8(output1).unwrap();
    assert_eq!(pubky_id.trim().len(), 64);
    assert!(pubky_id.trim().chars().all(|c| c.is_ascii_hexdigit()));
}

/// Test invalid seed handling
#[test]
fn test_invalid_seed_error() {
    let mut cmd = Command::cargo_bin("trustedge-pubky").unwrap();
    cmd.arg("generate")
        .arg("--seed")
        .arg("invalid_seed")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid seed length"));
}

/// Test encryption with invalid recipient ID
#[test]
fn test_encrypt_invalid_recipient() {
    let temp_dir = TempDir::new().unwrap();
    let input_file = temp_dir.path().join("input.txt");
    let output_file = temp_dir.path().join("output.trst");

    fs::write(&input_file, "test data").unwrap();

    let mut cmd = Command::cargo_bin("trustedge-pubky").unwrap();
    cmd.arg("encrypt")
        .arg("--input")
        .arg(&input_file)
        .arg("--output")
        .arg(&output_file)
        .arg("--recipient")
        .arg("invalid_recipient")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Invalid recipient Pubky ID length",
        ));
}

/// Test encryption with valid format but non-existent recipient
#[test]
fn test_encrypt_nonexistent_recipient() {
    let temp_dir = TempDir::new().unwrap();
    let input_file = temp_dir.path().join("input.txt");
    let output_file = temp_dir.path().join("output.trst");

    fs::write(&input_file, "test data").unwrap();

    // Valid format but non-existent recipient
    let fake_recipient = "1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";

    let mut cmd = Command::cargo_bin("trustedge-pubky").unwrap();
    cmd.arg("encrypt")
        .arg("--input")
        .arg(&input_file)
        .arg("--output")
        .arg(&output_file)
        .arg("--recipient")
        .arg(fake_recipient)
        .assert()
        .failure()
        .stderr(predicate::str::contains("Failed to encrypt data"));
}

/// Test resolve command with invalid Pubky ID
#[test]
fn test_resolve_invalid_id() {
    let mut cmd = Command::cargo_bin("trustedge-pubky").unwrap();
    cmd.arg("resolve")
        .arg("invalid_id")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Failed to resolve Pubky ID"));
}

/// Test decrypt with missing key file
#[test]
fn test_decrypt_missing_key() {
    let temp_dir = TempDir::new().unwrap();
    let input_file = temp_dir.path().join("input.trst");
    let output_file = temp_dir.path().join("output.txt");
    let key_file = temp_dir.path().join("nonexistent_key.txt");

    // Create dummy envelope file
    fs::write(&input_file, "dummy envelope data").unwrap();

    let mut cmd = Command::cargo_bin("trustedge-pubky").unwrap();
    cmd.arg("decrypt")
        .arg("--input")
        .arg(&input_file)
        .arg("--output")
        .arg(&output_file)
        .arg("--key")
        .arg(&key_file)
        .assert()
        .failure()
        .stderr(predicate::str::contains("Failed to read private key file"));
}

/// Test decrypt with invalid key format
#[test]
fn test_decrypt_invalid_key_format() {
    let temp_dir = TempDir::new().unwrap();
    let input_file = temp_dir.path().join("input.trst");
    let output_file = temp_dir.path().join("output.txt");
    let key_file = temp_dir.path().join("invalid_key.txt");

    // Create dummy envelope file
    fs::write(&input_file, "dummy envelope data").unwrap();

    // Create invalid key file
    fs::write(&key_file, "invalid_key_content").unwrap();

    let mut cmd = Command::cargo_bin("trustedge-pubky").unwrap();
    cmd.arg("decrypt")
        .arg("--input")
        .arg(&input_file)
        .arg("--output")
        .arg(&output_file)
        .arg("--key")
        .arg(&key_file)
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid private key hex"));
}

/// Test migration command with missing files
#[test]
fn test_migrate_missing_files() {
    let temp_dir = TempDir::new().unwrap();
    let input_file = temp_dir.path().join("nonexistent.trst");
    let output_file = temp_dir.path().join("output.trst");
    let v1_key = temp_dir.path().join("v1_key.txt");
    let pubky_key = temp_dir.path().join("pubky_key.txt");

    let recipient = "1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";

    let mut cmd = Command::cargo_bin("trustedge-pubky").unwrap();
    cmd.arg("migrate")
        .arg("--input")
        .arg(&input_file)
        .arg("--output")
        .arg(&output_file)
        .arg("--recipient")
        .arg(recipient)
        .arg("--v1-key")
        .arg(&v1_key)
        .arg("--pubky-key")
        .arg(&pubky_key)
        .assert()
        .failure()
        .stderr(predicate::str::contains("Failed to read v1 envelope file"));
}

/// Test that generated keys are actually random (not deterministic without seed)
#[test]
fn test_key_generation_randomness() {
    // Generate two keys without seed
    let mut cmd1 = Command::cargo_bin("trustedge-pubky").unwrap();
    let output1 = cmd1
        .arg("generate")
        .arg("--id-only")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let mut cmd2 = Command::cargo_bin("trustedge-pubky").unwrap();
    let output2 = cmd2
        .arg("generate")
        .arg("--id-only")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    // Should produce different Pubky IDs
    assert_ne!(output1, output2);
}

/// Test file I/O error handling
#[test]
fn test_file_io_errors() {
    let temp_dir = TempDir::new().unwrap();
    let nonexistent_input = temp_dir.path().join("nonexistent.txt");
    let output_file = temp_dir.path().join("output.trst");

    let recipient = "1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";

    let mut cmd = Command::cargo_bin("trustedge-pubky").unwrap();
    cmd.arg("encrypt")
        .arg("--input")
        .arg(&nonexistent_input)
        .arg("--output")
        .arg(&output_file)
        .arg("--recipient")
        .arg(recipient)
        .assert()
        .failure()
        .stderr(predicate::str::contains("Failed to read input file"));
}
