use std::ops::RangeInclusive;

use itertools::Itertools;

const INPUT_STR: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day04.txt"));

#[derive(Default)]
struct Day04 {
    pairs: Vec<(RangeInclusive<usize>, RangeInclusive<usize>)>,
}

pub fn solve() {
    Day04::default().p1().p2();
}

impl Day04 {
    fn p1(mut self) -> Self {
        self.pairs = INPUT_STR
            .lines()
            .filter_map(|line| {
                line.split(',')
                    .filter_map(|sections| {
                        sections
                            .split('-')
                            .map(|e| e.parse::<usize>().unwrap())
                            .collect_tuple()
                    })
                    .map(|(start, end)| start..=end)
                    .collect_tuple()
            })
            .collect();

        let res = self
            .pairs
            .iter()
            .filter(|pair| {
                (pair.0.start() <= pair.1.start() && pair.0.end() >= pair.1.end())
                    || (pair.1.start() <= pair.0.start() && pair.1.end() >= pair.0.end())
            })
            .count();

        println!("Part 1: {}", res);

        self
    }

    fn p2(self) {
        let res = self
            .pairs
            .iter()
            .filter(|pair| pair.0.clone().any(|e| pair.1.contains(&e)))
            .count();

        println!("Part 2: {}", res);
    }
}
