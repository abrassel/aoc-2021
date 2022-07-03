#![feature(mixed_integer_ops)]

use std::{cmp::Reverse, collections::HashSet, fs};

const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn neighbors(
    input: &'_ [Vec<u8>],
    (x, y): (usize, usize),
) -> impl Iterator<Item = ((usize, usize), u8)> + '_ {
    DIRECTIONS.iter().filter_map(move |(y_dir, x_dir)| {
        let y = y.checked_add_signed(*y_dir)?;
        let x = x.checked_add_signed(*x_dir)?;
        let row = input.get(y)?;
        let val = *row.get(x)?;
        Some(((x, y), val))
    })
}

fn main() {
    let input = fs::read_to_string("day9/day9.txt").unwrap();
    let input = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect::<Vec<Vec<u8>>>();

    let mut minimums = vec![];
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            let val = input[y][x];
            if neighbors(&input, (x, y)).all(|(_, n)| n > val) {
                minimums.push((x, y));
            }
        }
    }

    let mut basins: Vec<_> = minimums
        .into_iter()
        .map(|minima| {
            let mut basin = HashSet::new();
            let mut to_visit = vec![minima];
            while let Some(cur) = to_visit.pop() {
                basin.insert(cur);
                for (nloc, nval) in neighbors(&input, cur) {
                    if nval < 9 && nval > input[cur.1][cur.0] && !basin.contains(&nloc) {
                        to_visit.push(nloc);
                    }
                }
            }
            basin
        })
        .collect();

    basins.sort_unstable_by_key(|basin| Reverse(basin.len()));
    println!("{}", basins[0].len() * basins[1].len() * basins[2].len());
}
