fn encrypt(text: &str, shift: i16) -> String {
    let code_a = 'A' as i16;
    let code_z = 'Z' as i16;
    let mut result = String::new();
    let text = text.to_ascii_uppercase();
    for c in text.chars() {
        let mut code = c as i16;
        if code_a <= code && code <= code_z {
            code = (code - code_a + shift + 26) % 26 + code_a;
            }
            result.push((code as u8) as char);
        }
        return result;
}

fn main() {
    println!("Please enter a text to encrypt:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)
                    .expect("Failed to read line...");
    let text = input.trim()
                    .to_string();

    let enc = encrypt(&text, 3);
    let dec = encrypt(&enc, -3);
    println!("{} => {}", enc, dec);
}

// fn caesar_enc(text: &str, key: i32) -> String {
//     let mut result = String::new();
//     for c in text.chars() {
//         let mut new_c = c;
//         if c.is_alphabetic() {
//             new_c = c.to_ascii_uppercase();
//             println!("new_c: {}", new_c);
//             new_c = ((new_c as u8 - b'A' as u8 + key as u8 + 26) % 26 + b'A' as u8) as char;
//             result.push(new_c);
//         } else {
//             result.push(c);
//         }
//     }
//     result
// }

// fn caesar_dec(text: &str, key: i32) -> String {
//     return caesar_enc(text, -key);
// }

// fn main() {
//     println!("Please enter a text to encrypt:");
//     let mut input = String::new();
//     std::io::stdin().read_line(&mut input)
//                     .expect("Failed to read line...");
//     let text = input.trim()
//                     .to_string();
//     // print!("Please enter a key: ");
//     let enc = caesar_enc(&text, 3);
//     let dec = caesar_dec(&enc, 3);
//     println!("{}", enc);
//     println!("{}", dec);
// }