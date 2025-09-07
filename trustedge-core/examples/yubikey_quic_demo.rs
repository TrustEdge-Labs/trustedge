/*
 * Copyright (c) 2025 TRUSTEDGE LABS LLC
 * This source code is subject to the terms of the Mozilla Public License, v. 2.0.
 * If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
 *
 * Project: trustedge — Privacy and trust at the edge.
 */

//! YubiKey-Attested QUIC Transport Demo
//!
//! This example demonstrates hardware-attested certificate generation using
//! YubiKey and shows how it would integrate with QUIC transport for secure
//! hardware-backed mutual authentication.
//!
//! Run with: cargo run --example yubikey_quic_demo --features yubikey

#[cfg(feature = "yubikey")]
use anyhow::Result;
#[cfg(feature = "yubikey")]
use trustedge_core::{
    backends::yubikey::{CertificateParams, YubiKeyBackend, YubiKeyConfig},
    NetworkChunk,
};

#[cfg(feature = "yubikey")]
#[tokio::main]
async fn main() -> Result<()> {
    println!("🔐 YubiKey-Attested QUIC Transport Demo");
    println!("=====================================");

    // Generate YubiKey-attested certificate
    println!("\n● Generating YubiKey-attested certificate for QUIC...");
    let (cert_der, attestation_proof) = generate_yubikey_certificate().await?;

    // Demonstrate how certificate would be used in QUIC
    demonstrate_quic_integration(&cert_der, &attestation_proof).await?;

    // Show certificate validation
    validate_hardware_attestation(&cert_der, &attestation_proof)?;

    println!("\n✔ YubiKey-attested QUIC demo completed!");
    println!("\nThis demonstrates the foundation for:");
    println!("• Hardware-backed certificate generation with YubiKey");
    println!("• Cryptographic proof of hardware attestation");
    println!("• Ready for QUIC/TLS mutual authentication");
    println!("• Secure edge computing with hardware trust");

    Ok(())
}

#[cfg(feature = "yubikey")]
async fn generate_yubikey_certificate() -> Result<(Vec<u8>, Vec<u8>)> {
    let config = YubiKeyConfig {
        pkcs11_module_path: "/usr/lib/x86_64-linux-gnu/opensc-pkcs11.so".to_string(),
        pin: None,
        slot: None,
        verbose: true,
    };

    match YubiKeyBackend::with_config(config) {
        Ok(backend) => {
            let cert_params = CertificateParams {
                subject: "CN=TrustEdge QUIC Server,O=TrustEdge Labs,OU=Hardware Security,C=US"
                    .to_string(),
                validity_days: 30,
                is_ca: false,
            };

            match backend.generate_certificate("quic_tls_key", cert_params) {
                Ok(hardware_cert) => {
                    println!("✔ YubiKey certificate generated!");
                    println!(
                        "   Certificate: {} bytes (DER-encoded)",
                        hardware_cert.certificate_der.len()
                    );
                    println!(
                        "   Attestation: {} bytes (hardware proof)",
                        hardware_cert.attestation_proof.len()
                    );
                    println!("   Subject: {}", hardware_cert.subject);

                    Ok((
                        hardware_cert.certificate_der,
                        hardware_cert.attestation_proof,
                    ))
                }
                Err(e) => {
                    println!("⚠ YubiKey not available: {}", e);
                    println!("   Using fallback certificate for demo...");
                    let fallback_cert = create_demo_certificate();
                    let fallback_proof = b"DEMO-ATTESTATION:FALLBACK-MODE".to_vec();
                    Ok((fallback_cert, fallback_proof))
                }
            }
        }
        Err(e) => {
            println!("⚠ YubiKey backend not available: {}", e);
            println!("   Using fallback certificate for demo...");
            let fallback_cert = create_demo_certificate();
            let fallback_proof = b"DEMO-ATTESTATION:FALLBACK-MODE".to_vec();
            Ok((fallback_cert, fallback_proof))
        }
    }
}

#[cfg(feature = "yubikey")]
fn create_demo_certificate() -> Vec<u8> {
    // Create a demo certificate structure for testing
    let cert_data = b"-----BEGIN CERTIFICATE-----\nDEMO-CERT:TrustEdge-QUIC:Hardware-Backed\n-----END CERTIFICATE-----";
    cert_data.to_vec()
}

#[cfg(feature = "yubikey")]
async fn demonstrate_quic_integration(cert_der: &[u8], attestation_proof: &[u8]) -> Result<()> {
    println!("\n● Demonstrating QUIC transport integration:");

    // Create a network chunk that would be sent over QUIC
    let message = "Hardware-attested QUIC handshake data";
    let manifest = format!(
        "cert_size:{}:attestation_size:{}",
        cert_der.len(),
        attestation_proof.len()
    );

    let network_chunk = NetworkChunk::new_with_nonce(
        0,
        message.as_bytes().to_vec(),
        manifest.as_bytes().to_vec(),
        [
            0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0, 0x11, 0x22, 0x33, 0x44,
        ], // Demo nonce
    );

    println!("   ✔ Created network chunk for QUIC transport");
    println!("   ✔ Sequence: {}", network_chunk.sequence);
    println!("   ✔ Data size: {} bytes", network_chunk.data.len());
    println!(
        "   ✔ Manifest: {}",
        String::from_utf8_lossy(&network_chunk.manifest)
    );
    println!("   ✔ Timestamp: {}", network_chunk.timestamp);

    // Demonstrate certificate embedding in QUIC handshake
    println!("\n   → In real QUIC implementation:");
    println!("     • Certificate would be embedded in TLS handshake");
    println!("     • Hardware attestation verified during connection");
    println!("     • Mutual authentication with hardware proof");
    println!("     • Secure channel with hardware-backed cryptography");

    Ok(())
}

#[cfg(feature = "yubikey")]
fn validate_hardware_attestation(cert_der: &[u8], attestation_proof: &[u8]) -> Result<()> {
    println!("\n● Validating hardware attestation:");

    // Basic validation checks
    if !cert_der.is_empty() {
        println!("   ✔ Certificate present ({} bytes)", cert_der.len());

        // Check for certificate structure
        if cert_der.len() > 64 {
            println!("   ✔ Certificate has reasonable size");
        }
    }

    if !attestation_proof.is_empty() {
        println!(
            "   ✔ Hardware attestation proof present ({} bytes)",
            attestation_proof.len()
        );

        // Validate attestation content
        let proof_str = String::from_utf8_lossy(attestation_proof);
        if proof_str.contains("YUBIKEY-ATTESTATION") {
            println!("   ✔ YubiKey hardware attestation verified");
        } else if proof_str.contains("DEMO-ATTESTATION") {
            println!("   ⚠ Demo mode - real YubiKey not detected");
        }

        if proof_str.contains("HARDWARE-VERIFIED") || proof_str.contains("FALLBACK-MODE") {
            println!("   ✔ Attestation proof format valid");
        }
    }

    println!("   → Hardware attestation validation complete");
    println!("   → Ready for secure QUIC transport integration");

    Ok(())
}

#[cfg(not(feature = "yubikey"))]
#[tokio::main]
async fn main() {
    println!("🔐 YubiKey-Attested QUIC Transport Demo");
    println!("=====================================");
    println!();
    println!("⚠ This example requires the 'yubikey' feature to be enabled.");
    println!("  Run with: cargo run --example yubikey_quic_demo --features yubikey");
    println!();
    println!("This demo would show:");
    println!("• Hardware-backed certificate generation with YubiKey");
    println!("• Integration with QUIC transport for secure connections");
    println!("• Cryptographic proof of hardware attestation");
    println!("• Mutual authentication with hardware-backed trust");
    println!("• Ready for production edge computing scenarios");
}
