#[derive(Debug, PartialEq)]
struct GItem {
    name: String,
    price: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_test() {
        let apple = GItem {
            name: String::from("リンゴ"),
            price: 2400,
        };
        let mut banana = GItem {
            name: "バナナ".to_string(),
            price: 0,
        };
        banana.price = 2400;

        assert_ne!(apple.name, banana.name);
        assert_eq!(apple.price, banana.price);
    }
}
