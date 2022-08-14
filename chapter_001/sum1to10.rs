fn main() {
    let mut sum =0;
    for i in 1..=10 {
        sum += i;
    }
    println!("sum(for) is {}", sum);

    let nums = [1,2,3,4,5,6,7,8,9,10];
    let mut sum = 0;
    for i in nums {
        sum += i;
    }
    println!("sum(array) is {}", sum);

    let nums = vec![1,2,3,4,5,6,7,8,9,10];
    let mut sum = 0;
    for i in nums {
        sum += i;
    }
    println!("sum(vec) is {}", sum);
}