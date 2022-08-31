fn main() {
    let age = 8;
    let age_str = match age {
        0 => "baby",
        1..=5 => "child",
        6..=12 => "kid",
        13..=19 => "teenager",
        20..=30 => "young",
        31..=60 => "adult",
        _ => "old",
    };
    println!("{} is {}", age, age_str);
}
