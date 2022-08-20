use std::collections::HashMap;

const V_DATA: &str =
        "A,A,C,C,B,B,C,A,A,C,A,A,B,A,B,B,A,C,B,B,A,A,A,C,C,A,A,A,B,C";

fn main() {
    let mut c_map = HashMap::new();

    c_map.insert("A", 0);
    c_map.insert("B", 0);
    c_map.insert("C", 0);

    for w in V_DATA.split(',') {
        c_map.insert(w, c_map[w] + 1);
    }

    for (k, v) in c_map.iter() {
        println!("{}:{}", k, v);
    }
}
