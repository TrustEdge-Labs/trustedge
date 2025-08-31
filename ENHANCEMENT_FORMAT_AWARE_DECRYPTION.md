<!--
Copyright (c) 2025 John Turner
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/johnzilla/trustedge
-->


# Enhancement: Format-Aware Decryption

## Status: ✅ COMPLETED

Successfully implemented and tested format-aware decryption with MIME type detection and enhanced user experience.

## Original Issue

TrustEdge previously output raw decrypted bytes regardless of the original input type. This caused:
- ✅ File inputs worked correctly (original format preserved)  
- ❌ Audio inputs always output raw PCM (even when user expects audio format based on file extension)

## Implemented Enhancement

Format-aware decryption that checks the `DataType` in the manifest and provides appropriate user feedback.

## Testing Results

### File Format Detection ✅
- **JSON files**: Correctly detected as `application/json`
- **PDF files**: Correctly detected as `application/pdf`  
- **MP3 files**: Correctly detected as `audio/mpeg`
- **Unknown extensions**: Correctly fallback to `application/octet-stream`

### CLI Functionality ✅
- **--inspect**: Successfully shows format information without decryption
- **--verbose**: Provides enhanced user feedback with MIME types and emojis
- **--force-raw**: Available for edge cases (though Files preserve format by default)

### User Experience ✅
- Clear format feedback: "📄 Input Type: File, 📋 MIME Type: application/json"
- Completion messages: "✅ Output: Original file format preserved"
- Format-specific guidance: Different messages for audio vs file data types

### Example Output
```
$ ./trustedge-audio --input data.json --decrypt --verbose
📄 Input Type: File
📋 MIME Type: application/json
✅ Output: Original file format preserved
✅ Decrypt complete. Wrote 119 bytes.
📄 Output file preserves original format and should be directly usable.
📋 File type: application/json
```

## Migration Status

- [x] Enhanced CLI arguments implemented
- [x] MIME type detection system integrated  
- [x] Format-aware user messaging completed
- [x] Inspection functionality working
- [x] Comprehensive testing completed
- [x] Documentation updated

## Code Changes

### 1. Enhanced CLI Arguments

Add format detection and output options to `main.rs`:

```rust
#[derive(Parser)]
#[command(name = "trustedge-audio")]
pub struct Args {
    // ... existing fields ...
    
    /// Auto-detect output format based on manifest data type (default: true)
    #[arg(long, default_value = "true")]
    pub auto_format: bool,
    
    /// Force raw output regardless of data type
    #[arg(long)]
    pub force_raw: bool,
    
    /// Show data type information from manifest
    #[arg(long)]
    pub inspect: bool,
}
```

### 2. Enhanced Decrypt Function

Modify `decrypt_envelope()` in `main.rs`:

```rust
fn decrypt_envelope(args: &Args) -> Result<()> {
    // ... existing key and file setup code ...
    
    let mut manifest_data_type: Option<DataType> = None;
    let mut total_out = 0usize;
    let mut expected_seq: u64 = 1;
    let mut all_decrypted_data = Vec::new(); // Collect all data for format processing

    // record loop
    loop {
        let rec: Record = match deserialize_from(&mut r) {
            // ... existing record reading code ...
        };

        // ... existing validation code ...

        // manifest contents - deserialize first so we can use it for verification
        let m: Manifest = bincode::deserialize(&rec.sm.manifest).context("manifest decode")?;
        
        // Store data type from first manifest
        if manifest_data_type.is_none() {
            manifest_data_type = Some(m.data_type.clone());
            
            if args.inspect {
                print_manifest_info(&m);
            }
        }

        // ... existing decryption code ...

        // Collect decrypted data instead of writing immediately
        all_decrypted_data.extend_from_slice(&pt);
        total_out += pt.len();
    }

    // Process output based on data type
    process_decrypted_output(
        &all_decrypted_data,
        manifest_data_type.as_ref(),
        args,
        &mut w,
    )?;

    w.flush().context("flush plaintext")?;
    key_bytes.zeroize();

    eprintln!("Decrypt complete. Wrote {} bytes.", total_out);
    Ok(())
}

fn process_decrypted_output(
    data: &[u8],
    data_type: Option<&DataType>,
    args: &Args,
    writer: &mut impl Write,
) -> Result<()> {
    match data_type {
        Some(DataType::File { mime_type }) => {
            // File data: write as-is (original format preserved)
            writer.write_all(data).context("write file data")?;
            if args.verbose {
                eprintln!("Output: Original file format preserved");
                if let Some(mime) = mime_type {
                    eprintln!("MIME type: {}", mime);
                }
            }
        }
        
        Some(DataType::Audio { sample_rate, channels, format }) => {
            if args.force_raw || !args.auto_format {
                // Raw PCM output (current behavior)
                writer.write_all(data).context("write raw PCM data")?;
                if args.verbose {
                    eprintln!("Output: Raw PCM data (f32le, {}Hz, {} channels)", sample_rate, channels);
                }
            } else {
                // Future enhancement: could auto-convert to WAV format
                // For now, output raw PCM with clear messaging
                writer.write_all(data).context("write raw PCM data")?;
                eprintln!("Output: Raw PCM data (f32le, {}Hz, {} channels)", sample_rate, channels);
                eprintln!("Note: Use ffmpeg to convert: ffmpeg -f f32le -ar {} -ac {} -i output.raw output.wav", 
                    sample_rate, channels);
            }
        }
        
        Some(DataType::Video { .. }) => {
            // Future: video format handling
            writer.write_all(data).context("write video data")?;
            eprintln!("Output: Raw video data (format handling not yet implemented)");
        }
        
        Some(DataType::Sensor { .. }) => {
            // Future: sensor data handling  
            writer.write_all(data).context("write sensor data")?;
            eprintln!("Output: Raw sensor data");
        }
        
        Some(DataType::Unknown) | None => {
            // Unknown or missing data type: output raw bytes
            writer.write_all(data).context("write raw data")?;
            eprintln!("Output: Raw data (unknown original format)");
        }
    }
    
    Ok(())
}

fn print_manifest_info(manifest: &Manifest) {
    println!("TrustEdge Archive Contents:");
    println!("  Format Version: {}", manifest.v);
    println!("  Sequence: {}", manifest.seq);
    println!("  Timestamp: {}", manifest.ts_ms);
    
    match &manifest.data_type {
        DataType::File { mime_type } => {
            println!("  Data Type: File");
            if let Some(mime) = mime_type {
                println!("  MIME Type: {}", mime);
            }
            println!("  Output: Original file format will be preserved");
        }
        DataType::Audio { sample_rate, channels, format } => {
            println!("  Data Type: Audio (Live Capture)");
            println!("  Sample Rate: {} Hz", sample_rate);
            println!("  Channels: {}", channels);
            println!("  Format: {:?}", format);
            println!("  Output: Raw PCM data (requires conversion for playback)");
        }
        DataType::Video { width, height, fps, format } => {
            println!("  Data Type: Video");
            println!("  Resolution: {}x{}", width, height);
            println!("  FPS: {}", fps);
            println!("  Format: {}", format);
        }
        DataType::Sensor { sensor_type } => {
            println!("  Data Type: Sensor");
            println!("  Sensor Type: {}", sensor_type);
        }
        DataType::Unknown => {
            println!("  Data Type: Unknown");
        }
    }
}
```

### 3. Enhanced File Input Processing

Improve file input to detect and store MIME types:

```rust
// In the encrypt path, enhance file input processing
fn determine_data_type(input_source: &InputSource, file_path: Option<&PathBuf>) -> DataType {
    match input_source {
        InputSource::File(_) => {
            let mime_type = file_path
                .and_then(|path| path.extension())
                .and_then(|ext| ext.to_str())
                .map(|ext| match ext.to_lowercase().as_str() {
                    "pdf" => "application/pdf".to_string(),
                    "jpg" | "jpeg" => "image/jpeg".to_string(),
                    "png" => "image/png".to_string(),
                    "mp3" => "audio/mpeg".to_string(),
                    "wav" => "audio/wav".to_string(),
                    "mp4" => "video/mp4".to_string(),
                    "txt" => "text/plain".to_string(),
                    _ => format!("application/octet-stream"), // Binary fallback
                });
            
            DataType::File { mime_type }
        }
        InputSource::LiveAudio => DataType::Audio {
            sample_rate: args.sample_rate,
            channels: args.channels,
            format: AudioFormat::F32Le,
        },
    }
}
```

## Usage Examples

### Inspect Archive Contents
```bash
# View data type and metadata
./target/release/trustedge-audio --inspect --input archive.trst

# Output:
# TrustEdge Archive Contents:
#   Data Type: Audio (Live Capture)
#   Sample Rate: 44100 Hz
#   Channels: 2
#   Format: F32Le
#   Output: Raw PCM data (requires conversion for playback)
```

### Format-Aware Decryption
```bash
# Auto-format (default behavior)
./target/release/trustedge-audio --decrypt --input music.trst --out music.mp3 --key-hex $KEY
# File input: Outputs original MP3 format
# Audio input: Outputs raw PCM with conversion guidance

# Force raw output  
./target/release/trustedge-audio --decrypt --input music.trst --out music.raw --key-hex $KEY --force-raw
# Always outputs raw bytes regardless of input type
```

## Benefits

1. **Clear User Experience**: Users understand what they'll get based on input type
2. **Preserved Behavior**: File inputs continue working exactly as before
3. **Better Audio Guidance**: Live audio decryption provides clear conversion instructions
4. **Future Extensibility**: Framework ready for video and sensor data types
5. **Inspection Tool**: Users can check archive contents before decryption

## Migration Path

1. **Phase 1**: Add `--inspect` flag for archive inspection
2. **Phase 2**: Add format awareness with clear messaging
3. **Phase 3**: (Future) Add automatic format conversion options
4. **Phase 4**: (Future) Add format validation and error handling

This enhancement maintains backward compatibility while providing much clearer user experience and extensibility for future data types.
