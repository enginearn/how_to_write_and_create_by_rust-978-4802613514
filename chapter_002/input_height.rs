fn input_str() -> String {
    let mut s = String::new();
    std::io::stdin()
            .read_line(&mut s)
            .expect("Failed to read line");
    s.trim_end().to_string()
}

fn input_f(def: f64) -> f64 {
    let s = input_str();
    match s.trim().parse::<f64>() {
        Ok(n) => n,
        Err(_) => def,
    }
}

fn main() {
    let mut height: f64 = 0.0;
    loop {
        println!("Enter your height (cm): ");
        height = input_f(height);
        if height > 0.0 {
            break;
        }
        println!("Invalid height");
    }
    let weight = 22.0 * (height / 100.0).powf(2.0);
    println!("Your standard weight is {:.2} kg", weight);
}
