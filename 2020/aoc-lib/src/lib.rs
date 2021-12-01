use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    fs::File,
    io::{BufRead, BufReader},
    iter::FromIterator,
    path::Path,
    str::FromStr,
};

mod macros;

#[derive(Debug)]
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

/// Loads a file where each line of the file represents an object, parsing as we go and returning a
/// collection of these objects.
/// This function returns an error if parsing fails.
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

/// Load a collection of objects from a file where a single object may be spread across multiple
/// lines, and a single blank line divides objects.
/// New line characters are either omitted if `replace_newline` is `None` or replaced by the str if
/// `replace_newline` is `Some(&str)`.
/// This function does not return an error if parsing fails.
pub fn load_split_input<C, T>(
    path: impl AsRef<Path>,
    replace_newline: Option<&str>,
) -> Result<C, LoadError>
where
    C: FromIterator<Result<T, <T as FromStr>::Err>>,
    T: FromStr,
{
    let path = path.as_ref();

    if let Ok(file) = File::open(path) {
        Ok(BufReader::new(file)
            .lines()
            .filter_map(Result::ok)
            .split_blank_lines(replace_newline)
            .map(|s| s.parse())
            .collect())
    } else {
        Err(LoadError::FileOpenError {
            path: path.display().to_string(),
        })
    }
}

struct SplitBlankLineIterator<'a, I: Iterator<Item = String>> {
    replace_newline: Option<&'a str>,
    iter: I,
}

impl<'a, I: Iterator<Item = String>> SplitBlankLineIterator<'a, I> {
    fn new(iter: I, replace_newline: Option<&'a str>) -> Self {
        SplitBlankLineIterator {
            replace_newline,
            iter,
        }
    }
}

impl<'a, I: Iterator<Item = String>> Iterator for SplitBlankLineIterator<'a, I> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let replace_newline = &self.replace_newline;
        let mut out = self
            .iter
            .by_ref()
            .take_while(|s| s != "") // Note: consumes blank line
            .fold(String::new(), |mut s, l| {
                s.push_str(&l);
                if let Some(replacement) = replace_newline.as_deref() {
                    s.push_str(replacement);
                }
                s
            });
        if let Some(replacement) = replace_newline.as_deref() {
            out = out.trim_end_matches(replacement).to_owned();
        }
        if !out.is_empty() {
            Some(out)
        } else {
            None
        }
    }
}

trait SplitBlankLine<'a>: Iterator<Item = String> + Sized {
    fn split_blank_lines(
        self,
        replace_newline: Option<&'a str>,
    ) -> SplitBlankLineIterator<'a, Self>;
}

impl<'a, I: Iterator<Item = String>> SplitBlankLine<'a> for I {
    fn split_blank_lines(
        self,
        replace_newline: Option<&'a str>,
    ) -> SplitBlankLineIterator<'a, Self> {
        SplitBlankLineIterator::new(self, replace_newline)
    }
}

#[cfg(test)]
mod tests {}
