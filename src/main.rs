use std::{
    hint::black_box,
    thread::{available_parallelism, spawn},
    time::{Duration, Instant},
};

const TIME_LIMIT: Duration = Duration::new(1, 0);

fn main() {
    // Single-threaded test
    let result = run_benchmark();
    println!("Single-threaded result: {result}");

    // Multi-threaded test
    let mut handles = Vec::new();
    let mut sum = 0;

    if let Ok(total_threads) = available_parallelism() {
        let total_threads = total_threads.get();

        for _ in 0..total_threads {
            handles.push(spawn(|| run_benchmark()));
        }

        for handle in handles {
            match handle.join() {
                Ok(v) => sum += v,
                Err(e) => eprintln!("Thread join error: {e:?}"),
            }
        }

        let average = sum as f64 / total_threads as f64;

        println!("Multi-threaded result: {sum} ({total_threads} threads, {average} average)");
    } else {
        eprintln!("Multi-threaded test failed. Unable to get threads number.");
    }
}

fn run_benchmark() -> u32 {
    let mut n = 0;

    loop {
        let start = Instant::now();
        black_box(fibonacci(n as u128));
        let stop = Instant::now();

        if stop.duration_since(start) > TIME_LIMIT {
            return n;
        }

        n += 1;
    }
}

fn fibonacci(n: u128) -> u128 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
