//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
/// Project: trustedge — Privacy and trust at the edge.
//
/// transport/mod.rs - Transport abstraction for TrustEdge
//
/// Provides pluggable transport implementations (TCP, QUIC) with a unified interface.
use crate::NetworkChunk;
use anyhow::Result;
use std::net::SocketAddr;

pub mod quic;
pub mod tcp;

/// Generic transport trait for network communication.
///
/// This trait abstracts over different transport protocols (TCP, QUIC, etc.)
/// to provide a unified interface for sending and receiving NetworkChunks.
#[async_trait::async_trait]
pub trait Transport: Send + Sync {
    /// Connect to a remote endpoint.
    async fn connect(&mut self, addr: SocketAddr) -> Result<()>;

    /// Send a network chunk to the connected endpoint.
    async fn send_chunk(&mut self, chunk: &NetworkChunk) -> Result<()>;

    /// Receive a network chunk from the connected endpoint.
    async fn receive_chunk(&mut self) -> Result<NetworkChunk>;

    /// Close the connection gracefully.
    async fn close(&mut self) -> Result<()>;

    /// Get the local address of the connection (if available).
    fn local_addr(&self) -> Result<SocketAddr>;

    /// Get the remote address of the connection (if available).
    fn peer_addr(&self) -> Result<SocketAddr>;
}

/// Transport configuration options.
#[derive(Debug, Clone)]
pub struct TransportConfig {
    /// Connection timeout in milliseconds.
    pub connect_timeout_ms: u64,
    /// Read timeout in milliseconds.
    pub read_timeout_ms: u64,
    /// Maximum message size.
    pub max_message_size: usize,
    /// Keep-alive interval in milliseconds (0 = disabled).
    pub keep_alive_ms: u64,
    /// Maximum bytes per connection (0 = unlimited).
    pub max_connection_bytes: u64,
    /// Maximum chunks per connection (0 = unlimited).
    pub max_connection_chunks: u64,
    /// Connection idle timeout in milliseconds.
    pub connection_idle_timeout_ms: u64,
}

impl Default for TransportConfig {
    fn default() -> Self {
        Self {
            connect_timeout_ms: 10_000,               // 10 seconds
            read_timeout_ms: 30_000,                  // 30 seconds
            max_message_size: 16 * 1024 * 1024,       // 16 MB
            keep_alive_ms: 0,                         // Disabled by default
            max_connection_bytes: 1024 * 1024 * 1024, // 1 GB per connection
            max_connection_chunks: 10_000,            // 10k chunks per connection
            connection_idle_timeout_ms: 300_000,      // 5 minutes
        }
    }
}

/// Transport factory for creating transport instances.
pub struct TransportFactory;

impl TransportFactory {
    /// Create a TCP transport instance.
    pub fn create_tcp(config: TransportConfig) -> Box<dyn Transport> {
        Box::new(tcp::TcpTransport::new(config))
    }

    /// Create a QUIC transport instance.
    pub fn create_quic(config: TransportConfig) -> Result<Box<dyn Transport>> {
        Ok(Box::new(quic::QuicTransport::new(config)?))
    }
}
