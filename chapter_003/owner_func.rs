fn show_message(message: String) -> String {
    println!("{}", message);
    message
}

fn add_quote(msg: &mut String) {
    msg.insert(0, '『');
    msg.push('』');
}

fn x2(arg: &mut i32) {
    *arg *= 2;
}

fn main() {
    let mut g1 = String::from("過ちを見過ごす人は美しい");
    g1 = show_message(g1);
    println!("{}", g1);
    g1 = String::from("怪物と戦う者は、その際自分が怪物にならぬように気をつけるがいい。
                    長い間、深淵をのぞきこんでいると、深淵もまた、君をのぞきこむ。");
    add_quote(&mut g1);
    println!("{}", g1);

    let mut v1 = 16;
    x2(&mut v1);
    println!("v1: {}", v1);
}
