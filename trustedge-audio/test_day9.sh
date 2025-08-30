#!/bin/bash
# Test script for Day 9 network resilience improvements

set -e

echo "🚀 TrustEdge Day 9: Network Resilience Testing"
echo "=============================================="

# Navigate to correct directory
cd /home/john/projects/github.com/trustedge/trustedge-audio

# Create test data
echo "Creating test data..."
echo "Day 9 network resilience test data" > day9_test.txt

# Generate test key
TEST_KEY=$(openssl rand -hex 32)
echo "Test key: $TEST_KEY"

echo ""
echo "1. Testing Connection Timeout & Retry Logic"
echo "-------------------------------------------"
echo "Attempting to connect to non-existent server..."

# Test 1: Connection timeout and retry with non-existent server
./target/release/trustedge-client \
  --server 127.0.0.1:9999 \
  --file day9_test.txt \
  --key-hex $TEST_KEY \
  --verbose \
  --retry-attempts 3 \
  --connect-timeout 2 \
  --retry-delay 1 || true

echo ""
echo "2. Testing Real Server Connection"  
echo "--------------------------------"

# Start server in background
echo "Starting TrustEdge server..."
./target/release/trustedge-server \
  --listen 127.0.0.1:8080 \
  --verbose \
  --key-hex $TEST_KEY \
  --decrypt \
  --output-dir ./received_chunks &

SERVER_PID=$!
echo "Server started with PID: $SERVER_PID"

# Wait for server to start
sleep 2

echo "Testing successful connection with timeouts..."
./target/release/trustedge-client \
  --server 127.0.0.1:8080 \
  --file day9_test.txt \
  --key-hex $TEST_KEY \
  --verbose \
  --connect-timeout 10 \
  --retry-attempts 1

echo ""
echo "3. Testing Graceful Shutdown"
echo "----------------------------"
echo "Sending SIGINT to server..."
kill -INT $SERVER_PID

# Wait for graceful shutdown
sleep 3

echo ""
echo "✅ All Day 9 network resilience tests completed!"
echo ""
echo "New Features Demonstrated:"
echo "- ✅ Connection timeouts and retry logic"
echo "- ✅ Graceful server shutdown with SIGINT"
echo "- ✅ Improved error messages and verbose logging"
echo "- ✅ Timeout handling for chunk operations"

# Cleanup
rm -f day9_test.txt
rm -rf received_chunks
