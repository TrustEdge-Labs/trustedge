<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->

# Backend Examples

Universal Backend system and hardware integration examples.

## Universal Backend Workflows

### Software HSM Backend

```bash
# Use software HSM for key generation
./target/release/trustedge-core \
  --input document.txt \
  --envelope document.trst \
  --backend software-hsm \
  --key-out generated.key
```

### Keyring Backend

```bash
# Store passphrase in OS keyring
./target/release/trustedge-core --set-passphrase "my secure passphrase"

# Use keyring-derived keys
./target/release/trustedge-core \
  --input file.txt \
  --envelope file.trst \
  --backend keyring \
  --salt-hex $(openssl rand -hex 16)
```

## Hardware Backend Demonstrations

### YubiKey Examples (Library-Based)

YubiKey functionality is accessed through **Rust examples**, not CLI flags:

```bash
# Verify YubiKey connectivity (auto-detects OpenSC)
cargo run --example verify_yubikey --features yubikey

# Verify with custom PIN
cargo run --example verify_yubikey_custom_pin --features yubikey -- YOUR_PIN

# Full YubiKey integration demo
cargo run --example yubikey_demo --features yubikey

# Hardware certificate generation
cargo run --example yubikey_certificate_demo --features yubikey

# Hardware signing operations
cargo run --example yubikey_hardware_signing_demo --features yubikey

# QUIC with hardware-backed certificates
cargo run --example yubikey_quic_hardware_demo --features yubikey
```

**Note**: YubiKey operations require:
- YubiKey with PIV applet
- OpenSC PKCS#11 module: `sudo apt install opensc-pkcs11`
- See `YUBIKEY_VERIFICATION.md` for setup guide

### YubiKey Integration with CLI

The CLI supports YubiKey through the backend system:

```bash
# List available backends (includes yubikey if compiled with feature)
./target/release/trustedge-core --list-backends

# Use YubiKey backend for encryption (requires backend implementation)
./target/release/trustedge-core \
  --input sensitive.pdf \
  --envelope sensitive.trst \
  --backend yubikey \
  --backend-config "pin=YOUR_PIN" \
  --backend-config "slot=9c"
```

**Current Status**: YubiKey examples are fully functional, CLI integration is in development.

### Cross-Backend Compatibility

```bash
# Generate with software HSM
./target/release/trustedge-core \
  --input data.txt \
  --envelope data.trst \
  --backend software-hsm \
  --key-out sw-key.hex

# Decrypt with keyring (after importing key)
./target/release/trustedge-core \
  --decrypt \
  --input data.trst \
  --out recovered.txt \
  --backend keyring \
  --key-hex $(cat sw-key.hex)
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
