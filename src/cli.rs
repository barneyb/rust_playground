use std::env;

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

pub fn aoc_filename(fallback: &str) -> String {
    if let Some(n) = util_args().next() {
        n
    } else {
        String::from(fallback)
    }
}
