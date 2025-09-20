//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/
//
// Project: trustedge — Privacy and trust at the edge.
//

use assert_cmd::prelude::*;
use base64::Engine;
use ed25519_dalek::{Signer, SigningKey, VerifyingKey};
use predicates::str::contains;
use rand_core::OsRng;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::TempDir;
use trustedge_trst_core::Manifest;

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

fn resign_manifest(archive: &Path, signing_key: &SigningKey, manifest: &Manifest) {
    let signature = signing_key.sign(&manifest.to_canonical_bytes(false).unwrap());
    let signature_prefixed = format!(
        "ed25519:{}",
        base64::engine::general_purpose::STANDARD.encode(signature.to_bytes())
    );
    let updated = manifest.clone().with_signature(signature_prefixed.clone());
    let canonical = updated.to_canonical_bytes(true).unwrap();
    fs::write(archive.join("manifest.json"), canonical).unwrap();
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
        .stderr(contains("signature error"));
}

#[test]
fn acceptance_a2_missing_chunk() {
    let tempdir = TempDir::new().unwrap();
    let (archive, device_pub) = wrap_archive(&tempdir);

    fs::remove_file(archive.join("chunks/00007.bin")).unwrap();

    run_verify(&tempdir, &archive, &device_pub)
        .failure()
        .stderr(contains("missing chunk"));
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
        .stderr(contains("unexpected end"));
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
        .stderr(contains("signature error"));
}

#[test]
fn acceptance_a6_duration_sanity() {
    let tempdir = TempDir::new().unwrap();
    let (archive, device_pub) = wrap_archive(&tempdir);
    let manifest_path = archive.join("manifest.json");
    let manifest_bytes = fs::read(&manifest_path).unwrap();
    let mut manifest: Manifest = serde_json::from_slice(&manifest_bytes).unwrap();

    // Inflate the first segment duration to trip the sanity check while
    // keeping the signature valid by re-signing with the original key.
    if let Some(first) = manifest.segments.first_mut() {
        first.t1 = first.t0 + 5.0;
    }

    let signing_key = decode_signing_key(&tempdir.path().join("device.key"));
    resign_manifest(&archive, &signing_key, &manifest);

    run_verify(&tempdir, &archive, &device_pub)
        .failure()
        .stderr(contains("unexpected end"));
}
