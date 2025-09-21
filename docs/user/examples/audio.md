<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->

# Audio Examples

Live audio capture, processing, and streaming examples for TrustEdge.

## Live Audio Capture

**Basic audio capture:**
```bash
# List available audio devices
./target/release/trustedge-core --list-audio-devices

# Capture 10 seconds of audio
./target/release/trustedge-core \
  --live-capture \
  --envelope voice_note.trst \
  --key-out voice_key.hex \
  --max-duration 10
```

## Advanced Live Audio Capture

### Voice Memo Recording

```bash
# Quick voice note with system keyring
./target/release/trustedge-core \
  --audio-capture \
  --duration 30 \
  --envelope voice_note_$(date +%Y%m%d_%H%M%S).trst \
  --backend keyring \
  --salt-hex "voice_notes_salt_1234567890abcdef" \
  --use-keyring
```

### High-Quality Recording Session

```bash
# Professional audio recording with device selection
./target/release/trustedge-core --list-devices

# Record from professional interface
./target/release/trustedge-core \
  --audio-capture \
  --device 1 \
  --duration 1800 \
  --sample-rate 48000 \
  --channels 2 \
  --envelope studio_session.trst \
  --key-out session_key.hex \
  --verbose
```

## Audio Pipeline Examples

### Audio Device Discovery and Setup

#### Discovering Available Audio Devices
```bash
# List all available audio input devices
./target/release/trustedge-core --list-audio-devices --verbose

# Cross-platform device discovery
./target/release/trustedge-core \
  --audio-device "Microphone (Realtek Audio)" \
  --sample-rate 44100 \
  --channels 1 \
  --envelope test_audio.trst \
  --key-out test.key
```

### Real-time Audio Chunking

### Audio Streaming Simulation

### Audio Troubleshooting Examples

#### Testing Audio Device Access
```bash
# Test minimal audio capture
./target/release/trustedge-core \
  --audio-capture \
  --duration 1 \
  --envelope audio_test.trst \
  --key-out test.key \
  --verbose
```

### Audio Post-Processing and Format Conversion

**Converting Raw PCM to Standard Formats (Live Audio Only):**
```bash
# Convert raw PCM to WAV
ffmpeg -f f32le -ar 44100 -ac 1 -i recovered_audio.raw recovered_audio.wav

# Convert to MP3
ffmpeg -f f32le -ar 44100 -ac 1 -i recovered_audio.raw -b:a 128k recovered_audio.mp3
```

### Cross-Platform Audio Workflows

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
