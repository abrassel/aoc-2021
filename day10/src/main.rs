use std::fs;

fn close(c: char) -> char {
    match c {
        '{' => '}',
        '(' => ')',
        '[' => ']',
        '<' => '>',
        _ => unreachable!(),
    }
}

fn is_open(c: char) -> bool {
    match c {
        '{' | '(' | '[' | '<' => true,
        _ => false,
    }
}

fn score(line: &str) -> Option<usize> {
    let mut stack = vec![];

    for c in line.chars() {
        if is_open(c) {
            stack.push(close(c));
        } else {
            if let Some(val) = stack.pop() {
                if val != c {
                    return None;
                }
            }
        }
    }

    if stack.is_empty() {
        return None;
    }

    let mut score = 0;
    while let Some(missing) = stack.pop() {
        print!("{} ", missing);
        score = score * 5 + score_char(missing);
    }

    Some(score)
}

fn score_char(c: char) -> usize {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => unreachable!("Found {}", c),
    }
}

fn main() {
    let input = fs::read_to_string("day10/day10.txt").unwrap();
    let input = input.lines();

    let mut scores: Vec<_> = input.filter_map(score).collect();
    println!("{:?}", scores);
    scores.sort_unstable();

    println!("{}", scores[scores.len() / 2])
}
