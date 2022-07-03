#![feature(mixed_integer_ops)]
#![feature(bool_to_option)]

use keyed_priority_queue::KeyedPriorityQueue;
use std::{cmp::Reverse, collections::HashSet, fs};
use twox_hash::XxHash;

type Point = (usize, usize);

fn neighbors<'a>(maze: &'a [Vec<u32>], (x, y): &'a Point) -> impl Iterator<Item = Point> + 'a {
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .into_iter()
        .filter_map(|(xdir, ydir)| {
            let x = x.checked_add_signed(xdir)?;
            let y = y.checked_add_signed(ydir)?;
            ((..maze.len() * 5).contains(&y) && (..maze[0].len() * 5).contains(&x))
                .then_some((x, y))
        })
}

fn cost(maze: &[Vec<u32>], &(x, y): &Point) -> u32 {
    let y_len = maze.len();
    let x_len = maze[0].len();
    let base_value = maze[y % y_len][x % x_len];
    let x_tile = u32::try_from(x / x_len).unwrap();
    let y_tile = u32::try_from(y / y_len).unwrap();
    (base_value + x_tile + y_tile - 1) % 9 + 1
}

fn djisktras(maze: &[Vec<u32>], start: Point, stop: Point) -> u32 {
    let mut visited: HashSet<_, std::hash::BuildHasherDefault<XxHash>> = Default::default();
    let mut to_visit = KeyedPriorityQueue::new();

    to_visit.push((start, vec![]), Reverse(0));

    while let Some(((cur, chain), Reverse(total_cost))) = to_visit.pop() {
        if !visited.insert(cur) {
            continue;
        }
        if cur == stop {
            return total_cost;
        }
        let mut new_chain = chain.clone();
        new_chain.push(cur.clone());
        for neighbor in neighbors(maze, &cur) {
            to_visit.push(
                (neighbor, new_chain.clone()),
                Reverse(total_cost + cost(maze, &neighbor)),
            );
        }
    }

    unreachable!()
}

fn end(maze: &[Vec<u32>]) -> Point {
    (maze[0].len() * 5 - 1, maze.len() * 5 - 1)
}

fn main() {
    let input = fs::read_to_string("day15/day15.txt").unwrap();
    let maze: Vec<_> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let shortest_cost = djisktras(&maze, (0, 0), end(&maze));
    println!("Shortest: {}", shortest_cost);
}
