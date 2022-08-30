fn add_i32(a: i32, b: i32) -> i32 {
    a + b
}

fn add_f32(a: f32, b: f32) -> f32 {
    a + b
}

// trait Add {
//     fn add(self, other: Self) -> Self;
// }

// impl<T: std::ops::Add<Output = T>> Add for T {
//     fn add(self, other: T) -> T {
//         self + other
//     }
// }

fn add <T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

fn x2 <T: std::ops::Add<Output = T> + Copy> (n: T) -> T {
    n + n
}

fn add_where <T> (a: T, b: T) -> T
where T: std::ops::Add<Output = T> {
    a + b
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl <T> Point<T> where T: std::ops::AddAssign {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }

    fn add(&mut self, pt: Point<T>) {
        self.x += pt.x;
        self.y += pt.y;
    }
}

fn main() {
    let mut v1: Vec<i32> = Vec::<i32>::new();
    v1.push(10);
    v1.push(20);
    v1.push(30);
    v1.pop();

    for i in v1.iter() {
        println!("{}", i);
    }

    let mut v2: Vec<char> = Vec::<char>::new();
    v2.push('a');
    v2.push('b');
    v2.push('c');
    v2.pop();

    for i in v2.iter() {
        println!("{}", i);
    }

    println!("{}", add_i32(10, 20));
    println!("{}", add_f32(10.0, 20.0));

    println!("{}", add(10, 20));
    println!("{}", add(10.0, 20.0));
    println!("{}", add(10.0f32, 20.0f32));
    println!("{}", add::<i32>(10, 20));
    println!("{}", add::<f32>(10.0, 20.0));
    // println!("{}", add('a', 'a'));

    println!("{}", x2(10));
    println!("{}", x2(10.0));
    println!("{}", x2(10.0f32));
    println!("{}", x2::<i32>(10));
    println!("{}", x2::<f32>(10.0));
    println!("{}", x2::<f64>(3.0f64));

    println!("{}", add_where(10, 20));
    println!("{}", add_where(10.0, 20.0));

    let mut pt = Point::new(20, 30);
    println!("{:?}", pt);
    pt.add(Point{x: 20, y: 30});
    println!("{:?}", pt);
}
