fn main() {
    let mut _i: u8;
    for _i in 1..101 {
        let s: String = _i.to_string();
        if s.contains('3') {
            println!("{}", 'A');
        } else {
            println!("{}", s);
        }
    }
}