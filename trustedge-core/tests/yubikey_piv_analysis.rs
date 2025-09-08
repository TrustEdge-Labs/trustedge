/*
 * Copyright (c) 2025 TRUSTEDGE LABS LLC
 * This source code is subject to the terms of the Mozilla Public License, v. 2.0.
 * If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
 *
 * Project: trustedge — Privacy and trust at the edge.
 */

//! YubiKey PIV Slot Analysis - Phase 2 Diagnostic
//!
//! These tests analyze what's actually present in PIV slots to fix key enumeration.

use anyhow::Result;

mod yubikey_hardware_detection;
use yubikey_hardware_detection::YubikeyTestEnvironment;

#[cfg(feature = "yubikey")]
use trustedge_core::{
    backends::YubiKeyBackend, CryptoOperation, CryptoResult, SignatureAlgorithm, UniversalBackend,
};

/// Test what objects are actually present in PIV slots
#[tokio::test]
#[ignore] // Requires real hardware
async fn test_piv_slot_analysis() -> Result<()> {
    let env = YubikeyTestEnvironment::detect();

    if !env.has_hardware() {
        println!("⚠ Skipping real hardware test - {}", env.description());
        return Ok(());
    }

    #[cfg(feature = "yubikey")]
    {
        println!("● Analyzing PIV slot contents...");

        let config = env.get_config().expect("Hardware should have config");
        let backend = YubiKeyBackend::with_config(config)?;

        // Test the standard PIV slots
        let piv_slots = [
            ("9a", "PIV Authentication"),
            ("9c", "Digital Signature"),
            ("9d", "Key Management"),
            ("9e", "Card Authentication"),
        ];

        for (slot_id, slot_name) in &piv_slots {
            println!("\n● Analyzing PIV slot {} ({}):", slot_id, slot_name);

            // Test if we can get public key from this slot
            let pubkey_op = CryptoOperation::GetPublicKey;

            if backend.supports_operation(&pubkey_op) {
                match backend.perform_operation(slot_id, pubkey_op) {
                    Ok(CryptoResult::PublicKey(pubkey)) => {
                        println!("  ✔ Public key found: {} bytes", pubkey.len());
                        println!("  ✔ This slot has a usable key pair!");
                    }
                    Ok(other) => {
                        println!("  ● Unexpected result: {:?}", other);
                    }
                    Err(e) => {
                        println!("  ● No public key accessible: {}", e);
                    }
                }
            } else {
                println!("  ● GetPublicKey operation not supported");
            }

            // Test if we can perform signing operations
            let sign_op = CryptoOperation::Sign {
                data: format!("Test data for slot {}", slot_id).into_bytes(),
                algorithm: SignatureAlgorithm::EcdsaP256,
            };

            if backend.supports_operation(&sign_op) {
                println!("  ● Signing operation supported (would need PIN)");
            } else {
                println!("  ● Signing operation not supported");
            }
        }

        // Test the backend's key listing vs what we found
        println!("\n● Testing backend key enumeration:");

        match backend.list_keys() {
            Ok(keys) => {
                println!("  • Backend reports {} keys", keys.len());
                for (i, key) in keys.iter().enumerate() {
                    println!(
                        "    Key {}: ID={:02x?}, Description={}",
                        i + 1,
                        key.key_id,
                        key.description
                    );
                }
            }
            Err(e) => {
                println!("  • Backend key enumeration failed: {}", e);
            }
        }

        println!("\n✔ PIV slot analysis complete");
    }

    Ok(())
}

/// Test direct PKCS#11 object enumeration
#[tokio::test]
#[ignore] // Requires real hardware
async fn test_direct_pkcs11_enumeration() -> Result<()> {
    let env = YubikeyTestEnvironment::detect();

    if !env.has_hardware() {
        println!("⚠ Skipping real hardware test - {}", env.description());
        return Ok(());
    }

    println!("● Testing direct PKCS#11 object enumeration...");

    // Use external pkcs11-tool to show what's actually there
    let output = std::process::Command::new("pkcs11-tool")
        .arg("--list-objects")
        .arg("--module")
        .arg("/usr/lib/x86_64-linux-gnu/opensc-pkcs11.so")
        .output()
        .expect("Failed to run pkcs11-tool");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("✔ PKCS#11 objects found:");

        // Parse and categorize the objects
        let mut public_keys = 0;
        let mut certificates = 0;
        let mut private_keys = 0;
        let mut data_objects = 0;

        for line in stdout.lines() {
            if line.contains("Public Key Object") {
                public_keys += 1;
                println!("  ✔ Public Key: {}", line.trim());
            } else if line.contains("Certificate Object") {
                certificates += 1;
                println!("  ✔ Certificate: {}", line.trim());
            } else if line.contains("Private Key Object") {
                private_keys += 1;
                println!("  ✔ Private Key: {}", line.trim());
            } else if line.contains("Data object") {
                data_objects += 1;
            } else if line.trim().starts_with("ID:") || line.trim().starts_with("label:") {
                println!("    {}", line.trim());
            }
        }

        println!("\n● Summary:");
        println!("  • Public Keys: {}", public_keys);
        println!("  • Certificates: {}", certificates);
        println!("  • Private Keys: {}", private_keys);
        println!("  • Data Objects: {}", data_objects);

        if public_keys > 0 && private_keys == 0 {
            println!("\n⚠ ISSUE IDENTIFIED:");
            println!("  • YubiKey has public keys but no visible private key objects");
            println!("  • This is why backend.list_keys() returns 0 keys");
            println!("  • Backend only searches for private key objects");
            println!("  • PIV cards often don't expose private keys directly");
        }
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("● pkcs11-tool failed: {}", stderr);
    }

    Ok(())
}

/// Test certificate-based key discovery
#[tokio::test]
#[ignore] // Requires real hardware
async fn test_certificate_based_discovery() -> Result<()> {
    let env = YubikeyTestEnvironment::detect();

    if !env.has_hardware() {
        println!("⚠ Skipping real hardware test - {}", env.description());
        return Ok(());
    }

    println!("● Testing certificate-based key discovery...");

    // Use pkcs11-tool to find certificates specifically
    let output = std::process::Command::new("pkcs11-tool")
        .arg("--list-objects")
        .arg("--type")
        .arg("cert")
        .arg("--module")
        .arg("/usr/lib/x86_64-linux-gnu/opensc-pkcs11.so")
        .output()
        .expect("Failed to run pkcs11-tool");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("✔ Certificates found:");

        let mut cert_count = 0;
        let mut current_cert_id: Option<String> = None;
        let mut current_cert_label: Option<String> = None;

        for line in stdout.lines() {
            if line.contains("Certificate Object") {
                cert_count += 1;

                // Print previous certificate info if we have it
                if let (Some(id), Some(label)) =
                    (current_cert_id.as_ref(), current_cert_label.as_ref())
                {
                    println!("  • Certificate: ID={}, Label={}", id, label);

                    // Map certificate ID to PIV slot
                    match id.as_str() {
                        "02" => println!("    → PIV Slot 9c (Digital Signature)"),
                        "01" => println!("    → PIV Slot 9a (PIV Authentication)"),
                        "03" => println!("    → PIV Slot 9d (Key Management)"),
                        "04" => println!("    → PIV Slot 9e (Card Authentication)"),
                        _ => println!("    → Unknown PIV mapping"),
                    }
                }

                current_cert_id = None;
                current_cert_label = None;
            } else if line.trim().starts_with("ID:") {
                current_cert_id = Some(line.trim().replace("ID:", "").trim().to_string());
            } else if line.trim().starts_with("label:") {
                current_cert_label = Some(line.trim().replace("label:", "").trim().to_string());
            }
        }

        // Print the last certificate
        if let (Some(id), Some(label)) = (current_cert_id, current_cert_label) {
            println!("  • Certificate: ID={}, Label={}", id, label);
            match id.as_str() {
                "02" => println!("    → PIV Slot 9c (Digital Signature) ✔ YOUR KEY IS HERE!"),
                "01" => println!("    → PIV Slot 9a (PIV Authentication)"),
                "03" => println!("    → PIV Slot 9d (Key Management)"),
                "04" => println!("    → PIV Slot 9e (Card Authentication)"),
                _ => println!("    → Unknown PIV mapping"),
            }
        }

        println!("\n● Certificate Summary:");
        println!("  • Found {} certificates", cert_count);
        if cert_count > 0 {
            println!("  ✔ This proves keys are present in PIV slots");
            println!("  ✔ Backend should enumerate certificates, not just private keys");
        }
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("● Certificate enumeration failed: {}", stderr);
    }

    Ok(())
}
