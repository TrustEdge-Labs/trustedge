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
///
/// # Security rationale for timeout defaults
///
/// All timeouts are bounded to prevent resource exhaustion attacks (slowloris, idle
/// connection hoarding). Defaults are conservative for edge-device workloads where
/// connections are short-lived and data transfers are bounded.
///
/// - **Connect timeout (10s):** Prevents SYN-flood resource holding. Edge devices on
///   unreliable networks should fail fast and retry rather than hold half-open connections.
/// - **Read timeout (30s):** Bounds the window for slow-read attacks. A 16 MB max message
///   at typical edge bandwidth (1 Mbps) transfers in ~128s; 30s covers per-frame reads
///   with margin while preventing indefinite blocking.
/// - **Idle timeout (5 min):** Reclaims connections that completed their transfer but
///   weren't explicitly closed. Shorter than typical HTTP keep-alive (which serves
///   connection reuse) because TrustEdge connections are single-purpose data transfers.
///   Set to 0 to disable (not recommended in production).
#[derive(Debug, Clone)]
pub struct TransportConfig {
    /// Connection timeout in milliseconds (default: 10,000 = 10s).
    /// Bounds half-open connection duration to prevent SYN-flood resource exhaustion.
    pub connect_timeout_ms: u64,
    /// Read timeout in milliseconds (default: 30,000 = 30s).
    /// Bounds per-frame read wait to prevent slow-read attacks.
    pub read_timeout_ms: u64,
    /// Maximum message size in bytes (default: 16 MB).
    pub max_message_size: usize,
    /// Keep-alive interval in milliseconds (0 = disabled).
    pub keep_alive_ms: u64,
    /// Maximum bytes per connection (0 = unlimited, default: 1 GB).
    pub max_connection_bytes: u64,
    /// Maximum chunks per connection (0 = unlimited, default: 10,000).
    pub max_connection_chunks: u64,
    /// Connection idle timeout in milliseconds (default: 300,000 = 5 min).
    /// Reclaims connections that completed transfer but weren't explicitly closed.
    /// Set to 0 to disable idle timeout (not recommended in production).
    pub connection_idle_timeout_ms: u64,
}

impl Default for TransportConfig {
    fn default() -> Self {
        Self {
            connect_timeout_ms: 10_000, // 10s — fail fast on unreliable networks
            read_timeout_ms: 30_000,    // 30s — per-frame read bound
            max_message_size: 16 * 1024 * 1024, // 16 MB
            keep_alive_ms: 0,           // Disabled — connections are single-purpose
            max_connection_bytes: 1024 * 1024 * 1024, // 1 GB per connection
            max_connection_chunks: 10_000, // 10k chunks per connection
            connection_idle_timeout_ms: 300_000, // 5 min — reclaim idle connections
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
