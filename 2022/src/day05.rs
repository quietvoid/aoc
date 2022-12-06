use std::collections::{BTreeMap, VecDeque};

use once_cell::sync::Lazy;
use regex::Regex;

const INPUT_STR: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day05.txt"));
static MOVE_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap());

#[derive(Default)]
struct Day05 {
    stacks: BTreeMap<usize, VecDeque<char>>,
    moves: Vec<Move>,
}

#[derive(Debug)]
struct Move {
    count: usize,
    src: usize,
    dst: usize,
}

pub fn solve() {
    Day05::default().p1().p2();
}

impl Day05 {
    #[allow(clippy::needless_collect)]
    fn p1(mut self) -> Self {
        let mut sections = INPUT_STR.split("\n\n");
        let (stacks, moves) = (sections.next().unwrap(), sections.next().unwrap());

        stacks.lines().for_each(|line| {
            line.char_indices()
                .filter(|e| e.1.is_alphabetic())
                .for_each(|e| {
                    self.stacks
                        .entry((e.0 as f32 / 4.0).ceil() as usize)
                        .or_insert_with(VecDeque::default)
                        .push_back(e.1);
                });
        });

        self.moves = moves.lines().map(Move::from_str).collect();

        let mut stacks = self.stacks.clone();
        self.moves.iter().for_each(|m| {
            let crates: Vec<char> = stacks.get_mut(&m.src).unwrap().drain(0..m.count).collect();

            let dst = stacks.get_mut(&m.dst).unwrap();
            crates.into_iter().for_each(|c| dst.push_front(c));
        });

        let res = stacks
            .values_mut()
            .filter_map(|s| s.pop_front())
            .collect::<String>();

        println!("Part 1: {}", res);

        self
    }

    #[allow(clippy::needless_collect)]
    fn p2(self) {
        let mut stacks = self.stacks.clone();
        self.moves.iter().for_each(|m| {
            let mut crates: Vec<char> = stacks.get_mut(&m.src).unwrap().drain(0..m.count).collect();
            crates.reverse();

            let dst = stacks.get_mut(&m.dst).unwrap();
            crates.into_iter().for_each(|c| dst.push_front(c));
        });

        let res = stacks
            .values_mut()
            .filter_map(|s| s.pop_front())
            .collect::<String>();

        println!("Part 2: {}", res);
    }
}

impl Move {
    fn from_str(v: &str) -> Self {
        let caps = MOVE_RE.captures(v).unwrap();

        Self {
            count: caps[1].parse::<usize>().unwrap(),
            src: caps[2].parse::<usize>().unwrap(),
            dst: caps[3].parse::<usize>().unwrap(),
        }
    }
}
