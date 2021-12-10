use std::{collections::HashSet, str::FromStr};

use aoc_lib::input::{load_simple_input, ProblemInputError};
use thiserror::Error;

#[derive(Default, Clone, Copy)]
struct SegDisplay {
    state: [bool; 7],
}

impl SegDisplay {
    fn count_active(&self) -> u8 {
        self.state.iter().fold(0, |t, s| if *s { t + 1 } else { t })
    }

    fn is_active(&self, segment: usize) -> bool {
        self.state[segment]
    }

    fn active_indicies(&self) -> Vec<usize> {
        let mut active = Vec::new();
        for (segment, _) in self.state.iter().enumerate().filter(|(_, s)| **s) {
            active.push(segment);
        }
        active
    }

    fn remap(&mut self, map: &[usize; 7]) {
        let mut new_state = [false; 7];

        for (seg, active) in self.state.iter().enumerate() {
            if *active {
                new_state[map[seg]] = true;
            }
        }

        self.state = new_state;
    }

    fn identify(&self) -> Option<u16> {
        const ZERO: [bool; 7] = [true, true, true, false, true, true, true];
        const ONE: [bool; 7] = [false, false, true, false, false, true, false];
        const TWO: [bool; 7] = [true, false, true, true, true, false, true];
        const THREE: [bool; 7] = [true, false, true, true, false, true, true];
        const FOUR: [bool; 7] = [false, true, true, true, false, true, false];
        const FIVE: [bool; 7] = [true, true, false, true, false, true, true];
        const SIX: [bool; 7] = [true, true, false, true, true, true, true];
        const SEVEN: [bool; 7] = [true, false, true, false, false, true, false];
        const EIGHT: [bool; 7] = [true, true, true, true, true, true, true];
        const NINE: [bool; 7] = [true, true, true, true, false, true, true];

        match self.state {
            ZERO => Some(0),
            ONE => Some(1),
            TWO => Some(2),
            THREE => Some(3),
            FOUR => Some(4),
            FIVE => Some(5),
            SIX => Some(6),
            SEVEN => Some(7),
            EIGHT => Some(8),
            NINE => Some(9),
            _ => None,
        }
    }
}

impl FromStr for SegDisplay {
    type Err = ParseSegDisplayError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut state = [false; 7];
        for char in s.chars() {
            match char {
                ch @ 'a'..='g' => {
                    let index = ch as usize - 'a' as usize;
                    state[index] = true;
                }
                ch => return Err(ParseSegDisplayError::InvalidChar { ch }),
            }
        }

        Ok(SegDisplay { state })
    }
}

struct TestPattern {
    patterns: [SegDisplay; 10],
}

impl TestPattern {
    fn segment_counts(&self) -> [usize; 7] {
        let mut counts = [0; 7];

        for display in &self.patterns {
            for (i, c) in counts.iter_mut().enumerate() {
                if display.is_active(i) {
                    *c += 1;
                }
            }
        }

        counts
    }

    fn generate_map(&self) -> [usize; 7] {
        let mut map = [0; 7];
        let mut matched_segs = HashSet::new();
        let counts = self.segment_counts();

        let one = self
            .patterns
            .iter()
            .find(|d| d.count_active() == 2)
            .expect("There is no pattern that could be one")
            .active_indicies();

        for seg in one {
            if counts[seg] == 8 {
                map[seg] = 2;
                matched_segs.insert(seg);
            } else {
                map[seg] = 5;
                matched_segs.insert(seg);
            }
        }

        let seven = self
            .patterns
            .iter()
            .find(|d| d.count_active() == 3)
            .expect("There is no pattern that could be 7")
            .active_indicies();

        for seg in seven {
            if matched_segs.contains(&seg) {
                continue;
            }
            map[seg] = 0;
            matched_segs.insert(seg);
        }

        let four = self
            .patterns
            .iter()
            .find(|d| d.count_active() == 4)
            .expect("There is no pattern that could be 4")
            .active_indicies();

        for seg in four {
            if matched_segs.contains(&seg) {
                continue;
            }
            if counts[seg] == 7 {
                map[seg] = 3;
                matched_segs.insert(seg);
            }
            if counts[seg] == 6 {
                map[seg] = 1;
                matched_segs.insert(seg);
            }
        }

        for (seg, count) in counts.iter().enumerate() {
            if matched_segs.contains(&seg) {
                continue;
            }

            if *count == 4 {
                map[seg] = 4;
            }
            if *count == 7 {
                map[seg] = 6;
            }
        }

        map
    }
}

impl FromStr for TestPattern {
    type Err = ParseSegDisplayError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut patterns = [SegDisplay::default(); 10];
        let pattern_strs: Vec<_> = s.split(' ').collect();
        for (i, d) in patterns.iter_mut().enumerate() {
            *d = pattern_strs
                .get(i)
                .ok_or(ParseSegDisplayError::ShortTest)?
                .parse()?;
        }
        Ok(TestPattern { patterns })
    }
}

struct Output([u16; 4]);

impl Output {
    fn to_int(&self) -> u64 {
        (self.0[0] * 1000 + self.0[1] * 100 + self.0[2] * 10 + self.0[3]).into()
    }
}

impl FromStr for Output {
    type Err = ParseSegDisplayError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (test_pattern, outs) = s
            .split_once(" | ")
            .ok_or(ParseSegDisplayError::MissingSeparator)?;

        let test_pattern = test_pattern.parse::<TestPattern>()?;

        let map = test_pattern.generate_map();

        let mut output = [0; 4];

        for (i, out) in outs.split(' ').map(|s| s.parse::<SegDisplay>()).enumerate() {
            let mut out = out?;
            out.remap(&map);
            output[i] = out.identify().ok_or(ParseSegDisplayError::NoDigit)?;
        }

        Ok(Output(output))
    }
}

#[derive(Debug, Error)]
enum ParseSegDisplayError {
    #[error("This display does not have enough segments for {ch}")]
    InvalidChar { ch: char },
    #[error("Missing output separator")]
    MissingSeparator,
    #[error("Test pattern too short")]
    ShortTest,
    #[error("An output pattern did not match any digit")]
    NoDigit,
}

impl From<ParseSegDisplayError> for ProblemInputError {
    fn from(source: ParseSegDisplayError) -> Self {
        ProblemInputError::Parse {
            source: source.into(),
        }
    }
}

fn solve_part1(outputs: &[Output]) -> u64 {
    outputs.iter().fold(0, |t, o| {
        o.0.iter().fold(t, |t, d| match *d {
            1 | 4 | 7 | 8 => t + 1,
            _ => t,
        })
    })
}

fn solve_part2(outputs: &[Output]) -> u64 {
    outputs.iter().map(|o| o.to_int()).sum()
}

fn main() -> anyhow::Result<()> {
    let outputs: Vec<_> = load_simple_input("inputs/8.txt")?;
    println!(
        "There are {} trivially identifiable digits in the output",
        solve_part1(&outputs)
    );
    println!("The sum of the outputs is: {}", solve_part2(&outputs));

    Ok(())
}

#[cfg(test)]
mod tests {
    use aoc_lib::input::parse_test_input;

    use super::{solve_part1, solve_part2, Output};

    const TEST_INPUT: &'static str = include_str!("test.txt");

    #[test]
    fn test_part1() {
        let outputs: Vec<Output> = parse_test_input(TEST_INPUT);
        assert_eq!(26, solve_part1(&outputs));
    }

    #[test]
    fn test_part2() {
        let outputs: Vec<Output> = parse_test_input(TEST_INPUT);
        assert_eq!(61229, solve_part2(&outputs));
    }
}
