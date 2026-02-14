//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
/// Project: trustedge — Privacy and trust at the edge.
//
/// transport/tcp.rs - TCP transport implementation
//
/// Provides TCP-based transport for NetworkChunks with proper error handling.
use super::{Transport, TransportConfig};
use crate::NetworkChunk;
use anyhow::{Context, Result};
use futures_util::{SinkExt, StreamExt};
use std::net::SocketAddr;
use std::time::{Duration, Instant};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::time::timeout;
use tokio_util::codec::{Framed, LengthDelimitedCodec};

/// TCP transport implementation.
pub struct TcpTransport {
    config: TransportConfig,
    framed: Option<Framed<TcpStream, LengthDelimitedCodec>>,
    // Connection tracking
    bytes_received: u64,
    bytes_sent: u64,
    chunks_received: u64,
    chunks_sent: u64,
    last_activity: Instant,
}

impl TcpTransport {
    /// Create a new TCP transport with the given configuration.
    pub fn new(config: TransportConfig) -> Self {
        let now = Instant::now();
        Self {
            config,
            framed: None,
            bytes_received: 0,
            bytes_sent: 0,
            chunks_received: 0,
            chunks_sent: 0,
            last_activity: now,
        }
    }

    /// Check if connection limits are exceeded.
    fn check_connection_limits(&self) -> Result<()> {
        // Check byte limits
        if self.config.max_connection_bytes > 0 {
            let total_bytes = self.bytes_received + self.bytes_sent;
            if total_bytes > self.config.max_connection_bytes {
                anyhow::bail!(
                    "Connection byte limit exceeded: {} bytes (max: {})",
                    total_bytes,
                    self.config.max_connection_bytes
                );
            }
        }

        // Check chunk limits
        if self.config.max_connection_chunks > 0 {
            let total_chunks = self.chunks_received + self.chunks_sent;
            if total_chunks > self.config.max_connection_chunks {
                anyhow::bail!(
                    "Connection chunk limit exceeded: {} chunks (max: {})",
                    total_chunks,
                    self.config.max_connection_chunks
                );
            }
        }

        // Check idle timeout
        if self.config.connection_idle_timeout_ms > 0 {
            let idle_duration = self.last_activity.elapsed();
            let idle_timeout = Duration::from_millis(self.config.connection_idle_timeout_ms);
            if idle_duration > idle_timeout {
                anyhow::bail!(
                    "Connection idle timeout: {:?} (max: {:?})",
                    idle_duration,
                    idle_timeout
                );
            }
        }

        Ok(())
    }

    /// Update activity timestamp and check limits.
    fn update_activity(&mut self) -> Result<()> {
        self.last_activity = Instant::now();
        self.check_connection_limits()
    }
}

#[async_trait::async_trait]
impl Transport for TcpTransport {
    async fn connect(&mut self, addr: SocketAddr) -> Result<()> {
        let connect_timeout = Duration::from_millis(self.config.connect_timeout_ms);

        let stream = timeout(connect_timeout, TcpStream::connect(addr))
            .await
            .context("Connection timeout")?
            .context("Failed to connect to server")?;

        // Configure TCP socket options
        stream
            .set_nodelay(true)
            .context("Failed to set TCP_NODELAY")?;

        // Create framed transport with length-delimited codec
        let codec = LengthDelimitedCodec::builder()
            .max_frame_length(self.config.max_message_size)
            .new_codec();

        self.framed = Some(Framed::new(stream, codec));
        self.update_activity()?;

        Ok(())
    }

    async fn send_chunk(&mut self, chunk: &NetworkChunk) -> Result<()> {
        self.check_connection_limits()?;

        let framed = self.framed.as_mut().context("Transport not connected")?;

        // Serialize the chunk
        let serialized = bincode::serialize(chunk).context("Failed to serialize NetworkChunk")?;

        // Check message size limit (codec will also enforce this)
        if serialized.len() > self.config.max_message_size {
            anyhow::bail!(
                "Message too large: {} bytes (max: {})",
                serialized.len(),
                self.config.max_message_size
            );
        }

        // Send through the framed transport (automatically handles length prefix)
        let serialized_len = serialized.len();
        framed
            .send(serialized.into())
            .await
            .context("Failed to send chunk")?;

        // Update tracking
        self.bytes_sent += serialized_len as u64;
        self.chunks_sent += 1;
        self.update_activity()?;

        Ok(())
    }

    async fn receive_chunk(&mut self) -> Result<NetworkChunk> {
        self.check_connection_limits()?;

        let framed = self.framed.as_mut().context("Transport not connected")?;
        let read_timeout = Duration::from_millis(self.config.read_timeout_ms);

        // Receive frame with timeout (automatically handles length prefix)
        let frame = timeout(read_timeout, framed.next())
            .await
            .context("Read timeout while receiving chunk")?
            .ok_or_else(|| anyhow::anyhow!("Connection closed by peer"))?
            .context("Failed to receive frame")?;

        // Deserialize the chunk
        let chunk: NetworkChunk =
            bincode::deserialize(&frame).context("Failed to deserialize NetworkChunk")?;

        // Update tracking
        self.bytes_received += frame.len() as u64;
        self.chunks_received += 1;
        self.update_activity()?;

        Ok(chunk)
    }

    async fn close(&mut self) -> Result<()> {
        if let Some(framed) = self.framed.take() {
            let mut stream = framed.into_parts().io;
            stream
                .shutdown()
                .await
                .context("Failed to shutdown TCP stream")?;
        }
        Ok(())
    }

    fn local_addr(&self) -> Result<SocketAddr> {
        self.framed
            .as_ref()
            .context("Transport not connected")?
            .get_ref()
            .local_addr()
            .context("Failed to get local address")
    }

    fn peer_addr(&self) -> Result<SocketAddr> {
        self.framed
            .as_ref()
            .context("Transport not connected")?
            .get_ref()
            .peer_addr()
            .context("Failed to get peer address")
    }
}

impl Drop for TcpTransport {
    fn drop(&mut self) {
        if self.framed.is_some() {
            // Note: We can't await in Drop, so this is a best-effort cleanup
            // In production, users should call close() explicitly
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_tcp_transport_creation() {
        let config = TransportConfig::default();
        let transport = TcpTransport::new(config.clone());

        assert_eq!(
            transport.config.connect_timeout_ms,
            config.connect_timeout_ms
        );
        assert_eq!(transport.config.read_timeout_ms, config.read_timeout_ms);
        assert!(transport.framed.is_none());
        assert_eq!(transport.bytes_received, 0);
        assert_eq!(transport.bytes_sent, 0);
        assert_eq!(transport.chunks_received, 0);
        assert_eq!(transport.chunks_sent, 0);
    }

    #[tokio::test]
    async fn test_tcp_transport_stats() {
        let config = TransportConfig::default();
        let transport = TcpTransport::new(config);

        // Initial stats should be zero
        assert_eq!(transport.bytes_received, 0);
        assert_eq!(transport.bytes_sent, 0);
        assert_eq!(transport.chunks_received, 0);
        assert_eq!(transport.chunks_sent, 0);

        // Timestamp should be initialized
        assert!(transport.last_activity <= Instant::now());
    }

    #[tokio::test]
    async fn test_tcp_transport_config_validation() {
        let custom_config = TransportConfig {
            connect_timeout_ms: 45000,
            read_timeout_ms: 90000,
            max_message_size: 32 * 1024 * 1024,
            keep_alive_ms: 15000,
            max_connection_bytes: 2048 * 1024 * 1024,
            max_connection_chunks: 20000,
            connection_idle_timeout_ms: 600000,
        };

        let transport = TcpTransport::new(custom_config.clone());
        assert_eq!(transport.config.connect_timeout_ms, 45000);
        assert_eq!(transport.config.keep_alive_ms, 15000);
    }

    #[tokio::test]
    async fn test_address_parsing_for_tcp() {
        // Test various TCP-compatible addresses
        let addresses = vec![
            "127.0.0.1:8080",
            "0.0.0.0:9090",
            "[::1]:8080",
            "localhost:3000",
        ];

        for addr_str in addresses {
            if addr_str == "localhost:3000" {
                // localhost requires DNS resolution, skip in unit test
                continue;
            }

            if let Ok(addr) = addr_str.parse::<SocketAddr>() {
                // Validate TCP can use this address
                assert!(addr.port() > 0 || addr_str.contains(":0")); // Port 0 is valid for auto-assignment
                assert!(addr.is_ipv4() || addr.is_ipv6());
            }
        }
    }

    #[tokio::test]
    async fn test_network_chunk_framing() {
        use crate::NetworkChunk;

        // Test that TCP transport can handle NetworkChunk serialization
        let test_data = b"TCP framing test data";
        let manifest = r#"{"sequence":2,"transport":"tcp","length":21}"#.as_bytes().to_vec();

        let chunk = NetworkChunk::new(2, test_data.to_vec(), manifest);
        assert_eq!(chunk.sequence, 2);
        assert_eq!(chunk.data, test_data);

        // Test validation for TCP framing
        let validation_result = chunk.validate();
        assert!(validation_result.is_ok());
    }

    #[tokio::test]
    async fn test_transport_config_defaults() {
        let default_config = TransportConfig::default();
        let transport = TcpTransport::new(default_config);

        // Verify default configuration is sensible for TCP
        assert!(transport.config.connect_timeout_ms > 0);
        assert!(transport.config.connect_timeout_ms <= 300_000); // 5 minutes max

        // Verify other default values
        assert!(transport.config.read_timeout_ms > 0);
        assert!(transport.config.max_message_size > 0);
    }

    #[tokio::test]
    async fn test_tcp_transport_cleanup() {
        let config = TransportConfig::default();
        let transport = TcpTransport::new(config);

        // Test that Drop implementation doesn't panic
        drop(transport);
        // If we reach here, drop succeeded without panic
        // TCP transport creation always succeeds in our design
    }

    #[tokio::test]
    async fn test_tcp_connection_state() {
        let config = TransportConfig::default();
        let transport = TcpTransport::new(config);

        // Initially not connected
        assert!(transport.framed.is_none());

        // Connection timestamp should be initialized
        let activity_time = transport.last_activity;

        assert!(activity_time <= Instant::now());
    }

    #[tokio::test]
    async fn test_large_data_handling() {
        use crate::NetworkChunk;

        // Test TCP transport with larger data chunks
        let large_data = vec![0xAB; 64 * 1024]; // 64KB test data
        let manifest = format!(
            r#"{{"sequence":3,"size":{},"transport":"tcp"}}"#,
            large_data.len()
        )
        .into_bytes();

        let chunk = NetworkChunk::new(3, large_data.clone(), manifest);
        assert_eq!(chunk.sequence, 3);
        assert_eq!(chunk.data.len(), 64 * 1024);

        // Verify validation works for large chunks
        let validation_result = chunk.validate();
        assert!(validation_result.is_ok());
    }
}
