<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->
# TrustEdge CLI Reference

Complete command-line interface documentation for TrustEdge.

## Table of Contents
- [CLI Options](#cli-options)
- [Network Operations](#network-operations)
- [Authentication](#authentication)
- [Universal Backend Registry](#universal-backend-registry)
- [Error Handling](#error-handling)
- [Complete Workflows](#complete-workflows)

---

## CLI Options

### Core Options

| Option | Description | Example |
|--------|-------------|---------|
| `-i, --input <INPUT>` | Input file (opaque bytes) | `--input document.txt` |
| `-o, --out <OUT>` | Output file for round-tripped plaintext (encrypt mode) or decrypt target (decrypt mode) | `--out decrypted.txt` |
| `--chunk <CHUNK>` | Chunk size in bytes [default: 4096] | `--chunk 8192` |
| `--envelope <ENVELOPE>` | Optional: write envelope (header + records) to this .trst file | `--envelope encrypted.trst` |
| `--no-plaintext` | Skip writing plaintext during encrypt (still verifies+envelopes) | `--no-plaintext` |
| `--decrypt` | Decrypt mode: read .trst from --input and write plaintext to --out | `--decrypt` |

### Format-Aware Options

| Option | Description | Example |
|--------|-------------|---------|
| `--inspect` | Show data type information from manifest without decryption | `--inspect` |
| `--force-raw` | Force raw output regardless of data type | `--force-raw` |
| `--verbose` | Enable verbose output for format details | `--verbose` |

### Key Management Options

| Option | Description | Example |
|--------|-------------|---------|
| `--key-hex <KEY_HEX>` | 64 hex chars (32 bytes) AES-256 key | `--key-hex 0123456789abcdef...` |
| `--key-out <KEY_OUT>` | Where to store generated key (encrypt mode) as hex | `--key-out mykey.hex` |
| `--set-passphrase <PASSPHRASE>` | Store passphrase in OS keyring (one-time setup) | `--set-passphrase "my_secure_passphrase"` |
| `--salt-hex <SALT_HEX>` | Salt for key derivation (32 hex chars = 16 bytes) | `--salt-hex "abcdef1234567890abcdef1234567890"` |
| `--use-keyring` | Use key derived from keyring passphrase + salt instead of --key-hex | `--use-keyring` |

### Universal Backend Options

| Option | Description | Example |
|--------|-------------|---------|
| `--list-backends` | List all available backends with capabilities | `--list-backends` |
| `--backend-info <NAME>` | Show detailed capabilities for a specific backend | `--backend-info universal_keyring` |
| `--backend-preference <OP>:<BACKEND>` | Set backend preference for specific operations | `--backend-preference encryption:universal_keyring` |
| `--backend-config <CONFIG>` | Backend-specific configuration (format: key=value) | `--backend-config "iterations=150000"` |
| `--show-operation-flow` | Display which backends handle each operation | `--show-operation-flow` |

### Audio Capture Options

| Option | Description | Example |
|--------|-------------|---------|
| `--live-capture` | Enable live audio capture from microphone | `--live-capture` |
| `--audio-device <DEVICE>` | Audio device name (use --list-audio-devices to see options) | `--audio-device "hw:CARD=USB_AUDIO,DEV=0"` |
| `--list-audio-devices` | List available audio input devices | `--list-audio-devices` |
| `--sample-rate <RATE>` | Audio sample rate in Hz [default: 44100] | `--sample-rate 48000` |
| `--channels <CHANNELS>` | Number of audio channels (1=mono, 2=stereo) [default: 1] | `--channels 2` |
| `--chunk-duration-ms <MS>` | Duration of each audio chunk in milliseconds [default: 1000] | `--chunk-duration-ms 500` |
| `--max-duration <SECONDS>` | Maximum capture duration in seconds (0 = unlimited) [default: 0] | `--max-duration 30` |

#### Audio Device Selection and Formatting

**To discover available devices, always run first:**
```bash
./target/release/trustedge-core --list-audio-devices
```

**Common Device Name Formats:**

```bash
# Linux ALSA device names
--audio-device "hw:CARD=PCH,DEV=0"          # Built-in audio
--audio-device "hw:CARD=USB_AUDIO,DEV=0"    # USB microphone
--audio-device "hw:CARD=Headset,DEV=0"      # Bluetooth headset
--audio-device "default"                     # System default device

# macOS device names
--audio-device "Built-in Microphone"        # Internal mic
--audio-device "USB Audio CODEC"            # USB microphone
--audio-device "AirPods Pro"                # Bluetooth headset

# Windows device names  
--audio-device "Microphone (Realtek Audio)" # Built-in mic
--audio-device "USB Audio Device"           # USB microphone
--audio-device "Headset Microphone"         # USB/Bluetooth headset
```

**Audio Troubleshooting Quick Reference:**

| Issue | Quick Check | Solution |
|-------|-------------|----------|
| `No audio devices found` | Check permissions | Run `--list-audio-devices` as current user |
| `Device access denied` | Check system audio | Verify microphone permissions in OS settings |
| `Silent audio capture` | Check device levels | Test with `arecord`/system audio tools |
| `Invalid device name` | Check exact spelling | Copy device name exactly from `--list-audio-devices` |
| `Audio choppy/distorted` | Check sample rates | Use `--sample-rate` matching device capability |

**● For detailed audio troubleshooting, device configuration, and system-specific setup, see [TESTING.md](TESTING.md#audio-system-testing).**

**Note**: Audio features require building with `--features audio`. Install audio system dependencies first:
- **Linux**: `sudo apt-get install libasound2-dev pkg-config`
- **macOS**: Included with Xcode/Command Line Tools
- **Windows**: Included with Windows SDK

#### Audio Output Format & Format-Aware Decryption

**Format-Aware Behavior:** TrustEdge now provides format-aware decryption with MIME type detection and intelligent output handling:

**For File Inputs (any file type):**
- **Input**: Any file (PDF, JSON, MP3, WAV, etc.) → Encrypted to .trst with MIME type detection
- **Output**: Original file format preserved exactly (byte-for-byte identical)
- **Detection**: Automatic MIME type detection (application/json, application/pdf, audio/mpeg, etc.)
- **Example**: `document.pdf` → `document.trst` → decrypt to `document.pdf`

**For Live Audio Inputs (--live-capture):**
- **Input**: Live microphone capture → Encrypted to .trst with audio metadata
- **Output**: Raw PCM data (32-bit float, little-endian)
- **Detection**: Automatic audio parameter detection (sample rate, channels, format)
- **Requires Conversion**: Must convert PCM to playable format

**Format Inspection:**
```bash
# Inspect format without decryption
./target/release/trustedge-core --input data.trst --inspect --verbose

# Example output for a JSON file:
# TrustEdge Archive Information:
#   File: data.trst
#   Data Type: File
#   MIME Type: application/json
#   Output Behavior: Original file format preserved

# Example output for live audio:
# TrustEdge Archive Information:
#   File: audio.trst
#   Data Type: Audio
#   Sample Rate: 44100 Hz
#   Channels: 1 (mono)
#   Format: f32le
#   Output Behavior: Raw PCM data (requires conversion)
```

**Enhanced Decrypt Output:**
```bash
# Decrypt with verbose format information
./target/release/trustedge-core --decrypt --input data.trst --out output --verbose

# Example output for files:
# ● Input Type: File
#   MIME Type: application/json
# ✔ Output: Original file format preserved
# ✔ Decrypt complete. Wrote 1337 bytes.

# Example output for audio:
# ♪ Input Type: Audio (44.1kHz, mono)
# ⚠ Output: Raw PCM data (requires conversion)
# ✔ Decrypt complete. Wrote 441000 bytes.
```

**PCM Format Specifications (Live Audio Only):**
- **Data Type**: 32-bit floating-point samples (`f32le`)
- **Byte Order**: Little-endian
- **Range**: [-1.0, 1.0] normalized audio samples
- **No Headers**: Pure sample data without WAV/MP3 headers

**Converting Raw PCM to Playable Formats:**

```bash
# For live audio captures only - convert mono 44.1kHz PCM to WAV
ffmpeg -f f32le -ar 44100 -ac 1 -i audio.raw audio.wav

# For live audio captures only - convert stereo 48kHz PCM to WAV  
ffmpeg -f f32le -ar 48000 -ac 2 -i audio.raw audio.wav

# For live audio captures only - convert to MP3 (compressed)
ffmpeg -f f32le -ar 44100 -ac 1 -i audio.raw -c:a libmp3lame -b:a 128k audio.mp3

# For live audio captures only - play directly without conversion (Linux with sox)
play -t f32 -r 44100 -c 1 audio.raw
```

**Extract Audio Metadata from .trst (Live Audio Only):**
Live audio captures store the original audio parameters in the encrypted envelope:
```bash
./target/release/trustedge-core --decrypt --input audio.trst --out audio.raw --key-hex $KEY --verbose
# Output shows: Sample Rate: 44100Hz, Channels: 1, Format: f32
```

---

## Network Operations

TrustEdge supports secure client-server operations with mutual authentication and robust connection handling.

### Basic Network Usage

**Start an authenticated server:**
```bash
./target/release/trustedge-server \
  --require-auth \
  --listen 127.0.0.1:8080 \
  --verbose \
  --decrypt \
  --key-hex $(openssl rand -hex 32)
```

**Connect authenticated client:**
```bash
./target/release/trustedge-client \
  --enable-auth \
  --server 127.0.0.1:8080 \
  --file document.txt \
  --verbose \
  --key-hex $(openssl rand -hex 32)
```

### Server Options

| Option | Description | Example |
|--------|-------------|---------|
| `-l, --listen <ADDRESS>` | Address to listen on [default: 127.0.0.1:8080] | `--listen 0.0.0.0:9001` |
| `-o, --output-dir <DIR>` | Directory to save received chunks | `--output-dir ./received` |
| `--decrypt` | Decrypt received chunks and save plaintext | `--decrypt` |

### Client Options  

| Option | Description | Example |
|--------|-------------|---------|
| `-s, --server <ADDRESS>` | Server address to connect to [default: 127.0.0.1:8080] | `--server 192.168.1.100:8080` |
| `-f, --file <FILE>` | File to send (will be processed into chunks) | `--file document.pdf` |
| `--test-chunks <COUNT>` | Send synthetic encrypted chunks instead of real file | `--test-chunks 5` |
| `--chunk-size <SIZE>` | Chunk size for file processing [default: 4096] | `--chunk-size 8192` |

---

## Authentication

TrustEdge implements **Ed25519 mutual authentication** with automatic certificate management.

### Authentication Workflow

1. **Server generates certificate** (if --server-key not provided)
2. **Client generates certificate** (if --client-cert not provided)  
3. **Mutual challenge-response authentication** using Ed25519 signatures
4. **Session established** with configurable timeout
5. **Encrypted data transfer** over authenticated connection

### Authentication Examples

**Authenticated Server with Custom Identity:**
```bash
./target/release/trustedge-server \
  --require-auth \
  --server-identity "Production TrustEdge Server v1.0" \
  --listen 0.0.0.0:8080 \
  --verbose \
  --decrypt \
  --use-keyring \
  --salt-hex $(openssl rand -hex 16)
```

**Authenticated Client with Existing Certificates:**
```bash
./target/release/trustedge-client \
  --enable-auth \
  --client-cert ./client.cert \
  --server-cert ./server.cert \
  --server 192.168.1.100:8080 \
  --file sensitive-document.pdf \
  --verbose
```

**Development Mode with Auto-Generated Certificates:**
```bash
# Server (generates server certificate automatically)
./target/release/trustedge-server --require-auth --verbose

# Client (generates client certificate automatically) 
./target/release/trustedge-client \
  --enable-auth \
  --client-identity "Development Client" \
  --server-cert "TrustEdge Server_server.cert" \
  --file test.txt
```

### Authentication Options

| Option | Description | Example |
|--------|-------------|---------|
| **Server Options** | | |
| `--require-auth` | Enable mutual authentication (server) | `--require-auth` |
| `--server-identity <ID>` | Server identity for certificate generation [default: "TrustEdge Server"] | `--server-identity "Production Server"` |
| `--server-key <PATH>` | Path to server signing key file (auto-generates if not found) | `--server-key /opt/server.key` |
| **Client Options** | | |
| `--enable-auth` | Enable authentication with server certificate verification (client) | `--enable-auth` |
| `--client-cert <PATH>` | Path to client certificate file for authentication | `--client-cert ~/.config/client.cert` |
| `--client-identity <ID>` | Client identity for certificate generation [default: "TrustEdge Client"] | `--client-identity "Mobile App v1.2"` |
| `--server-cert <PATH>` | Path to server certificate file (for authentication) | `--server-cert /etc/trustedge/server.cert` |

### Connection Management Options

| Option | Description | Example |
|--------|-------------|---------|
| `--connect-timeout <SECONDS>` | Connection establishment timeout [default: 10] | `--connect-timeout 15` |
| `--retry-attempts <COUNT>` | Number of connection retry attempts [default: 3] | `--retry-attempts 5` |
| `--retry-delay <SECONDS>` | Delay between retry attempts [default: 2] | `--retry-delay 3` |

### Certificate Management

**Automatic Certificate Generation:**
- Server: Creates `{server-identity}_server.cert` and `{server-identity}_server.key`
- Client: Creates `{client-identity}_client.cert` and `{client-identity}_client.key`

**Certificate File Naming Examples:**
```bash
# Server with default identity "TrustEdge Server"  
# Creates: "TrustEdge Server_server.cert" and "TrustEdge Server_server.key"

# Client with custom identity "Mobile App v1.2"
# Creates: "Mobile App v1.2_client.cert" and "Mobile App v1.2_client.key"

# Custom server identity "Production Server"
# Creates: "Production Server_server.cert" and "Production Server_server.key"
```

**Using Existing Certificates:**
```bash
# Server with existing certificate
./target/release/trustedge-server \
  --require-auth \
  --server-key /etc/ssl/trustedge/production.key

# Client with existing certificate  
./target/release/trustedge-client \
  --enable-auth \
  --client-cert ~/.config/trustedge/mobile.cert \
  --server-cert /etc/ssl/trustedge/production_server.cert
```

**📖 For complete authentication documentation including certificate management, security considerations, and deployment examples, see [AUTHENTICATION_GUIDE.md](AUTHENTICATION_GUIDE.md).**

---

## Universal Backend Registry

### List Available Backends with Capabilities

```bash
$ trustedge-core --list-backends
Universal Backend Registry:

📊 Registry Status:
  ✓ 3 backends registered
  ✓ Capability-based operation routing enabled
  ✓ Auto-fallback configured

🔧 Available Backends:

  ✓ keyring (KeyringBackend)
    Capabilities: [KeyDerivation, SecureStorage]
    Priority: Normal
    Config: iterations=100000, secure_enclave=true

  ✓ universal_keyring (UniversalKeyringBackend) 
    Capabilities: [KeyDerivation, Hashing, SecureStorage]
    Priority: High
    Config: pbkdf2_iterations=150000, argon2_memory=65536

  ✓ universal_registry (UniversalRegistryBackend)
    Capabilities: [OperationRouting, BackendSelection, Fallback]
    Priority: System
    Config: auto_fallback=true, performance_monitoring=enabled

💡 Usage Examples:
  Basic keyring:     --use-keyring --salt-hex <salt>
  Universal routing: --backend-preference encryption:universal_keyring
  Show flow:         --show-operation-flow
```

### Capability-Based Operation Examples

#### Automatic Backend Selection

```bash
# System automatically selects best backend for each operation
$ trustedge-core \
    --input document.txt \
    --out roundtrip.txt \
    --envelope encrypted.trst \
    --use-keyring \
    --salt-hex "abcdef1234567890abcdef1234567890" \
    --show-operation-flow

Operation Flow:
  🔐 Key Derivation → universal_keyring (PBKDF2 + Argon2)
  💾 Secure Storage → keyring (OS native)
  🔄 Registry Management → universal_registry (routing)
```

#### Manual Backend Preferences

```bash
# Set specific backend preferences for operations
$ trustedge-core \
    --input sensitive.pdf \
    --backend-preference "encryption:universal_keyring" \
    --backend-preference "storage:keyring" \
    --backend-config "pbkdf2_iterations=200000" \
    --use-keyring

# Show detailed backend information
$ trustedge-core --backend-info universal_keyring
Backend: universal_keyring (UniversalKeyringBackend)

🎯 Capabilities:
  ✓ KeyDerivation - PBKDF2 + Argon2 hybrid
  ✓ Hashing - SHA-256, SHA-512, BLAKE3
  ✓ SecureStorage - Memory-safe key handling

⚙️  Configuration Options:
  pbkdf2_iterations: 150000 (default) | Range: 100000-1000000
  argon2_memory: 65536 (default) | Range: 32768-1048576
  hash_algorithm: SHA256 (default) | Options: SHA256, SHA512, BLAKE3

📈 Performance Characteristics:
  Key derivation: ~150ms (secure profile)
  Memory usage: ~64KB (bounded)
  Platform support: All (cross-platform)
```

#### Registry Management Examples

```bash
# View current registry configuration
$ trustedge-core --backend-info universal_registry
Registry Configuration:

🎛️  Backend Priorities:
  1. universal_keyring (High)    - Advanced crypto operations
  2. keyring (Normal)            - Standard OS integration  
  3. universal_registry (System) - Operation routing

🔄 Operation Routing Rules:
  KeyDerivation → universal_keyring (preferred) → keyring (fallback)
  SecureStorage → keyring (preferred) → universal_keyring (fallback)
  Hashing → universal_keyring (only)

⚡ Performance Monitoring:
  ✓ Operation timing enabled
  ✓ Backend health checks enabled
  ✓ Auto-fallback on errors enabled
```

### Advanced Backend Configuration

#### High-Security Profile

```bash
# Maximum security configuration
$ trustedge-core \
    --input classified.docx \
    --backend-preference "encryption:universal_keyring" \
    --backend-config "pbkdf2_iterations=500000" \
    --backend-config "argon2_memory=1048576" \
    --backend-config "hash_algorithm=BLAKE3" \
    --use-keyring \
    --verbose

Security Profile: Maximum
  🔐 Key Derivation: PBKDF2 (500K iterations) + Argon2 (1MB memory)
  🏷️  Hashing: BLAKE3 (quantum-resistant)
  💾 Storage: OS keyring with secure enclave
  ⏱️  Estimated time: ~2-3 seconds (security vs speed trade-off)
```

#### Performance-Optimized Profile

```bash
# Speed-optimized configuration
$ trustedge-core \
    --input large_dataset.bin \
    --backend-preference "encryption:universal_keyring" \
    --backend-config "pbkdf2_iterations=100000" \
    --backend-config "argon2_memory=32768" \
    --backend-config "hash_algorithm=SHA256" \
    --use-keyring

Performance Profile: Optimized
  🚀 Key Derivation: PBKDF2 (100K iterations) + Argon2 (32KB memory)
  ⚡ Hashing: SHA256 (hardware-accelerated)
  📊 Estimated time: ~50ms (speed-optimized)
```

---

## Error Handling

### Common Error Scenarios

For comprehensive error handling, troubleshooting steps, and solutions, see **[TROUBLESHOOTING.md](TROUBLESHOOTING.md)**.

**Quick Reference - Most Common Issues:**

| Error Message | Cause | Quick Fix |
|---------------|-------|-----------|
| `No such file or directory` | File doesn't exist | Check file path with `ls -la` |
| `Backend capability not available` | Operation not supported by selected backend | Use `--list-backends`, check capabilities |
| `Odd number of digits` | Invalid salt format | Use `openssl rand -hex 16` for valid salt |
| `bad magic` | Wrong file type | Ensure input is a `.trst` file |
| `AES-GCM decrypt/verify failed` | Wrong key/passphrase | Verify key matches encryption key |
| `Connection refused` | Server not running | Start server, check port and address |
| `Session expired` | Authentication timeout | Reconnect with `--require-auth` |

**📖 For detailed diagnosis, solutions, and debug commands, see [TROUBLESHOOTING.md](TROUBLESHOOTING.md).**

---

## Complete Workflows

The CLI supports various encryption and network workflows. For detailed end-to-end examples:

**● See [EXAMPLES.md](EXAMPLES.md) for comprehensive workflows including:**
- Basic file encryption and decryption
- Live audio capture and processing  
- Secure network operations with authentication
- Key management scenarios across different backends
- Integration examples and automation scripts

**● See [AUTHENTICATION_GUIDE.md](AUTHENTICATION_GUIDE.md) for:**
- Complete authentication setup procedures
- Certificate management and security considerations
- Production deployment configurations

**📖 Quick Reference Examples:**

```bash
# Basic file encryption
./target/release/trustedge-core --input file.txt --envelope file.trst --key-out key.hex

# Live audio capture (requires --features audio)
./target/release/trustedge-core --live-capture --envelope audio.trst --max-duration 10

# Format-aware decryption with inspection
./target/release/trustedge-core --input file.trst --inspect --verbose
./target/release/trustedge-core --input file.trst --decrypt --out restored.txt --key-hex $(cat key.hex)
```

For comprehensive step-by-step examples, see the respective documentation files listed above.

---

## Error Handling

Common CLI errors and their solutions:

| Error | Cause | Solution |
|-------|-------|----------|
| `--out is required` | Missing output file in encrypt mode | Add `--out filename` or use `--no-plaintext` |
| `Decrypt mode requires key material` | No key provided for decryption | Add `--key-hex <key>` or `--use-keyring --salt-hex <salt>` |
| `Invalid key length` | Wrong key format | Use 64 hex characters (32 bytes) for `--key-hex` |
| `Invalid salt length` | Wrong salt format | Use 32 hex characters (16 bytes) for `--salt-hex` |

**● For detailed troubleshooting including audio, network, and authentication issues, see [TROUBLESHOOTING.md](TROUBLESHOOTING.md).**

---

## Reference Links

- **[EXAMPLES.md](EXAMPLES.md)** — Complete workflows and real-world usage examples
- **[AUTHENTICATION_GUIDE.md](AUTHENTICATION_GUIDE.md)** — Security setup and credential management  
- **[TROUBLESHOOTING.md](TROUBLESHOOTING.md)** — Debugging and problem resolution
- **[TESTING.md](TESTING.md)** — Testing procedures and validation methods
