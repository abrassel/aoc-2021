use itertools::Itertools;
use std::{collections::HashMap, fs};

fn update(map: &mut HashMap<(char, char), usize>, new_pair: (char, char), count: usize) {
    let existing_count = map.entry(new_pair).or_default();
    *existing_count += count;
}

fn mutate_polymer(
    polymer: HashMap<(char, char), usize>,
    mutations: &HashMap<(char, char), char>,
) -> HashMap<(char, char), usize> {
    let mut new = HashMap::new();
    for (pair, count) in polymer {
        match mutations.get(&pair) {
            Some(&output) => {
                update(&mut new, (pair.0, output), count);
                update(&mut new, (output, pair.1), count);
            }
            None => {
                update(&mut new, pair, count);
            }
        }
    }

    new
}

fn get_input(input: &str) -> (HashMap<(char, char), usize>, HashMap<(char, char), char>) {
    let (start, input) = input.split_once("\n\n").unwrap();
    let transforms = input
        .lines()
        .map(|line| {
            let (from, to) = line.split_once(" -> ").unwrap();
            (
                from.chars().collect_tuple().unwrap(),
                to.chars().next().unwrap(),
            )
        })
        .collect();
    (start.chars().tuple_windows().counts(), transforms)
}

fn main() {
    let input = fs::read_to_string("day14/day14.txt").unwrap();
    let last = input.chars().last().unwrap();
    let (mut polymer, transforms) = get_input(&input);
    for _ in 0..40 {
        polymer = mutate_polymer(polymer, &transforms);
    }

    let first_letter_counts = polymer
        .into_iter()
        .map(|((start, _), count)| {
            if start == last {
                (start, count + 1)
            } else {
                (start, count)
            }
        })
        .into_grouping_map_by(|&(start, _)| start);
    let counts = first_letter_counts.fold(0, |acc, _key, (_, val)| acc + val);
    let minmax = counts.into_iter().minmax_by_key(|(_, count)| *count);

    let diff = match minmax {
        itertools::MinMaxResult::MinMax((_, least), (_, greatest)) => greatest - least,
        _ => unreachable!(),
    };

    eprintln!("Answer: {}", diff);
}
