use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    num::ParseIntError,
    ops::Deref,
    path::Path,
    str::FromStr,
};

use thiserror::Error;

use crate::motion::MotionParseError;

#[derive(Debug, Error)]
pub enum ProblemInputError {
    #[error("Could not open the input \"{path}\"")]
    InputOpen { path: String, source: io::Error },
    #[error("Failed to parse input file")]
    Parse { source: anyhow::Error },
}

impl From<ParseIntError> for ProblemInputError {
    fn from(e: ParseIntError) -> Self {
        ProblemInputError::Parse { source: e.into() }
    }
}

impl From<MotionParseError> for ProblemInputError {
    fn from(e: MotionParseError) -> Self {
        ProblemInputError::Parse { source: e.into() }
    }
}

#[derive(Clone, Copy)]
pub struct BinaryEncodedInt(u16);

impl std::fmt::Debug for BinaryEncodedInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:b}", self.0)
    }
}

impl Deref for BinaryEncodedInt {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for BinaryEncodedInt {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        u16::from_str_radix(s, 2).map(BinaryEncodedInt)
    }
}

pub fn load_simple_input<C, T, E>(path: impl AsRef<Path>) -> Result<C, ProblemInputError>
where
    C: FromIterator<T>,
    T: FromStr<Err = E>,
    E: Into<ProblemInputError>,
{
    let path = path.as_ref();

    let file = File::open(path).map_err(|e| ProblemInputError::InputOpen {
        path: path.display().to_string(),
        source: e,
    })?;

    BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .map(|l| l.parse())
        .collect::<Result<C, E>>()
        .map_err(|e| e.into())
}

pub fn parse_test_input<C, T, E>(input: &'static str) -> C
where
    C: FromIterator<T>,
    T: FromStr<Err = E>,
    E: Into<ProblemInputError>,
{
    match input
        .lines()
        .map(|l| l.parse())
        .collect::<Result<C, E>>()
        .map_err(|e| e.into())
    {
        Ok(c) => c,
        Err(e) => panic!("Parsing test input failed: {}", e),
    }
}
