fn main() {
    let s = "隣の客はよく柿食う客だ。";

    match s.find('客') {
        Some(i) => println!("客={}B", i),
        None => println!("Not found"),
    }

    match s.find("食う") {
        Some(i) => println!("食う={}B", i),
        None => println!("Not found"),
    }

    let s = format!("{}{}",
        "There is more happiness in giving",
        "than in receiving.");

    let res = s.find(|c: char|c.to_ascii_uppercase() == 'S');
    match res {
        Some(i) => println!("S={}B", i),
        None => println!("Not found"),
    };
}
