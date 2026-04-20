<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: sealedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/sealedge
-->

# YubiKey Hardware Integration Guide

Sealedge supports hardware-backed cryptographic operations via YubiKey PIV. This guide covers setup, the hardware signing demo, and integration testing. For general usage, see the [root README](../README.md).

---

## Prerequisites

- YubiKey 5 series with PIV applet enabled
- [ykman](https://developers.yubico.com/yubikey-manager/) CLI (YubiKey Manager)
- PCSC daemon (`pcscd`) running on your system

---

## Hardware Signing Demo

Sealedge's flagship capability: real cryptographic operations backed by YubiKey hardware.
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
git clone https://github.com/sealedge-labs/sealedge.git
cd sealedge
cargo test --features yubikey --test yubikey_integration
```

### Step 3: Sign a .seal archive with YubiKey

```bash
# Generate a software key for encryption (YubiKey handles signing only)
cargo run -p sealedge-seal-cli -- keygen --out-key device.key --out-pub device.pub

# Wrap and sign with YubiKey (ECDSA P-256 via PIV slot 9c)
cargo run -p sealedge-seal-cli --features yubikey -- wrap \
  --backend yubikey --in data.bin --out archive.seal --device-key device.key

# Verify the hardware-signed archive
cargo run -p sealedge-seal-cli -- verify archive.seal --device-pub "ecdsa-p256:..."
```

The CLI prompts for your YubiKey PIN interactively. The `--device-key` is used for chunk encryption (HKDF key derivation); YubiKey handles the ECDSA P-256 manifest signature.

---

## What Happens

Sealedge connects to your YubiKey via PCSC, extracts the public key from PIV slot 9a,
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
the standard `cargo test -p sealedge-core --lib` suite.

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
archive demo using the `.seal` format.
