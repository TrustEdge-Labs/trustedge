<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->

# TrustEdge Troubleshooting Guide

Comprehensive error handling, common issues, and diagnostic procedures for TrustEdge.

## Table of Contents
- [Common Error Messages](#common-error-messages)
- [Configuration Issues](#configuration-issues)
- [Universal Backend Issues](#universal-backend-issues)
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

[↑ Back to top](#table-of-contents)

---

## Configuration Issues

### Backend Configuration

#### `Backend capability not available`
**Error Example:**
```
Error: Operation not supported by available backends
Available backends: keyring, universal_keyring
Required capability: AdvancedHashing
```

**Solution:**
```bash
# List available backends with capabilities
trustedge-core --list-backends

# Check specific backend capabilities
trustedge-core --backend-info universal_keyring

# Use backend with required capabilities
trustedge-core --backend-preference "hashing:universal_keyring"
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

[↑ Back to top](#table-of-contents)

---

## Universal Backend Issues

### Registry Initialization Failures

#### `Failed to initialize Universal Backend Registry`

**Error Example:**
```
Error: Backend registry initialization failed
Caused by: universal_registry backend not available
```

**Cause:** The Universal Backend Registry failed to initialize properly, usually due to:
- Missing backend dependencies
- Insufficient system permissions
- Corrupted backend configuration

**Solution:**
```bash
# Check registry status
trustedge-core --list-backends

# If empty or error, verify system dependencies
# Linux: Check keyring service
systemctl status gnome-keyring-daemon
# or
systemctl status kwallet

# Reset registry configuration
rm -rf ~/.config/trustedge/backend_registry.json

# Force registry reinitialization
trustedge-core --backend-info universal_registry
```

#### `Backend registration failed: capability conflict`

**Error Example:**
```
Error: Failed to register backend 'custom_backend'
Caused by: Capability 'KeyDerivation' conflicts with existing backend 'universal_keyring'
```

**Cause:** Multiple backends trying to register the same capability with conflicting configurations.

**Solution:**
```bash
# Check current backend registrations
trustedge-core --list-backends

# Manually specify backend preferences to resolve conflicts
trustedge-core --backend-preference "keyderivation:universal_keyring"

# Or use explicit backend selection
trustedge-core --backend-info universal_keyring
```

#### `Registry corruption detected`

**Error Example:**
```
Error: Backend registry corrupted
Caused by: Invalid registry state - checksum mismatch
```

**Cause:** Registry metadata file corruption, often from:
- Improper shutdown during registry updates
- Filesystem corruption
- Concurrent access conflicts

**Solution:**
```bash
# Backup current registry (if recoverable)
cp ~/.config/trustedge/backend_registry.json ~/.config/trustedge/backend_registry.json.backup

# Remove corrupted registry
rm -rf ~/.config/trustedge/backend_registry.json

# Force clean reinitialization
trustedge-core --list-backends --verbose

# Verify registry health
trustedge-core --backend-info universal_registry
```

### Capability Mismatch Errors

#### `Operation not supported by selected backend`

**Error Example:**
```
Error: KeyDerivation operation failed
Caused by: Backend 'basic_keyring' does not support capability 'AdvancedHashing'
```

**Cause:** Requested operation requires capabilities not available in the selected backend.

**Solution:**
```bash
# Check which backends support the required capability
trustedge-core --list-backends | grep -A 5 "Capabilities.*Hashing"

# Use a backend with the required capability
trustedge-core --backend-preference "hashing:universal_keyring"

# Or let the registry auto-select
trustedge-core --show-operation-flow
```

#### `Capability version mismatch`

**Error Example:**
```
Error: Backend capability version conflict
Caused by: Required KeyDerivation v2.0, but backend provides v1.5
```

**Cause:** Backend provides an older version of the required capability.

**Solution:**
```bash
# Check available capability versions
trustedge-core --backend-info universal_keyring | grep -A 10 "Capabilities"

# Update to a backend with newer capability versions
trustedge-core --backend-preference "keyderivation:universal_keyring"

# Check if system updates are available
# Update TrustEdge to latest version for newest capabilities
```

#### `No backend available for operation`

**Error Example:**
```
Error: Operation dispatch failed
Caused by: No registered backend supports capability 'QuantumResistantHashing'
```

**Cause:** No currently registered backend supports the required capability.

**Solution:**
```bash
# List all available backends and their capabilities
trustedge-core --list-backends

# Check if a fallback capability can be used
trustedge-core --backend-preference "hashing:universal_keyring"

# For future capabilities, check for TrustEdge updates
# Some capabilities may require specific backend plugins
```

### Backend Selection Problems

#### `Backend selection timeout`

**Error Example:**
```
Error: Backend selection failed
Caused by: Registry timeout after 30 seconds
```

**Cause:** Registry taking too long to select an appropriate backend, usually due to:
- Backend health checks timing out
- Network latency for remote backends
- Heavy system load

**Solution:**
```bash
# Check backend health status
trustedge-core --backend-info universal_registry | grep -A 20 "Health Monitoring"

# Manually specify a known-good backend
trustedge-core --backend-preference "keyderivation:keyring"

# Reduce timeout for testing
trustedge-core --backend-config "selection_timeout=10"

# Check system resources
top -p $(pgrep trustedge)
```

#### `Circular backend dependency detected`

**Error Example:**
```
Error: Backend dependency resolution failed
Caused by: Circular dependency: universal_keyring -> keyring -> universal_keyring
```

**Cause:** Backend configuration creates circular dependencies in capability routing.

**Solution:**
```bash
# Check current backend routing configuration
trustedge-core --backend-info universal_registry | grep -A 15 "Operation Routing"

# Reset to default routing preferences
rm -rf ~/.config/trustedge/backend_preferences.json

# Use explicit, non-circular preferences
trustedge-core --backend-preference "keyderivation:universal_keyring"
trustedge-core --backend-preference "storage:keyring"
```

#### `Backend performance degradation`

**Error Example:**
```
Warning: Backend 'universal_keyring' performance below threshold
Average latency: 2.5s (threshold: 1.0s)
Switching to fallback backend 'keyring'
```

**Cause:** Selected backend is performing poorly, triggering automatic failover.

**Diagnostic Steps:**
```bash
# Check detailed performance metrics
trustedge-core --backend-info universal_registry | grep -A 20 "Performance Analysis"

# Monitor real-time performance
trustedge-core --backend-config "performance_monitoring=detailed" --verbose

# Check system resources affecting the backend
# Memory usage
free -h
# CPU usage
top -p $(pgrep trustedge)
# Disk I/O
iostat -x 1 5
```

**Solutions:**
```bash
# Optimize backend configuration for performance
trustedge-core --backend-config "pbkdf2_iterations=100000"  # Reduce iterations
trustedge-core --backend-config "argon2_memory=32768"      # Reduce memory usage

# Use performance-optimized backend preference
trustedge-core --backend-preference "keyderivation:keyring"  # Faster backend

# Increase performance thresholds if acceptable
trustedge-core --backend-config "performance_threshold=2000"  # 2 second threshold
```

### Registry Maintenance and Recovery

#### Emergency Backend Reset

```bash
# Complete registry reset (nuclear option)
echo "Performing complete backend registry reset..."

# 1. Stop any running TrustEdge processes
pkill trustedge

# 2. Backup current configuration
mkdir -p ~/.config/trustedge/backup/$(date +%Y%m%d_%H%M%S)
cp -r ~/.config/trustedge/*.json ~/.config/trustedge/backup/$(date +%Y%m%d_%H%M%S)/ 2>/dev/null || true

# 3. Remove all registry files
rm -rf ~/.config/trustedge/backend_registry.json
rm -rf ~/.config/trustedge/backend_preferences.json
rm -rf ~/.config/trustedge/backend_cache.json

# 4. Reinitialize with defaults
trustedge-core --list-backends

# 5. Verify registry health
trustedge-core --backend-info universal_registry
```

#### Registry Health Check Script

```bash
#!/bin/bash
# Backend Health Check Script

echo "🔍 TrustEdge Backend Health Check"
echo "================================="

# Check registry status
echo "📊 Registry Status:"
if trustedge-core --list-backends >/dev/null 2>&1; then
    echo "  ✅ Registry accessible"
    
    # Count registered backends
    BACKEND_COUNT=$(trustedge-core --list-backends 2>/dev/null | grep -c "✓.*Backend")
    echo "  📊 Backends registered: $BACKEND_COUNT"
    
    # Check each backend health
    echo "🔍 Backend Health:"
    trustedge-core --backend-info universal_registry | grep -A 5 "Backend Health"
    
    # Performance check
    echo "⚡ Performance Check:"
    START_TIME=$(date +%s%N)
    trustedge-core --backend-info keyring >/dev/null 2>&1
    END_TIME=$(date +%s%N)
    DURATION=$((($END_TIME - $START_TIME) / 1000000))  # Convert to milliseconds
    
    if [ $DURATION -lt 1000 ]; then
        echo "  ✅ Backend response time: ${DURATION}ms (healthy)"
    else
        echo "  ⚠️  Backend response time: ${DURATION}ms (slow)"
    fi
    
else
    echo "  ❌ Registry inaccessible - requires attention"
    echo "  💡 Try: trustedge-core --list-backends --verbose"
fi

echo ""
echo "🎯 Quick Fix Commands:"
echo "  Reset registry: rm ~/.config/trustedge/backend_registry.json"
echo "  Reinitialize: trustedge-core --list-backends"
echo "  Health check: trustedge-core --backend-info universal_registry"
```

[↑ Back to top](#table-of-contents)

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

[↑ Back to top](#table-of-contents)

---

## Audio System Issues

### Audio Device Problems

#### `No audio input devices found`
**Cause:** System audio drivers not available or TrustEdge built without audio features.

**Solutions:**
```bash
# Verify audio features are enabled
./target/release/trustedge-core --help | grep -i audio

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
./target/release/trustedge-core \
  --live-capture \
  --audio-device "pulse" \
  --max-duration 5
```

#### `Audio device "device_name" not found`
**Cause:** Incorrect device name or device no longer available.

**Solutions:**
```bash
# Always check available devices first
./target/release/trustedge-core --list-audio-devices

# Copy device name exactly from the list
./target/release/trustedge-core \
  --live-capture \
  --audio-device "hw:CARD=USB_AUDIO,DEV=0" \
  --max-duration 5

# Use system default as fallback
./target/release/trustedge-core \
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
./target/release/trustedge-core \
  --live-capture \
  --max-duration 5 \
  --verbose

# Try different sample rates
./target/release/trustedge-core \
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
./target/release/trustedge-core \
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
./target/release/trustedge-core \
  --decrypt \
  --input music_file.trst \
  --out music_file.mp3 \
  --key-hex $KEY
# Output will be playable MP3 file (original format preserved)
```

**📋 For comprehensive audio testing and system configuration, see [TESTING.md](TESTING.md#audio-system-testing).**

[↑ Back to top](#table-of-contents)

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
./target/release/trustedge-core \
  --decrypt \
  --input encrypted.trst \
  --out test.txt \
  --key-hex "known_good_key_64_hex_chars"

# 3. Test passphrase/salt combination
./target/release/trustedge-core \
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
./target/release/trustedge-core \
  --input original_file.txt \
  --envelope new_envelope.trst \
  --key-hex $(openssl rand -hex 32)
```

[↑ Back to top](#table-of-contents)

---

## File and Format Issues

### Format-Aware Decryption Issues

#### Unknown File Type Detection
**Symptoms:** File shows as `application/octet-stream` instead of expected type

**Diagnosis:**
```bash
# Inspect file format detection
./target/release/trustedge-core --input file.trst --inspect --verbose

# Check original file extension and content
file original_file.pdf  # Should show PDF document
hexdump -C original_file.pdf | head -2  # Check file headers
```

**Solutions:**
```bash
# For unknown extensions, the file will still decrypt correctly
# but will show as binary data. This is expected behavior.

# To verify correct handling:
./target/release/trustedge-core --decrypt --input file.trst --out restored_file.pdf --key-hex $KEY
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
./target/release/trustedge-core --input file.trst --inspect

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
./target/release/trustedge-core --input suspicious_file.trst --inspect --verbose

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
./target/release/trustedge-core --decrypt --input test_copy.trst
```

#### Record Tampering Detection
**Symptoms:** Decryption fails partway through file

**Validation Test:**
```bash
# Create test file
echo "test data" > test.txt

# Encrypt
./target/release/trustedge-core \
  --input test.txt \
  --envelope test.trst \
  --key-hex $(openssl rand -hex 32)

# Verify encryption worked
./target/release/trustedge-core \
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
./target/release/trustedge-core --input file.trst --inspect

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
./target/release/trustedge-core --decrypt --input music.trst --out music.mp3 --key-hex $KEY
# Output: Playable MP3 file

# Live audio requires conversion
./target/release/trustedge-core --decrypt --input live_capture.trst --out audio.raw --key-hex $KEY
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
./target/release/trustedge-core --decrypt --input test_copy.trst
```

#### Record Tampering Detection
**Symptoms:** Decryption fails partway through file

**Validation Test:**
```bash
# Create test file
echo "test data" > test.txt

# Encrypt
./target/release/trustedge-core \
  --input test.txt \
  --envelope test.trst \
  --key-hex $(openssl rand -hex 32)

# Verify encryption worked
./target/release/trustedge-core \
  --decrypt \
  --input test.trst \
  --out recovered.txt \
  --key-hex $(cat last_key.hex)

# Compare files
diff test.txt recovered.txt
```

[↑ Back to top](#table-of-contents)

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
./target/release/trustedge-core \
  --decrypt \
  --input file.trst \
  --out restored.txt \
  --key-hex $KEY \
  --verbose

# Example verbose output:
# ● Input Type: File
#   MIME Type: application/json
# ✔ Output: Original file format preserved
# ✔ Decrypt complete. Wrote 1337 bytes.
# ● Output file preserves original format and should be directly usable.
```

### Format Inspection Commands

```bash
# Quick format check (no decryption)
./target/release/trustedge-core --input file.trst --inspect

# Detailed format inspection
./target/release/trustedge-core --input file.trst --inspect --verbose

# Compare multiple files
for file in *.trst; do
  echo "=== $file ==="
  ./target/release/trustedge-core --input "$file" --inspect
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
./target/release/trustedge-core --version

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
./target/release/trustedge-core \
  --input test_input.txt \
  --envelope test.trst \
  --key-out test.key

./target/release/trustedge-core \
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

[↑ Back to top](#table-of-contents)

---

## Getting Help

If issues persist after following this guide:

1. **Check logs**: Always run with `--verbose` for detailed output
2. **Test minimal case**: Use simplest possible command that reproduces issue
3. **Environment**: Note OS, Rust version, and TrustEdge version
4. **Create issue**: Use [GitHub issue templates](https://github.com/TrustEdge-Labs/trustedge/issues/new/choose)

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

[↑ Back to top](#table-of-contents)

---

This troubleshooting guide covers the most common TrustEdge issues. For authentication-specific problems, also see [AUTHENTICATION_GUIDE.md](authentication.md#troubleshooting).

---

**📖 Links:**
- **[TrustEdge Home](https://github.com/TrustEdge-Labs/trustedge)** - Main repository
- **[Documentation](../README.md)** - Complete docs index
- **[CLI Reference](cli.md)** - Command reference

**⚖️ Legal:**
- **Copyright**: © 2025 TrustEdge Labs LLC
- **License**: Mozilla Public License 2.0 ([MPL-2.0](https://mozilla.org/MPL/2.0/))
- **Commercial**: [Enterprise licensing available](mailto:enterprise@trustedgelabs.com)
