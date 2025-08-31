<!--
Copyright (c) 2025 John Turner
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/johnzilla/trustedge
-->
# TrustEdge CL# Extract Audio Metadata from .trst (Live Audio Only):**
Live audio captures store the original audio parameters in the encrypted envelope:
```bash
./target/release/trustedge-audio --decrypt --input audio.trst --out audio.raw --key-hex $KEY --verbose
# Output shows: Sample Rate: 44100Hz, Channels: 1, Format: f32
```

**Format-Aware Capabilities:**
- **Automatic MIME detection**: Recognizes 30+ file types including documents, images, audio, video
- **Format preservation**: File inputs maintain original format perfectly
- **Audio-aware output**: Live audio provides format-specific guidance and metadata
- **Inspection tools**: View format information without decryption using `--inspect`nce

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

#### Audio Device Selection and Formatting

**To discover available devices, always run first:**
```bash
./target/release/trustedge-audio --list-audio-devices
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

**🔧 For detailed audio troubleshooting, device configuration, and system-specific setup, see [TESTING.md](TESTING.md#audio-system-testing).**

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
./target/release/trustedge-audio --input data.trst --inspect --verbose

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
./target/release/trustedge-audio --decrypt --input data.trst --out output --verbose

# Example output for files:
# 📄 Input Type: File
# 📋 MIME Type: application/json
# ✅ Output: Original file format preserved
# ✅ Decrypt complete. Wrote 1337 bytes.

# Example output for audio:
# 🎵 Input Type: Audio (44.1kHz, mono)
# ⚠️  Output: Raw PCM data (requires conversion)
# ✅ Decrypt complete. Wrote 441000 bytes.
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
./target/release/trustedge-audio --decrypt --input audio.trst --out audio.raw --key-hex $KEY --verbose
# Output shows: Sample Rate: 44100Hz, Channels: 1, Format: f32
```

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
| `--server-key <PATH>` | Custom server certificate path | `--server-key /opt/trustedge/server.key` |
| `--client-key <PATH>` | Custom client certificate path | `--client-key ~/.config/app/client.key` |
| `--session-timeout <SECONDS>` | Session timeout in seconds [default: 300] | `--session-timeout 600` |

#### Default Credential Storage Locations

**Server Certificates (Generated Automatically):**
```bash
# Default location (current working directory)
./trustedge-server.key     # Private key file  
./trustedge-server.cert    # Public certificate file

# Custom location
--server-key /opt/trustedge/production.key
# Creates: production.key and production.cert
```

**Client Certificates (Generated Automatically):**
```bash
# Default location (current working directory)  
./trustedge-client.key     # Private key file
./trustedge-client.cert    # Public certificate file

# Custom location
--client-key ~/.config/trustedge/mobile.key  
# Creates: mobile.key and mobile.cert
```

**📖 For complete authentication documentation including certificate management, security considerations, and deployment examples, see [AUTHENTICATION_GUIDE.md](AUTHENTICATION_GUIDE.md).**

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

For comprehensive error handling, troubleshooting steps, and solutions, see **[TROUBLESHOOTING.md](TROUBLESHOOTING.md)**.

**Quick Reference - Most Common Issues:**

| Error Message | Cause | Quick Fix |
|---------------|-------|-----------|
| `No such file or directory` | File doesn't exist | Check file path with `ls -la` |
| `Backend 'X' not yet implemented` | Unsupported backend | Use `--list-backends`, try `--backend keyring` |
| `Odd number of digits` | Invalid salt format | Use `openssl rand -hex 16` for valid salt |
| `bad magic` | Wrong file type | Ensure input is a `.trst` file |
| `AES-GCM decrypt/verify failed` | Wrong key/passphrase | Verify key matches encryption key |
| `Connection refused` | Server not running | Start server, check port and address |
| `Session expired` | Authentication timeout | Reconnect with `--require-auth` |

**📖 For detailed diagnosis, solutions, and debug commands, see [TROUBLESHOOTING.md](TROUBLESHOOTING.md).**

---

## Complete Workflows

The CLI supports various encryption and network workflows. For detailed end-to-end examples:

**📋 See [EXAMPLES.md](EXAMPLES.md) for comprehensive workflows including:**
- Basic file encryption and decryption
- Live audio capture and processing  
- Secure network operations with authentication
- Key management scenarios across different backends
- Integration examples and automation scripts

**🔐 See [AUTHENTICATION_GUIDE.md](AUTHENTICATION_GUIDE.md) for:**
- Complete authentication setup procedures
- Certificate management and security considerations
- Production deployment configurations

**📖 Quick Reference Examples:**

```bash
# Basic file encryption
./target/release/trustedge-audio --input file.txt --envelope file.trst --key-out key.hex

# Live audio capture (requires --features audio)
./target/release/trustedge-audio --live-capture --envelope audio.trst --max-duration 10

# Format-aware decryption with inspection
./target/release/trustedge-audio --input file.trst --inspect --verbose
./target/release/trustedge-audio --input file.trst --decrypt --out restored.txt --key-hex $(cat key.hex)
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

**🔧 For detailed troubleshooting including audio, network, and authentication issues, see [TROUBLESHOOTING.md](TROUBLESHOOTING.md).**

---

## Reference Links

- **[EXAMPLES.md](EXAMPLES.md)** — Complete workflows and real-world usage examples
- **[AUTHENTICATION_GUIDE.md](AUTHENTICATION_GUIDE.md)** — Security setup and credential management  
- **[TROUBLESHOOTING.md](TROUBLESHOOTING.md)** — Debugging and problem resolution
- **[TESTING.md](TESTING.md)** — Testing procedures and validation methods
