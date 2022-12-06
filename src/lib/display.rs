use std::time::{Duration, Instant};

pub fn print_results<T: std::fmt::Display>(
    label: &str,
    result: T,
    start: Duration,
    timer: Instant,
) -> Duration {
    let duration = timer.elapsed() - start;
    println!("{label}: {result} in {:?}", duration);
    duration
}
