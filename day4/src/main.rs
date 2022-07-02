use std::{collections::HashMap, convert::Infallible, fs, str::FromStr};

use nalgebra::Matrix5;

struct Board {
    board: Matrix5<bool>,
    mapping: HashMap<usize, (usize, usize)>,
}

struct Input {
    moves: Vec<usize>,
    boards: Vec<Board>,
}

impl FromStr for Input {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        struct Moves(Vec<usize>);

        impl FromStr for Moves {
            type Err = Infallible;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(Self(s.split(',').map(|num| num.parse().unwrap()).collect()))
            }
        }

        impl FromStr for Board {
            type Err = Infallible;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let mut mapping = HashMap::with_capacity(5 * 5);
                let raw = s.lines().enumerate().flat_map(|(row, line)| {
                    line.split_whitespace()
                        .enumerate()
                        .map(|(col, num)| {
                            let num = usize::from_str(num).unwrap();
                            mapping.insert(num, (col, row));
                            false
                        })
                        .collect::<Vec<_>>()
                });

                let board = Matrix5::from_iterator(raw);

                Ok(Board { board, mapping })
            }
        }

        let (moves, boards) = s.split_once("\n\n").unwrap();

        let moves = Moves::from_str(moves).unwrap().0;
        let boards = boards
            .split("\n\n")
            .map(|board| board.parse().unwrap())
            .collect();

        Ok(Self { moves, boards })
    }
}

impl Board {
    pub fn perform_move(&mut self, m: &usize) {
        let index = match self.mapping.remove(m) {
            Some(index) => index,
            None => return,
        };
        *self.board.index_mut(index) = true;
    }

    pub fn has_won(&self) -> bool {
        self.board.row_iter().any(|row| row.iter().all(|&x| x))
            || self.board.column_iter().any(|col| col.iter().all(|&x| x))
    }

    pub fn score(&self) -> usize {
        self.mapping.keys().sum()
    }
}

pub fn main() {
    let Input { moves, mut boards } = {
        let input = fs::read_to_string("inputs/day4.txt").unwrap();
        Input::from_str(&input).unwrap()
    };

    for m in moves {
        for board in &mut boards {
            board.perform_move(&m);
        }
        if boards.len() != 1 {
            boards.retain(|board| !board.has_won());
        } else if boards[0].has_won() {
            println!("{}, {:?}, {}", boards[0].score() * m, boards[0].score(), m);
            return;
        }
    }
}
