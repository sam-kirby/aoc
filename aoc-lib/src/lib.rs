use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    fs::File,
    io::{BufRead, BufReader},
    iter::FromIterator,
    path::Path,
    str::FromStr,
};

#[derive(Clone, Debug)]
pub enum LoadError {
    FileOpenError { path: String },
    ParseError,
}

impl Display for LoadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::FileOpenError { path } => {
                f.write_str(&format!("failed to open the file: {}", path))
            }
            Self::ParseError => f.write_str("failed to parse input"),
        }
    }
}

impl Error for LoadError {}

pub fn load_simple_input<C, T>(path: impl AsRef<Path>) -> Result<C, LoadError>
where
    C: FromIterator<T>,
    T: FromStr,
{
    let path = path.as_ref();

    if let Ok(file) = File::open(path) {
        match BufReader::new(file)
            .lines()
            .filter_map(Result::ok)
            .map(|l| l.parse())
            .collect()
        {
            Ok(vec) => Ok(vec),
            Err(_) => Err(LoadError::ParseError),
        }
    } else {
        Err(LoadError::FileOpenError {
            path: path.display().to_string(),
        })
    }
}

#[cfg(test)]
mod tests {}
