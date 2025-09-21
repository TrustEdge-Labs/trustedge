<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->

# TrustEdge Installation Guide

Complete installation instructions for TrustEdge with all features.

## Basic Installation

**Prerequisites:**
```bash
# Install Rust (if needed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

**Basic Installation (Core Features Only):**
```bash
# Clone the repository
git clone https://github.com/TrustEdge-Labs/trustedge.git
cd trustedge

# Build core features only (file encryption, basic operations)
cargo build --workspace --release --no-default-features
```

**Full Installation with Audio Support:**
```bash
# Install audio system dependencies
# On Ubuntu/Debian:
sudo apt-get update
sudo apt-get install libasound2-dev pkg-config

# On macOS (via Homebrew):
# Audio libraries included with Xcode/Command Line Tools
# No additional packages needed

# On Windows:
# Audio libraries included with Windows SDK
# No additional packages needed

# Build with audio features
cargo build --package trustedge-core --release --features audio
```

**YubiKey Hardware Support:**
```bash
# Install PKCS#11 module for YubiKey support
# On Ubuntu/Debian:
sudo apt-get install opensc-pkcs11

# On macOS (via Homebrew):
brew install opensc

# On Windows:
# Download and install OpenSC from https://github.com/OpenSC/OpenSC/releases

# Build with YubiKey hardware backend
cargo build --package trustedge-core --release --features yubikey

# Or build with all features
cargo build --workspace --release --features audio,yubikey
```

**Verification:**
```bash
# Verify installation
./target/release/trustedge-core --version
./target/release/trustedge-server --version
./target/release/trustedge-client --version
./target/release/trst --version

# Test basic functionality
echo "Hello TrustEdge!" > test.txt
./target/release/trustedge-core --input test.txt --envelope test.trst --key-out test.key
./target/release/trustedge-core --decrypt --input test.trst --out recovered.txt --key-hex $(cat test.key)
diff test.txt recovered.txt  # Should show no differences
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