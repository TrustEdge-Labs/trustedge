//
// Copyright (c) 2025 John Turner
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
use std::net::SocketAddr;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::timeout;

/// TCP transport implementation.
pub struct TcpTransport {
    config: TransportConfig,
    stream: Option<TcpStream>,
}

impl TcpTransport {
    /// Create a new TCP transport with the given configuration.
    pub fn new(config: TransportConfig) -> Self {
        Self {
            config,
            stream: None,
        }
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

        self.stream = Some(stream);
        Ok(())
    }

    async fn send_chunk(&mut self, chunk: &NetworkChunk) -> Result<()> {
        let stream = self.stream.as_mut().context("Transport not connected")?;

        // Serialize the chunk
        let serialized = bincode::serialize(chunk).context("Failed to serialize NetworkChunk")?;

        // Check message size limit
        if serialized.len() > self.config.max_message_size {
            anyhow::bail!(
                "Message too large: {} bytes (max: {})",
                serialized.len(),
                self.config.max_message_size
            );
        }

        // Send length prefix (4 bytes, big-endian)
        let len = serialized.len() as u32;
        let len_bytes = len.to_be_bytes();
        stream
            .write_all(&len_bytes)
            .await
            .context("Failed to write message length")?;

        // Send the actual data
        stream
            .write_all(&serialized)
            .await
            .context("Failed to write chunk data")?;

        // Ensure data is sent immediately
        stream.flush().await.context("Failed to flush stream")?;

        Ok(())
    }

    async fn receive_chunk(&mut self) -> Result<NetworkChunk> {
        let stream = self.stream.as_mut().context("Transport not connected")?;

        let read_timeout = Duration::from_millis(self.config.read_timeout_ms);

        // Read length prefix (4 bytes, big-endian)
        let mut len_bytes = [0u8; 4];
        timeout(read_timeout, stream.read_exact(&mut len_bytes))
            .await
            .context("Read timeout while reading message length")?
            .context("Failed to read message length")?;

        let len = u32::from_be_bytes(len_bytes) as usize;

        // Check message size limit
        if len > self.config.max_message_size {
            anyhow::bail!(
                "Received message too large: {} bytes (max: {})",
                len,
                self.config.max_message_size
            );
        }

        // Read the actual message
        let mut buffer = vec![0u8; len];
        timeout(read_timeout, stream.read_exact(&mut buffer))
            .await
            .context("Read timeout while reading message data")?
            .context("Failed to read message data")?;

        // Deserialize the chunk
        let chunk: NetworkChunk =
            bincode::deserialize(&buffer).context("Failed to deserialize NetworkChunk")?;

        Ok(chunk)
    }

    async fn close(&mut self) -> Result<()> {
        if let Some(mut stream) = self.stream.take() {
            stream
                .shutdown()
                .await
                .context("Failed to shutdown TCP stream")?;
        }
        Ok(())
    }

    fn local_addr(&self) -> Result<SocketAddr> {
        self.stream
            .as_ref()
            .context("Transport not connected")?
            .local_addr()
            .context("Failed to get local address")
    }

    fn peer_addr(&self) -> Result<SocketAddr> {
        self.stream
            .as_ref()
            .context("Transport not connected")?
            .peer_addr()
            .context("Failed to get peer address")
    }
}

impl Drop for TcpTransport {
    fn drop(&mut self) {
        if self.stream.is_some() {
            // Note: We can't await in Drop, so this is a best-effort cleanup
            // In production, users should call close() explicitly
        }
    }
}
