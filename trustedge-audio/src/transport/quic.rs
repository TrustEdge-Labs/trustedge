//
// Copyright (c) 2025 John Turner
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
/// Project: trustedge — Privacy and trust at the edge.
//
/// transport/quic.rs - QUIC transport implementation
//
/// Provides QUIC-based transport for NetworkChunks with built-in encryption and reliability.
use super::{Transport, TransportConfig};
use crate::NetworkChunk;
use anyhow::{Context, Result};
use quinn::{Connection, Endpoint, RecvStream, SendStream};
use rustls::pki_types::{CertificateDer, ServerName};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::AsyncWriteExt;
use tokio::time::timeout;

/// QUIC transport implementation.
pub struct QuicTransport {
    config: TransportConfig,
    endpoint: Option<Endpoint>,
    connection: Option<Connection>,
    send_stream: Option<SendStream>,
    recv_stream: Option<RecvStream>,
}

impl QuicTransport {
    /// Create a new QUIC transport with the given configuration.
    pub fn new(config: TransportConfig) -> Result<Self> {
        Ok(Self {
            config,
            endpoint: None,
            connection: None,
            send_stream: None,
            recv_stream: None,
        })
    }

    /// Create a client endpoint with default TLS configuration.
    fn create_client_endpoint() -> Result<Endpoint> {
        // Create a rustls client config that accepts any certificate
        // Note: In production, you should use proper certificate validation
        let crypto = rustls::ClientConfig::builder()
            .dangerous()
            .with_custom_certificate_verifier(Arc::new(SkipServerVerification))
            .with_no_client_auth();

        let client_config = quinn::ClientConfig::new(Arc::new(
            quinn::crypto::rustls::QuicClientConfig::try_from(crypto)?,
        ));

        let mut endpoint = Endpoint::client("0.0.0.0:0".parse()?)?;
        endpoint.set_default_client_config(client_config);

        Ok(endpoint)
    }

    /// Establish a bi-directional stream for communication.
    async fn setup_streams(&mut self) -> Result<()> {
        let connection = self
            .connection
            .as_ref()
            .context("No QUIC connection available")?;

        // Open a bidirectional stream
        let (send_stream, recv_stream) = connection
            .open_bi()
            .await
            .context("Failed to open bidirectional stream")?;

        self.send_stream = Some(send_stream);
        self.recv_stream = Some(recv_stream);

        Ok(())
    }
}

#[async_trait::async_trait]
impl Transport for QuicTransport {
    async fn connect(&mut self, addr: SocketAddr) -> Result<()> {
        // Create client endpoint
        let endpoint =
            Self::create_client_endpoint().context("Failed to create QUIC client endpoint")?;

        let connect_timeout = Duration::from_millis(self.config.connect_timeout_ms);

        // Connect to the server
        let connection = timeout(
            connect_timeout,
            endpoint.connect(addr, "localhost")?, // Note: server name should match certificate
        )
        .await
        .context("QUIC connection timeout")?
        .context("Failed to establish QUIC connection")?;

        self.endpoint = Some(endpoint);
        self.connection = Some(connection);

        // Set up bidirectional streams
        self.setup_streams()
            .await
            .context("Failed to setup QUIC streams")?;

        Ok(())
    }

    async fn send_chunk(&mut self, chunk: &NetworkChunk) -> Result<()> {
        let send_stream = self
            .send_stream
            .as_mut()
            .context("No QUIC send stream available")?;

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
        send_stream
            .write_all(&len_bytes)
            .await
            .context("Failed to write message length")?;

        // Send the actual data
        send_stream
            .write_all(&serialized)
            .await
            .context("Failed to write chunk data")?;

        // Ensure data is sent immediately
        send_stream
            .flush()
            .await
            .context("Failed to flush QUIC stream")?;

        Ok(())
    }

    async fn receive_chunk(&mut self) -> Result<NetworkChunk> {
        let recv_stream = self
            .recv_stream
            .as_mut()
            .context("No QUIC receive stream available")?;

        let _read_timeout = Duration::from_millis(self.config.read_timeout_ms);

        // Read length prefix (4 bytes, big-endian)
        let len_bytes = recv_stream
            .read_chunk(4, false)
            .await
            .context("Read timeout while reading message length")?
            .context("Failed to read message length")?
            .bytes;

        if len_bytes.len() != 4 {
            anyhow::bail!("Failed to read complete message length");
        }

        let len =
            u32::from_be_bytes([len_bytes[0], len_bytes[1], len_bytes[2], len_bytes[3]]) as usize;

        // Check message size limit
        if len > self.config.max_message_size {
            anyhow::bail!(
                "Received message too large: {} bytes (max: {})",
                len,
                self.config.max_message_size
            );
        }

        // Read the actual message
        let message_bytes = recv_stream
            .read_chunk(len, true)
            .await
            .context("Read timeout while reading message data")?
            .context("Failed to read message data")?
            .bytes;

        if message_bytes.len() != len {
            anyhow::bail!("Failed to read complete message data");
        }

        // Deserialize the chunk
        let chunk: NetworkChunk =
            bincode::deserialize(&message_bytes).context("Failed to deserialize NetworkChunk")?;

        Ok(chunk)
    }

    async fn close(&mut self) -> Result<()> {
        // Close streams first
        if let Some(mut send_stream) = self.send_stream.take() {
            let _ = send_stream.finish();
        }
        self.recv_stream.take();

        // Close connection
        if let Some(connection) = self.connection.take() {
            connection.close(0u32.into(), b"Normal closure");

            // Wait a bit for graceful closure
            let _ = timeout(Duration::from_millis(1000), connection.closed()).await;
        }

        // Close endpoint
        if let Some(endpoint) = self.endpoint.take() {
            endpoint.close(0u32.into(), b"Normal closure");

            // Wait for endpoint to close
            let _ = timeout(Duration::from_millis(1000), endpoint.wait_idle()).await;
        }

        Ok(())
    }

    fn local_addr(&self) -> Result<SocketAddr> {
        self.endpoint
            .as_ref()
            .context("QUIC endpoint not available")?
            .local_addr()
            .context("Failed to get local address")
    }

    fn peer_addr(&self) -> Result<SocketAddr> {
        self.connection
            .as_ref()
            .context("QUIC connection not available")?
            .remote_address()
            .pipe(Ok)
    }
}

impl Drop for QuicTransport {
    fn drop(&mut self) {
        // Best-effort cleanup
        if let Some(connection) = &self.connection {
            connection.close(0u32.into(), b"Transport dropped");
        }
        if let Some(endpoint) = &self.endpoint {
            endpoint.close(0u32.into(), b"Transport dropped");
        }
    }
}

/// A helper struct that skips certificate verification.
/// WARNING: This is insecure and should only be used for development/testing.
#[derive(Debug)]
struct SkipServerVerification;

impl rustls::client::danger::ServerCertVerifier for SkipServerVerification {
    fn verify_server_cert(
        &self,
        _end_entity: &CertificateDer<'_>,
        _intermediates: &[CertificateDer<'_>],
        _server_name: &ServerName<'_>,
        _ocsp_response: &[u8],
        _now: rustls::pki_types::UnixTime,
    ) -> Result<rustls::client::danger::ServerCertVerified, rustls::Error> {
        Ok(rustls::client::danger::ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer<'_>,
        _dss: &rustls::DigitallySignedStruct,
    ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
        Ok(rustls::client::danger::HandshakeSignatureValid::assertion())
    }

    fn verify_tls13_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer<'_>,
        _dss: &rustls::DigitallySignedStruct,
    ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
        Ok(rustls::client::danger::HandshakeSignatureValid::assertion())
    }

    fn supported_verify_schemes(&self) -> Vec<rustls::SignatureScheme> {
        vec![
            rustls::SignatureScheme::RSA_PKCS1_SHA1,
            rustls::SignatureScheme::ECDSA_SHA1_Legacy,
            rustls::SignatureScheme::RSA_PKCS1_SHA256,
            rustls::SignatureScheme::ECDSA_NISTP256_SHA256,
            rustls::SignatureScheme::RSA_PKCS1_SHA384,
            rustls::SignatureScheme::ECDSA_NISTP384_SHA384,
            rustls::SignatureScheme::RSA_PKCS1_SHA512,
            rustls::SignatureScheme::ECDSA_NISTP521_SHA512,
            rustls::SignatureScheme::RSA_PSS_SHA256,
            rustls::SignatureScheme::RSA_PSS_SHA384,
            rustls::SignatureScheme::RSA_PSS_SHA512,
            rustls::SignatureScheme::ED25519,
            rustls::SignatureScheme::ED448,
        ]
    }
}

// Helper trait for piping operations
trait Pipe<T> {
    fn pipe<F, U>(self, f: F) -> U
    where
        F: FnOnce(T) -> U;
}

impl<T> Pipe<T> for T {
    fn pipe<F, U>(self, f: F) -> U
    where
        F: FnOnce(T) -> U,
    {
        f(self)
    }
}
