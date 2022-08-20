fn main() {
    let x = 100u8;
    let y = -100i128;
    let z = -100_000;

    println!("x: {}, y: {}, z: {}", x, y, z);

    let a = 'a';
    let b = b'a';
    let c = '\u{1F600}';

    println!("a: {}, b: {}, c: {}", a, b, c);

    let h1 = 0xFF;
    let h2 = 0o377;
    let h3 = 0b1111_1111;
    let h4 = 0b1111_1111_1111_1111_1111_1111_1111_1111_1111 as i64;

    println!("h1: {}, h2: {}, h3: {}, h4: {}", h1, h2, h3, h4);

    let f1 = 3.14;
    let f2 = 3.5f32;
    let f3 = 3.5f64;
    let f4 = 3.5f32 as f64;
    let f5 = 3.5f64 as f32;
    let f6 = 10.5e+8;

    println!("f1: {}, f2: {}, f3: {}, f4: {}, f5: {}, f6: {}", f1, f2, f3, f4, f5, f6);
}
