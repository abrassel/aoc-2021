use itertools::Itertools;
use util::read_input;

fn main() {
    let input = read_input("inputs/day1.txt");

    let mut prev = None;
    let increasing = input
        .tuple_windows()
        .filter(|(one, two, three): &(usize, usize, usize)| {
            let sum = one + two + three;
            let is_increasing = matches!(prev, Some(prev) if prev < sum);

            prev = Some(sum);

            is_increasing
        })
        .count();

    println!("Increasing count: {}", increasing);
}
