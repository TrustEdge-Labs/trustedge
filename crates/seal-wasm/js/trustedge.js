/**
 * TrustEdge WASM JavaScript SDK
 * High-level wrapper for TrustEdge cryptographic operations
 */

import init, {
    // Basic functions
    greet,
    test_basic_functionality,
    version,
    safe_log,
    Timer,
    
    // Crypto functions
    EncryptedData,
    generate_key,
    generate_nonce,
    encrypt,
    decrypt,
    encrypt_simple,
    generate_random_bytes,
    validate_key,
    validate_nonce
} from '../pkg/trustedge_wasm.js';

class TrustEdge {
    constructor() {
        this.initialized = false;
        this.wasmModule = null;
    }

    /**
     * Initialize the TrustEdge WASM module
     * Must be called before using any cryptographic functions
     */
    async init() {
        if (this.initialized) {
            return this;
        }

        try {
            this.wasmModule = await init();
            this.initialized = true;
            console.log('TrustEdge WASM initialized successfully');
            return this;
        } catch (error) {
            console.error('Failed to initialize TrustEdge WASM:', error);
            throw new Error(`TrustEdge initialization failed: ${error.message}`);
        }
    }

    /**
     * Check if the module is initialized
     */
    isInitialized() {
        return this.initialized;
    }

    /**
     * Get the version of TrustEdge WASM
     */
    getVersion() {
        this._ensureInitialized();
        return version();
    }

    /**
     * Generate a new 256-bit encryption key
     * @returns {string} Base64-encoded key
     */
    generateKey() {
        this._ensureInitialized();
        return generate_key();
    }

    /**
     * Generate a new nonce for encryption
     * @returns {string} Base64-encoded nonce
     */
    generateNonce() {
        this._ensureInitialized();
        return generate_nonce();
    }

    /**
     * Encrypt data using AES-256-GCM
     * @param {string} data - The data to encrypt
     * @param {string} key - Base64-encoded 256-bit key
     * @param {string} [nonce] - Optional base64-encoded nonce (auto-generated if not provided)
     * @returns {EncryptedData} Encrypted data object
     */
    encrypt(data, key, nonce = null) {
        this._ensureInitialized();
        
        if (!this.validateKey(key)) {
            throw new Error('Invalid key format or length');
        }
        
        if (nonce && !this.validateNonce(nonce)) {
            throw new Error('Invalid nonce format or length');
        }

        return encrypt(data, key, nonce);
    }

    /**
     * Encrypt data with auto-generated nonce (convenience method)
     * @param {string} data - The data to encrypt
     * @param {string} key - Base64-encoded 256-bit key
     * @returns {EncryptedData} Encrypted data object
     */
    encryptSimple(data, key) {
        this._ensureInitialized();
        
        if (!this.validateKey(key)) {
            throw new Error('Invalid key format or length');
        }

        return encrypt_simple(data, key);
    }

    /**
     * Decrypt data using AES-256-GCM
     * @param {EncryptedData|Object} encryptedData - The encrypted data object
     * @param {string} key - Base64-encoded 256-bit key
     * @returns {string} Decrypted plaintext
     */
    decrypt(encryptedData, key) {
        this._ensureInitialized();
        
        if (!this.validateKey(key)) {
            throw new Error('Invalid key format or length');
        }

        // Handle plain objects by converting to EncryptedData
        if (!(encryptedData instanceof EncryptedData)) {
            encryptedData = new EncryptedData(
                encryptedData.ciphertext,
                encryptedData.nonce,
                encryptedData.key_id || null
            );
        }

        return decrypt(encryptedData, key);
    }

    /**
     * Generate secure random bytes
     * @param {number} length - Number of bytes to generate
     * @returns {string} Base64-encoded random bytes
     */
    generateRandomBytes(length) {
        this._ensureInitialized();
        
        if (length <= 0 || !Number.isInteger(length)) {
            throw new Error('Length must be a positive integer');
        }

        return generate_random_bytes(length);
    }

    /**
     * Validate a base64-encoded key
     * @param {string} key - Base64-encoded key to validate
     * @returns {boolean} True if valid
     */
    validateKey(key) {
        this._ensureInitialized();
        return validate_key(key);
    }

    /**
     * Validate a base64-encoded nonce
     * @param {string} nonce - Base64-encoded nonce to validate
     * @returns {boolean} True if valid
     */
    validateNonce(nonce) {
        this._ensureInitialized();
        return validate_nonce(nonce);
    }

    /**
     * Create a new timer for performance measurement
     * @returns {Timer} Timer instance
     */
    createTimer() {
        this._ensureInitialized();
        return new Timer();
    }

    /**
     * Log a message safely to console
     * @param {string} message - Message to log
     */
    log(message) {
        this._ensureInitialized();
        safe_log(message);
    }

    /**
     * Run basic functionality test
     * @returns {string} Test result
     */
    test() {
        this._ensureInitialized();
        return test_basic_functionality();
    }

    /**
     * Show greeting alert (for testing)
     * @param {string} name - Name to greet
     */
    greet(name) {
        this._ensureInitialized();
        greet(name);
    }

    /**
     * Ensure the module is initialized before calling WASM functions
     * @private
     */
    _ensureInitialized() {
        if (!this.initialized) {
            throw new Error('TrustEdge WASM module not initialized. Call init() first.');
        }
    }
}

// Export the main class and individual functions
export default TrustEdge;
export {
    TrustEdge,
    EncryptedData,
    Timer
};

// Create a default instance for convenience
export const trustedge = new TrustEdge();