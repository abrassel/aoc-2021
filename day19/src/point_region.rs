use std::{collections::HashSet, convert::Infallible, str::FromStr};

use nalgebra::Point3;

fn parse_point(s: &str) -> Point3<i64> {
    let data: [i64; 3] = s
        .split(',')
        .map(|part| part.parse().unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    data.into()
}

#[derive(Default)]
pub struct PointRegion {
    pub points: HashSet<Point3<i64>>,
}

impl PointRegion {
    pub fn find_other_orientation(&self, other: &Self) -> Option<Self> {}

    pub fn acculumate_other_region(&mut self, other: &Self) -> bool {
        if let Some(oriented) = self.find_other_orientation(other) {
            self.points.extend(oriented.points.into_iter());
            return true;
        }

        false
    }

    pub fn len(&self) -> usize {
        self.points.len()
    }
}
