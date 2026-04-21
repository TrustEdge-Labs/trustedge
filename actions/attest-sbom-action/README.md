# Sealedge SBOM Attestation Action

> **Renamed from `TrustEdge-Labs/attest-sbom-action`.** This repo was renamed in v6.0 to match the Sealedge product name. `@v1` stays frozen as the pre-rebrand behavior; `@v2+` uses Sealedge binary/URL naming. GitHub's built-in 301 redirect covers existing `uses: TrustEdge-Labs/attest-sbom-action@v1` references.

> Attest a binary artifact with its CycloneDX SBOM using Sealedge — one YAML line, cryptographic proof.

## Usage

### Example 1: Ephemeral key (recommended for CI)

The action generates a fresh Ed25519 keypair on every run — nothing is stored. The public
key is embedded in the attestation file, so verification is self-contained.

```yaml
- name: Generate SBOM
  uses: anchore/sbom-action@v0
  with:
    path: ./target/release/my-app
    output-file: sbom.cdx.json
    upload-artifact: false

- name: Attest SBOM
  id: attest
  uses: TrustEdge-Labs/sealedge-attest-sbom-action@v2
  with:
    binary: ./target/release/my-app
    sbom: sbom.cdx.json
```

### Example 2: Persistent key (stored as GitHub Secret)

Store your signing key as a GitHub Actions secret to maintain a stable device identity
across builds — useful when you want attestations traceable to the same device over time.

```yaml
- name: Restore signing key
  run: echo "${{ secrets.SEALEDGE_KEY }}" | base64 -d > build.key

- name: Generate SBOM
  uses: anchore/sbom-action@v0
  with:
    path: ./target/release/my-app
    output-file: sbom.cdx.json
    upload-artifact: false

- name: Attest SBOM
  id: attest
  uses: TrustEdge-Labs/sealedge-attest-sbom-action@v2
  with:
    binary: ./target/release/my-app
    sbom: sbom.cdx.json
    key: ./build.key
    seal-version: 'v6.0.0'

- name: Upload attestation
  env:
    GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
  run: |
    gh release upload "${{ github.ref_name }}" \
      "${{ steps.attest.outputs.attestation-path }}" \
      --clobber
```

To generate a key for the `SEALEDGE_KEY` secret, run locally:

```bash
seal keygen --out-key build.key --out-pub build.pub --unencrypted
base64 -w0 build.key   # paste this as SEALEDGE_KEY secret
```

## What you get

The action writes a `.se-attestation.json` file to `$RUNNER_TEMP` and exposes its path
via `steps.<id>.outputs.attestation-path`. This file is a local cryptographic proof — no
network calls are made. To get a signed receipt from the Sealedge platform, POST the
file to your platform instance (optional follow-on step).

## Inputs

| Input | Required | Default | Description |
|-------|----------|---------|-------------|
| `binary` | yes | — | Path to the binary artifact to attest |
| `sbom` | yes | — | Path to CycloneDX JSON SBOM file |
| `key` | no | `''` | Path to Ed25519 device key file. Generates an ephemeral keypair when not provided. |
| `seal-version` | no | `'latest'` | Sealedge release version to download (e.g., `v6.0.0`). |

## Outputs

| Output | Description |
|--------|-------------|
| `attestation-path` | Absolute path to the generated `.se-attestation.json` file |

## How it works

1. Downloads the `seal` binary from [TrustEdge-Labs/sealedge releases](https://github.com/TrustEdge-Labs/sealedge/releases) and verifies its SHA256 checksum (skips verification with a warning if no checksum file is present in the release).
2. Generates an ephemeral Ed25519 keypair (unless you provide a persistent `key`).
3. Runs `seal attest-sbom` to create a cryptographically signed attestation that binds:
   - The binary artifact (via BLAKE3 hash)
   - The CycloneDX SBOM (via BLAKE3 hash)
   - Ed25519 signature over the attestation payload
4. Writes the attestation to `$RUNNER_TEMP/<binary-name>.se-attestation.json`.

## Verification

Verify an attestation locally:

```bash
seal verify-attestation my-app.se-attestation.json \
  --device-pub "ed25519:..." \
  --binary ./my-app \
  --sbom sbom.cdx.json
```

Or submit to the public Sealedge verifier:

```bash
curl -X POST https://verify.sealedge.dev/v1/verify-attestation \
  -H "Content-Type: application/json" \
  -d @my-app.se-attestation.json
```

## Links

- [Sealedge repository](https://github.com/TrustEdge-Labs/sealedge)
- [Sealedge public verifier](https://verify.sealedge.dev)
- [CycloneDX SBOM specification](https://cyclonedx.org/)
- [anchore/sbom-action](https://github.com/anchore/sbom-action)

## License

Mozilla Public License 2.0 — see [LICENSE](LICENSE).
