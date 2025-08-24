#!/bin/bash
# Lightweight CDP test suite for CI environments
# FAIL FAST - Any test failure stops everything

set -e  # Exit immediately on any error
set -u  # Exit on undefined variables
set -o pipefail  # Exit on pipe failures

# Setup paths
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
CDP_BIN="$PROJECT_ROOT/build/cdp-install/bin"
TEST_DIR="$PROJECT_ROOT/test-output-ci"

# Simple output (no colors in CI)
export CDP_PATH="$CDP_BIN"
export PATH="$CDP_BIN:$PATH"

echo "========================================="
echo "CDP CI Test Suite"
echo "========================================="

# Check CDP installation
if [ ! -d "$CDP_BIN" ]; then
    echo "ERROR: CDP not found at $CDP_BIN"
    exit 1
fi

echo "Found CDP at $CDP_BIN"
echo "Found $(ls -1 "$CDP_BIN" | wc -l) CDP programs"

# Create test directory
rm -rf "$TEST_DIR"
mkdir -p "$TEST_DIR"
cd "$TEST_DIR"

# Generate test audio using Python
echo ""
echo "Generating test audio..."
python3 "$SCRIPT_DIR/generate-test-audio.py" "test.wav"

if [ ! -f "test.wav" ]; then
    echo "ERROR: Failed to generate test audio"
    exit 1
fi

echo "Generated test.wav: $(ls -lh test.wav | awk '{print $5}')"

# Run core CDP operations - FAIL FAST on any error
echo ""
echo "Testing core CDP operations..."

# Test 1: housekeep copy
echo -n "  housekeep copy... "
$CDP_BIN/housekeep copy 1 test.wav copy.wav
echo "PASS"

# Test 2: modify speed
echo -n "  modify speed... "
$CDP_BIN/modify speed 1 test.wav slow.wav 0.5
echo "PASS"

# Test 3: pvoc analysis
echo -n "  pvoc anal... "
$CDP_BIN/pvoc anal 1 test.wav test.ana
echo "PASS"

# Test 4: blur
echo -n "  blur avrg... "
$CDP_BIN/blur avrg test.ana blur.ana 10
echo "PASS"

# Test 5: pvoc synthesis
echo -n "  pvoc synth... "
$CDP_BIN/pvoc synth test.ana resynth.wav
echo "PASS"

# Test 6: filter
echo -n "  filter lohi... "
$CDP_BIN/filter lohi 1 test.wav filtered.wav -96 200 2000 -s1
echo "PASS"

# Test 7: distort
echo -n "  distort multiply... "
$CDP_BIN/distort multiply test.wav distorted.wav 2
echo "PASS"

# Test 8: extend
echo -n "  extend zigzag... "
$CDP_BIN/extend zigzag 1 test.wav zigzag.wav 0.1 1.9 3 0.2
echo "PASS"

# If we got here, everything passed
echo ""
echo "========================================="
echo "All CDP tests passed!"
echo "========================================="

# List generated files
echo ""
echo "Generated files:"
ls -lh *.wav *.ana 2>/dev/null | head -20

exit 0