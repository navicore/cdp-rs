#!/usr/bin/env bash
# CDP Oracle Testing for CI/CD
set -e

echo "=== CDP Oracle Testing ==="

# Create oracle-test directory if it doesn't exist
if [ ! -d "oracle-test" ]; then
    mkdir -p oracle-test
fi
cd oracle-test

# Generate test audio if it doesn't exist
if [ ! -f "input.wav" ]; then
    echo "Generating test audio..."
    python3 ../scripts/generate-test-audio.py input.wav
fi

# Test 1: housekeep copy
echo "Testing: housekeep copy..."
rm -f rust_output.wav cdp_output.wav
../target/release/housekeep copy 1 input.wav rust_output.wav
../build/cdp/NewRelease/housekeep copy 1 input.wav cdp_output.wav > /dev/null 2>&1

if python3 ../scripts/oracle-compare.py rust_output.wav cdp_output.wav; then
    echo "✓ housekeep copy: PASSED"
else
    echo "✗ housekeep copy: FAILED"
    exit 1
fi

# Clean up
rm -f rust_output.wav cdp_output.wav

echo "=== All Oracle Tests Passed ==="