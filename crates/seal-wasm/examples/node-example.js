#!/usr/bin/env node

/**
 * TrustEdge WASM Node.js Example
 * Demonstrates basic usage in Node.js environment
 */

import TrustEdge from '../js/trustedge.js';

async function basicExample() {
    console.log('🔐 TrustEdge WASM Node.js Example\n');
    
    try {
        // Initialize TrustEdge
        console.log('Initializing TrustEdge WASM...');
        const trustedge = new TrustEdge();
        await trustedge.init();
        console.log('✅ Initialized successfully\n');
        
        // Show version
        console.log('Version:', trustedge.getVersion());
        console.log('');
        
        // Generate key
        console.log('Generating encryption key...');
        const key = trustedge.generateKey();
        console.log('✅ Key generated:', key.substring(0, 32) + '...\n');
        
        // Test data
        const testData = 'Hello from TrustEdge WASM in Node.js! This is a secret message.';
        console.log('Original data:', testData);
        console.log('');
        
        // Encrypt
        console.log('Encrypting data...');
        const timer = trustedge.createTimer();
        const encrypted = trustedge.encryptSimple(testData, key);
        const encryptTime = timer.elapsed();
        
        console.log('✅ Encryption successful');
        console.log('   Time:', encryptTime.toFixed(2) + 'ms');
        console.log('   Ciphertext:', encrypted.ciphertext.substring(0, 32) + '...');
        console.log('   Nonce:', encrypted.nonce);
        console.log('');
        
        // Decrypt
        console.log('Decrypting data...');
        const decryptTimer = trustedge.createTimer();
        const decrypted = trustedge.decrypt(encrypted, key);
        const decryptTime = decryptTimer.elapsed();
        
        console.log('✅ Decryption successful');
        console.log('   Time:', decryptTime.toFixed(2) + 'ms');
        console.log('   Decrypted:', decrypted);
        console.log('');
        
        // Verify integrity
        const isValid = testData === decrypted;
        console.log('Data integrity check:', isValid ? '✅ PASSED' : '❌ FAILED');
        
        if (!isValid) {
            console.error('ERROR: Decrypted data does not match original!');
            process.exit(1);
        }
        
        console.log('\n🎉 All tests passed successfully!');
        
    } catch (error) {
        console.error('❌ Error:', error.message);
        process.exit(1);
    }
}

async function performanceTest() {
    console.log('\n⚡ Performance Test\n');
    
    try {
        const trustedge = new TrustEdge();
        await trustedge.init();
        
        const iterations = 1000;
        const testData = 'Performance test data '.repeat(100); // ~2KB
        const key = trustedge.generateKey();
        
        console.log('Test parameters:');
        console.log('   Data size:', (testData.length / 1024).toFixed(1) + ' KB');
        console.log('   Iterations:', iterations);
        console.log('');
        
        // Encryption performance
        console.log('Testing encryption performance...');
        const encryptTimer = trustedge.createTimer();
        const encryptedResults = [];
        
        for (let i = 0; i < iterations; i++) {
            encryptedResults.push(trustedge.encryptSimple(testData, key));
        }
        const encryptTime = encryptTimer.elapsed();
        
        // Decryption performance
        console.log('Testing decryption performance...');
        const decryptTimer = trustedge.createTimer();
        
        for (let i = 0; i < iterations; i++) {
            trustedge.decrypt(encryptedResults[i], key);
        }
        const decryptTime = decryptTimer.elapsed();
        
        // Results
        console.log('Performance Results:');
        console.log('');
        console.log('Encryption:');
        console.log('   Total time:', encryptTime.toFixed(2) + 'ms');
        console.log('   Average:', (encryptTime / iterations).toFixed(2) + 'ms per operation');
        console.log('   Throughput:', ((testData.length * iterations) / (encryptTime / 1000) / 1024 / 1024).toFixed(2) + ' MB/s');
        console.log('');
        console.log('Decryption:');
        console.log('   Total time:', decryptTime.toFixed(2) + 'ms');
        console.log('   Average:', (decryptTime / iterations).toFixed(2) + 'ms per operation');
        console.log('   Throughput:', ((testData.length * iterations) / (decryptTime / 1000) / 1024 / 1024).toFixed(2) + ' MB/s');
        console.log('');
        console.log('Total time:', (encryptTime + decryptTime).toFixed(2) + 'ms');
        
    } catch (error) {
        console.error('❌ Performance test failed:', error.message);
        process.exit(1);
    }
}

async function jsonSerializationTest() {
    console.log('\n📄 JSON Serialization Test\n');
    
    try {
        const trustedge = new TrustEdge();
        await trustedge.init();
        
        const key = trustedge.generateKey();
        const testData = 'JSON serialization test data';
        
        // Encrypt
        const encrypted = trustedge.encryptSimple(testData, key);
        console.log('Original encrypted object:', encrypted);
        
        // Serialize to JSON
        const jsonString = encrypted.to_json();
        console.log('Serialized JSON:', jsonString);
        
        // Parse back from JSON
        const parsedData = JSON.parse(jsonString);
        console.log('Parsed object:', parsedData);
        
        // Decrypt using parsed object
        const decrypted = trustedge.decrypt(parsedData, key);
        console.log('Decrypted from JSON:', decrypted);
        
        // Verify
        const isValid = testData === decrypted;
        console.log('JSON round-trip test:', isValid ? '✅ PASSED' : '❌ FAILED');
        
        if (!isValid) {
            throw new Error('JSON serialization round-trip failed');
        }
        
    } catch (error) {
        console.error('❌ JSON serialization test failed:', error.message);
        process.exit(1);
    }
}

// Main execution
async function main() {
    const args = process.argv.slice(2);
    
    if (args.includes('--performance') || args.includes('-p')) {
        await performanceTest();
    } else if (args.includes('--json') || args.includes('-j')) {
        await jsonSerializationTest();
    } else if (args.includes('--all') || args.includes('-a')) {
        await basicExample();
        await performanceTest();
        await jsonSerializationTest();
    } else {
        await basicExample();
    }
}

// Handle command line arguments
if (process.argv.includes('--help') || process.argv.includes('-h')) {
    console.log('TrustEdge WASM Node.js Example');
    console.log('');
    console.log('Usage: node node-example.js [options]');
    console.log('');
    console.log('Options:');
    console.log('  --help, -h         Show this help message');
    console.log('  --performance, -p  Run performance test');
    console.log('  --json, -j         Run JSON serialization test');
    console.log('  --all, -a          Run all tests');
    console.log('');
    console.log('Default: Run basic example');
    process.exit(0);
}

main().catch(error => {
    console.error('Fatal error:', error);
    process.exit(1);
});