pub fn run() {
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
