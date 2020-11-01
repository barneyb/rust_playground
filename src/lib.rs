use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::Error;
use std::path::Path;

mod guess_number;
mod temp_convert;
mod aoc_2019_01;

pub fn cmd_args() -> env::Args {
    let mut args = env::args();
    args.next(); // kill command name
    args
}

pub fn util_args() -> env::Args {
    let mut args = cmd_args();
    args.next(); // kill utility name
    args
}

/// Read a file (at `path`), passing each non-empty line through the `parse`
/// callback and returning a `Result` w/ a `Vec<T>` of parsed values.
fn read_lines<P, F, T>(path: P, mut parse: F) -> Result<Vec<T>, Error>
    where P: AsRef<Path>,
          F: FnMut(&str) -> T
{
    filter_lines(path, |l| Some(parse(l)))
}

/// Read a file at (`path`), passing each non-empty line through the `parse`
/// callback and returning a `Result` w/ a `Vec<T>` of all non-`None` parsed
/// values. Note that the `Option` wrapper returned from `parse` will have
/// been removed.
fn filter_lines<P, F, T>(path: P, parse: F) -> Result<Vec<T>, Error>
    where P: AsRef<Path>,
        F: FnMut(&str) -> Option<T>
{
    Ok(fs::read_to_string(path)?
        .lines()
        .map(|l| l.trim())
        .filter_map(|l| if l.is_empty() {
            None
        } else {
            Some(l)
        })
        .filter_map(parse)
        .collect())
}

pub struct Utilities<'a> {
    utilities: HashMap<&'a str, fn()>,
    names: Vec<&'a str>,
}

impl<'a> Utilities<'a> {

    pub fn new() -> Utilities<'a> {
        let mut utilities: HashMap<&str, fn()> = HashMap::new();
        // todo: "add Sally to Engineering" / "show Engineering" interface
        // todo: convert strings to pig latin
        // todo: mean/median/mode of list of integers
        // todo: 12 days of christmas
        // todo: fibonacci
        utilities.insert("temp", temp_convert::run);
        utilities.insert("guess", guess_number::run);
        utilities.insert("AoC 2019-01", aoc_2019_01::run);
        let mut names: Vec<&str> = utilities.iter()
            .map(|(k, _)| *k)
            .collect();
        names.sort_unstable();
        Utilities {
            utilities,
            names,
        }
    }

    pub fn names(&self) -> &Vec<&str> {
        &self.names
    }

    pub fn by_name(&self, name: &str) -> Option<&fn()> {
        self.utilities.get(name)
    }

}
