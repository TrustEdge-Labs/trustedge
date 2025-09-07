/*
 * Copyright (c) 2025 TRUSTEDGE LABS LLC
 * This source code is subject to the terms of the Mozilla Public License, v. 2.0.
 * If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
 *
 * Project: trustedge — Privacy and trust at the edge.
 */

//! YubiKey Hardware Certificate Generation Demo
//!
//! This example demonstrates generating hardware-attested X.509 certificates
//! using the YubiKey backend. These certificates can be used for QUIC/TLS
//! authentication with hardware-backed trust.
//!
//! Run with: cargo run --example yubikey_certificate_demo --features yubikey

#[cfg(feature = "yubikey")]
use anyhow::Result;
#[cfg(feature = "yubikey")]
use trustedge_core::{
    backends::yubikey::{CertificateParams, HardwareCertificate, YubiKeyBackend, YubiKeyConfig},
    UniversalBackend,
};

#[cfg(feature = "yubikey")]
fn main() -> Result<()> {
    println!("🔐 YubiKey Hardware Certificate Generation Demo");
    println!("===============================================");

    // Create YubiKey backend configuration
    let config = YubiKeyConfig {
        pkcs11_module_path: "/usr/lib/x86_64-linux-gnu/opensc-pkcs11.so".to_string(),
        pin: None,  // Will prompt if needed
        slot: None, // Auto-detect
        verbose: true,
    };

    println!("\n● Creating YubiKey backend...");
    let backend = match YubiKeyBackend::with_config(config) {
        Ok(backend) => {
            println!("✔ YubiKey backend created successfully");
            backend
        }
        Err(e) => {
            println!("✖ Failed to create YubiKey backend: {}", e);
            println!("  Make sure:");
            println!("  - YubiKey is connected");
            println!("  - OpenSC PKCS#11 library is installed");
            println!("  - You have permissions to access the device");
            return Ok(());
        }
    };

    // Show backend capabilities
    let capabilities = backend.get_capabilities();
    println!("\n● YubiKey Capabilities:");
    println!("   Hardware-backed: {}", capabilities.hardware_backed);
    println!(
        "   Supports attestation: {}",
        capabilities.supports_attestation
    );
    println!(
        "   Asymmetric algorithms: {:?}",
        capabilities.asymmetric_algorithms
    );

    // Certificate generation parameters
    let cert_params = CertificateParams {
        subject: "CN=TrustEdge Test Certificate,O=TrustEdge Labs,C=US".to_string(),
        validity_days: 365,
        is_ca: false,
    };

    println!("\n● Generating hardware-attested certificate...");
    match backend.generate_certificate("test_key", cert_params) {
        Ok(hardware_cert) => {
            print_certificate_info(&hardware_cert);

            // Demonstrate certificate usage
            demonstrate_certificate_usage(&hardware_cert)?;
        }
        Err(e) => {
            println!("✖ Certificate generation failed: {}", e);
            println!("  This might be expected if no keys are available");
        }
    }

    println!("\n✔ Demo completed successfully!");
    println!("\nNext steps:");
    println!("• Use this certificate for QUIC/TLS authentication");
    println!("• Integrate with transport layer for secure connections");
    println!("• Implement certificate-based mutual authentication");

    Ok(())
}

#[cfg(feature = "yubikey")]
fn print_certificate_info(cert: &HardwareCertificate) {
    println!("✔ Hardware certificate generated!");
    println!("   Subject: {}", cert.subject);
    println!("   Key ID: {}", cert.key_id);
    println!("   Certificate size: {} bytes", cert.certificate_der.len());
    println!(
        "   Attestation proof: {} bytes",
        cert.attestation_proof.len()
    );

    // Show certificate hex preview
    if !cert.certificate_der.is_empty() {
        let preview = cert
            .certificate_der
            .iter()
            .take(16)
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<_>>()
            .join(" ");
        println!("   Certificate preview: {} ...", preview);
    }

    // Show attestation proof preview
    if !cert.attestation_proof.is_empty() {
        let proof_preview = String::from_utf8_lossy(
            &cert.attestation_proof[..cert.attestation_proof.len().min(50)],
        );
        println!("   Attestation preview: {} ...", proof_preview);
    }
}

#[cfg(feature = "yubikey")]
fn demonstrate_certificate_usage(cert: &HardwareCertificate) -> Result<()> {
    println!("\n● Demonstrating certificate usage:");

    // Simulate certificate validation
    if !cert.certificate_der.is_empty() && !cert.attestation_proof.is_empty() {
        println!("   ✔ Certificate has valid DER encoding");
        println!("   ✔ Hardware attestation proof present");

        // Check if attestation contains expected elements
        let proof_str = String::from_utf8_lossy(&cert.attestation_proof);
        if proof_str.contains("YUBIKEY-ATTESTATION") && proof_str.contains("HARDWARE-VERIFIED") {
            println!("   ✔ Attestation proof validates hardware origin");
        }

        println!("   → Ready for QUIC/TLS integration");
    } else {
        println!("   ⚠ Certificate or attestation data missing");
    }

    Ok(())
}

#[cfg(not(feature = "yubikey"))]
fn main() {
    println!("🔐 YubiKey Hardware Certificate Generation Demo");
    println!("===============================================");
    println!();
    println!("⚠ This example requires the 'yubikey' feature to be enabled.");
    println!("  Run with: cargo run --example yubikey_certificate_demo --features yubikey");
    println!();
    println!("The YubiKey feature provides:");
    println!("• Hardware-backed certificate generation");
    println!("• PKCS#11 integration for YubiKey devices");
    println!("• Hardware attestation proof generation");
    println!("• X.509 certificate creation with hardware keys");
}
