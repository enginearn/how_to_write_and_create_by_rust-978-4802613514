use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("A", 30);
    map.insert("B", 40);

    if map.get("D") == None {
        println!("D is not found...");
    } else {
        println!("D={}", map["D"]);
    }

    map.insert("D", 50);

    match map.get("D") {
        Some(v) => println!("D={}", v),
        None => println!("D is not found..."),
    }
}
