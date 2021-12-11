use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

const NUMBERS: [&str; 10] = [
    "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
];

fn frequencies(numbers: &[&str]) -> HashMap<char, usize> {
    numbers.into_iter().map(|c| c.chars()).flatten().counts()
}

fn score(reference: &HashMap<usize, usize>, (input, output): (Vec<&str>, Vec<&str>)) -> usize {
    let observed = frequencies(&input);
    output.into_iter().fold(0, |prev, rep| {
        let count = count_num(rep, &observed);
        let map = reference.get(&count).unwrap();
        prev * 10 + map
    })
}

fn count_num(num: &str, freqs: &HashMap<char, usize>) -> usize {
    num.chars().map(|char| freqs.get(&char).unwrap()).sum()
}

fn main() {
    let input = fs::read_to_string("day8/day8.txt").unwrap();
    let input: Vec<(Vec<&str>, Vec<&str>)> = input
        .lines()
        .map(|line| {
            let (input, output) = line.split_once(" | ").unwrap();
            let input = input.split_whitespace().collect();
            let output = output.split_whitespace().collect();

            (input, output)
        })
        .collect();
    let reference = {
        let freqs = frequencies(&NUMBERS);
        NUMBERS
            .iter()
            .enumerate()
            .map(|(n, rep)| {
                let score = count_num(rep, &freqs);
                (score, n)
            })
            .collect()
    };

    let count: usize = input.into_iter().map(|line| score(&reference, line)).sum();

    println!("Answer: {}", count);
}
