use std::fs::read_to_string;

fn main() {
    let input = read_to_string("day7/day7.txt").unwrap();
    let input = input
        .split(',')
        .map(|elm| elm.parse().unwrap())
        .collect::<Vec<i32>>();
    // input.sort_unstable();
    let score = |val: i32| {
        input
            .iter()
            .map(|elm| {
                let dist = (elm - val).abs();
                (dist * dist + dist) / 2
            })
            .sum::<i32>()
    };

    // let middle = input.len() / 2;
    // let upper_score = score(input[middle]);

    // let score = if middle % 2 == 0 {
    //     let lower = input[middle - 1];
    //     let lower_score = score(lower);
    //     std::cmp::min(lower_score, upper_score)
    // } else {
    //     upper_score
    // };
    let avg = input.iter().sum::<i32>() / input.len() as i32;
    let score = std::cmp::min(score(avg), score(avg + 1));

    println!("Score: {}", score)
}
