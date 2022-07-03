use std::{
    convert::Infallible,
    fmt::Debug,
    iter::{Peekable, Sum},
    ops::Add,
    str::FromStr,
};

use itertools::Itertools;

#[derive(Clone)]
struct SfDatum {
    depth: u8,
    value: u8,
}

impl Debug for SfDatum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.depth, self.value)
    }
}

#[derive(Debug, Clone)]
struct SfNumber(Vec<SfDatum>);

impl SfNumber {
    fn explode(&mut self, i: usize) {
        // we have found the first of the pair, so i = left, i+1 = right.
        // always pair, so always guaranteed right.
        let lval = self.0[i].value;
        let rval = self.0[i + 1].value;

        // try to add to the pred and succ if they exist.
        if let Some(pred) = i.checked_sub(1).map(|i| &mut self.0[i]) {
            pred.value += lval;
        }
        if let Some(succ) = self.0.get_mut(i + 2) {
            succ.value += rval;
        }
        // replace oneself with 0.
        self.0.remove(i + 1);
        self.0[i] = SfDatum { depth: 4, value: 0 };
    }

    fn split(&mut self, i: usize) {
        let SfDatum { depth, value } = self.0[i];
        let new_left = SfDatum {
            depth: depth + 1,
            value: value / 2,
        };
        let new_right = SfDatum {
            depth: depth + 1,
            value: (value + 1) / 2,
        };
        self.0[i] = new_left;
        self.0.insert(i + 1, new_right);
    }

    fn reduce(&mut self) {
        // explode
        for i in 0..self.0.len() {
            if self.0[i].depth == 5 {
                self.explode(i);
                return self.reduce();
            }
        }

        // split
        for i in 0..self.0.len() {
            if self.0[i].value > 9 {
                self.split(i);
                return self.reduce();
            }
        }
    }

    #[inline(always)]
    pub fn magnitude(&self) -> u64 {
        fn magnitude_rec<'a>(
            nums: &mut Peekable<impl Iterator<Item = &'a SfDatum>>,
            depth: u8,
        ) -> u64 {
            fn next<'a>(nums: &mut Peekable<impl Iterator<Item = &'a SfDatum>>, depth: u8) -> u64 {
                if nums.peek().unwrap().depth == depth {
                    // in this case, no nested number
                    nums.next().unwrap().value.into()
                } else {
                    magnitude_rec(nums, depth + 1)
                }
            }
            let left = next(nums, depth);
            let right = next(nums, depth);
            3 * left + 2 * right
        }
        magnitude_rec(&mut self.0.iter().peekable(), 1)
    }
}

impl Add for SfNumber {
    type Output = Self;

    fn add(mut self, mut rhs: Self) -> Self::Output {
        self.0.append(&mut rhs.0);
        for SfDatum { depth, .. } in &mut self.0 {
            *depth += 1;
        }

        self.reduce();
        self
    }
}

impl Sum for SfNumber {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|accum, item| accum + item).unwrap()
    }
}

impl FromStr for SfNumber {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.bytes()
                .fold(
                    (0, Vec::with_capacity(s.len() / 2)),
                    |(mut depth, mut num), b| {
                        match b {
                            b'[' => depth += 1,
                            b']' => depth -= 1,
                            b'0'..=b'9' => num.push(SfDatum {
                                depth,
                                value: b - b'0',
                            }),
                            _ => {}
                        };
                        (depth, num)
                    },
                )
                .1,
        ))
    }
}

fn main() {
    // part 1
    // let sum: SfNumber = include_str!("../day18.txt")
    //     .lines()
    //     .map(|line| line.parse().unwrap())
    //     .sum();
    // println!("{}", sum.magnitude())

    // part 2
    let largest = include_str!("../day18.txt")
        .lines()
        .map(|line| line.parse::<SfNumber>().unwrap())
        .permutations(2)
        .map(|summands| summands.into_iter().sum::<SfNumber>().magnitude())
        .max()
        .unwrap();

    println!("{}", largest);
}

#[cfg(test)]
mod tests {
    use crate::SfNumber;

    #[test]
    fn test_mag() {
        let num = "[9,1]".parse::<SfNumber>().unwrap();
        assert_eq!(num.magnitude(), 29);
    }

    #[test]
    fn test_mag_complex() {
        let num = "[[9,1],[1,9]]".parse::<SfNumber>().unwrap();
        assert_eq!(num.magnitude(), 129);
    }
}
