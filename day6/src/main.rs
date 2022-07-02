use nalgebra::{SMatrix, SVector};
use std::fs::read_to_string;

const DIM: usize = 9;

fn power(mut matrix: SMatrix<u64, DIM, DIM>, mut pow: u64) -> SMatrix<u64, DIM, DIM> {
    let mut total = SMatrix::identity();
    while pow > 0 {
        if pow % 2 == 0 {
            matrix *= matrix;
            pow /= 2;
        } else {
            total *= matrix;
            pow -= 1;
        }
    }

    total
}

fn evolve_fish(start: SMatrix<u64, 1, DIM>, days: u64) -> SMatrix<u64, 1, DIM> {
    let transformation = SMatrix::<u64, DIM, DIM>::from([
        [0, 0, 0, 0, 0, 0, 1, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 1, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 1, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 1, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 1, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 1, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 1, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 1, 0],
    ])
    .transpose();

    let transform = power(transformation, days);
    start * transform
}

fn main() {
    let mut fishies: [u64; DIM] = [0; DIM];
    let tmp = read_to_string("day6/day6.txt").unwrap();
    let input = tmp.split(',').map(|val| val.parse::<usize>().unwrap());

    for fish in input {
        fishies[fish] += 1;
    }

    let input: SMatrix<u64, 1, DIM> = SVector::from(fishies).transpose();
    let result_state = evolve_fish(input, 256);
    let fish_sum: u64 = result_state.iter().sum();

    println!("Answer: {}", fish_sum);
}
