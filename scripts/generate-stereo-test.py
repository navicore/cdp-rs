#!/usr/bin/env python3
"""
Generate a stereo test WAV file for CDP testing.
Left channel: 440Hz sine wave (A4)
Right channel: 880Hz sine wave (A5)
"""

import struct
import math
import sys

def generate_stereo_wav(filename="stereo_test.wav", duration=2.0, sample_rate=44100):
    """Generate a stereo WAV file with different frequencies in each channel."""
    
    # Calculate number of samples
    num_samples = int(sample_rate * duration)
    
    # Generate samples for both channels
    samples = []
    for i in range(num_samples):
        t = i / sample_rate
        # Left channel: 440Hz
        left = math.sin(2 * math.pi * 440 * t)
        left_sample = int(left * 16383)  # Half amplitude to avoid clipping
        
        # Right channel: 880Hz  
        right = math.sin(2 * math.pi * 880 * t)
        right_sample = int(right * 16383)  # Half amplitude
        
        samples.append((left_sample, right_sample))
    
    # Write WAV file
    with open(filename, 'wb') as wav_file:
        # WAV header
        channels = 2  # Stereo
        sample_width = 2  # 16-bit
        byte_rate = sample_rate * channels * sample_width
        block_align = channels * sample_width
        data_size = num_samples * channels * sample_width
        
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
        
        # Write audio data (interleaved stereo)
        for left, right in samples:
            wav_file.write(struct.pack('<h', left))
            wav_file.write(struct.pack('<h', right))
    
    print(f"Generated {filename}: {duration}s stereo (440Hz left, 880Hz right)")
    return filename

if __name__ == "__main__":
    if len(sys.argv) > 1:
        filename = sys.argv[1]
    else:
        filename = "stereo_test.wav"
    
    generate_stereo_wav(filename)