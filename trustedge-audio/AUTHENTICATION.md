# TrustEdge Authentication System

This document describes the mutual authentication system implemented in TrustEdge for secure network operations.

## Overview

TrustEdge implements a robust mutual authentication system using Ed25519 cryptographic signatures to establish secure sessions between clients and servers. This system ensures that:

- **Server Authentication**: Clients can verify they're connecting to the legitimate TrustEdge server
- **Client Authentication**: Servers can verify client identity and authorize access
- **Session Security**: All subsequent communications are tied to authenticated sessions
- **Certificate Management**: Public key infrastructure for identity verification

## Architecture

### Components

1. **Client Certificates**: Ed25519 key pairs with identity information
2. **Server Certificates**: Self-signed server identity certificates  
3. **Session Manager**: Server-side session tracking and validation
4. **Challenge-Response Protocol**: Cryptographic proof of key ownership

### Authentication Flow

```
Client                                    Server
  |                                         |
  |--- ClientHello ----------------------->|
  |                                         |--- Generate Challenge
  |<-- ServerChallenge + ServerCert -------|
  |                                         |
  |--- Verify ServerCert                   |
  |--- Sign Challenge                      |
  |--- ClientAuth + Signature ------------>|
  |                                         |--- Verify Client Signature
  |                                         |--- Create Session
  |<-- ServerConfirm + SessionInfo --------|
  |                                         |
  |=== Authenticated Session Active ======|
```

## Usage

### Server Setup

1. **Generate Server Certificate**:
```bash
# Server automatically generates certificate on first run
cargo run --bin trustedge-server -- \
    --require-auth \
    --server-identity "production-server" \
    --bind-addr 0.0.0.0:8080
```

2. **Configure Authentication**:
```bash
# Enable authentication with custom certificate path
cargo run --bin trustedge-server -- \
    --require-auth \
    --server-cert ./certs/server.cert \
    --bind-addr 0.0.0.0:8080
```

### Client Authentication

1. **First Time Setup**:
```bash
# Client generates certificate automatically
cargo run --bin trustedge-client -- \
    --enable-auth \
    --client-identity "client-workstation" \
    --server-cert ./certs/server.cert \
    --server 127.0.0.1:8080 \
    --file example.txt
```

2. **Using Existing Certificate**:
```bash
# Use existing client certificate
cargo run --bin trustedge-client -- \
    --enable-auth \
    --client-cert ./certs/client.cert \
    --server-cert ./certs/server.cert \
    --server 127.0.0.1:8080 \
    --file example.txt
```

## Certificate Management

### Client Certificates

Client certificates contain:
- **Identity**: Human-readable client name
- **Public Key**: Ed25519 verification key (32 bytes)
- **Private Key**: Ed25519 signing key (stored locally only)
- **Timestamp**: Certificate creation time

Example certificate structure:
```json
{
  "identity": "client-workstation",
  "public_key": [/* 32 bytes */],
  "created_at": {
    "secs_since_epoch": 1704067200,
    "nanos_since_epoch": 0
  }
}
```

### Server Certificates

Server certificates are self-signed and contain:
- **Identity**: Server name/hostname
- **Public Key**: Ed25519 verification key
- **Validity Period**: Start and end timestamps
- **Self-Signature**: Cryptographic proof of authenticity

### Security Features

1. **Challenge-Response Authentication**:
   - Server generates 32-byte random challenge
   - Client signs challenge with private key
   - Server verifies signature with client's public key

2. **Session Management**:
   - Unique 16-byte session IDs
   - Configurable session timeouts (default: 30 minutes)
   - Automatic cleanup of expired sessions

3. **Certificate Validation**:
   - Timestamp verification for server certificates
   - Cryptographic signature verification
   - Identity matching against expected values

## File and Audio Streaming with Authentication

### Authenticated File Transfer

```bash
# Server with authentication
cargo run --bin trustedge-server -- \
    --require-auth \
    --output-dir ./received \
    --verbose

# Client with file transfer
cargo run --bin trustedge-client -- \
    --enable-auth \
    --client-identity "secure-client" \
    --server-cert ./server.cert \
    --file sensitive-data.txt \
    --verbose
```

### Authenticated Audio Streaming

```bash
# Server ready for audio
cargo run --bin trustedge-server -- \
    --require-auth \
    --output-dir ./audio-captures \
    --verbose

# Client with live audio (requires audio feature)
cargo run --features audio --bin trustedge-client -- \
    --enable-auth \
    --client-identity "audio-workstation" \
    --server-cert ./server.cert \
    --audio-live \
    --sample-rate 44100 \
    --channels 2 \
    --duration 30 \
    --verbose
```

## Security Considerations

### Threat Model

The authentication system protects against:
- **Impersonation Attacks**: Cryptographic proof prevents identity spoofing
- **Man-in-the-Middle**: Certificate verification ensures endpoint authenticity  
- **Replay Attacks**: Challenge-response prevents reuse of authentication data
- **Session Hijacking**: Session IDs are cryptographically random and time-limited

### Best Practices

1. **Certificate Storage**:
   - Store server certificates in secure, read-only locations
   - Protect client private keys with appropriate file permissions
   - Consider using hardware security modules for production

2. **Network Security**:
   - Use authentication in conjunction with TLS for transport security
   - Implement rate limiting to prevent brute force attacks
   - Monitor authentication failures and suspicious patterns

3. **Session Management**:
   - Configure appropriate session timeouts for your use case
   - Implement session cleanup to prevent resource exhaustion
   - Consider implementing session renewal for long-running operations

## Integration Examples

### Docker Deployment

```dockerfile
# Server container with authentication
FROM rust:latest
COPY . /app
WORKDIR /app
RUN cargo build --release
EXPOSE 8080
CMD ["./target/release/trustedge-server", "--require-auth", "--bind-addr", "0.0.0.0:8080"]
```

### Kubernetes Configuration

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: trustedge-certs
data:
  server.cert: |
    # Server certificate content
---
apiVersion: apps/v1
kind: Deployment
metadata:
  name: trustedge-server
spec:
  replicas: 1
  selector:
    matchLabels:
      app: trustedge-server
  template:
    metadata:
      labels:
        app: trustedge-server
    spec:
      containers:
      - name: trustedge-server
        image: trustedge:latest
        args: ["--require-auth", "--bind-addr", "0.0.0.0:8080"]
        volumeMounts:
        - name: certs
          mountPath: /certs
      volumes:
      - name: certs
        configMap:
          name: trustedge-certs
```

## Troubleshooting

### Common Issues

1. **Certificate Not Found**:
   ```
   Error: Failed to load server certificate from ./server.cert
   ```
   - Ensure certificate file exists and is readable
   - Check file path is correct
   - Verify certificate format is valid JSON

2. **Authentication Failed**:
   ```
   Error: Authentication failed: Invalid signature
   ```
   - Verify client certificate contains valid signing key
   - Check server certificate is trusted and valid
   - Ensure clocks are synchronized between client and server

3. **Session Expired**:
   ```
   Error: Session validation failed: Session expired
   ```
   - Check session timeout configuration
   - Verify client is using current session ID
   - Consider implementing session renewal

### Debug Mode

Enable verbose logging for authentication debugging:

```bash
# Server debug
cargo run --bin trustedge-server -- \
    --require-auth \
    --verbose

# Client debug  
cargo run --bin trustedge-client -- \
    --enable-auth \
    --verbose
```

## Future Enhancements

- **Certificate Revocation**: Infrastructure for revoking compromised certificates
- **Multi-Factor Authentication**: Integration with hardware tokens or biometrics
- **Certificate Authorities**: Support for external CA-signed certificates
- **Key Rotation**: Automatic renewal and rotation of cryptographic keys
- **Audit Logging**: Comprehensive logging of authentication events
