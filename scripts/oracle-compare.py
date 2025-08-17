#!/usr/bin/env python3
"""
Compare CDP and Rust outputs, ignoring timestamp differences.
Returns 0 if files match (except timestamps), 1 otherwise.
"""

import sys
import struct

def compare_files(file1_path, file2_path):
    """Compare two CDP WAV files, ignoring timestamp fields."""
    
    with open(file1_path, 'rb') as f1, open(file2_path, 'rb') as f2:
        data1 = f1.read()
        data2 = f2.read()
        
        if len(data1) != len(data2):
            print(f"ERROR: Size mismatch: {file1_path}={len(data1)}, {file2_path}={len(data2)}")
            return False
            
        # CDP timestamp fields that we expect to differ
        timestamp_ranges = [
            (0x30, 0x34),  # PEAK chunk timestamp (4 bytes)
            (0x77, 0x90),  # LIST/note timestamp hex string (variable, be generous)
        ]
        
        # Find all differences
        diffs = []
        for i, (b1, b2) in enumerate(zip(data1, data2)):
            if b1 != b2:
                # Check if this byte is in a timestamp field
                in_timestamp = False
                for start, end in timestamp_ranges:
                    if start <= i < end:
                        in_timestamp = True
                        break
                        
                if not in_timestamp:
                    diffs.append(i)
                    
        if diffs:
            print(f"ERROR: Non-timestamp differences found at byte positions:")
            for d in diffs[:10]:  # Show first 10 differences
                print(f"  0x{d:04x} ({d}): {file1_path}=0x{data1[d]:02x} {file2_path}=0x{data2[d]:02x}")
            if len(diffs) > 10:
                print(f"  ... and {len(diffs)-10} more differences")
            return False
            
        # Verify structure and audio data
        # Check key chunk positions match
        riff1 = data1[:4]
        riff2 = data2[:4]
        if riff1 != b'RIFF' or riff2 != b'RIFF':
            print("ERROR: Not valid RIFF files")
            return False
            
        # Find and compare audio data chunk
        data_marker = b'data'
        data_pos1 = data1.find(data_marker, 100)  # Start search after headers
        data_pos2 = data2.find(data_marker, 100)
        
        if data_pos1 != data_pos2:
            print(f"ERROR: Data chunk position mismatch: {data_pos1} vs {data_pos2}")
            return False
            
        if data_pos1 > 0:
            # Read data chunk size
            size_start = data_pos1 + 4
            data_size1 = struct.unpack('<I', data1[size_start:size_start+4])[0]
            data_size2 = struct.unpack('<I', data2[size_start:size_start+4])[0]
            
            if data_size1 != data_size2:
                print(f"ERROR: Audio data size mismatch: {data_size1} vs {data_size2}")
                return False
                
            # Compare actual audio samples
            audio_start = data_pos1 + 8
            audio_end = audio_start + data_size1
            
            audio1 = data1[audio_start:audio_end]
            audio2 = data2[audio_start:audio_end]
            
            if audio1 != audio2:
                # Find first difference in audio
                for i, (a1, a2) in enumerate(zip(audio1, audio2)):
                    if a1 != a2:
                        print(f"ERROR: Audio data mismatch at sample byte {i}")
                        print(f"  Position 0x{audio_start+i:04x}: {a1:02x} vs {a2:02x}")
                        break
                return False
                
        print(f"SUCCESS: Files match (ignoring timestamps)")
        return True

if __name__ == '__main__':
    if len(sys.argv) != 3:
        print("Usage: oracle-compare.py <file1> <file2>")
        sys.exit(1)
        
    if compare_files(sys.argv[1], sys.argv[2]):
        sys.exit(0)
    else:
        sys.exit(1)