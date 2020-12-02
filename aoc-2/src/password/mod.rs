mod sled_password;
mod toboggan_password;

pub(crate) use sled_password::SledPassword;
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
pub(crate) use toboggan_password::TobogganPassword;

pub(crate) trait Password {
    fn validate(&self) -> bool;
}

#[derive(Debug)]
pub(crate) struct ParsePassErr;

impl Display for ParsePassErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("failed to parse password")
    }
}

impl Error for ParsePassErr {}
