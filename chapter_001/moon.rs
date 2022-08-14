fn round_2_decimals(x: f64) -> f64 {
    (x * 100.0).round() / 100.0
}

fn main() {
    let moon: f64 = 384400.0;
    let car: f64 = 80.0;
    let train: f64 = 300.0;
    let hour: f64 = 24.0;

    let mut result: f64;
    result = moon / car / hour;
    println!("take {}days to the moon by car.", round_2_decimals(result));

    result = moon / train / hour;
    println!("take {}days to the moon by train.", round_2_decimals(result));
}