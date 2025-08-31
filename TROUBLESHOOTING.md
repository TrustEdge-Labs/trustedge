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
