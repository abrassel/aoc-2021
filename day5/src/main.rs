use serde::Deserialize;
use serde_scan::ScanError;
use std::{collections::HashMap, fmt::Display, str::FromStr};

#[derive(Deserialize, Debug)]
struct Line {
    start: (i32, i32),
    stop: (i32, i32),
}

impl Line {
    pub fn iter(&self) -> LineIter {
        LineIter::new(self)
    }
}

#[derive(Debug)]
struct LineIter {
    pos: (i32, i32),
    step: (i32, i32),
    stop: (i32, i32),
    complete: bool,
}

impl LineIter {
    pub fn new(line: &Line) -> Self {
        let x_step = num::signum(line.stop.0 - line.start.0);
        let y_step = num::signum(line.stop.1 - line.start.1);

        Self {
            pos: line.start,
            step: (x_step, y_step),
            stop: line.stop,
            complete: false,
        }
    }
}

impl Iterator for LineIter {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.complete {
            return None;
        }
        if self.pos == self.stop {
            self.complete = true;
        }

        let cur = self.pos;
        {
            self.pos.0 += self.step.0;
            self.pos.1 += self.step.1;
        }
        Some(cur)
    }
}

impl FromStr for Line {
    type Err = ScanError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_scan::scan!("{},{} -> {},{}" <- s)
    }
}

#[derive(Debug)]
struct Ocean(HashMap<(i32, i32), i32>);

impl Display for Ocean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_x = self.0.keys().map(|(x, _)| x).max().unwrap();
        let max_y = self.0.keys().map(|(y, _)| y).max().unwrap();

        for x in 0..=*max_x {
            for y in 0..*max_y {
                match self.0.get(&(x, y)) {
                    Some(value) => write!(f, "{}", value)?,
                    None => write!(f, ".")?,
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl FromIterator<Line> for Ocean {
    fn from_iter<T: IntoIterator<Item = Line>>(iter: T) -> Self {
        let mut ocean = HashMap::new();
        for line in iter.into_iter() {
            for square in line.iter() {
                *ocean.entry(square).or_default() += 1;
            }
        }

        Self(ocean)
    }
}

fn main() {
    let lines = util::read_input("day5/day5.txt");
    let ocean = Ocean::from_iter(lines);
    let danger = ocean.0.values().filter(|&&v| v >= 2).count();

    println!("{}", danger);
}
