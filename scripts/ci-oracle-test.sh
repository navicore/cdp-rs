#!/usr/bin/env bash
# CDP Oracle Testing for CI/CD
set -e

echo "=== CDP Oracle Testing ==="

# Get to the root directory if we're not already there
if [ -d "oracle-test" ]; then
    cd oracle-test
elif [ -d "../oracle-test" ]; then
    cd ../oracle-test
elif [ ! -f "input.wav" ]; then
    echo "ERROR: Cannot find oracle-test directory"
    exit 1
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