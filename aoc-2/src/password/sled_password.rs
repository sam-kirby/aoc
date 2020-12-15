use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    ops::RangeInclusive,
    str::FromStr,
};

use crate::password::{ParsePassErr, Password};

use regex::Regex;

pub(crate) struct SledPassword {
    range: RangeInclusive<usize>,
    char: char,
    pass: String,
}

impl Password for SledPassword {
    fn validate(&self) -> bool {
        self.range
            .contains(&self.pass.chars().filter(|&c| c == self.char).count())
    }
}

impl Display for SledPassword {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(&format!(
            "{}-{} {}: {}",
            self.range.start(),
            self.range.end(),
            self.char,
            self.pass
        ))
    }
}

impl FromStr for SledPassword {
    type Err = ParsePassErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static::lazy_static! {
            static ref RE: Regex = Regex::new(r"^(?P<lower_bound>\d+)-(?P<upper_bound>\d+) (?P<char>[a-z]): (?P<pass>[a-z]+)$").unwrap();
        }

        let caps = match RE.captures(s) {
            Some(caps) => caps,
            None => return Err(ParsePassErr),
        };

        let lower_bound = caps.name("lower_bound").unwrap().as_str().parse().unwrap();
        let upper_bound = caps.name("upper_bound").unwrap().as_str().parse().unwrap();

        Ok(Self {
            range: (lower_bound..=upper_bound),
            char: caps.name("char").unwrap().as_str().parse().unwrap(),
            pass: caps.name("pass").unwrap().as_str().to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{Password, SledPassword};
    use std::str::FromStr;

    #[test]
    fn parse() {
        let example = "1-5 c: ffffff";
        let pass = SledPassword::from_str(example).unwrap();
        assert_eq!(pass.range, (1..=5));
        assert_eq!(pass.char, 'c');
        assert_eq!(pass.pass, "ffffff".to_string());
    }

    #[test]
    fn validate() {
        let pass = SledPassword::from_str("1-4 c: ccccad").unwrap();
        assert!(pass.validate());
    }

    #[test]
    fn validate_2() {
        let pass = SledPassword::from_str("11-14 a: aaafaaafaaafaaa").unwrap();
        assert!(pass.validate());
    }

    #[test]
    fn validate_3() {
        let pass = SledPassword::from_str("1-4 c: cd").unwrap();
        assert!(pass.validate());
    }

    #[test]
    fn validate_fail() {
        let pass = SledPassword::from_str("1-3 a: aaaacd").unwrap();
        assert!(!pass.validate());
    }

    #[test]
    fn validate_fail_2() {
        let pass = SledPassword::from_str("2-3 d: aaaacd").unwrap();
        assert!(!pass.validate());
    }
}
