fn main() {
    let mut v = 10;

    set_value(&mut v);

    println!("v is {}", v);
}

fn set_value(v: &mut i32) {
    *v = 200;
}