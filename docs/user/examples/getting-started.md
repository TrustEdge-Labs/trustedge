<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->

# Getting Started Examples

Basic examples to get you started with TrustEdge encryption and decryption.

## Simple File Encryption

**Basic file encryption with random key:**
```bash
# Encrypt a document
./target/release/trustedge-core \
  --input document.txt \
  --envelope document.trst \
  --key-out mykey.hex

# Decrypt the document
./target/release/trustedge-core \
  --decrypt \
  --input document.trst \
  --out recovered.txt \
  --key-hex $(cat mykey.hex) \
  --verbose

# Example verbose output:
# ● Input Type: File
#   MIME Type: text/plain
# ✔ Output: Original file format preserved
# ✔ Decrypt complete. Wrote 1337 bytes.

# Verify integrity
diff document.txt recovered.txt  # Should be identical
```

**Keyring-based encryption (password-derived keys):**
```bash
# One-time setup: store passphrase in OS keyring
./target/release/trustedge-core --set-passphrase "my secure passphrase"

# Encrypt using keyring-derived key
./target/release/trustedge-core \
  --input file.txt \
  --envelope file.trst \
  --use-keyring \
  --salt-hex $(openssl rand -hex 16)

# Decrypt using keyring (you'll be prompted for passphrase if needed)
./target/release/trustedge-core \
  --decrypt \
  --input file.trst \
  --out recovered.txt \
  --use-keyring \
  --salt-hex <same-salt-as-encryption>
```

## Format-Aware Operations

**Inspect encrypted data without decrypting:**
```bash
# Create sample data
echo '{"message": "Hello TrustEdge!", "timestamp": 1234567890}' > data.json

# Encrypt the JSON file
./target/release/trustedge-core --input data.json --envelope data.trst --key-out key.hex

# Inspect without decrypting
./target/release/trustedge-core --input data.trst --inspect --verbose

# Example output:
# TrustEdge Archive Information:
#   File: data.trst
#   Data Type: File
#   MIME Type: application/json
#   Original Size: 58 bytes
#   Chunks: 1
#   Output Behavior: Original file format preserved
```

**Format-aware decryption:**
```bash
# Decrypt preserves original format
./target/release/trustedge-core \
  --decrypt \
  --input data.trst \
  --out recovered.json \
  --key-hex $(cat key.hex)

# Verify JSON structure is preserved
cat recovered.json | jq .  # Pretty-print JSON
```

## Live Audio Capture Examples

**Basic audio capture:**
```bash
# List available audio devices
./target/release/trustedge-core --list-audio-devices

# Example output:
# Available Audio Devices:
#   0: Default Input Device
#   1: Built-in Microphone
#   2: USB Audio Device

# Capture 10 seconds of audio
./target/release/trustedge-core \
  --live-capture \
  --envelope voice_note.trst \
  --key-out voice_key.hex \
  --max-duration 10

# Decrypt captured audio (produces raw PCM data)
./target/release/trustedge-core \
  --decrypt \
  --input voice_note.trst \
  --out recovered_audio.raw \
  --key-hex $(cat voice_key.hex)

# Convert to playable WAV file (requires ffmpeg)
ffmpeg -f f32le -ar 44100 -ac 1 -i recovered_audio.raw recovered_audio.wav
```

**Advanced audio capture with specific device and quality:**
```bash
# High-quality stereo capture from specific device
./target/release/trustedge-core \
  --live-capture \
  --audio-device "hw:CARD=USB_AUDIO,DEV=0" \
  --sample-rate 48000 \
  --channels 2 \
  --envelope stereo_voice.trst \
  --use-keyring \
  --max-duration 30

# The captured audio maintains format information
./target/release/trustedge-core --input stereo_voice.trst --inspect

# Example output:
# TrustEdge Archive Information:
#   File: stereo_voice.trst
#   Data Type: Audio
#   Sample Rate: 48000 Hz
#   Channels: 2 (Stereo)
#   Duration: ~30 seconds
#   Output Behavior: Raw PCM data (requires conversion)
```

## Network Mode Quick Start

**Authenticated server setup:**
```bash
# Start server with authentication required
./target/release/trustedge-server \
  --listen 127.0.0.1:8080 \
  --require-auth \
  --decrypt \
  --verbose

# Server will generate certificates automatically and display connection info
```

**Authenticated client connection:**
```bash
# Connect client with authentication
./target/release/trustedge-client \
  --server 127.0.0.1:8080 \
  --input file.txt \
  --require-auth \
  --verbose

# Client will perform mutual authentication and transfer the file securely
```

---

[← Back to Examples Index](README.md)

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