/**
 * TrustEdge WASM TypeScript Definitions
 * Type definitions for TrustEdge cryptographic operations
 */

/**
 * Encrypted data structure containing ciphertext, nonce, and optional key ID
 */
export class EncryptedData {
    constructor(ciphertext: string, nonce: string, keyId?: string | null);
    
    readonly ciphertext: string;
    readonly nonce: string;
    readonly key_id: string | null;
    
    /**
     * Serialize to JSON string
     */
    to_json(): string;
    
    /**
     * Deserialize from JSON string
     */
    static from_json(json: string): EncryptedData;
}

/**
 * Timer for performance measurement
 */
export class Timer {
    constructor();
    
    /**
     * Get elapsed time in milliseconds
     */
    elapsed(): number;
    
    /**
     * Log elapsed time with operation name
     */
    log_elapsed(operation: string): void;
}

/**
 * Main TrustEdge class providing high-level cryptographic operations
 */
export class TrustEdge {
    constructor();
    
    /**
     * Initialize the TrustEdge WASM module
     * Must be called before using any cryptographic functions
     */
    init(): Promise<TrustEdge>;
    
    /**
     * Check if the module is initialized
     */
    isInitialized(): boolean;
    
    /**
     * Get the version of TrustEdge WASM
     */
    getVersion(): string;
    
    /**
     * Generate a new 256-bit encryption key
     * @returns Base64-encoded key
     */
    generateKey(): string;
    
    /**
     * Generate a new nonce for encryption
     * @returns Base64-encoded nonce
     */
    generateNonce(): string;
    
    /**
     * Encrypt data using AES-256-GCM
     * @param data The data to encrypt
     * @param key Base64-encoded 256-bit key
     * @param nonce Optional base64-encoded nonce (auto-generated if not provided)
     * @returns Encrypted data object
     */
    encrypt(data: string, key: string, nonce?: string | null): EncryptedData;
    
    /**
     * Encrypt data with auto-generated nonce (convenience method)
     * @param data The data to encrypt
     * @param key Base64-encoded 256-bit key
     * @returns Encrypted data object
     */
    encryptSimple(data: string, key: string): EncryptedData;
    
    /**
     * Decrypt data using AES-256-GCM
     * @param encryptedData The encrypted data object
     * @param key Base64-encoded 256-bit key
     * @returns Decrypted plaintext
     */
    decrypt(encryptedData: EncryptedData | EncryptedDataLike, key: string): string;
    
    /**
     * Generate secure random bytes
     * @param length Number of bytes to generate
     * @returns Base64-encoded random bytes
     */
    generateRandomBytes(length: number): string;
    
    /**
     * Validate a base64-encoded key
     * @param key Base64-encoded key to validate
     * @returns True if valid
     */
    validateKey(key: string): boolean;
    
    /**
     * Validate a base64-encoded nonce
     * @param nonce Base64-encoded nonce to validate
     * @returns True if valid
     */
    validateNonce(nonce: string): boolean;
    
    /**
     * Create a new timer for performance measurement
     */
    createTimer(): Timer;
    
    /**
     * Log a message safely to console
     * @param message Message to log
     */
    log(message: string): void;
    
    /**
     * Run basic functionality test
     * @returns Test result
     */
    test(): string;
    
    /**
     * Show greeting alert (for testing)
     * @param name Name to greet
     */
    greet(name: string): void;
}

/**
 * Interface for encrypted data objects (for compatibility with plain objects)
 */
export interface EncryptedDataLike {
    ciphertext: string;
    nonce: string;
    key_id?: string | null;
}

/**
 * Configuration options for TrustEdge operations
 */
export interface TrustEdgeConfig {
    /**
     * Enable debug logging
     */
    debug?: boolean;
    
    /**
     * Custom WASM module path
     */
    wasmPath?: string;
}

/**
 * Error types that can be thrown by TrustEdge operations
 */
export class TrustEdgeError extends Error {
    constructor(message: string, cause?: Error);
    readonly cause?: Error;
}

/**
 * Default TrustEdge instance for convenience
 */
export const trustedge: TrustEdge;

/**
 * Export the main class as default
 */
export default TrustEdge;