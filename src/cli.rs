use std::env;

pub fn cmd_args() -> env::Args {
    let mut args = env::args();
    args.next(); // kill command name
    args
}

pub fn util_args() -> env::Args {
    let mut args = cmd_args();
    if let Some(n) = args.next() {
        if n.starts_with("-") {
            while let Some(_) = args.next() {}
        }
    }
    args
}

pub fn aoc_filename(fallback: &str) -> String {
    if let Some(n) = util_args().next() {
        n
    } else {
        String::from(fallback)
    }
}
