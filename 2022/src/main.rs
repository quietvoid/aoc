use std::time::Instant;

mod day05;

fn main() {
    let start = Instant::now();

    day05::solve();

    println!("Elapsed: {} μs", start.elapsed().as_micros());
}
