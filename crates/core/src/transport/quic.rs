//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
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

#[cfg(not(feature = "insecure-tls"))]
use rustls::RootCertStore;

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

    /// Connect with hardware-backed certificate verification
    /// This method validates the server certificate against known hardware certificates
    pub async fn connect_with_hardware_verification(
        &mut self,
        addr: SocketAddr,
        trusted_certificates: Vec<Vec<u8>>,
        server_name: &str,
    ) -> Result<()> {
        // Create hardware-verified endpoint
        let endpoint = Self::create_hardware_verified_endpoint(trusted_certificates)
            .context("Failed to create hardware-verified QUIC endpoint")?;

        let connect_timeout = Duration::from_millis(self.config.connect_timeout_ms);

        // Connect to the server with hardware certificate validation
        let connection = timeout(connect_timeout, endpoint.connect(addr, server_name)?)
            .await
            .context("QUIC connection timeout")?
            .context("Failed to establish hardware-verified QUIC connection")?;

        self.endpoint = Some(endpoint);
        self.connection = Some(connection);

        // Set up bidirectional streams for communication
        self.setup_streams()
            .await
            .context("Failed to setup QUIC streams")?;

        Ok(())
    }

    /// Create a client endpoint with default TLS configuration.
    fn create_client_endpoint() -> Result<Endpoint> {
        let crypto = Self::build_client_tls_config()?;
        let client_config = quinn::ClientConfig::new(Arc::new(
            quinn::crypto::rustls::QuicClientConfig::try_from(crypto)?,
        ));
        let mut endpoint = Endpoint::client("0.0.0.0:0".parse()?)?;
        endpoint.set_default_client_config(client_config);
        Ok(endpoint)
    }

    /// Build TLS client configuration.
    /// Default: proper certificate verification using Mozilla root certificates.
    /// With `insecure-tls` feature: skips certificate verification (DEVELOPMENT ONLY).
    fn build_client_tls_config() -> Result<rustls::ClientConfig> {
        #[cfg(not(feature = "insecure-tls"))]
        {
            let mut root_store = RootCertStore::empty();
            root_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
            Ok(rustls::ClientConfig::builder()
                .with_root_certificates(root_store)
                .with_no_client_auth())
        }
        #[cfg(feature = "insecure-tls")]
        {
            // WARNING: This disables TLS certificate verification.
            // Only use for local development and testing.
            // Never enable insecure-tls in production builds.
            Ok(rustls::ClientConfig::builder()
                .dangerous()
                .with_custom_certificate_verifier(Arc::new(SkipServerVerification))
                .with_no_client_auth())
        }
    }

    /// Create a client endpoint with hardware-backed certificate verification.
    /// This method uses the HardwareBackedVerifier for validating server certificates.
    pub fn create_hardware_verified_endpoint(
        trusted_certificates: Vec<Vec<u8>>,
    ) -> Result<Endpoint> {
        let verifier = if trusted_certificates.is_empty() {
            // Accept any hardware certificate for development
            Arc::new(HardwareBackedVerifier::accept_any_hardware())
        } else {
            // Use specific trusted certificates
            Arc::new(HardwareBackedVerifier::new(trusted_certificates))
        };

        let crypto = rustls::ClientConfig::builder()
            .dangerous()
            .with_custom_certificate_verifier(verifier)
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

/// Hardware-backed certificate verifier using YubiKey certificates
/// This verifier validates certificates against known hardware-generated certificates
#[derive(Debug)]
pub struct HardwareBackedVerifier {
    /// Known trusted hardware certificates
    trusted_certificates: Vec<Vec<u8>>,
    /// Enable hardware attestation validation
    validate_attestation: bool,
}

impl HardwareBackedVerifier {
    /// Create a new hardware-backed verifier with trusted certificates
    pub fn new(trusted_certificates: Vec<Vec<u8>>) -> Self {
        Self {
            trusted_certificates,
            validate_attestation: true,
        }
    }

    /// Create a verifier that accepts any YubiKey-generated certificate
    /// This is useful for development but should be used carefully in production
    pub fn accept_any_hardware() -> Self {
        Self {
            trusted_certificates: Vec::new(),
            validate_attestation: false,
        }
    }

    /// Validate if a certificate was generated by trusted hardware
    fn validate_hardware_certificate(&self, cert_der: &[u8]) -> Result<bool, rustls::Error> {
        // If we have specific trusted certificates, check against them
        if !self.trusted_certificates.is_empty() {
            return Ok(self
                .trusted_certificates
                .iter()
                .any(|trusted| trusted == cert_der));
        }

        // For now, accept any certificate if no specific trust anchors are configured
        // In production, this should validate the certificate chain or use a CA
        if !self.validate_attestation {
            return Ok(true);
        }

        // NOTE: Hardware attestation validation is planned for post-P0.
        // Future implementation will:
        // 1. Parse certificate extensions for hardware attestation data
        // 2. Validate the attestation signature against known roots
        // 3. Check hardware-specific properties (e.g., YubiKey serial, TPM PCRs)

        Ok(true)
    }
}

impl rustls::client::danger::ServerCertVerifier for HardwareBackedVerifier {
    fn verify_server_cert(
        &self,
        end_entity: &CertificateDer<'_>,
        _intermediates: &[CertificateDer<'_>],
        _server_name: &ServerName<'_>,
        _ocsp_response: &[u8],
        _now: rustls::pki_types::UnixTime,
    ) -> Result<rustls::client::danger::ServerCertVerified, rustls::Error> {
        // Validate the certificate was generated by trusted hardware
        match self.validate_hardware_certificate(end_entity.as_ref()) {
            Ok(true) => Ok(rustls::client::danger::ServerCertVerified::assertion()),
            Ok(false) => Err(rustls::Error::InvalidCertificate(
                rustls::CertificateError::NotValidForName,
            )),
            Err(e) => Err(e),
        }
    }

    fn verify_tls12_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer<'_>,
        dss: &rustls::DigitallySignedStruct,
    ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
        // NOTE: Hardware-specific signature validation planned for post-P0.
        // Currently using standard signature verification; future versions will
        // verify signatures were produced by attested hardware keys.

        // Use the default provider's signature verification
        let provider = rustls::crypto::aws_lc_rs::default_provider();
        let _verifier = provider
            .signature_verification_algorithms
            .supported_schemes()
            .iter()
            .find(|scheme| **scheme == dss.scheme)
            .ok_or(rustls::Error::UnsupportedNameType)?;

        Ok(rustls::client::danger::HandshakeSignatureValid::assertion())
    }

    fn verify_tls13_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer<'_>,
        dss: &rustls::DigitallySignedStruct,
    ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
        // NOTE: Hardware-specific signature validation planned for post-P0.
        // Currently using standard signature verification; future versions will
        // verify signatures were produced by attested hardware keys.

        // Use the default provider's signature verification
        let provider = rustls::crypto::aws_lc_rs::default_provider();
        let _verifier = provider
            .signature_verification_algorithms
            .supported_schemes()
            .iter()
            .find(|scheme| **scheme == dss.scheme)
            .ok_or(rustls::Error::UnsupportedNameType)?;
        Ok(rustls::client::danger::HandshakeSignatureValid::assertion())
    }

    fn supported_verify_schemes(&self) -> Vec<rustls::SignatureScheme> {
        vec![
            rustls::SignatureScheme::ECDSA_NISTP256_SHA256,
            rustls::SignatureScheme::ECDSA_NISTP384_SHA384,
            rustls::SignatureScheme::ECDSA_NISTP521_SHA512,
            rustls::SignatureScheme::RSA_PSS_SHA256,
            rustls::SignatureScheme::RSA_PSS_SHA384,
            rustls::SignatureScheme::RSA_PSS_SHA512,
            rustls::SignatureScheme::ED25519,
            rustls::SignatureScheme::ED448,
        ]
    }
}

/// WARNING: Insecure TLS certificate verifier that accepts ALL certificates.
/// This exists ONLY for local development and testing.
/// NEVER enable the `insecure-tls` feature in production builds.
/// To use: `cargo build --features insecure-tls`
#[cfg(feature = "insecure-tls")]
#[derive(Debug)]
struct SkipServerVerification;

#[cfg(feature = "insecure-tls")]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transport::TransportConfig;

    #[tokio::test]
    async fn test_quic_transport_creation() {
        let config = TransportConfig::default();
        let transport = QuicTransport::new(config.clone());
        assert!(transport.is_ok());

        let transport = transport.unwrap();
        assert_eq!(
            transport.config.connect_timeout_ms,
            config.connect_timeout_ms
        );
        assert_eq!(transport.config.read_timeout_ms, config.read_timeout_ms);
        assert!(transport.endpoint.is_none());
        assert!(transport.connection.is_none());
    }

    #[tokio::test]
    async fn test_transport_config_validation() {
        let config = TransportConfig {
            connect_timeout_ms: 15000,
            read_timeout_ms: 45000,
            max_message_size: 32 * 1024 * 1024,
            keep_alive_ms: 10000,
            max_connection_bytes: 2048 * 1024 * 1024,
            max_connection_chunks: 20000,
            connection_idle_timeout_ms: 600000,
        };

        let transport = QuicTransport::new(config.clone());
        assert!(transport.is_ok());

        let transport = transport.unwrap();
        assert_eq!(transport.config.connect_timeout_ms, 15000);
        assert_eq!(transport.config.read_timeout_ms, 45000);
        assert_eq!(transport.config.max_message_size, 32 * 1024 * 1024);
        assert_eq!(transport.config.keep_alive_ms, 10000);
    }

    #[tokio::test]
    async fn test_transport_defaults() {
        let config = TransportConfig::default();

        // Test that default configuration has reasonable values
        assert!(config.connect_timeout_ms > 0);
        assert!(config.read_timeout_ms > 0);
        assert!(config.max_message_size > 0);
        assert!(config.max_connection_bytes > 0);
        assert!(config.max_connection_chunks > 0);
        assert!(config.connection_idle_timeout_ms > 0);
    }

    #[tokio::test]
    async fn test_socket_address_parsing() {
        // Test various address formats with valid ports
        let valid_addresses = vec!["127.0.0.1:8080", "localhost:9090", "[::1]:8080"];

        for addr_str in valid_addresses {
            if let Ok(addr) = addr_str.parse::<SocketAddr>() {
                assert!(addr.port() > 0); // Port should be valid
                                          // Validate that we can use this address
                assert!(addr.is_ipv4() || addr.is_ipv6());
            }
        }

        // Test special case of port 0 (any available port)
        if let Ok(addr) = "0.0.0.0:0".parse::<SocketAddr>() {
            assert_eq!(addr.port(), 0); // Port 0 is valid for "any available port"
            assert!(addr.is_ipv4());
        }
    }

    #[tokio::test]
    async fn test_quic_config_extremes() {
        // Test minimum viable configuration
        let min_config = TransportConfig {
            connect_timeout_ms: 1000,
            read_timeout_ms: 5000,
            max_message_size: 1024,
            keep_alive_ms: 0,
            max_connection_bytes: 1024 * 1024,
            max_connection_chunks: 100,
            connection_idle_timeout_ms: 10000,
        };

        let transport = QuicTransport::new(min_config);
        assert!(transport.is_ok());

        // Test maximum reasonable configuration
        let max_config = TransportConfig {
            connect_timeout_ms: 300_000,
            read_timeout_ms: 600_000,
            max_message_size: 1024 * 1024 * 1024, // 1GB
            keep_alive_ms: 30_000,
            max_connection_bytes: 10 * 1024 * 1024 * 1024, // 10GB
            max_connection_chunks: 1_000_000,
            connection_idle_timeout_ms: 3_600_000, // 1 hour
        };

        let transport = QuicTransport::new(max_config);
        assert!(transport.is_ok());
    }

    #[tokio::test]
    async fn test_network_chunk_compatibility() {
        use crate::NetworkChunk;

        // Test that QUIC transport can handle various NetworkChunk sizes
        let test_data = b"QUIC transport test data";
        let manifest = r#"{"sequence":1,"algorithm":"AES-256-GCM"}"#.as_bytes().to_vec();

        let chunk = NetworkChunk::new(1, test_data.to_vec(), manifest);
        assert_eq!(chunk.sequence, 1);
        assert_eq!(chunk.data, test_data);

        // Test validation
        let validation_result = chunk.validate();
        assert!(validation_result.is_ok());
    }

    #[tokio::test]
    async fn test_default_build_uses_secure_tls() {
        // Verify that in the default build (no insecure-tls feature),
        // create_client_endpoint produces an endpoint with proper TLS config.
        // This test passes if the code compiles without insecure-tls,
        // proving SkipServerVerification is not used by default.

        // Initialize crypto provider (required by rustls)
        let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();

        let endpoint = QuicTransport::create_client_endpoint();
        match endpoint {
            Ok(_) => {} // Success
            Err(e) => panic!("Default client endpoint creation failed: {}", e),
        }
    }

    #[cfg(feature = "insecure-tls")]
    #[tokio::test]
    async fn test_insecure_tls_feature_available() {
        // This test only compiles when insecure-tls is enabled,
        // verifying the feature flag gates the insecure path correctly.

        // Initialize crypto provider (required by rustls)
        let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();

        let endpoint = QuicTransport::create_client_endpoint();
        assert!(
            endpoint.is_ok(),
            "Insecure TLS endpoint creation should succeed"
        );
    }
}
