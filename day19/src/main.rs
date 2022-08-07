#![feature(const_for)]

use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

use point_region::PointRegion;

mod point_region;
mod rotate;

fn parse_input() -> VecDeque<PointRegion> {
    let mut regions = VecDeque::new();
    let mut cur_point_region = PointRegion::default();
    for line in BufReader::new(File::open("../data.txt").unwrap())
        .lines()
        .map(Result::unwrap)
    {
        if line.starts_with("---") {
            regions.push_back(cur_point_region);
            cur_point_region = PointRegion::default();
        } else if line.is_empty() {
            continue;
        } else {
            cur_point_region.points.insert(line.parse().unwrap());
        }
    }

    todo!()
}

fn main() {
    let mut remaining = parse_input();
    let mut common = PointRegion::default();

    while let Some(region) = remaining.pop_front() {
        let accumulated = common.acculumate_other_region(&region);

        if !accumulated {
            remaining.push_back(region);
        }
    }

    println!("{}", common.len())
}
