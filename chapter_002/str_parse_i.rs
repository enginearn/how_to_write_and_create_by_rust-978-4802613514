fn main() {
    let s = "365";

    let i: i32 = match s.parse() {
        Ok(n) => n,
        Err(_) => 0,
    };
    println!("{}", i);

    let s = "365abc";
    let i: i32 = s.parse().unwrap_or(0);
    println!("{}", i);
}
