use itertools::Itertools;

const INPUT_STR: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day04-ex.txt"));

#[derive(Default)]
struct Day04 {
}

pub fn solve() {
    Day04::default().p1().p2();
}

impl Day04 {
    fn p1(mut self) -> Self {
        let res = 0;
        println!("Part 1: {}", res);

        self
    }

    fn p2(self) {
        let res = 0;
        println!("Part 2: {}", res);
    }
}
