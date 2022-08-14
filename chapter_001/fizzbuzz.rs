fn main() {
    for i in 1..101 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("fizzbuzz");
        } else if i % 3 == 0 {
            println!("fizz");
        } else if i % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", i);
        }
    }

    for i in 1..101 {
        match (i % 3 == 0, i % 5 == 0) {
            (true, true) => println!("match! fizzbuzz!"),
            (true, false) => println!("match! fizz!"),
            (false, true) => println!("match! buzz!"),
            (false, false) => println!("match! {}", i),
        }
    }
}