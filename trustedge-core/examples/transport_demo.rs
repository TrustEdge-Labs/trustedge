//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
/// Project: trustedge — Privacy and trust at the edge.
//
/// examples/transport_demo.rs - Transport abstraction demo
//
/// Demonstrates how to use the transport abstraction with TCP and QUIC.
use anyhow::Result;
use trustedge_core::{NetworkChunk, TransportConfig, TransportFactory};

#[tokio::main]
async fn main() -> Result<()> {
    println!("● TrustEdge Transport Demo");

    // Create a sample NetworkChunk for testing
    let test_chunk = NetworkChunk::new(1, b"Hello, TrustEdge!".to_vec(), b"manifest".to_vec());

    println!(
        "● Created test chunk: sequence={}, data_len={}",
        test_chunk.sequence,
        test_chunk.data.len()
    );

    // Demonstrate TCP transport creation
    let tcp_config = TransportConfig::default();
    let _tcp_transport = TransportFactory::create_tcp(tcp_config.clone());

    println!("● TCP transport created");

    // Demonstrate QUIC transport creation
    match TransportFactory::create_quic(tcp_config) {
        Ok(_quic_transport) => {
            println!("● QUIC transport created");

            // In a real scenario, you would connect to a server:
            // let server_addr: SocketAddr = "127.0.0.1:8080".parse()?;
            // quic_transport.connect(server_addr).await?;
            // quic_transport.send_chunk(&test_chunk).await?;

            println!("✔ Transport abstraction demo completed successfully!");
        }
        Err(e) => {
            println!("⚠ QUIC transport creation failed: {}", e);
            println!("✔ TCP transport demo completed successfully!");
        }
    }

    Ok(())
}
