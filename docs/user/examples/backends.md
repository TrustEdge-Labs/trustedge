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

### YubiKey PKCS#11 Operations

```bash
# List available YubiKey slots
./target/release/trustedge-core --backend yubikey --list-slots

# Generate key pair in YubiKey slot
./target/release/trustedge-core \
  --backend yubikey \
  --slot 9a \
  --generate-key-pair \
  --pin-prompt

# Sign with YubiKey
./target/release/trustedge-core \
  --input document.txt \
  --envelope document.trst \
  --backend yubikey \
  --slot 9a \
  --pin-prompt
```

### YubiKey Integration Testing

```bash
# Test YubiKey connectivity
./target/release/trustedge-core \
  --backend yubikey \
  --test-connection \
  --verbose

# YubiKey-based encryption workflow
./target/release/trustedge-core \
  --input sensitive.pdf \
  --envelope sensitive.trst \
  --backend yubikey \
  --slot 9c \
  --pin "123456"
```

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
