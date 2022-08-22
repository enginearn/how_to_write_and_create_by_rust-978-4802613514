use std::fs;
use std::fs::File;
use std::io::Write;
use encoding_rs;

fn save_sjis(file_name: &str, text: &str) {
    let (enc, _, _) = encoding_rs::SHIFT_JIS.encode(text);
    let buf = enc.into_owned();
    let mut file = File::create(file_name).expect("file not created");
    file.write(&buf[..]).expect("file not written");
}

fn load_sjis(file_name: &str) -> String {
    let buf = fs::read(file_name).expect("file not read");
    let (dec, _, _) = encoding_rs::SHIFT_JIS.decode(&buf);
    return dec.into_owned();
}

fn main() {
    let file_name = "test-sjis.txt";
    save_sjis(file_name, "こっそりRustで書き込みます。");
    let s = load_sjis(file_name);
    println!("{}", s);
}
