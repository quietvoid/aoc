use itertools::Itertools;

const INPUT_STR: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day08.txt"));

#[derive(Debug, Default)]
struct Day08 {
    grid: Vec<u32>,
    rows: Vec<Vec<u32>>,
    columns: Vec<Vec<u32>>,
}

pub fn solve() {
    Day08::default().p1().p2();
}

impl Day08 {
    fn process_tree<'a>(
        trees: impl Iterator<Item = &'a u32>,
        max: u32,
        idx: usize,
        invert_idx: bool,
    ) -> impl Iterator<Item = (usize, usize)> {
        let mut max = max;

        trees.copied().enumerate().filter_map(move |(idx2, e)| {
            if e > max {
                max = e;

                if invert_idx {
                    Some((idx2, idx))
                } else {
                    Some((idx, idx2))
                }
            } else {
                None
            }
        })
    }

    fn find_visible(
        &self,
        trees: &[u32],
        reverse: bool,
        idx: usize,
        invert_idx: bool,
    ) -> Vec<(usize, usize)> {
        let range = 1..trees.len() - 1;

        if reverse {
            Self::process_tree(
                trees[range].iter().rev(),
                trees[trees.len() - 1],
                idx,
                invert_idx,
            )
            .map(|(x, y)| {
                if invert_idx {
                    (self.columns.len() - 3 - x, y)
                } else {
                    (x, self.rows.len() - 3 - y)
                }
            })
            .collect()
        } else {
            Self::process_tree(trees[range].iter(), trees[0], idx, invert_idx).collect()
        }
    }

    fn compute_visible_trees(&mut self) -> usize {
        let mut left_right: Vec<(usize, usize)> = self.rows[1..self.rows.len() - 1]
            .iter()
            .enumerate()
            .flat_map(|(idx, data)| {
                let mut left = self.find_visible(data, false, idx, false);
                let right = self.find_visible(data, true, idx, false);

                left.extend(right);

                left
            })
            .collect();

        let top_bottom: Vec<(usize, usize)> = self.columns[1..self.columns.len() - 1]
            .iter()
            .enumerate()
            .flat_map(|(idx, data)| {
                let mut top = self.find_visible(data, false, idx, true);
                let bottom = self.find_visible(data, true, idx, true);

                top.extend(bottom);

                top
            })
            .collect();

        left_right.extend(top_bottom);

        left_right.iter().unique().count()
    }

    fn fill_intermediate_grids(&mut self, stride: usize, rows: usize) {
        self.rows.resize(rows, vec![0; stride]);
        self.columns.resize(stride, vec![0; rows]);

        self.rows.iter_mut().enumerate().for_each(|(row, data)| {
            data.iter_mut()
                .enumerate()
                .for_each(|(col, tree)| *tree = self.grid[(row * stride) + col])
        });

        self.columns.iter_mut().enumerate().for_each(|(col, data)| {
            data.iter_mut()
                .enumerate()
                .for_each(|(row, tree)| *tree = self.grid[(row * stride) + col])
        });
    }

    fn compute_scenic_score(&self, tree: (usize, usize)) -> usize {
        let row = &self.rows[tree.0];
        let col = &self.columns[tree.1];

        let tree_height = self.grid[(tree.0 * self.rows.len()) + tree.1];

        let left = tree.1
            - row[..tree.1]
                .iter()
                .rposition(|x| *x >= tree_height)
                .unwrap_or(0);
        let right = row[tree.1 + 1..]
            .iter()
            .position(|x| *x >= tree_height)
            .map(|pos| pos + 1)
            .unwrap_or(row.len() - tree.1 - 1);

        let top = tree.0
            - col[..tree.0]
                .iter()
                .rposition(|x| *x >= tree_height)
                .unwrap_or(0);
        let bottom = col[tree.0 + 1..]
            .iter()
            .position(|x| *x >= tree_height)
            .map(|pos| pos + 1)
            .unwrap_or(col.len() - tree.0 - 1);

        left * right * top * bottom
    }

    fn p1(mut self) -> Self {
        let stride = INPUT_STR.lines().next().unwrap().len();

        self.grid = INPUT_STR
            .lines()
            .flat_map(|line| line.chars().map(|c| c.to_digit(10).unwrap()))
            .collect();
        let rows = self.grid.len() / stride;
        let visible_edge = (rows * 2) + (stride * 2) - 4;

        self.fill_intermediate_grids(stride, rows);

        let res = visible_edge + self.compute_visible_trees();

        println!("Part 1: {}", res);

        self
    }

    fn p2(self) {
        let res = (0..self.rows.len())
            .cartesian_product(0..self.columns.len())
            .map(|tree| self.compute_scenic_score(tree))
            .max()
            .unwrap();

        println!("Part 2: {}", res);
    }
}
