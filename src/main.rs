mod collatz;
use collatz::CollatzResult;
use collatz::collatz;
use std::io;
use std::io::Write;
use std::sync::mpsc;
use std::thread;
use std::time::Instant;
macro_rules! flush {
    () => {
        io::stdout().flush().unwrap();
    };
}
const NUM_THREADS: u64 = 8;
const BATCH_SIZE: u64 = 1024;
fn main() {
    print!(
        "The Collatz Conjecture: Records!!\n\
        Only records will be printed.\n"
    );
    let stdin = io::stdin();
    let mut record = CollatzResult { seed: 0, steps: 0 };
    let mut input = String::new();
    let min = loop {
        input.clear();
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
        input.clear();
        print!("What is the last number you would like to calculate? ");
        flush!();
        match stdin.read_line(&mut input) {
            Ok(_) => match input.trim().parse::<u64>() {
                Ok(n) => {
                    if n < min {
                        eprintln!("Must be more or equal to first number")
                    } else {
                        break n;
                    }
                }
                Err(e) => eprintln!(
                    "Not a number. Note: numbers above
                    18,446,744,073,709,551,616
                     are not supported. Error: {e}"
                ),
            },
            Err(e) => eprintln!("Something went wrong! Error: {e}"),
        }
    };
    println!("Starting Collatz Calculations...");
    let start = Instant::now();
    let (tresult, rresult) = mpsc::channel();
    for i in 0..NUM_THREADS {
        let tresult = tresult.clone();
        thread::spawn(move || {
            let mut batch =
                vec![CollatzResult { seed: 0, steps: 0 }; BATCH_SIZE.try_into().unwrap()];
            for (j, num) in (0_u64..).zip(((min + i)..=max).step_by(NUM_THREADS as usize)) {
                if j.is_multiple_of(BATCH_SIZE) && j != 0 {
                    for res in &batch {
                        match tresult.send(*res) {
                            Ok(()) => (),
                            Err(e) => {
                                eprintln!("{e}");
                                panic!("Bad Send");
                            }
                        };
                    }
                }
                batch[(j % BATCH_SIZE) as usize] = collatz(&num);
            }
            for res in batch {
                if res.steps == 0 {
                    break;
                } else {
                    match tresult.send(res) {
                        Ok(()) => (),
                        Err(e) => {
                            eprintln!("{e}");
                            panic!("Bad Send");
                        }
                    };
                }
            }
        });
    }
    for _ in min..=max {
        let current = rresult.recv().unwrap();
        if current.steps > record.steps {
            println!(
                "A new record! {} broke the record for most steps with {} steps!",
                current.seed, current.steps
            );
            record = current;
        }
    }
    println!(
        "Calculations took {} seconds!",
        start.elapsed().as_secs_f64()
    );
}
