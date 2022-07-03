use std::{collections::HashSet, fmt::Display, fs};

#[derive(Debug)]
enum MoveKind {
    X,
    Y,
}

#[derive(Debug)]
struct Move {
    dir: MoveKind,
    loc: usize,
}

impl Display for Points {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_x = self.0.iter().map(|(x, _)| x).max().unwrap();
        let max_y = self.0.iter().map(|(_, y)| y).max().unwrap();

        for y in 0..(max_y + 1) {
            for x in 0..(max_x + 1) {
                if self.0.contains(&(x, y)) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

struct Points(HashSet<(usize, usize)>);

impl Points {
    pub fn fold(self, Move { dir, loc }: Move) -> Self {
        Self(
            self.0
                .into_iter()
                .map(|(mut x, mut y)| {
                    match dir {
                        MoveKind::X if loc < x => {
                            x = loc - (x - loc);
                        }
                        MoveKind::Y if loc < y => {
                            y = loc - (y - loc);
                        }
                        _ => {}
                    };
                    (x, y)
                })
                .collect(),
        )
    }
}

fn get_input() -> (Points, Vec<Move>) {
    let input = fs::read_to_string("day13/day13.txt").unwrap();
    let (coords, folds) = input.split_once("\n\n").unwrap();
    let coords = coords
        .lines()
        .map(|line| {
            let (l, r) = line.split_once(',').unwrap();
            (l.parse().unwrap(), r.parse().unwrap())
        })
        .collect();
    let moves = folds
        .lines()
        .map(|line| {
            let (dir, amt) = serde_scan::scan!("fold along {}={}" <- line).unwrap();
            let dir = match dir {
                "x" => MoveKind::X,
                "y" => MoveKind::Y,
                _ => unreachable!(),
            };

            Move { dir, loc: amt }
        })
        .collect();

    (Points(coords), moves)
}

fn main() {
    let (mut points, moves) = get_input();
    for fold in moves {
        points = points.fold(fold);
    }

    println!("{}", points);
}
