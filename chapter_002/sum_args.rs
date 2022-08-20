fn main() {
    let args = std::env::args();
    let mut total = 0.0;

    for (i, s) in args.enumerate() {
        if i == 0 {
            continue;
        }
        let n: f64 = match s.parse() {
            Ok(n) => n,
            Err(_) => 0.0,
        };
        total += n;
    }
    println!("{}", total);

    let args = std::env::args();
    for arg in args {
        println!("{}", arg);

    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);
    }
}
