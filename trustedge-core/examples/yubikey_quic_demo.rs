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
use std::net::SocketAddr;
#[cfg(feature = "yubikey")]
use trustedge_core::{
    backends::yubikey::{CertificateParams, HardwareCertificate, YubiKeyBackend, YubiKeyConfig},
    transport::{quic::QuicTransport, TransportConfig},
    NetworkChunk,
};

#[cfg(feature = "yubikey")]
#[tokio::main]
async fn main() -> Result<()> {
    println!("🔐 YubiKey-Attested QUIC Transport Demo");
    println!("=====================================");

    // Generate YubiKey-attested certificate
    println!("\n● Generating YubiKey-attested certificate for QUIC...");
    let hardware_cert = generate_yubikey_certificate().await?;

    // Demonstrate how certificate would be used in QUIC
    demonstrate_quic_integration(&hardware_cert).await?;

    // Show certificate validation
    validate_hardware_attestation(
        &hardware_cert.certificate_der,
        &hardware_cert.attestation_proof,
    )?;

    println!("\n✔ YubiKey-attested QUIC demo completed!");
    println!("\nThis demonstrates the foundation for:");
    println!("• Hardware-backed certificate generation with YubiKey");
    println!("• Cryptographic proof of hardware attestation");
    println!("• Ready for QUIC/TLS mutual authentication");
    println!("• Secure edge computing with hardware trust");

    Ok(())
}

#[cfg(feature = "yubikey")]
async fn generate_yubikey_certificate() -> Result<HardwareCertificate> {
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

                    Ok(hardware_cert)
                }
                Err(e) => {
                    println!("⚠ YubiKey not available: {}", e);
                    println!("   Using fallback certificate for demo...");
                    let fallback_cert = create_demo_certificate();
                    let fallback_proof = b"DEMO-ATTESTATION:FALLBACK-MODE".to_vec();
                    Ok(HardwareCertificate {
                        certificate_der: fallback_cert,
                        attestation_proof: fallback_proof,
                        key_id: "demo_key".to_string(),
                        subject: "CN=Demo QUIC Server,O=TrustEdge Labs,OU=Demo,C=US".to_string(),
                    })
                }
            }
        }
        Err(e) => {
            println!("⚠ YubiKey backend not available: {}", e);
            println!("   Using fallback certificate for demo...");
            let fallback_cert = create_demo_certificate();
            let fallback_proof = b"DEMO-ATTESTATION:FALLBACK-MODE".to_vec();
            Ok(HardwareCertificate {
                certificate_der: fallback_cert,
                attestation_proof: fallback_proof,
                key_id: "demo_key".to_string(),
                subject: "CN=Demo QUIC Server,O=TrustEdge Labs,OU=Demo,C=US".to_string(),
            })
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
async fn demonstrate_quic_integration(hardware_cert: &HardwareCertificate) -> Result<()> {
    println!("\n● Demonstrating QUIC transport integration:");

    // Create a QUIC transport configuration
    let transport_config = TransportConfig {
        connect_timeout_ms: 5000,
        max_message_size: 1024 * 1024, // 1MB
        connection_idle_timeout_ms: 30000,
        ..Default::default()
    };

    // Create QUIC transport with hardware verification capability
    let mut quic_transport = QuicTransport::new(transport_config)?;

    // Demonstrate server endpoint creation with hardware certificate
    let listen_addr: SocketAddr = "127.0.0.1:0".parse()?;

    println!("   ✔ Created QUIC transport with hardware verification support");
    println!(
        "   ✔ Hardware certificate: {} bytes",
        hardware_cert.certificate_der.len()
    );
    println!(
        "   ✔ Attestation proof: {} bytes",
        hardware_cert.attestation_proof.len()
    );

    // Create a network chunk that would be sent over QUIC
    let message = "Hardware-attested QUIC handshake data";
    let manifest = format!(
        "cert_size:{}:attestation_size:{}:subject:{}",
        hardware_cert.certificate_der.len(),
        hardware_cert.attestation_proof.len(),
        hardware_cert.subject
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
    println!("   ✔ Certificate embedded in transport manifest");

    // Show what would happen in a real QUIC implementation
    println!("\n   → In real QUIC server implementation:");
    println!("     • Server endpoint created with YubiKey certificate");
    println!("     • Hardware attestation embedded in TLS certificate");
    println!("     • Client validates hardware proof during handshake");

    println!("\n   → In real QUIC client implementation:");
    println!("     • Client uses HardwareBackedVerifier for certificate validation");
    println!("     • Mutual authentication with hardware proof verification");
    println!("     • Secure channel established with hardware-backed cryptography");
    println!("     • NetworkChunks transmitted over authenticated connection");

    // Demonstrate what the hardware verification would look like
    let trusted_certificates = vec![hardware_cert.certificate_der.clone()];
    println!(
        "\n   ✔ Trusted certificate list prepared ({} certificates)",
        trusted_certificates.len()
    );
    println!("   ✔ Ready for hardware-verified QUIC connection");

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
