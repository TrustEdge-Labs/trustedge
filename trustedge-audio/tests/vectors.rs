// tests/vectors.rs
//
// Copyright (c) 2025 John Turner
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//

use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::path::exists;
use std::process::Command;

#[test]
fn golden_envelope_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
    // Fixed inputs for deterministic vector
    // NOTE: Keep these constants stable (first time you run, copy the printed digest below
    // into GOLDEN_TRST_BLAKE3 and commit). If you change format/crypto, rebase the golden.
    let key_hex = "000102030405060708090a0b0c0d0e0f\
                   101112131415161718191a1b1c1d1e1f";
    // Deterministic input bytes
    let input_data = b"TrustEdge test input -- deterministic vector v1";

    let temp = assert_fs::TempDir::new()?;
    let in_file = temp.child("input.bin");
    let out_file = temp.child("restored.bin");
    let trst = temp.child("out.trst");

    in_file.write_binary(input_data)?;

    // Encrypt → produce envelope + restored plaintext
    Command::cargo_bin("trustedge-audio")?
        .args([
            "-i",
            in_file.path().to_str().unwrap(),
            "-o",
            out_file.path().to_str().unwrap(),
            "--chunk",
            "1024",
            "--envelope",
            trst.path().to_str().unwrap(),
            "--key-hex",
            key_hex,
            // keep default random nonce prefix, that's okay for round-trip;
            // golden of .trst will vary unless we add a fixed prefix; see tamper test below
        ])
        .assert()
        .success();

    // Decrypt → round-trip again (from envelope only)
    let out2 = temp.child("restored2.bin");
    Command::cargo_bin("trustedge-audio")?
        .args([
            "--decrypt",
            "-i",
            trst.path().to_str().unwrap(),
            "-o",
            out2.path().to_str().unwrap(),
            "--key-hex",
            key_hex,
        ])
        .assert()
        .success();

    // Compare plaintexts to input
    out_file.assert(exists());
    out2.assert(exists());

    let input_bytes = std::fs::read(in_file.path())?;
    let output_bytes = std::fs::read(out_file.path())?;
    let output2_bytes = std::fs::read(out2.path())?;

    assert_eq!(input_bytes, output_bytes, "First round-trip failed");
    assert_eq!(input_bytes, output2_bytes, "Second round-trip failed");

    // (Optional) lock a golden for the envelope if you switch to a fixed nonce prefix in tests
    // let trst_bytes = std::fs::read(trst.path())?;
    // let digest = blake3_hex(&trst_bytes);
    // eprintln!("test-only .trst BLAKE3 = {}", digest);
    // const GOLDEN_TRST_BLAKE3: &str = "<fill me after first run>";
    // assert_eq!(digest, GOLDEN_TRST_BLAKE3);

    temp.close()?;
    Ok(())
}

#[test]
fn tamper_fails_on_manifest_change() -> Result<(), Box<dyn std::error::Error>> {
    let key_hex = "000102030405060708090a0b0c0d0e0f\
                   101112131415161718191a1b1c1d1e1f";
    let input_data = b"tamper manifest test";

    let temp = assert_fs::TempDir::new()?;
    let in_file = temp.child("input.bin");
    let out_file = temp.child("restored.bin");
    let trst = temp.child("out.trst");

    in_file.write_binary(input_data)?;

    // Encrypt → envelope
    Command::cargo_bin("trustedge-audio")?
        .args([
            "-i",
            in_file.path().to_str().unwrap(),
            "-o",
            out_file.path().to_str().unwrap(),
            "--chunk",
            "512",
            "--envelope",
            trst.path().to_str().unwrap(),
            "--key-hex",
            key_hex,
            "--no-plaintext",
        ])
        .assert()
        .success();

    // Tamper: flip one byte somewhere in the envelope (best effort)
    // Simple approach: read file, flip a byte in the middle, write back
    let mut bytes = std::fs::read(trst.path())?;
    if !bytes.is_empty() {
        let mid = bytes.len() / 2;
        bytes[mid] ^= 0x01;
        std::fs::write(trst.path(), &bytes)?;
    }

    // Decrypt should FAIL
    let out2 = temp.child("restored2.bin");
    Command::cargo_bin("trustedge-audio")?
        .args([
            "--decrypt",
            "-i",
            trst.path().to_str().unwrap(),
            "-o",
            out2.path().to_str().unwrap(),
            "--key-hex",
            key_hex,
        ])
        .assert()
        .failure();

    temp.close()?;
    Ok(())
}
