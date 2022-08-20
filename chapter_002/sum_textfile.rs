use std::{env, fs};

fn main() {
    let args = env::args();
    let mut total: u32 = 0;

    for (i, filename) in args.enumerate() {
        if i == 0 {
            continue;
        }
        let contents = fs::read_to_string(filename)
                            .expect("Something went wrong reading the file");

        let lines = contents.split("\n");
        for line in lines {
            let n: u32 = match line.parse() {
                Ok(n) => n,
                Err(_) => 0,
            };
            total += n;
        }
    }
    println!("{}", total);
}
