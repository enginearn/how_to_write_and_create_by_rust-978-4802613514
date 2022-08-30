fn main() {
    for i in 1..=7 {
        if i % 2 == 0 {
            println!("{}", i);
        }
    }

    let array = [1, 2, 3, 4, 5, 6, 7];
    for a in array.iter() {
        println!("{}", a);
    }
    println!("len: {}", array.len());

    let array = [
        "Apple".to_string(),
        "Banana".to_string(),
        "Cherry".to_string(),
        "Durian".to_string(),
        "Elderberry".to_string(),
    ];
    for a in array.iter() {
        println!("{}", a);
    }
    println!("len: {}", array.len());
}
