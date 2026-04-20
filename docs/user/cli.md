<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: sealedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/sealedge
-->

# Sealedge CLI Reference

Complete command-line interface documentation for Sealedge, covering both the core encryption system and the .seal archive format.

## Table of Contents
- [Overview](#overview)
- [Point Attestation (.se-attestation.json)](#point-attestation-te-attestationjson)
  - [seal attest-sbom - Create SBOM Attestation](#seal-attest-sbom---create-sbom-attestation)
  - [seal verify-attestation - Verify Attestation](#seal-verify-attestation---verify-attestation)
- [Archive System (.seal)](#archive-system-trst)
  - [seal keygen - Generate Key Pair](#seal-keygen---generate-key-pair)
  - [seal wrap - Create Archives](#seal-wrap---create-archives)
  - [seal verify - Verify Archives](#seal-verify---verify-archives)
  - [seal unwrap - Decrypt Archives](#seal-unwrap---decrypt-archives)
  - [seal emit-request - Submit for Verification](#seal-emit-request---submit-for-verification)
- [Encrypted Key Files](#encrypted-key-files)
- [Core Encryption System](#core-encryption-system)
  - [sealedge - Envelope Encryption](#sealedge---envelope-encryption)
  - [Network Operations](#network-operations)
- [Complete Workflows](#complete-workflows)
- [Error Handling](#error-handling)

---

## Overview

Sealedge provides two complementary CLI tools:

1. **`seal`** - Archive + attestation CLI (keygen, wrap, verify, unwrap, emit-request, attest-sbom, verify-attestation)
2. **`sealedge`** - Core envelope encryption and network operations

Both tools are built after running `cargo build --workspace --release`.

---

## Point Attestation (.se-attestation.json)

Point attestation creates a lightweight JSON document that cryptographically binds two artifacts together (e.g., an SBOM and a binary). The attestation is self-contained: it includes the Ed25519 signature, BLAKE3 hashes, a random nonce, and the signer's public key. Any third party can verify it without access to Sealedge infrastructure.

### seal attest-sbom - Create SBOM Attestation

Bind a CycloneDX SBOM to a binary artifact and sign the binding with an Ed25519 key.

```bash
seal attest-sbom --binary <path> --sbom <path> \
  --device-key <key-path> --device-pub <pub-path> \
  --out <output-path>
```

**Arguments:**

| Flag | Required | Description |
|------|----------|-------------|
| `--binary` | Yes | Path to the binary artifact (the subject) |
| `--sbom` | Yes | Path to the CycloneDX JSON SBOM (the evidence) |
| `--device-key` | Yes | Path to Ed25519 private key file |
| `--device-pub` | Yes | Path to Ed25519 public key file |
| `--out` | No | Output path (default: `attestation.se-attestation.json`) |
| `--unencrypted` | No | Read key without passphrase prompt (for CI/automation) |

**Input validation:**
- Binary must not be empty (0 bytes)
- Binary must not exceed 256 MB
- SBOM must be valid JSON
- Key file must exist and be readable

**Output:** A `.se-attestation.json` file containing:
- `format`: `"te-point-attestation-v1"`
- `subject`: BLAKE3 hash, filename, and label ("binary") of the binary artifact
- `evidence`: BLAKE3 hash, filename, and label ("sbom") of the SBOM
- `signature`: Ed25519 signature over canonical JSON (signature field excluded)
- `nonce`: 16 random bytes (hex-encoded) for replay prevention
- `timestamp`: ISO 8601 timestamp
- `public_key`: The signer's public key (embedded for self-contained verification)

**Example:**

```bash
seal attest-sbom --binary target/release/myapp --sbom bom.cdx.json \
  --device-key build.key --device-pub build.pub
# Output: attestation.se-attestation.json
```

### seal verify-attestation - Verify Attestation

Verify an attestation document's Ed25519 signature, with optional file hash checking.

```bash
seal verify-attestation <attestation-path> --device-pub <pub-key>
```

**Arguments:**

| Flag | Required | Description |
|------|----------|-------------|
| `<attestation-path>` | Yes | Path to `.se-attestation.json` file |
| `--device-pub` | Yes | Public key string (`ed25519:...`) or path to `.pub` file |
| `--binary` | No | Path to binary file for hash verification |
| `--sbom` | No | Path to SBOM file for hash verification |

**Exit codes:**
- `0` - Verification passed
- `1` - General error (IO, JSON parsing, bad input)
- `10` - Verification failed (invalid signature or hash mismatch)

**Example:**

```bash
# Signature verification only
seal verify-attestation attestation.se-attestation.json \
  --device-pub "$(cat build.pub)"

# Signature + file hash verification
seal verify-attestation attestation.se-attestation.json \
  --device-pub "$(cat build.pub)" \
  --binary target/release/myapp --sbom bom.cdx.json
```

---

## Archive System (.seal)

The `seal` command provides secure archival capabilities with Ed25519 digital signatures and cryptographic chunk verification.

### seal keygen - Generate Key Pair

Generate a device Ed25519 signing key pair for archive signing.

```bash
seal keygen --out-key <KEY_PATH> --out-pub <PUB_PATH> [--unencrypted]
```

| Option | Description |
|--------|-------------|
| `--out-key <PATH>` | Output path for the private key file |
| `--out-pub <PATH>` | Output path for the public key file |
| `--unencrypted` | Generate plaintext key (no passphrase). CI/automation only — see [Encrypted Key Files](#encrypted-key-files) |

```bash
# Generate encrypted key (passphrase prompted)
seal keygen --out-key device.key --out-pub device.pub

# Generate unencrypted key for CI/automation
seal keygen --out-key device.key --out-pub device.pub --unencrypted
```

### seal wrap - Create Archives

Create a signed .seal archive from input data.

```bash
seal wrap --in <INPUT> --out <OUTPUT> [OPTIONS]
```

#### Required Arguments

| Option | Description | Example |
|--------|-------------|---------|
| `--in <PATH>` | Input file or data stream | `--in video.bin` |
| `--out <PATH>` | Output .seal archive directory | `--out recording.seal` |

#### Archive Options

| Option | Default | Description | Example |
|--------|---------|-------------|---------|
| `--profile <PROFILE>` | `generic` | Archive profile type: `generic`, `cam.video`, `sensor`, `audio`, `log` | `--profile cam.video` |
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
seal wrap --in recording.bin --out recording.seal

# High-quality security camera archive
seal wrap \
  --in security_feed.bin \
  --out evidence.seal \
  --profile cam.video \
  --fps 60 \
  --resolution 3840x2160 \
  --device-model "SecureCam X1" \
  --device-id "CAM-LOBBY-01"

# Continuous recording with linking
seal wrap \
  --in segment_002.bin \
  --out segment_002.seal \
  --prev-archive-hash "$(cat segment_001.hash)"
```

### seal verify - Verify Archives

Verify the cryptographic integrity of a .seal archive.

```bash
seal verify <ARCHIVE> --device-pub <PUBLIC_KEY>
```

#### Arguments

| Argument | Description | Example |
|----------|-------------|---------|
| `<ARCHIVE>` | Path to .seal archive directory | `recording.seal` |
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
seal verify recording.seal --device-pub "ed25519:GAUpGXoor5gP6JDkeVtj/PV4quuyLlZlojizplendEUlSU="

# Verify with stored public key
seal verify evidence.seal --device-pub "$(cat device.pub)"
```

#### Verification Output

```
Signature: PASS
Continuity: PASS
Segments: 16  Duration(s): 32.0  Chunk(s): 2.0
```

### seal unwrap - Decrypt Archives

Decrypt a .seal archive and recover the original data.

```bash
seal unwrap <ARCHIVE> --device-key <KEY_PATH> --out <OUTPUT> [--unencrypted]
```

| Argument | Description |
|----------|-------------|
| `<ARCHIVE>` | Path to .seal archive directory |
| `--device-key <PATH>` | Path to device private key file |
| `--out <PATH>` | Output path for recovered data |
| `--unencrypted` | Read key without passphrase prompt (CI/automation only) |

```bash
# Recover data from an archive (passphrase prompted if key is encrypted)
seal unwrap recording.seal --device-key device.key --out recovered.bin

# Recover without passphrase (unencrypted key)
seal unwrap recording.seal --device-key device.key --out recovered.bin --unencrypted
```

### seal emit-request - Submit for Verification

Submit an archive to a Sealedge platform server for remote verification.

```bash
seal emit-request --archive <PATH> --device-pub <KEY> --out <PATH> [--post <URL>]
```

| Option | Description |
|--------|-------------|
| `--archive <PATH>` | Path to .seal archive directory |
| `--device-pub <KEY>` | Device public key (ed25519: prefixed string) |
| `--out <PATH>` | Output path for the JSON verification request |
| `--post <URL>` | POST the request to this platform endpoint |

```bash
# Write request to file
seal emit-request --archive archive.seal --device-pub device.pub --out request.json

# Submit directly to platform server
seal emit-request --archive archive.seal --device-pub device.pub --out request.json --post http://localhost:3001/v1/verify
```

---

## Encrypted Key Files

Device private keys are encrypted at rest using PBKDF2-HMAC-SHA256 (600k iterations) + AES-256-GCM (format: `SEALEDGE-KEY-V1`). A passphrase is prompted at runtime.

For CI/automation where interactive prompts are not possible, use `--unencrypted`:
- `seal keygen --unencrypted` — generates plaintext key file
- `seal wrap --unencrypted` — reads key without passphrase prompt
- `seal unwrap --unencrypted` — reads key without passphrase prompt

**Production devices should always use encrypted key files.** The `--unencrypted` flag is an explicit escape hatch.

---

## Core Encryption System

The `sealedge` command provides envelope encryption, key management, and network operations.

### sealedge - Envelope Encryption

Encrypt and decrypt files using AES-256-GCM with metadata preservation.

```bash
sealedge [OPTIONS]
```

#### Core Operations

| Option | Description | Example |
|--------|-------------|---------|
| `-i, --input <INPUT>` | Input file (any binary data) | `--input document.pdf` |
| `-o, --out <OUT>` | Output file path | `--out decrypted.pdf` |
| `--envelope <ENVELOPE>` | Write encrypted envelope to .seal file | `--envelope encrypted.seal` |
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
sealedge --input document.pdf --envelope encrypted.seal --key-out mykey.hex

# Decrypt file
sealedge --decrypt --input encrypted.seal --out recovered.pdf --key-hex $(cat mykey.hex)

# Encrypt with keyring
sealedge --set-passphrase "my_secure_passphrase"
sealedge --input file.txt --envelope file.seal --use-keyring --salt-hex "abcdef1234567890abcdef1234567890"

# Inspect without decryption
sealedge --input encrypted.seal --inspect
```

### Network Operations

Sealedge supports secure client-server communication with mutual authentication.

#### Server Mode

```bash
sealedge-server --listen <ADDRESS> [OPTIONS]
```

| Option | Description | Example |
|--------|-------------|---------|
| `--listen <ADDR>` | Server bind address | `--listen 127.0.0.1:8080` |
| `--require-auth` | Enable mutual authentication | `--require-auth` |
| `--decrypt` | Auto-decrypt received files | `--decrypt` |
| `--key-hex <KEY>` | Shared encryption key | `--key-hex $(openssl rand -hex 32)` |

#### Client Mode

```bash
sealedge-client --server <ADDRESS> [OPTIONS]
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
sealedge-server --listen 127.0.0.1:8080 --require-auth --decrypt --key-hex $(openssl rand -hex 32)

# Connect with authenticated client
sealedge-client --server 127.0.0.1:8080 --input file.txt --require-auth --key-hex $(cat shared.key)
```

---

## Complete Workflows

### Secure Evidence Chain

Create a cryptographically linked chain of evidence archives:

```bash
# First archive
seal wrap --in evidence_001.bin --out evidence_001.seal --device-id "CAM-COURT-01"
HASH_001=$(blake3sum evidence_001.seal/manifest.json | cut -d' ' -f1)

# Linked archive
seal wrap --in evidence_002.bin --out evidence_002.seal --device-id "CAM-COURT-01" --prev-archive-hash "$HASH_001"

# Verify chain
seal verify evidence_001.seal --device-pub "$(cat device.pub)"
seal verify evidence_002.seal --device-pub "$(cat device.pub)"
```

### Hybrid Encryption + Archive

Combine envelope encryption with archive format:

```bash
# Encrypt sensitive data
sealedge --input sensitive.pdf --envelope encrypted.seal --key-out secret.key

# Archive the encrypted envelope
seal wrap --in encrypted.seal --out archived.seal --profile data.secure

# Verify archive integrity
seal verify archived.seal --device-pub "$(cat device.pub)"

# Recover data
sealedge --decrypt --input encrypted.seal --out recovered.pdf --key-hex $(cat secret.key)
```

### Network + Archive Pipeline

Stream encrypted data over network and archive:

```bash
# Server: receive and archive
sealedge-server --listen 127.0.0.1:8080 --decrypt --key-hex $(cat shared.key) &
SERVER_PID=$!

# Client: send encrypted data
sealedge-client --server 127.0.0.1:8080 --input data.bin --key-hex $(cat shared.key)

# Archive received data
seal wrap --in received_data.bin --out network_archive.seal

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
| `Format error` | Unrecognized envelope format | Verify file is Sealedge format |

### Network Errors

| Error Type | Description | Solution |
|------------|-------------|----------|
| `Connection refused` | Server not reachable | Check server status, network connectivity |
| `Authentication failed` | Mutual auth rejected | Verify certificates, key compatibility |
| `Protocol error` | Communication protocol mismatch | Ensure compatible Sealedge versions |

### Debugging Commands

```bash
# Run with debug logging
RUST_LOG=debug sealedge --input file.txt --envelope test.seal --key-out test.key 2>&1 | head -20

# Test archive validation
cargo test -p sealedge-seal-cli --test acceptance -- --nocapture

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