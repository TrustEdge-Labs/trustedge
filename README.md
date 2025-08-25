# TrustEdge — Trustable Edge AI (Rust)

> Not another CRUD app. Learning Rust through **Trustable Edge AI** — privacy-preserving edge pipelines.

---

## Why this project?

Most people learning Rust start with CRUD web apps. I’m taking a different route that
aligns with my background in IoT product development, security/PKI and edge systems:

* **Privacy by design**: encrypt at the edge, not just TLS in transit
* **Rust at the edge**: safety + performance for streaming workloads
* **Learning in public**: small, honest milestones → real, reviewable code

**TrustEdge** is a Rust prototype for privacy-preserving, provenance-aware edge audio.

- **Private by default:** audio chunks are encrypted with AES-256-GCM before leaving the device.
- **Provenance by design:** each chunk carries a signed manifest (C2PA-inspired) whose hash is bound into AEAD AAD; tampering breaks decryption.
- **Streaming-friendly:** fixed nonce discipline (prefix||counter) and per-chunk records.

**Non-goals (for now):**

- No key management (KMS/TPM) or device identity lifecycle.
- Not C2PA compliant yet (just “C2PA-inspired”).
- Not production crypto config (demo keys; no rotation or revocation).

If you’re into Rust, IoT, ML at the edge, or security and have ideas or
suggestions, I’d love your feedback.


See the included threat model ([`THREAT_MODEL.md`](./THREAT_MODEL.md)) for a detailed breakdown of security goals, risks, and mitigations.

For details on the wire format and network protocol, see [`PROTOCOL.md`](./PROTOCOL.md).

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


# Encrypt and write envelope (with hex key)
./target/release/trustedge-audio \
  --input ./sample.wav \
  --envelope ./sample.trst \
  --key-out ./aeskey.hex

# Decrypt envelope to plaintext (with hex key)
./target/release/trustedge-audio \
  --decrypt \
  --input ./sample.trst \
  --out ./roundtrip.wav \
  --key-hex $(cat ./aeskey.hex)

# Set a passphrase in the system keyring (run once)
./target/release/trustedge-audio --set-passphrase "my secret passphrase"

# Encrypt using keyring-derived key
./target/release/trustedge-audio \
  --input ./sample.wav \
  --envelope ./sample.trst \
  --use-keyring \
  --salt-hex <32-hex-chars>

# Decrypt using keyring-derived key
./target/release/trustedge-audio \
  --decrypt \
  --input ./sample.trst \
  --out ./roundtrip.wav \
  --use-keyring \
  --salt-hex <32-hex-chars>

# Or, for a simple round-trip (no envelope):
./target/release/trustedge-audio \
  --input ./sample.wav \
  --out ./roundtrip.wav \
  --chunk 8192


# Verify byte-for-byte round trip
sha256sum ./sample.wav ./roundtrip.wav
# hashes should match
```

---

## Network Mode Example

If you have the networked client/server binaries (`trustedge-client` and `trustedge-server`), you can transfer encrypted audio chunks over the network:

### 1. Start the server

```bash
./target/release/trustedge-server --listen 127.0.0.1:8080 --output_dir ./received_chunks --key-hex <64-char-hex-key> --decrypt
```

### 2. Run the client

```bash
./target/release/trustedge-client --server 127.0.0.1:8080 --file ./sample.wav --key-hex <64-char-hex-key>
```

* The client reads and encrypts the file in chunks, sending each chunk with a signed manifest and nonce.
* The server receives, validates, and (if `--decrypt` is set) decrypts and saves the plaintext.
* Use the same key for both client and server for successful decryption.

See [`PROTOCOL.md`](./PROTOCOL.md) for protocol details.


**Heads-up:** A matching hash doesn’t “prove” encryption occurred — it proves the **encrypt→decrypt** pipeline is lossless. The code actually performs AES-GCM per chunk and immediately verifies the tag before writing plaintext out.

---



### Documentation

* [`PROTOCOL.md`](./PROTOCOL.md) — Wire format and network protocol for chunk transfer
* [`THREAT_MODEL.md`](./THREAT_MODEL.md) — Security goals, threat analysis, mitigations
* `src/main.rs` — CLI tool: chunked file read, per-chunk AES-256-GCM, signed manifest, envelope output, decrypt/verify mode, key management
* `Cargo.toml` — all crypto and serialization dependencies

### CLI options

| Flag               | Description                                                      | Mode(s)           |
|--------------------|------------------------------------------------------------------|-------------------|
| `--input`          | Input file (audio or any bytes)                                  | Both              |
| `--out`            | Output file (decrypted/plaintext)                                | Both              |
| `--chunk`          | Chunk size in bytes (default: 4096)                              | Both              |
| `--envelope`       | Write envelope file (.trst) with header + records                | Envelope          |
| `--no-plaintext`   | Skip writing round-tripped plaintext                             | Both              |
| `--decrypt`        | Decrypt envelope to plaintext                                    | Envelope/Decrypt  |
| `--key-hex`        | 64-char hex AES-256 key (for encrypt/decrypt)                    | Both              |
| `--key-out`        | Save generated key to file (encrypt mode)                        | Envelope/Encrypt  |
| `--set-passphrase` | Store a passphrase in the system keyring (run once)              | Key management    |
| `--use-keyring`    | Use keyring passphrase for key derivation (PBKDF2)               | Both              |
| `--salt-hex`       | 32-char hex salt for PBKDF2 key derivation (with keyring)        | Both              |

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


### Envelope file format and integrity

The `.trst` envelope file is a binary format containing:

- **StreamHeader**: version, header bytes (58 bytes), header hash (BLAKE3)
- **Record(s)**: sequence number, nonce, signed manifest (with Ed25519 signature), ciphertext (AES-GCM)

All fields are bincode-encoded for compactness and speed.

**Nonce Prefix Integrity:**
Each record's nonce prefix (first 4 bytes of the 12-byte nonce) must match the stream header's nonce prefix. This is strictly enforced during decryption and helps prevent record tampering or mixing between streams. If any validation fails (e.g., signature, nonce prefix, hash), the record is rejected and an error is reported.



### Key management

- `--key-hex`: Use a user-supplied 64-char hex key for AES-256 (encrypt/decrypt). **Mutually exclusive** with `--use-keyring`.
- `--key-out`: Save the randomly generated key to a file (encrypt mode).
- `--set-passphrase`: Store a passphrase in the system keyring (run once).
- `--use-keyring`: Use the keyring passphrase for key derivation (PBKDF2). **Mutually exclusive** with `--key-hex`.
- `--salt-hex`: 32-char hex salt for PBKDF2 key derivation (required with `--use-keyring`, must be 16 bytes).
- In decrypt mode, you must provide either `--key-hex` or `--use-keyring` (random key is not allowed).
- In encrypt mode, if neither is provided, a random key is generated and optionally saved with `--key-out`.
- **PBKDF2 parameters:** SHA-256, 100,000 iterations, 16-byte (32 hex char) salt.
### Error handling

If any validation fails during decryption (e.g., manifest signature, nonce prefix, header hash, or plaintext hash), the record is rejected and an error is reported or logged. This ensures that tampered or out-of-sequence records cannot be decrypted or accepted.

---

**Protocol Versioning:**
The protocol is versioned (see StreamHeader and file preamble). Future changes will increment the version and document compatibility requirements.



### Next steps

* [x] Write header and manifest+ct to output for a real encrypted file format
* [x] Add a decrypt/verify mode to the CLI
* [x] Document the file format (header, manifest, chunk) in the README
* [x] Add passphrase/keyring-based key management and PBKDF2 support
* [ ] Add more tests for serialization, AAD, and round-trip
* [ ] (Optional) Add logging for chunk/manifest info

---

## Security and Threat Model

For a detailed analysis of security goals, threat actors, attack vectors, mitigations, and future roadmap, see [`THREAT_MODEL.md`](./THREAT_MODEL.md).

- Covers network, application, cryptographic, side-channel, and physical threats
- Describes current and planned mitigations
- Outlines security requirements and risk assessment
- Documents ongoing and future security work

**If you are reviewing, deploying, or contributing to TrustEdge, please read the threat model for context on security assumptions and limitations.**

---

## License

This project is licensed under the **Mozilla Public License 2.0 (MPL-2.0)**.
See [`LICENSE`](./LICENSE) for details.

**Disclaimer:** This project is developed independently, on personal time and equipment, and is **not affiliated with or endorsed by my employer**.
