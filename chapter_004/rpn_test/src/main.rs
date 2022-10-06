fn main() {
    let src = "1 2 + 3 4 + *".to_string();
    let ans = rpn_calcurator::eval(src).unwrap();
    println!("{}", ans);
}
