use std::time::Instant;

pub fn fib(n: i64) -> i64 {
    if n <= 1 {
        return n;
    }
    return fib(n - 1) + fib(n - 2);
}

pub fn show_time(start_time: Instant) {
    let elapsed = start_time.elapsed();
    println!("Time: {}.{:03} seconds", elapsed.as_secs(), elapsed.subsec_millis());
}
