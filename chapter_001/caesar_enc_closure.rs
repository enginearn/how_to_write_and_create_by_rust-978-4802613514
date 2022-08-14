fn encrypt(text: &str, key: i16) -> String {
    let text = text.to_lowercase();
    let a = 'a' as i16;
    let is_az = |c| 'a' <= c && c <= 'z';
    let conv = |c| (((c - a + key + 26) % 26 + a) as u8) as char;
    let enc = |c| if is_az(c) { conv(c as i16) } else { c };
    return text.chars().map(|c| enc(c)).collect();
}

fn main() {
    let text = "Hello, world!";
    let key = 3;
    let enc = encrypt(text, key);
    let dec = encrypt(&enc, -key);
    println!("{} -> {}" , enc, dec);
}