mod collatz;
use collatz::CollatzResult;
use collatz::collatz;
use std::time::Instant;
use std::{io, u64};
fn main() {
    let stdin = io::stdin();
    let mut record = CollatzResult { seed: 0, steps: 0 };
    let mut input = String::new();
    let max = loop {
        println!(
            "How many numbers would you like to calculate? Only record holders will be shown."
        );
        match stdin.read_line(&mut input) {
            Ok(_) => match input.trim().parse::<u64>() {
                Ok(n) => match n {
                    1..=u64::MAX => break n,
                    0 => eprintln!("Sorry, must be 1 or above."),
                },
                Err(_) => eprintln!("Not a number."),
            },
            Err(e) => eprintln!("Something went wrong! Error: {e}"),
        }
    };
    println!("Starting Collatz Calculations...");
    let start = Instant::now();
    for i in 1..=max {
        let current = collatz(&i);
        if current.steps > record.steps {
            println!(
                "A new record! {} broke the record for most steps with {} steps!",
                current.seed, current.steps
            );
            record = current;
        }
    }
    println!("Calculations took just {}", start.elapsed().as_secs_f64());
}
