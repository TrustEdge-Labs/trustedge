# TrustEdge WASM

WebAssembly bindings for TrustEdge cryptographic operations, providing high-performance encryption and decryption in web browsers and Node.js environments.

## Features

- **AES-256-GCM Encryption**: Industry-standard authenticated encryption
- **WebAssembly Performance**: Near-native speed cryptographic operations
- **Browser Compatible**: Works in all modern web browsers
- **TypeScript Support**: Full type definitions included
- **Easy Integration**: Simple JavaScript API
- **Secure Random Generation**: Cryptographically secure random number generation
- **Zero Dependencies**: Self-contained WASM module

## Installation

### NPM Package (Coming Soon)

```bash
npm install @trustedge/wasm
```

### Direct Usage

1. Build the WASM module:
```bash
wasm-pack build --target web --out-dir pkg
```

2. Include in your HTML:
```html
<script type="module">
import TrustEdge from './js/trustedge.js';

const trustedge = new TrustEdge();
await trustedge.init();

// Ready to use!
</script>
```

## Quick Start

### Basic Encryption/Decryption

```javascript
import TrustEdge from '@trustedge/wasm';

// Initialize
const trustedge = new TrustEdge();
await trustedge.init();

// Generate a key
const key = trustedge.generateKey();

// Encrypt data
const encrypted = trustedge.encryptSimple("Hello, World!", key);
console.log('Encrypted:', encrypted);

// Decrypt data
const decrypted = trustedge.decrypt(encrypted, key);
console.log('Decrypted:', decrypted); // "Hello, World!"
```

### Advanced Usage

```javascript
// Generate custom nonce
const nonce = trustedge.generateNonce();

// Encrypt with custom nonce
const encrypted = trustedge.encrypt("Secret data", key, nonce);

// Validate key format
if (trustedge.validateKey(key)) {
    console.log('Key is valid');
}

// Generate random bytes
const randomBytes = trustedge.generateRandomBytes(32);

// Performance timing
const timer = trustedge.createTimer();
// ... perform operations ...
console.log('Operation took:', timer.elapsed(), 'ms');
```

## API Reference

### TrustEdge Class

#### Constructor
```typescript
const trustedge = new TrustEdge();
```

#### Methods

##### `init(): Promise<TrustEdge>`
Initialize the WASM module. Must be called before using any cryptographic functions.

##### `generateKey(): string`
Generate a new 256-bit encryption key (base64-encoded).

##### `generateNonce(): string`
Generate a new 96-bit nonce for encryption (base64-encoded).

##### `encryptSimple(data: string, key: string): EncryptedData`
Encrypt data with auto-generated nonce.

##### `encrypt(data: string, key: string, nonce?: string): EncryptedData`
Encrypt data with optional custom nonce.

##### `decrypt(encryptedData: EncryptedData, key: string): string`
Decrypt encrypted data.

##### `validateKey(key: string): boolean`
Validate a base64-encoded key format.

##### `validateNonce(nonce: string): boolean`
Validate a base64-encoded nonce format.

##### `generateRandomBytes(length: number): string`
Generate secure random bytes (base64-encoded).

##### `createTimer(): Timer`
Create a performance timer.

### EncryptedData Class

```typescript
class EncryptedData {
    readonly ciphertext: string;
    readonly nonce: string;
    readonly key_id: string | null;
    
    to_json(): string;
    static from_json(json: string): EncryptedData;
}
```

### Timer Class

```typescript
class Timer {
    elapsed(): number;
    log_elapsed(operation: string): void;
}
```

## Examples

### Web Browser Example

See `examples/basic-usage.html` for a complete interactive example.

### Node.js Example

```javascript
import TrustEdge from '@trustedge/wasm';

async function example() {
    const trustedge = new TrustEdge();
    await trustedge.init();
    
    const key = trustedge.generateKey();
    const data = "Sensitive information";
    
    const encrypted = trustedge.encryptSimple(data, key);
    const decrypted = trustedge.decrypt(encrypted, key);
    
    console.log('Original:', data);
    console.log('Decrypted:', decrypted);
    console.log('Match:', data === decrypted);
}

example().catch(console.error);
```

## Performance

TrustEdge WASM provides excellent performance for cryptographic operations:

- **Encryption**: ~0.1-0.5ms per operation (1KB data)
- **Decryption**: ~0.1-0.5ms per operation (1KB data)
- **Throughput**: 50-200 MB/s (depending on browser and hardware)

Performance may vary based on:
- Browser engine (V8, SpiderMonkey, etc.)
- Hardware capabilities
- Data size
- System load

## Security

- **AES-256-GCM**: Authenticated encryption with 256-bit keys
- **Secure Random**: Uses cryptographically secure random number generation
- **Memory Safety**: Rust's memory safety guarantees
- **Side-Channel Resistance**: Constant-time operations where possible

## Browser Compatibility

- Chrome/Chromium 57+
- Firefox 52+
- Safari 11+
- Edge 16+

## Building from Source

### Prerequisites

- Rust 1.89+
- wasm-pack
- Node.js 16+ (for testing)

### Build Steps

```bash
# Install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Add WASM target
rustup target add wasm32-unknown-unknown

# Build for web
wasm-pack build --target web --out-dir pkg

# Build for Node.js
wasm-pack build --target nodejs --out-dir pkg-node

# Build for bundlers
wasm-pack build --target bundler --out-dir pkg-bundler
```

### Testing

```bash
# Start test server
python3 -m http.server 8080

# Open browser to http://localhost:8080/test.html
```

## License

This project is licensed under the Mozilla Public License 2.0 (MPL-2.0).

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](../CONTRIBUTING.md) for guidelines.

## Support

- **Issues**: [GitHub Issues](https://github.com/trustedge-labs/trustedge/issues)
- **Documentation**: [TrustEdge Docs](https://github.com/trustedge-labs/trustedge#readme)
- **Enterprise**: [enterprise@trustedgelabs.com](mailto:enterprise@trustedgelabs.com)

## Changelog

### v0.1.0
- Initial release
- AES-256-GCM encryption/decryption
- WebAssembly bindings
- JavaScript/TypeScript SDK
- Browser and Node.js support