// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//

//! Integration tests for authentication system

use anyhow::Result;
use std::time::Duration;
use tokio::net::{TcpListener, TcpStream};
use tokio::time::timeout;

use trustedge_core::auth::{
    client_authenticate, server_authenticate, ClientCertificate, SessionManager,
};

#[tokio::test]
async fn test_mutual_authentication() -> Result<()> {
    // Create server session manager
    let mut server_manager = SessionManager::new("test-server".to_string())?;

    // Create client certificate
    let client_cert = ClientCertificate::generate("test-client")?;

    // Pin the server's public key (client must know this beforehand)
    let server_pubkey = server_manager.server_certificate().public_key;

    // Set up a local server for testing
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let server_addr = listener.local_addr()?;

    // Spawn server task
    let server_handle = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        server_authenticate(&mut stream, &mut server_manager)
            .await
            .map_err(|e| format!("Server auth failed: {}", e))
    });

    // Give server time to start
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Connect as client
    let mut client_stream =
        timeout(Duration::from_secs(5), TcpStream::connect(server_addr)).await??;

    // Perform client authentication with pinned server public key
    let client_result = client_authenticate(
        &mut client_stream,
        client_cert.signing_key()?,
        Some(client_cert.identity.clone()),
        &server_pubkey,
    )
    .await?;

    // Wait for server to complete
    let server_result = server_handle.await?;
    let auth_session = server_result.map_err(|e| anyhow::anyhow!(e))?;

    // Verify session IDs match
    assert_eq!(client_result.session_id, auth_session.session_id);
    assert_eq!(
        auth_session.client_identity,
        Some(client_cert.identity.clone())
    );

    // Verify both sides derived the same session key via ECDH
    assert_eq!(
        client_result.session_key, auth_session.session_key,
        "Client and server must derive identical session keys via ECDH"
    );

    // Verify session key is not all zeros
    assert!(!client_result.session_key.iter().all(|&b| b == 0));

    println!("✔ Mutual authentication test passed!");
    println!("   Session ID: {}", hex::encode(client_result.session_id));
    println!("   Client: {}", client_cert.identity);
    println!("   Session key derived: yes (ECDH)");

    Ok(())
}

#[tokio::test]
async fn test_certificate_generation_and_verification() -> Result<()> {
    // Test client certificate generation
    let client_cert = ClientCertificate::generate("test-client-identity")?;

    assert_eq!(client_cert.identity, "test-client-identity");
    assert_eq!(client_cert.public_key.len(), 32);
    assert!(client_cert.signing_key.is_some());

    // Test server certificate generation
    let server_manager = SessionManager::new("test-server-identity".to_string())?;
    let server_cert = server_manager.server_certificate();

    assert_eq!(server_cert.identity, "test-server-identity");
    assert_eq!(server_cert.public_key.len(), 32);

    // Verify server certificate is self-signed correctly
    server_cert.verify()?;

    println!("✔ Certificate generation test passed!");
    println!("   Client identity: {}", client_cert.identity);
    println!("   Server identity: {}", server_cert.identity);

    Ok(())
}

#[tokio::test]
async fn test_session_management() -> Result<()> {
    let session_manager = SessionManager::new("test-server".to_string())?;

    // Initially no sessions
    assert_eq!(session_manager.active_session_count(), 0);

    // Create a mock session by performing authentication
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let server_addr = listener.local_addr()?;

    let client_cert = ClientCertificate::generate("session-test-client")?;

    // Pin the server's public key
    let server_pubkey = session_manager.server_certificate().public_key;

    // Server task - capture manager after authentication
    let mut server_manager_copy = session_manager.clone();
    let server_handle = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        let result = server_authenticate(&mut stream, &mut server_manager_copy).await;
        (result, server_manager_copy)
    });

    tokio::time::sleep(Duration::from_millis(200)).await;

    // Client connection with timeout
    let mut client_stream =
        timeout(Duration::from_secs(5), TcpStream::connect(server_addr)).await??;
    let client_result = client_authenticate(
        &mut client_stream,
        client_cert.signing_key()?,
        Some(client_cert.identity.clone()),
        &server_pubkey,
    )
    .await?;
    let session_id = client_result.session_id;

    // Complete server authentication and get updated manager
    let (auth_result, mut updated_manager) = server_handle.await?;
    let _auth_session = auth_result?;

    // Should have one active session
    assert_eq!(updated_manager.active_session_count(), 1);

    // Validate the session exists
    let session_info = updated_manager.validate_session(&session_id)?;
    assert_eq!(session_info.client_identity, Some(client_cert.identity));

    // Remove the session
    updated_manager.remove_session(&session_id);
    assert_eq!(updated_manager.active_session_count(), 0);

    println!("✔ Session management test passed!");
    println!("   Session ID: {}", hex::encode(session_id));

    Ok(())
}

#[tokio::test]
async fn test_session_key_uniqueness() -> Result<()> {
    // Run two separate auth handshakes with different clients and verify
    // that the resulting session keys differ.

    // First handshake
    let mut server_manager1 = SessionManager::new("test-server-unique".to_string())?;
    let server_pubkey1 = server_manager1.server_certificate().public_key;
    let client_cert1 = ClientCertificate::generate("client-1")?;

    let listener1 = TcpListener::bind("127.0.0.1:0").await?;
    let addr1 = listener1.local_addr()?;

    let server1 = tokio::spawn(async move {
        let (mut stream, _) = listener1.accept().await.unwrap();
        server_authenticate(&mut stream, &mut server_manager1)
            .await
            .map_err(|e| format!("Server1 auth failed: {}", e))
    });

    tokio::time::sleep(Duration::from_millis(100)).await;

    let mut stream1 = timeout(Duration::from_secs(5), TcpStream::connect(addr1)).await??;
    let result1 = client_authenticate(
        &mut stream1,
        client_cert1.signing_key()?,
        Some("client-1".to_string()),
        &server_pubkey1,
    )
    .await?;

    let server_session1 = server1.await?.map_err(|e| anyhow::anyhow!(e))?;

    // Second handshake with a different client
    let mut server_manager2 = SessionManager::new("test-server-unique".to_string())?;
    let server_pubkey2 = server_manager2.server_certificate().public_key;
    let client_cert2 = ClientCertificate::generate("client-2")?;

    let listener2 = TcpListener::bind("127.0.0.1:0").await?;
    let addr2 = listener2.local_addr()?;

    let server2 = tokio::spawn(async move {
        let (mut stream, _) = listener2.accept().await.unwrap();
        server_authenticate(&mut stream, &mut server_manager2)
            .await
            .map_err(|e| format!("Server2 auth failed: {}", e))
    });

    tokio::time::sleep(Duration::from_millis(100)).await;

    let mut stream2 = timeout(Duration::from_secs(5), TcpStream::connect(addr2)).await??;
    let result2 = client_authenticate(
        &mut stream2,
        client_cert2.signing_key()?,
        Some("client-2".to_string()),
        &server_pubkey2,
    )
    .await?;

    let server_session2 = server2.await?.map_err(|e| anyhow::anyhow!(e))?;

    // Both handshakes should have internally consistent keys
    assert_eq!(result1.session_key, server_session1.session_key);
    assert_eq!(result2.session_key, server_session2.session_key);

    // But the two session keys must differ (different keypairs + different challenges)
    assert_ne!(
        result1.session_key, result2.session_key,
        "Different auth handshakes must produce different session keys"
    );

    println!("✔ Session key uniqueness test passed!");

    Ok(())
}

#[tokio::test]
async fn test_mitm_rejected_wrong_pubkey() -> Result<()> {
    // Simulate MITM: client expects one server's public key but connects to
    // a different server (attacker). The pinning check must reject the handshake.

    let mut attacker_manager = SessionManager::new("legitimate-server".to_string())?;

    // Client pins the REAL server's public key (not the attacker's)
    let real_server_manager = SessionManager::new("legitimate-server".to_string())?;
    let real_server_pubkey = real_server_manager.server_certificate().public_key;

    // Attacker's key is different (different SessionManager = different key pair)
    assert_ne!(
        attacker_manager.server_certificate().public_key,
        real_server_pubkey,
        "Test setup: attacker and real server must have different keys"
    );

    let client_cert = ClientCertificate::generate("victim-client")?;

    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let addr = listener.local_addr()?;

    // Attacker's server accepts the connection
    let _attacker_handle = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        // Attacker tries to authenticate — will send their own cert
        let _ = server_authenticate(&mut stream, &mut attacker_manager).await;
    });

    tokio::time::sleep(Duration::from_millis(100)).await;

    let mut client_stream = timeout(Duration::from_secs(5), TcpStream::connect(addr)).await??;

    // Client authenticates with the REAL server's pinned key
    let result = client_authenticate(
        &mut client_stream,
        client_cert.signing_key()?,
        Some("victim-client".to_string()),
        &real_server_pubkey, // pinned to real server, not attacker
    )
    .await;

    // Must fail — attacker's cert has a different public key
    assert!(
        result.is_err(),
        "MITM must be rejected by public key pinning"
    );
    let err_msg = result.err().unwrap().to_string();
    assert!(
        err_msg.contains("pinning"),
        "Error should mention pinning failure, got: {}",
        err_msg
    );

    println!("✔ MITM rejection test passed!");

    Ok(())
}
