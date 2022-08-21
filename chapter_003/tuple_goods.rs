
fn print_tuple(item: &(&str, i32)) {
    println!("{}を{}円で購入", item.0, item.1);
}

struct Item(String, i32);

fn print_tuple_items(item: &Item) {
    println!("{}を{}円で購入", item.0, item.1);
}

fn print_and_sum_items(items: &Vec<Item>) -> i32 {
    let mut sum = 0;
    for item in items {
        print_tuple_items(&item);
        sum += item.1;
    }
    sum
}

fn main() {
    println!("tupleのみで処理");
    let banana = ("banana", 300);
    let apple = ("apple", 200);
    let total_price = banana.1 + apple.1;

    print_tuple(&banana);
    print_tuple(&apple);
    println!("total_price: {}円", total_price);
    println!("");

    println!("tupleをstructで定義して処理");
    let mango = Item("mango".to_string(), 500);
    let orange = Item("orange".to_string(), 400);

    let items = vec![mango, orange];
    let sum = print_and_sum_items(&items);
    println!("合計金額は{:?}円です", sum);
}
