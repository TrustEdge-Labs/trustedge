<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->

# YubiKey Hardware Integration Guide

TrustEdge supports hardware-backed cryptographic operations via YubiKey PIV. This guide covers setup, the hardware signing demo, and integration testing. For general usage, see the [root README](../README.md).

---

## Prerequisites

- YubiKey 5 series with PIV applet enabled
- [ykman](https://developers.yubico.com/yubikey-manager/) CLI (YubiKey Manager)
- PCSC daemon (`pcscd`) running on your system

---

## Hardware Signing Demo

TrustEdge's flagship capability: real cryptographic operations backed by YubiKey hardware.
This showcases hardware-backed signing, key extraction from PIV slots, X.509 certificate
generation, and certificate validation — all using your physical security key.

**[Watch the demo (2 min)](https://asciinema.org/a/aMaUEmOfw42TNYdXwAgtefcsy)**

### Step 1: Generate a key on YubiKey

```bash
ykman piv keys generate 9a /tmp/pubkey.pem --algorithm ECCP256
ykman piv certificates generate 9a /tmp/pubkey.pem --subject "CN=Test"
```

### Step 2: Run the hardware integration tests

```bash
git clone https://github.com/trustedge-labs/trustedge.git
cd trustedge
cargo test --features yubikey --test yubikey_integration
```

---

## What Happens

TrustEdge connects to your YubiKey via PCSC, extracts the public key from PIV slot 9a,
performs a hardware-backed ECDSA P-256 signature, generates a complete X.509 certificate
signed by the YubiKey via rcgen, and validates the certificate chain.

**Security properties of hardware-backed operations:**
- Private key never leaves the YubiKey hardware
- All signing operations performed on-device via PIV applet
- ECDSA P-256 and RSA-2048 supported
- X.509 certificate generation with hardware-backed signing

---

## Integration Tests

The hardware integration test suite covers 9 test scenarios:

```bash
# Run all YubiKey hardware integration tests (requires physical device)
cargo test --features yubikey --test yubikey_integration
```

Tests cover:
- PIV slot key extraction and public key verification
- Hardware-backed ECDSA P-256 signing
- X.509 certificate generation via rcgen with YubiKey signing
- Certificate chain validation
- Error handling for missing or locked devices

**Note:** These tests require a physical YubiKey 5 series device connected via USB. The test
suite also includes 18 YubiKey simulation tests (no hardware required) that run as part of
the standard `cargo test -p trustedge-core --lib` suite.

---

## Asciinema Demo

Full walkthrough recording of the hardware signing demo:

```
https://asciinema.org/a/aMaUEmOfw42TNYdXwAgtefcsy
```

---

## No YubiKey?

If you do not have a YubiKey, the Software HSM backend provides equivalent functionality
for development and testing. See the [root README](../README.md) for the software-only
archive demo using the `.trst` format.
