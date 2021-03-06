use std::io;

use rust_playground::cli;
use rust_playground::Utilities;

fn main() {
    let utils = Utilities::new();

    if let Some(n) = cli::cmd_args().next() {
        match handle_input(&utils, &n) {
            Err(msg) => println!("{}", msg),
            Ok(_) => return,
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
                Err(msg) => println!("{}", msg),
                Ok(_) => return,
            },
        }
    }
}

fn handle_input(utils: &Utilities, input: &String) -> Result<usize, String> {
    if "q\n" == input {
        return Ok(0);
    }
    let mut nori = input.trim();
    if let Ok(i) = nori.parse::<usize>() {
        if let Some(n) = utils.names().get(i - 1) {
            nori = n;
        }
    }
    if let Some(f) = utils.by_name(&nori) {
        println!("######################################################################");
        println!("# Running '{}'", &nori);
        println!("######################################################################");
        f();
        Ok(0)
    } else {
        Err(format!("No '{}' utility is known", nori))
    }
}
