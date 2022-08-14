fn main() {
    let mut a = 1;
    let mut b = 0;
    let mut i = 1;

    while i <= 10 {
        let c = a + b;
        a = b;
        b = c;
        i += 1;
        println!("{}", b);
    }
}