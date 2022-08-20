use std::fs;

fn main() {
    let a_file = "./fizzbuzz_rust.txt";
    let b_file = "./fizzbuzz_python.txt";

    let a_content = fs::read_to_string(a_file)
                        .expect("Something went wrong reading the file");
    let b_content = fs::read_to_string(b_file)
                        .expect("Something went wrong reading the file");


    if a_content == b_content {
        println!("The files are the same.");
    } else {
        println!("The files are different.");
    }

}
