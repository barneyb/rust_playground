fn main() {
    temp_convert();
    guess_number();
}

// todo: fibonacci
// todo: 12 days of christmas

fn temp_convert() {
    use std::io;

    println!("Temperature converter!");

    loop {
        println!("Enter temp in F or C (or 'q' to quit)");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        if "q\n" == input {
            break;
        }
        let input: f64 = match input.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Try a number?");
                continue;
            }
        };
        println!("{}F is {:.1}C, and {0}C is {:.1}F", input, f2c(input), c2f(input));
    }
}

fn f2c(f: f64) -> f64 {
    (f - 32.0) / 9.0 * 5.0
}

fn c2f(c: f64) -> f64 {
    c / 5.0 * 9.0 + 32.0
}

fn guess_number() {
    use std::io;
    use std::cmp::Ordering;
    use rand::Rng;

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(0, 100) + 1;

    loop {
        println!("Please input your guess (or 'q' to quit).");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
	        .expect("Failed to read line");

        if "q\n" == guess {
            break;
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Try an integer?");
                continue;
            },
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
