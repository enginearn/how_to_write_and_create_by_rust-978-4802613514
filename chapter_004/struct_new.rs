struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: String, age: u8) -> Self {
        Person {
            name,
            age,
        }
    }
}

fn main() {
    let mary = Person::new("メアリー".to_string(), 20);
    println!("{}は{}才です。", mary.name, mary.age);
}
