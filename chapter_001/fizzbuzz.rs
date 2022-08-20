fn main() {
    for i in 1..101 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
    }

    for i in 1..101 {
        match (i % 3 == 0, i % 5 == 0) {
            (true, true) => println!("match! FizzBuzz!"),
            (true, false) => println!("match! Fizz!"),
            (false, true) => println!("match! Buzz!"),
            (false, false) => println!("{}", i),
        }
    }
}
