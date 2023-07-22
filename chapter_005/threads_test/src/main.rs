use std::{thread, time};

fn sleep_print(name: &str) {
    for i in 1..=3 {
        println!("{}: i={}", name, i);
        thread::sleep(time::Duration::from_millis(1000)); // 1秒間スリープ
    }
}

fn main() {
    println!("=== no thread ===");
    sleep_print("no thread");

    println!("=== thread ===");
    thread::spawn(|| {
        sleep_print("thread1");
    });

    thread::spawn(|| {
        sleep_print("thread2");
    });

    thread::spawn(|| {
        sleep_print("thread3");
    });

    sleep_print("main thread");
}
