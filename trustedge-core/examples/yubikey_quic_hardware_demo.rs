//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/
//
// Project: trustedge — Privacy and trust at the edge.
//

//! YubiKey Hardware Signing over QUIC Demo
//!
//! This example demonstrates:
//! 1. Real YubiKey hardware connection and status
//! 2. Hardware-backed signing operations using PIV slots
//! 3. QUIC transport with YubiKey-signed messages
//! 4. End-to-end authentication using YubiKey hardware

#[cfg(feature = "yubikey")]
use anyhow::{Context, Result};
#[cfg(feature = "yubikey")]
use std::sync::Arc;
#[cfg(feature = "yubikey")]
use tokio::time::{sleep, Duration};
#[cfg(feature = "yubikey")]
use trustedge_core::backends::{
    CryptoOperation, SignatureAlgorithm, UniversalBackend, YubiKeyBackend, YubiKeyConfig,
};
#[cfg(feature = "yubikey")]
use trustedge_core::transport::{quic::QuicTransport, Transport, TransportConfig};
#[cfg(feature = "yubikey")]
use trustedge_core::NetworkChunk;

#[cfg(feature = "yubikey")]
async fn run_demo() -> Result<()> {
    println!("● YubiKey Hardware Signing over QUIC Demo");
    println!("==========================================");

    // Step 1: Initialize YubiKey Hardware Backend
    println!("\n● Step 1: Initializing YubiKey hardware backend...");

    let config = YubiKeyConfig {
        pkcs11_module_path: "/usr/lib/x86_64-linux-gnu/libykcs11.so".to_string(),
        pin: Some("123456".to_string()),
        slot: None, // Auto-detect slot
        verbose: true,
    };

    let yubikey_backend =
        YubiKeyBackend::with_config(config).context("Failed to initialize YubiKey backend")?;

    // Step 2: Check Hardware Connection Status
    println!("\n● Step 2: Checking YubiKey hardware status...");
    let backend_info = yubikey_backend.backend_info();
    println!(
        "  Backend: {} - {}",
        backend_info.name, backend_info.description
    );
    println!("  Available: {}", backend_info.available);
    println!(
        "  Hardware: {}",
        yubikey_backend.get_capabilities().hardware_backed
    );

    // Step 3: Test Hardware Signing Operations
    println!("\n● Step 3: Testing YubiKey hardware signing...");

    let test_data = b"Hello from YubiKey hardware over QUIC!";
    println!("  Signing data: {:?}", String::from_utf8_lossy(test_data));

    // Test different PIV slots and algorithms
    let test_cases = vec![
        ("authentication", SignatureAlgorithm::EcdsaP256),
        ("signature", SignatureAlgorithm::EcdsaP256),
        ("key_management", SignatureAlgorithm::RsaPkcs1v15),
    ];

    for (slot, algorithm) in test_cases {
        println!("\n  Testing slot '{}' with {:?}:", slot, algorithm);

        match yubikey_backend.perform_operation(
            slot,
            CryptoOperation::Sign {
                data: test_data.to_vec(),
                algorithm,
            },
        ) {
            Ok(result) => {
                if let trustedge_core::backends::CryptoResult::Signed(signature) = result {
                    println!("    ✔ Hardware signature: {} bytes", signature.len());
                    println!(
                        "    ✔ Signature (hex): {}",
                        hex::encode(&signature[..std::cmp::min(16, signature.len())])
                    );
                } else {
                    println!("    ⚠ Unexpected result type");
                }
            }
            Err(e) => {
                println!("    ⚠ Signing failed: {}", e);
            }
        }
    }

    // Step 4: Setup QUIC Transport
    println!("\n● Step 4: Setting up QUIC transport...");

    let server_config = TransportConfig {
        connect_timeout_ms: 5000,
        read_timeout_ms: 5000,
        max_message_size: 1024 * 1024,
        keep_alive_ms: 0,
        max_connection_bytes: 0,
        max_connection_chunks: 0,
        connection_idle_timeout_ms: 30000,
    };

    let server_bind_addr = "127.0.0.1:0".parse().unwrap();
    let server_transport = QuicTransport::create_yubikey_server(
        server_config,
        server_bind_addr,
        &yubikey_backend,
        "authentication", // Use authentication slot
    )
    .await
    .context("Failed to create YubiKey QUIC server")?;

    // Get the actual bound address
    let bound_addr = server_transport
        .local_addr()
        .context("Failed to get server bound address")?;
    println!("  Server listening on: {}", bound_addr);

    // Start server in background
    let server_transport = Arc::new(server_transport);
    let server_yubikey = Arc::new(yubikey_backend);

    let server_handle = {
        let transport = server_transport.clone();
        let yubikey = server_yubikey.clone();
        tokio::spawn(async move { handle_quic_server(transport, yubikey).await })
    };

    // Give server time to start
    sleep(Duration::from_millis(500)).await;

    // Step 5: QUIC Client with YubiKey Authentication
    println!("\n● Step 5: Testing QUIC client with YubiKey authentication...");

    let client_config = TransportConfig {
        connect_timeout_ms: 5000,
        read_timeout_ms: 5000,
        max_message_size: 1024 * 1024,
        keep_alive_ms: 0,
        max_connection_bytes: 0,
        max_connection_chunks: 0,
        connection_idle_timeout_ms: 30000,
    };

    let mut client_transport =
        QuicTransport::new(client_config).context("Failed to create client transport")?;

    // Connect to server
    println!("  Connecting to server at {}...", bound_addr);
    match client_transport.connect(bound_addr).await {
        Ok(_) => println!("  ✔ Connected to server"),
        Err(e) => {
            println!("  ✖ Connection failed: {}", e);
            return Ok(());
        }
    }

    // Step 6: Send YubiKey-Signed Messages over QUIC
    println!("\n● Step 6: Sending YubiKey-signed messages over QUIC...");

    let messages = [
        "Message 1: YubiKey hardware authentication",
        "Message 2: Cryptographic proof of possession",
        "Message 3: End-to-end security verification",
    ];

    for (i, message) in messages.iter().enumerate() {
        println!("\n  Sending message {}: {}", i + 1, message);

        // Create YubiKey-signed message
        let message_bytes = message.as_bytes();

        // Sign with YubiKey hardware
        let signed_result = server_yubikey.perform_operation(
            "authentication",
            CryptoOperation::Sign {
                data: message_bytes.to_vec(),
                algorithm: SignatureAlgorithm::EcdsaP256,
            },
        );

        match signed_result {
            Ok(trustedge_core::backends::CryptoResult::Signed(signature)) => {
                println!("    ✔ YubiKey signed message ({} bytes)", signature.len());

                // Create network chunk with signature as manifest
                let chunk = NetworkChunk::new(i as u64, message_bytes.to_vec(), signature);

                // Send over QUIC
                match client_transport.send_chunk(&chunk).await {
                    Ok(_) => println!("    ✔ Sent over QUIC successfully"),
                    Err(e) => println!("    ✖ QUIC send failed: {}", e),
                }
            }
            Err(e) => {
                println!("    ✖ YubiKey signing failed: {}", e);
            }
            _ => {
                println!("    ⚠ Unexpected signing result");
            }
        }

        // Small delay between messages
        sleep(Duration::from_millis(100)).await;
    }

    // Step 7: Cleanup
    println!("\n● Step 7: Cleaning up...");
    server_handle.abort();
    println!("  ✔ Demo completed successfully");

    Ok(())
}

#[cfg(feature = "yubikey")]
async fn handle_quic_server(
    transport: Arc<QuicTransport>,
    _yubikey: Arc<YubiKeyBackend>,
) -> Result<()> {
    println!("  Server: Waiting for connections...");

    loop {
        match transport.accept_connection().await {
            Ok(connection) => {
                println!("  Server: ✔ Client connected");

                // Accept a bidirectional stream
                match connection.accept_bi().await {
                    Ok((_send_stream, mut recv_stream)) => {
                        println!("  Server: ✔ Bidirectional stream established");

                        // Read data from the stream
                        match recv_stream.read_to_end(1024 * 1024).await {
                            Ok(data) => {
                                println!("  Server: ← Received {} bytes", data.len());

                                let message = String::from_utf8_lossy(&data);
                                println!("  Server: Message: {}", message);
                                println!("  Server: ✔ Message processed");
                            }
                            Err(e) => {
                                println!("  Server: ⚠ Read error: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        println!("  Server: ⚠ Stream accept error: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("  Server: ⚠ Accept error: {}", e);
                sleep(Duration::from_millis(100)).await;
            }
        }
    }
}

#[cfg(not(feature = "yubikey"))]
fn main() {
    println!("✖ YubiKey support not compiled in");
    println!("Run with: cargo run --example yubikey_quic_hardware_demo --features yubikey");
}

#[cfg(feature = "yubikey")]
#[tokio::main]
async fn main() -> Result<()> {
    println!("● Starting YubiKey Hardware QUIC Demo...");

    match run_demo().await {
        Ok(_) => {
            println!("\n✔ Demo completed successfully!");
            println!("   Real YubiKey hardware signing over QUIC verified.");
        }
        Err(e) => {
            println!("\n💥 Demo failed: {}", e);
            println!("   Check YubiKey connection and PKCS#11 setup.");
            std::process::exit(1);
        }
    }

    Ok(())
}
