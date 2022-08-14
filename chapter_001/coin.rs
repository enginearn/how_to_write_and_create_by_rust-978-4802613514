fn main() {
    let price = 3950;
    for coin_500 in 0..11 {
        for coin_100 in 0..4 {
            for coin_50 in 0..11 {
                let total = coin_500 * 500 + coin_100 * 100 + coin_50 * 50;
                if total == price {
                    println!("500円 * {} + 100円 * {} + 50円 * {} = {}", coin_500, coin_100, coin_50, total);
                }
            }
        }
    }
}