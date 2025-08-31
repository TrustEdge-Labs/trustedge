<!--
Copyright (c) 2025 John Turner
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/johnzilla/trustedge
-->

# TrustEdge Troubleshooting Guide

Comprehensive error handling, common issues, and diagnostic procedures for TrustEdge.

## Table of Contents
- [Common Error Messages](#common-error-messages)
- [Configuration Issues](#configuration-issues)
- [Network Problems](#network-problems)
- [Authentication Issues](#authentication-issues)
- [Audio System Issues](#audio-system-issues)
- [Cryptographic Errors](#cryptographic-errors)
- [File and Format Issues](#file-and-format-issues)
- [Debug and Diagnostic Commands](#debug-and-diagnostic-commands)

---

## Common Error Messages

### File System Errors

#### `No such file or directory (os error 2)`
**Error Example:**
```
Error: open envelope. Caused by: No such file or directory (os error 2)
```

**Cause:** Input file doesn't exist or path is incorrect.

**Solution:**
```bash
# Check file exists
ls -la your_file.trst

# Use absolute path if needed
./target/release/trustedge-client --input /full/path/to/file.trst
```

---

## Configuration Issues

### Backend Configuration

#### `Backend 'tpm' not yet implemented`
**Error Example:**
```
Backend 'tpm' not yet implemented. Available: keyring. Future backends: tpm, hsm, matter.
```

**Solution:**
```bash
# List available backends
./target/release/trustedge-audio --list-backends

# Use supported backend
./target/release/trustedge-audio --backend keyring
```

### Salt Format Issues

#### `Odd number of digits`
**Error Example:**
```
Error: salt_hex decode. Caused by: Odd number of digits
```

**Cause:** Salt hex string has odd number of characters (must be even).

**Solution:**
```bash
# Wrong: 15 characters
--salt-hex "abcdef1234567890abc"

# Correct: 32 characters (16 bytes)
--salt-hex "abcdef1234567890abcdef1234567890"

# Generate valid salt
openssl rand -hex 16
```

---

## Network Problems

### Connection Issues

#### `Connection refused`
**Symptoms:**
```
Connection attempt 1 failed: connection refused
```

**Diagnosis:**
1. Check if server is running:
   ```bash
   netstat -tlnp | grep :8080
   ```

2. Verify server address and port:
   ```bash
   # Test connectivity
   telnet 127.0.0.1 8080
   ```

**Solutions:**
```bash
# Start server on correct port
./target/release/trustedge-server --listen 127.0.0.1:8080

# Check firewall rules
sudo ufw status
```

#### `Connection timeout`
**Symptoms:**
```
Connection attempt 2 failed: timeout after 15s
```

**Solutions:**
```bash
# Increase timeout for slow networks
./target/release/trustedge-client \
  --server remote.example.com:8080 \
  --connect-timeout 30 \
  --retry-attempts 3

# Use retry logic for unstable networks
./target/release/trustedge-client \
  --retry-attempts 5 \
  --retry-delay 3
```

### Server Issues

#### Server Startup Problems
**Check server logs with verbose mode:**
```bash
./target/release/trustedge-server \
  --listen 0.0.0.0:8080 \
  --verbose \
  --decrypt
```

**Common server issues:**
- Port already in use: `Address already in use (os error 98)`
- Permission denied: `Permission denied (os error 13)` - try different port > 1024
- Interface binding issues: Use `127.0.0.1` instead of `0.0.0.0`

---

## Authentication Issues

### Authentication Configuration

#### `Server requires authentication but client not configured for auth`
**Error Example:**
```
❌ Error: Server requires authentication but client not configured for auth
```

**Solution:**
```bash
# Add authentication to client
./target/release/trustedge-client \
  --server 127.0.0.1:8080 \
  --input data.wav \
  --require-auth \
  --client-identity "My Client App"
```

#### `Authentication failed - client certificate rejected by server`
**Possible Causes:**
1. **Corrupted certificates**: Delete and regenerate
2. **Clock skew**: Sync system clocks
3. **Wrong identity**: Check client/server identity strings

**Solutions:**
```bash
# Delete corrupted certificates
rm *_identity.cert *.key

# Regenerate with verbose logging
./target/release/trustedge-server \
  --require-auth \
  --verbose \
  --server-identity "Debug Server"

./target/release/trustedge-client \
  --require-auth \
  --verbose \
  --client-identity "Debug Client"
```

#### `Session expired - please reconnect`
**Cause:** Session timeout exceeded (default: 300 seconds).

**Solutions:**
```bash
# Reconnect with fresh authentication
./target/release/trustedge-client --require-auth --client-identity "Client"

# Use longer session timeout for server
./target/release/trustedge-server \
  --require-auth \
  --session-timeout 600  # 10 minutes
```

---

## Audio System Issues

### Audio Device Problems

#### `No audio input devices found`
**Cause:** System audio drivers not available or TrustEdge built without audio features.

**Solutions:**
```bash
# Verify audio features are enabled
./target/release/trustedge-audio --help | grep -i audio

# If missing, rebuild with audio features
cargo build --release --features audio

# Check system audio devices
arecord --list-devices  # Linux
system_profiler SPAudioDataType  # macOS
```

#### `Failed to open audio device: Permission denied`
**Cause:** Insufficient permissions to access audio hardware.

**Solutions:**
```bash
# Linux: Add user to audio group
sudo usermod -a -G audio $USER
# Logout and login required

# Check current groups
groups $USER

# Test with PulseAudio
./target/release/trustedge-audio \
  --live-capture \
  --audio-device "pulse" \
  --max-duration 5
```

#### `Audio device "device_name" not found`
**Cause:** Incorrect device name or device no longer available.

**Solutions:**
```bash
# Always check available devices first
./target/release/trustedge-audio --list-audio-devices

# Copy device name exactly from the list
./target/release/trustedge-audio \
  --live-capture \
  --audio-device "hw:CARD=USB_AUDIO,DEV=0" \
  --max-duration 5

# Use system default as fallback
./target/release/trustedge-audio \
  --live-capture \
  --max-duration 5
```

#### Silent Audio Capture
**Cause:** Microphone muted, wrong input levels, or incorrect device.

**Solutions:**
```bash
# Check microphone levels (Linux)
alsamixer  # Adjust capture levels

# Test with system tools first
arecord -d 3 test_system.wav  # Linux
sox -d test_system.wav trim 0 3  # macOS/Linux

# Use verbose output for debugging
./target/release/trustedge-audio \
  --live-capture \
  --max-duration 5 \
  --verbose

# Try different sample rates
./target/release/trustedge-audio \
  --live-capture \
  --sample-rate 44100 \
  --max-duration 5
```

#### Decrypted Audio Not Playable
**Cause:** Live audio captures output raw PCM data, not playable audio files.

**Important:** TrustEdge decryption behavior varies by input type:
- **File inputs** (MP3, WAV, etc.): Original format preserved
- **Live audio captures** (`--live-capture`): Outputs **raw PCM data** (32-bit float, little-endian)

**Solutions:**
```bash
# For live audio captures: Always use .raw extension for clarity
./target/release/trustedge-audio \
  --decrypt \
  --input live_audio.trst \
  --out audio.raw \
  --key-hex $KEY \
  --verbose

# For live audio captures: Extract audio parameters from verbose output
# Look for: "Sample Rate: 44100Hz, Channels: 2, Format: f32"

# For live audio captures: Convert raw PCM to playable WAV
ffmpeg -f f32le -ar 44100 -ac 2 -i audio.raw audio.wav

# For file inputs: Use original extension
./target/release/trustedge-audio \
  --decrypt \
  --input music_file.trst \
  --out music_file.mp3 \
  --key-hex $KEY
# Output will be playable MP3 file (original format preserved)
```

**📋 For comprehensive audio testing and system configuration, see [TESTING.md](TESTING.md#audio-system-testing).**

---

## Cryptographic Errors

### Decryption Failures

#### `AES-GCM decrypt/verify failed`
**Common Causes:**
1. **Wrong key**: Key doesn't match encryption key
2. **Wrong passphrase/salt**: PBKDF2 derivation mismatch  
3. **File corruption**: Encrypted data has been modified
4. **Format mismatch**: File isn't a valid .trst file

**Diagnostic Steps:**
```bash
# 1. Verify file is valid .trst format
file encrypted.trst
hexdump -C encrypted.trst | head -1
# Should start with magic bytes

# 2. Test with known good key
./target/release/trustedge-audio \
  --decrypt \
  --input encrypted.trst \
  --out test.txt \
  --key-hex "known_good_key_64_hex_chars"

# 3. Test passphrase/salt combination
./target/release/trustedge-audio \
  --decrypt \
  --input encrypted.trst \
  --out test.txt \
  --use-keyring \
  --salt-hex "original_salt_used_for_encryption"
```

#### `bad magic`
**Cause:** File is not a valid TrustEdge envelope format.

**Solutions:**
```bash
# Check file format
file suspicious_file.trst

# Verify file wasn't corrupted
./target/release/trustedge-audio \
  --input original_file.txt \
  --envelope new_envelope.trst \
  --key-hex $(openssl rand -hex 32)
```

---

## File and Format Issues

### Format-Aware Decryption Issues

#### Unknown File Type Detection
**Symptoms:** File shows as `application/octet-stream` instead of expected type

**Diagnosis:**
```bash
# Inspect file format detection
./target/release/trustedge-audio --input file.trst --inspect --verbose

# Check original file extension and content
file original_file.pdf  # Should show PDF document
hexdump -C original_file.pdf | head -2  # Check file headers
```

**Solutions:**
```bash
# For unknown extensions, the file will still decrypt correctly
# but will show as binary data. This is expected behavior.

# To verify correct handling:
./target/release/trustedge-audio --decrypt --input file.trst --out restored_file.pdf --key-hex $KEY
file restored_file.pdf  # Should match original type
diff original_file.pdf restored_file.pdf  # Should be identical
```

#### MIME Type Mismatch
**Symptoms:** Expected MIME type doesn't match detected type

**Common Causes:**
- File extension doesn't match content (e.g., `.txt` file containing JSON)
- Corrupted file headers
- Custom file formats not in MIME database

**Verification:**
```bash
# Check what MIME type was detected
./target/release/trustedge-audio --input file.trst --inspect

# Expected output:
# MIME Type: application/pdf  (for PDF files)
# MIME Type: application/json (for JSON files)
# MIME Type: text/plain      (for text files)
# MIME Type: application/octet-stream (for unknown types)
```

#### Format Inspection Without Decryption
**Use Case:** Verify file type before decryption

```bash
# Inspect encrypted archive
./target/release/trustedge-audio --input suspicious_file.trst --inspect --verbose

# Example output:
# TrustEdge Archive Information:
#   File: suspicious_file.trst
#   Format Version: 1
#   Algorithm: AES-256-GCM
#   Data Type: File
#   MIME Type: application/pdf
#   Output Behavior: Original file format preserved

# This tells you it's a PDF file without decrypting it
```

### Format Validation

#### Header Corruption
**Test for header corruption:**
```bash
# Verify file magic bytes
hexdump -C file.trst | head -1
# Should show expected magic bytes

# Test with known good file
cp known_good.trst test_copy.trst
./target/release/trustedge-audio --decrypt --input test_copy.trst
```

#### Record Tampering Detection
**Symptoms:** Decryption fails partway through file

**Validation Test:**
```bash
# Create test file
echo "test data" > test.txt

# Encrypt
./target/release/trustedge-audio \
  --input test.txt \
  --envelope test.trst \
  --key-hex $(openssl rand -hex 32)

# Verify encryption worked
./target/release/trustedge-audio \
  --decrypt \
  --input test.trst \
  --out recovered.txt \
  --key-hex $(cat last_key.hex)

# Compare files
diff test.txt recovered.txt
```

### Format-Aware Output Verification

#### Audio vs File Confusion
**Symptoms:** Expected audio file but got different output

**Diagnosis:**
```bash
# Check what type of data was originally encrypted
./target/release/trustedge-audio --input file.trst --inspect

# For file inputs (MP3, WAV, etc.):
# Data Type: File
# MIME Type: audio/mpeg (or audio/wav)
# Output Behavior: Original file format preserved

# For live audio capture:
# Data Type: Audio
# Sample Rate: 44100 Hz
# Channels: 1 (mono)
# Output Behavior: Raw PCM data (requires conversion)
```

**Solution:**
```bash
# File inputs preserve format automatically
./target/release/trustedge-audio --decrypt --input music.trst --out music.mp3 --key-hex $KEY
# Output: Playable MP3 file

# Live audio requires conversion
./target/release/trustedge-audio --decrypt --input live_capture.trst --out audio.raw --key-hex $KEY
ffmpeg -f f32le -ar 44100 -ac 1 -i audio.raw audio.wav
```

#### Header Corruption
**Test for header corruption:**
```bash
# Verify file magic bytes
hexdump -C file.trst | head -1
# Should show expected magic bytes

# Test with known good file
cp known_good.trst test_copy.trst
./target/release/trustedge-audio --decrypt --input test_copy.trst
```

#### Record Tampering Detection
**Symptoms:** Decryption fails partway through file

**Validation Test:**
```bash
# Create test file
echo "test data" > test.txt

# Encrypt
./target/release/trustedge-audio \
  --input test.txt \
  --envelope test.trst \
  --key-hex $(openssl rand -hex 32)

# Verify encryption worked
./target/release/trustedge-audio \
  --decrypt \
  --input test.trst \
  --out recovered.txt \
  --key-hex $(cat last_key.hex)

# Compare files
diff test.txt recovered.txt
```

---

## Debug and Diagnostic Commands

### Verbose Logging

Enable verbose output for detailed troubleshooting:

```bash
# Server with debug output
./target/release/trustedge-server \
  --listen 127.0.0.1:8080 \
  --verbose \
  --decrypt

# Client with debug output  
./target/release/trustedge-client \
  --server 127.0.0.1:8080 \
  --input file.txt \
  --verbose

# File encryption/decryption with format details
./target/release/trustedge-audio \
  --decrypt \
  --input file.trst \
  --out restored.txt \
  --key-hex $KEY \
  --verbose

# Example verbose output:
# 📄 Input Type: File
# 📋 MIME Type: application/json
# ✅ Output: Original file format preserved
# ✅ Decrypt complete. Wrote 1337 bytes.
# 📄 Output file preserves original format and should be directly usable.
```

### Format Inspection Commands

```bash
# Quick format check (no decryption)
./target/release/trustedge-audio --input file.trst --inspect

# Detailed format inspection
./target/release/trustedge-audio --input file.trst --inspect --verbose

# Compare multiple files
for file in *.trst; do
  echo "=== $file ==="
  ./target/release/trustedge-audio --input "$file" --inspect
  echo
done
```

# Authentication debug
./target/release/trustedge-server \
  --require-auth \
  --verbose \
  --server-identity "Debug Server"
```

### System Information

Gather system information for bug reports:

```bash
# TrustEdge version
./target/release/trustedge-audio --version

# System information
uname -a
rustc --version

# Network connectivity
netstat -tlnp | grep trustedge
ss -tlnp | grep :8080

# Certificate files
ls -la *_identity.cert *.key

# File permissions
ls -la input_file.txt output_dir/
```

### Test Environment Setup

Create clean test environment:

```bash
# Clean slate for testing
rm -f *.trst *.hex *_identity.cert *.key

# Generate test data
echo "Hello TrustEdge Testing" > test_input.txt

# Test basic encryption/decryption
./target/release/trustedge-audio \
  --input test_input.txt \
  --envelope test.trst \
  --key-out test.key

./target/release/trustedge-audio \
  --decrypt \
  --input test.trst \
  --out test_output.txt \
  --key-hex $(cat test.key)

# Verify round-trip
diff test_input.txt test_output.txt
```

### Network Testing

Test network components in isolation:

```bash
# Test server startup
./target/release/trustedge-server \
  --listen 127.0.0.1:8080 \
  --verbose &
SERVER_PID=$!

# Wait for startup
sleep 2

# Test connection
echo "test" | nc 127.0.0.1 8080

# Clean shutdown
kill $SERVER_PID
```

---

## Getting Help

If issues persist after following this guide:

1. **Check logs**: Always run with `--verbose` for detailed output
2. **Test minimal case**: Use simplest possible command that reproduces issue
3. **Environment**: Note OS, Rust version, and TrustEdge version
4. **Create issue**: Use [GitHub issue templates](https://github.com/johnzilla/trustedge/issues/new/choose)

### Issue Report Template

```markdown
**System Information:**
- OS: [e.g., Ubuntu 22.04]
- Rust version: [e.g., 1.75.0]
- TrustEdge version: [output of --version]

**Command that failed:**
```bash
./target/release/trustedge-client --server 127.0.0.1:8080 --input file.txt
```

**Error output:**
```
[paste complete error message with --verbose]
```

**Expected behavior:**
[what should have happened]

**Additional context:**
[any other relevant information]
```

---

This troubleshooting guide covers the most common TrustEdge issues. For authentication-specific problems, also see [AUTHENTICATION_GUIDE.md](AUTHENTICATION_GUIDE.md#troubleshooting).
