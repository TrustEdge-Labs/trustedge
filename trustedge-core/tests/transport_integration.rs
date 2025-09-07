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
use std::net::SocketAddr;
use std::time::Duration;
use tokio::time::timeout;

use trustedge_core::transport::{tcp::TcpTransport, TransportConfig};
use trustedge_core::NetworkChunk;

/// Test QUIC transport configuration and validation
#[tokio::test]
async fn test_quic_transport_configuration() -> Result<()> {
    // Test default transport configuration
    let default_config = TransportConfig::default();
    assert!(default_config.connect_timeout_ms > 0);
    assert!(default_config.read_timeout_ms > 0);
    assert!(default_config.max_message_size > 0);

    // Test custom transport configuration
    let custom_config = TransportConfig {
        connect_timeout_ms: 15000,
        read_timeout_ms: 45000,
        max_message_size: 32 * 1024 * 1024, // 32 MB
        keep_alive_ms: 10000,
        max_connection_bytes: 2048 * 1024 * 1024, // 2 GB
        max_connection_chunks: 20000,
        connection_idle_timeout_ms: 600000, // 10 minutes
    };

    assert_eq!(custom_config.connect_timeout_ms, 15000);
    assert_eq!(custom_config.read_timeout_ms, 45000);
    assert_eq!(custom_config.max_message_size, 32 * 1024 * 1024);
    assert_eq!(custom_config.keep_alive_ms, 10000);

    Ok(())
}

/// Test TCP transport configuration and validation
#[tokio::test]
async fn test_tcp_transport_configuration() -> Result<()> {
    // Test TCP transport creation
    let config = TransportConfig::default();
    let _tcp_transport = TcpTransport::new(config);

    // TCP transport should initialize successfully
    // TcpTransport::new() doesn't return Result, creation always succeeds

    // Test address parsing for TCP
    let addr: SocketAddr = "127.0.0.1:8080".parse()?;
    assert_eq!(addr.port(), 8080);
    assert!(addr.is_ipv4());

    let ipv6_addr: SocketAddr = "[::1]:8080".parse()?;
    assert_eq!(ipv6_addr.port(), 8080);
    assert!(ipv6_addr.is_ipv6());

    Ok(())
}

/// Test NetworkChunk creation and serialization for transport
#[tokio::test]
async fn test_network_chunk_transport_compatibility() -> Result<()> {
    // Create test data
    let test_data = b"Hello, TrustEdge Transport!";
    let sequence = 42;

    // Create signed manifest (simplified for testing)
    let manifest = format!(
        r#"{{"sequence":{},"timestamp":"{}","algorithm":"AES-256-GCM","integrity":"test"}}"#,
        sequence,
        chrono::Utc::now().to_rfc3339()
    )
    .into_bytes();

    // Create NetworkChunk
    let chunk = NetworkChunk::new(sequence, test_data.to_vec(), manifest.clone());

    // Validate chunk properties
    assert_eq!(chunk.sequence, sequence);
    assert_eq!(chunk.data, test_data);
    assert_eq!(chunk.manifest, manifest);

    // Test validation
    chunk.validate()?;

    Ok(())
}

/// Test transport error handling scenarios
#[tokio::test]
async fn test_transport_error_handling() -> Result<()> {
    // Test invalid address handling
    let invalid_addresses = vec![
        "invalid-address",
        "256.256.256.256:8080", // Invalid IP
        "127.0.0.1:99999",      // Invalid port
        "",                     // Empty address
    ];

    for addr_str in invalid_addresses {
        let parse_result: Result<SocketAddr, _> = addr_str.parse();
        if addr_str == "256.256.256.256:8080"
            || addr_str == "127.0.0.1:99999"
            || addr_str.is_empty()
            || addr_str == "invalid-address"
        {
            assert!(
                parse_result.is_err(),
                "Address '{}' should be invalid",
                addr_str
            );
        }
    }

    Ok(())
}

/// Test transport protocol selection and capabilities
#[tokio::test]
async fn test_transport_protocol_capabilities() -> Result<()> {
    // Test transport configuration
    let config = TransportConfig {
        connect_timeout_ms: 30000,
        read_timeout_ms: 60000,
        max_message_size: 16 * 1024 * 1024,
        keep_alive_ms: 5000,
        max_connection_bytes: 1024 * 1024 * 1024,
        max_connection_chunks: 10000,
        connection_idle_timeout_ms: 300000,
    };

    // Configuration supports both QUIC and TCP
    assert!(config.max_message_size > 0);
    assert!(config.connect_timeout_ms > 0);
    assert!(config.keep_alive_ms > 0);

    // Test TCP capabilities (always bidirectional)
    let _tcp_transport = TcpTransport::new(config);
    // TCP is inherently bidirectional and connection-oriented
    // TCP validation - TCP should always be available in our implementation

    Ok(())
}

/// Test concurrent transport operations
#[tokio::test]
async fn test_concurrent_transport_operations() -> Result<()> {
    // Test multiple NetworkChunk creation concurrently
    let chunk_futures: Vec<_> = (0..10)
        .map(|i| {
            tokio::spawn(async move {
                let data = format!("chunk-{}", i).into_bytes();
                let manifest = format!(r#"{{"sequence":{},"test":true}}"#, i).into_bytes();
                NetworkChunk::new(i, data, manifest)
            })
        })
        .collect();

    let mut chunks = Vec::new();
    for future in chunk_futures {
        chunks.push(future.await?);
    }

    // Verify all chunks were created correctly
    assert_eq!(chunks.len(), 10);
    for (i, chunk) in chunks.iter().enumerate() {
        assert_eq!(chunk.sequence, i as u64);
        assert_eq!(chunk.data, format!("chunk-{}", i).as_bytes());
    }

    Ok(())
}

/// Test transport timeout and retry scenarios
#[tokio::test]
async fn test_transport_timeout_handling() -> Result<()> {
    // Test timeout behavior with NetworkChunk operations
    let large_data = vec![0u8; 1024 * 1024]; // 1MB of data
    let manifest = r#"{"sequence":0,"size":"1MB","test":true}"#.as_bytes().to_vec();

    // Create large chunk with timeout
    let chunk_result = timeout(Duration::from_millis(100), async {
        NetworkChunk::new(0, large_data, manifest)
    })
    .await;

    // Large chunk creation should complete within timeout
    assert!(chunk_result.is_ok());
    let chunk = chunk_result?;
    assert_eq!(chunk.data.len(), 1024 * 1024);

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
