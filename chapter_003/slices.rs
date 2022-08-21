fn sum_slice(item: &[i32]) -> i32 {
    let mut sum = 0;
    for i in item {
        sum += i;
    }
    sum
}

fn main() {
    let a = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let a_slice = &a[..3];
    println!("{:?}", a_slice);
    println!("{:?}", &a[3..5]);
    println!("{:?}", &a[..]);

    println!("{}", sum_slice(&a[..]));

    let b = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("{}", sum_slice(&b[..]));
}
