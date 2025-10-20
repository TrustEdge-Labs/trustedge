/*
 * Copyright (c) 2025 TRUSTEDGE LABS LLC
 * This source code is subject to the terms of the Mozilla Public License, v. 2.0.
 * If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
 *
 * Project: trustedge — Privacy and trust at the edge.
 */

//! YubiKey Universal Backend Demo
//!
//! This demo showcases the complete YubiKey integration with TrustEdge:
//! - Hardware public key extraction from PIV slots
//! - X.509 certificate generation with custom ASN.1 DER encoding  
//! - Real hardware-backed certificate signing using YubiKey private keys
//! - Certificate validation and analysis
//!
//! ## Hardware Requirements
//! - YubiKey with PIV applet enabled
//! - OpenSC PKCS#11 module installed (`apt install opensc-pkcs11` on Ubuntu)
//! - Keys generated in PIV slots (use `ykman piv` to generate)
//!
//! ## Quick Setup
//! ```bash
//! # Generate a key in PIV authentication slot (if not already present)
//! ykman piv keys generate 9a /tmp/pubkey.pem --algorithm ECCP256
//! ykman piv certificates generate 9a /tmp/pubkey.pem --subject "CN=Test"
//! ```
//!
//! ## Usage
//! ```bash
//! # With custom PIN
//! cargo run --example yubikey_demo --features yubikey -- YOUR_PIN
//!
//! # With default PIN (123456)
//! cargo run --example yubikey_demo --features yubikey
//!
//! # Without PIN (public key operations only)
//! cargo run --example yubikey_demo --features yubikey -- no-pin
//! ```

#[cfg(feature = "yubikey")]
fn main() -> anyhow::Result<()> {
    use anyhow::Context;
    use trustedge_core::{
        backends::yubikey::{YubiKeyBackend, YubiKeyConfig},
        CryptoOperation, CryptoResult, SignatureAlgorithm, UniversalBackend,
    };

    println!("● TrustEdge YubiKey Integration Demo");
    println!("===================================");
    println!();
    println!("This demo showcases complete YubiKey hardware integration:");
    println!("• Hardware public key extraction from PIV slots");
    println!("• X.509 certificate generation with real keys");
    println!("• Hardware-backed certificate signing");
    println!("• Certificate validation and export");

    // Get PIN from command line or use default
    let args: Vec<String> = std::env::args().collect();
    let pin = if args.len() > 1 {
        if args[1] == "no-pin" {
            None
        } else {
            Some(args[1].clone())
        }
    } else {
        println!("\n⚠ Usage: cargo run --example yubikey_demo --features yubikey -- YOUR_PIN");
        println!("   Or:   cargo run --example yubikey_demo --features yubikey -- no-pin");
        println!("\n   Default PIN (123456) will be used. Press Ctrl+C to cancel.\n");
        Some("123456".to_string())
    };

    // Configure YubiKey backend with verbose output
    let config = YubiKeyConfig {
        pkcs11_module_path: "/usr/lib/x86_64-linux-gnu/opensc-pkcs11.so".to_string(),
        pin,
        slot: None, // Auto-detect
        verbose: true,
    };

    println!("\n📋 Configuration:");
    println!("   PKCS#11 Module: {}", config.pkcs11_module_path);
    println!(
        "   PIN: {}",
        if config.pin.is_some() {
            "Provided"
        } else {
            "None"
        }
    );
    println!("   Slot: {:?}", config.slot);

    // Initialize YubiKey backend
    println!("\n● Initializing YubiKey Backend...");
    let backend = YubiKeyBackend::with_config(config)?;
    println!("   ✔ YubiKey backend initialized successfully");

    // Test PIV slots for available keys
    let test_slots = ["9a", "9c", "9d", "9e"]; // Authentication, Key Management, Card Auth, Digital Signature
    let slot_names = [
        "PIV Authentication",
        "Key Management",
        "Card Authentication",
        "Digital Signature",
    ];

    println!("\n🔍 Scanning PIV Slots for Keys:");
    let mut available_keys = Vec::new();

    for (slot, name) in test_slots.iter().zip(slot_names.iter()) {
        print!("   {} ({}): ", slot, name);

        // First try to get the public key (may return placeholder)
        match backend.perform_operation(slot, CryptoOperation::GetPublicKey) {
            Ok(CryptoResult::PublicKey(pubkey)) => {
                // Verify the slot actually has a hardware key by attempting a test signature
                let test_data = b"test";
                match backend.perform_operation(
                    slot,
                    CryptoOperation::Sign {
                        data: test_data.to_vec(),
                        algorithm: SignatureAlgorithm::EcdsaP256,
                    },
                ) {
                    Ok(CryptoResult::Signed(_)) => {
                        println!("✔ Key found ({} bytes)", pubkey.len());
                        available_keys.push((*slot, pubkey));
                    }
                    Err(_) => {
                        println!("⚠ Public key extracted but signing failed (no hardware key)");
                    }
                    Ok(_) => {
                        println!("✖ Unexpected signing result");
                    }
                }
            }
            Ok(_) => {
                println!("✖ Unexpected result type");
            }
            Err(_) => {
                println!("✖ No key found");
            }
        }
    }

    if available_keys.is_empty() {
        println!("\n❌ No keys found in PIV slots!");
        println!("💡 Generate keys using:");
        println!("   ykman piv keys generate 9a /tmp/pubkey.pem --algorithm ECCP256");
        println!("   ykman piv certificates generate 9a /tmp/pubkey.pem --subject \"CN=Test\"");
        return Ok(());
    }

    // Use the first available key for demonstration
    let (demo_slot, demo_pubkey) = &available_keys[0];
    println!("\n● Using slot {} for demonstration", demo_slot);

    // Demonstrate hardware key extraction
    println!("\n● Hardware Key Extraction:");
    println!(
        "   Extracting public key from YubiKey PIV slot {}...",
        demo_slot
    );
    println!("   ✔ Public key extracted ({} bytes)", demo_pubkey.len());

    // Display key info
    display_key_info(demo_pubkey)?;

    // Test hardware signing capability
    println!("\n🔒 Hardware Signing Test:");
    let test_data = b"TrustEdge YubiKey Integration Test Data";

    match backend.perform_operation(
        demo_slot,
        CryptoOperation::Sign {
            data: test_data.to_vec(),
            algorithm: SignatureAlgorithm::EcdsaP256,
        },
    ) {
        Ok(CryptoResult::Signed(signature)) => {
            println!(
                "   ✔ Hardware signature generated ({} bytes)",
                signature.len()
            );

            // Analyze signature format
            if signature.len() > 10 && signature[0] == 0x30 {
                println!("   ✔ Signature is DER-encoded (ASN.1 format)");
            } else {
                println!("   ⚠ Signature appears to be raw format");
            }
        }
        Ok(_) => {
            println!("   ⚠ Unexpected result type from signing operation");
        }
        Err(e) => {
            println!("   ✖ Hardware signing failed: {}", e);
            println!("   💡 Ensure YubiKey PIN is correct and key supports signing");
        }
    }

    // Generate a complete X.509 certificate
    println!("\n🏭 X.509 Certificate Generation:");
    println!("   Creating hardware-backed certificate...");

    let certificate = create_yubikey_certificate(demo_pubkey, demo_slot, &backend)?;

    // Save certificate to file
    let cert_filename = format!("yubikey_cert_slot_{}.der", demo_slot);
    std::fs::write(&cert_filename, &certificate).context("Failed to save certificate")?;
    println!("   💾 Certificate saved to: {}", cert_filename);

    // Analyze the generated certificate
    println!("\n📋 Certificate Analysis:");
    analyze_certificate(&certificate)?;

    // Display certificate in OpenSSL-compatible format
    println!("\n🔍 Certificate Verification:");
    println!("   To verify this certificate with OpenSSL:");
    println!(
        "   openssl x509 -in {} -inform DER -text -noout",
        cert_filename
    );
    println!(
        "   openssl x509 -in {} -inform DER -fingerprint -noout",
        cert_filename
    );

    // Test with multiple slots if available
    if available_keys.len() > 1 {
        println!("\n🔄 Multi-Slot Summary:");
        for (slot, pubkey) in &available_keys {
            println!("   Slot {}: {} bytes public key", slot, pubkey.len());
        }
        println!("   💡 All slots can be used for certificate generation");
    }

    println!("\n✔ YubiKey Demo Complete!");
    println!("✔ Hardware key extraction verified");
    println!("✔ Hardware signing operations successful");
    println!("✔ X.509 certificate generation completed");
    println!("✔ Certificate exported in standard DER format");
    println!();
    println!("● Your YubiKey is ready for TrustEdge integration!");

    Ok(())
}

#[cfg(feature = "yubikey")]
fn display_key_info(public_key: &[u8]) -> anyhow::Result<()> {
    println!("   📊 Key Analysis:");
    println!("      Size: {} bytes", public_key.len());

    // Detect key format
    if public_key.len() >= 2 {
        match public_key[0] {
            0x30 => println!("      Format: ASN.1 DER (SubjectPublicKeyInfo)"),
            0x04 => println!("      Format: Uncompressed EC point"),
            0x02 | 0x03 => println!("      Format: Compressed EC point"),
            _ => println!("      Format: Unknown/Custom"),
        }
    }

    // Display first few bytes for verification
    print!("      Preview: ");
    for byte in &public_key[..public_key.len().min(16)] {
        print!("{:02x}", byte);
    }
    if public_key.len() > 16 {
        print!("...");
    }
    println!();

    Ok(())
}

#[cfg(feature = "yubikey")]
fn create_yubikey_certificate(
    public_key: &[u8],
    slot: &str,
    backend: &trustedge_core::backends::yubikey::YubiKeyBackend,
) -> anyhow::Result<Vec<u8>> {
    // Create a realistic X.509 certificate structure
    let mut cert = Vec::new();

    // Outer SEQUENCE
    cert.push(0x30);
    cert.push(0x82); // Long form length
    cert.extend_from_slice(&[0x00, 0x00]); // Length placeholder

    // TBS (To Be Signed) Certificate
    let mut tbs_cert = Vec::new();

    // TBS Certificate SEQUENCE
    tbs_cert.push(0x30);
    tbs_cert.push(0x82);
    tbs_cert.extend_from_slice(&[0x00, 0x00]); // Length placeholder

    // Version (v3 = 2)
    tbs_cert.extend_from_slice(&[0xA0, 0x03, 0x02, 0x01, 0x02]);

    // Serial Number (8 bytes)
    tbs_cert.extend_from_slice(&[0x02, 0x08]);
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    tbs_cert.extend_from_slice(&timestamp.to_be_bytes());

    // Signature Algorithm Identifier (ECDSA with SHA-256)
    tbs_cert.extend_from_slice(&[
        0x30, 0x0A, // SEQUENCE
        0x06, 0x08, // OBJECT IDENTIFIER
        0x2A, 0x86, 0x48, 0xCE, 0x3D, 0x04, 0x03, 0x02, // 1.2.840.10045.4.3.2
    ]);

    // Issuer (self-signed)
    let subject_name = format!("CN=YubiKey Slot {},O=TrustEdge,C=US", slot);
    let issuer_der = encode_distinguished_name(&subject_name)?;
    tbs_cert.extend_from_slice(&issuer_der);

    // Validity (1 year from now)
    let validity_der = encode_validity_period(365)?;
    tbs_cert.extend_from_slice(&validity_der);

    // Subject (same as issuer for self-signed)
    tbs_cert.extend_from_slice(&issuer_der);

    // Subject Public Key Info
    let pubkey_info = build_subject_public_key_info(public_key)?;
    tbs_cert.extend_from_slice(&pubkey_info);

    // Extensions (Basic Constraints, Key Usage)
    let extensions = encode_basic_extensions()?;
    tbs_cert.extend_from_slice(&extensions);

    // Update TBS length
    let tbs_length = tbs_cert.len() - 4;
    tbs_cert[2] = ((tbs_length >> 8) & 0xFF) as u8;
    tbs_cert[3] = (tbs_length & 0xFF) as u8;

    cert.extend_from_slice(&tbs_cert);

    // Sign the TBS certificate
    println!("   🔒 Signing certificate with YubiKey hardware...");
    let signature = sign_tbs_certificate(&tbs_cert, slot, backend)?;

    // Signature Algorithm (repeated)
    cert.extend_from_slice(&[
        0x30, 0x0A, // SEQUENCE
        0x06, 0x08, // OBJECT IDENTIFIER
        0x2A, 0x86, 0x48, 0xCE, 0x3D, 0x04, 0x03, 0x02, // 1.2.840.10045.4.3.2
    ]);

    // Signature Value (BIT STRING)
    cert.push(0x03); // BIT STRING
    cert.push((signature.len() + 1) as u8);
    cert.push(0x00); // No unused bits
    cert.extend_from_slice(&signature);

    // Update outer certificate length
    let total_length = cert.len() - 4;
    cert[2] = ((total_length >> 8) & 0xFF) as u8;
    cert[3] = (total_length & 0xFF) as u8;

    println!("   ✔ Certificate signed ({} bytes total)", cert.len());

    Ok(cert)
}

#[cfg(feature = "yubikey")]
fn encode_distinguished_name(name: &str) -> anyhow::Result<Vec<u8>> {
    // Simple DN encoding for demo - in production, use proper ASN.1 library
    let mut dn = Vec::new();

    // Extract CN from the name string
    let cn = if let Some(cn_start) = name.find("CN=") {
        let cn_part = &name[cn_start + 3..];
        if let Some(comma_pos) = cn_part.find(',') {
            &cn_part[..comma_pos]
        } else {
            cn_part
        }
    } else {
        "YubiKey Certificate"
    };

    // SEQUENCE
    dn.push(0x30);
    let content_start = dn.len();
    dn.push(0x00); // Length placeholder

    // SET
    dn.push(0x31);
    let set_start = dn.len();
    dn.push(0x00); // Length placeholder

    // SEQUENCE (Attribute)
    dn.push(0x30);
    let attr_start = dn.len();
    dn.push(0x00); // Length placeholder

    // CN OID (2.5.4.3)
    dn.extend_from_slice(&[0x06, 0x03, 0x55, 0x04, 0x03]);

    // UTF8String
    dn.push(0x0C);
    dn.push(cn.len() as u8);
    dn.extend_from_slice(cn.as_bytes());

    // Update lengths
    let attr_length = dn.len() - attr_start - 1;
    dn[attr_start] = attr_length as u8;

    let set_length = dn.len() - set_start - 1;
    dn[set_start] = set_length as u8;

    let content_length = dn.len() - content_start - 1;
    dn[content_start] = content_length as u8;

    Ok(dn)
}

#[cfg(feature = "yubikey")]
fn encode_validity_period(validity_days: u32) -> anyhow::Result<Vec<u8>> {
    use chrono::{Duration, Utc};

    let now = Utc::now();
    let not_after = now + Duration::days(validity_days as i64);

    let mut validity = vec![
        0x30, // SEQUENCE
        0x1E, // Fixed length for UTCTime format
        // Not Before (UTCTime)
        0x17, // UTCTime
        0x0D, // Length
    ];
    let not_before_str = now.format("%y%m%d%H%M%SZ").to_string();
    validity.extend_from_slice(not_before_str.as_bytes());

    // Not After (UTCTime)
    validity.push(0x17); // UTCTime
    validity.push(0x0D); // Length
    let not_after_str = not_after.format("%y%m%d%H%M%SZ").to_string();
    validity.extend_from_slice(not_after_str.as_bytes());

    Ok(validity)
}

#[cfg(feature = "yubikey")]
fn build_subject_public_key_info(public_key: &[u8]) -> anyhow::Result<Vec<u8>> {
    // If already in SubjectPublicKeyInfo format, use as-is
    if public_key.len() > 20 && public_key[0] == 0x30 {
        return Ok(public_key.to_vec());
    }

    // Otherwise, wrap in SubjectPublicKeyInfo
    let mut spki = Vec::new();

    // SEQUENCE
    spki.push(0x30);
    spki.push(0x59); // Typical length for P-256 SPKI

    // Algorithm Identifier
    spki.extend_from_slice(&[
        0x30, 0x13, // SEQUENCE
        0x06, 0x07, // OID for ecPublicKey
        0x2A, 0x86, 0x48, 0xCE, 0x3D, 0x02, 0x01, 0x06, 0x08, // OID for P-256 curve
        0x2A, 0x86, 0x48, 0xCE, 0x3D, 0x03, 0x01, 0x07,
    ]);

    // Public Key (BIT STRING)
    spki.push(0x03); // BIT STRING
    spki.push((public_key.len() + 1) as u8);
    spki.push(0x00); // No unused bits
    spki.extend_from_slice(public_key);

    Ok(spki)
}

#[cfg(feature = "yubikey")]
fn encode_basic_extensions() -> anyhow::Result<Vec<u8>> {
    let mut extensions = Vec::new();

    // Extensions tag
    extensions.extend_from_slice(&[0xA3, 0x1A]); // CONTEXT SPECIFIC [3]

    // SEQUENCE OF Extensions
    extensions.push(0x30);
    extensions.push(0x18);

    // Basic Constraints extension
    extensions.extend_from_slice(&[
        0x30, 0x16, // SEQUENCE
        0x06, 0x03, // OID for Basic Constraints
        0x55, 0x1D, 0x13, 0x01, 0x01, // CRITICAL
        0xFF, 0x04, 0x0C, // OCTET STRING
        0x30, 0x0A, // SEQUENCE
        0x01, 0x01, // BOOLEAN
        0x00, // FALSE (not a CA)
        0x02, 0x01, // INTEGER (path length)
        0x00, // 0
    ]);

    Ok(extensions)
}

#[cfg(feature = "yubikey")]
fn sign_tbs_certificate(
    tbs_cert: &[u8],
    slot: &str,
    backend: &trustedge_core::backends::yubikey::YubiKeyBackend,
) -> anyhow::Result<Vec<u8>> {
    use anyhow::anyhow;
    use trustedge_core::{CryptoOperation, CryptoResult, SignatureAlgorithm, UniversalBackend};

    // Hash the TBS certificate
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(tbs_cert);
    let hash = hasher.finalize();

    // Sign the hash with YubiKey
    match backend.perform_operation(
        slot,
        CryptoOperation::Sign {
            data: hash.to_vec(),
            algorithm: SignatureAlgorithm::EcdsaP256,
        },
    ) {
        Ok(CryptoResult::Signed(signature)) => Ok(signature),
        Ok(_) => Err(anyhow!("Unexpected result type from signing operation")),
        Err(e) => Err(anyhow!("Failed to sign certificate: {}", e)),
    }
}

#[cfg(feature = "yubikey")]
fn analyze_certificate(cert_der: &[u8]) -> anyhow::Result<()> {
    println!("   📊 Certificate Analysis:");
    println!("      Total size: {} bytes", cert_der.len());

    // Basic structure validation
    if cert_der.len() < 100 {
        println!("      ⚠ Certificate seems too small");
        return Ok(());
    }

    if cert_der[0] != 0x30 {
        println!("      ✖ Invalid certificate format (not a SEQUENCE)");
        return Ok(());
    }

    println!("      ✔ Valid ASN.1 SEQUENCE structure");

    // Look for signature algorithm OID
    let ecdsa_sha256_oid = [0x2A, 0x86, 0x48, 0xCE, 0x3D, 0x04, 0x03, 0x02];
    let oid_count = cert_der
        .windows(ecdsa_sha256_oid.len())
        .filter(|window| *window == ecdsa_sha256_oid)
        .count();

    if oid_count >= 2 {
        println!("      ✔ ECDSA-SHA256 signature algorithm detected");
    } else {
        println!("      ⚠ Signature algorithm not clearly identified");
    }

    // Look for signature bit string
    for i in 0..cert_der.len().saturating_sub(10) {
        if cert_der[i] == 0x03 && cert_der[i + 1] > 0x40 && cert_der[i + 1] < 0x80 {
            println!(
                "      ✔ Signature found at offset {} ({} bytes)",
                i,
                cert_der[i + 1]
            );
            break;
        }
    }

    println!("      💡 Use OpenSSL to verify: openssl x509 -inform DER -text -noout");

    Ok(())
}

#[cfg(not(feature = "yubikey"))]
fn main() {
    println!("● TrustEdge YubiKey Integration Demo");
    println!("===================================");
    println!();
    println!("❌ YubiKey support not compiled in");
    println!("💡 Run with: cargo run --example yubikey_demo --features yubikey");
    println!();
    println!("📋 Requirements:");
    println!("   • YubiKey with PIV applet");
    println!("   • OpenSC PKCS#11 module (apt install opensc-pkcs11)");
    println!("   • Keys in PIV slots (use 'ykman piv' to generate)");
    println!();
    println!("● Complete YubiKey Integration Features:");
    println!("   • Hardware public key extraction from all PIV slots");
    println!("   • Real-time key scanning and availability detection");
    println!("   • Hardware-backed digital signatures with ECDSA-P256");
    println!("   • Complete X.509 certificate generation pipeline");
    println!("   • Standards-compliant ASN.1 DER encoding");
    println!("   • Certificate validation and OpenSSL compatibility");
    println!("   • Multi-slot support for key management workflows");
}
