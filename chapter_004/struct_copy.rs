struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: String, age: u8) -> Self {
        Self {
            name,
            age,
        }
    }
}

fn main() {
    let p1 = Person::new(String::from("John"), 30);
    let p2 = Person {
        name: String::from("Jane"),
        ..p1
    };

    println!("{} {}", p1.name, p1.age);
    println!("{} {}", p2.name, p2.age);
}
