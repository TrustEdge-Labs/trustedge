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
#[cfg(feature = "yubikey")]
use crate::backends::HardwareCertificate;
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

    /// Create a server endpoint with hardware-backed certificate
    /// Uses YubiKey-generated certificates for server authentication
    #[cfg(feature = "yubikey")]
    pub fn create_hardware_server_endpoint(
        listen_addr: SocketAddr,
        hardware_cert: &HardwareCertificate,
    ) -> Result<Endpoint> {
        // Parse the DER certificate
        let cert_der = CertificateDer::from(hardware_cert.certificate_der.clone());

        // For now, we'll create a placeholder private key since YubiKey holds the actual key
        // In a full implementation, this would require custom key provider integration
        let private_key = Self::create_placeholder_private_key()?;

        let server_config = rustls::ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(vec![cert_der], private_key)
            .context("Failed to create server config with hardware certificate")?;

        let server_config = quinn::ServerConfig::with_crypto(Arc::new(
            quinn::crypto::rustls::QuicServerConfig::try_from(server_config)?,
        ));

        let endpoint = Endpoint::server(server_config, listen_addr)
            .context("Failed to create server endpoint")?;

        Ok(endpoint)
    }

    /// Create a placeholder private key for demo purposes
    /// In production, this would integrate with hardware key providers
    #[cfg(feature = "yubikey")]
    fn create_placeholder_private_key() -> Result<rustls::pki_types::PrivateKeyDer<'static>> {
        // Generate a temporary ECDSA key for demo purposes
        // This is NOT secure and should be replaced with proper hardware integration

        // Create a minimal valid Ed25519 PKCS#8 key structure for demonstration
        // In production, this would be replaced with hardware key provider integration
        let placeholder_key = [
            0x30, 0x2e, 0x02, 0x01, 0x00, 0x30, 0x05, 0x06, 0x03, 0x2b, 0x65, 0x70, 0x04, 0x22,
            0x04, 0x20, 0x9d, 0x61, 0xb1, 0x9d, 0xef, 0xfd, 0x5a, 0x60, 0xba, 0x84, 0x4a, 0xf4,
            0x92, 0xec, 0x2c, 0xc4, 0x44, 0x49, 0xc5, 0x69, 0x7b, 0x32, 0x69, 0x19, 0x70, 0x3b,
            0xac, 0x03, 0x1c, 0xae, 0x7f, 0x60,
        ];

        Ok(rustls::pki_types::PrivateKeyDer::Pkcs8(
            rustls::pki_types::PrivatePkcs8KeyDer::from(placeholder_key.to_vec()),
        ))
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

        // TODO: Add actual hardware attestation validation here
        // This would involve:
        // 1. Parsing the certificate extensions for hardware attestation data
        // 2. Validating the attestation signature
        // 3. Checking hardware-specific properties

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
        // For hardware-backed certificates, delegate to standard signature verification
        // TODO: Add hardware-specific signature validation

        // Use the default provider's signature verification
        let provider = rustls::crypto::aws_lc_rs::default_provider();
        let _verifier = provider
            .signature_verification_algorithms
            .supported_schemes()
            .iter()
            .find(|scheme| **scheme == dss.scheme)
            .ok_or(rustls::Error::UnsupportedNameType)?;

        // For now, we'll use assertion for hardware certificates
        // In production, this should validate the hardware signature
        Ok(rustls::client::danger::HandshakeSignatureValid::assertion())
    }

    fn verify_tls13_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer<'_>,
        dss: &rustls::DigitallySignedStruct,
    ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
        // For hardware-backed certificates, delegate to standard signature verification
        // TODO: Add hardware-specific signature validation

        // Use the default provider's signature verification
        let provider = rustls::crypto::aws_lc_rs::default_provider();
        let _verifier = provider
            .signature_verification_algorithms
            .supported_schemes()
            .iter()
            .find(|scheme| **scheme == dss.scheme)
            .ok_or(rustls::Error::UnsupportedNameType)?;

        // For now, we'll use assertion for hardware certificates
        // In production, this should validate the hardware signature
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
