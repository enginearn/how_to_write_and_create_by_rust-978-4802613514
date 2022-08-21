fn echo(s: &'static str) {
    println!("{}", s);
}

fn main() {
    let s = "こんにちは！🍣";
    println!("{}文字", s.chars().count());
    let ch = s.chars().nth(0).unwrap();
    println!("{}", ch);
    let ch = &s.chars().nth(6).unwrap();
    println!("{}", ch);
    let ch = s.bytes().nth(0).unwrap();
    println!("{}", ch);
    let ch = &s.bytes().nth(6).unwrap();
    println!("{}", ch);

    let s2 = "abcdefg";
    println!("{}", &s2[0..3]);
    let ch = &s[0..3];
    println!("{}", ch);

    let ch = &s[6..9];
    println!("{} len = {}", ch, s.len());

    let ch = &s[18..22];
    println!("{} len = {}", ch, s.len());

    let ss: &str = "気前よく与えるなら豊かになる";
    let so1: String = String::from(ss);
    let so2: String = ss.to_string();

    let ss2: &str = &so1;
    let ss3: &str = so1.as_str();

    println!("so1={}\nso2={}\nss2={}\nss3={}", so1, so2, ss2, ss3);
    println!("so1={:p}\nso2={:p}\nss2={:p}\nss3={:p}", &so1, &so2, &ss2, &ss3);

    let pr = "猫に小判";
    for c in pr.bytes() {
        println!("{}", c);
    }
    println!("{}文字", pr.chars().count());
    println!("バイト数{}B", pr.len());

    let pr = "窮鼠猫を噛む";
    for c in pr.chars() {
        println!("[{}]", c);
    }
    println!("{}文字", pr.chars().count());

    let pr_chars: Vec<char> = pr.chars().collect();
    for c in pr_chars.iter() {
        println!("[{}]", c);
    }
    println!("{}文字", pr_chars.len());

    let pr_vec: Vec<u8> = pr.bytes().collect();
    for c in pr_vec.iter() {
        println!("[{}]", c);
    }
    println!("{}文字", pr_vec.len());

    echo("愚かな人でも黙っていると");
    echo("賢いと見られる。");

    // let s = String::from("こんにちは！🍣");
    // echo(&s);
}
