use std::fs;
use std::io::Error;
use std::path::Path;

/// Read a file (at `path`), passing each non-empty line through the `parse`
/// callback and returning a `Result` w/ a `Vec<T>` of parsed values.
pub fn read_lines<P, F, T>(path: P, mut parse: F) -> Result<Vec<T>, Error>
where
    P: AsRef<Path>,
    F: FnMut(&str) -> T,
{
    filter_lines(path, |l| Some(parse(l)))
}

/// Read a file at (`path`), passing each non-empty line through the `parse`
/// callback and returning a `Result` w/ a `Vec<T>` of all non-`None` parsed
/// values. Note that the `Option` wrapper returned from `parse` will have
/// been removed.
pub fn filter_lines<P, F, T>(path: P, parse: F) -> Result<Vec<T>, Error>
where
    P: AsRef<Path>,
    F: FnMut(&str) -> Option<T>,
{
    Ok(fs::read_to_string(path)?
        .lines()
        .map(|l| l.trim())
        .filter_map(|l| if l.is_empty() { None } else { Some(l) })
        .filter_map(parse)
        .collect())
}
