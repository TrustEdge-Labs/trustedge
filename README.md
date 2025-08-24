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
cd trustedge/trustedge-audio

# Build
cargo build --release

# Encrypt and write envelope
./target/release/trustedge-audio \
  -i ./sample.wav \
  --envelope ./sample.trst \
  --key-out ./aeskey.hex

# Decrypt envelope to plaintext
./target/release/trustedge-audio \
  --decrypt \
  -i ./sample.trst \
  -o ./roundtrip.wav \
  --key-hex $(cat ./aeskey.hex)

# Or, for a simple round-trip (no envelope):
./target/release/trustedge-audio \
  -i ./sample.wav \
  -o ./roundtrip.wav \
  --chunk 8192

# Verify byte-for-byte round trip
sha256sum ./sample.wav ./roundtrip.wav
# hashes should match
```


**Heads-up:** A matching hash doesn’t “prove” encryption occurred — it proves the **encrypt→decrypt** pipeline is lossless. The code actually performs AES-GCM per chunk and immediately verifies the tag before writing plaintext out.

---


## What’s here

* `src/main.rs` — CLI tool:
  * Chunked file read
  * Per-chunk AES-256-GCM with robust AAD
  * Signed manifest per chunk (Ed25519, bincode-encoded)
  * Envelope output: header + records (for real encrypted file format)
  * Decrypt/verify mode for envelope files
  * Key management: hex input/output, random key generation
* `Cargo.toml` — all crypto and serialization dependencies

### CLI options

| Flag           | Description |
|----------------|-------------|
| `-i, --input`  | Input file (audio or any bytes) |
| `-o, --out`    | Output file (decrypted/plaintext) |
| `--chunk`      | Chunk size in bytes (default: 4096) |
| `--envelope`   | Write envelope file (.trst) with header + records |
| `--no-plaintext` | Skip writing round-tripped plaintext |
| `--decrypt`    | Decrypt envelope to plaintext |
| `--key-hex`    | 64-char hex AES-256 key (for encrypt/decrypt) |
| `--key-out`    | Save generated key to file (encrypt mode) |

### How it works

- Reads the input file in user-defined chunks.
- For each chunk:
  - Constructs a unique nonce: 4-byte random prefix + 8-byte counter.
  - Builds AAD (Additional Authenticated Data): `[header_hash][seq][nonce][manifest_hash]`.
  - Creates a signed manifest (Ed25519 signature and public key as bytes) with provenance and integrity info.
  - Encrypts the chunk with AES-256-GCM and the AAD.
  - Immediately verifies the manifest signature, re-derives AAD, decrypts, and checks plaintext integrity.
  - Writes the verified plaintext to the output file (unless `--no-plaintext`).
  - Optionally writes each record to an envelope file (`--envelope`).
- For round-trip testing, the output file does **not** include a header, so its hash matches the input. Envelope files contain all metadata for real-world use.

### What is AAD?

AAD (Additional Authenticated Data) is extra data that is authenticated (integrity-checked) but not encrypted. Here, AAD binds each chunk to the file/session context and the signed manifest, preventing tampering and replay. Layout: `[header_hash][seq][nonce][manifest_hash]`.

### What is a manifest?

Each chunk includes a signed manifest (bincode-encoded struct) containing:
- Manifest version, timestamp, sequence number
- Hash of the file header and plaintext chunk
- AI/model provenance fields (placeholders)
- Ed25519 signature and public key (as bytes)
This allows for strong provenance, integrity, and future extensibility.

### Envelope file format

The `.trst` envelope file is a binary format containing:
- **StreamHeader**: version, header bytes (58 bytes), header hash (BLAKE3)
- **Record(s)**: sequence number, nonce, signed manifest (with Ed25519 signature), ciphertext (AES-GCM)
All fields are bincode-encoded for compactness and speed.

### Key management

- `--key-hex`: Use a user-supplied 64-char hex key for AES-256 (encrypt/decrypt)
- `--key-out`: Save the randomly generated key to a file (encrypt mode)
- If neither is provided, a random key is generated and printed to stderr (demo only)

### Next steps

* [x] Write header and manifest+ct to output for a real encrypted file format
* [x] Add a decrypt/verify mode to the CLI
* [x] Document the file format (header, manifest, chunk) in the README
* [ ] Add more tests for serialization, AAD, and round-trip
* [ ] (Optional) Add logging for chunk/manifest info

---

## License

This project is licensed under the **Mozilla Public License 2.0 (MPL-2.0)**.
See [`LICENSE`](./LICENSE) for details.

**Disclaimer:** This project is developed independently, on personal time and equipment, and is **not affiliated with or endorsed by my employer**.
