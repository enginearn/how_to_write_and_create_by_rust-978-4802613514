use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use std::time::Instant;

mod funcs;
use funcs::fib;
use funcs::show_time;

// mpsc: multiple producer, single consumer
fn sleep_sender(name: &str, sender: mpsc::Sender<String>) {
    for i in 1..=5 {
        let msg = format!("{}: {}", name, i);
         // send() will block the current thread until the receiver is ready to receive the message
        sender.send(msg).unwrap();
        thread::sleep(Duration::from_millis(1000)); // sleep 1 second
    }
    sender.send(String::from("done")).unwrap();
}

fn main() {
    let (tx, rx) = mpsc::channel::<String>();

    let sender = tx.clone();
    thread::spawn(move || {
        sleep_sender("thread1", sender);
    });

    let sender = tx.clone();
    thread::spawn(move || {
        sleep_sender("thread2", sender);
    });

    // rx.recv() will block the current thread until a message is received
    loop {
        let msg = rx.recv().unwrap();
        println!("Got: {}", msg);
        if msg == "done" {
            break;
        }
    }

    // no thread
    println!("\nno thread");
    let request_nums = vec![30, 40, 35, 20, 25];
    let start_time = Instant::now();
    for num in request_nums {
        println!("fib({}) = {}", num, fib(num));
    }
    show_time(start_time);

    // thread
    println!("\nthread");
    let request_nums2 = [30, 40, 35, 20, 25];
    let start_time = Instant::now();

    let (tx, rx) = mpsc::channel::<(i64, i64)>();
    for num in request_nums2 {
        let sender = tx.clone();
        thread::spawn(move || {
            let result = fib(num);
            sender.send((num, result)).unwrap();
        });
    }

    let mut job = request_nums2.len();
    loop {
        if let Ok((arg, result)) = rx.recv() {
            job -= 1;
            println!("fib({}) = {}", arg, result);
            if job <= 0 {
                show_time(start_time);
                break;
            }
        }
        thread::sleep(Duration::from_millis(1000)); // sleep 1 second
    }
}
