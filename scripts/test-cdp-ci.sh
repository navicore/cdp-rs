#!/bin/bash
# Lightweight CDP test suite for CI environments
# Focuses on binary operations without audio playback

set -e

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

# Run core CDP operations
echo ""
echo "Testing core CDP operations..."

PASSED=0
FAILED=0

# Test 1: housekeep copy
echo -n "  housekeep copy... "
if $CDP_BIN/housekeep copy 1 test.wav copy.wav 2>/dev/null; then
    echo "PASS"
    ((PASSED++))
else
    echo "FAIL"
    ((FAILED++))
fi

# Test 2: modify speed
echo -n "  modify speed... "
if $CDP_BIN/modify speed 1 test.wav slow.wav 0.5 2>/dev/null; then
    echo "PASS"
    ((PASSED++))
else
    echo "FAIL"
    ((FAILED++))
fi

# Test 3: pvoc analysis
echo -n "  pvoc anal... "
if $CDP_BIN/pvoc anal 1 test.wav test.ana 2>/dev/null; then
    echo "PASS"
    ((PASSED++))
else
    echo "FAIL"
    ((FAILED++))
fi

# Test 4: blur if analysis succeeded
if [ -f "test.ana" ]; then
    echo -n "  blur avrg... "
    if $CDP_BIN/blur avrg test.ana blur.ana 10 2>/dev/null; then
        echo "PASS"
        ((PASSED++))
    else
        echo "FAIL"
        ((FAILED++))
    fi
    
    # Test 5: pvoc synthesis
    echo -n "  pvoc synth... "
    if $CDP_BIN/pvoc synth test.ana resynth.wav 2>/dev/null; then
        echo "PASS"
        ((PASSED++))
    else
        echo "FAIL"
        ((FAILED++))
    fi
fi

# Test 6: filter
echo -n "  filter lohi... "
if $CDP_BIN/filter lohi 1 test.wav filtered.wav -96 200 2000 -s1 2>/dev/null; then
    echo "PASS"
    ((PASSED++))
else
    echo "FAIL"
    ((FAILED++))
fi

# Test 7: distort
echo -n "  distort multiply... "
if $CDP_BIN/distort multiply test.wav distorted.wav 2 2>/dev/null; then
    echo "PASS"
    ((PASSED++))
else
    echo "FAIL"
    ((FAILED++))
fi

# Test 8: extend
echo -n "  extend zigzag... "
if $CDP_BIN/extend zigzag 1 test.wav zigzag.wav 0.1 1.9 3 0.2 2>/dev/null; then
    echo "PASS"
    ((PASSED++))
else
    echo "FAIL"
    ((FAILED++))
fi

# Summary
echo ""
echo "========================================="
echo "Test Results: $PASSED passed, $FAILED failed"
echo "========================================="

# List generated files
echo ""
echo "Generated files:"
ls -lh *.wav *.ana 2>/dev/null | head -20 || echo "No files generated"

# Exit with appropriate code
if [ $FAILED -gt 0 ]; then
    echo ""
    echo "Some tests failed, but CDP is partially working"
    exit 1
else
    echo ""
    echo "All tests passed!"
    exit 0
fi