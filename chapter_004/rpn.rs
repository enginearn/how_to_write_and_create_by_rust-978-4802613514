use std::io;

fn main() {
    let mut stack: Vec<f64> = vec![];
    loop {
        print!("RPN > ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.split_whitespace();
        for token in input {
            let token = token.trim();
            match token.parse::<f64>() {
                Ok(v) => { stack.push(v); continue; },
                Err(_) => 0.0,
            };

            let b = stack.pop().expect("Stack underflow");
            let a = stack.pop().expect("Stack underflow");
            match token {
                "+" => stack.push(a + b),
                "-" => stack.push(a - b),
                "*" => stack.push(a * b),
                "/" => stack.push(a / b),
                "%" => stack.push(a % b),
                "^" => stack.push(a.powf(b)),
                _ => println!("Unknown operator: {}", token),
            }
        }

    println!("{}", stack.pop().unwrap());
    }
}

