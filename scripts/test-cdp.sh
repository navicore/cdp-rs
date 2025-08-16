#!/bin/bash
# Test CDP installation

set -e

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Find CDP binaries
find_cdp_path() {
    # Check environment variable first
    if [ -n "$CDP_PATH" ]; then
        echo "$CDP_PATH"
        return 0
    fi
    
    # Check common locations
    for dir in "cdp-bin/bin" "cdp-bin/cdp/bin" "/usr/local/cdp/bin"; do
        if [ -d "$dir" ]; then
            echo "$dir"
            return 0
        fi
    done
    
    return 1
}

# Test basic CDP functionality
test_cdp() {
    echo "======================================"
    echo "Testing CDP Installation"
    echo "======================================"
    echo ""
    
    CDP_BIN=$(find_cdp_path) || {
        echo -e "${RED}Error: Could not find CDP binaries${NC}"
        echo "Please run 'make install-cdp' first"
        exit 1
    }
    
    echo "CDP binaries found at: $CDP_BIN"
    echo ""
    
    # Create a test audio file using sox or generate with Ruby/Python if available
    TEMP_DIR=$(mktemp -d)
    TEST_WAV="$TEMP_DIR/test.wav"
    
    echo "Creating test audio file..."
    
    # Try different methods to create a test wav file
    if command -v sox &> /dev/null; then
        # Use sox to generate a sine wave
        sox -n -r 44100 -c 1 -b 16 "$TEST_WAV" synth 1 sine 440
        echo "Generated test file with sox"
    elif command -v python3 &> /dev/null; then
        # Use Python to generate a test file
        python3 -c "
import wave
import struct
import math

with wave.open('$TEST_WAV', 'w') as wav:
    wav.setnchannels(1)
    wav.setsampwidth(2)
    wav.setframerate(44100)
    for i in range(44100):
        value = int(32767.0 * math.sin(2.0 * math.pi * 440.0 * i / 44100.0))
        data = struct.pack('<h', value)
        wav.writeframesraw(data)
" && echo "Generated test file with Python"
    elif command -v ruby &> /dev/null; then
        # Use Ruby to generate a test file
        ruby -e "
require 'wavefile'
include WaveFile
buffer = Buffer.new([0] * 44100, Format.new(:mono, :pcm_16, 44100))
44100.times do |i|
  buffer.samples[i] = (32767 * Math.sin(2 * Math::PI * 440 * i / 44100.0)).to_i
end
Writer.new('$TEST_WAV', Format.new(:mono, :pcm_16, 44100)) do |writer|
  writer.write(buffer)
end
" 2>/dev/null && echo "Generated test file with Ruby" || {
            echo -e "${YELLOW}Warning: Could not generate test audio file${NC}"
            echo "Please install sox: brew install sox (Mac) or apt-get install sox (Linux)"
        }
    else
        echo -e "${YELLOW}Warning: No tool available to generate test audio${NC}"
        echo "Install sox for better testing: brew install sox (Mac) or apt-get install sox (Linux)"
    fi
    
    # Test CDP programs
    echo ""
    echo "Testing CDP programs..."
    echo ""
    
    # Test 1: Check if pvoc exists
    PVOC="$CDP_BIN/pvoc"
    if [ -f "$PVOC" ] || [ -f "$PVOC.exe" ]; then
        echo -e "${GREEN}✓ pvoc found${NC}"
        
        # Try to run pvoc help
        if "$PVOC" 2>&1 | grep -q "USAGE" || "$PVOC" -h 2>&1 | grep -q "pvoc"; then
            echo -e "${GREEN}✓ pvoc executes successfully${NC}"
        else
            echo -e "${YELLOW}⚠ pvoc found but may not be executable${NC}"
        fi
    else
        echo -e "${RED}✗ pvoc not found${NC}"
    fi
    
    # Test 2: Count available programs
    echo ""
    if [ -d "$CDP_BIN" ]; then
        # Count executables
        if [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "cygwin" ]]; then
            COUNT=$(ls -1 "$CDP_BIN"/*.exe 2>/dev/null | wc -l)
        else
            COUNT=$(find "$CDP_BIN" -type f -executable 2>/dev/null | wc -l)
        fi
        echo -e "${GREEN}✓ Found $COUNT CDP programs${NC}"
        
        # List a few programs
        echo ""
        echo "Sample CDP programs available:"
        if [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "cygwin" ]]; then
            ls -1 "$CDP_BIN"/*.exe 2>/dev/null | head -10 | xargs -n1 basename
        else
            find "$CDP_BIN" -type f -executable 2>/dev/null | head -10 | xargs -n1 basename
        fi
    fi
    
    # Clean up
    rm -rf "$TEMP_DIR"
    
    echo ""
    echo -e "${GREEN}CDP installation test complete!${NC}"
    echo ""
    echo "To use CDP in oracle tests:"
    echo "  export CDP_PATH=$CDP_BIN"
    echo "  make oracle"
}

# Run test
test_cdp "$@"