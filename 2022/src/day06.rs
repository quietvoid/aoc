use itertools::Itertools;

const INPUT_STR: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day06.txt"));

#[derive(Default)]
struct Day06 {}

pub fn solve() {
    Day06::default().p1().p2();
}

impl Day06 {
    fn first_marker(unique_chars: usize) -> usize {
        INPUT_STR
            .as_bytes()
            .windows(unique_chars)
            .position(|w| w.iter().unique().count() == w.len())
            .unwrap()
            + unique_chars
    }

    fn p1(self) -> Self {
        const PKT_MARKER_SIZE: usize = 4;
        let res = Self::first_marker(PKT_MARKER_SIZE);

        println!("Part 1: {}", res);

        self
    }

    fn p2(self) {
        const MSG_MARKER_SIZE: usize = 14;
        let res = Self::first_marker(MSG_MARKER_SIZE);

        println!("Part 2: {}", res);
    }
}
