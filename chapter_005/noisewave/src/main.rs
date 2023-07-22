use hound;
use rand::prelude::*;

const SAMPLE_RATE: f32 = 44100.0;

fn main() {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE as u32,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };
    let mut writer = hound::WavWriter::create("noise.wav", spec).unwrap();

    let mut wav: Vec<f32> = Vec::new();
    let bpm = 120;

    wav.extend(noise(2.0, -1.0, calc_len(bpm, 2)));
    wav.extend(noise(0.2, 0.8, calc_len(bpm, 2)));
    wav.extend(noise(0.8, -1.0, calc_len(bpm, 2)));

    for v in wav.into_iter() {
        writer.write_sample(v).unwrap();
        println!("{}", v);
    }
}

fn calc_len(bpm: usize, beat: usize) -> usize {
    ((60.0 / bpm as f32) * (4.0 / beat as f32) * SAMPLE_RATE) as usize
}

fn noise(range: f32, offset: f32, len: usize) -> Vec<f32> {
    let mut wav: Vec<f32> = vec![0.0; len];
    let mut rng = rand::thread_rng();
    for i in 0..len {
        wav[i] = rng.gen::<f32>() * range + offset;
    }
    let gain = 0.5;
    wav.into_iter().map(|v| (v * gain) as f32).collect()
}
