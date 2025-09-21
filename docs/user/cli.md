<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->

# TrustEdge CLI Reference

Complete command-line interface documentation for TrustEdge, covering both the core encryption system and the .trst archive format.

## Table of Contents
- [Overview](#overview)
- [Archive System (.trst)](#archive-system-trst)
  - [trst wrap - Create Archives](#trst-wrap---create-archives)
  - [trst verify - Verify Archives](#trst-verify---verify-archives)
- [Core Encryption System](#core-encryption-system)
  - [trustedge-core - Envelope Encryption](#trustedge-core---envelope-encryption)
  - [Network Operations](#network-operations)
- [Complete Workflows](#complete-workflows)
- [Error Handling](#error-handling)

---

## Overview

TrustEdge provides two complementary CLI tools:

1. **`trst`** - .trst archive creation and verification system
2. **`trustedge-core`** - Core envelope encryption and network operations

Both tools are built after running `cargo build --workspace --release`.

---

## Archive System (.trst)

The `trst` command provides secure archival capabilities with Ed25519 digital signatures and cryptographic chunk verification.

### trst wrap - Create Archives

Create a signed .trst archive from input data.

```bash
trst wrap --in <INPUT> --out <OUTPUT> [OPTIONS]
```

#### Required Arguments

| Option | Description | Example |
|--------|-------------|---------|
| `--in <PATH>` | Input file or data stream | `--in video.bin` |
| `--out <PATH>` | Output .trst archive directory | `--out recording.trst` |

#### Archive Options

| Option | Default | Description | Example |
|--------|---------|-------------|---------|
| `--profile <PROFILE>` | `cam.video` | Archive profile type | `--profile cam.video` |
| `--chunk-size <SIZE>` | `1048576` | Chunk size in bytes (1MB) | `--chunk-size 4096` |
| `--chunk-seconds <SECONDS>` | `2` | Time duration per chunk | `--chunk-seconds 1.5` |

#### Device Configuration

| Option | Default | Description | Example |
|--------|---------|-------------|---------|
| `--device-key <PATH>` | (generated) | Existing device signing key | `--device-key device.key` |
| `--device-id <ID>` | (generated) | Device identifier | `--device-id "CAM001"` |
| `--device-model <MODEL>` | `TrustEdgeRefCam` | Device model name | `--device-model "SecurityCam Pro"` |
| `--device-fw <VERSION>` | `1.0.0` | Device firmware version | `--device-fw "2.1.3"` |

#### Capture Metadata

| Option | Default | Description | Example |
|--------|---------|-------------|---------|
| `--fps <FPS>` | `30` | Frames per second | `--fps 60` |
| `--resolution <RES>` | `1920x1080` | Video resolution | `--resolution 4096x2160` |
| `--codec <CODEC>` | `raw` | Video codec | `--codec h264` |
| `--started-at <TIME>` | (current) | Capture start time (RFC3339) | `--started-at 2025-01-15T10:30:00Z` |
| `--tz <TIMEZONE>` | `UTC` | Timezone | `--tz "America/New_York"` |

#### Continuity Options

| Option | Description | Example |
|--------|-------------|---------|
| `--prev-archive-hash <HASH>` | Link to previous archive for chain continuity | `--prev-archive-hash "abc123..."` |

#### Example Usage

```bash
# Basic archive creation
trst wrap --in recording.bin --out recording.trst

# High-quality security camera archive
trst wrap \
  --in security_feed.bin \
  --out evidence.trst \
  --profile cam.video \
  --fps 60 \
  --resolution 3840x2160 \
  --device-model "SecureCam X1" \
  --device-id "CAM-LOBBY-01"

# Continuous recording with linking
trst wrap \
  --in segment_002.bin \
  --out segment_002.trst \
  --prev-archive-hash "$(cat segment_001.hash)"
```

### trst verify - Verify Archives

Verify the cryptographic integrity of a .trst archive.

```bash
trst verify <ARCHIVE> --device-pub <PUBLIC_KEY>
```

#### Arguments

| Argument | Description | Example |
|----------|-------------|---------|
| `<ARCHIVE>` | Path to .trst archive directory | `recording.trst` |
| `--device-pub <KEY>` | Device public key for verification | `--device-pub "ed25519:GAUpGXoor5gP..."` |

#### Verification Process

The verify command performs comprehensive validation:

1. **Signature Verification** - Ed25519 signature validation against manifest
2. **Chunk Integrity** - BLAKE3 hash verification of all chunk files
3. **Continuity Checks** - Temporal and sequential consistency validation
4. **Duration Sanity** - Detection of unrealistic time segments

#### Example Usage

```bash
# Basic verification
trst verify recording.trst --device-pub "ed25519:GAUpGXoor5gP6JDkeVtj/PV4quuyLlZlojizplendEUlSU="

# Verify with stored public key
trst verify evidence.trst --device-pub "$(cat device.pub)"
```

#### Verification Output

```
Signature: PASS
Continuity: PASS
Segments: 16  Duration(s): 32.0  Chunk(s): 2.0
```

---

## Core Encryption System

The `trustedge-core` command provides envelope encryption, key management, and network operations.

### trustedge-core - Envelope Encryption

Encrypt and decrypt files using AES-256-GCM with metadata preservation.

```bash
trustedge-core [OPTIONS]
```

#### Core Operations

| Option | Description | Example |
|--------|-------------|---------|
| `-i, --input <INPUT>` | Input file (any binary data) | `--input document.pdf` |
| `-o, --out <OUT>` | Output file path | `--out decrypted.pdf` |
| `--envelope <ENVELOPE>` | Write encrypted envelope to .trst file | `--envelope encrypted.trst` |
| `--decrypt` | Decrypt mode (read from --input, write to --out) | `--decrypt` |

#### Chunk Configuration

| Option | Default | Description | Example |
|--------|---------|-------------|---------|
| `--chunk <SIZE>` | `4096` | Chunk size in bytes | `--chunk 8192` |
| `--no-plaintext` | - | Skip plaintext output (encrypt only) | `--no-plaintext` |

#### Key Management

| Option | Description | Example |
|--------|-------------|---------|
| `--key-hex <KEY>` | 64 hex chars (32 bytes) AES-256 key | `--key-hex 0123456789abcdef...` |
| `--key-out <PATH>` | Save generated key to file | `--key-out mykey.hex` |
| `--set-passphrase <PASS>` | Store passphrase in OS keyring | `--set-passphrase "secure_phrase"` |
| `--salt-hex <SALT>` | 32 hex chars (16 bytes) for key derivation | `--salt-hex "abcdef..."` |
| `--use-keyring` | Use keyring passphrase + salt for key | `--use-keyring` |

#### Format Options

| Option | Description | Example |
|--------|-------------|---------|
| `--inspect` | Show metadata without decryption | `--inspect` |
| `--force-raw` | Force raw output regardless of detected type | `--force-raw` |
| `--verbose` | Enable verbose format details | `--verbose` |

#### Example Usage

```bash
# Basic file encryption
trustedge-core --input document.pdf --envelope encrypted.trst --key-out mykey.hex

# Decrypt file
trustedge-core --decrypt --input encrypted.trst --out recovered.pdf --key-hex $(cat mykey.hex)

# Encrypt with keyring
trustedge-core --set-passphrase "my_secure_passphrase"
trustedge-core --input file.txt --envelope file.trst --use-keyring --salt-hex "abcdef1234567890abcdef1234567890"

# Inspect without decryption
trustedge-core --input encrypted.trst --inspect
```

### Network Operations

TrustEdge supports secure client-server communication with mutual authentication.

#### Server Mode

```bash
trustedge-server --listen <ADDRESS> [OPTIONS]
```

| Option | Description | Example |
|--------|-------------|---------|
| `--listen <ADDR>` | Server bind address | `--listen 127.0.0.1:8080` |
| `--require-auth` | Enable mutual authentication | `--require-auth` |
| `--decrypt` | Auto-decrypt received files | `--decrypt` |
| `--key-hex <KEY>` | Shared encryption key | `--key-hex $(openssl rand -hex 32)` |

#### Client Mode

```bash
trustedge-client --server <ADDRESS> [OPTIONS]
```

| Option | Description | Example |
|--------|-------------|---------|
| `--server <ADDR>` | Server address | `--server 127.0.0.1:8080` |
| `--input <FILE>` | File to send | `--input document.txt` |
| `--require-auth` | Use mutual authentication | `--require-auth` |
| `--key-hex <KEY>` | Shared encryption key | `--key-hex $(cat shared.key)` |

#### Network Example

```bash
# Start authenticated server
trustedge-server --listen 127.0.0.1:8080 --require-auth --decrypt --key-hex $(openssl rand -hex 32)

# Connect with authenticated client
trustedge-client --server 127.0.0.1:8080 --input file.txt --require-auth --key-hex $(cat shared.key)
```

---

## Complete Workflows

### Secure Evidence Chain

Create a cryptographically linked chain of evidence archives:

```bash
# First archive
trst wrap --in evidence_001.bin --out evidence_001.trst --device-id "CAM-COURT-01"
HASH_001=$(blake3sum evidence_001.trst/manifest.json | cut -d' ' -f1)

# Linked archive
trst wrap --in evidence_002.bin --out evidence_002.trst --device-id "CAM-COURT-01" --prev-archive-hash "$HASH_001"

# Verify chain
trst verify evidence_001.trst --device-pub "$(cat device.pub)"
trst verify evidence_002.trst --device-pub "$(cat device.pub)"
```

### Hybrid Encryption + Archive

Combine envelope encryption with archive format:

```bash
# Encrypt sensitive data
trustedge-core --input sensitive.pdf --envelope encrypted.trst --key-out secret.key

# Archive the encrypted envelope
trst wrap --in encrypted.trst --out archived.trst --profile data.secure

# Verify archive integrity
trst verify archived.trst --device-pub "$(cat device.pub)"

# Recover data
trustedge-core --decrypt --input encrypted.trst --out recovered.pdf --key-hex $(cat secret.key)
```

### Network + Archive Pipeline

Stream encrypted data over network and archive:

```bash
# Server: receive and archive
trustedge-server --listen 127.0.0.1:8080 --decrypt --key-hex $(cat shared.key) &
SERVER_PID=$!

# Client: send encrypted data
trustedge-client --server 127.0.0.1:8080 --input data.bin --key-hex $(cat shared.key)

# Archive received data
trst wrap --in received_data.bin --out network_archive.trst

# Cleanup
kill $SERVER_PID
```

---

## Error Handling

### Archive Verification Errors

| Error Type | Description | Solution |
|------------|-------------|----------|
| `signature error` | Ed25519 signature validation failed | Check device public key, verify archive integrity |
| `missing chunk` | Required chunk file not found | Check archive completeness, file permissions |
| `hash mismatch` | Chunk content doesn't match expected hash | Archive corrupted, re-create from source |
| `unexpected end` | Archive truncated or malformed | Check for incomplete transfers, storage issues |

### Encryption Errors

| Error Type | Description | Solution |
|------------|-------------|----------|
| `Invalid key length` | AES key not 32 bytes (64 hex chars) | Verify key format and length |
| `Decryption failed` | Wrong key or corrupted data | Check key correctness, file integrity |
| `Format error` | Unrecognized envelope format | Verify file is TrustEdge format |

### Network Errors

| Error Type | Description | Solution |
|------------|-------------|----------|
| `Connection refused` | Server not reachable | Check server status, network connectivity |
| `Authentication failed` | Mutual auth rejected | Verify certificates, key compatibility |
| `Protocol error` | Communication protocol mismatch | Ensure compatible TrustEdge versions |

### Debugging Commands

```bash
# Run with debug logging
RUST_LOG=debug trustedge-core --input file.txt --envelope test.trst --key-out test.key 2>&1 | head -20

# Test archive validation
cargo test -p trustedge-trst-cli --test acceptance -- --nocapture

# Check network connectivity
telnet 127.0.0.1 8080

# Verify YubiKey hardware
ykman piv info
```

---

## Performance Notes

- **Chunk Size**: Larger chunks (1MB+) improve throughput, smaller chunks (4KB) reduce memory usage
- **Network**: Use authentication for production, skip for high-throughput scenarios
- **Archive**: Ed25519 signing adds ~1ms per archive, BLAKE3 hashing is very fast
- **Memory**: Streaming design maintains <50MB RAM regardless of file size

For additional examples and advanced usage, see [Examples Index](examples/README.md).

---

*This document is part of the TrustEdge project documentation.*

**📖 Links:**
- **[TrustEdge Home](https://github.com/TrustEdge-Labs/trustedge)** - Main repository
- **[TrustEdge Labs](https://github.com/TrustEdge-Labs)** - Organization profile
- **[Documentation](https://github.com/TrustEdge-Labs/trustedge/tree/main/docs)** - Complete docs
- **[Issues](https://github.com/TrustEdge-Labs/trustedge/issues)** - Bug reports & features

**⚖️ Legal:**
- **Copyright**: © 2025 TrustEdge Labs LLC
- **License**: Mozilla Public License 2.0 ([MPL-2.0](https://mozilla.org/MPL/2.0/))
- **Commercial**: [Enterprise licensing available](mailto:enterprise@trustedgelabs.com)