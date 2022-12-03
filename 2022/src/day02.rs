use itertools::Itertools;

const INPUT_STR: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day02.txt"));

#[derive(Default)]
struct Day02 {
    rounds: Vec<(Shape, Shape)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

pub fn solve() {
    Day02::default().p1().p2();
}

impl Day02 {
    fn get_score(opp: &Shape, me: &Shape) -> usize {
        let me_score = *me as usize;

        if opp == me {
            return me_score + 3;
        }

        match (&me, &opp) {
            (Shape::Rock, Shape::Scissors)
            | (Shape::Paper, Shape::Rock)
            | (Shape::Scissors, Shape::Paper) => me_score + 6,
            _ => me_score,
        }
    }

    fn get_shape(opp: &Shape, win: bool) -> Shape {
        match opp {
            Shape::Rock => {
                if win {
                    Shape::Paper
                } else {
                    Shape::Scissors
                }
            }
            Shape::Paper => {
                if win {
                    Shape::Scissors
                } else {
                    Shape::Rock
                }
            }
            Shape::Scissors => {
                if win {
                    Shape::Rock
                } else {
                    Shape::Paper
                }
            }
        }
    }

    fn p1(mut self) -> Self {
        self.rounds = INPUT_STR
            .split('\n')
            .filter_map(|e| {
                e.split(' ')
                    .filter(|v| !v.is_empty())
                    .map(Shape::from_str)
                    .collect_tuple()
            })
            .collect();

        let res: usize = self
            .rounds
            .iter()
            .map(|(opp, me)| Self::get_score(opp, me))
            .sum();

        println!("Part 1: {}", res);

        self
    }

    fn p2(self) {
        let res: usize = self
            .rounds
            .iter()
            .map(|(opp, me_result)| {
                let me = match me_result {
                    Shape::Paper => *opp,                          // Draw
                    Shape::Rock => Self::get_shape(opp, false),    // Lose
                    Shape::Scissors => Self::get_shape(opp, true), // Win
                };

                Self::get_score(opp, &me)
            })
            .sum();

        println!("Part 2: {}", res);
    }
}

impl Shape {
    fn from_str(v: &str) -> Self {
        match v {
            "A" | "X" => Shape::Rock,
            "B" | "Y" => Shape::Paper,
            "C" | "Z" => Shape::Scissors,
            _ => unreachable!(),
        }
    }
}
