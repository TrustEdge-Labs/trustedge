//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: sealedge — Privacy and trust at the edge.
//

fn main() {
    // Re-run if the override env var changes
    println!("cargo:rerun-if-env-changed=ALLOW_INSECURE_TLS");

    let profile = std::env::var("PROFILE").unwrap_or_default();
    let insecure_tls = std::env::var("CARGO_FEATURE_INSECURE_TLS").is_ok();
    let allow_override = std::env::var("ALLOW_INSECURE_TLS").unwrap_or_default() == "1";

    if profile == "release" && insecure_tls && !allow_override {
        panic!(
            "\n\nSECURITY ERROR: The `insecure-tls` feature is enabled in a release build.\n\
             This feature disables TLS certificate verification and must never ship in production.\n\n\
             To build release with insecure-tls for testing (NOT for deployment):\n\
             \n    ALLOW_INSECURE_TLS=1 cargo build --release --features insecure-tls\n\n\
             Refusing to compile.\n"
        );
    }
}
