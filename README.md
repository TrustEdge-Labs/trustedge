# TrustEdge — Trustable Edge AI (Rust)

> Not another CRUD app. Learning Rust through **Trustable Edge AI** — privacy-preserving edge pipelines.

This repo is my public learning journey in Rust. Phase 1 is a tiny demo that
reads an audio file in chunks, **encrypts each chunk with AES-256-GCM**, then
**decrypts and verifies** it locally. No networking yet — just the crypto &
streaming skeleton that future phases will build on.

- Language: Rust (stable)
- Crypto: `aes-gcm` (AEAD), 256-bit keys, 96-bit nonces
- Goal of Phase 1: a clean, verifiable round-trip on real audio bytes

---

## Quick start

```bash
# Install Rust (if needed)
# https://rustup.rs

# Clone
git clone git@github.com:johnzilla/trustedge.git
cd trustedge/trustedge-audio   # if this demo lives in a subcrate; else just `cd trustedge`

# Build
cargo build --release

# Run (any audio file is fine; it's treated as opaque bytes)
./target/release/trustedge-audio \
  -i ./sample.wav \
  -o ./roundtrip.wav \
  --chunk 8192

# Verify byte-for-byte round trip
sha256sum ./sample.wav ./roundtrip.wav
# hashes should match
