fn main() {
    let s1 = "苦しむ人にはどの日も最悪の日を過ごしている。";
    let s2 = s1.replace("苦しむ人", "楽しむ人");
    let s3 = s2.replace("最悪の日", "宴会");

    println!("置換前; {}\n置換後: {}", s1, s3);

    let s = "苦しむ人にはどの日も最悪の日を過ごしている。";
    let s = s.replace("苦しむ人", "楽しむ人");
    let s = s.replace("最悪の日", "宴会");
    println!("s: {}", s);
}
