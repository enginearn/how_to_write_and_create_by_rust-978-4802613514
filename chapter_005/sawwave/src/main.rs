use hound;

const SAMPLE_RATE: f32 = 44100.0;

fn main() {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE as u32,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };
    let mut fw = hound::WavWriter::create("sawwave.wav", spec).unwrap();

    // create sawwave
    let mut wav: Vec<f32> = vec![];
    let bpm = 120;
    wav.extend(sawtooth_wave(60, calc_len(bpm, 4), 0.5));
    wav.extend(sawtooth_wave(64, calc_len(bpm, 4), 0.5));
    wav.extend(sawtooth_wave(67, calc_len(bpm, 4), 0.5));

    // write wav
    for v in wav.into_iter() {
        fw.write_sample(v).unwrap();
        println!("{}", v);
    }
}

fn noteno_to_hz(no: i32) -> f32 {
    440.0 * 2.0f32.powf((no - 69) as f32 / 12.0)
}

fn calc_len(bpm: usize, beat: usize) -> usize {
    let base_len = (60.0 / bpm as f32) * SAMPLE_RATE;
    ((4.0 / beat as f32) * base_len) as usize
}

fn sawtooth_wave(noteno: i32, len: usize, gain: f32) -> Vec<f32> {
    let tone = noteno_to_hz(noteno); // 周波数を求める
    let form_sample = SAMPLE_RATE / tone; // 1周期のサンプル数
    let mut wav: Vec<f32> = vec![0.0; len];
    for i in 0..len {
        let pif = (i as f32 / form_sample) % 1.0; // 1周期の波形を作る
        wav[i] = pif * 2.0 - 1.0; // -1.0～1.0の範囲にする
    }
    wav.into_iter().map(|v| (v * gain) as f32).collect() // 音量を調整する
}
