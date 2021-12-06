use std::{
    collections::HashSet,
    fmt::{Debug, Display},
    str::FromStr,
    vec::IntoIter,
};

use bimap::BiHashMap;
use once_cell::sync::Lazy;
use thiserror::Error;

pub struct Table {
    data: BiHashMap<u8, u8>, // positon, value
    called: HashSet<u8>,
}

impl Table {
    pub fn new(rows: &[Vec<u8>]) -> Self {
        let data = rows
            .iter()
            .flatten()
            .enumerate()
            .map(|(p, &n)| (p as u8, n))
            .collect();

        let called = HashSet::new();

        Table { data, called }
    }

    pub fn call(&mut self, called: u8) -> bool {
        if let Some(&i) = self.data.get_by_right(&called) {
            self.called.insert(i)
        } else {
            false
        }
    }

    pub fn score(&self) -> Option<u64> {
        if !self.has_won() {
            return None;
        }

        Some(
            self.data
                .iter()
                .filter(|(p, _)| !self.called.contains(p))
                .fold(0, |t, (_, &v)| t + v as u64),
        )
    }

    fn has_won(&self) -> bool {
        static ROWS: Lazy<[Vec<u8>; 5]> = Lazy::new(|| {
            [
                (0..5).collect(),
                (5..10).collect(),
                (10..15).collect(),
                (15..20).collect(),
                (20..25).collect(),
            ]
        });
        static COLUMNS: Lazy<[Vec<u8>; 5]> = Lazy::new(|| {
            [
                (0..5).map(|x| x * 5).collect(),
                (0..5).map(|x| x * 5 + 1).collect(),
                (0..5).map(|x| x * 5 + 2).collect(),
                (0..5).map(|x| x * 5 + 3).collect(),
                (0..5).map(|x| x * 5 + 4).collect(),
            ]
        });

        for row in ROWS.iter() {
            if row.iter().all(|i| self.called.contains(i)) {
                return true;
            }
        }

        for column in COLUMNS.iter() {
            if column.iter().all(|i| self.called.contains(i)) {
                return true;
            }
        }

        false
    }
}

impl Debug for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in (0..25)
            .filter_map(|i| self.data.get_by_left(&i))
            .collect::<Vec<_>>()
            .chunks(5)
        {
            if let Err(e) = writeln!(f, "{} {} {} {} {}", i[0], i[1], i[2], i[3], i[4]) {
                return Err(e);
            }
        }

        Ok(())
    }
}

impl Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in (0..25)
            .filter_map(|i| self.data.get_by_left(&i))
            .collect::<Vec<_>>()
            .chunks(5)
        {
            if let Err(e) = writeln!(f, "{} {} {} {} {}", i[0], i[1], i[2], i[3], i[4]) {
                return Err(e);
            }
        }

        Ok(())
    }
}

pub struct Hall {
    balls: IntoIter<u8>,
    tables: Vec<Table>,
}

impl Hall {
    pub fn new(balls: Vec<u8>, mut tables: Vec<Table>) -> Self {
        let mut balls = balls.into_iter();
        for b in (&mut balls).take(4) {
            tables.iter_mut().for_each(|t| {
                t.call(b);
            });
        }
        Hall { balls, tables }
    }
}

#[derive(Debug, Error)]
pub enum BingoParseError {
    #[error("input file does not list the upcoming balls")]
    MissingBalls,
}

impl FromStr for Hall {
    type Err = BingoParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split("\n\n");

        let balls = parts
            .next()
            .ok_or(BingoParseError::MissingBalls)?
            .split(',')
            .filter_map(|n| n.parse().ok())
            .collect();

        let tables = parts
            .map(|chunk| {
                chunk
                    .split('\n')
                    .map(|line| {
                        line.split(' ')
                            .filter_map(|num| num.parse::<u8>().ok())
                            .collect()
                    })
                    .collect()
            })
            .map(|t: Vec<_>| Table::new(&t))
            .collect();

        Ok(Hall::new(balls, tables))
    }
}

impl Iterator for Hall {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.tables.is_empty() {
                return None;
            }

            let b = match self.balls.next() {
                Some(b) => b,
                None => panic!("Ran out of balls while game is still being played"),
            };

            let mut scores = Vec::new();

            let mut finished_games = self
                .tables
                .iter_mut()
                .enumerate()
                .filter_map(|(i, t)| match t.call(b) {
                    true => Some((i, t)),
                    false => None,
                })
                .filter_map(|(i, t)| t.score().map(|s| (i, s * b as u64)))
                .collect::<Vec<_>>();

            finished_games.sort_by(|(ia, _), (ib, _)| ib.cmp(ia)); // sort by decreasing index so removal is safe

            for (i, s) in finished_games {
                self.tables.remove(i);
                scores.push(s);
            }

            if !scores.is_empty() {
                return scores.into_iter().max();
            }
        }
    }
}
