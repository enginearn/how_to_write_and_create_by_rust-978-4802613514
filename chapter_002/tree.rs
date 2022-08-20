use std::{env, path};

fn tree(target: &path::PathBuf, Level: isize) {
    let files = target.read_dir().expect("read_dir failed");

    for entry in files {
        let path = entry.unwrap().path();
        for _ in 1..=Level {
            print!("|  ");
        }
        let file_name = path.file_name()
                            .unwrap()
                            .to_string_lossy();
        if path.is_dir() {
            println!("|-- <{}>", file_name);
            tree(&path, Level + 1);
            continue;
        }
        println!("|-- {}", file_name);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut target_dir = ".";
    if args.len() >= 2 {
        target_dir = &args[1];
    }

    let target = path::PathBuf::from(target_dir);
    println!("{:}", target_dir);
    tree(&target, 0);
}
