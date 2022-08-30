struct FiboIterator {
    a: usize,
    b: usize,
}

impl FiboIterator {
    fn new() -> Self {
        FiboIterator {a: 0, b: 1}
    }
}

impl Iterator for FiboIterator {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        let c = self.a + self.b;
        self.a = self.b;
        self.b = c;
        Some(self.a)
    }
}

fn main() {
    let fibo_iterator = FiboIterator::new();
    for (i, n) in fibo_iterator.enumerate() {
        if i >= 10 {
            break;
        }
        println!("{}", n);
    }
    println!("---");

    let fibo_iterator = FiboIterator::new();
    fibo_iterator.take(10).for_each(|n| println!("{}", n));
}
