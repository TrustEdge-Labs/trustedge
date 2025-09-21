//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/
//
// Project: trustedge — Privacy and trust at the edge.
//

use assert_cmd::Command;
use serde_json::Value;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

fn get_trst_binary() -> Command {
    Command::cargo_bin("trst").unwrap()
}

fn create_test_archive_and_keys() -> (TempDir, PathBuf, String) {
    let temp_dir = TempDir::new().unwrap();
    let input_file = temp_dir.path().join("test_input.txt");
    let archive_path = temp_dir.path().join("test_archive.trst");

    // Create a larger test input file to ensure multiple chunks
    let large_data: Vec<u8> = (0..8192).map(|i| (i % 256) as u8).collect();
    fs::write(&input_file, large_data).unwrap();

    // Create the archive using the wrap command, setting working directory to temp_dir
    let output = get_trst_binary()
        .current_dir(temp_dir.path())
        .args([
            "wrap",
            "--in",
            input_file.to_str().unwrap(),
            "--out",
            archive_path.to_str().unwrap(),
            "--chunk-size",
            "2048",
        ])
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "Failed to create test archive: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Read the generated public key (created in temp_dir due to current_dir setting)
    let device_pub_path = temp_dir.path().join("device.pub");
    let device_pub = fs::read_to_string(device_pub_path)
        .unwrap()
        .trim()
        .to_string();

    (temp_dir, archive_path, device_pub)
}

#[test]
fn test_verify_happy_path_json() {
    let (_temp_dir, archive_path, device_pub) = create_test_archive_and_keys();

    let output = get_trst_binary()
        .args([
            "verify",
            archive_path.to_str().unwrap(),
            "--device-pub",
            &device_pub,
            "--json",
        ])
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(0), "Expected exit code 0");

    let json_output = String::from_utf8(output.stdout).unwrap();
    let json: Value = serde_json::from_str(&json_output).expect("Invalid JSON output");

    assert_eq!(json["signature"], "pass");
    assert_eq!(json["continuity"], "pass");
    assert!(json["segments"].as_u64().unwrap() > 0);
    assert!(json["verify_time_ms"].as_u64().unwrap() > 0);
    assert_eq!(json["profile"], "cam.video");
}

#[test]
fn test_verify_happy_path_human() {
    let (_temp_dir, archive_path, device_pub) = create_test_archive_and_keys();

    let output = get_trst_binary()
        .args([
            "verify",
            archive_path.to_str().unwrap(),
            "--device-pub",
            &device_pub,
        ])
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(0), "Expected exit code 0");

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Signature: PASS"));
    assert!(stdout.contains("Continuity: PASS"));
    assert!(stdout.contains("Segments:"));
}

#[test]
fn test_verify_wrong_key_json() {
    let (_temp_dir, archive_path, _device_pub) = create_test_archive_and_keys();
    let wrong_key = "ed25519:AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=";

    let output = get_trst_binary()
        .args([
            "verify",
            archive_path.to_str().unwrap(),
            "--device-pub",
            wrong_key,
            "--json",
        ])
        .output()
        .unwrap();

    assert_eq!(
        output.status.code(),
        Some(10),
        "Expected exit code 10 for signature failure"
    );

    let json_output = String::from_utf8(output.stdout).unwrap();
    let json: Value = serde_json::from_str(&json_output).expect("Invalid JSON output");

    assert_eq!(json["signature"], "fail");
    assert_eq!(json["continuity"], "skip");
    assert!(json["error"]
        .as_str()
        .unwrap()
        .contains("Signature verification"));
}

#[test]
fn test_verify_missing_archive() {
    let temp_dir = TempDir::new().unwrap();
    let missing_archive = temp_dir.path().join("nonexistent.trst");
    let fake_key = "ed25519:AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=";

    let output = get_trst_binary()
        .args([
            "verify",
            missing_archive.to_str().unwrap(),
            "--device-pub",
            fake_key,
            "--json",
        ])
        .output()
        .unwrap();

    assert_eq!(
        output.status.code(),
        Some(12),
        "Expected exit code 12 for IO error"
    );

    let json_output = String::from_utf8(output.stdout).unwrap();
    let json: Value = serde_json::from_str(&json_output).expect("Invalid JSON output");

    assert_eq!(json["signature"], "unknown");
    assert_eq!(json["continuity"], "unknown");
    assert!(json["error"]
        .as_str()
        .unwrap()
        .contains("Failed to read archive"));
}

#[test]
fn test_verify_missing_chunk_file() {
    let (_temp_dir, archive_path, device_pub) = create_test_archive_and_keys();

    // Delete a chunk file to simulate a gap
    let chunks_dir = archive_path.join("chunks");
    if let Ok(entries) = fs::read_dir(&chunks_dir) {
        let mut chunk_files: Vec<_> = entries
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .filter(|path| path.extension().is_some_and(|ext| ext == "bin"))
            .collect();

        if chunk_files.len() > 1 {
            // Sort and remove the middle chunk file
            chunk_files.sort();
            if let Some(middle_chunk) = chunk_files.get(chunk_files.len() / 2) {
                fs::remove_file(middle_chunk).unwrap();
            }
        }
    }

    let output = get_trst_binary()
        .args([
            "verify",
            archive_path.to_str().unwrap(),
            "--device-pub",
            &device_pub,
            "--json",
        ])
        .output()
        .unwrap();

    assert_eq!(
        output.status.code(),
        Some(12),
        "Expected exit code 12 for IO error when chunk file is missing"
    );

    let json_output = String::from_utf8(output.stdout).unwrap();
    let json: Value = serde_json::from_str(&json_output).expect("Invalid JSON output");

    assert_eq!(json["signature"], "unknown");
    assert_eq!(json["continuity"], "unknown");
    assert!(json["error"].as_str().unwrap().contains("Missing chunk file"));
}

#[test]
fn test_verify_invalid_json_manifest() {
    let temp_dir = TempDir::new().unwrap();
    let archive_path = temp_dir.path().join("broken_archive.trst");

    // Create a basic archive structure with invalid JSON
    fs::create_dir_all(&archive_path).unwrap();
    fs::create_dir_all(archive_path.join("chunks")).unwrap();
    fs::create_dir_all(archive_path.join("signatures")).unwrap();

    // Write invalid JSON to manifest
    fs::write(archive_path.join("manifest.json"), "{invalid json}").unwrap();

    let fake_key = "ed25519:AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=";

    let output = get_trst_binary()
        .args([
            "verify",
            archive_path.to_str().unwrap(),
            "--device-pub",
            fake_key,
            "--json",
        ])
        .output()
        .unwrap();

    assert_eq!(
        output.status.code(),
        Some(12),
        "Expected exit code 12 for schema error"
    );

    let json_output = String::from_utf8(output.stdout).unwrap();
    let json: Value = serde_json::from_str(&json_output).expect("Invalid JSON output");

    assert_eq!(json["signature"], "unknown");
    assert_eq!(json["continuity"], "unknown");
}
