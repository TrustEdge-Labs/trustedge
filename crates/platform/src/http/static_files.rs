//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//

//! Static file handlers — HTML pages embedded at compile time via `include_str!`.

use axum::response::Html;

/// The verify page HTML, embedded at compile time from `web/verify/index.html`.
const VERIFY_HTML: &str = include_str!("../../../../web/verify/index.html");

/// GET /verify — serve the self-contained attestation verifier page.
///
/// The page is compiled into the binary at build time; no runtime file dependency.
pub async fn verify_page_handler() -> Html<&'static str> {
    Html(VERIFY_HTML)
}
