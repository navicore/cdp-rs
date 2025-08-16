#!/bin/bash
# Test CDP installation with various operations
# This script generates test audio and runs it through CDP programs

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Setup paths
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
CDP_BIN="$PROJECT_ROOT/build/cdp-install/bin"
TEST_DIR="$PROJECT_ROOT/test-output"

# Check CDP installation
check_cdp() {
    if [ ! -d "$CDP_BIN" ]; then
        echo -e "${RED}✗ CDP not found at $CDP_BIN${NC}"
        echo "Run 'make build-cdp' first"
        exit 1
    fi
    
    # Set CDP path
    export CDP_PATH="$CDP_BIN"
    export PATH="$CDP_BIN:$PATH"
    
    echo -e "${GREEN}✓ Found CDP at $CDP_BIN${NC}"
    echo "Found $(ls -1 "$CDP_BIN" | wc -l) CDP programs"
}

# Create test directory
setup_test_dir() {
    echo -e "\n${BLUE}Setting up test directory...${NC}"
    rm -rf "$TEST_DIR"
    mkdir -p "$TEST_DIR"
    cd "$TEST_DIR"
}

# Generate test audio
generate_test_audio() {
    echo -e "\n${BLUE}Generating test audio...${NC}"
    
    # Check for Python
    if command -v python3 &> /dev/null; then
        python3 "$SCRIPT_DIR/generate-test-audio.py" "test_input.wav"
    else
        echo -e "${YELLOW}Python3 not found, generating with sox if available...${NC}"
        if command -v sox &> /dev/null; then
            # Generate 2-second 440Hz sine wave
            sox -n test_input.wav synth 2 sine 440
        else
            echo -e "${RED}Neither Python3 nor sox found. Please install one to generate test audio.${NC}"
            echo "You can install sox with: brew install sox"
            exit 1
        fi
    fi
    
    if [ -f "test_input.wav" ]; then
        echo -e "${GREEN}✓ Generated test_input.wav${NC}"
        ls -lh test_input.wav
    else
        echo -e "${RED}✗ Failed to generate test audio${NC}"
        exit 1
    fi
}

# Test function with error handling
run_test() {
    local name="$1"
    local command="$2"
    
    echo -n "  Testing $name... "
    if eval "$command" 2>/dev/null; then
        echo -e "${GREEN}✓${NC}"
        return 0
    else
        echo -e "${RED}✗${NC}"
        return 1
    fi
}

# Test basic CDP info commands
test_info_commands() {
    echo -e "\n${BLUE}Testing CDP info commands...${NC}"
    
    local passed=0
    local failed=0
    
    # Test sndinfo
    if [ -f "$CDP_BIN/sndinfo" ]; then
        if run_test "sndinfo" "$CDP_BIN/sndinfo test_input.wav > sndinfo.txt"; then
            ((passed++))
            echo "    $(head -1 sndinfo.txt)"
        else
            ((failed++))
        fi
    fi
    
    # Test maxsamp2
    if [ -f "$CDP_BIN/maxsamp2" ]; then
        if run_test "maxsamp2" "$CDP_BIN/maxsamp2 test_input.wav > maxsamp.txt"; then
            ((passed++))
        else
            ((failed++))
        fi
    fi
    
    echo -e "Info commands: ${GREEN}$passed passed${NC}, ${RED}$failed failed${NC}"
}

# Test basic transformations
test_transformations() {
    echo -e "\n${BLUE}Testing CDP transformations...${NC}"
    
    local passed=0
    local failed=0
    
    # Test modify speed (slow down by half)
    if [ -f "$CDP_BIN/modify" ]; then
        if run_test "modify speed" "$CDP_BIN/modify speed 1 test_input.wav test_slow.wav 0.5"; then
            ((passed++))
        else
            ((failed++))
        fi
    fi
    
    # Test housekeep chans (extract channel if stereo, or copy if mono)
    if [ -f "$CDP_BIN/housekeep" ]; then
        if run_test "housekeep copy" "$CDP_BIN/housekeep copy 1 test_input.wav test_copy.wav"; then
            ((passed++))
        else
            ((failed++))
        fi
    fi
    
    # Test envel attack (add fade in) - fixed parameter range
    if [ -f "$CDP_BIN/envel" ]; then
        if run_test "envel attack" "$CDP_BIN/envel attack 1 test_input.wav test_attack.wav 0.5 1000"; then
            ((passed++))
        else
            ((failed++))
        fi
    fi
    
    # Test filter lohi (bandpass filter) - fixed parameter order
    if [ -f "$CDP_BIN/filter" ]; then
        if run_test "filter lohi" "$CDP_BIN/filter lohi 1 test_input.wav test_filtered.wav -96 200 2000 -s1"; then
            ((passed++))
        else
            ((failed++))
        fi
    fi
    
    # Test distort multiply
    if [ -f "$CDP_BIN/distort" ]; then
        if run_test "distort multiply" "$CDP_BIN/distort multiply test_input.wav test_distorted.wav 2"; then
            ((passed++))
        else
            ((failed++))
        fi
    fi
    
    echo -e "Transformations: ${GREEN}$passed passed${NC}, ${RED}$failed failed${NC}"
}

# Test spectral processing (if pvoc available)
test_spectral() {
    echo -e "\n${BLUE}Testing CDP spectral processing...${NC}"
    
    local passed=0
    local failed=0
    
    # First, check if we can do spectral analysis
    if [ -f "$CDP_BIN/pvoc" ]; then
        if run_test "pvoc anal" "$CDP_BIN/pvoc anal 1 test_input.wav test.ana"; then
            ((passed++))
            
            # If analysis worked, try blur
            if [ -f "$CDP_BIN/blur" ]; then
                if run_test "blur avrg" "$CDP_BIN/blur avrg test.ana test_blur.ana 10"; then
                    ((passed++))
                else
                    ((failed++))
                fi
            fi
            
            # Try synthesis back to audio
            if run_test "pvoc synth" "$CDP_BIN/pvoc synth test.ana test_resynth.wav"; then
                ((passed++))
            else
                ((failed++))
            fi
        else
            ((failed++))
            echo -e "${YELLOW}  Note: pvoc anal failed - spectral processing unavailable${NC}"
        fi
    else
        echo -e "${YELLOW}  pvoc not found - skipping spectral tests${NC}"
    fi
    
    if [ $passed -gt 0 ] || [ $failed -gt 0 ]; then
        echo -e "Spectral: ${GREEN}$passed passed${NC}, ${RED}$failed failed${NC}"
    fi
}

# Test texture programs
test_texture() {
    echo -e "\n${BLUE}Testing CDP texture programs...${NC}"
    
    local passed=0
    local failed=0
    
    # Test grain - using correct syntax
    if [ -f "$CDP_BIN/grain" ]; then
        if run_test "grain duplicate" "$CDP_BIN/grain duplicate test_input.wav test_grain.wav 2 -b0.1"; then
            ((passed++))
        else
            ((failed++))
        fi
    fi
    
    # Test extend - using zigzag mode which works better
    if [ -f "$CDP_BIN/extend" ]; then
        if run_test "extend zigzag" "$CDP_BIN/extend zigzag 1 test_input.wav test_zigzag.wav 0.1 1.9 3 0.2"; then
            ((passed++))
        else
            ((failed++))
        fi
    fi
    
    echo -e "Texture: ${GREEN}$passed passed${NC}, ${RED}$failed failed${NC}"
}

# List generated files
list_outputs() {
    echo -e "\n${BLUE}Generated test files:${NC}"
    ls -lh *.wav *.ana *.txt 2>/dev/null | head -20 || echo "No output files found"
    
    echo -e "\n${BLUE}Output directory: $TEST_DIR${NC}"
}

# Summary
print_summary() {
    echo -e "\n${GREEN}========================================${NC}"
    echo -e "${GREEN}CDP Test Suite Complete!${NC}"
    echo -e "${GREEN}========================================${NC}"
    
    # Count output files
    local wav_count=$(ls -1 *.wav 2>/dev/null | wc -l)
    local ana_count=$(ls -1 *.ana 2>/dev/null | wc -l)
    
    echo "Generated $wav_count WAV files and $ana_count analysis files"
    
    echo -e "\n${YELLOW}To examine outputs:${NC}"
    echo "  cd $TEST_DIR"
    echo "  ls -la"
    
    if command -v afplay &> /dev/null; then
        echo -e "\n${YELLOW}To play a test file on macOS:${NC}"
        echo "  afplay $TEST_DIR/test_input.wav"
    elif command -v play &> /dev/null; then
        echo -e "\n${YELLOW}To play a test file:${NC}"
        echo "  play $TEST_DIR/test_input.wav"
    fi
}

# Main execution
main() {
    echo -e "${GREEN}========================================${NC}"
    echo -e "${GREEN}CDP Oracle Test Suite${NC}"
    echo -e "${GREEN}========================================${NC}"
    
    check_cdp
    setup_test_dir
    generate_test_audio
    test_info_commands
    test_transformations
    test_spectral
    test_texture
    list_outputs
    print_summary
}

# Run tests
main "$@"