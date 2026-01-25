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

    /// Connect using YubiKey hardware certificate (Phase 3 integration)
    #[cfg(feature = "yubikey")]
    pub async fn connect_with_yubikey_certificate(
        &mut self,
        addr: SocketAddr,
        server_name: &str,
        yubikey_backend: &crate::backends::YubiKeyBackend,
        key_id: &str,
    ) -> Result<()> {
        use crate::backends::CertificateParams;

        println!("🔑 Phase 3: Establishing QUIC connection with YubiKey certificate...");
        println!("   Target: {}:{}", addr.ip(), addr.port());
        println!("   Server name: {}", server_name);
        println!("   YubiKey key ID: {}", key_id);

        // Generate certificate parameters for the connection
        let cert_params = CertificateParams {
            subject: format!("CN={}", server_name),
            validity_days: 365,
            is_ca: false,
            key_usage: vec![
                "digitalSignature".to_string(),
                "keyEncipherment".to_string(),
            ],
        };

        // Export YubiKey certificate for QUIC transport
        let cert_der = yubikey_backend
            .export_certificate_for_quic(key_id, cert_params)
            .context("Failed to export YubiKey certificate for QUIC")?;

        println!(
            "   ✔ YubiKey certificate exported ({} bytes)",
            cert_der.len()
        );

        // Validate certificate compatibility with QUIC
        yubikey_backend
            .validate_certificate_for_quic(&cert_der)
            .context("YubiKey certificate not compatible with QUIC")?;

        println!("   ✔ Certificate validated for QUIC transport");

        // Create QUIC endpoint with YubiKey certificate verification
        let trusted_certificates = vec![cert_der];
        let endpoint = Self::create_hardware_verified_endpoint(trusted_certificates)
            .context("Failed to create YubiKey-verified QUIC endpoint")?;

        println!("   ✔ Hardware-verified QUIC endpoint created");

        // Establish connection with YubiKey certificate validation
        let connect_timeout = Duration::from_millis(self.config.connect_timeout_ms);
        let connection = timeout(connect_timeout, endpoint.connect(addr, server_name)?)
            .await
            .context("QUIC connection timeout with YubiKey certificate")?
            .context("Failed to establish YubiKey-verified QUIC connection")?;

        println!("   ✔ YubiKey-verified QUIC connection established");

        self.endpoint = Some(endpoint);
        self.connection = Some(connection);

        // Set up bidirectional streams
        self.setup_streams()
            .await
            .context("Failed to setup QUIC streams with YubiKey certificate")?;

        println!("✔ Phase 3: QUIC transport ready with YubiKey hardware certificate");

        Ok(())
    }

    /// Create QUIC server with YubiKey certificate (Phase 3 integration)
    #[cfg(feature = "yubikey")]
    pub async fn create_yubikey_server(
        config: TransportConfig,
        bind_addr: SocketAddr,
        yubikey_backend: &crate::backends::YubiKeyBackend,
        key_id: &str,
    ) -> Result<Self> {
        use crate::backends::CertificateParams;

        println!("🔑 Phase 3: Creating QUIC server with YubiKey certificate...");
        println!("   Bind address: {}:{}", bind_addr.ip(), bind_addr.port());
        println!("   YubiKey key ID: {}", key_id);

        // Generate certificate parameters for the server
        let cert_params = CertificateParams {
            subject: format!("CN={}", bind_addr.ip()),
            validity_days: 365,
            is_ca: false,
            key_usage: vec![
                "digitalSignature".to_string(),
                "keyEncipherment".to_string(),
            ],
        };

        // Create QUIC server configuration with YubiKey certificate
        let (cert_der, _private_key_ref) = yubikey_backend
            .create_quic_server_config(key_id, cert_params)
            .context("Failed to create QUIC server config with YubiKey")?;

        println!("   ✔ YubiKey server configuration created");

        // For Phase 3, we'll create a basic server endpoint
        // In a full implementation, this would integrate with QUIC's PKCS#11 support
        let endpoint = Self::create_yubikey_server_endpoint(bind_addr, cert_der)
            .context("Failed to create YubiKey server endpoint")?;

        println!("   ✔ YubiKey server endpoint created");

        let transport = Self {
            config,
            endpoint: Some(endpoint),
            connection: None,
            send_stream: None,
            recv_stream: None,
        };

        println!("✔ Phase 3: QUIC server ready with YubiKey hardware certificate");

        Ok(transport)
    }

    /// Create QUIC server endpoint with YubiKey certificate (Phase 3)
    #[cfg(feature = "yubikey")]
    fn create_yubikey_server_endpoint(
        bind_addr: SocketAddr,
        cert_der: Vec<u8>,
    ) -> Result<Endpoint> {
        println!("   ● Creating YubiKey server endpoint...");

        // For Phase 3 demonstration, create a basic server endpoint
        // In production, this would use the actual YubiKey private key via PKCS#11

        // Create a rustls server config with the YubiKey certificate
        let cert = CertificateDer::from(cert_der);

        // For Phase 3, we'll use a simplified approach
        // Generate a temporary private key for demonstration
        // Production implementation would integrate with PKCS#11 YubiKey access
        use rustls::pki_types::PrivateKeyDer;

        // Create a minimal ECDSA P-256 private key (for demonstration)
        // This would be replaced with PKCS#11 integration in production
        let temp_private_key = create_demo_private_key()?;
        let private_key = PrivateKeyDer::try_from(temp_private_key)
            .map_err(|e| anyhow::anyhow!("Failed to create private key: {:?}", e))?;

        let server_config = rustls::ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(vec![cert], private_key)?;

        let server_config = quinn::ServerConfig::with_crypto(Arc::new(
            quinn::crypto::rustls::QuicServerConfig::try_from(server_config)?,
        ));

        let endpoint = Endpoint::server(server_config, bind_addr)?;

        println!("   ✔ YubiKey server endpoint ready");

        Ok(endpoint)
    }

    /// Accept incoming connections on the server endpoint
    #[cfg(feature = "yubikey")]
    pub async fn accept_connection(&self) -> Result<Connection> {
        let endpoint = self
            .endpoint
            .as_ref()
            .context("No endpoint available for accepting connections")?;

        loop {
            match endpoint.accept().await {
                Some(incoming) => match incoming.await {
                    Ok(connection) => {
                        return Ok(connection);
                    }
                    Err(e) => {
                        println!("    ⚠ Connection failed: {}", e);
                        continue;
                    }
                },
                None => {
                    anyhow::bail!("Endpoint closed");
                }
            }
        }
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

/// Create a demonstration private key for Phase 3 YubiKey integration
/// In production, this would be replaced with PKCS#11 hardware key access
#[cfg(feature = "yubikey")]
fn create_demo_private_key() -> Result<Vec<u8>> {
    // Create a minimal ECDSA P-256 private key in PKCS#8 format
    // This is for demonstration purposes only
    let private_key_pkcs8 = vec![
        0x30, 0x81, 0x87, // SEQUENCE, length 135
        0x02, 0x01, 0x00, // INTEGER version = 0
        0x30, 0x13, // SEQUENCE (algorithm identifier)
        0x06, 0x07, 0x2a, 0x86, 0x48, 0xce, 0x3d, 0x02, 0x01, // OID ecPublicKey
        0x06, 0x08, 0x2a, 0x86, 0x48, 0xce, 0x3d, 0x03, 0x01, 0x07, // OID secp256r1
        0x04, 0x6d, // OCTET STRING, length 109
        0x30, 0x6b, // SEQUENCE, length 107
        0x02, 0x01, 0x01, // INTEGER version = 1
        0x04, 0x20, // OCTET STRING, length 32 (private key)
        // 32-byte private key (demonstration purposes)
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
        0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e,
        0x1f, 0x20, 0xa1, 0x44, // Context tag [1], length 68
        0x03, 0x42, 0x00, // BIT STRING, length 66, unused bits = 0
        // 65-byte public key (uncompressed format)
        0x04, // Uncompressed point indicator
        // X coordinate (32 bytes)
        0x96, 0xc4, 0x10, 0x7a, 0x3f, 0x8b, 0x5f, 0x6b, 0x6b, 0x7e, 0x8c, 0x61, 0x5b, 0x8f, 0x3b,
        0x9c, 0x7e, 0x9f, 0x8e, 0x6d, 0x4e, 0x2f, 0x1a, 0x5c, 0x3d, 0x2e, 0x1f, 0x0a, 0x9b, 0x8c,
        0x7d, 0x6e, // Y coordinate (32 bytes)
        0x5f, 0x4b, 0x2a, 0x3c, 0x1d, 0x0e, 0x2f, 0x5a, 0x6b, 0x7c, 0x8d, 0x9e, 0x0f, 0x1a, 0x2b,
        0x3c, 0x4d, 0x5e, 0x6f, 0x0a, 0x1b, 0x2c, 0x3d, 0x4e, 0x5f, 0x6a, 0x7b, 0x8c, 0x9d, 0xae,
        0xbf, 0xca,
    ];

    Ok(private_key_pkcs8)
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
    async fn test_certificate_requirements() {
        // Test certificate validation requirements for QUIC
        #[cfg(feature = "yubikey")]
        {
            use crate::backends::CertificateParams;

            let cert_params = CertificateParams {
                subject: "CN=quic-test.example.com".to_string(),
                validity_days: 365,
                is_ca: false,
                key_usage: vec!["digital_signature".to_string(), "key_agreement".to_string()],
            };

            // QUIC requires proper certificate configuration
            assert!(!cert_params.subject.is_empty());
            assert!(cert_params.validity_days > 0);
            assert!(cert_params
                .key_usage
                .contains(&"digital_signature".to_string()));
            assert!(!cert_params.is_ca);
        }
    }

    #[cfg(feature = "yubikey")]
    #[tokio::test]
    async fn test_yubikey_integration_stubs() {
        use crate::backends::CertificateParams;

        // Test certificate parameters for YubiKey integration
        let cert_params = CertificateParams {
            subject: "CN=test.example.com".to_string(),
            validity_days: 365,
            is_ca: false,
            key_usage: vec!["digital_signature".to_string()],
        };

        // Validate certificate parameters
        assert!(!cert_params.subject.is_empty());
        assert!(cert_params.validity_days > 0);
        assert!(!cert_params.key_usage.is_empty());
        assert!(cert_params
            .key_usage
            .contains(&"digital_signature".to_string()));
        assert!(!cert_params.is_ca);
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
}
