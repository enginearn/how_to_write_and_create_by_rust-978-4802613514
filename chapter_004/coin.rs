enum Coin {
    Coin1 (isize),
    Coin5 (isize),
    Coin10 (isize),
    Coin50 (isize),
    Coin100 (isize),
    Coin500 (isize),
}

impl Coin {
    fn calc_price(&self) -> isize {
        match *self {
            Coin::Coin1 (n) => n * 1,
            Coin::Coin5 (n) => n * 5,
            Coin::Coin10 (n) => n * 10,
            Coin::Coin50 (n) => n * 50,
            Coin::Coin100 (n) => n * 100,
            Coin::Coin500 (n) => n * 500,
        }
    }
}

fn main() {
    let coin1 = Coin::Coin1 (3);
    let coin5 = Coin::Coin5 (2);
    let coin10 = Coin::Coin10 (5);
    let coin50 = Coin::Coin50 (2);
    let coin100 = Coin::Coin100 (9);
    let coin500 = Coin::Coin500 (2);

    println!("{}円", coin1.calc_price());
    println!("{}円", coin5.calc_price());
    println!("{}円", coin10.calc_price());
    println!("{}円", coin50.calc_price());
    println!("{}円", coin100.calc_price());
    println!("{}円", coin500.calc_price());

    let wallet: Vec<Coin> = vec![
        Coin::Coin1 (3),
        Coin::Coin5 (2),
        Coin::Coin10 (5),
        Coin::Coin50 (2),
        Coin::Coin100 (9),
        Coin::Coin500 (2),
    ];

    let total = wallet.iter()
                .fold(0, |sum, coin| sum + coin.calc_price());
    println!("合計: {}円", total);
}
