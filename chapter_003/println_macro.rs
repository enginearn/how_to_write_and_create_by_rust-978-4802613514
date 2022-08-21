fn main() {
    let s = "人生を楽しむ秘訣は普通にこだわらないこと。\n普通と言われる人生を送る人間なんて、一人としていやしない。\nいたらお目にかかりたいものだ。";
    println!("{}", s);
    println!("====================================================");

    let year = 2025;
    let month = 12;
    let day = 31;

    println!("JP: {}/{}/{}", year, month, day);
    println!("US: {}/{}/{}", day, month, year);
    println!("EU: {}/{}/{}", day, month, year);
    println!("{year}年{month}月{day}日", year = year, month = month, day = day);
    println!("====================================================");

    println!("|{:>8}| #{:06x}", "red", 0xFF0000);
    println!("|{:>8}| #{:06x}", "green", 0x00FF00);
    println!("|{:>8}| #{:06x}", "blue", 0x0000FF);
    println!("|{:>8}| RGB{:?}", "yellow", (255, 255, 0));
    println!("====================================================");

    /*
    note: the only appropriate formatting traits are:
           - ``, which uses the `Display` trait
           - `?`, which uses the `Debug` trait
           - `e`, which uses the `LowerExp` trait
           - `E`, which uses the `UpperExp` trait
           - `o`, which uses the `Octal` trait
           - `p`, which uses the `Pointer` trait
           - `b`, which uses the `Binary` trait
           - `x`, which uses the `LowerHex` trait
           - `X`, which uses the `UpperHex` trait
    */

    println!("n桁左寄せ: {:<5}_", 30);
    println!("n桁右寄せ: {:>5}", 30);
    println!("n桁中央寄せ: {:^5}", 3);
    println!("n桁左寄せで0埋め: {:<05}", 123);
    println!("n桁右寄せで0埋め: {:>05}", 123);
    println!("2進数: {:b}", 5);
    println!("8進数: {:o}", 5);
    println!("16進数: {:x}", 12);
    println!("16進数n桁0埋め: {:04X}", 21);
    println!("小数点以下n桁{:.2}", 3.141592);
    println!("ポインタアドレス: {:p}", "Hello, world!");
    println!("指数: {:e}", 10);
    println!("{:E}", 123.456);
    println!("degub: {:?}", 123);


}
