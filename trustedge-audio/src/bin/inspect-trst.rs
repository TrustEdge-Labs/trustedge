#!/usr/bin/env rust-script
//
// Copyright (c) 2025 John Turner
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/
//
// Project: trustedge — Privacy and trust at the edge.
//
//! Simple utility to inspect .trst file metadata
//!
//! Usage: cargo run --bin inspect-trst -- <file.trst>

use anyhow::{Context, Result};
use bincode::deserialize_from;
use std::env;
use std::fs::File;
use std::io::BufReader;
use trustedge_audio::{Record, StreamHeader, MAGIC};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file.trst>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let file = File::open(filename).context("Failed to open file")?;
    let mut reader = BufReader::new(file);

    // Read magic and version
    let mut magic = [0u8; 4];
    std::io::Read::read_exact(&mut reader, &mut magic)?;
    if magic != *MAGIC {
        return Err(anyhow::anyhow!("Invalid magic number"));
    }

    let mut version = [0u8; 1];
    std::io::Read::read_exact(&mut reader, &mut version)?;
    println!("File format: TRST v{}", version[0]);

    // Read stream header
    let header: StreamHeader = deserialize_from(&mut reader)?;
    println!("Header hash: {:02x?}...", &header.header_hash[..8]);

    // Read first record to inspect manifest
    let record: Record = deserialize_from(&mut reader)?;
    println!("First record sequence: {}", record.seq);

    // Deserialize and inspect the manifest
    let manifest: trustedge_audio::Manifest = bincode::deserialize(&record.sm.manifest)?;

    println!("Manifest:");
    println!("  Version: {}", manifest.v);
    println!("  Timestamp: {} ms", manifest.ts_ms);
    println!("  Sequence: {}", manifest.seq);
    println!("  AI Used: {}", manifest.ai_used);
    println!("  Data Type: {:?}", manifest.data_type);

    Ok(())
}
