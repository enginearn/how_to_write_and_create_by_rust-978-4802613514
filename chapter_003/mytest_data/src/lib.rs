pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn array_test() {
        let a1 = [100, 200, 300];
        let a2 = [100, 200, 300];
        assert_eq!(a1, a2);

        let a3 = [
            "リンゴ".to_string(),
            "バナナ".to_string(),
        ];
        let a4 = [
            String::from("リンゴ"),
            String::from("バナナ"),
        ];
        assert_eq!(a3, a4);
        }

    #[test]
    fn vec_test() {
        let v1 = vec!["apple", "banana", "merlon"];
        let mut v2: Vec<&str> = Vec::new();
        v2.push("apple");
        v2.push("banana");
        v2.push("merlon");
        assert_eq!(v1, v2);
    }
}
