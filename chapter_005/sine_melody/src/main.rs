use std::f32::consts::PI;
use std::io::{Write, Seek};
use hound::WavWriter;

const SAMPLE_RATE: u32 = 44100;
const BPM: f32 = 120.0; // beats per minute

#[allow(unused_variables)]
fn main() {
    // WavWriter
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE as u32,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = WavWriter::create("sine_melody.wav", spec).unwrap();

    // sine wave
    let (c4, d4, e4, f4) = (261.626, 293.665, 329.628, 349.228);
    let (g4, a4, b4, c5) = (391.995, 440.00, 493.883, 523.251);
    let l4 = ((60.0 / BPM) * SAMPLE_RATE as f32) as u32; // length of 1/4 note
    let l2 = l4 * 2; // length of 1/2 note

    write_tone(&mut writer, c4, l4);
    write_tone(&mut writer, d4, l4);
    write_tone(&mut writer, e4, l2);
    write_tone(&mut writer, c4, l4);
    write_tone(&mut writer, d4, l4);
    write_tone(&mut writer, e4, l2);
    write_tone(&mut writer, g4, l4);
    write_tone(&mut writer, e4, l4);
    write_tone(&mut writer, d4, l4);
    write_tone(&mut writer, c4, l4);
    write_tone(&mut writer, d4, l4);
    write_tone(&mut writer, e4, l4);
    write_tone(&mut writer, d4, l2);
}

fn write_tone<W>(writer: &mut WavWriter<W>, tone: f32, length: u32)
    where W: Write + Seek
{
    for i in 0..length {
        let t = i as f32 / SAMPLE_RATE as f32;
        let sample = (t * tone * 2.0 * PI).sin();
        writer.write_sample((sample * i16::max_value() as f32) as i16).unwrap();
    }
}
