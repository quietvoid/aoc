use std::time::Instant;

mod day07;

fn main() {
    let start = Instant::now();

    day07::solve();

    println!("Elapsed: {} μs", start.elapsed().as_micros());
}
