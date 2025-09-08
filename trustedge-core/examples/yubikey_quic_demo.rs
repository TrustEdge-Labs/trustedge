//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
/// Project: trustedge — Privacy and trust at the edge.
//
/// examples/yubikey_quic_demo.rs - YubiKey Hardware with QUIC Transport Demo
//
/// This example demonstrates using real YubiKey hardware signing operations
/// combined with QUIC transport for network communication.
use anyhow::Result;

#[cfg(feature = "yubikey")]
use {
    std::net::SocketAddr,
    std::time::Instant,
    trustedge_core::backends::{YubiKeyBackend, YubiKeyConfig},
    trustedge_core::transport::{quic::QuicTransport, TransportConfig},
};

#[tokio::main]
async fn main() -> Result<()> {
    println!("YubiKey + QUIC Transport Demo");
    println!("=============================");
    println!();

    #[cfg(not(feature = "yubikey"))]
    {
        println!("✖ YubiKey support not compiled in");
        println!("● Run with: cargo run --example yubikey_quic_demo --features yubikey");
        println!();
        println!("● Requirements:");
        println!("   • YubiKey with PIV applet");
        println!("   • OpenSC PKCS#11 module (apt install opensc-pkcs11)");
        println!("   • Keys in PIV slots (use 'ykman piv' to generate)");
        println!();
        println!("● Demo Features:");
        println!("   • Real YubiKey hardware signing operations");
        println!("   • QUIC transport for network communication");
        println!("   • Demonstration of hardware + network integration patterns");
    }

    #[cfg(feature = "yubikey")]
    {
        println!("This demo shows real YubiKey hardware integration with QUIC transport:");
        println!("• Hardware-based cryptographic operations using real YubiKey device");
        println!("• QUIC transport layer for secure network communication");
        println!("• Integration patterns for hardware security + network protocols");
        println!();

        demo_yubikey_quic_integration().await?;
    }

    Ok(())
}

#[cfg(feature = "yubikey")]
async fn demo_yubikey_quic_integration() -> Result<()> {
    println!("● Configuration:");
    let config = YubiKeyConfig::default();
    println!("   PKCS#11 Module: {}", config.pkcs11_module_path);
    println!(
        "   PIN: {}",
        if config.pin.is_some() {
            "Provided"
        } else {
            "Required"
        }
    );
    println!("   Slot: {:?}", config.slot);
    println!();

    println!("● Initializing YubiKey Backend...");
    let mut yubikey_backend = match YubiKeyBackend::new() {
        Ok(backend) => {
            println!("✔ YubiKey backend initialized successfully");
            backend
        }
        Err(e) => {
            println!("✖ Failed to initialize YubiKey backend: {}", e);
            println!();
            println!("● Troubleshooting:");
            println!("   • Ensure YubiKey is inserted and PIV applet is enabled");
            println!("   • Install OpenSC: apt install opensc-pkcs11");
            println!("   • Generate keys: ykman piv keys generate 9c /tmp/pubkey.pem");
            println!(
                "   • Check PKCS#11 module path: {}",
                config.pkcs11_module_path
            );
            return Err(e);
        }
    };

    println!();
    println!("● Demo: YubiKey Hardware + QUIC Transport Integration");
    println!("=====================================================");

    // Demo 1: YubiKey hardware signing capabilities
    demo_hardware_signing(&mut yubikey_backend).await?;

    // Demo 2: QUIC transport setup
    demo_quic_transport_setup().await?;

    // Demo 3: Integration patterns
    demo_integration_patterns(&mut yubikey_backend).await?;

    println!();
    println!("✔ YubiKey + QUIC Integration Demo Complete!");
    println!("   Real hardware operations + network transport demonstrated");

    Ok(())
}

#[cfg(feature = "yubikey")]
async fn demo_hardware_signing(yubikey_backend: &mut YubiKeyBackend) -> Result<()> {
    println!();
    println!("● Demo 1: YubiKey Hardware Signing Operations");
    println!("   Testing real hardware cryptographic operations...");

    // Prompt for PIN if needed
    if yubikey_backend.config.pin.is_none() {
        use std::io::{self, Write};
        print!("   Enter YubiKey PIN: ");
        io::stdout().flush()?;
        let mut pin = String::new();
        io::stdin().read_line(&mut pin)?;
        yubikey_backend.config.pin = Some(pin.trim().to_string());
    }

    let test_data = b"TrustEdge QUIC Integration Test Data";
    let key_id = "02"; // PIV slot 9C mapped to PKCS#11 object ID 02

    println!("   ● Performing hardware signing operation...");
    let start_time = Instant::now();

    match yubikey_backend.hardware_sign(key_id, test_data) {
        Ok(signature) => {
            let duration = start_time.elapsed();
            println!(
                "   ✔ Hardware signature generated ({} bytes, {:.2}s)",
                signature.len(),
                duration.as_secs_f64()
            );

            // Verify signature is different each time (proving real hardware operation)
            match yubikey_backend.hardware_sign(key_id, test_data) {
                Ok(signature2) => {
                    if signature != signature2 {
                        println!("   ✔ Signatures are unique (proving real hardware randomness)");
                    } else {
                        println!("   ⚠ Signatures identical (unexpected for ECDSA)");
                    }
                }
                Err(e) => println!("   ⚠ Second signature failed: {}", e),
            }
        }
        Err(e) => {
            println!("   ✖ Hardware signing failed: {}", e);
            println!("   ● This may indicate PIN locked or hardware communication issues");
            return Err(e);
        }
    }

    Ok(())
}

#[cfg(feature = "yubikey")]
async fn demo_quic_transport_setup() -> Result<()> {
    println!();
    println!("● Demo 2: QUIC Transport Setup");
    println!("   Testing QUIC transport configuration...");

    let transport_config = TransportConfig {
        connect_timeout_ms: 5000,
        read_timeout_ms: 10000,
        max_message_size: 1024 * 1024, // 1MB
        keep_alive_ms: 30000,
        max_connection_bytes: 0,  // unlimited
        max_connection_chunks: 0, // unlimited
        connection_idle_timeout_ms: 60000,
    };

    let quic_transport = QuicTransport::new(transport_config)?;
    println!("   ✔ QUIC transport initialized successfully");
    println!("   ✔ Transport ready for secure connections");

    // Test basic transport capabilities
    println!("   ● Transport configuration:");
    println!("     - Connect timeout: 5000ms");
    println!("     - Max message size: 1MB");
    println!("     - Keep alive: 30s");
    println!("     - Connection idle timeout: 60s");

    Ok(())
}

#[cfg(feature = "yubikey")]
async fn demo_integration_patterns(yubikey_backend: &mut YubiKeyBackend) -> Result<()> {
    println!();
    println!("● Demo 3: Integration Patterns for Hardware + Network");
    println!("   Demonstrating how YubiKey operations can enhance QUIC security...");

    let test_data = b"QUIC session initialization data";
    let key_id = "02";

    println!("   ● Integration Workflow Example:");
    println!("   1. Client generates session challenge data");
    println!("   2. YubiKey signs challenge for authentication");

    let start_time = Instant::now();
    match yubikey_backend.hardware_sign(key_id, test_data) {
        Ok(signature) => {
            let duration = start_time.elapsed();
            println!(
                "   3. ✔ Hardware signature ready for QUIC handshake ({:.2}s)",
                duration.as_secs_f64()
            );
            println!("   4. Signature can be embedded in QUIC transport authentication");
            println!("   5. Remote peer verifies hardware-backed authentication");
        }
        Err(e) => {
            println!("   3. ✖ Hardware signing failed: {}", e);
            return Err(e);
        }
    }

    println!();
    println!("   ● Security Benefits:");
    println!("   • Hardware-backed identity verification");
    println!("   • Non-repudiation of QUIC session establishment");
    println!("   • Protection against software key compromise");
    println!("   • Compliance with hardware security requirements");

    println!();
    println!("   ● Implementation Notes:");
    println!("   • YubiKey signatures provide strong authentication");
    println!("   • QUIC transport ensures fast, secure communication");
    println!("   • Integration enables hardware security + network performance");
    println!("   • Real hardware timing: ~1.5s per signature operation");
    Ok(())
}
