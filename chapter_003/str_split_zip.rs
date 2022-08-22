fn main() {
    let zip_code = "150-0002";

    println!("-- slice --");
    println!("前半: {}", &zip_code[..=2]);
    println!("後半: {}", &zip_code[4..]);

    println!("-- split_at --");
    let (zip1, zip2) = zip_code.split_at(3);
    let (zip2, zip3) = zip2.split_at(1);
    println!("前半: {}", zip1);
    println!("記号: {}", zip2);
    println!("後半: {}", zip3);

    println!("-- split_off --");
    let mut zip1 = String::from(zip_code);
    let mut zip2 = zip1.split_off(3);
    let zip3 = zip2.split_off(1);
    println!("前半: {}", zip1);
    println!("記号: {}", zip2);
    println!("後半: {}", zip3);

    println!("-- split --");
    let zip_a: Vec<&str> = zip_code.split("-").collect();
    println!("前半: {}", zip_a[0]);
    println!("後半: {}", zip_a[1]);
}
