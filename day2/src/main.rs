use std::{convert::Infallible, str::FromStr};

use util::read_input;

enum Direction {
    Forward,
    Up,
    Down,
}

impl FromStr for Direction {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => Ok(Self::Forward),
            "up" => Ok(Self::Up),
            "down" => Ok(Self::Down),
            _ => unreachable!(),
        }
    }
}

struct Movement {
    direction: Direction,
    distance: usize,
}

impl FromStr for Movement {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(' ') {
            Some((dir, dist)) => Ok(Self {
                direction: dir.parse().unwrap(),
                distance: dist.parse().unwrap(),
            }),
            None => unreachable!(),
        }
    }
}

#[derive(Default)]
struct Pos {
    x: usize,
    y: usize,
    aim: usize,
}

impl Pos {
    pub fn movement(
        self,
        Movement {
            direction,
            distance,
        }: Movement,
    ) -> Self {
        let Pos { x, y, aim } = self;

        match direction {
            Direction::Forward => Self {
                x: x + distance,
                y: y + distance * aim,
                aim,
            },
            Direction::Up => Self {
                x,
                y,
                aim: aim - distance,
            },
            Direction::Down => Self {
                x,
                y,
                aim: aim + distance,
            },
        }
    }
}

fn main() {
    let input = read_input("inputs/day2.txt");

    let Pos { x, y, .. } = input.fold(Pos::default(), Pos::movement);
    println!("The result is: {}", x * y);
}
