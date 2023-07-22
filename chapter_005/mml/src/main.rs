mod mml_parser;
mod wav_writer;

fn main() {
    // カエルの歌
    let mml = format!(
        "{}{}",
        "o5 l4 cdef edl2c", "l4 crcr crcr l8 ccdd eeff l4 ed l2c"
    );
    let notes = mml_parser::parser(mml);

    // 4分音符=120bpm で wav ファイルを作成
    let bpm = 120.0;
    wav_writer::write("flog.wav", notes, bpm);

    // きらきら星
    let mml = format!(
        "{}{}{}",
        "o5 l4 ccgg aal2g l4 ffee ddl2c",
        "l4 ggff eel2d l4 ggff eel2d",
        "l4 ccgg aal2g l4 ffee ddl2c"
    );
    let notes = mml_parser::parser(mml);

    // 4分音符=120bpm で wav ファイルを作成
    let bpm = 120.0;
    wav_writer::write("kirakira.wav", notes, bpm);

    println!("Done.");
}
