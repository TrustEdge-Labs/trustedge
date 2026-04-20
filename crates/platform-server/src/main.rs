//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: sealedge — Privacy and trust at the edge.
//

//! TrustEdge Platform Server — thin entry point for the Axum HTTP service.
//!
//! All routing logic lives in `sealedge_platform::http::create_router`. This binary
//! is responsible only for: CLI parsing, env config loading, AppState wiring,
//! server binding, and graceful shutdown.

use anyhow::Result;
use clap::{Parser, Subcommand};
use sealedge_platform::http::{create_router, AppState, Config};
use sealedge_platform::verify::jwks::KeyManager;
use std::sync::Arc;
use tokio::sync::RwLock;

#[cfg(feature = "postgres")]
use sealedge_platform::database::{create_connection_pool, run_migrations};

/// TrustEdge Platform Server — boots the TrustEdge platform HTTP service.
#[derive(Parser)]
#[command(
    name = "sealedge-platform-server",
    version = env!("CARGO_PKG_VERSION"),
    about = "Sealedge Platform Server — privacy and trust at the edge"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the HTTP server (default)
    Serve,
    /// Run database migrations
    Migrate,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command.unwrap_or(Commands::Serve) {
        Commands::Serve => serve().await,
        Commands::Migrate => migrate().await,
    }
}

async fn serve() -> Result<()> {
    tracing_subscriber::fmt::init();

    let config = Config::from_env()?;

    // Determine runtime mode: full (postgres) vs verify-only
    #[cfg(feature = "postgres")]
    let mode = "full (postgres)";
    #[cfg(not(feature = "postgres"))]
    let mode = "verify-only";

    tracing::info!(
        "sealedge-platform-server v{} starting",
        env!("CARGO_PKG_VERSION")
    );
    tracing::info!("Port: {}", config.port);
    tracing::info!("Mode: {}", mode);
    tracing::info!("Receipt TTL: {}s", config.receipt_ttl_secs);
    let jwks_key_path = std::env::var("JWKS_KEY_PATH").unwrap_or_else(|_| {
        std::env::temp_dir()
            .join("sealedge_signing_key.json")
            .to_string_lossy()
            .into_owned()
    });
    tracing::info!("JWKS key path: {}", jwks_key_path);

    #[cfg(feature = "postgres")]
    tracing::info!(
        "Routes: POST /v1/verify, GET /.well-known/jwks.json, GET /healthz, POST /v1/devices, GET /v1/receipts/:id"
    );
    #[cfg(not(feature = "postgres"))]
    tracing::info!("Routes: POST /v1/verify, GET /.well-known/jwks.json, GET /healthz");

    let keys = Arc::new(RwLock::new(KeyManager::new()?));

    #[cfg(feature = "postgres")]
    let state = {
        let db_pool = create_connection_pool(&config.database_url).await?;
        AppState {
            keys,
            db_pool,
            receipt_ttl_secs: config.receipt_ttl_secs,
        }
    };

    #[cfg(not(feature = "postgres"))]
    let state = AppState {
        keys,
        receipt_ttl_secs: config.receipt_ttl_secs,
    };

    let router = create_router(state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.port)).await?;

    tracing::info!("Listening on 0.0.0.0:{}", config.port);

    let rps = std::env::var("RATE_LIMIT_RPS")
        .ok()
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(10);
    tracing::info!("Rate limit: {} req/sec per IP on /v1/verify", rps);

    axum::serve(
        listener,
        router.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await?;

    tracing::info!("Server shut down cleanly");

    Ok(())
}

async fn migrate() -> Result<()> {
    #[cfg(feature = "postgres")]
    {
        tracing_subscriber::fmt::init();
        let config = Config::from_env()?;
        let pool = create_connection_pool(&config.database_url).await?;
        run_migrations(&pool).await?;
        tracing::info!("Migrations complete");
        Ok(())
    }

    #[cfg(not(feature = "postgres"))]
    {
        Err(anyhow::anyhow!(
            "Built without postgres feature — migrations not available"
        ))
    }
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("Shutdown signal received, draining connections...");
}
