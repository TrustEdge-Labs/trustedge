# TrustEdge — Trustable Edge AI (Rust)

> Not another CRUD app. Learning Rust through **Trustable Edge AI** — privacy-preserving edge pipelines.

---

## Why this project?

Most people learning Rust start with CRUD web apps. I’m taking a different route that
aligns with my background in IoT product development, security/PKI and edge systems:

* **Privacy by design**: encrypt at the edge, not just TLS in transit
* **Rust at the edge**: safety + performance for streaming workloads
* **Learning in public**: small, honest milestones → real, reviewable code

If you’re into Rust, IoT, ML at the edge, or security and have ideas or
suggestions, I’d love your feedback.

TL;DR: this repo is my public learning journey in Rust. The first step was initial Rust environment setup on my Linux laptop. Then testing which led to this tiny demo that
reads an audio file in chunks, **encrypts each chunk with AES-256-GCM**, then
**decrypts and verifies** it locally. No networking yet — just the crypto &
streaming skeleton that future work and phases will build on. More to come!

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
````

**Heads-up:** A matching hash doesn’t “prove” encryption occurred — it proves the **encrypt→decrypt** pipeline is lossless. The code actually performs AES-GCM per chunk and immediately verifies the tag before writing plaintext out.

---

## What’s here (Phase 1)

* `src/main.rs` — minimal CLI:

  * chunked file read
  * per-chunk AES-256-GCM with robust AAD
  * signed manifest per chunk (Ed25519, bincode-encoded)
  * immediate decrypt, signature verify, and integrity check
* `Cargo.toml` — `aes-gcm`, `anyhow`, `clap`, `blake3`, `zeroize`

### How it works

- Reads the input file in user-defined chunks.
- For each chunk:
  - Constructs a unique nonce: 4-byte random prefix + 8-byte counter.
  - Builds AAD (Additional Authenticated Data) using a helper function: includes a BLAKE3 hash of the file header, chunk sequence, nonce, and a hash of the signed manifest.
  - For each chunk, creates a signed manifest (Ed25519 signature and public key stored as bytes) with provenance and integrity info.
  - Encrypts the chunk with AES-256-GCM and the AAD.
  - Immediately verifies the manifest signature, re-derives AAD, decrypts, and checks plaintext integrity.
  - Writes the verified plaintext to the output file.
- For round-trip testing, the output file does **not** include a header, so its hash matches the input. In future versions, a header will be written for real encrypted file formats.

### What is AAD?

AAD (Additional Authenticated Data) is extra data that is authenticated (integrity-checked) but not encrypted. In this project, AAD binds each chunk to the file/session context and the signed manifest, preventing tampering and replay.

### What is a manifest?

Each chunk includes a signed manifest (bincode-encoded struct) containing:
- Manifest version, timestamp, sequence number
- Hash of the file header and plaintext chunk
- AI/model provenance fields (placeholders)
- Ed25519 signature and public key (as bytes)
This allows for strong provenance, integrity, and future extensibility.
### Next small steps (in order)

* [ ] Write header and manifest+ct to output for a real encrypted file format
* [ ] Add a decrypt/verify mode to the CLI
* [ ] Document the file format (header, manifest, chunk) in the README
* [ ] Add tests for serialization, AAD, and round-trip
* [ ] (Optional) Add logging for chunk/manifest info

---

## License

This project is licensed under the **Mozilla Public License 2.0 (MPL-2.0)**.
See [`LICENSE`](./LICENSE) for details.

**Disclaimer:** This project is developed independently, on personal time and equipment, and is **not affiliated with or endorsed by my employer**.
