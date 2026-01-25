<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->

# YubiKey Verification Guide

Step-by-step guide to verify YubiKey integration with TrustEdge.

## Prerequisites

1. **YubiKey Manager** - Install if not already present:
   ```bash
   # Ubuntu/Debian
   sudo apt install yubikey-manager
   
   # macOS
   brew install ykman
   ```

2. **OpenSC PKCS#11 Module** - Required for TrustEdge:
   ```bash
   # Ubuntu/Debian
   sudo apt install opensc-pkcs11
   
   # macOS
   brew install opensc
   ```

3. **YubiKey with PIV applet** - Any modern YubiKey (4, 5 series)

## Step 1: Verify YubiKey Detection

```bash
# Check if ykman can see your YubiKey
ykman list

# Get detailed PIV information
ykman piv info
```

**Expected output**: Should show your YubiKey model and PIV slots.

## Step 2: Check Slot 9c Contents

```bash
# View what's in slot 9c (Key Management)
ykman piv keys export 9c /tmp/pubkey_9c.pem

# Or check all slots
ykman piv info
```

**What you're looking for**: 
- If slot 9c shows "Key present: Yes" → You have a key
- If it shows certificate info → Even better, you have a full cert

## Step 3: Run TrustEdge YubiKey Example

The simplest working example to test:

```bash
cd /path/to/trustedge

# Run the basic YubiKey demo
cargo run --example yubikey_demo --features yubikey
```

**What it does**:
- Scans PIV slots 9a, 9c, 9d, 9e for keys
- Extracts public keys from hardware
- Reports which slots have keys

### If the demo fails:

**Error: "Failed to load PKCS#11 module"**
```bash
# Find your PKCS#11 module location
find /usr/lib -name "opensc-pkcs11.so" 2>/dev/null
find /usr/local/lib -name "opensc-pkcs11.so" 2>/dev/null

# Common locations:
# Ubuntu: /usr/lib/x86_64-linux-gnu/opensc-pkcs11.so
# macOS: /usr/local/lib/opensc-pkcs11.so
```

**Error: "PIN required"**
- Default YubiKey PIN is `123456`
- If you changed it, update the example code or set `YubiKeyConfig.pin`

**Error: "No keys found"**
- You need to generate a key first (see Step 4)

## Step 4: Generate a Test Key (If Slot 9c is Empty)

```bash
# Generate an ECDSA P-256 key in slot 9c
ykman piv keys generate 9c /tmp/pubkey_9c.pem --algorithm ECCP256

# Generate a self-signed certificate for the key
ykman piv certificates generate 9c /tmp/pubkey_9c.pem --subject "CN=TrustEdge Test"

# Verify it's there
ykman piv info
```

**NOTE**: The default PIN for YubiKey is `123456`. You'll be prompted if needed.

## Step 5: Test with Custom Configuration

Create a test file `test_yubikey.rs`:

```rust
use anyhow::Result;
use trustedge_core::{
    backends::yubikey::{YubiKeyBackend, YubiKeyConfig},
    CryptoOperation, CryptoResult, UniversalBackend,
};

fn main() -> Result<()> {
    // Configure with YOUR system's PKCS#11 path
    let config = YubiKeyConfig {
        pkcs11_module_path: "/usr/lib/x86_64-linux-gnu/opensc-pkcs11.so".to_string(),
        pin: Some("123456".to_string()), // Change if you set a custom PIN
        slot: Some("9c".to_string()),    // Target slot 9c specifically
        verbose: true,                   // Show detailed output
    };

    println!("Connecting to YubiKey...");
    let backend = YubiKeyBackend::with_config(config)?;
    
    println!("Getting public key from slot 9c...");
    match backend.perform_operation("9c", CryptoOperation::GetPublicKey) {
        Ok(CryptoResult::PublicKey(pubkey)) => {
            println!("✔ Success! Public key extracted:");
            println!("  Length: {} bytes", pubkey.len());
            println!("  First 32 bytes: {:02x?}", &pubkey[..32.min(pubkey.len())]);
        }
        Ok(other) => println!("✖ Unexpected result: {:?}", other),
        Err(e) => println!("✖ Error: {}", e),
    }
    
    Ok(())
}
```

Save this as `examples/test_my_yubikey.rs` and run:
```bash
cargo run --example test_my_yubikey --features yubikey
```

## Common Issues

### Issue: "PKCS#11 module not found"

**Solution**: Update the path in the config to match your system:
```bash
# Find the correct path
locate opensc-pkcs11.so
# or
dpkg -L opensc-pkcs11 | grep "\.so$"
```

### Issue: "Invalid PIN" or "Authentication failed"

**Solutions**:
1. Check if you changed the default PIN (default is `123456`)
2. Try without PIN first: `pin: None` in config
3. Reset PIN if locked:
   ```bash
   ykman piv access change-pin
   ```

### Issue: "No keys found in any slots"

**Solution**: Generate a test key:
```bash
ykman piv keys generate 9c /tmp/test_pubkey.pem --algorithm ECCP256
ykman piv certificates generate 9c /tmp/test_pubkey.pem --subject "CN=Test"
```

### Issue: Example code has wrong flags like `--list-slots`

**Solution**: The examples are **library code**, not CLI commands. Run them with:
```bash
cargo run --example yubikey_demo --features yubikey
```

NOT as a CLI with arguments like `--list-slots`.

## Quick Verification Command

Run this one-liner to test everything:

```bash
cd /path/to/trustedge && \
  ykman piv info && \
  echo "---" && \
  cargo run --example yubikey_demo --features yubikey 2>&1 | head -50
```

This will:
1. Show your YubiKey PIV status
2. Run the TrustEdge demo
3. Display first 50 lines of output

## Success Criteria

✅ **YubiKey working correctly if you see:**
```
● TrustEdge YubiKey Integration Demo
===================================
...
✔ YubiKey backend initialized successfully

🔍 Scanning PIV Slots for Keys:
   9a (PIV Authentication): ✔ Key found (91 bytes)
   9c (Key Management): ✔ Key found (91 bytes)
   ...
```

## Next Steps

Once basic detection works:
1. Try `yubikey_certificate_demo` - Hardware certificate generation
2. Try `yubikey_hardware_signing_demo` - Real hardware signing
3. Try `yubikey_quic_hardware_demo` - QUIC with YubiKey certs

## Need Help?

If you're still stuck, provide:
1. Output of `ykman piv info`
2. Your PKCS#11 module path: `find /usr -name opensc-pkcs11.so 2>/dev/null`
3. Full error output from the example
