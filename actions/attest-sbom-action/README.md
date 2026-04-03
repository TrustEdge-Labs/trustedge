# TrustEdge SBOM Attestation Action

> Attest a binary artifact with its CycloneDX SBOM using TrustEdge — one YAML line, cryptographic proof.

## Usage

### Minimal example

```yaml
- uses: TrustEdge-Labs/attest-sbom-action@v1
  with:
    binary: ./target/release/my-app
    sbom: sbom.cdx.json
```

This downloads the latest `trst` binary, generates an ephemeral Ed25519 keypair, and
creates a `.te-attestation.json` file that cryptographically links your binary to its SBOM.

### Full example with all inputs

```yaml
- name: Generate SBOM
  run: syft ./target/release/my-app -o cyclonedx-json > sbom.cdx.json

- name: Attest SBOM
  id: attest
  uses: TrustEdge-Labs/attest-sbom-action@v1
  with:
    binary: ./target/release/my-app
    sbom: sbom.cdx.json
    key: ./device.key          # optional: use persistent device key
    trst-version: 'v4.0.0'    # optional: pin to a specific version

- name: Upload attestation as release asset
  env:
    GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
  run: |
    gh release upload "${{ github.ref_name }}" \
      "${{ steps.attest.outputs.attestation-path }}" \
      --clobber
```

### Upload as workflow artifact

```yaml
- uses: actions/upload-artifact@v4
  with:
    name: attestation
    path: ${{ steps.attest.outputs.attestation-path }}
```

## Inputs

| Input | Required | Default | Description |
|-------|----------|---------|-------------|
| `binary` | yes | — | Path to the binary artifact to attest |
| `sbom` | yes | — | Path to CycloneDX JSON SBOM file |
| `key` | no | `''` | Path to Ed25519 device key file. Generates an ephemeral keypair when not provided. |
| `trst-version` | no | `'latest'` | TrustEdge release version to download (e.g., `v4.0.0`). |

## Outputs

| Output | Description |
|--------|-------------|
| `attestation-path` | Absolute path to the generated `.te-attestation.json` file |

## How it works

1. Downloads the `trst` binary from [TrustEdge-Labs/trustedge releases](https://github.com/TrustEdge-Labs/trustedge/releases).
2. Generates an ephemeral Ed25519 keypair (unless you provide a persistent `key`).
3. Runs `trst attest-sbom` to create a cryptographically signed attestation that binds:
   - The binary artifact (via BLAKE3 hash)
   - The CycloneDX SBOM (via BLAKE3 hash)
   - Ed25519 signature over the attestation payload
4. Writes the attestation to `$RUNNER_TEMP/<binary-name>.te-attestation.json`.

## Verification

Verify an attestation locally:

```bash
trst verify-attestation my-app.te-attestation.json \
  --device-pub "ed25519:..." \
  --binary ./my-app \
  --sbom sbom.cdx.json
```

Or submit to the public TrustEdge verifier:

```bash
curl -X POST https://verify.trustedge.dev/v1/verify-attestation \
  -H "Content-Type: application/json" \
  -d @my-app.te-attestation.json
```

## Links

- [TrustEdge repository](https://github.com/TrustEdge-Labs/trustedge)
- [TrustEdge public verifier](https://verify.trustedge.dev)
- [CycloneDX SBOM specification](https://cyclonedx.org/)
- [syft SBOM generator](https://github.com/anchore/syft)

## License

Mozilla Public License 2.0 — see [LICENSE](LICENSE).
