pub fn run() {
    use std::io;

    loop {
        println!("Enter temp in F or C (or 'q' to quit)");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        if "q\n" == input {
            break;
        }
        let input: f64 = if let Ok(n) = input.trim().parse() {
            n
        } else {
            println!("Try a number?");
            continue
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn freezing() {
        assert_eq!(f2c(32.0), 0.0);
        assert_eq!(c2f(0.0), 32.0);
    }

    #[test]
    fn negative_forty() {
        assert_eq!(f2c(-40.0), -40.0);
        assert_eq!(c2f(-40.0), -40.0);
    }

    #[test]
    fn boiling() {
        assert_eq!(f2c(212.0), 100.0);
        assert_eq!(c2f(100.0), 212.0);
    }
}