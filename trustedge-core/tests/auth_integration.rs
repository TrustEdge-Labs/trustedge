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

    // Set up a local server for testing
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let server_addr = listener.local_addr()?;

    // Spawn server task
    let _server_cert = server_manager.server_certificate().clone();
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

    // Perform client authentication
    let (session_id, _returned_server_cert) = client_authenticate(
        &mut client_stream,
        client_cert.signing_key()?,
        Some(client_cert.identity.clone()),
        Some("test-server"),
    )
    .await?;

    // Wait for server to complete
    let server_result = server_handle.await?;
    let auth_session = server_result.map_err(|e| anyhow::anyhow!(e))?;

    // Verify session IDs match
    assert_eq!(session_id, auth_session.session_id);
    assert_eq!(
        auth_session.client_identity,
        Some(client_cert.identity.clone())
    );

    println!("✔ Mutual authentication test passed!");
    println!("   Session ID: {}", hex::encode(session_id));
    println!("   Client: {}", client_cert.identity);

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
    let (session_id, _) = client_authenticate(
        &mut client_stream,
        client_cert.signing_key()?,
        Some(client_cert.identity.clone()),
        None,
    )
    .await?;

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
