use simple_matrix::Matrix;
use std::{collections::HashSet, fs, path::Path};

struct Octopi(Matrix<u32>);

impl Octopi {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let input = fs::read_to_string(path).unwrap();
        let input = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|char| char.to_digit(10).unwrap())
                    .collect()
            })
            .collect();
        Self(Matrix::from_vecs(input))
    }

    pub fn step(&mut self) {
        // step 1: update
        for elm in (&mut self.0).into_iter() {
            *elm += 1;
        }

        // step 2: seed the dfs
        let mut flashed = HashSet::new();
        let mut queue: Vec<_> = self
            .0
            .iter()
            .enumerate()
            .filter_map(|(idx, val)| {
                let row = idx / self.0.cols();
                let col = idx % self.0.cols();
                if *val > 9 {
                    Some((row, col))
                } else {
                    None
                }
            })
            .collect();

        // step 3: execute the dfs
        while let Some((row, col)) = queue.pop() {
            if !flashed.insert((row, col)) {
                continue;
            }
            self.0.set(row, col, 0);
            let locs: Vec<_> = self
                .0
                .neighbors_with_diagonals(row, col)
                .map(|n| n.loc)
                .filter(|loc| !flashed.contains(loc))
                .collect();
            for loc in locs {
                let val = self.0.get_mut(loc.0, loc.1).unwrap();
                *val += 1;
                if *val > 9 {
                    queue.push(loc);
                }
            }
        }
    }

    pub fn is_synchronized(&self) -> bool {
        self.0.iter().all(|x| *x == 0)
    }
}

fn main() {
    let mut octopi = Octopi::new("day11/day11.txt");
    // let score: usize = iter::repeat_with(|| octopi.step()).take_while(100).sum();
    let mut score = 0;
    while !octopi.is_synchronized() {
        octopi.step();
        score += 1;
    }
    println!("Answer: {}", score);
}
