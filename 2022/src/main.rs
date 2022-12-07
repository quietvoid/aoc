use std::time::Instant;

mod day06;

fn main() {
    let start = Instant::now();

    day06::solve();

    println!("Elapsed: {} μs", start.elapsed().as_micros());
}
