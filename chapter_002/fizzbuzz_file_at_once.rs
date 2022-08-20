use std::fs::File;
use std::io::Write;

fn get_fizzbuzz(max: u32) -> String {
    let mut result = String::new();
    for i in 1..max {
        if (i % 3 == 0) && (i % 5 == 0) {
            result.push_str("FizzBuzz\n");
        } else if i % 3 == 0 {
            result.push_str("Fizz\n");
        } else if i % 5 == 0 {
            result.push_str("Buzz\n");
        } else {
            result.push_str(&i.to_string());
            result.push_str("\n");
        }
    }
    result // return the result
}

fn main() {
    let file_name = "fizzbuzz_result_once.txt";
    let data = get_fizzbuzz(100);

    let mut fp = File::create(file_name).unwrap();

    let bytes = data.as_bytes();
    fp.write_all(bytes).unwrap();
}
