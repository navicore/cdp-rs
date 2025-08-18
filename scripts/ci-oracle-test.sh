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

# Test 2: housekeep chans (extract channel)
echo "Testing: housekeep chans (extract channel)..."

# Generate stereo test file if needed
if [ ! -f "stereo.wav" ]; then
    echo "Generating stereo test audio..."
    cat > generate_stereo.py << 'EOF'
import struct
import math

def generate_stereo():
    sample_rate = 44100
    duration = 2.0
    samples = []
    
    for i in range(int(sample_rate * duration)):
        t = i / sample_rate
        left = int(math.sin(2 * math.pi * 440 * t) * 16383)
        right = int(math.sin(2 * math.pi * 880 * t) * 16383)
        samples.append((left, right))
    
    with open("stereo.wav", 'wb') as f:
        data_size = len(samples) * 4
        f.write(b'RIFF')
        f.write(struct.pack('<I', 36 + data_size))
        f.write(b'WAVE')
        f.write(b'fmt ')
        f.write(struct.pack('<I', 16))
        f.write(struct.pack('<HHI', 1, 2, sample_rate))
        f.write(struct.pack('<IHH', sample_rate * 4, 4, 16))
        f.write(b'data')
        f.write(struct.pack('<I', data_size))
        for l, r in samples:
            f.write(struct.pack('<hh', l, r))

generate_stereo()
EOF
    python3 generate_stereo.py
    rm generate_stereo.py
fi

# Test channel extraction
rm -f stereo_c*.wav
../target/release/housekeep chans 1 stereo.wav 1
mv stereo_c1.wav rust_stereo_c1.wav
../build/cdp/NewRelease/housekeep chans 1 stereo.wav 1 > /dev/null 2>&1
mv stereo_c1.wav cdp_stereo_c1.wav

if python3 ../scripts/oracle-compare.py rust_stereo_c1.wav cdp_stereo_c1.wav; then
    echo "✓ housekeep chans: PASSED"
else
    echo "✗ housekeep chans: FAILED"
    exit 1
fi

# Clean up
rm -f rust_stereo_c1.wav cdp_stereo_c1.wav

echo "=== All Oracle Tests Passed ==="