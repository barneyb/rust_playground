pub fn run() {
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
