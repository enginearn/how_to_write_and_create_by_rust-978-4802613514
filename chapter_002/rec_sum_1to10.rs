fn sum_1to10(n: i32) -> i32 {
    if n <= 0 {
        return 0;
    }
    return sum_1to10(n - 1) + n;
}

fn main() {
    println!("{}", sum_1to10(10));
}
