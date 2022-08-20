use std::fs;

fn main() {
    let files = fs::read_dir(".").expect("Unable to read current directory...");
    for file in files {
        let entry = file.unwrap();
        let path = entry.path();

        let file_names = path.to_str().unwrap_or("Unable to get file name...");
        println!("{}", file_names);
    }
}
