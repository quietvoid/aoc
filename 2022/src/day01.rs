const INPUT_STR: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day01.txt"));

#[derive(Default)]
struct Day01 {
    cals: Vec<usize>,
}

pub fn solve() {
    let mut s = Day01::default();
    s.p1();
    s.p2();
}

impl Day01 {
    fn p1(&mut self) {
        self.cals = INPUT_STR.split('\n').fold(vec![0], |mut sums, line| {
            if line.is_empty() {
                sums.push(0);
            } else if let Some(last) = sums.last_mut() {
                *last += line.parse::<usize>().unwrap();
            }

            sums
        });

        println!("Part 1: {}", self.cals.iter().max().unwrap());
    }

    fn p2(&mut self) {
        self.cals.sort();

        let len = self.cals.len();
        let res = self.cals[len - 3..].iter().sum::<usize>();

        println!("Part 2: {}", res);
    }
}
