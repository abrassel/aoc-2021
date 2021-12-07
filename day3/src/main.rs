use std::cmp::Ordering;

use util::read_input_with_parse;

#[allow(dead_code)]
pub fn find_gamma_epsilon(input: Vec<usize>, cols: usize, rows: usize) -> (usize, usize) {
    let mut gamma = 0;
    let mut epsilon = 0;
    for bitshift in 0..cols {
        let count = input.iter().map(|i| (i >> bitshift) & 0b1).sum::<usize>();
        let most_common = if 2 * count > rows { 0b1 } else { 0b0 };
        let least_common = !most_common & 0b1;
        gamma |= most_common << bitshift;
        epsilon |= least_common << bitshift;
    }

    (gamma, epsilon)
}

enum FilterMode {
    MostCommon,
    LeastCommon,
}

fn repeat_filter(mut input: Vec<usize>, cols: usize, filter_mode: FilterMode) -> usize {
    use FilterMode::*;
    use Ordering::*;
    for bitshift in (0..cols).rev() {
        if input.len() == 1 {
            break;
        }

        let (ones, zeroes): (Vec<_>, Vec<_>) = input
            .into_iter()
            .partition(|number| ((number >> bitshift) & 0b1) == 1);

        input = match (&filter_mode, ones.len().cmp(&zeroes.len())) {
            (MostCommon, Greater) | (LeastCommon, Less) | (MostCommon, Equal) => ones,
            (LeastCommon, Greater) | (MostCommon, Less) | (LeastCommon, Equal) => zeroes,
        };
    }

    input[0]
}

pub fn main() {
    let mut length = None;
    // read in binary strings
    let input = read_input_with_parse("inputs/day3.txt", |src: String| {
        length = Some(src.len());
        usize::from_str_radix(&src, 2).unwrap()
    })
    .collect::<Vec<_>>();
    let cols = length.unwrap();

    let o2 = repeat_filter(input.clone(), cols, FilterMode::MostCommon);
    let co2 = repeat_filter(input, cols, FilterMode::LeastCommon);

    println!("The answer is {} (o2: {}, co2: {})", o2 * co2, o2, co2);
}
