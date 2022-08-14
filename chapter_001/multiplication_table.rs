fn main() {
    let mut i =1;
    while i < 10 {
        for a in 1..10 {
            for b in 1..10 {
                    println!("{} x {} = {}", a, b, a * b);
                    i += 1;
                    continue;
                }
                println!("");
            }
    }

    println!("==========================================");

    for i in 1..10 {
        let s = (1..10)
                .map(|j| format!("{:3}", i * j))
                .collect::<Vec<String>>().join(" ");
        println!("{}", s);
    }
}