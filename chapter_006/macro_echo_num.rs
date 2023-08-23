macro_rules! echo_num {
    ($x:expr) => {
        println!("{} = {}", stringify!($x), $x);
    };
}

fn main() {
    let a = 1;
    let b = 2;
    let c = 3;
    echo_num!(a);
    echo_num!(b);
    echo_num!(c);
}
