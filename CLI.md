<!--
Copyright (c) 2025 John Turner
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/johnzilla/trustedge
-->
# TrustEdge CLI Reference

Complete command-line interface documentation for TrustEdge.

## Table of Contents
- [CLI Options](#cli-options)
- [Backend Management](#backend-management)
- [Error Handling](#error-handling)
- [Complete Workflows](#complete-workflows)
- [Network Operations](#network-operations)

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

### Key Management Options

| Option | Description | Example |
|--------|-------------|---------|
| `--key-hex <KEY_HEX>` | 64 hex chars (32 bytes) AES-256 key | `--key-hex 0123456789abcdef...` |
| `--key-out <KEY_OUT>` | Where to store generated key (encrypt mode) as hex | `--key-out mykey.hex` |
| `--set-passphrase <PASSPHRASE>` | Store passphrase in OS keyring (one-time setup) | `--set-passphrase "my_secure_passphrase"` |
| `--salt-hex <SALT_HEX>` | Salt for key derivation (32 hex chars = 16 bytes) | `--salt-hex "abcdef1234567890abcdef1234567890"` |
| `--use-keyring` | Use key derived from keyring passphrase + salt instead of --key-hex | `--use-keyring` |

### Backend Options

| Option | Description | Example |
|--------|-------------|---------|
| `--backend <BACKEND>` | Key management backend to use (keyring, tpm, hsm, matter) [default: keyring] | `--backend keyring` |
| `--list-backends` | List available key management backends | `--list-backends` |
| `--backend-config <CONFIG>` | Backend-specific configuration (format: key=value) | `--backend-config "iterations=150000"` |

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

**Note**: Audio features require building with `--features audio`. Install audio system dependencies first:
- **Linux**: `sudo apt-get install libasound2-dev pkg-config`
- **macOS**: Included with Xcode/Command Line Tools
- **Windows**: Included with Windows SDK

### Connection Management Options (Client)

| Option | Description | Example |
|--------|-------------|---------|
| `--connect-timeout <SECONDS>` | Connection establishment timeout [default: 10] | `--connect-timeout 15` |
| `--retry-attempts <COUNT>` | Number of connection retry attempts [default: 3] | `--retry-attempts 5` |
| `--retry-delay <SECONDS>` | Delay between retry attempts [default: 2] | `--retry-delay 3` |

### Network Options

| Option | Description | Example |
|--------|-------------|---------|
| `--server <ADDRESS>` | Server address for network mode (client) | `--server 127.0.0.1:8080` |
| `--listen <ADDRESS>` | Address to listen on (server mode) | `--listen 127.0.0.1:8080` |
| `--output-dir <DIR>` | Directory to save decrypted chunks (server mode) | `--output-dir ./chunks` |

### Authentication Options

| Option | Description | Example |
|--------|-------------|---------|
| `--require-auth` | Enable mutual authentication (server/client) | `--require-auth` |
| `--server-identity <ID>` | Server identity for certificate generation | `--server-identity "Production Server"` |
| `--client-identity <ID>` | Client identity for certificate generation | `--client-identity "Mobile App v1.2"` |

**📖 For complete authentication documentation including all options, security considerations, and deployment examples, see [AUTHENTICATION_GUIDE.md](AUTHENTICATION_GUIDE.md).**

---

## Backend Management

### List Available Backends

```bash
$ trustedge-audio --list-backends
Available key management backends:
  ✓ keyring - OS keyring with PBKDF2 key derivation
    Required config: passphrase, salt

Usage examples:
  --backend keyring --use-keyring --salt-hex <salt>
  --backend tpm --backend-config device_path=/dev/tpm0
  --backend hsm --backend-config pkcs11_lib=/usr/lib/libpkcs11.so
```

### Backend Management Examples

#### Keyring Backend (Default)

```bash
# Set up passphrase (one-time setup)
$ trustedge-audio --set-passphrase "my_secure_passphrase_123"
Passphrase stored in system keyring

# Use keyring with salt for encryption
$ trustedge-audio \
    --input document.txt \
    --out roundtrip.txt \
    --envelope encrypted.trst \
    --backend keyring \
    --salt-hex "abcdef1234567890abcdef1234567890" \
    --use-keyring

# Use specific backend configuration
$ trustedge-audio --backend keyring --backend-config "iterations=150000"
```

#### Future Backends (Planned)

```bash
# TPM 2.0 backend (planned)
$ trustedge-audio --backend tpm --backend-config "device_path=/dev/tpm0"

# HSM backend (planned)  
$ trustedge-audio --backend hsm --backend-config "pkcs11_lib=/usr/lib/libpkcs11.so"

# Matter/Thread ecosystem (planned)
$ trustedge-audio --backend matter --backend-config "device_id=12345"
```

---

## Error Handling

### Common Error Scenarios

#### Unsupported Backend
```bash
$ trustedge-audio --backend tpm
Backend 'tpm' not yet implemented. Available: keyring. Future backends: tpm, hsm, matter. Use --list-backends to see all options
```

#### Missing File
```bash
$ trustedge-audio --decrypt --input nonexistent.trst
Error: open envelope. Caused by: No such file or directory (os error 2)
```

#### Invalid Salt Format
```bash
$ trustedge-audio --salt-hex 'invalid'
Error: salt_hex decode. Caused by: Odd number of digits
```

#### Wrong File Type for Decryption
```bash
$ trustedge-audio --decrypt --input input.mp3 --out output.wav --backend keyring --salt-hex "abcdef1234567890abcdef1234567890"
Error: bad magic
```

#### Wrong Passphrase/Salt Combination
```bash
$ trustedge-audio --decrypt --input test_examples.trst --out output.txt --backend keyring --salt-hex "deadbeefdeadbeefdeadbeefdeadbeef" --use-keyring
Error: AES-GCM decrypt/verify failed
```

---

## Complete Workflows

### Basic File Encryption and Decryption

#### 1. Set up keyring passphrase (one-time setup)
```bash
$ trustedge-audio --set-passphrase "my_secure_passphrase_123" --backend keyring
Passphrase stored in system keyring
```

#### 2. Encrypt a file
```bash
$ echo "Hello TrustEdge!" > document.txt
$ trustedge-audio \
    --input document.txt \
    --out roundtrip.txt \
    --envelope encrypted.trst \
    --backend keyring \
    --salt-hex "abcdef1234567890abcdef1234567890" \
    --use-keyring
Round-trip complete. Read 18 bytes, wrote 18 bytes.
```

#### 3. Decrypt the file
```bash
$ trustedge-audio \
    --decrypt \
    --input encrypted.trst \
    --out decrypted.txt \
    --backend keyring \
    --salt-hex "abcdef1234567890abcdef1234567890" \
    --use-keyring
Decrypt complete. Wrote 18 bytes.
```

#### 4. Verify the content
```bash
$ diff document.txt decrypted.txt
(no output = files are identical)
```

### Live Audio Capture Workflows

#### 1. Real-time Audio Encryption
```bash
# Capture 10 seconds of high-quality audio and encrypt it
$ trustedge-audio \
    --audio-capture \
    --duration 10 \
    --sample-rate 48000 \
    --channels 2 \
    --envelope voice_memo.trst \
    --backend keyring \
    --salt-hex "abcdef1234567890abcdef1234567890" \
    --use-keyring
Audio capture started (48kHz, 2ch)...
Captured 10.0 seconds, encrypted 1920000 bytes
```

#### 2. Decrypt and Restore Audio
```bash
# Decrypt the audio and save as MP3
$ trustedge-audio \
    --decrypt \
    --input voice_memo.trst \
    --out restored_voice.mp3 \
    --backend keyring \
    --salt-hex "abcdef1234567890abcdef1234567890" \
    --use-keyring
Decrypt complete. Wrote 1920000 bytes.
Audio metadata: 48000Hz, 2 channels, f32 format
```

#### 3. Quick Voice Notes with Device Selection
```bash
# List available audio devices
$ trustedge-audio --list-devices
Available audio input devices:
  0: Default (Built-in Microphone)
  1: USB Microphone [Manufacturer]
  2: Line In (External Interface)

# Record from specific device
$ trustedge-audio \
    --audio-capture \
    --device 1 \
    --duration 30 \
    --envelope quick_note.trst \
    --key-out note_key.hex
Using device: USB Microphone [Manufacturer]
Generated AES-256 key: f4e8c2a1...
Captured 30.0 seconds, encrypted 2880000 bytes
```

#### 4. Continuous Recording with Size Limits
```bash
# Record until file reaches ~10MB, then auto-stop
$ trustedge-audio \
    --audio-capture \
    --max-size 10485760 \
    --sample-rate 44100 \
    --channels 1 \
    --envelope interview.trst \
    --backend keyring \
    --use-keyring
Audio capture started (44kHz, 1ch)...
Reached size limit (10.0 MB), stopping capture
Captured 238.1 seconds, encrypted 10485760 bytes
```

### Using Raw Hex Keys

#### Generate and save a key
```bash
$ trustedge-audio \
    --input document.txt \
    --out roundtrip.txt \
    --envelope encrypted.trst \
    --key-out generated_key.hex
Generated AES-256 key: a1b2c3d4e5f6...
Round-trip complete. Read 18 bytes, wrote 18 bytes.
```

#### Use the saved key for decryption
```bash
$ trustedge-audio \
    --decrypt \
    --input encrypted.trst \
    --out decrypted.txt \
    --key-hex $(cat generated_key.hex)
Decrypt complete. Wrote 18 bytes.
```

### Data-Agnostic Encryption Examples

#### Inspect Encrypted File Metadata
```bash
# Check what type of data was encrypted
$ trustedge-audio --inspect voice_memo.trst
TrustEdge Archive Contents:
  Data Type: Audio
  Original Size: 1920000 bytes
  Audio Format: f32
  Sample Rate: 48000 Hz
  Channels: 2
  Encryption: AES-256-GCM
  Created: 2024-01-15 14:30:22 UTC
```

#### Mixed Data Workflows
```bash
# Encrypt various data types with same key
$ KEY="a1b2c3d4e5f6789012345678901234567890abcdef1234567890abcdef123456"

# Encrypt a document
$ trustedge-audio --input report.pdf --envelope report.trst --key-hex $KEY

# Encrypt live audio
$ trustedge-audio --audio-capture --duration 60 --envelope meeting.trst --key-hex $KEY

# Both use same decryption process
$ trustedge-audio --decrypt --input report.trst --out restored_report.pdf --key-hex $KEY
$ trustedge-audio --decrypt --input meeting.trst --out meeting_audio.wav --key-hex $KEY
```

---

## Connection Management & Error Recovery

### Network Resilience Examples

#### Robust Client with Retry Logic
```bash
# For unstable networks - aggressive retry strategy
$ trustedge-client \
    --server remote.example.com:8080 \
    --input large_file.wav \
    --backend keyring \
    --salt-hex "network_salt_abcdef1234567890abcdef" \
    --use-keyring \
    --connect-timeout 15 \
    --retry-attempts 5 \
    --retry-delay 3 \
    --verbose

Connecting to TrustEdge server at remote.example.com:8080
Connection attempt 1 failed: connection refused
Waiting 3s before retry...
Connection attempt 2 of 5
Connection attempt 2 failed: timeout after 15s
Waiting 3s before retry...
Connection attempt 3 of 5
Connected to remote.example.com:8080 on attempt 3
Connected successfully!
```

#### Conservative Settings for Stable Networks
```bash
# Minimal retry for high-reliability environments
$ trustedge-client \
    --server local-server:8080 \
    --input data.txt \
    --key-hex "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef" \
    --connect-timeout 30 \
    --retry-attempts 1
```

#### Server with Graceful Shutdown
```bash
# Start server with connection tracking
$ trustedge-server --listen 0.0.0.0:8080 --verbose --decrypt
[SRV] TrustEdge server listening on 0.0.0.0:8080
[DIR] Output directory: "(none)"
[SEC] Decryption: ENABLED
[CONN] New connection #1 from 10.0.1.50:45678

# Press Ctrl+C for graceful shutdown
^C
[SRV] Shutdown signal received, stopping server...
[SRV] Graceful shutdown initiated...
[SRV] Waiting for 1 active connections to complete...
[OK] Connection #1 completed
[SRV] Server shutdown complete
```

### Large File Processing

#### Process large files with custom chunk size
```bash
$ trustedge-audio \
    --input large_audio.wav \
    --out large_roundtrip.wav \
    --envelope large_encrypted.trst \
    --chunk 8192 \
    --backend keyring \
    --salt-hex "1234567890abcdef1234567890abcdef" \
    --use-keyring
Round-trip complete. Read 10485760 bytes, wrote 10485760 bytes.
```

#### Encrypt without writing plaintext (envelope only)
```bash
$ trustedge-audio \
    --input sensitive_data.bin \
    --envelope secure.trst \
    --no-plaintext \
    --backend keyring \
    --salt-hex "fedcba0987654321fedcba0987654321" \
    --use-keyring
Envelope created. Input processed but plaintext not written.
```

---

## Network Operations

### Server Mode

#### Start a decrypting server
```bash
$ trustedge-audio \
    --port 8080 \
    --decrypt \
    --use-keyring \
    --salt-hex "networkkey1234567890abcdef1234" \
    --output-dir ./received_chunks
Server listening on 0.0.0.0:8080
Waiting for encrypted chunks...
```

### Client Mode

#### Send encrypted data to server
```bash
$ trustedge-client \
    --server 127.0.0.1:8080 \
    --input audio_chunk.wav \
    --use-keyring \
    --salt-hex "networkkey1234567890abcdef1234"
Connecting to TrustEdge server at 127.0.0.1:8080
Connected successfully!
Sent chunk 1/1 (4096 bytes)
```

### Authenticated Network Operations

#### Secure Server with Authentication
```bash
# Start server requiring mutual authentication
$ trustedge-server \
    --listen 127.0.0.1:8080 \
    --require-auth \
    --server-identity "Production TrustEdge Server" \
    --decrypt \
    --use-keyring \
    --salt-hex "networkkey1234567890abcdef1234" \
    --output-dir ./received_chunks \
    --verbose

🔧 Authentication enabled - generating server certificates...
✅ Server identity certificate created
🚀 TrustEdge Server starting with authentication...
🔐 Listening on 127.0.0.1:8080 (authenticated connections only)
⏱️  Session timeout: 300 seconds
📁 Output directory: ./received_chunks
🔍 Waiting for authenticated clients...
```

#### Authenticated Client Connection
```bash
# Connect with mutual authentication
$ trustedge-client \
    --server 127.0.0.1:8080 \
    --input sensitive_data.wav \
    --require-auth \
    --client-identity "Mobile App v1.2.3" \
    --use-keyring \
    --salt-hex "networkkey1234567890abcdef1234" \
    --verbose

🔧 Authentication enabled - generating client certificates...
✅ Client identity certificate created
🔐 Connecting to authenticated server at 127.0.0.1:8080...
🤝 Performing mutual authentication handshake...
✅ Server authenticated successfully
✅ Client authentication completed
🆔 Session ID: 0x7f9a2e8b1c4d3f6a
📤 Sending encrypted data...
✅ Transfer completed successfully
```

#### Authentication Failure Scenarios
```bash
# Server rejects unauthenticated clients
$ trustedge-client \
    --server 127.0.0.1:8080 \
    --input data.wav

❌ Error: Server requires authentication but client not configured for auth
💡 Add --require-auth and --client-identity to connect to authenticated servers

# Client certificate invalid
$ trustedge-client \
    --server 127.0.0.1:8080 \
    --require-auth \
    --client-identity "Invalid Client"

❌ Error: Authentication failed - client certificate rejected by server
💡 Check client certificate and server trust configuration

# Session timeout
$ trustedge-client --server 127.0.0.1:8080 --require-auth --client-identity "Client"
# ... wait 5+ minutes without activity ...

❌ Error: Session expired - please reconnect
💡 Sessions expire after 300 seconds of inactivity by default
```

---

## Key Management

### Passphrase Management
- `--set-passphrase`: Store a passphrase in the system keyring (run once).
- `--use-keyring`: Use the keyring passphrase for key derivation (PBKDF2). **Mutually exclusive** with `--key-hex`.
- `--salt-hex`: 32-char hex salt for PBKDF2 key derivation (required with `--use-keyring`, must be 16 bytes).

### Key Derivation
- In decrypt mode, you must provide either `--key-hex` or `--use-keyring` (random key is not allowed).
- In encrypt mode, if neither is provided, a random key is generated and optionally saved with `--key-out`.
- **PBKDF2 parameters:** SHA-256, 100,000 iterations, 16-byte (32 hex char) salt.

### Security Notes
- Keys are zeroized after use
- Passphrases are stored securely in OS keyring
- Salt must be consistent between encrypt/decrypt operations
- Different salts produce different keys from the same passphrase

---

## Advanced Configuration

### Backend-Specific Configuration

#### Keyring Backend
```bash
# Custom iteration count
--backend-config "iterations=200000"

# Multiple parameters (future)
--backend-config "iterations=150000,timeout=30"
```

#### TPM Backend (Planned)
```bash
# Specify TPM device
--backend-config "device_path=/dev/tpm0"

# Use specific key handle
--backend-config "key_handle=0x81000001"
```

#### HSM Backend (Planned)
```bash
# PKCS#11 library path
--backend-config "pkcs11_lib=/usr/lib/softhsm/libsofthsm2.so"

# Slot and pin
--backend-config "slot=0,pin=1234"
```

---

For more technical details about the underlying protocol and formats, see [PROTOCOL.md](./PROTOCOL.md).

For examples and use cases, see [EXAMPLES.md](./EXAMPLES.md).
