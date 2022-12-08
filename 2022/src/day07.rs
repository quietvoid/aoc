use std::{collections::BTreeMap, path::PathBuf};

use itertools::Itertools;

const INPUT_STR: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day07.txt"));

#[derive(Debug, Default)]
struct Day07 {
    paths: BTreeMap<PathBuf, Directory>,
}

#[derive(Debug, Default, Clone)]
struct Directory {
    dirs: Vec<PathBuf>,
    files: Vec<File>,
    size: usize,
}

#[derive(Debug, Clone)]
struct File {
    size: usize,
    _name: &'static str,
}

pub fn solve() {
    Day07::default().p1().p2();
}

impl Day07 {
    fn parse_tree(&mut self) {
        let mut path = PathBuf::new();

        for line in INPUT_STR.lines() {
            if line.starts_with('$') {
                let cmd = line.get(2..).unwrap();
                if cmd.starts_with("cd") {
                    let next_path = cmd.get(3..).unwrap();

                    if next_path == ".." {
                        path.pop();
                    } else {
                        path.push(next_path);
                    }

                    self.paths.entry(path.clone()).or_default();
                }
            } else if line.starts_with("dir") {
                let mut new_path = path.clone();
                new_path.push(line.get(4..).unwrap());

                let cur_dir = self.paths.get_mut(&path).unwrap();
                cur_dir.dirs.push(new_path.clone());

                self.paths.entry(new_path).or_default();
            } else {
                let mut info = line.split_whitespace();

                self.paths.get_mut(&path).unwrap().files.push(File {
                    size: info.next().unwrap().parse::<usize>().unwrap(),
                    _name: info.next().unwrap(),
                });
            }
        }
    }

    fn compute_size(&mut self) {
        let path_sizes: Vec<usize> = self
            .paths
            .values()
            .map(|dir| dir.compute_size(&self.paths))
            .collect();

        self.paths
            .values_mut()
            .zip(path_sizes)
            .for_each(|(dir, size)| {
                dir.size = size;
            });
    }

    fn p1(mut self) -> Self {
        self.parse_tree();
        self.compute_size();

        let res: usize = self
            .paths
            .values()
            .filter(|dir| dir.size <= 100_000)
            .map(|dir| dir.size)
            .sum();

        println!("Part 1: {}", res);

        self
    }

    fn p2(self) {
        let unused = 70_000_000 - self.paths.get(&PathBuf::from("/")).unwrap().size;
        let min_size = 30_000_000 - unused;

        let res = self
            .paths
            .values()
            .filter(|dir| dir.size >= min_size)
            .map(|dir| dir.size)
            .sorted()
            .next()
            .unwrap();

        println!("Part 2: {}", res);
    }
}

impl Directory {
    fn compute_size(&self, paths: &BTreeMap<PathBuf, Self>) -> usize {
        let file_size: usize = self.files.iter().map(|f| f.size).sum();
        let dir_size: usize = self
            .dirs
            .iter()
            .map(|d| paths.get(d).unwrap().compute_size(paths))
            .sum();

        file_size + dir_size
    }
}
