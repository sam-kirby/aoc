use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    str::FromStr,
};

use aoc_lib::regex;

use crate::password::{ParsePassErr, Password};

pub(crate) struct TobogganPassword {
    first_idx: usize,
    second_idx: usize,
    char: char,
    pass: String,
}

impl Password for TobogganPassword {
    fn validate(&self) -> bool {
        self.pass
            .chars()
            .enumerate()
            .filter(|&(idx, char)| {
                (idx == self.first_idx || idx == self.second_idx) && char == self.char
            })
            .count()
            == 1
    }
}

impl Display for TobogganPassword {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(&format!(
            "{}-{} {}: {}",
            self.first_idx + 1,
            self.second_idx + 1,
            self.char,
            self.pass
        ))
    }
}

impl FromStr for TobogganPassword {
    type Err = ParsePassErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = regex!(r"^(?P<idx_1>\d+)-(?P<idx_2>\d+) (?P<char>[a-z]): (?P<pass>[a-z]+)$");

        let caps = match re.captures(s) {
            Some(caps) => caps,
            None => return Err(ParsePassErr),
        };

        let first_idx = caps
            .name("idx_1")
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap()
            - 1;
        let second_idx = caps
            .name("idx_2")
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap()
            - 1;

        Ok(Self {
            first_idx,
            second_idx,
            char: caps.name("char").unwrap().as_str().parse().unwrap(),
            pass: caps.name("pass").unwrap().as_str().to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::{Password, TobogganPassword};

    #[test]
    fn parse() {
        let example = "1-5 c: ffffff";
        let pass = TobogganPassword::from_str(example).unwrap();
        assert_eq!(pass.first_idx, 0);
        assert_eq!(pass.second_idx, 4);
        assert_eq!(pass.char, 'c');
        assert_eq!(pass.pass, "ffffff".to_string());
    }

    #[test]
    fn validate() {
        let pass = TobogganPassword::from_str("1-4 c: cccacd").unwrap();
        assert!(pass.validate());
    }

    #[test]
    fn validate_2() {
        let pass = TobogganPassword::from_str("12-14 a: aaafaaafaaafaaa").unwrap();
        assert!(pass.validate());
    }

    #[test]
    fn validate_3() {
        let pass = TobogganPassword::from_str("1-4 c: cd").unwrap();
        assert!(pass.validate());
    }

    #[test]
    fn validate_fail() {
        let pass = TobogganPassword::from_str("1-3 a: aaaacd").unwrap();
        assert!(!pass.validate());
    }

    #[test]
    fn validate_fail_2() {
        let pass = TobogganPassword::from_str("2-3 d: aaaacd").unwrap();
        assert!(!pass.validate());
    }
}
