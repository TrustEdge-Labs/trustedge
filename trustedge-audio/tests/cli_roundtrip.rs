//
// Copyright (c) 2025 John Turner
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//
use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::fs;
use std::process::Command;
use tempfile::tempdir;

#[test]
fn roundtrip_envelope_matches_input() {
    let dir = tempdir().unwrap();
    let input = dir.path().join("in.bin");
    let out_rt = dir.path().join("roundtrip.bin");
    let env_file = dir.path().join("out.trst");
    let out_restored = dir.path().join("restored.bin");

    // deterministic-ish input
    fs::write(
        &input,
        (0..65536u32)
            .flat_map(|x| x.to_be_bytes())
            .collect::<Vec<_>>(),
    )
    .unwrap();

    // provide a fixed key for repeatability
    let key_hex = "00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff";

    // encrypt → produce envelope + plaintext
    Command::cargo_bin("trustedge-audio")
        .unwrap()
        .args([
            "-i",
            input.to_str().unwrap(),
            "-o",
            out_rt.to_str().unwrap(),
            "--chunk",
            "8192",
            "--envelope",
            env_file.to_str().unwrap(),
            "--key-hex",
            key_hex,
        ])
        .assert()
        .success();

    // decrypt → restore from envelope
    Command::cargo_bin("trustedge-audio")
        .unwrap()
        .args([
            "--decrypt",
            "-i",
            env_file.to_str().unwrap(),
            "-o",
            out_restored.to_str().unwrap(),
            "--key-hex",
            key_hex,
        ])
        .assert()
        .success()
        .stderr(predicate::str::contains("Decrypt complete"));

    let orig = fs::read(&input).unwrap();
    let restored = fs::read(&out_restored).unwrap();
    assert_eq!(orig, restored, "restored plaintext must equal original");
}
