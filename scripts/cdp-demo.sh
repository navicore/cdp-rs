#!/bin/bash
# Interactive CDP Demo
# Shows off CDP's audio processing capabilities

set -e

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Setup
CDP_BIN="$(dirname "$(dirname "$(readlink -f "$0" 2>/dev/null || echo "$0")")")/build/cdp-install/bin"
export PATH="$CDP_BIN:$PATH"

echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}CDP Audio Processing Demo${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""
echo "This demo will:"
echo "1. Generate a test tone"
echo "2. Apply time stretching"
echo "3. Apply spectral blurring"
echo "4. Add granular texture"
echo ""

# Check for audio player
PLAYER=""
if command -v afplay &> /dev/null; then
    PLAYER="afplay"
elif command -v play &> /dev/null; then
    PLAYER="play"
else
    echo -e "${YELLOW}No audio player found. Install sox for playback.${NC}"
fi

# Create demo directory
DEMO_DIR="cdp-demo-output"
rm -rf "$DEMO_DIR"
mkdir -p "$DEMO_DIR"
cd "$DEMO_DIR"

echo -e "\n${BLUE}Step 1: Generating test audio (440Hz sine wave)...${NC}"
python3 ../scripts/generate-test-audio.py original.wav
[ -n "$PLAYER" ] && echo "Playing original..." && $PLAYER original.wav 2>/dev/null

echo -e "\n${BLUE}Step 2: Time-stretching by 1.5x...${NC}"
$CDP_BIN/modify speed 1 original.wav stretched.wav 0.667
echo "Created stretched.wav"
[ -n "$PLAYER" ] && echo "Playing stretched..." && $PLAYER stretched.wav 2>/dev/null

echo -e "\n${BLUE}Step 3: Creating spectral analysis...${NC}"
$CDP_BIN/pvoc anal 1 original.wav original.ana 2>/dev/null
echo "Created spectral analysis file"

echo -e "\n${BLUE}Step 4: Applying spectral blur...${NC}"
$CDP_BIN/blur avrg original.ana blurred.ana 10 2>/dev/null
# Synthesize (gain control happens automatically in pvoc synth)
$CDP_BIN/pvoc synth blurred.ana blurred.wav 2>/dev/null
echo "Created blurred.wav"
[ -n "$PLAYER" ] && echo "Playing blurred..." && $PLAYER blurred.wav 2>/dev/null

echo -e "\n${BLUE}Step 5: Creating texture effect...${NC}"
# Using grain duplicate to create a granular texture (correct syntax)
$CDP_BIN/grain duplicate original.wav granular.wav 3 -b0.1 2>/dev/null || {
    echo "Trying extend zigzag instead..."
    # Correct syntax for extend zigzag mode 1
    $CDP_BIN/extend zigzag 1 original.wav granular.wav 0.1 1.9 3 0.2 2>/dev/null
}
echo "Created granular.wav"
[ -n "$PLAYER" ] && echo "Playing granular..." && $PLAYER granular.wav 2>/dev/null

echo -e "\n${GREEN}========================================${NC}"
echo -e "${GREEN}Demo Complete!${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""
echo "Generated files in $DEMO_DIR:"
ls -lh *.wav
echo ""
echo "CDP successfully demonstrated:"
echo "  ✓ Time stretching (modify speed)"
echo "  ✓ Spectral analysis (pvoc anal)"
echo "  ✓ Spectral blurring (blur avrg)"
echo "  ✓ Resynthesis (pvoc synth)"
echo "  ✓ Granular synthesis (grain count)"