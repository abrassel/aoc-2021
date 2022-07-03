#![feature(try_blocks)]
#![feature(mixed_integer_ops)]

use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
    fs,
};

use keyed_priority_queue::KeyedPriorityQueue;

fn dijsktra(
    grid: &[Vec<u32>],
    start: (usize, usize),
    end: (usize, usize),
) -> HashMap<(usize, usize), ((usize, usize), u32)> {
    let n_size = grid.len() * grid[0].len();
    let mut visited = HashSet::with_capacity(n_size);
    let mut to_visit = KeyedPriorityQueue::with_capacity(n_size);
    let mut paths = HashMap::with_capacity(n_size);

    to_visit.push(start, Reverse(0));

    while let Some((cur, Reverse(cost))) = to_visit.pop() {
        println!("Visiting: {:?} at a cost of {}", cur, cost);
        if cur == end {
            break;
        }
        visited.insert(cur);

        for dir in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
            let neighbor: Option<_> = try {
                let x = cur.0.checked_add_signed(dir.0)?;
                let y = cur.1.checked_add_signed(dir.1)?;
                let cost = *grid.get(y)?.get(x)?;

                Some(((x, y), cost))
            };
            if let Some(Some((neighbor, edge))) = neighbor {
                if !visited.contains(&neighbor) {
                    let visit_cost = cost + edge;
                    paths
                        .entry(neighbor)
                        .and_modify(|orig: &mut ((usize, usize), u32)| {
                            if orig.1 > visit_cost {
                                *orig = (cur, visit_cost);
                            }
                        })
                        .or_insert((cur, visit_cost));

                    to_visit.push(neighbor, Reverse(visit_cost));
                }
            }
        }
    }

    paths
}

fn main() {
    let input = fs::read_to_string("day15/day15.txt").unwrap();
    let grid: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    let end = (grid[0].len() - 1, grid.len() - 1);
    let paths = dijsktra(&grid, (0, 0), end);
    let mut cur = &end;
    let mut sum = grid[end.1][end.0];
    while let Some((prev, cost)) = paths.get(cur) {
        println!(
            "Prev: {:?}, cost: {}, grid: {}",
            prev, cost, grid[prev.1][prev.0]
        );
        sum += grid[prev.1][prev.0];
        cur = prev;
    }
    println!("Paths: {:?}", paths);
    println!("{}", sum - 1);
    println!("The answer is: {}", paths.get(&end).unwrap().1)
}
