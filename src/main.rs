mod collatz;
use collatz::CollatzResult;
use collatz::collatz;
use std::io;
use std::io::Write;
use std::time::Instant;
macro_rules! flush {
    () => {
        io::stdout().flush().unwrap();
    };
}
fn main() {
    print!(
        "The Collatz Conjecture: Records!!\n\
        Only records will be printed.\n"
    );
    let stdin = io::stdin();
    let mut record = CollatzResult { seed: 0, steps: 0 };
    let mut input = String::new();
    let min = loop {
        print!("What is the first number you would like to calculate? ");
        flush!();
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
    let max = loop {
        print!("What is the last number you would like to calculate? ");
        flush!();
        match stdin.read_line(&mut input) {
            Ok(_) => match input.trim().parse::<u64>() {
                Ok(n) => break n,
                Err(_) => eprintln!(
                    "Not a number. Note: numbers above
                    18,446,744,073,709,551,616
                     are not supported"
                ),
            },
            Err(e) => eprintln!("Something went wrong! Error: {e}"),
        }
    };
    println!("Starting Collatz Calculations...");
    let start = Instant::now();
    for i in min..=max {
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
