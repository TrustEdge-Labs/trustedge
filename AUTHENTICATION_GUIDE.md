<!--
Copyright (c) 2025 John Turner
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/johnzilla/trustedge
-->

# TrustEdge Authentication Guide

Complete guide to TrustEdge's mutual authentication system for secure network operations.

## Table of Contents
- [Overview](#overview)
- [Quick Start](#quick-start)
- [Authentication Architecture](#authentication-architecture)
- [Certificate Management](#certificate-management)
- [Session Management](#session-management)
- [Configuration Reference](#configuration-reference)
- [Security Considerations](#security-considerations)
- [Troubleshooting](#troubleshooting)
- [Production Deployment](#production-deployment)

---

## Overview

TrustEdge implements **mutual authentication** using Ed25519 digital signatures to establish secure, validated sessions between clients and servers. This system ensures:

- **Server Authentication**: Clients verify they're connecting to legitimate TrustEdge servers
- **Client Authorization**: Servers validate client identity and authorize access
- **Session Security**: Cryptographically secure session IDs prevent unauthorized access
- **Automatic Certificate Management**: Ed25519 key pairs and certificates generated automatically

### Security Benefits

✅ **Mutual Trust**: Both client and server authenticate each other  
✅ **Identity Verification**: Ed25519 certificates provide cryptographic proof of identity  
✅ **Session Isolation**: Each connection gets unique, time-limited session  
✅ **Replay Protection**: Challenge-response protocol prevents replay attacks  
✅ **Forward Security**: Sessions expire automatically with configurable timeouts  

---

## Quick Start

### Enable Authentication

**Server:**
```bash
./target/release/trustedge-server \
  --listen 127.0.0.1:8080 \
  --require-auth \
  --server-identity "My Secure Server" \
  --decrypt \
  --use-keyring \
  --salt-hex $(openssl rand -hex 16)
```

**Client:**
```bash
./target/release/trustedge-client \
  --server 127.0.0.1:8080 \
  --input my_file.txt \
  --require-auth \
  --client-identity "My Client App" \
  --use-keyring \
  --salt-hex <same-salt-as-server>
```

### First Run Experience

1. **Server startup**: Generates Ed25519 signing key (saved to disk)
2. **Server certificate**: Creates identity certificate with your specified identity
3. **Client connection**: Generates client identity certificate on first use
4. **Mutual handshake**: Challenge-response authentication between both parties
5. **Secure session**: Establishes cryptographically secure session for data transfer

---

## Authentication Architecture

### How TrustEdge Secure Session Works

The following diagram shows the complete security flow from initial connection through authenticated data transfer:

```mermaid
sequenceDiagram
    participant C as Client App
    participant CC as Client Certificate
    participant S as Server
    participant SC as Server Certificate
    participant SM as Session Manager
    
    Note over C,SM: Phase 1: Certificate Preparation
    
    C->>CC: Check for client certificate
    alt No client certificate exists
        C->>CC: Generate Ed25519 key pair
        C->>CC: Create self-signed certificate<br/>Identity: "Client App v1.0"
        CC-->>C: Client certificate ready
    end
    
    S->>SC: Check for server certificate
    alt No server certificate exists
        S->>SC: Generate Ed25519 key pair
        S->>SC: Create self-signed certificate<br/>Identity: "Production Server"
        SC-->>S: Server certificate ready
    end
    
    Note over C,SM: Phase 2: Connection & Authentication
    
    C->>S: TCP Connection Request
    S-->>C: Connection Established
    
    C->>S: Authentication Request<br/>{"type": "auth_request"}
    
    S->>S: Generate random challenge (32 bytes)
    S->>S: Sign challenge with server private key
    S->>C: Server Challenge + Server Certificate<br/>{"challenge": "...", "server_cert": {...}}
    
    C->>C: Verify server certificate signature
    C->>C: Extract server public key
    C->>C: Verify challenge signature
    
    alt Server authentication fails
        C->>C: Reject connection
        C-->>S: Authentication Failed
    else Server authentication succeeds
        C->>C: Sign server challenge with client key
        C->>C: Generate client challenge (32 bytes)
        C->>C: Sign client challenge with client key
        C->>S: Client Response + Client Certificate<br/>{"challenge_response": "...", "client_challenge": "...", "client_cert": {...}}
    end
    
    S->>S: Verify client certificate signature
    S->>S: Extract client public key
    S->>S: Verify challenge response signature
    
    alt Client authentication fails
        S->>S: Reject connection
        S-->>C: Authentication Failed
    else Client authentication succeeds
        S->>S: Sign client challenge with server key
        S->>SM: Create new session
        SM->>SM: Generate cryptographic session ID (64-bit)
        SM->>SM: Set session timeout (default: 300s)
        SM-->>S: Session created
        S->>C: Session Established<br/>{"session_id": "0x...", "challenge_response": "...", "timeout": 300}
    end
    
    C->>C: Verify server's client challenge response
    C->>C: Store session ID
    
    Note over C,SM: Phase 3: Secure Data Transfer
    
    loop For each data chunk
        C->>C: Encrypt data with AES-256-GCM
        C->>C: Create signed manifest
        C->>S: Send encrypted chunk<br/>Session ID: 0x...
        S->>SM: Validate session ID
        
        alt Session invalid/expired
            SM-->>S: Session invalid
            S-->>C: Session Expired - Reconnect
        else Session valid
            SM-->>S: Session valid
            S->>S: Decrypt and process chunk
            S-->>C: Chunk received successfully
        end
    end
    
    Note over C,SM: Phase 4: Session Cleanup
    
    alt Normal completion
        C->>S: Transfer complete
        S->>SM: Mark session complete
    else Timeout or error
        SM->>SM: Session timeout (300s)
        SM->>SM: Clean up expired session
    end
```

### Security Properties

This authentication flow provides:

1. **🔐 Mutual Authentication**: Both client and server prove their identities using Ed25519 signatures
2. **🛡️ Identity Verification**: Certificates cryptographically bind identity strings to public keys  
3. **🔄 Challenge-Response**: Prevents replay attacks with fresh random challenges
4. **⏱️ Session Security**: Time-limited sessions with cryptographically secure IDs
5. **🎯 Forward Security**: Each session uses unique identifiers, limiting exposure

### Implementation References

| Component | Source Code | Purpose |
|-----------|-------------|---------|
| **Authentication Flow** | [`src/auth.rs:server_authenticate()`](trustedge-audio/src/auth.rs) | Server-side authentication implementation |
| **Challenge Generation** | [`src/auth.rs:client_authenticate()`](trustedge-audio/src/auth.rs) | Client-side authentication implementation |
| **Certificate Management** | [`src/auth.rs:ClientCertificate`](trustedge-audio/src/auth.rs) | Certificate creation and validation |
| **Session Management** | [`src/auth.rs:SessionManager`](trustedge-audio/src/auth.rs) | Session lifecycle and validation |
| **Integration Tests** | [`tests/auth_integration.rs`](trustedge-audio/tests/auth_integration.rs) | Complete authentication test suite |

### Certificate Format

TrustEdge certificates contain:
- **Ed25519 Public Key**: For signature verification (32 bytes)
- **Identity String**: Human-readable client/server identity  
- **Timestamp**: Certificate creation time
- **Signature**: Self-signed with corresponding private key (64 bytes)

---

## Certificate Management

### Server Credential Storage

**Where Server Identity Lives:**

```bash
# Default server certificate location (current working directory)
./trustedge-server.key    # Ed25519 private key (32 bytes + metadata)
./trustedge-server.cert   # Self-signed certificate with identity

# Custom server certificate path
./target/release/trustedge-server \
  --require-auth \
  --server-key /opt/trustedge/server-production.key \
  --server-identity "Production TrustEdge Server v1.0"

# Creates:
/opt/trustedge/server-production.key   # Private key file
/opt/trustedge/server-production.cert  # Certificate file
```

**Server Certificate Generation Process:**

1. **Check for Existing Certificate**: Server looks for `--server-key` path or `./trustedge-server.key`
2. **Generate Ed25519 Key Pair**: If missing, creates new 256-bit private key
3. **Create Self-Signed Certificate**: Binds server identity to public key
4. **Write to Filesystem**: Saves both private key and certificate files
5. **Set File Permissions**: Restricts private key access (recommended: `chmod 600`)

### Client Credential Storage

**Where Client Certificates Get Stored:**

```bash
# Default client certificate location (current working directory)
./trustedge-client.key    # Ed25519 private key (32 bytes + metadata)
./trustedge-client.cert   # Self-signed certificate with identity

# Custom client certificate path
./target/release/trustedge-client \
  --require-auth \
  --client-key ~/.config/trustedge/mobile-app.key \
  --client-identity "Executive Mobile App v2.1"

# Creates:
~/.config/trustedge/mobile-app.key   # Private key file
~/.config/trustedge/mobile-app.cert  # Certificate file
```

**Client Certificate Generation Process:**

1. **Check for Existing Certificate**: Client looks for `--client-key` path or `./trustedge-client.key`
2. **Generate Ed25519 Key Pair**: If missing, creates new 256-bit private key
3. **Create Self-Signed Certificate**: Binds client identity to public key
4. **Write to Filesystem**: Saves both private key and certificate files
5. **Secure Storage**: Client apps should store keys in secure locations

### Certificate File Format

**Private Key File (`.key`):**
```
-----BEGIN PRIVATE KEY-----
MC4CAQAwBQYDK2VwBCIEIFOr7...Ed25519PrivateKeyData...
-----END PRIVATE KEY-----
```

**Certificate File (`.cert`):**
```json
{
  "identity": "Production TrustEdge Server v1.0",
  "public_key": "AGd8f2V1...",
  "signature": "BDGd9f8A...",
  "created": "2025-08-30T10:15:30Z"
}
```

### Storage Security Best Practices

**File Permissions:**
```bash
# Secure server private key (root/service account only)
chmod 600 /opt/trustedge/server-production.key
chown trustedge-service:trustedge-service /opt/trustedge/server-production.key

# Secure client private key (user only)  
chmod 600 ~/.config/trustedge/mobile-app.key

# Certificate files can be world-readable (contain only public data)
chmod 644 /opt/trustedge/server-production.cert
chmod 644 ~/.config/trustedge/mobile-app.cert
```

**Directory Structure Examples:**

```
# Production Server Layout
/opt/trustedge/
├── server-production.key     # 600 trustedge-service:trustedge-service  
├── server-production.cert    # 644 trustedge-service:trustedge-service
├── logs/
└── config/

# Client Application Layout  
~/.config/trustedge/
├── mobile-app.key           # 600 user:user
├── mobile-app.cert          # 644 user:user
├── server-certificates/     # Trusted server cert store (optional)
└── config.json

# Development Environment
./trustedge-audio/
├── trustedge-server.key     # 600 developer:developer
├── trustedge-server.cert    # 644 developer:developer  
├── trustedge-client.key     # 600 developer:developer
├── trustedge-client.cert    # 644 developer:developer
└── target/release/
```

### Manual Certificate Paths

```bash
# Specify custom certificate locations
./target/release/trustedge-server \
  --require-auth \
  --server-identity "Production TrustEdge" \
  --server-key ./certificates/production_server.key

./target/release/trustedge-client \
  --require-auth \
  --client-identity "Mobile App v2.1" \
  --client-key ./certificates/mobile_client.key
```

### Certificate Security

- **Ed25519 Keys**: 256-bit private keys with 32-byte public keys
- **Self-Signed**: Each certificate self-signed with corresponding private key
- **Identity Binding**: Human-readable identity cryptographically bound to key
- **File Protection**: Private keys should be protected with appropriate file permissions
- **No Certificate Authority**: Eliminates single point of failure and complex PKI infrastructure
- **Automatic Rotation**: Generate new certificates by deleting existing files and restarting

---

## Session Management

### Session Lifecycle

1. **Authentication**: Mutual challenge-response creates session
2. **Session ID**: 64-bit cryptographically random session identifier  
3. **Timeout**: Configurable session timeout (default: 300 seconds)
4. **Validation**: All requests validated against active session
5. **Cleanup**: Expired sessions automatically removed

### Session Configuration

```bash
# Custom session timeout (600 seconds = 10 minutes)
./target/release/trustedge-server \
  --require-auth \
  --session-timeout 600 \
  --server-identity "Long Session Server"

# Default timeout (300 seconds = 5 minutes)
./target/release/trustedge-server \
  --require-auth \
  --server-identity "Standard Server"
```

### Session Security

- **Cryptographic Randomness**: Session IDs generated with secure random number generator
- **Time-Limited**: All sessions expire automatically 
- **Single Use**: Each session tied to specific client connection
- **Memory Protection**: Session data cleared on expiration

---

## Configuration Reference

### Authentication CLI Options

#### Server Options
| Option | Required | Description | Example |
|--------|----------|-------------|---------|
| `--require-auth` | Yes | Enable authentication requirement | `--require-auth` |
| `--server-identity` | Recommended | Server identity for certificates | `--server-identity "Production Server"` |
| `--server-key` | Optional | Path to server signing key | `--server-key ./server.key` |
| `--session-timeout` | Optional | Session timeout in seconds [default: 300] | `--session-timeout 600` |

#### Client Options  
| Option | Required | Description | Example |
|--------|----------|-------------|---------|
| `--require-auth` | Yes | Enable authentication | `--require-auth` |
| `--client-identity` | Recommended | Client identity for certificates | `--client-identity "Mobile App v1.0"` |
| `--client-key` | Optional | Path to client signing key | `--client-key ./client.key` |

### Configuration Examples

#### High-Security Production
```bash
# Production server with tight security
./target/release/trustedge-server \
  --listen 0.0.0.0:8443 \
  --require-auth \
  --server-identity "TrustEdge Production v2.1" \
  --server-key /etc/trustedge/server.key \
  --session-timeout 180 \  # 3 minutes
  --decrypt \
  --use-keyring \
  --output-dir /secure/uploads
```

#### Development Environment
```bash
# Development server with longer sessions
./target/release/trustedge-server \
  --listen 127.0.0.1:8080 \
  --require-auth \
  --server-identity "Development Server" \
  --session-timeout 1800 \  # 30 minutes
  --decrypt \
  --key-hex "dev_key_for_testing_only"
```

---

## Security Considerations

### Threat Model

**Protects Against:**
- ✅ Unauthorized server access (client authentication)
- ✅ Man-in-the-middle attacks (mutual authentication)
- ✅ Session hijacking (cryptographic session IDs)
- ✅ Replay attacks (challenge-response protocol)
- ✅ Identity spoofing (Ed25519 signatures)

**Does NOT Protect Against:**
- ❌ Compromised client/server private keys
- ❌ Network traffic analysis (use TLS for transport encryption)
- ❌ Side-channel attacks on cryptographic operations
- ❌ Social engineering or credential theft

### Best Practices

#### Certificate Management
- **Rotate Keys Regularly**: Generate new Ed25519 keys periodically
- **Secure Storage**: Protect private keys with appropriate file permissions (`600` or `400`)
- **Backup Strategy**: Securely backup signing keys for disaster recovery
- **Access Control**: Limit access to certificate files and directories

#### Network Security
- **TLS Transport**: Use TLS/SSL for transport-layer encryption in addition to authentication
- **Network Isolation**: Deploy servers in isolated network segments when possible
- **Monitoring**: Log authentication events for security monitoring
- **Firewall Rules**: Restrict network access to authenticated endpoints

#### Session Management
- **Short Timeouts**: Use shorter session timeouts for high-security environments
- **Session Monitoring**: Monitor active sessions and detect anomalies
- **Clean Shutdown**: Ensure proper session cleanup on server shutdown
- **Resource Limits**: Implement connection limits to prevent resource exhaustion

### Compliance Considerations

- **FIPS 140-2**: Ed25519 may not be FIPS-approved in all contexts
- **Industry Standards**: Verify authentication meets your industry requirements
- **Audit Trails**: Implement comprehensive logging for compliance auditing
- **Key Management**: Follow organizational key management policies

---

## Troubleshooting

### Common Authentication Errors

#### Client Cannot Connect

**Error:**
```
❌ Error: Server requires authentication but client not configured for auth
```

**Solution:**
Add authentication parameters to client:
```bash
./target/release/trustedge-client \
  --server 127.0.0.1:8080 \
  --input data.txt \
  --require-auth \
  --client-identity "My Client"
```

#### Authentication Handshake Fails

**Error:**
```
❌ Error: Authentication failed - client certificate rejected by server
```

**Possible Causes:**
1. **Client certificate corrupted**: Delete client certificate and regenerate
2. **Clock skew**: Ensure client/server clocks are synchronized
3. **Network issues**: Check network connectivity and firewall rules

**Solution:**
```bash
# Delete corrupted certificates and regenerate
rm client_identity.cert server_identity.cert
# Restart with --verbose for detailed error information
```

#### Session Expired

**Error:**
```
❌ Error: Session expired - please reconnect
```

**Solution:**
- Sessions expire after configured timeout (default: 300 seconds)
- Reconnect with fresh authentication
- Consider longer session timeout for your use case:
```bash
--session-timeout 600  # 10 minutes
```

### Debug Mode

Enable verbose logging for authentication debugging:

```bash
# Server debug
./target/release/trustedge-server \
  --require-auth \
  --verbose \
  --server-identity "Debug Server"

# Client debug  
./target/release/trustedge-client \
  --require-auth \
  --verbose \
  --client-identity "Debug Client"
```

### Certificate Inspection

Examine certificate contents:
```bash
# Check if certificates exist
ls -la *_identity.cert *.key

# View certificate creation time
stat server_identity.cert client_identity.cert
```

---

## Production Deployment

### Pre-Deployment Checklist

#### Security Setup
- [ ] Generate production Ed25519 key pairs
- [ ] Set appropriate file permissions on private keys (`chmod 600`)
- [ ] Configure secure certificate storage location
- [ ] Set production-appropriate session timeouts
- [ ] Enable comprehensive logging

#### Network Configuration
- [ ] Configure TLS/SSL for transport encryption
- [ ] Set up firewall rules for authenticated endpoints
- [ ] Configure load balancers for authenticated traffic
- [ ] Test authentication under expected load

#### Operational Readiness
- [ ] Document certificate rotation procedures
- [ ] Set up monitoring for authentication failures
- [ ] Configure alerting for security events
- [ ] Train operations team on authentication troubleshooting

### Production Example

```bash
#!/bin/bash
# Production TrustEdge server deployment script

# Security settings
SERVER_KEY="/etc/trustedge/production_server.key"
UPLOAD_DIR="/secure/trustedge/uploads"
LOG_FILE="/var/log/trustedge/auth.log"

# Ensure directories exist with correct permissions
mkdir -p "$(dirname $SERVER_KEY)" "$UPLOAD_DIR" "$(dirname $LOG_FILE)"
chmod 700 "$(dirname $SERVER_KEY)" "$UPLOAD_DIR"

# Start production server
./target/release/trustedge-server \
  --listen 0.0.0.0:8443 \
  --require-auth \
  --server-identity "TrustEdge Production Server $(date +%Y)" \
  --server-key "$SERVER_KEY" \
  --session-timeout 300 \
  --decrypt \
  --use-keyring \
  --salt-hex "$PRODUCTION_SALT" \
  --output-dir "$UPLOAD_DIR" \
  --verbose 2>&1 | tee -a "$LOG_FILE"
```

### Monitoring and Alerting

Monitor these authentication metrics:
- Authentication success/failure rates
- Session creation and expiration events
- Certificate generation and rotation events
- Connection attempt frequency and sources
- Session timeout and cleanup operations

### Backup and Recovery

Critical authentication components to backup:
- Server and client Ed25519 private keys
- Certificate files and metadata
- Authentication configuration parameters
- Session management configuration

---

## Advanced Topics

### Custom Identity Schemes

Use structured identity strings for complex deployments:
```bash
# Hierarchical identity naming
--server-identity "TrustEdge-Production-East-Server01"
--client-identity "MobileApp-v2.1.3-iOS-Device12345"

# Environment-specific identities
--server-identity "TrustEdge-${ENVIRONMENT}-${REGION}-${INSTANCE}"
--client-identity "Client-${APP_VERSION}-${DEVICE_TYPE}-${USER_ID}"
```

### Certificate Rotation

Implement certificate rotation for long-running deployments:
```bash
# Generate new server key
mv server_signing.key server_signing.key.backup
./target/release/trustedge-server --require-auth --server-identity "Rotated Server"
# Server generates new key automatically
```

### Integration with PKI

For enterprise deployments, consider integrating with existing PKI:
- Use TrustEdge authentication for application-layer security
- Combine with TLS client certificates for transport-layer security
- Implement certificate validation against enterprise CA

---

This completes the TrustEdge Authentication Guide. For additional questions or advanced use cases, refer to the [AUTHENTICATION.md](AUTHENTICATION.md) technical specification.
