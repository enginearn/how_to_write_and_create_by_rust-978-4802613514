use rand::seq::SliceRandom;

fn main() {
    let mut u_vec: Vec<u32> = Vec::new();
    for i in 1..75 {
        u_vec.push(i);
    }

    let mut rng = rand::thread_rng();
    u_vec.shuffle(&mut rng);

    for i in 0..25 {
        if i == 12 {
            print!(" ★  ");
        } else {
            print!("{:3} ", u_vec[i]);
        }
        if i % 5 == 4 {
            println!("");
        }
    }
}