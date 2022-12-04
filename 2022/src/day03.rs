use itertools::Itertools;

const INPUT_STR: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day03.txt"));

#[derive(Default)]
struct Day03 {
    rucksacks: Vec<Rucksack>,
}

struct Rucksack {
    full: &'static str,
    c1: &'static str,
    c2: &'static str,
}

pub fn solve() {
    Day03::default().p1().p2();
}

impl Day03 {
    fn p1(mut self) -> Self {
        self.rucksacks = INPUT_STR.lines().map(Rucksack::from_line).collect();

        let res: usize = self
            .rucksacks
            .iter()
            .flat_map(Rucksack::duplicate_priorities)
            .sum();
        println!("Part 1: {}", res);

        self
    }

    fn p2(self) {
        let res: usize = self
            .rucksacks
            .chunks_exact(3)
            .map(|group| {
                group[0]
                    .full
                    .chars()
                    .filter(|c| group[1].full.contains(*c) && group[2].full.contains(*c))
                    .unique()
                    .map(Rucksack::priority)
                    .sum::<usize>()
            })
            .sum();

        println!("Part 2: {}", res);
    }
}

impl Rucksack {
    fn from_line(line: &'static str) -> Self {
        let (c1, c2) = line.split_at(line.len() / 2);
        Self { full: line, c1, c2 }
    }

    fn priority(c: char) -> usize {
        const LOWER_OFFSET: usize = 96;
        const UPPER_OFFSET: usize = 38;

        if c.is_lowercase() {
            c as usize - LOWER_OFFSET
        } else {
            c as usize - UPPER_OFFSET
        }
    }

    fn duplicate_items(&self) -> impl Iterator<Item = char> {
        self.c1.chars().filter(|c| self.c2.contains(*c)).unique()
    }

    fn duplicate_priorities(&self) -> impl Iterator<Item = usize> {
        self.duplicate_items().map(Self::priority)
    }
}
