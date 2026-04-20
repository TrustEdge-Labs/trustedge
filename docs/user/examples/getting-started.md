<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: sealedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/sealedge
-->

# Getting Started Examples

Basic examples to get you started with Sealedge encryption and decryption.

> **📦 Workspace Note**: Sealedge is organized as a Cargo workspace. Use `cargo run -p package-name` for development, or `./target/release/binary-name` for installed binaries.

## Simple File Encryption

**Basic file encryption with random key:**
```bash
# Development (from workspace root)
cargo run -p sealedge-core -- \
  --input document.txt \
  --envelope document.seal \
  --key-out mykey.hex

# OR using installed binary
./target/release/sealedge-core \
  --input document.txt \
  --envelope document.seal \
  --key-out mykey.hex

# Decrypt the document
cargo run -p sealedge-core -- \
  --decrypt \
  --input document.seal \
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
cargo run -p sealedge-core -- --set-passphrase "my secure passphrase"

# Encrypt using keyring-derived key
cargo run -p sealedge-core -- \
  --input file.txt \
  --envelope file.seal \
  --use-keyring \
  --salt-hex $(openssl rand -hex 16)

# Decrypt using keyring (you'll be prompted for passphrase if needed)
cargo run -p sealedge-core -- \
  --decrypt \
  --input file.seal \
  --out recovered.txt \
  --use-keyring \
  --salt-hex <same-salt-as-encryption>
```

## Format-Aware Operations

**Inspect encrypted data without decrypting:**
```bash
# Create sample data
echo '{"message": "Hello Sealedge!", "timestamp": 1234567890}' > data.json

# Encrypt the JSON file
cargo run -p sealedge-core -- --input data.json --envelope data.seal --key-out key.hex

# Inspect without decrypting
cargo run -p sealedge-core -- --input data.seal --inspect --verbose

# Example output:
# Sealedge Archive Information:
#   File: data.seal
#   Data Type: File
#   MIME Type: application/json
#   Original Size: 58 bytes
#   Chunks: 1
#   Output Behavior: Original file format preserved
```

**Format-aware decryption:**
```bash
# Decrypt preserves original format
cargo run -p sealedge-core -- \
  --decrypt \
  --input data.seal \
  --out recovered.json \
  --key-hex $(cat key.hex)

# Verify JSON structure is preserved
cat recovered.json | jq .  # Pretty-print JSON
```

## Live Audio Capture Examples

**Basic audio capture:**
```bash
# List available audio devices
./target/release/sealedge-core --list-audio-devices

# Example output:
# Available Audio Devices:
#   0: Default Input Device
#   1: Built-in Microphone
#   2: USB Audio Device

# Capture 10 seconds of audio
./target/release/sealedge-core \
  --live-capture \
  --envelope voice_note.seal \
  --key-out voice_key.hex \
  --max-duration 10

# Decrypt captured audio (produces raw PCM data)
./target/release/sealedge-core \
  --decrypt \
  --input voice_note.seal \
  --out recovered_audio.raw \
  --key-hex $(cat voice_key.hex)

# Convert to playable WAV file (requires ffmpeg)
ffmpeg -f f32le -ar 44100 -ac 1 -i recovered_audio.raw recovered_audio.wav
```

**Advanced audio capture with specific device and quality:**
```bash
# High-quality stereo capture from specific device
./target/release/sealedge-core \
  --live-capture \
  --audio-device "hw:CARD=USB_AUDIO,DEV=0" \
  --sample-rate 48000 \
  --channels 2 \
  --envelope stereo_voice.seal \
  --use-keyring \
  --max-duration 30

# The captured audio maintains format information
./target/release/sealedge-core --input stereo_voice.seal --inspect

# Example output:
# Sealedge Archive Information:
#   File: stereo_voice.seal
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
./target/release/sealedge-server \
  --listen 127.0.0.1:8080 \
  --require-auth \
  --decrypt \
  --verbose

# Server will generate certificates automatically and display connection info
```

**Authenticated client connection:**
```bash
# Connect client with authentication
./target/release/sealedge-client \
  --server 127.0.0.1:8080 \
  --input file.txt \
  --require-auth \
  --verbose

# Client will perform mutual authentication and transfer the file securely
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