#!/bin/bash
# Oracle test for housekeep copy operation
# Compares CDP's housekeep copy with our Rust implementation

set -e

# Setup
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
CDP_BIN="$PROJECT_ROOT/build/cdp-install/bin"
RUST_BIN="$PROJECT_ROOT/target/debug"
TEST_DIR="$PROJECT_ROOT/oracle-test"

echo "==================================="
echo "Oracle Test: housekeep copy"
echo "==================================="

# Cleanup and setup
rm -rf "$TEST_DIR"
mkdir -p "$TEST_DIR"
cd "$TEST_DIR"

# Generate test input
echo "Generating test input..."
python3 "$SCRIPT_DIR/generate-test-audio.py" input.wav

# Run CDP version
echo ""
echo "Running CDP housekeep copy..."
"$CDP_BIN/housekeep" copy 1 input.wav cdp_output.wav

# Run Rust version
echo "Running Rust housekeep copy..."
"$RUST_BIN/housekeep" copy 1 input.wav rust_output.wav

# Compare outputs
echo ""
echo "Comparing outputs..."
if diff cdp_output.wav rust_output.wav > /dev/null; then
    echo "✅ PASS: Outputs are identical!"
    exit 0
else
    echo "❌ FAIL: Outputs differ"
    echo "File sizes:"
    ls -l *output.wav
    
    # Show first difference
    echo ""
    echo "First bytes that differ:"
    cmp -l cdp_output.wav rust_output.wav | head -5
    exit 1
fi