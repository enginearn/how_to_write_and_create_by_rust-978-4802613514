fn main() {
    let pr = "知恵は武器よりも強い。";
    println!("先頭2文字: {}", &pr[..6]);
    println!("末尾2文字: {}", &pr[9..15]);
    println!("末尾3文字: {}", &pr[pr.len()-9..]);

    let mut s1 = String::new();
    for (i, c) in pr.chars().enumerate() {
        if i < 2 {
            s1.push(c);
            continue;
        }
        break;
    }
    println!("先頭2文字: {}", s1);

    let mut s2 = String::new();
    for (i, c) in pr.chars().enumerate() {
        if i <= pr.len() - 4 {
            s2.push(c);
            continue;
        }
        break;
    }
    println!("末尾3文字: {}", s2);

    let mut s3 = String::new();
    for (i, c) in pr.chars().enumerate() {
        if 3 <= i && i <= 4 {
            s3.push(c);
        }
    }
    println!("中2文字: {}", s3);

    let sub1: String = pr.chars().take(2).collect();
    println!("先頭2文字: {}", sub1);

    let sub2: String = pr.chars().skip(pr.len()-4).take(3).collect();
    println!("末尾3文字: {}", sub2);

    let sub3: String = pr.chars().skip(3).take(2).collect();
    println!("中2文字: {}", sub3);

    let sub4: Vec<char> = pr.chars().collect();
    let sub_chars = &sub4[sub4.len()-3..].into_iter().collect::<String>();
    println!("末尾3文字: {}", sub_chars);
}
