//
// Copyright (c) 2025 John Turner
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/
//
// Project: trustedge — Privacy and trust at the edge.
//

//! Authentication and session management for TrustEdge network operations

use anyhow::{anyhow, Context, Result};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand_core::{OsRng, RngCore};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

/// Session timeout duration (30 minutes)
pub const SESSION_TIMEOUT: Duration = Duration::from_secs(1800);

/// Challenge size for authentication (32 bytes)
pub const CHALLENGE_SIZE: usize = 32;

/// Session ID size (16 bytes)
pub const SESSION_ID_SIZE: usize = 16;

/// Client certificate containing identity and signing key
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClientCertificate {
    /// Client identity name
    pub identity: String,
    /// Client's public key (32 bytes)
    pub public_key: [u8; 32],
    /// Private signing key (for client use only, not serialized)
    #[serde(skip)]
    pub signing_key: Option<SigningKey>,
    /// Creation timestamp
    pub created_at: SystemTime,
}

impl ClientCertificate {
    /// Generate a new client certificate with identity
    pub fn generate(identity: &str) -> Result<Self> {
        let signing_key = SigningKey::generate(&mut OsRng);
        let public_key = signing_key.verifying_key().to_bytes();

        Ok(Self {
            identity: identity.to_string(),
            public_key,
            signing_key: Some(signing_key),
            created_at: SystemTime::now(),
        })
    }

    /// Get the signing key (required for authentication)
    pub fn signing_key(&self) -> Result<&SigningKey> {
        self.signing_key
            .as_ref()
            .ok_or_else(|| anyhow!("Signing key not available in certificate"))
    }
}

/// Load server certificate from file
pub fn load_server_cert(path: &str) -> Result<ServerCertificate> {
    let cert_data = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read server certificate from {}", path))?;

    let cert: ServerCertificate =
        serde_json::from_str(&cert_data).context("Failed to parse server certificate JSON")?;

    cert.verify()?;
    Ok(cert)
}

/// Save client certificate to file
pub fn save_client_cert(cert: &ClientCertificate, path: &str) -> Result<()> {
    let cert_json =
        serde_json::to_string_pretty(cert).context("Failed to serialize client certificate")?;

    std::fs::write(path, cert_json)
        .with_context(|| format!("Failed to write client certificate to {}", path))?;

    Ok(())
}

/// Save server certificate to file
pub fn save_server_cert(cert: &ServerCertificate, path: &str) -> Result<()> {
    let cert_json =
        serde_json::to_string_pretty(cert).context("Failed to serialize server certificate")?;

    std::fs::write(path, cert_json)
        .with_context(|| format!("Failed to write server certificate to {}", path))?;

    Ok(())
}

/// Authentication message types
#[derive(Debug, Serialize, Deserialize, Clone)]
#[repr(u8)]
pub enum AuthMessageType {
    /// Client initiates authentication
    ClientHello = 1,
    /// Server responds with challenge
    ServerChallenge = 2,
    /// Client responds to challenge
    ClientAuth = 3,
    /// Server confirms authentication
    ServerConfirm = 4,
    /// Authentication failed
    AuthError = 5,
}

/// Server certificate containing identity and public key
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerCertificate {
    /// Server identity/name
    pub identity: String,
    /// Ed25519 public key for verification
    pub public_key: [u8; 32],
    /// Certificate validity period start
    pub valid_from: u64,
    /// Certificate validity period end  
    pub valid_until: u64,
    /// Self-signed signature of the certificate
    #[serde(with = "serde_bytes")]
    pub signature: [u8; 64],
}

impl ServerCertificate {
    /// Create a new self-signed server certificate
    pub fn new_self_signed(
        identity: String,
        signing_key: &SigningKey,
        validity_days: u64,
    ) -> Result<Self> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let valid_until = now + (validity_days * 24 * 3600);

        let public_key = signing_key.verifying_key().to_bytes();

        // Create certificate data for signing
        let cert_data = format!(
            "{}:{}:{}:{}",
            identity,
            hex::encode(public_key),
            now,
            valid_until
        );

        let signature = signing_key.sign(cert_data.as_bytes()).to_bytes();

        Ok(Self {
            identity,
            public_key,
            valid_from: now,
            valid_until,
            signature,
        })
    }

    /// Verify the certificate signature and validity
    pub fn verify(&self) -> Result<()> {
        // Check validity period
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        if now < self.valid_from || now > self.valid_until {
            return Err(anyhow!("Certificate expired or not yet valid"));
        }

        // Verify self-signature
        let verifying_key = VerifyingKey::from_bytes(&self.public_key)
            .map_err(|e| anyhow!("Invalid public key: {}", e))?;

        let cert_data = format!(
            "{}:{}:{}:{}",
            self.identity,
            hex::encode(self.public_key),
            self.valid_from,
            self.valid_until
        );

        let signature = Signature::from_bytes(&self.signature);
        verifying_key
            .verify(cert_data.as_bytes(), &signature)
            .map_err(|e| anyhow!("Certificate signature verification failed: {}", e))?;

        Ok(())
    }
}

/// Authentication challenge from server to client
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthChallenge {
    /// Random challenge bytes
    pub challenge: [u8; CHALLENGE_SIZE],
    /// Server certificate
    pub server_cert: ServerCertificate,
    /// Timestamp
    pub timestamp: u64,
}

/// Client authentication response
#[derive(Debug, Serialize, Deserialize)]
pub struct ClientAuthResponse {
    /// Client's public key
    pub client_public_key: [u8; 32],
    /// Signature of the challenge using client's private key
    #[serde(with = "serde_bytes")]
    pub challenge_signature: [u8; 64],
    /// Client identity (optional)
    pub client_identity: Option<String>,
    /// Timestamp
    pub timestamp: u64,
}

/// Server authentication confirmation
#[derive(Debug, Serialize, Deserialize)]
pub struct ServerAuthConfirm {
    /// Session ID for this authenticated session
    pub session_id: [u8; SESSION_ID_SIZE],
    /// Session timeout (seconds from now)
    pub session_timeout: u64,
    /// Server signature of session details
    #[serde(with = "serde_bytes")]
    pub session_signature: [u8; 64],
}

/// Authentication message wrapper
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthMessage {
    /// Message type
    pub msg_type: AuthMessageType,
    /// Serialized message payload
    pub payload: Vec<u8>,
}

impl AuthMessage {
    /// Create a new authentication message
    pub fn new<T: Serialize>(msg_type: AuthMessageType, payload: &T) -> Result<Self> {
        let payload_bytes =
            bincode::serialize(payload).context("Failed to serialize auth message payload")?;
        Ok(Self {
            msg_type,
            payload: payload_bytes,
        })
    }

    /// Deserialize the payload as the specified type
    pub fn deserialize_payload<T: for<'de> Deserialize<'de>>(&self) -> Result<T> {
        bincode::deserialize(&self.payload).context("Failed to deserialize auth message payload")
    }
}

/// Active session information
#[derive(Debug, Clone)]
pub struct SessionInfo {
    /// Unique session identifier
    pub session_id: [u8; SESSION_ID_SIZE],
    /// Client's public key
    pub client_public_key: [u8; 32],
    /// Client identity (if provided)
    pub client_identity: Option<String>,
    /// Session creation timestamp
    pub created_at: u64,
    /// Session expiration timestamp
    pub expires_at: u64,
    /// Whether the session is authenticated
    pub authenticated: bool,
}

impl SessionInfo {
    /// Check if the session is still valid
    pub fn is_valid(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        self.authenticated && now < self.expires_at
    }

    /// Check if the session is expired
    pub fn is_expired(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        now >= self.expires_at
    }
}

/// Session manager for tracking active sessions
#[derive(Debug, Clone)]
pub struct SessionManager {
    /// Active sessions mapped by session ID
    sessions: HashMap<[u8; SESSION_ID_SIZE], SessionInfo>,
    /// Server signing key for authentication
    server_signing_key: SigningKey,
    /// Server certificate
    server_certificate: ServerCertificate,
}

impl SessionManager {
    /// Create a new session manager with server identity
    pub fn new(server_identity: String) -> Result<Self> {
        let server_signing_key = SigningKey::generate(&mut OsRng);
        let server_certificate = ServerCertificate::new_self_signed(
            server_identity,
            &server_signing_key,
            365, // Valid for 1 year
        )?;

        Ok(Self {
            sessions: HashMap::new(),
            server_signing_key,
            server_certificate,
        })
    }

    /// Create a new session manager with existing signing key
    pub fn with_signing_key(server_identity: String, signing_key: SigningKey) -> Result<Self> {
        let server_certificate =
            ServerCertificate::new_self_signed(server_identity, &signing_key, 365)?;

        Ok(Self {
            sessions: HashMap::new(),
            server_signing_key: signing_key,
            server_certificate,
        })
    }

    /// Get server certificate
    pub fn server_certificate(&self) -> &ServerCertificate {
        &self.server_certificate
    }

    /// Create a new authentication challenge
    pub fn create_challenge(&self) -> Result<AuthChallenge> {
        let mut challenge = [0u8; CHALLENGE_SIZE];
        OsRng.fill_bytes(&mut challenge);

        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

        Ok(AuthChallenge {
            challenge,
            server_cert: self.server_certificate.clone(),
            timestamp,
        })
    }

    /// Verify client authentication response and create session
    pub fn authenticate_client(
        &mut self,
        challenge: &AuthChallenge,
        response: &ClientAuthResponse,
    ) -> Result<SessionInfo> {
        // Verify timestamp (allow 5 minute window)
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        if response.timestamp.abs_diff(now) > 300 {
            return Err(anyhow!("Authentication response timestamp out of range"));
        }

        // Verify client signature of challenge
        let client_verifying_key = VerifyingKey::from_bytes(&response.client_public_key)
            .map_err(|e| anyhow!("Invalid client public key: {}", e))?;

        let signature = Signature::from_bytes(&response.challenge_signature);
        client_verifying_key
            .verify(&challenge.challenge, &signature)
            .map_err(|e| anyhow!("Client challenge signature verification failed: {}", e))?;

        // Generate session ID
        let mut session_id = [0u8; SESSION_ID_SIZE];
        OsRng.fill_bytes(&mut session_id);

        // Create session info
        let expires_at = now + SESSION_TIMEOUT.as_secs();
        let session = SessionInfo {
            session_id,
            client_public_key: response.client_public_key,
            client_identity: response.client_identity.clone(),
            created_at: now,
            expires_at,
            authenticated: true,
        };

        // Store session
        self.sessions.insert(session_id, session.clone());

        Ok(session)
    }

    /// Create server authentication confirmation
    pub fn create_auth_confirm(&self, session: &SessionInfo) -> Result<ServerAuthConfirm> {
        let timeout_secs = session.expires_at - session.created_at;

        // Sign session details
        let session_data = format!(
            "{}:{}:{}",
            hex::encode(session.session_id),
            hex::encode(session.client_public_key),
            session.expires_at
        );

        let session_signature = self
            .server_signing_key
            .sign(session_data.as_bytes())
            .to_bytes();

        Ok(ServerAuthConfirm {
            session_id: session.session_id,
            session_timeout: timeout_secs,
            session_signature,
        })
    }

    /// Validate an existing session
    pub fn validate_session(&mut self, session_id: &[u8; SESSION_ID_SIZE]) -> Result<&SessionInfo> {
        // Clean up expired sessions
        self.cleanup_expired_sessions();

        let session = self
            .sessions
            .get(session_id)
            .ok_or_else(|| anyhow!("Session not found"))?;

        if !session.is_valid() {
            return Err(anyhow!("Session expired or invalid"));
        }

        Ok(session)
    }

    /// Remove a session
    pub fn remove_session(&mut self, session_id: &[u8; SESSION_ID_SIZE]) {
        self.sessions.remove(session_id);
    }

    /// Clean up expired sessions
    pub fn cleanup_expired_sessions(&mut self) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        self.sessions.retain(|_, session| now < session.expires_at);
    }

    /// Get count of active sessions
    pub fn active_session_count(&self) -> usize {
        self.sessions.len()
    }
}

/// Perform server-side authentication handshake
pub async fn server_authenticate(
    stream: &mut TcpStream,
    session_manager: &mut SessionManager,
) -> Result<SessionInfo> {
    // Read client hello
    let mut msg_len_buf = [0u8; 4];
    stream
        .read_exact(&mut msg_len_buf)
        .await
        .context("Failed to read client hello length")?;
    let msg_len = u32::from_le_bytes(msg_len_buf) as usize;

    if msg_len > 8192 {
        return Err(anyhow!("Client hello message too large"));
    }

    let mut msg_buf = vec![0u8; msg_len];
    stream
        .read_exact(&mut msg_buf)
        .await
        .context("Failed to read client hello")?;

    let client_hello: AuthMessage =
        bincode::deserialize(&msg_buf).context("Failed to deserialize client hello")?;

    if !matches!(client_hello.msg_type, AuthMessageType::ClientHello) {
        return Err(anyhow!("Expected ClientHello message"));
    }

    // Create and send challenge
    let challenge = session_manager.create_challenge()?;
    let challenge_msg = AuthMessage::new(AuthMessageType::ServerChallenge, &challenge)?;
    let challenge_bytes = bincode::serialize(&challenge_msg)?;

    stream.write_u32_le(challenge_bytes.len() as u32).await?;
    stream.write_all(&challenge_bytes).await?;
    stream.flush().await?;

    // Read client auth response
    stream
        .read_exact(&mut msg_len_buf)
        .await
        .context("Failed to read client auth length")?;
    let msg_len = u32::from_le_bytes(msg_len_buf) as usize;

    if msg_len > 8192 {
        return Err(anyhow!("Client auth message too large"));
    }

    msg_buf.resize(msg_len, 0);
    stream
        .read_exact(&mut msg_buf)
        .await
        .context("Failed to read client auth")?;

    let client_auth: AuthMessage =
        bincode::deserialize(&msg_buf).context("Failed to deserialize client auth")?;

    if !matches!(client_auth.msg_type, AuthMessageType::ClientAuth) {
        return Err(anyhow!("Expected ClientAuth message"));
    }

    let auth_response: ClientAuthResponse = client_auth.deserialize_payload()?;

    // Authenticate client
    match session_manager.authenticate_client(&challenge, &auth_response) {
        Ok(session) => {
            // Send confirmation
            let confirm = session_manager.create_auth_confirm(&session)?;
            let confirm_msg = AuthMessage::new(AuthMessageType::ServerConfirm, &confirm)?;
            let confirm_bytes = bincode::serialize(&confirm_msg)?;

            stream.write_u32_le(confirm_bytes.len() as u32).await?;
            stream.write_all(&confirm_bytes).await?;
            stream.flush().await?;

            Ok(session)
        }
        Err(e) => {
            // Send error
            let error_msg = AuthMessage::new(AuthMessageType::AuthError, &e.to_string())?;
            let error_bytes = bincode::serialize(&error_msg)?;

            stream.write_u32_le(error_bytes.len() as u32).await?;
            stream.write_all(&error_bytes).await?;
            stream.flush().await?;

            Err(e)
        }
    }
}

/// Perform client-side authentication handshake
pub async fn client_authenticate(
    stream: &mut TcpStream,
    client_signing_key: &SigningKey,
    client_identity: Option<String>,
    expected_server_identity: Option<&str>,
) -> Result<([u8; SESSION_ID_SIZE], ServerCertificate)> {
    // Send client hello
    let hello_msg = AuthMessage::new(AuthMessageType::ClientHello, &"TrustEdge Client v1.0")?;
    let hello_bytes = bincode::serialize(&hello_msg)?;

    stream.write_u32_le(hello_bytes.len() as u32).await?;
    stream.write_all(&hello_bytes).await?;
    stream.flush().await?;

    // Read server challenge
    let mut msg_len_buf = [0u8; 4];
    stream
        .read_exact(&mut msg_len_buf)
        .await
        .context("Failed to read server challenge length")?;
    let msg_len = u32::from_le_bytes(msg_len_buf) as usize;

    if msg_len > 8192 {
        return Err(anyhow!("Server challenge message too large"));
    }

    let mut msg_buf = vec![0u8; msg_len];
    stream
        .read_exact(&mut msg_buf)
        .await
        .context("Failed to read server challenge")?;

    let challenge_msg: AuthMessage =
        bincode::deserialize(&msg_buf).context("Failed to deserialize server challenge")?;

    if !matches!(challenge_msg.msg_type, AuthMessageType::ServerChallenge) {
        return Err(anyhow!("Expected ServerChallenge message"));
    }

    let challenge: AuthChallenge = challenge_msg.deserialize_payload()?;

    // Verify server certificate
    challenge
        .server_cert
        .verify()
        .context("Server certificate verification failed")?;

    // Check server identity if expected
    if let Some(expected) = expected_server_identity {
        if challenge.server_cert.identity != expected {
            return Err(anyhow!(
                "Server identity mismatch: expected '{}', got '{}'",
                expected,
                challenge.server_cert.identity
            ));
        }
    }

    // Sign challenge
    let challenge_signature = client_signing_key.sign(&challenge.challenge).to_bytes();
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

    let auth_response = ClientAuthResponse {
        client_public_key: client_signing_key.verifying_key().to_bytes(),
        challenge_signature,
        client_identity,
        timestamp,
    };

    // Send auth response
    let auth_msg = AuthMessage::new(AuthMessageType::ClientAuth, &auth_response)?;
    let auth_bytes = bincode::serialize(&auth_msg)?;

    stream.write_u32_le(auth_bytes.len() as u32).await?;
    stream.write_all(&auth_bytes).await?;
    stream.flush().await?;

    // Read server confirmation or error
    stream
        .read_exact(&mut msg_len_buf)
        .await
        .context("Failed to read server response length")?;
    let msg_len = u32::from_le_bytes(msg_len_buf) as usize;

    if msg_len > 8192 {
        return Err(anyhow!("Server response message too large"));
    }

    msg_buf.resize(msg_len, 0);
    stream
        .read_exact(&mut msg_buf)
        .await
        .context("Failed to read server response")?;

    let response_msg: AuthMessage =
        bincode::deserialize(&msg_buf).context("Failed to deserialize server response")?;

    match response_msg.msg_type {
        AuthMessageType::ServerConfirm => {
            let confirm: ServerAuthConfirm = response_msg.deserialize_payload()?;

            // Verify session signature
            let session_data = format!(
                "{}:{}:{}",
                hex::encode(confirm.session_id),
                hex::encode(client_signing_key.verifying_key().to_bytes()),
                timestamp + confirm.session_timeout
            );

            let server_verifying_key = VerifyingKey::from_bytes(&challenge.server_cert.public_key)?;
            let signature = Signature::from_bytes(&confirm.session_signature);
            server_verifying_key
                .verify(session_data.as_bytes(), &signature)
                .context("Server session signature verification failed")?;

            Ok((confirm.session_id, challenge.server_cert))
        }
        AuthMessageType::AuthError => {
            let error_msg: String = response_msg.deserialize_payload()?;
            Err(anyhow!("Authentication failed: {}", error_msg))
        }
        _ => Err(anyhow!("Unexpected server response type")),
    }
}
