use std::fs::{self, File};
use std::io::{Write, BufWriter};

fn main() {
    let output_file = "fizzbuzz_result.txt";
    {
        let fp = File::create(output_file).unwrap();
        let mut writer = BufWriter::new(fp);

        for i in 1..=100 {
            if i % 15 == 0 {
                writeln!(writer, "fizzbuzz").unwrap();
            } else if i % 3 == 0 {
                writeln!(writer, "fizz").unwrap();
            } else if i % 5 == 0 {
                writeln!(writer, "buzz").unwrap();
            } else {
                writeln!(writer, "{}", i).unwrap();
            }
        }
    }
    println!("{} is created", output_file);
    let res = fs::read_to_string(output_file).unwrap();
    println!("{}", res);
}
