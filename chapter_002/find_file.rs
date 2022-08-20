use std::{env, path};

fn find_file(target: &path::PathBuf, target_file: &str) {
    let files = target.read_dir().expect("read_dir failed...");
    for dir_entry in files {
        let path = dir_entry.expect("dir_entry failed...").path();
        if path.is_dir() {
            find_file(&path, target_file);
            continue;
        }
        let filename = path.file_name()
                            .expect("file_name failed...")
                            .to_string_lossy();
        if None == filename.find(target_file) {
            continue;
        }
        println!("{}", path.to_string_lossy());
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <path> <filename>", args[0]);
        return;
    }

    let target_dir = &args[1];
    let target_file = &args[2];

    let target = path::PathBuf::from(target_dir);
    find_file(&target, target_file);
}
