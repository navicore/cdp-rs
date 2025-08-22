use cdp_distort::{divide, multiply, overload, ClipType};
use hound::{SampleFormat, WavSpec, WavWriter};
use std::fs;
use tempfile::tempdir;

fn create_test_wav(path: &std::path::Path, samples: Vec<f32>) {
    let spec = WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 32,
        sample_format: SampleFormat::Float,
    };

    let mut writer = WavWriter::create(path, spec).unwrap();
    for sample in samples {
        writer.write_sample(sample).unwrap();
    }
    writer.finalize().unwrap();
}

#[test]
fn test_multiply_basic() {
    let dir = tempdir().unwrap();
    let input_path = dir.path().join("input.wav");
    let output_path = dir.path().join("output.wav");

    // Create test signal
    let samples: Vec<f32> = (0..44100)
        .map(|i| (i as f32 * 2.0 * std::f32::consts::PI * 440.0 / 44100.0).sin() * 0.5)
        .collect();
    create_test_wav(&input_path, samples);

    // Apply multiplication
    multiply(&input_path, &output_path, 2.0, 0.5).unwrap();

    // Verify output exists
    assert!(output_path.exists());
    let metadata = fs::metadata(&output_path).unwrap();
    assert!(metadata.len() > 0);
}

#[test]
fn test_divide_basic() {
    let dir = tempdir().unwrap();
    let input_path = dir.path().join("input.wav");
    let output_path = dir.path().join("output.wav");

    // Create test signal
    let samples: Vec<f32> = (0..44100)
        .map(|i| (i as f32 * 2.0 * std::f32::consts::PI * 440.0 / 44100.0).sin() * 0.5)
        .collect();
    create_test_wav(&input_path, samples);

    // Apply division
    divide(&input_path, &output_path, 2, 0.5).unwrap();

    // Verify output exists
    assert!(output_path.exists());
    let metadata = fs::metadata(&output_path).unwrap();
    assert!(metadata.len() > 0);
}

#[test]
fn test_overload_hard_clip() {
    let dir = tempdir().unwrap();
    let input_path = dir.path().join("input.wav");
    let output_path = dir.path().join("output.wav");

    // Create test signal with some peaks
    let samples: Vec<f32> = (0..44100)
        .map(|i| {
            let t = i as f32 / 44100.0;
            (t * 2.0 * std::f32::consts::PI * 440.0).sin() * (0.3 + 0.7 * (t * 2.0).sin())
        })
        .collect();
    create_test_wav(&input_path, samples);

    // Apply hard clipping
    overload(&input_path, &output_path, 0.5, 2.0, ClipType::Hard).unwrap();

    // Verify output exists
    assert!(output_path.exists());
}

#[test]
fn test_overload_soft_clip() {
    let dir = tempdir().unwrap();
    let input_path = dir.path().join("input.wav");
    let output_path = dir.path().join("output.wav");

    // Create test signal
    let samples: Vec<f32> = (0..44100)
        .map(|i| (i as f32 * 2.0 * std::f32::consts::PI * 440.0 / 44100.0).sin() * 0.8)
        .collect();
    create_test_wav(&input_path, samples);

    // Apply soft clipping
    overload(&input_path, &output_path, 0.7, 1.5, ClipType::Soft).unwrap();

    // Verify output exists and check it's properly normalized
    assert!(output_path.exists());

    let reader = hound::WavReader::open(&output_path).unwrap();
    let output_samples: Vec<f32> = reader
        .into_samples::<f32>()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    // Check all samples are within bounds
    for sample in output_samples {
        assert!(sample.abs() <= 1.0);
    }
}

#[test]
fn test_overload_tube_saturation() {
    let dir = tempdir().unwrap();
    let input_path = dir.path().join("input.wav");
    let output_path = dir.path().join("output.wav");

    // Create test signal
    let samples: Vec<f32> = (0..44100)
        .map(|i| (i as f32 * 2.0 * std::f32::consts::PI * 440.0 / 44100.0).sin() * 0.5)
        .collect();
    create_test_wav(&input_path, samples);

    // Apply tube saturation
    overload(&input_path, &output_path, 0.6, 3.0, ClipType::Tube).unwrap();

    // Verify output exists
    assert!(output_path.exists());
}

#[test]
fn test_multiply_extreme_values() {
    let dir = tempdir().unwrap();
    let input_path = dir.path().join("input.wav");
    let output_path = dir.path().join("output.wav");

    // Create test signal with extreme values
    let samples = vec![1.0, -1.0, 0.5, -0.5, 0.0];
    create_test_wav(&input_path, samples);

    // Apply maximum multiplication
    multiply(&input_path, &output_path, 16.0, 1.0).unwrap();

    // Verify output is normalized
    let reader = hound::WavReader::open(&output_path).unwrap();
    let output_samples: Vec<f32> = reader
        .into_samples::<f32>()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    for sample in output_samples {
        assert!(sample.abs() <= 1.0);
    }
}
