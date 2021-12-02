use std::{
    iter::Sum,
    num::ParseIntError,
    ops::{Add, Mul},
    str::FromStr,
};

use thiserror::Error;

/// Simple Cartesian Vector implementation
/// Good for part 2.1 but not how submarines work!
#[derive(Debug, Default)]
pub struct CartVec {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Add<CartVec> for CartVec {
    type Output = CartVec;

    /// Vector addition
    fn add(self, rhs: CartVec) -> Self::Output {
        CartVec {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Mul<CartVec> for i64 {
    type Output = CartVec;

    /// Scalar * vector
    fn mul(self, rhs: CartVec) -> Self::Output {
        CartVec {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Sum for CartVec {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(CartVec::default(), |t, v| t + v)
    }
}

impl From<(i64, i64, i64)> for CartVec {
    fn from((x, y, z): (i64, i64, i64)) -> Self {
        CartVec { x, y, z }
    }
}

#[derive(Debug, Error)]
pub enum MotionParseError {
    #[error("encountered invalid direction: {found}")]
    Direction { found: String },
    #[error("Failed to parse distance")]
    Distance { source: ParseIntError },
    #[error("Source format invalid - missing space")]
    NoSpace,
}

#[derive(Debug)]
pub enum Direction {
    Forward,
    Up,
    Down,
}

impl Direction {
    /// 3 dimensional right handed Cartesian coordinates
    /// Positive z is increased depth, x is forwards, y is right
    pub fn to_cart_vec(&self) -> CartVec {
        match self {
            Direction::Forward => (1, 0, 0),
            Direction::Up => (0, 0, -1),
            Direction::Down => (0, 0, 1),
        }
        .into()
    }
}

impl FromStr for Direction {
    type Err = MotionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => Ok(Self::Forward),
            "up" => Ok(Self::Up),
            "down" => Ok(Self::Down),
            s => Err(MotionParseError::Direction {
                found: s.to_string(),
            }),
        }
    }
}

/// Struct representing submarine's motion vector
#[derive(Debug)]
pub struct MotionVec {
    pub dir: Direction,
    pub dis: i64,
}

impl FromStr for MotionVec {
    type Err = MotionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, dis) = {
            let (dir_str, dis_str) = s.split_once(' ').ok_or(MotionParseError::NoSpace)?;
            (
                dir_str.parse()?,
                dis_str
                    .parse()
                    .map_err(|e| MotionParseError::Distance { source: e })?,
            )
        };

        Ok(MotionVec { dir, dis })
    }
}

/// Struct representing a submarine. Can add [`MotionVec`]s to change its state
#[derive(Debug, Default)]
pub struct Submarine {
    pub x: i64,
    pub d: i64,
    pub a: i64,
}

impl Add<&MotionVec> for Submarine {
    type Output = Submarine;

    fn add(self, rhs: &MotionVec) -> Self::Output {
        match rhs.dir {
            Direction::Forward => Submarine {
                x: self.x + rhs.dis,
                d: self.d + self.a * rhs.dis,
                a: self.a,
            },
            Direction::Up => Submarine {
                x: self.x,
                d: self.d,
                a: self.a - rhs.dis,
            },
            Direction::Down => Submarine {
                x: self.x,
                d: self.d,
                a: self.a + rhs.dis,
            },
        }
    }
}
