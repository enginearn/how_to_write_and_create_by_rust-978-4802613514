macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
        let mut map = std::collections::HashMap::new();
        $( map.insert($key, $val); )*
        map
    }}
}

fn main() {
    let map = hashmap!['a' => 1, 'b' => 2, 'c' => 3];
    println!("{:?}", map);

    let week = hashmap!['M' => "Monday", 'T' => "Tuesday", 'W' => "Wednesday", 'R' => "Thursday", 'F' => "Friday", 'S' => "Saturday", 'U' => "Sunday"];
    for (key, value) in &week {
        println!("{}: {}", key, value);
    }
    println!("{:?}", week);
}
