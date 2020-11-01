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
