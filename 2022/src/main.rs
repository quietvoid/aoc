use std::time::Instant;

mod day08;

fn main() {
    let start = Instant::now();

    day08::solve();

    println!("Elapsed: {} μs", start.elapsed().as_micros());
}
