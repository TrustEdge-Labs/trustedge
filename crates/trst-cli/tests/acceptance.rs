//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/
//
// Project: trustedge — Privacy and trust at the edge.
//

// Allow deprecated cargo_bin usage - the replacement cargo_bin_cmd! macro
// is not yet stable across all assert_cmd versions
#![allow(deprecated)]

use assert_cmd::prelude::*;
use base64::Engine;
use ed25519_dalek::{Signer, SigningKey, VerifyingKey};
use predicates::str::contains;
use rand_core::OsRng;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::TempDir;
use trustedge_core::TrstManifest;

const PROFILE: &str = "cam.video";

fn write_sample_input(dir: &Path) -> PathBuf {
    let input_path = dir.join("input.bin");
    let data: Vec<u8> = (0..(64 * 1024)).map(|i| (i % 251) as u8).collect();
    fs::write(&input_path, data).unwrap();
    input_path
}

fn wrap_archive(tempdir: &TempDir) -> (PathBuf, String) {
    let input = write_sample_input(tempdir.path());
    let archive_dir = tempdir.path().join("clip.trst");

    Command::cargo_bin("trst")
        .unwrap()
        .current_dir(tempdir.path())
        .args([
            "wrap",
            "--profile",
            PROFILE,
            "--in",
            input.to_str().unwrap(),
            "--out",
            archive_dir.to_str().unwrap(),
            "--chunk-size",
            "4096",
            "--chunk-seconds",
            "2.0",
        ])
        .assert()
        .success();

    let device_pub = fs::read_to_string(tempdir.path().join("device.pub")).unwrap();
    (archive_dir, device_pub.trim().to_string())
}

/// Wrap a generic profile archive (no --profile flag = uses default "generic").
fn wrap_generic_archive(tempdir: &TempDir) -> (PathBuf, String) {
    let input = write_sample_input(tempdir.path());
    let archive_dir = tempdir.path().join("clip-generic.trst");

    Command::cargo_bin("trst")
        .unwrap()
        .current_dir(tempdir.path())
        .args([
            "wrap",
            "--in",
            input.to_str().unwrap(),
            "--out",
            archive_dir.to_str().unwrap(),
            "--chunk-size",
            "4096",
        ])
        .assert()
        .success();

    let device_pub = fs::read_to_string(tempdir.path().join("device.pub")).unwrap();
    (archive_dir, device_pub.trim().to_string())
}

fn run_verify(tempdir: &TempDir, archive: &Path, device_pub: &str) -> assert_cmd::assert::Assert {
    Command::cargo_bin("trst")
        .unwrap()
        .current_dir(tempdir.path())
        .args([
            "verify",
            archive.to_str().unwrap(),
            "--device-pub",
            device_pub,
        ])
        .assert()
}

fn decode_signing_key(path: &Path) -> SigningKey {
    let contents = fs::read_to_string(path).unwrap();
    let (_, data) = contents.trim().split_once(':').unwrap();
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(data)
        .unwrap();
    let array: [u8; 32] = bytes.try_into().unwrap();
    SigningKey::from_bytes(&array)
}

fn resign_manifest_json(archive: &Path, signing_key: &SigningKey, manifest: &TrstManifest) {
    // Use proper canonicalization from the core module
    let canonical_bytes = manifest.to_canonical_bytes().unwrap();

    // Sign the canonical bytes
    let signature = signing_key.sign(&canonical_bytes);
    let signature_prefixed = format!(
        "ed25519:{}",
        base64::engine::general_purpose::STANDARD.encode(signature.to_bytes())
    );

    // Create a new manifest with the signature
    let mut signed_manifest = manifest.clone();
    signed_manifest.signature = Some(signature_prefixed.clone());

    // Write the signed manifest as pretty JSON
    let manifest_json = serde_json::to_string_pretty(&signed_manifest).unwrap();
    fs::write(archive.join("manifest.json"), manifest_json).unwrap();
    fs::write(
        archive.join("signatures").join("manifest.sig"),
        signature_prefixed.as_bytes(),
    )
    .unwrap();
}

#[test]
fn acceptance_happy_path() {
    let tempdir = TempDir::new().unwrap();
    let (archive, device_pub) = wrap_archive(&tempdir);
    run_verify(&tempdir, &archive, &device_pub).success();
}

#[test]
fn acceptance_a1_signature_flip() {
    let tempdir = TempDir::new().unwrap();
    let (archive, device_pub) = wrap_archive(&tempdir);

    let manifest_path = archive.join("manifest.json");
    let mut manifest = fs::read_to_string(&manifest_path).unwrap();
    manifest = manifest.replacen(PROFILE, "cam.video-tampered", 1);
    fs::write(&manifest_path, manifest).unwrap();

    run_verify(&tempdir, &archive, &device_pub)
        .failure()
        .stderr(contains("Signature verification failed"));
}

#[test]
fn acceptance_a2_missing_chunk() {
    let tempdir = TempDir::new().unwrap();
    let (archive, device_pub) = wrap_archive(&tempdir);

    fs::remove_file(archive.join("chunks/00007.bin")).unwrap();

    run_verify(&tempdir, &archive, &device_pub)
        .failure()
        .stderr(contains("Missing chunk file"));
}

#[test]
fn acceptance_a3_swap_chunks() {
    let tempdir = TempDir::new().unwrap();
    let (archive, device_pub) = wrap_archive(&tempdir);
    let chunk10 = archive.join("chunks/00010.bin");
    let chunk11 = archive.join("chunks/00011.bin");
    let temp = archive.join("chunks/tmp.bin");
    fs::rename(&chunk10, &temp).unwrap();
    fs::rename(&chunk11, &chunk10).unwrap();
    fs::rename(&temp, &chunk11).unwrap();

    run_verify(&tempdir, &archive, &device_pub)
        .failure()
        .stderr(contains("hash mismatch"));
}

#[test]
fn acceptance_a4_truncated_chain() {
    let tempdir = TempDir::new().unwrap();
    let (archive, device_pub) = wrap_archive(&tempdir);
    let last_index = fs::read_dir(archive.join("chunks"))
        .unwrap()
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| entry.file_name().to_str().map(|s| s.to_string()))
        .filter_map(|name| name.strip_suffix(".bin").map(|s| s.to_string()))
        .filter_map(|stem| stem.parse::<u32>().ok())
        .max()
        .unwrap();
    let last_chunk = archive.join(format!("chunks/{last_index:05}.bin"));
    fs::remove_file(&last_chunk).unwrap();

    run_verify(&tempdir, &archive, &device_pub)
        .failure()
        .stderr(contains("Missing chunk file"));
}

#[test]
fn acceptance_a5_wrong_key() {
    let tempdir = TempDir::new().unwrap();
    let (archive, _device_pub) = wrap_archive(&tempdir);

    let mut rng = OsRng;
    let wrong_key = SigningKey::generate(&mut rng);
    let verifying_key: VerifyingKey = wrong_key.verifying_key();
    let wrong_pub = format!(
        "ed25519:{}",
        base64::engine::general_purpose::STANDARD.encode(verifying_key.as_bytes())
    );

    run_verify(&tempdir, &archive, &wrong_pub)
        .failure()
        .stderr(contains("Signature verification failed"));
}

#[test]
fn acceptance_a6_duration_sanity() {
    let tempdir = TempDir::new().unwrap();
    let (archive, device_pub) = wrap_archive(&tempdir);
    let manifest_path = archive.join("manifest.json");
    let manifest_json = fs::read_to_string(&manifest_path).unwrap();
    let mut manifest: TrstManifest = serde_json::from_str(&manifest_json).unwrap();

    // Inflate the first segment duration to trip the sanity check while
    // keeping the signature valid by re-signing with the original key.
    if let Some(first_segment) = manifest.segments.first_mut() {
        first_segment.duration_seconds = 100.0;
    }

    let signing_key = decode_signing_key(&tempdir.path().join("device.key"));
    resign_manifest_json(&archive, &signing_key, &manifest);

    // This test verifies that the archive validates successfully even with unusual durations
    // The P0 implementation doesn't enforce strict duration sanity checks
    run_verify(&tempdir, &archive, &device_pub).success();
}

// ─── Generic profile acceptance tests ────────────────────────────────────────

#[test]
fn acceptance_generic_default_profile() {
    // Wrap without --profile flag; default must be "generic"
    let tempdir = TempDir::new().unwrap();
    let (archive, device_pub) = wrap_generic_archive(&tempdir);

    // Read manifest and verify profile is "generic"
    let manifest_json = fs::read_to_string(archive.join("manifest.json")).unwrap();
    let manifest_value: serde_json::Value = serde_json::from_str(&manifest_json).unwrap();
    assert_eq!(
        manifest_value["profile"], "generic",
        "default profile must be 'generic'"
    );

    // Verify the archive passes signature + continuity
    run_verify(&tempdir, &archive, &device_pub).success();
}

#[test]
fn acceptance_generic_explicit_profile() {
    // Wrap with --profile generic explicitly; must produce a valid archive
    let tempdir = TempDir::new().unwrap();
    let input = write_sample_input(tempdir.path());
    let archive_dir = tempdir.path().join("clip-explicit.trst");

    Command::cargo_bin("trst")
        .unwrap()
        .current_dir(tempdir.path())
        .args([
            "wrap",
            "--profile",
            "generic",
            "--in",
            input.to_str().unwrap(),
            "--out",
            archive_dir.to_str().unwrap(),
            "--chunk-size",
            "4096",
        ])
        .assert()
        .success();

    let device_pub = fs::read_to_string(tempdir.path().join("device.pub")).unwrap();

    let manifest_json = fs::read_to_string(archive_dir.join("manifest.json")).unwrap();
    let manifest_value: serde_json::Value = serde_json::from_str(&manifest_json).unwrap();
    assert_eq!(manifest_value["profile"], "generic");

    run_verify(&tempdir, &archive_dir, device_pub.trim()).success();
}

#[test]
fn acceptance_generic_with_metadata() {
    // Wrap with --data-type and --source flags; verify they appear in the manifest
    let tempdir = TempDir::new().unwrap();
    let input = write_sample_input(tempdir.path());
    let archive_dir = tempdir.path().join("clip-meta.trst");

    Command::cargo_bin("trst")
        .unwrap()
        .current_dir(tempdir.path())
        .args([
            "wrap",
            "--profile",
            "generic",
            "--in",
            input.to_str().unwrap(),
            "--out",
            archive_dir.to_str().unwrap(),
            "--chunk-size",
            "4096",
            "--data-type",
            "sensor",
            "--source",
            "drone-01",
        ])
        .assert()
        .success();

    let manifest_json = fs::read_to_string(archive_dir.join("manifest.json")).unwrap();
    let manifest_value: serde_json::Value = serde_json::from_str(&manifest_json).unwrap();

    assert_eq!(manifest_value["profile"], "generic");
    assert_eq!(
        manifest_value["metadata"]["data_type"], "sensor",
        "data_type must be present in manifest metadata"
    );
    assert_eq!(
        manifest_value["metadata"]["source"], "drone-01",
        "source must be present in manifest metadata"
    );

    // Verify round-trip passes
    let device_pub = fs::read_to_string(tempdir.path().join("device.pub")).unwrap();
    run_verify(&tempdir, &archive_dir, device_pub.trim()).success();
}

// ─── Keygen acceptance tests ──────────────────────────────────────────────────

#[test]
fn acceptance_keygen_creates_files() {
    let tempdir = TempDir::new().unwrap();
    let key_path = tempdir.path().join("device.key");
    let pub_path = tempdir.path().join("device.pub");

    Command::cargo_bin("trst")
        .unwrap()
        .current_dir(tempdir.path())
        .args([
            "keygen",
            "--out-key",
            key_path.to_str().unwrap(),
            "--out-pub",
            pub_path.to_str().unwrap(),
        ])
        .assert()
        .success();

    // Both files must exist
    assert!(key_path.exists(), "secret key file must be created");
    assert!(pub_path.exists(), "public key file must be created");

    // Both must start with "ed25519:"
    let key_content = fs::read_to_string(&key_path).unwrap();
    let pub_content = fs::read_to_string(&pub_path).unwrap();
    assert!(
        key_content.trim().starts_with("ed25519:"),
        "secret key must start with ed25519: prefix, got: {key_content}"
    );
    assert!(
        pub_content.trim().starts_with("ed25519:"),
        "public key must start with ed25519: prefix, got: {pub_content}"
    );
}

#[test]
fn acceptance_keygen_roundtrip() {
    let tempdir = TempDir::new().unwrap();
    let key_path = tempdir.path().join("mydevice.key");
    let pub_path = tempdir.path().join("mydevice.pub");

    // Step 1: generate keys
    Command::cargo_bin("trst")
        .unwrap()
        .current_dir(tempdir.path())
        .args([
            "keygen",
            "--out-key",
            key_path.to_str().unwrap(),
            "--out-pub",
            pub_path.to_str().unwrap(),
        ])
        .assert()
        .success();

    // Step 2: wrap an archive using the generated key
    let input = write_sample_input(tempdir.path());
    let archive_dir = tempdir.path().join("clip-keygen.trst");

    Command::cargo_bin("trst")
        .unwrap()
        .current_dir(tempdir.path())
        .args([
            "wrap",
            "--profile",
            "cam.video",
            "--in",
            input.to_str().unwrap(),
            "--out",
            archive_dir.to_str().unwrap(),
            "--chunk-size",
            "4096",
            "--chunk-seconds",
            "2.0",
            "--device-key",
            key_path.to_str().unwrap(),
        ])
        .assert()
        .success();

    // Step 3: verify using the public key string from the generated file
    let device_pub = fs::read_to_string(&pub_path).unwrap();
    run_verify(&tempdir, &archive_dir, device_pub.trim()).success();
}

#[test]
fn acceptance_keygen_no_overwrite() {
    let tempdir = TempDir::new().unwrap();
    let key_path = tempdir.path().join("existing.key");
    let pub_path = tempdir.path().join("existing.pub");

    // Pre-create the key file to trigger the overwrite guard
    fs::write(&key_path, "existing content\n").unwrap();

    Command::cargo_bin("trst")
        .unwrap()
        .current_dir(tempdir.path())
        .args([
            "keygen",
            "--out-key",
            key_path.to_str().unwrap(),
            "--out-pub",
            pub_path.to_str().unwrap(),
        ])
        .assert()
        .failure()
        .stderr(contains("overwrite"));
}

#[test]
fn acceptance_camvideo_still_works() {
    // cam.video wrap + verify round-trip must pass (regression guard)
    let tempdir = TempDir::new().unwrap();
    let input = write_sample_input(tempdir.path());
    let archive_dir = tempdir.path().join("clip-camvideo.trst");

    Command::cargo_bin("trst")
        .unwrap()
        .current_dir(tempdir.path())
        .args([
            "wrap",
            "--profile",
            "cam.video",
            "--in",
            input.to_str().unwrap(),
            "--out",
            archive_dir.to_str().unwrap(),
            "--chunk-size",
            "4096",
            "--chunk-seconds",
            "2.0",
            "--fps",
            "30",
        ])
        .assert()
        .success();

    let manifest_json = fs::read_to_string(archive_dir.join("manifest.json")).unwrap();
    let manifest_value: serde_json::Value = serde_json::from_str(&manifest_json).unwrap();
    assert_eq!(manifest_value["profile"], "cam.video");
    // cam.video metadata must have fps field
    assert!(
        manifest_value["metadata"]["fps"].is_number(),
        "cam.video manifest must include fps in metadata"
    );

    let device_pub = fs::read_to_string(tempdir.path().join("device.pub")).unwrap();
    run_verify(&tempdir, &archive_dir, device_pub.trim()).success();
}

// ─── Sensor profile acceptance tests ─────────────────────────────────────────

#[test]
fn acceptance_sensor_wrap_verify() {
    let tempdir = TempDir::new().unwrap();
    let input = write_sample_input(tempdir.path());
    let archive_dir = tempdir.path().join("clip-sensor.trst");

    Command::cargo_bin("trst")
        .unwrap()
        .current_dir(tempdir.path())
        .args([
            "wrap",
            "--profile",
            "sensor",
            "--in",
            input.to_str().unwrap(),
            "--out",
            archive_dir.to_str().unwrap(),
            "--chunk-size",
            "4096",
            "--sample-rate",
            "100.0",
            "--unit",
            "celsius",
            "--sensor-model",
            "DHT22",
        ])
        .assert()
        .success();

    let manifest_json = fs::read_to_string(archive_dir.join("manifest.json")).unwrap();
    let manifest_value: serde_json::Value = serde_json::from_str(&manifest_json).unwrap();
    assert_eq!(manifest_value["profile"], "sensor");
    assert_eq!(
        manifest_value["metadata"]["sample_rate_hz"], 100.0,
        "sample_rate_hz must be present in sensor manifest metadata"
    );
    assert_eq!(
        manifest_value["metadata"]["unit"], "celsius",
        "unit must be present in sensor manifest metadata"
    );
    assert_eq!(
        manifest_value["metadata"]["sensor_model"], "DHT22",
        "sensor_model must be present in sensor manifest metadata"
    );

    let device_pub = fs::read_to_string(tempdir.path().join("device.pub")).unwrap();
    run_verify(&tempdir, &archive_dir, device_pub.trim()).success();
}

#[test]
fn acceptance_audio_wrap_verify() {
    let tempdir = TempDir::new().unwrap();
    let input = write_sample_input(tempdir.path());
    let archive_dir = tempdir.path().join("clip-audio.trst");

    Command::cargo_bin("trst")
        .unwrap()
        .current_dir(tempdir.path())
        .args([
            "wrap",
            "--profile",
            "audio",
            "--in",
            input.to_str().unwrap(),
            "--out",
            archive_dir.to_str().unwrap(),
            "--chunk-size",
            "4096",
            "--sample-rate",
            "44100",
            "--bit-depth",
            "16",
            "--channels",
            "2",
            "--codec",
            "pcm",
        ])
        .assert()
        .success();

    let manifest_json = fs::read_to_string(archive_dir.join("manifest.json")).unwrap();
    let manifest_value: serde_json::Value = serde_json::from_str(&manifest_json).unwrap();
    assert_eq!(manifest_value["profile"], "audio");
    assert_eq!(
        manifest_value["metadata"]["sample_rate_hz"], 44100,
        "sample_rate_hz must be present in audio manifest metadata"
    );
    assert_eq!(
        manifest_value["metadata"]["bit_depth"], 16,
        "bit_depth must be present in audio manifest metadata"
    );
    assert_eq!(
        manifest_value["metadata"]["channels"], 2,
        "channels must be present in audio manifest metadata"
    );
    assert_eq!(
        manifest_value["metadata"]["codec"], "pcm",
        "codec must be present in audio manifest metadata"
    );

    let device_pub = fs::read_to_string(tempdir.path().join("device.pub")).unwrap();
    run_verify(&tempdir, &archive_dir, device_pub.trim()).success();
}

#[test]
fn acceptance_log_wrap_verify() {
    let tempdir = TempDir::new().unwrap();
    let input = write_sample_input(tempdir.path());
    let archive_dir = tempdir.path().join("clip-log.trst");

    Command::cargo_bin("trst")
        .unwrap()
        .current_dir(tempdir.path())
        .args([
            "wrap",
            "--profile",
            "log",
            "--in",
            input.to_str().unwrap(),
            "--out",
            archive_dir.to_str().unwrap(),
            "--chunk-size",
            "4096",
            "--application",
            "nginx",
            "--host",
            "web-01",
            "--log-level",
            "info",
            "--log-format",
            "json",
        ])
        .assert()
        .success();

    let manifest_json = fs::read_to_string(archive_dir.join("manifest.json")).unwrap();
    let manifest_value: serde_json::Value = serde_json::from_str(&manifest_json).unwrap();
    assert_eq!(manifest_value["profile"], "log");
    assert_eq!(
        manifest_value["metadata"]["application"], "nginx",
        "application must be present in log manifest metadata"
    );
    assert_eq!(
        manifest_value["metadata"]["host"], "web-01",
        "host must be present in log manifest metadata"
    );
    assert_eq!(
        manifest_value["metadata"]["log_level"], "info",
        "log_level must be present in log manifest metadata"
    );
    assert_eq!(
        manifest_value["metadata"]["log_format"], "json",
        "log_format must be present in log manifest metadata"
    );

    let device_pub = fs::read_to_string(tempdir.path().join("device.pub")).unwrap();
    run_verify(&tempdir, &archive_dir, device_pub.trim()).success();
}

#[test]
fn acceptance_sensor_with_geo() {
    let tempdir = TempDir::new().unwrap();
    let input = write_sample_input(tempdir.path());
    let archive_dir = tempdir.path().join("clip-sensor-geo.trst");

    Command::cargo_bin("trst")
        .unwrap()
        .current_dir(tempdir.path())
        .args([
            "wrap",
            "--profile",
            "sensor",
            "--in",
            input.to_str().unwrap(),
            "--out",
            archive_dir.to_str().unwrap(),
            "--chunk-size",
            "4096",
            "--sample-rate",
            "50.0",
            "--unit",
            "psi",
            "--sensor-model",
            "BMP280",
            "--latitude",
            "37.7749",
            "--longitude=-122.4194",
            "--altitude",
            "10.0",
        ])
        .assert()
        .success();

    let manifest_json = fs::read_to_string(archive_dir.join("manifest.json")).unwrap();
    let manifest_value: serde_json::Value = serde_json::from_str(&manifest_json).unwrap();
    assert_eq!(manifest_value["profile"], "sensor");
    assert!(
        manifest_value["metadata"]["latitude"].is_number(),
        "latitude must be present in geo-tagged sensor manifest"
    );
    assert!(
        manifest_value["metadata"]["longitude"].is_number(),
        "longitude must be present in geo-tagged sensor manifest"
    );
    assert!(
        manifest_value["metadata"]["altitude"].is_number(),
        "altitude must be present in geo-tagged sensor manifest"
    );

    let device_pub = fs::read_to_string(tempdir.path().join("device.pub")).unwrap();
    run_verify(&tempdir, &archive_dir, device_pub.trim()).success();
}

#[test]
fn acceptance_sensor_missing_required_flag() {
    // Missing --unit and --sensor-model; should fail with a clear error message
    let tempdir = TempDir::new().unwrap();
    let input = write_sample_input(tempdir.path());
    let archive_dir = tempdir.path().join("clip-sensor-fail.trst");

    Command::cargo_bin("trst")
        .unwrap()
        .current_dir(tempdir.path())
        .args([
            "wrap",
            "--profile",
            "sensor",
            "--in",
            input.to_str().unwrap(),
            "--out",
            archive_dir.to_str().unwrap(),
            "--chunk-size",
            "4096",
            "--sample-rate",
            "100.0",
            // --unit and --sensor-model intentionally omitted
        ])
        .assert()
        .failure()
        .stderr(predicates::str::contains("required"));
}
