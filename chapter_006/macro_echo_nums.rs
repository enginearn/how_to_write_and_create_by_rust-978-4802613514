#[macro_export]
macro_rules! echo_nums {
    // loop through comma-separated expressions
    ($($num:expr),*) => {
        {
            $(
                println!("num: {}", $num);
            )*
        }
    };
}

fn main() {
    echo_nums!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
}
