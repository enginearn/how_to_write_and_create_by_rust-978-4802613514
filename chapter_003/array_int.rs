// extern crate rand;
// use crates::rand::Rng;

fn print_array(e: &[i32; 5]) {
    println!("{:?}", e);
    println!("len = {}", e.len());
}

fn main() {
    // let mut rng = rand::thread_rng();
    // let mut array = [0; 10];
    // for i in 0..10 {
    //     array[i] = rng.gen_range(0, 100);
    // }
    // println!("{:?}", array);

    let points_1 = [80, 40, 50, 90, 70];
    println!("{:?}", points_1);
    println!("len = {}", points_1.len());

    let points_2: [i32; 5] = [60, 45, 57, 90, 63];
    print_array(&points_2);

    let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
    for i in 0..months.len() {
        println!("{}", months[i]);
    }
}
