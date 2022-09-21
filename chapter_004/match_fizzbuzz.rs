fn main() {
    for i in 1..101 {
        match (i % 3, i % 5) {
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            (_, _) => println!("{}", i),
        }
    }

    // match guard
    for i in 1..=100 {
        match i {
            n if n % 15 == 0 => println!("FizzBuzz"),
            n if n % 3 == 0 => println!("Fizz"),
            n if n % 5 == 0 => println!("Buzz"),
            n => println!("{}", n),
        }
    }
}
