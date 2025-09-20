<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


# cam.video demo

This directory contains a minimal end-to-end walkthrough for producing and verifying a `.trst` archive using the `cam.video` profile.

## 5-minute run

```bash
cargo run -p trst-cli -- wrap --profile cam.video --in ./examples/cam.video/sample.bin --out ./examples/cam.video/sample.trst
cargo run -p trst-cli -- verify ./examples/cam.video/sample.trst --device-pub $(cat device.pub)
```

After the first run the CLI writes `device.key`/`device.pub` into the working directory. Re-run the `wrap` command to reuse the same device identity, or pass `--device-key` to point at an existing signing key.

The `record_and_wrap.rs` and `verify_cli.rs` snippets illustrate how to call the core APIs programmatically without the CLI wrapper.
