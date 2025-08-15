use cdp_core::fft::FftProcessor;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_fft(c: &mut Criterion) {
    let sizes = [256, 512, 1024, 2048, 4096];
    
    for size in sizes {
        c.bench_function(&format!("fft_forward_{}", size), |b| {
            let mut processor = FftProcessor::new(size).unwrap();
            let input: Vec<f32> = (0..size).map(|i| (i as f32).sin()).collect();
            let mut output = vec![num_complex::Complex32::new(0.0, 0.0); size];
            
            b.iter(|| {
                processor.forward(black_box(&input), black_box(&mut output)).unwrap();
            });
        });
        
        c.bench_function(&format!("fft_roundtrip_{}", size), |b| {
            let mut processor = FftProcessor::new(size).unwrap();
            let input: Vec<f32> = (0..size).map(|i| (i as f32).sin()).collect();
            let mut spectrum = vec![num_complex::Complex32::new(0.0, 0.0); size];
            let mut output = vec![0.0; size];
            
            b.iter(|| {
                processor.forward(black_box(&input), black_box(&mut spectrum)).unwrap();
                processor.inverse(black_box(&mut spectrum), black_box(&mut output)).unwrap();
            });
        });
    }
}

criterion_group!(benches, benchmark_fft);
criterion_main!(benches);