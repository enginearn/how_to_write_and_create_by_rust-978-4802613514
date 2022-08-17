fn main() {
    let u_vec: Vec<u32> = vec![1, 2, 3, 4, 5];
    for u in u_vec {
        println!("{}", u);
    }

    let s_vec: Vec<String> = vec![
        String::from("a"),
        String::from("b"),
        String::from("c"),
        String::from("d"),
        String::from("e"),
    ];
    for s in s_vec {
        println!("{}", s);
    }
}