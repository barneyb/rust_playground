use rust_playground::Utilities;
use std::io;

fn main() {
    let utils = Utilities::new();
    println!("Available utilities:");
    for (i, n) in utils.names().iter().enumerate() {
        println!(" {:2}) {}", i + 1, n)
    }
    loop {
        println!("Select a module (name or number, 'q' to quit): ");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Err(_) => continue,
            Ok(2) if "q\n" == input => break,
            Ok(_) => {
                let mut nori = input.trim();
                if let Ok(i) = nori.parse::<usize>() {
                    if let Some(n) = utils.names().get(i - 1) {
                        nori = n;
                    }
                }
                if let Some(f) = utils.by_name(&nori) {
                    f();
                    break
                } else {
                    println!("No '{}' utility is known", nori);
                    continue
                }
            }
        }
    }
}
