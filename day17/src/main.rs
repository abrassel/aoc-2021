use num_integer::Roots;
use std::fs;

struct Rect {
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize,
}

impl Rect {
    pub fn x_hits(&self, mut x: isize) -> Option<(usize, usize)> {
        debug_assert!(
            x * x + x >= 2 * self.x_min,
            "X ({}) was too small for {}",
            x,
            self.x_min
        );
        let mut pos = 0;
        let mut time = 0;
        while pos < self.x_min {
            pos += x;
            x -= 1;
            time += 1;
        }
        if pos > self.x_max {
            return None;
        }
        let start = time;
        while pos <= self.x_max {
            pos += x;
            time += 1;
            if x == 0 {
                return Some((start, usize::MAX));
            } else {
                x -= 1;
            }
        }

        Some((start, time - 1))
    }

    pub fn y_hits(&self, mut y: isize) -> Option<(usize, usize)> {
        let mut pos = 0;
        let mut time = 0;
        while pos > self.y_max {
            pos += y;
            y -= 1;
            time += 1;
        }
        let start = time;
        if pos < self.y_min {
            return None;
        }
        while pos >= self.y_min {
            pos += y;
            y -= 1;
            time += 1;
        }

        Some((start, time - 1))
    }
}

pub fn has_intersection(xhits: &(usize, usize), yhits: &(usize, usize)) -> bool {
    std::cmp::max(xhits.0, yhits.0) <= std::cmp::min(xhits.1, yhits.1)
}

fn main() {
    let input = &fs::read_to_string("day17/day17.txt").unwrap();
    let (x_min, x_max, y_min, y_max): (isize, isize, isize, isize) =
        serde_scan::scan!("target area: x={}..{}, y={}..{}" <- input).unwrap();
    let input = Rect {
        x_min,
        x_max,
        y_min,
        y_max,
    };
    // part 1
    // eprintln!("{}, {}, {}, {}", _x_min, _x_max, y_min, _y_max);

    // let max_y = (y_min + 1) * y_min / 2;
    // println!("{}", max_y)

    // pre compute x and y directions
    let x_o_min = (-1 + (1 + 8 * input.x_min).sqrt()) / 2 + 1;
    let mut x_os: fxhash::FxHashMap<_, _> = Default::default();
    for x in x_o_min..=input.x_max {
        if let Some(hits) = input.x_hits(x) {
            x_os.insert(x, hits);
        }
    }
    let y_abs = input.y_min.abs();
    let mut y_os: fxhash::FxHashMap<_, _> = Default::default();
    for y in -y_abs..y_abs {
        if let Some(hits) = input.y_hits(y) {
            y_os.insert(y, hits);
        }
    }

    let hits = x_os
        .iter()
        .map(|(_, x_hits)| {
            y_os.iter()
                .filter(|(_, y_hits)| has_intersection(x_hits, y_hits))
                .count()
        })
        .sum::<usize>();

    println!("{}", hits);
}
