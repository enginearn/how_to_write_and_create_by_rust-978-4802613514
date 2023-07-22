use hound::WavWriter;
use std::f32::consts::PI;
use std::io::{Seek, Write};

const SAMPLE_RATE: f32 = 44100.0;

#[derive(Debug)]
pub struct Note {
    pub no: i32,
    pub len: i32,
}

pub fn write(filename: &str, notes: Vec<Note>, bpm: f32) {
    // 16bit, 1ch, 44100Hz
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE as u32,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = WavWriter::create(filename, spec).unwrap();

    for note in notes.into_iter() {
        // 4分音符の長さを計算
        let len = (4.0 / note.len as f32 * (60.0 / bpm) * SAMPLE_RATE) as i32;

        // 音の高さを計算
        let tone = if note.no >= 0 {
            440.0 * 2.0f32.powf((note.no - 69) as f32 / 12.0)
        } else {
            0.0
        };
        write_tone(&mut writer, tone, len);
    }
}

fn write_tone<W>(writer: &mut WavWriter<W>, tone: f32, len: i32)
where
    W: Write + Seek
{
    for i in 0..len {
        let t = i as f32 / SAMPLE_RATE;
        let v = (t * tone * 2.0 * PI).sin();
        writer.write_sample((v * i16::MAX as f32) as i16).unwrap();
    }
}

// Library test
#[cfg(test)]
mod wav_writer_tests {
    use super::*;

    #[test]
    fn notes_test() {
        let notes = vec![
            Note { no: 60, len: 4 },
            Note { no: 62, len: 4 },
            Note { no: 64, len: 4 },
            Note { no: 65, len: 4 },
            Note { no: 67, len: 4 },
            Note { no: 69, len: 4 },
            Note { no: 71, len: 4 },
            Note { no: 72, len: 4 },
        ];
        write("test.wav", notes, 120.0);
    }
}
