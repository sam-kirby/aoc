use std::{
    collections::HashMap,
    error::Error,
    fmt::{self, Display, Formatter},
    str::FromStr,
};

use aoc_lib::regex;

#[derive(Debug)]
pub struct Passport {
    birth_year: u16,
    issue_year: u16,
    expiration_year: u16,
    height: Height,
    hair_colour: String,
    eye_colour: String,
    id: String,
    country_id: Option<u16>,
}

impl Passport {
    pub fn passes_enhanced_validation(&self) -> bool {
        if self.birth_year < 1920 || self.birth_year > 2002 {
            return false;
        }

        if self.issue_year < 2010 || self.issue_year > 2020 {
            return false;
        }

        if self.expiration_year < 2020 || self.expiration_year > 2030 {
            return false;
        }

        match self.height {
            Height::Centimetres(height) => {
                if height < 150 || height > 193 {
                    return false;
                }
            }
            Height::Inches(height) => {
                if height < 59 || height > 76 {
                    return false;
                }
            }
            Height::UndefinedUnit => return false,
        }

        let re = regex!(r"^#[0-9a-f]{6}");

        if !re.is_match(&self.hair_colour) {
            return false;
        }

        if !["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&self.eye_colour.as_ref()) {
            return false;
        }

        if self.id.len() != 9 || self.id.parse::<u32>().is_err() {
            return false;
        }

        true
    }
}

impl FromStr for Passport {
    type Err = PassportValidationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let kvps = s
            .split_whitespace()
            .map(|kvp| {
                let mut kvp = kvp.split(':');
                if let (Some(key), Some(val)) = (kvp.next(), kvp.next()) {
                    Ok((key, val))
                } else {
                    Err(PassportValidationError::FormatError)
                }
            })
            .collect::<Result<HashMap<_, _>, _>>()?;

        let birth_year = kvps
            .get("byr")
            .ok_or(PassportValidationError::MissingBirthYear)?
            .parse()
            .map_err(|_| PassportValidationError::IntParseError)?;
        let issue_year = kvps
            .get("iyr")
            .ok_or(PassportValidationError::MissingIssueYear)?
            .parse()
            .map_err(|_| PassportValidationError::IntParseError)?;
        let expiration_year = kvps
            .get("eyr")
            .ok_or(PassportValidationError::MissingExpirationYear)?
            .parse()
            .map_err(|_| PassportValidationError::IntParseError)?;
        let height = kvps
            .get("hgt")
            .ok_or(PassportValidationError::MissingHeight)?
            .parse()?;
        let hair_colour = kvps
            .get("hcl")
            .ok_or(PassportValidationError::MissingHairColour)?
            .to_string();
        let eye_colour = kvps
            .get("ecl")
            .ok_or(PassportValidationError::MissingEyeColour)?
            .to_string();
        let id = kvps
            .get("pid")
            .ok_or(PassportValidationError::MissingId)?
            .to_string();
        let country_id = kvps
            .get("cid")
            .map(|s| {
                s.parse()
                    .map_err(|_| PassportValidationError::IntParseError)
            })
            .transpose()?;

        Ok(Passport {
            birth_year,
            issue_year,
            expiration_year,
            height,
            hair_colour,
            eye_colour,
            id,
            country_id,
        })
    }
}

#[derive(Debug)]
pub enum Height {
    Inches(u8),
    Centimetres(u8),
    UndefinedUnit,
}

impl FromStr for Height {
    type Err = PassportValidationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains("cm") {
            Ok(Height::Centimetres(
                s.trim_end_matches("cm")
                    .parse()
                    .map_err(|_| PassportValidationError::HeightParseError)?,
            ))
        } else if s.contains("in") {
            Ok(Height::Inches(
                s.trim_end_matches("in")
                    .parse()
                    .map_err(|_| PassportValidationError::HeightParseError)?,
            ))
        } else if s.parse::<u8>().is_ok() {
            Ok(Height::UndefinedUnit)
        } else {
            Err(PassportValidationError::HeightParseError)
        }
    }
}

#[derive(Debug)]
pub enum PassportValidationError {
    MissingBirthYear,
    MissingIssueYear,
    MissingExpirationYear,
    MissingHeight,
    MissingHairColour,
    MissingEyeColour,
    MissingId,
    HeightParseError,
    IntParseError,
    FormatError,
}

impl Display for PassportValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            PassportValidationError::MissingBirthYear => {
                write!(f, "Missing required field: Birth Year")
            }
            PassportValidationError::MissingIssueYear => {
                write!(f, "Missing required field: Issue Year")
            }
            PassportValidationError::MissingExpirationYear => {
                write!(f, "Missing required field: Expiration Year")
            }
            PassportValidationError::MissingHeight => {
                write!(f, "Missing required field: Height")
            }
            PassportValidationError::MissingHairColour => {
                write!(f, "Missing required field: Hair Colour")
            }
            PassportValidationError::MissingEyeColour => {
                write!(f, "Missing required field: Eye Colour")
            }
            PassportValidationError::MissingId => {
                write!(f, "Missing required field: Passport ID")
            }
            PassportValidationError::HeightParseError => {
                write!(f, "Failed to parse height!")
            }
            PassportValidationError::IntParseError => {
                write!(f, "Failed to parse int!")
            }
            PassportValidationError::FormatError => {
                write!(f, "Unexpected or missing colon")
            }
        }
    }
}

impl Error for PassportValidationError {}
