use rust_playground::Utilities;
use std::{io, env};

fn main() {
    let mut args = env::args();
    args.next(); // kill command name

    let utils = Utilities::new();

    if let Some(n) = args.next() {
        if let AndThen::Exit() = handle_input(&utils, &n) {
            return;
        }
    }
    println!("Available utilities:");
    for (i, n) in utils.names().iter().enumerate() {
        println!(" {:2}) {}", i + 1, n)
    }
    loop {
        println!("Select a module (name or number, 'q' to quit): ");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Err(_) => continue,
            Ok(_) => match handle_input(&utils, &input) {
                AndThen::Next() => continue,
                AndThen::Exit() => return,
            },
        }
    }
}

fn handle_input(utils: &Utilities, input: &String) -> AndThen {
    if "q\n" == input {
        return AndThen::Exit()
    }
    let mut nori = input.trim();
    if let Ok(i) = nori.parse::<usize>() {
        if let Some(n) = utils.names().get(i - 1) {
            nori = n;
        }
    }
    if let Some(f) = utils.by_name(&nori) {
        f();
        AndThen::Exit()
    } else {
        println!("No '{}' utility is known", nori);
        AndThen::Next()
    }
}

enum AndThen {
    Next(),
    Exit(),
}
