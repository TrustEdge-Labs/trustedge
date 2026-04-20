<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: sealedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/sealedge
-->

# Network Mode Examples

Secure client-server communication with mutual authentication and resilient connections.

## Network Mode Quick Start

**Authenticated server setup:**
```bash
# Start server with authentication required
./target/release/sealedge-server \
  --listen 127.0.0.1:8080 \
  --require-auth \
  --decrypt \
  --verbose
```

**Authenticated client connection:**
```bash
# Connect client with authentication
./target/release/sealedge-client \
  --server 127.0.0.1:8080 \
  --input file.txt \
  --require-auth \
  --verbose
```

## Connection Resilience & Error Recovery

### Automatic Retry with Exponential Backoff

```bash
# Client with retry configuration
./target/release/sealedge-client \
  --server 192.168.1.100:8080 \
  --input large_file.bin \
  --retry-attempts 5 \
  --retry-delay 1000 \
  --timeout 30000 \
  --verbose
```

### Network Interruption Handling

```bash
# Server with connection timeout handling
./target/release/sealedge-server \
  --listen 0.0.0.0:8080 \
  --connection-timeout 60000 \
  --max-connections 10 \
  --verbose \
  --decrypt
```

## Secure Authentication Examples

### Mutual TLS Authentication

```bash
# Generate server certificate
openssl req -x509 -newkey rsa:4096 -keyout server_key.pem -out server_cert.pem -days 365 -nodes

# Start server with certificate
./target/release/sealedge-server \
  --listen 0.0.0.0:8443 \
  --cert server_cert.pem \
  --key server_key.pem \
  --require-client-cert \
  --decrypt
```

### Certificate-Based Client Authentication

```bash
# Generate client certificate
openssl req -x509 -newkey rsa:4096 -keyout client_key.pem -out client_cert.pem -days 365 -nodes

# Connect with client certificate
./target/release/sealedge-client \
  --server secure-server.example.com:8443 \
  --cert client_cert.pem \
  --key client_key.pem \
  --ca-cert server_cert.pem \
  --input sensitive_data.txt
```

## Legacy Network Examples (No Authentication)

### Basic Server-Client Communication

```bash
# Simple server (no authentication)
./target/release/sealedge-server \
  --listen 127.0.0.1:8080 \
  --decrypt \
  --key-hex "a1b2c3d4e5f6789a0b1c2d3e4f5a6b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4"

# Simple client (no authentication)
./target/release/sealedge-client \
  --server 127.0.0.1:8080 \
  --input document.txt \
  --key-hex "a1b2c3d4e5f6789a0b1c2d3e4f5a6b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4"
```

---


[← Back to Examples Index](README.md)

---

*This document is part of the Sealedge project documentation.*

**📖 Links:**
- **[Sealedge Home](https://github.com/TrustEdge-Labs/sealedge)** - Main repository
- **[Sealedge Labs](https://github.com/TrustEdge-Labs)** - Organization profile
- **[Documentation](https://github.com/TrustEdge-Labs/sealedge/tree/main/docs)** - Complete docs
- **[Issues](https://github.com/TrustEdge-Labs/sealedge/issues)** - Bug reports & features

**⚖️ Legal:**
- **Copyright**: © 2025 Sealedge Labs LLC
- **License**: Mozilla Public License 2.0 ([MPL-2.0](https://mozilla.org/MPL/2.0/))
- **Commercial**: [Enterprise licensing available](mailto:enterprise@trustedgelabs.com)
