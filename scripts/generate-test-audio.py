#!/usr/bin/env python3
"""
Generate a simple test WAV file for CDP testing.
Creates a 2-second sine wave at 440Hz (A4 note).
No external dependencies required.
"""

import struct
import math
import sys

def generate_sine_wav(filename="test.wav", frequency=440, duration=2.0, sample_rate=44100):
    """Generate a simple sine wave WAV file."""
    
    # Calculate number of samples
    num_samples = int(sample_rate * duration)
    
    # Generate sine wave samples
    samples = []
    for i in range(num_samples):
        t = i / sample_rate
        value = math.sin(2 * math.pi * frequency * t)
        # Convert to 16-bit integer
        sample = int(value * 32767)
        # Clamp to 16-bit range
        sample = max(-32768, min(32767, sample))
        samples.append(sample)
    
    # Write WAV file
    with open(filename, 'wb') as wav_file:
        # WAV header
        channels = 1
        sample_width = 2  # 16-bit
        byte_rate = sample_rate * channels * sample_width
        block_align = channels * sample_width
        data_size = num_samples * sample_width
        
        # RIFF chunk
        wav_file.write(b'RIFF')
        wav_file.write(struct.pack('<I', 36 + data_size))
        wav_file.write(b'WAVE')
        
        # fmt chunk
        wav_file.write(b'fmt ')
        wav_file.write(struct.pack('<I', 16))  # Chunk size
        wav_file.write(struct.pack('<H', 1))   # Audio format (1 = PCM)
        wav_file.write(struct.pack('<H', channels))
        wav_file.write(struct.pack('<I', sample_rate))
        wav_file.write(struct.pack('<I', byte_rate))
        wav_file.write(struct.pack('<H', block_align))
        wav_file.write(struct.pack('<H', sample_width * 8))
        
        # data chunk
        wav_file.write(b'data')
        wav_file.write(struct.pack('<I', data_size))
        
        # Write audio data
        for sample in samples:
            wav_file.write(struct.pack('<h', sample))
    
    print(f"Generated {filename}: {duration}s sine wave at {frequency}Hz")
    return filename

if __name__ == "__main__":
    if len(sys.argv) > 1:
        filename = sys.argv[1]
    else:
        filename = "test.wav"
    
    generate_sine_wav(filename)