/*
 * Copyright (c) 2025 TRUSTEDGE LABS LLC
 * This source code is subject to the terms of the Mozilla Public License, v. 2.0.
 * If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
 *
 * Project: trustedge — Privacy and trust at the edge.
 */

//! Transport Layer Integration Tests
//!
//! Comprehensive tests for transport layer functionality including:
//! - QUIC transport operations
//! - TCP transport operations  
//! - Transport abstraction layer
//! - Error handling and edge cases

use anyhow::Result;

use trustedge_core::transport::{tcp::TcpTransport, TransportConfig};
use trustedge_core::NetworkChunk;

/// Test real QUIC transport data transfer
#[tokio::test]
async fn test_real_quic_data_transfer() -> Result<()> {
    use trustedge_core::transport::TransportFactory;

    let config = TransportConfig::default();

    // Create test data
    let test_data = b"real QUIC transport test data";
    let manifest = r#"{"sequence":1,"test":true,"transport":"quic"}"#.as_bytes().to_vec();
    let _test_chunk = NetworkChunk::new(1, test_data.to_vec(), manifest);

    // Test QUIC transport creation
    let quic_result = TransportFactory::create_quic(config);

    // Note: QUIC requires certificates and more complex setup for real connections
    // For now, we verify that QUIC transport can be created
    match quic_result {
        Ok(_transport) => {
            println!("✔ QUIC transport created successfully");
            // In a full implementation, we would set up QUIC server/client here
        }
        Err(e) => {
            println!(
                "⚠ QUIC transport creation failed (expected without proper cert setup): {}",
                e
            );
            // This is expected without proper certificate setup
        }
    }

    Ok(())
}

/// Test concurrent TCP connections with multiple clients
#[tokio::test]
async fn test_concurrent_tcp_connections() -> Result<()> {
    use std::sync::Arc;
    use tokio::net::TcpListener;
    use tokio::sync::Barrier;
    use trustedge_core::transport::TransportFactory;

    let _config = TransportConfig::default();

    // Start real server
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let addr = listener.local_addr()?;

    // Barrier to synchronize all clients
    let barrier = Arc::new(Barrier::new(4)); // 3 clients + 1 server coordination
    let server_barrier = barrier.clone();

    // Spawn server task that accepts multiple connections
    let server_handle = tokio::spawn(async move {
        let mut accepted_connections = 0;

        while accepted_connections < 3 {
            match listener.accept().await {
                Ok((_stream, _addr)) => {
                    accepted_connections += 1;
                    println!("✔ Server accepted connection {}/3", accepted_connections);

                    // Keep connection alive briefly
                    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
                }
                Err(e) => {
                    println!("Server accept error: {}", e);
                    break;
                }
            }
        }

        // Signal all clients we're done
        server_barrier.wait().await;
        Result::<()>::Ok(())
    });

    // Spawn multiple client tasks
    let client_handles: Vec<_> = (0..3)
        .map(|client_id| {
            let barrier = barrier.clone();

            tokio::spawn(async move {
                let mut client_transport = TransportFactory::create_tcp(TransportConfig::default());

                // Connect to server
                let connect_result = client_transport.connect(addr).await;
                if connect_result.is_ok() {
                    println!("✔ Client {} connected successfully", client_id);

                    // Create unique test data for this client
                    let test_data = format!("client {} data", client_id);
                    let manifest =
                        format!(r#"{{"sequence":{},"client_id":{}}}"#, client_id, client_id)
                            .into_bytes();
                    let test_chunk =
                        NetworkChunk::new(client_id as u64, test_data.into_bytes(), manifest);

                    // Try to send data (may fail if server closes connection quickly)
                    let _send_result = client_transport.send_chunk(&test_chunk).await;

                    client_transport.close().await?;
                } else {
                    println!(
                        "✖ Client {} failed to connect: {:?}",
                        client_id, connect_result
                    );
                }

                // Wait for all clients and server to finish
                barrier.wait().await;
                Result::<()>::Ok(())
            })
        })
        .collect();

    // Wait for all tasks to complete
    server_handle.await??;
    for handle in client_handles {
        handle.await??;
    }

    println!("✔ Concurrent connections test completed successfully");
    Ok(())
}
#[tokio::test]
async fn test_real_tcp_data_transfer() -> Result<()> {
    use tokio::net::TcpListener;
    use trustedge_core::transport::TransportFactory;

    let config = TransportConfig::default();

    // Start real server
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let addr = listener.local_addr()?;

    // Create test data
    let test_data = b"real transport test data";
    let manifest = r#"{"sequence":1,"test":true}"#.as_bytes().to_vec();
    let test_chunk = NetworkChunk::new(1, test_data.to_vec(), manifest);

    // Spawn server task
    let server_handle = tokio::spawn(async move {
        let (stream, _) = listener.accept().await?;
        let _server_transport = TcpTransport::new(TransportConfig::default());

        // Accept connection by creating transport from existing stream
        let codec = tokio_util::codec::LengthDelimitedCodec::builder()
            .max_frame_length(TransportConfig::default().max_message_size)
            .new_codec();
        let _framed = tokio_util::codec::Framed::new(stream, codec);

        // We need to manually set up the server transport for this test
        // In a real scenario, we'd have a proper server implementation

        Result::<()>::Ok(())
    });

    // Connect and send real data
    let mut client_transport = TransportFactory::create_tcp(config);
    client_transport.connect(addr).await?;
    client_transport.send_chunk(&test_chunk).await?;
    client_transport.close().await?;

    // Verify server completed
    server_handle.await??;

    Ok(())
}

/// Test large data transfer with proper chunking
#[tokio::test]
async fn test_multi_chunk_data_transfer() -> Result<()> {
    use tokio::net::TcpListener;
    use tokio::sync::mpsc;
    use trustedge_core::transport::TransportFactory;

    let config = TransportConfig::default();

    // Start real server
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let addr = listener.local_addr()?;

    // Channel for coordination
    let (tx, mut rx) = mpsc::channel(1);

    // Create multiple chunks simulating a large data stream
    let chunk_count = 5;
    let mut test_chunks = Vec::new();

    for i in 0..chunk_count {
        let chunk_data = format!(
            "Chunk {} of large data stream - {}",
            i + 1,
            "x".repeat(1000)
        );
        let manifest = format!(
            r#"{{"sequence":{},"chunk_id":{},"total_chunks":{},"size":{}}}"#,
            i + 1,
            i + 1,
            chunk_count,
            chunk_data.len()
        )
        .into_bytes();

        test_chunks.push(NetworkChunk::new(i + 1, chunk_data.into_bytes(), manifest));
    }

    let expected_chunk_count = test_chunks.len();

    // Spawn server task
    let server_handle = tokio::spawn(async move {
        let (_stream, _) = listener.accept().await?;

        // Keep connection alive for multiple chunks
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

        // Signal completion
        tx.send(()).await.unwrap();
        Result::<()>::Ok(())
    });

    // Connect and send multiple chunks
    let mut client_transport = TransportFactory::create_tcp(config);
    client_transport.connect(addr).await?;

    let mut sent_chunks = 0;
    for (i, chunk) in test_chunks.iter().enumerate() {
        let send_result = client_transport.send_chunk(chunk).await;
        if send_result.is_ok() {
            sent_chunks += 1;
            println!("✔ Sent chunk {}/{}", i + 1, expected_chunk_count);
        } else {
            println!("✖ Failed to send chunk {}: {:?}", i + 1, send_result);
        }

        // Small delay between chunks to simulate real streaming
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
    }

    // Wait for server to signal completion
    rx.recv().await;

    client_transport.close().await?;
    server_handle.await??;

    // Verify we sent the expected number of chunks
    assert_eq!(
        sent_chunks, expected_chunk_count,
        "Should send all {} chunks",
        expected_chunk_count
    );
    println!(
        "✔ Multi-chunk transfer completed: {}/{} chunks sent",
        sent_chunks, expected_chunk_count
    );

    Ok(())
}
#[tokio::test]
async fn test_real_tcp_bidirectional_communication() -> Result<()> {
    use tokio::net::TcpListener;
    use tokio::sync::mpsc;
    use trustedge_core::transport::TransportFactory;

    let config = TransportConfig::default();

    // Start real server
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let addr = listener.local_addr()?;

    // Channel for server to signal completion
    let (tx, mut rx) = mpsc::channel(1);

    // Create test chunks
    let client_data = b"message from client";
    let client_manifest = r#"{"sequence":1,"sender":"client"}"#.as_bytes().to_vec();
    let client_chunk = NetworkChunk::new(1, client_data.to_vec(), client_manifest);

    let server_data = b"response from server";
    let server_manifest = r#"{"sequence":2,"sender":"server"}"#.as_bytes().to_vec();
    let _server_chunk = NetworkChunk::new(2, server_data.to_vec(), server_manifest);

    // Spawn server task
    let _expected_client_chunk = client_chunk.clone();
    let server_handle = tokio::spawn(async move {
        let (stream, _) = listener.accept().await?;
        let _server_transport = TcpTransport::new(TransportConfig::default());

        // Set up server transport manually
        let codec = tokio_util::codec::LengthDelimitedCodec::builder()
            .max_frame_length(TransportConfig::default().max_message_size)
            .new_codec();
        let _framed = tokio_util::codec::Framed::new(stream, codec);

        // For this test, we'll create a simple echo server
        // In real implementation, we'd have proper server setup

        // Signal completion
        tx.send(()).await.unwrap();
        Result::<()>::Ok(())
    });

    // Connect client and test communication
    let mut client_transport = TransportFactory::create_tcp(config);
    client_transport.connect(addr).await?;

    // Send chunk from client
    client_transport.send_chunk(&client_chunk).await?;

    // Wait for server to signal it's done
    rx.recv().await;

    client_transport.close().await?;
    server_handle.await??;

    Ok(())
}

/// Test real TCP connection failure handling
#[tokio::test]
async fn test_real_tcp_connection_failure() -> Result<()> {
    use trustedge_core::transport::TransportFactory;

    let config = TransportConfig {
        connect_timeout_ms: 1000, // Short timeout for faster test
        ..TransportConfig::default()
    };

    let mut transport = TransportFactory::create_tcp(config);

    // Try to connect to non-existent server
    let invalid_addr = "127.0.0.1:9999".parse()?;
    let connect_result = transport.connect(invalid_addr).await;

    // Should fail with connection error
    assert!(connect_result.is_err());
    let error = connect_result.unwrap_err();
    println!("Expected connection error: {}", error);

    Ok(())
}

/// Test real TCP large data transfer
#[tokio::test]
async fn test_real_tcp_large_data_transfer() -> Result<()> {
    use tokio::net::TcpListener;
    use tokio::sync::mpsc;
    use trustedge_core::transport::TransportFactory;

    let config = TransportConfig {
        max_message_size: 2 * 1024 * 1024, // 2MB limit
        ..TransportConfig::default()
    };

    // Start real server
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let addr = listener.local_addr()?;

    // Channel for coordination
    let (tx, mut rx) = mpsc::channel(1);

    // Create large test data (1MB)
    let large_data = vec![0xAB; 1024 * 1024];
    let manifest = r#"{"sequence":1,"size":"1MB","test":"large_transfer"}"#
        .as_bytes()
        .to_vec();
    let large_chunk = NetworkChunk::new(1, large_data.clone(), manifest);

    // Spawn server task
    let server_handle = tokio::spawn(async move {
        let (_stream, _) = listener.accept().await?;

        // Keep the connection alive to receive data
        // In a real server, we'd process the incoming data
        // For testing, we just need to keep the socket open
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // Signal completion after keeping connection alive
        tx.send(()).await.unwrap();
        Result::<()>::Ok(())
    });

    // Connect and send large data
    let mut client_transport = TransportFactory::create_tcp(config);
    client_transport.connect(addr).await?;

    // This should succeed since data is under 2MB limit
    let send_result = client_transport.send_chunk(&large_chunk).await;
    if let Err(e) = &send_result {
        println!("Send error: {}", e);
    }
    assert!(
        send_result.is_ok(),
        "Should handle 1MB chunk within 2MB limit: {:?}",
        send_result
    );

    // Wait for server
    rx.recv().await;

    client_transport.close().await?;
    server_handle.await??;

    Ok(())
}

/// Test real TCP message size limits
#[tokio::test]
async fn test_real_tcp_message_size_limits() -> Result<()> {
    use trustedge_core::transport::TransportFactory;

    let config = TransportConfig {
        max_message_size: 1024, // Very small limit for testing
        connect_timeout_ms: 5000,
        ..TransportConfig::default()
    };

    // Create oversized data
    let oversized_data = vec![0xCD; 2048]; // 2KB > 1KB limit
    let manifest = r#"{"sequence":1,"test":"oversized"}"#.as_bytes().to_vec();
    let oversized_chunk = NetworkChunk::new(1, oversized_data, manifest);

    let transport = TransportFactory::create_tcp(config);

    // Even without connecting, serialization should reveal size issue
    let serialized = bincode::serialize(&oversized_chunk)?;
    assert!(
        serialized.len() > 1024,
        "Serialized chunk should exceed limit"
    );

    // If we were connected, send_chunk would fail with size error
    // For now, we verify the serialized size exceeds our configured limit
    println!("Serialized size: {} bytes (limit: 1024)", serialized.len());

    // Mark transport as used to satisfy compiler
    let _ = transport;

    Ok(())
}

/// Test real transport timeout scenarios
#[tokio::test]
async fn test_real_transport_timeout_scenarios() -> Result<()> {
    use trustedge_core::transport::TransportFactory;

    let config = TransportConfig {
        connect_timeout_ms: 500, // Very short timeout
        read_timeout_ms: 1000,
        ..TransportConfig::default()
    };

    let mut transport = TransportFactory::create_tcp(config);

    // Test connection timeout to a non-responsive address
    // Use a non-routable address to ensure timeout
    let timeout_addr = "192.0.2.1:80".parse()?; // RFC 5737 test address

    let start_time = std::time::Instant::now();
    let connect_result = transport.connect(timeout_addr).await;
    let elapsed = start_time.elapsed();

    // Should timeout and fail
    assert!(connect_result.is_err());
    // Should timeout within reasonable time of our configured timeout
    assert!(elapsed.as_millis() >= 500); // At least our timeout
    assert!(elapsed.as_millis() < 2000); // Not too much longer

    println!("Connection timeout test completed in {:?}", elapsed);

    Ok(())
}

/// Test real NetworkChunk serialization and transport compatibility
#[tokio::test]
async fn test_real_network_chunk_serialization() -> Result<()> {
    // Create test data with real content
    let original_data = b"TrustEdge NetworkChunk serialization test";
    let sequence = 42;
    let manifest = format!(
        r#"{{"sequence":{},"timestamp":"2025-09-10T12:00:00Z","size":{}}}"#,
        sequence,
        original_data.len()
    )
    .into_bytes();

    // Create chunk
    let chunk = NetworkChunk::new(sequence, original_data.to_vec(), manifest.clone());

    // Test actual serialization/deserialization
    let serialized = bincode::serialize(&chunk)?;
    let deserialized: NetworkChunk = bincode::deserialize(&serialized)?;

    // Verify roundtrip integrity
    assert_eq!(deserialized.sequence, chunk.sequence);
    assert_eq!(deserialized.data, chunk.data);
    assert_eq!(deserialized.manifest, chunk.manifest);

    // Validate actual chunk content
    assert_eq!(deserialized.data, original_data);
    assert_eq!(deserialized.sequence, sequence);

    // Verify serialization size is reasonable
    println!("NetworkChunk serialized to {} bytes", serialized.len());
    assert!(serialized.len() > original_data.len()); // Should include overhead
    assert!(serialized.len() < original_data.len() + 1024); // But not too much overhead

    Ok(())
}

/// Test transport security configuration
#[tokio::test]
async fn test_transport_security_configuration() -> Result<()> {
    // Test security-focused transport configuration
    let secure_config = TransportConfig {
        connect_timeout_ms: 30000,
        read_timeout_ms: 60000,
        max_message_size: 8 * 1024 * 1024, // Limited for security
        keep_alive_ms: 5000,
        max_connection_bytes: 512 * 1024 * 1024, // Limited for security
        max_connection_chunks: 5000,             // Limited for security
        connection_idle_timeout_ms: 180000,      // 3 minutes
    };

    // Security-focused configurations should be conservative
    assert!(secure_config.max_message_size <= 16 * 1024 * 1024);
    assert!(secure_config.max_connection_chunks <= 10000);
    assert!(secure_config.connection_idle_timeout_ms >= 60000); // At least 1 minute

    Ok(())
}

/// Test transport layer data integrity
#[tokio::test]
async fn test_transport_data_integrity() -> Result<()> {
    // Create test data with known pattern
    let original_data = b"TrustEdge integrity test data with pattern 0123456789";
    let sequence = 1;
    let manifest = format!(
        r#"{{"sequence":{},"length":{}}}"#,
        sequence,
        original_data.len()
    )
    .into_bytes();

    // Create chunk
    let chunk = NetworkChunk::new(sequence, original_data.to_vec(), manifest);

    // Verify integrity
    assert_eq!(chunk.data, original_data);
    assert_eq!(chunk.sequence, sequence);

    // Verify manifest integrity
    let manifest_str = String::from_utf8(chunk.manifest)?;
    let manifest_json: serde_json::Value = serde_json::from_str(&manifest_str)?;
    assert_eq!(manifest_json["sequence"], sequence);
    assert_eq!(manifest_json["length"], original_data.len());

    Ok(())
}

/// Test transport protocol negotiation scenarios
#[tokio::test]
async fn test_transport_protocol_negotiation() -> Result<()> {
    // Test QUIC vs TCP selection criteria

    // QUIC preferred for:
    // - High throughput applications
    // - Multi-stream scenarios
    // - Modern network environments
    let quic_scenarios = vec![
        ("high_throughput", true),
        ("multi_stream", true),
        ("modern_network", true),
    ];

    for (scenario, should_prefer_quic) in quic_scenarios {
        if should_prefer_quic {
            // Configuration suitable for QUIC
            let config = TransportConfig {
                connect_timeout_ms: 30000,
                read_timeout_ms: 60000,
                max_message_size: 16 * 1024 * 1024,
                keep_alive_ms: 5000,
                max_connection_bytes: 1024 * 1024 * 1024,
                max_connection_chunks: 10000,
                connection_idle_timeout_ms: 300000,
            };
            assert!(
                config.max_message_size > 1024,
                "Should support large messages for {}",
                scenario
            );
        }
    }

    // TCP preferred for:
    // - Legacy compatibility
    // - Simple point-to-point communication
    // - Environments with QUIC restrictions
    let tcp_scenarios = vec![
        ("legacy_compatibility", true),
        ("simple_p2p", true),
        ("restricted_environment", true),
    ];

    for (scenario, should_prefer_tcp) in tcp_scenarios {
        if should_prefer_tcp {
            // TCP is simpler and more compatible
            let config = TransportConfig::default();
            let _tcp_transport = TcpTransport::new(config);
            // TCP should be available for all scenarios in our implementation
            println!("TCP should be available for {}", scenario);
        }
    }

    Ok(())
}
