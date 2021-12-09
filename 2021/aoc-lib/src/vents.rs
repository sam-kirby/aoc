use std::{collections::HashMap, str::FromStr};

use itertools::Itertools;
use thiserror::Error;

use crate::input::ProblemInputError;

#[derive(Debug)]
pub struct Vent {
    start: Point,
    end: Point,
}

impl Vent {
    pub fn is_aligned(&self) -> bool {
        self.start.x == self.end.x || self.start.y == self.end.y
    }
}

impl FromStr for Vent {
    type Err = ParseVentError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s
            .split(" -> ")
            .filter_map(|p| p.parse().ok())
            .collect_tuple()
            .ok_or(ParseVentError::Vent {
                vent: s.to_string(),
            })?;

        Ok(Vent { start, end })
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = ParseVentError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .split(',')
            .filter_map(|i| i.parse().ok())
            .collect_tuple()
            .ok_or(ParseVentError::Point {
                point: s.to_string(),
            })?;
        Ok(Point { x, y })
    }
}

#[derive(Debug, Error)]
pub enum ParseVentError {
    #[error("failed to parse point: {point}")]
    Point { point: String },
    #[error("failed to parse vent: {vent}")]
    Vent { vent: String },
}

impl From<ParseVentError> for ProblemInputError {
    fn from(e: ParseVentError) -> Self {
        ProblemInputError::Parse { source: e.into() }
    }
}

#[derive(Default)]
pub struct Field {
    hazard_counts: HashMap<Point, u8>,
}

impl Field {
    pub fn add_vent(
        &mut self,
        &Vent {
            start: Point { x: x1, y: y1 },
            end: Point { x: x2, y: y2 },
        }: &Vent,
    ) {
        let dx = (x2 - x1).signum();
        let dy = (y2 - y1).signum();

        let (mut x, mut y) = (x1, y1);
        while (x, y) != (x2 + dx, y2 + dy) {
            self.hazard_counts
                .entry(Point { x, y })
                .and_modify(|c| *c += 1)
                .or_insert(1);
            x += dx;
            y += dy;
        }
    }

    pub fn danger_zones(&self) -> usize {
        self.hazard_counts.values().filter(|&c| *c > 1).count()
    }
}
