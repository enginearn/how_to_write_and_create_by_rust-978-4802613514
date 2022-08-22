static mut TAX: f32 = 0.1;

fn main() {
    unsafe {
        println!("Price: {}", TAX * 3000.0);

        TAX = 0.2;
        println!("Price: {}", TAX * 3000.0);
    }
}
