fn multiplication(a: i32, b: i32) -> i32 {
    a * b
}

fn main() {
    println!("Enter 2 numbers to multiply a and b\neg;0: 3 6: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)
                    .expect("Failed to read line...");
    let numbers: Vec<i32> = input.trim()
                                .split_whitespace()
                                .map(|x| x.parse().unwrap())
                                .collect();
    let a = numbers[0];
    let b = numbers[1];
    println!("{} * {} = {}", a, b, multiplication(a, b));
}